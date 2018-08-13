use super::template_contexts::ListResourcesContext;
use db_conn::DbConn;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use models::brands::Brand;
use models::lenses::Lens;
use models::users::CurrentUser;
use rocket_contrib::{Json, Template};
use schema::{brands, lenses};
use serializables::DropDown;

#[derive(Serialize)]
struct FullLens {
    lens: Lens,
    brand: Brand,
}

#[get("/lenses", format = "text/html")]
fn index(current_user: CurrentUser, conn: DbConn) -> Template {
    let lens_vec = lenses::table
        .inner_join(brands::table)
        .load::<(Lens, Brand)>(&*conn)
        .expect("Error loading film stocks with brands");

    let full_lenses: Vec<FullLens> = lens_vec
        .into_iter()
        .map(|(lens, brand)| FullLens {
            lens: lens,
            brand: brand,
        })
        .collect();

    let context = ListResourcesContext {
        current_user: Some(current_user),
        flash: None,
        name: "Lenses",
        resources: full_lenses,
    };

    Template::render("lenses/index", context)
}

#[get("/lenses", format = "application/json")]
fn drop_down(_current_user: CurrentUser, conn: DbConn) -> Json<Vec<DropDown>> {
    use schema::brands::dsl::name as brand_name;
    use schema::lenses::dsl::model as lens_model;

    let lens_vec = lenses::table
        .inner_join(brands::table)
        .order((brand_name.asc(), lens_model.asc()))
        .load::<(Lens, Brand)>(&*conn)
        .expect("Error loading lenses with brands");

    let lens_drop_downs: Vec<DropDown> = lens_vec
        .into_iter()
        .map(|(lens, brand)| {
            let brand_and_model = format!("{} {}", brand.name, lens.model);

            DropDown {
                id: lens.id,
                label: brand_and_model,
            }
        })
        .collect();

    Json(lens_drop_downs)
}
