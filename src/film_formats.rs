use super::template_contexts::ListResourcesContext;
use db_conn::DbConn;
use diesel::RunQueryDsl;
use models::film_formats::FilmFormat;
use models::users::CurrentUser;
use rocket_contrib::templates::Template;
use schema::film_formats;

#[get("/film_formats", format = "text/html")]
pub(crate) fn index(current_user: CurrentUser, conn: DbConn) -> Template {
    let formats_result = film_formats::table.load::<FilmFormat>(&*conn);
    let formats = formats_result.expect("Error loading film_formats");

    let context = ListResourcesContext {
        current_user: Some(current_user),
        flash: None,
        name: "Film Formats",
        resources: formats,
    };

    Template::render("film_formats/index", context)
}
