use rocket_contrib::Template;
use diesel::{JoinDsl, LoadDsl};
use db_conn::DbConn;
use models::brands::Brand;
use models::film_formats::FilmFormat;
use models::film_stocks::FilmStock;
use models::users::CurrentUser;
use schema::{brands, film_formats, film_stocks};
use super::template_contexts::ListResourcesContext;

#[derive(Serialize)]
struct FullFilmStock {
    film_stock: FilmStock,
    brand: Brand,
    film_format: FilmFormat,
}

#[get("/film_stocks", format = "text/html")]
fn index(current_user: CurrentUser, conn: DbConn) -> Template {
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
        .map(|((fs1, b), (fs2, ff))| {
            assert_eq!(fs1.id, fs2.id, "Got mismatched film stocks");
            FullFilmStock { film_stock: fs1, brand: b, film_format: ff }
        })
        .collect();

    let context = ListResourcesContext {
        current_user: Some(current_user),
        flash: None,
        name: "Film Stocks",
        resources: full_stocks,
    };

    Template::render("film_stocks/index", context)
}
