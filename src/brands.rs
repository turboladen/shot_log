use super::template_contexts::ListResourcesContext;
use db_conn::DbConn;
use diesel::RunQueryDsl;
use models::brands::Brand;
use models::users::CurrentUser;
use rocket_contrib::templates::Template;
use schema::brands;

#[get("/brands", format = "text/html")]
pub(crate) fn index(current_user: CurrentUser, conn: DbConn) -> Template {
    let result = brands::table.load::<Brand>(&*conn);
    let brands = result.expect("Error loading brands");

    let context = ListResourcesContext {
        current_user: Some(current_user),
        flash: None,
        name: "Brands",
        resources: brands,
    };

    Template::render("brands/index", context)
}
