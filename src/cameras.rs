use diesel::{JoinDsl, LoadDsl};
use db_conn::DbConn;
use models::brands::Brand;
use models::cameras::Camera;
use models::users::CurrentUser;
use rocket_contrib::Template;
use schema::{brands, cameras};
use super::template_contexts::ListResourcesContext;

#[derive(Serialize)]
struct FullCamera {
    camera: Camera,
    brand: Brand,
}

#[get("/cameras", format = "text/html")]
fn index(current_user: CurrentUser, conn: DbConn) -> Template {
    let camera_vec = cameras::table
        .inner_join(brands::table)
        .load::<(Camera, Brand)>(&*conn)
        .expect("Error loading cameras with brands");

    let full_cameras: Vec<FullCamera> = camera_vec
        .into_iter()
        .map(|(camera, brand)| {
            FullCamera { camera: camera, brand: brand }
        })
        .collect();

    let context = ListResourcesContext {
        current_user: Some(current_user),
        flash: None,
        name: "Cameras",
        resources: full_cameras,
    };

    Template::render("cameras/index", context)
}
