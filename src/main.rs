#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(const_fn)]
#![recursion_limit = "256"]

extern crate argon2rs;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate uuid;

extern crate serde;
#[macro_use]
extern crate serde_derive;

mod db_conn;
pub mod models;
pub mod schema;
pub mod template_contexts;

mod brands;
mod cameras;
mod film_formats;
mod film_stocks;
mod form_values;
mod home;
mod lenses;
mod rolls;
mod serializables;
mod sessions;
mod user_cameras;
mod user_lenses;
mod users;

use dotenv::dotenv;
use rocket::Rocket;
use rocket_contrib::Template;

fn main() {
    rocket().launch();
}

fn rocket() -> Rocket {
    dotenv().ok();

    let routes = routes![
        brands::index,
        cameras::index,
        cameras::drop_down,
        film_formats::index,
        film_stocks::index,
        film_stocks::drop_down,
        home::index,
        home::index_no_user,
        home::files,
        lenses::index,
        lenses::drop_down,
        rolls::index,
        rolls::new,
        rolls::create,
        sessions::login_form,
        sessions::login,
        sessions::logout,
        users::new,
        users::create,
        user_cameras::index,
        user_cameras::drop_down,
        user_cameras::new,
        user_cameras::create,
        user_cameras::destroy,
        user_lenses::index,
        user_lenses::new,
        user_lenses::create,
        user_lenses::destroy
    ];

    rocket::ignite()
        .manage(db_conn::init_pool())
        .attach(Template::fairing())
        .mount("/", routes)
}
