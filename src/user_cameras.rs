use rocket_contrib::{Json, Template};
use diesel::{BelongingToDsl, ExecuteDsl, ExpressionMethods, FilterDsl, JoinDsl, JoinOnDsl, LoadDsl};
use db_conn::DbConn;
use models::brands::Brand;
use models::cameras::Camera;
use models::user_cameras::{NewUserCamera, UserCamera, UserCameraForm};
use models::users::CurrentUser;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::UUID;
use schema::{brands, cameras, user_cameras};
use serializables::DropDown;
use super::template_contexts::{EmptyResourceContext, FlashContext, ListResourcesContext};
use uuid::Uuid;

#[derive(Serialize)]
struct FullUserCamera {
    user_camera: UserCamera,
    camera: Camera,
    brand: Brand,
}

#[get("/user_cameras", format = "text/html")]
fn index(current_user: CurrentUser, flash: Option<FlashMessage>, conn: DbConn) -> Template {
    let flash_context = match flash {
        Some(fm) => Some(FlashContext::new(fm)),
        None => None,
    };

    let data = UserCamera::belonging_to(&current_user)
        .inner_join(
            brands::table
                .inner_join(cameras::table.on(cameras::brand_id.eq(brands::id)))
                .on(user_cameras::camera_id.eq(cameras::id)),
        )
        .load::<(UserCamera, (Brand, Camera))>(&*conn)
        .expect("Error loading user cameras");

    let full_user_cameras: Vec<FullUserCamera> = data.into_iter()
        .map(|(uc, (brand, camera))| {
            FullUserCamera {
                user_camera: uc,
                camera: camera,
                brand: brand,
            }
        })
        .collect();

    let context = ListResourcesContext {
        current_user: Some(current_user),
        flash: flash_context,
        name: "My Cameras",
        resources: full_user_cameras,
    };

    Template::render("user_cameras/index", context)
}

#[get("/user_cameras", format = "application/json")]
fn drop_down(current_user: CurrentUser, conn: DbConn) -> Json<Vec<DropDown>> {
    let data = user_cameras::table
        .inner_join(
            brands::table
                .inner_join(cameras::table.on(cameras::brand_id.eq(brands::id)))
                .on(user_cameras::camera_id.eq(cameras::id)),
        )
        .filter(user_cameras::user_id.eq(&current_user.id))
        .load::<(UserCamera, (Brand, Camera))>(&*conn)
        .expect("Error loading user cameras");

    let user_camera_dropdowns: Vec<DropDown> = data.into_iter()
        .map(|(uc, (brand, camera))| {
            let label = format!("{} {}", brand.name, camera.model);

            DropDown {
                id: uc.id,
                label: label
            }
        })
        .collect();

    Json(user_camera_dropdowns)
}

#[get("/user_cameras/new")]
fn new(current_user: CurrentUser, flash: Option<FlashMessage>) -> Template {
    let flash_context = match flash {
        Some(fm) => Some(FlashContext::new(fm)),
        None => None,
    };

    let context = EmptyResourceContext {
        current_user: Some(current_user),
        flash: flash_context,
    };

    Template::render("user_cameras/form", context)
}

#[post("/user_cameras", data = "<user_camera_form>")]
fn create(
    current_user: CurrentUser,
    user_camera_form: Form<UserCameraForm>,
    conn: DbConn,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let uc = user_camera_form.get();
    let user_id: Uuid = current_user.id;
    let camera_id: Uuid = *uc.camera_id;

    let new_uc = NewUserCamera {
        user_id: user_id,
        camera_id: camera_id,
        roll_prefix: uc.roll_prefix.clone(),
        serial_number: uc.serial_number.clone(),
    };

    match ::diesel::insert_into(user_cameras::table)
        .values(&new_uc)
        .execute(&*conn)
    {
        Ok(_) => Ok(Flash::success(Redirect::to("/user_cameras"), "Added")),
        Err(err) => Err(Flash::error(
            Redirect::to("/user_cameras/new"),
            err.to_string(),
        )),
    }
}

#[delete("/user_cameras/<id>")]
fn destroy(
    current_user: CurrentUser,
    id: UUID,
    conn: DbConn,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    use schema::user_cameras::dsl::id as user_camera_id;
    use schema::user_cameras::dsl::user_id;

    let result = ::diesel::delete(
        user_cameras::table
            .filter(user_camera_id.eq(*id))
            .filter(user_id.eq(current_user.id)),
    ).execute(&*conn);

    match result {
        Ok(_uc) => Ok(Flash::success(Redirect::to("/user_cameras"), "Removed!")),
        Err(err) => Err(Flash::error(Redirect::to("/user_cameras"), err.to_string())),
    }
}
