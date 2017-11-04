use diesel::LoadDsl;
use db_conn::DbConn;
use models::brands::Brand;
use models::users::CurrentUser;
use rocket_contrib::Template;
use schema::brands;

#[derive(Serialize)]
struct TemplateContext<'a> {
    current_user: CurrentUser,
    name: &'a str,
    items: Vec<Brand>
}

#[get("/brands", format = "text/html")]
fn index(current_user: CurrentUser, conn: DbConn) -> Template {
    let result = brands::table.load::<Brand>(&*conn);
    let brands = result.expect("Error loading brands");

    let context = TemplateContext {
        current_user: current_user,
        name: "Brands",
        items: brands,
    };

    Template::render("brands/index", context)
}
