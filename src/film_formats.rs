use rocket_contrib::Json;
use rocket_contrib::Template;
use diesel::LoadDsl;
use db_conn::DbConn;
use models::{CurrentUser, FilmFormat};
use schema::*;

#[derive(Serialize)]
struct TemplateContext {
    current_user: CurrentUser,
    name: String,
    items: Vec<FilmFormat>
}

#[get("/film_formats", format = "application/json")]
fn index_json(conn: DbConn) -> Json<Vec<FilmFormat>> {
    let formats_result = film_formats::table.load::<FilmFormat>(&*conn);
    let formats = formats_result.expect("Error loading film_formats");

    Json(formats)
}

#[get("/film_formats", format = "text/html")]
fn index_html(current_user: CurrentUser, conn: DbConn) -> Template {
    let formats_result = film_formats::table.load::<FilmFormat>(&*conn);
    let formats = formats_result.expect("Error loading film_formats");

    let context = TemplateContext {
        current_user: current_user,
        name: "Film Formats".to_string(),
        items: formats,
    };

    Template::render("film_formats/index", context)
}
