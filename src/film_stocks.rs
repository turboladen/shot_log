use rocket_contrib::Json;
use rocket_contrib::Template;
use diesel::{JoinDsl, LoadDsl};
use db_conn::DbConn;
use models::{Brand, CurrentUser, FilmFormat, FilmStock};
use schema::*;

#[derive(Serialize)]
struct TemplateContext {
    current_user: CurrentUser,
    name: String,
    film_stocks: Vec<FullFilmStock>
}

#[derive(Serialize)]
struct FullFilmStock {
    film_stock: FilmStock,
    brand: Brand,
    film_format: FilmFormat,
}

#[get("/film_stocks", format = "application/json")]
fn index_json(conn: DbConn) -> Json<Vec<FilmStock>> {
    let stocks_result = film_stocks::table.load::<FilmStock>(&*conn);
    let stocks = stocks_result.expect("Error loading film_stocks");

    Json(stocks)
}

#[get("/film_stocks", format = "text/html")]
fn index_html(current_user: CurrentUser, conn: DbConn) -> Template {
    let fsb_vec = film_stocks::table
        .inner_join(brands::table)
        .load::<(FilmStock, Brand)>(&*conn)
        .expect("Error loading film stocks with brands");

    let fsff_vec = film_stocks::table
        .inner_join(film_formats::table)
        .load::<(FilmStock, FilmFormat)>(&*conn)
        .expect("Error loading film stocks with film formats");

    let full_stocks: Vec<FullFilmStock> = fsb_vec
        .into_iter()
        .zip(fsff_vec)
        .map(|((fs0, b), (fs1, ff))| {
            assert_eq!(fs0.id, fs1.id);
            FullFilmStock { film_stock: fs0, brand: b, film_format: ff }
        })
        .collect();

    let context = TemplateContext {
        current_user: current_user,
        name: "Film Stocks".to_string(),
        film_stocks: full_stocks,
    };

    Template::render("film_stocks/index", context)
}
