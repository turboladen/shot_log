use rocket_contrib::Template;
use diesel::{ExecuteDsl, ExpressionMethods, FilterDsl, JoinDsl, LoadDsl};
use db_conn::DbConn;
use models::brands::Brand;
use models::cameras::Camera;
use models::user_cameras::{NewUserCamera, UserCamera, UserCameraForm};
use models::users::CurrentUser;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::UUID;
use schema::{brands, cameras, user_cameras};
use super::template_contexts::{EmptyResourceContext, FlashContext, ListResourcesContext};
use uuid::Uuid;

#[derive(Serialize)]
struct FullUserCamera<'a> {
    user_camera: UserCamera,
    camera: &'a Camera,
    brand: &'a Brand,
}

#[get("/user_cameras", format = "text/html")]
fn index(current_user: CurrentUser, flash: Option<FlashMessage>, conn: DbConn) -> Template {
    let flash_context = match flash {
        Some(fm) => Some(FlashContext::new(fm)),
        None => None,
    };

    use schema::user_cameras::dsl::user_id;
    use schema::cameras::dsl::id as camera_id;

    let uc_cameras = user_cameras::table
        .filter(user_id.eq(&current_user.id))
        .load::<UserCamera>(&*conn)
        .expect("Error loading user cameras");

    let camera_ids: Vec<Uuid> = uc_cameras.iter().map(|&ref uc| uc.camera_id).collect();

    let camera_brands = cameras::table
        .inner_join(brands::table)
        .filter(camera_id.eq_any(camera_ids))
        .load::<(Camera, Brand)>(&*conn)
        .expect("Error loading cameras with brands");

    let full_user_cameras: Vec<FullUserCamera> = uc_cameras
        .into_iter()
        .map(|uc| {
            let &(ref camera, ref brand) = camera_brands.iter()
                .find(|&&(ref camera, _)| camera.id == uc.camera_id)
                .expect("meow");
            FullUserCamera { user_camera: uc, camera: &camera, brand: &brand }
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
fn create(current_user: CurrentUser, user_camera_form: Form<UserCameraForm>, conn: DbConn) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let uc = user_camera_form.get();
    let user_id: Uuid = current_user.id;
    let camera_id: Uuid = *uc.camera_id;

    let new_uc = NewUserCamera {
        user_id: user_id,
        camera_id: camera_id,
        roll_prefix: uc.roll_prefix.clone(),
        serial_number: uc.serial_number.clone()
    };

    match ::diesel::insert(&new_uc).into(user_cameras::table).execute(&*conn) {
        Ok(_) => {
            Ok(Flash::success(Redirect::to("/user_cameras"), "Added"))
        },
        Err(err) => {
            Err(Flash::error(Redirect::to("/user_cameras/new"), err.to_string()))
        }
    }
}

#[delete("/user_cameras/<id>")]
fn destroy(current_user: CurrentUser, id: UUID, conn: DbConn) -> Result<Flash<Redirect>, Flash<Redirect>> {
    use schema::user_cameras::dsl::id as user_camera_id;
    use schema::user_cameras::dsl::user_id;

    let result = ::diesel::delete(
        user_cameras::table
        .filter(user_camera_id.eq(*id))
        .filter(user_id.eq(current_user.id))
        )
        .execute(&*conn);

    match result {
        Ok(_uc) => {
            Ok(Flash::success(Redirect::to("/user_cameras"), "Removed!"))
        },
        Err(err) => {
            Err(Flash::error(Redirect::to("/user_cameras"), err.to_string()))
        }
    }
}
