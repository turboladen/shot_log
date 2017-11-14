use diesel::{ExpressionMethods, FilterDsl, FirstDsl, JoinDsl, LoadDsl};
use db_conn::DbConn;
use models::brands::Brand;
use models::cameras::{Camera, SerializableCamera};
use models::film_formats::FilmFormat;
use models::film_stocks::{FilmStock, SerializableFilmStock};
use models::users::CurrentUser;
use models::rolls::Roll;
use models::user_cameras::UserCamera;
use rocket::request::FlashMessage;
use rocket_contrib::Template;
use schema::{brands, cameras, film_formats, film_stocks, rolls, user_cameras};
use super::template_contexts::{FlashContext, ListResourcesContext};
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

    let user_cameras = current_user.user_cameras(&conn);
    let uc_ids: Vec<Uuid> = user_cameras.iter().map(|uc| uc.id).collect();

    let roll_ucs = rolls::table
        .inner_join(user_cameras::table)
        .filter(rolls::user_camera_id.eq_any(&uc_ids))
        .load::<(Roll, UserCamera)>(&*conn)
        .expect("Couldn't load rolls");

    let roll_fss = rolls::table
        .inner_join(film_stocks::table)
        .filter(rolls::user_camera_id.eq_any(&uc_ids))
        .load::<(Roll, FilmStock)>(&*conn)
        .expect("Couldn't load rolls");

    let full_rolls = roll_ucs
        .into_iter()
        .zip(roll_fss)
        .map(|((roll0, uc), (roll1, fs))| {
            assert_eq!(roll0.id, roll1.id);
            let serializable_film_stock = build_serializable_film_stock(fs, &conn);
            let serializable_camera = build_serializable_camera(uc.camera_id, &conn);

            FullRoll {
                roll: roll0,
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
