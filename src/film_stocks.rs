use rocket_contrib::Json;
use rocket_contrib::Template;
use diesel::LoadDsl;
use db_conn::DbConn;
use models::*;
use schema::*;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<FilmStock>
}

#[get("/film_stocks", format = "application/json")]
fn index_json(conn: DbConn) -> Json<Vec<FilmStock>> {
    let stocks_result = film_stocks::table.load::<FilmStock>(&*conn);
    let stocks = stocks_result.expect("Error loading film_stocks");

    Json(stocks)
}

#[get("/film_stocks", format = "text/html")]
fn index_html(conn: DbConn) -> Template {
    let result = film_stocks::table.load::<FilmStock>(&*conn);
    let stocks = result.expect("Error loading film_stocks");

    let context = TemplateContext {
        name: "Film Stocks".to_string(),
        items: stocks,
    };

    Template::render("film_stocks/index", context)
}
