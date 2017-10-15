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

mod brands;
mod film_formats;
mod film_stocks;

use rocket_contrib::Template;
use dotenv::dotenv;

#[get("/", format="text/html")]
fn index() -> Template {
    Template::render("home", "")
}

fn main() {
    dotenv().ok();

    let routes = routes![
        index,
        brands::index_json, brands::index_html,
        film_formats::index_json, film_formats::index_html,
        film_stocks::index_json, film_stocks::index_html,
    ];

    rocket::ignite()
        .manage(db_conn::init_pool())
        .attach(Template::fairing())
        .mount("/", routes)
        .launch();
}
