#![feature(plugin)]
#![plugin(rocket_codegen)]

#![feature(const_fn)]

extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2_diesel;
extern crate r2d2;

extern crate uuid;

#[macro_use] extern crate serde_derive;

pub mod schema;
pub mod models;
mod db_conn;

use rocket_contrib::Json;
use rocket_contrib::Template;
use diesel::LoadDsl;
use db_conn::DbConn;
use models::*;
use schema::*;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<FilmFormat>
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/brands")]
fn brands(conn: DbConn) -> Json<Vec<Brand>> {
    Json(brands::table.load::<Brand>(&*conn).unwrap())
}

#[get("/film_formats", format = "application/json")]
fn all_film_formats(conn: DbConn) -> Json<Vec<FilmFormat>> {
    let formats_result = film_formats::table.load::<FilmFormat>(&*conn);
    let formats = formats_result.expect("Error loading film_formats");

    Json(formats)
}

#[get("/film_formats", format = "text/html")]
fn html_film_formats(conn: DbConn) -> Template {
    let formats_result = film_formats::table.load::<FilmFormat>(&*conn);
    let formats = formats_result.expect("Error loading film_formats");

    let context = TemplateContext {
        name: "Film Formats".to_string(),
        items: formats,
    };

    Template::render("film_formats/index", context)
}

#[get("/film_stocks")]
fn all_film_stocks(conn: DbConn) -> Json<Vec<FilmStock>> {
    let stocks_result = film_stocks::table.load::<FilmStock>(&*conn);
    let stocks = stocks_result.expect("Error loading film_stocks");

    Json(stocks)
}

fn main() {
    rocket::ignite()
        .manage(db_conn::init_pool())
        .attach(Template::fairing())
        .mount("/", routes![index, brands, all_film_formats, html_film_formats, all_film_stocks])
        .launch();
}
