use rocket_contrib::Json;
use rocket_contrib::Template;
use diesel::{ExpressionMethods, JoinDsl, JoinOnDsl, LoadDsl};
use db_conn::DbConn;
use models::*;
use schema::*;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    film_stocks: Vec<FullFilmStock>
}

#[derive(Serialize)]
struct FullFilmStock {
    film_stock: FilmStock,
    brand: Brand,
}

#[get("/film_stocks", format = "application/json")]
fn index_json(conn: DbConn) -> Json<Vec<FilmStock>> {
    let stocks_result = film_stocks::table.load::<FilmStock>(&*conn);
    let stocks = stocks_result.expect("Error loading film_stocks");

    Json(stocks)
}

#[get("/film_stocks", format = "text/html")]
fn index_html(conn: DbConn) -> Template {
    let result = film_stocks::table
        .inner_join(brands::table.on(film_stocks::brand_id.eq(brands::id)))
        .load(&*conn);
    let stocks: Vec<(FilmStock, Brand)>  = result.expect("Error loading film_stocks");

    let full_stocks = stocks.into_iter()
        .map(|(fs, b)| FullFilmStock { film_stock: fs, brand: b })
        .collect();

    let context = TemplateContext {
        name: "Film Stocks".to_string(),
        film_stocks: full_stocks,
    };

    Template::render("film_stocks/index", context)
}
