use rocket_contrib::Template;
use diesel::LoadDsl;
use db_conn::DbConn;
use models::film_formats::FilmFormat;
use models::users::CurrentUser;
use schema::film_formats;

#[derive(Serialize)]
struct TemplateContext<'a> {
    current_user: CurrentUser,
    name: &'a str,
    items: Vec<FilmFormat>
}

#[get("/film_formats", format = "text/html")]
fn index(current_user: CurrentUser, conn: DbConn) -> Template {
    let formats_result = film_formats::table.load::<FilmFormat>(&*conn);
    let formats = formats_result.expect("Error loading film_formats");

    let context = TemplateContext {
        current_user: current_user,
        name: "Film Formats",
        items: formats,
    };

    Template::render("film_formats/index", context)
}
