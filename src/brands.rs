use diesel::LoadDsl;
use db_conn::DbConn;
use models::brand::Brand;
use models::user::CurrentUser;
use rocket_contrib::Template;
use schema::brands;

#[derive(Serialize)]
struct TemplateContext {
    current_user: CurrentUser,
    name: String,
    items: Vec<Brand>
}

#[get("/brands", format = "text/html")]
fn index(current_user: CurrentUser, conn: DbConn) -> Template {
    let result = brands::table.load::<Brand>(&*conn);
    let brands = result.expect("Error loading brands");

    let context = TemplateContext {
        current_user: current_user,
        name: "Brands".to_string(),
        items: brands,
    };

    Template::render("brands/index", context)
}
