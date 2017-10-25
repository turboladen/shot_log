#![feature(custom_derive)]

#![feature(plugin)]
#![plugin(rocket_codegen)]

#![feature(const_fn)]

#![recursion_limit="256"]

extern crate argon2rs;
extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate rocket;
extern crate rocket_contrib;
extern crate uuid;

#[macro_use] extern crate serde_derive;

pub mod schema;
pub mod models;
mod db_conn;

mod brands;
mod film_formats;
mod film_stocks;
mod home;
mod sessions;
mod users;

use dotenv::dotenv;
use rocket_contrib::Template;

fn main() {
    dotenv().ok();

    let routes = routes![
        home::index, home::index_no_user,
        sessions::login_form, sessions::do_login, sessions::logout,
        users::new, users::create,
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
