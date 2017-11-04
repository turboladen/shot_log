use rocket_contrib::Template;
use diesel::{JoinDsl, LoadDsl};
use db_conn::DbConn;
use models::brands::Brand;
use models::film_formats::FilmFormat;
use models::film_stocks::FilmStock;
use models::users::CurrentUser;
use schema::{brands, film_formats, film_stocks};

#[derive(Serialize)]
struct TemplateContext<'a> {
    current_user: CurrentUser,
    name: &'a str,
    film_stocks: Vec<FullFilmStock>
}

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
        .map(|((fs0, b), (fs1, ff))| {
            assert_eq!(fs0.id, fs1.id);
            FullFilmStock { film_stock: fs0, brand: b, film_format: ff }
        })
        .collect();

    let context = TemplateContext {
        current_user: current_user,
        name: "Film Stocks",
        film_stocks: full_stocks,
    };

    Template::render("film_stocks/index", context)
}
