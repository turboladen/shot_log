use rocket_contrib::Json;
use rocket_contrib::Template;
use diesel::LoadDsl;
use db_conn::DbConn;
use models::*;
use schema::*;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<Brand>
}

#[get("/brands", format = "application/json")]
fn index_json(conn: DbConn) -> Json<Vec<Brand>> {
    Json(brands::table.load::<Brand>(&*conn).unwrap())
}

#[get("/brands", format = "text/html")]
fn index_html(conn: DbConn) -> Template {
    let result = brands::table.load::<Brand>(&*conn);
    let brands = result.expect("Error loading brands");

    let context = TemplateContext {
        name: "Brands".to_string(),
        items: brands,
    };

    Template::render("brands/index", context)
}
