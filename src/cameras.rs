use diesel::{ExpressionMethods, JoinDsl, LoadDsl, OrderDsl};
use db_conn::DbConn;
use models::brands::Brand;
use models::cameras::{Camera, CameraDropDown};
use models::users::CurrentUser;
use rocket_contrib::{Json, Template};
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

#[get("/cameras", format = "application/json")]
fn index_json(_current_user: CurrentUser, conn: DbConn) -> Json<Vec<CameraDropDown>> {
    use schema::brands::dsl::name as brand_name;
    use schema::cameras::dsl::model as camera_model;

    let camera_vec = cameras::table
        .inner_join(brands::table)
        .order((brand_name.asc(), camera_model.asc()))
        .load::<(Camera, Brand)>(&*conn)
        .expect("Error loading cameras with brands");

    let camera_drop_downs: Vec<CameraDropDown> = camera_vec
        .into_iter()
        .map(|(camera, brand)| {
            let brand_and_model = format!("{} {}", brand.name, camera.model);

            CameraDropDown {
                id: camera.id,
                brand_and_model: brand_and_model,
            }
        })
        .collect();

    Json(camera_drop_downs)
}
