use super::template_contexts::{EmptyResourceContext, FlashContext, ListResourcesContext};
use db_conn::DbConn;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use models::brands::Brand;
use models::cameras::{Camera, SerializableCamera};
use models::film_formats::FilmFormat;
use models::film_stocks::{FilmStock, SerializableFilmStock};
use models::rolls::{NewRoll, Roll, RollForm};
use models::user_cameras::UserCamera;
use models::users::CurrentUser;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;
use schema::rolls::dsl::{rolls, user_camera_id};
use schema::{brands, cameras, film_formats, film_stocks, user_cameras};
use uuid::Uuid;

#[derive(Serialize)]
struct FullRoll {
    roll: Roll,
    film_stock: SerializableFilmStock,
    camera: SerializableCamera,
}

#[get("/rolls")]
fn index(current_user: CurrentUser, flash: Option<FlashMessage>, conn: DbConn) -> Template {
    let flash_context = match flash {
        Some(fm) => Some(FlashContext::new(fm)),
        None => None,
    };

    let uc_ids = current_user.user_camera_ids(&conn);

    let joined_rolls = rolls
        .inner_join(user_cameras::table)
        .inner_join(film_stocks::table)
        .filter(user_camera_id.eq_any(&uc_ids))
        .load::<(Roll, UserCamera, FilmStock)>(&*conn)
        .expect("Couldn't load rolls with joins");

    let full_rolls = joined_rolls
        .into_iter()
        .map(|(roll, uc, fs)| {
            let serializable_film_stock = build_serializable_film_stock(fs, &conn);
            let serializable_camera = build_serializable_camera(uc.camera_id, &conn);

            FullRoll {
                roll: roll,
                film_stock: serializable_film_stock,
                camera: serializable_camera,
            }
        })
        .collect();

    let context = ListResourcesContext {
        current_user: Some(current_user),
        flash: flash_context,
        name: "My Rolls",
        resources: full_rolls,
    };

    Template::render("rolls/index", context)
}

#[get("/rolls/new")]
fn new(current_user: CurrentUser, flash: Option<FlashMessage>) -> Template {
    let flash_context = match flash {
        Some(fm) => Some(FlashContext::new(fm)),
        None => None,
    };

    let context = EmptyResourceContext {
        current_user: Some(current_user),
        flash: flash_context,
    };

    Template::render("rolls/form", context)
}

#[post("/rolls", data = "<roll_form>")]
fn create(
    _current_user: CurrentUser,
    roll_form: Form<RollForm>,
    conn: DbConn,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let form = roll_form.get();

    // TODO: validate the user_camera belongs to the user.
    // let user_id: Uuid = current_user.id;
    let film_stock_id: Uuid = *form.film_stock_id;
    let uc_id: Uuid = *form.user_camera_id;
    let display_id = form.display_id.clone();
    let loaded_at = &form.loaded_at;
    let finished_at = match form.finished_at {
        Some(ref fa) => Some(fa.0),
        None => None,
    };

    let new_roll = NewRoll {
        film_stock_id: film_stock_id,
        user_camera_id: uc_id,
        display_id: display_id,
        loaded_at: loaded_at.0,
        finished_at: finished_at,
    };

    match ::diesel::insert_into(rolls)
        .values(&new_roll)
        .execute(&*conn)
    {
        Ok(_) => Ok(Flash::success(Redirect::to("/rolls"), "Added")),
        Err(err) => Err(Flash::error(Redirect::to("/rolls/new"), err.to_string())),
    }
}

fn build_serializable_film_stock(film_stock: FilmStock, conn: &DbConn) -> SerializableFilmStock {
    let brand = brands::table
        .filter(brands::id.eq(film_stock.brand_id))
        .first::<Brand>(&**conn)
        .expect("Couldn't load brand");

    let film_format = film_formats::table
        .filter(film_formats::id.eq(film_stock.film_format_id))
        .first::<FilmFormat>(&**conn)
        .expect("Couldn't load film format");

    SerializableFilmStock {
        film_stock: film_stock,
        brand: brand,
        film_format: film_format,
    }
}

fn build_serializable_camera(camera_id: Uuid, conn: &DbConn) -> SerializableCamera {
    let camera_brand = cameras::table
        .inner_join(brands::table)
        .filter(cameras::id.eq(camera_id))
        .first::<(Camera, Brand)>(&**conn)
        .expect("Couldn't load camera");

    SerializableCamera {
        camera: camera_brand.0,
        brand: camera_brand.1,
    }
}
