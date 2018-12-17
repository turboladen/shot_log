use super::template_contexts::ListResourcesContext;
use db_conn::DbConn;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use models::brands::Brand;
use models::cameras::{Camera, SerializableCamera};
use models::users::CurrentUser;
use rocket_contrib::{json::Json, templates::Template};
use schema::{brands, cameras};
use serializables::DropDown;

#[get("/cameras", format = "text/html")]
pub(crate) fn index(current_user: CurrentUser, conn: DbConn) -> Template {
    let camera_vec = cameras::table
        .inner_join(brands::table)
        .load::<(Camera, Brand)>(&*conn)
        .expect("Error loading cameras with brands");

    let serializable_cameras: Vec<SerializableCamera> = camera_vec
        .into_iter()
        .map(|(camera, brand)| SerializableCamera {
            camera: camera,
            brand: brand,
        })
        .collect();

    let context = ListResourcesContext {
        current_user: Some(current_user),
        flash: None,
        name: "Cameras",
        resources: serializable_cameras,
    };

    Template::render("cameras/index", context)
}

#[get("/cameras", format = "application/json")]
pub(crate) fn drop_down(_current_user: CurrentUser, conn: DbConn) -> Json<Vec<DropDown>> {
    use schema::brands::dsl::name as brand_name;
    use schema::cameras::dsl::model as camera_model;

    let camera_vec = cameras::table
        .inner_join(brands::table)
        .order((brand_name.asc(), camera_model.asc()))
        .load::<(Camera, Brand)>(&*conn)
        .expect("Error loading cameras with brands");

    let camera_drop_downs: Vec<DropDown> = camera_vec
        .into_iter()
        .map(|(camera, brand)| {
            let brand_and_model = format!("{} {}", brand.name, camera.model);

            DropDown {
                id: camera.id,
                label: brand_and_model,
            }
        })
        .collect();

    Json(camera_drop_downs)
}
