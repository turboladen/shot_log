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
mod cameras;
mod film_formats;
mod film_stocks;
mod home;
mod lenses;
mod sessions;
mod users;

use dotenv::dotenv;
use rocket_contrib::Template;
use rocket::Rocket;

fn main() {
    rocket().launch();
}

fn rocket() -> Rocket {
    dotenv().ok();

    let routes = routes![
        home::index, home::index_no_user,
        sessions::login_form, sessions::login, sessions::logout,
        users::new, users::create,
        brands::index,
        cameras::index,
        film_formats::index,
        film_stocks::index,
        lenses::index,
    ];

    rocket::ignite()
        .manage(db_conn::init_pool())
        .attach(Template::fairing())
        .mount("/", routes)
}
