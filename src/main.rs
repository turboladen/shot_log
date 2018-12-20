#![allow(proc_macro_derive_resolution_fallback)]

extern crate actix;
extern crate actix_web;
extern crate argon2rs;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate handlebars;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate uuid;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod db_conn;
mod flash_message;

pub(crate) mod app_state;
pub(crate) mod handlers;
pub(crate) mod models;
pub(crate) mod schema;
pub(crate) mod template_contexts;

// mod brands;
// mod cameras;
// mod film_formats;
// mod film_stocks;
// mod form_values;
mod home;
// mod lenses;
// mod rolls;
// mod serializables;
mod sessions;
// mod user_cameras;
// mod user_lenses;
mod route_helpers;
mod users;

use actix::prelude::*;
use actix_web::middleware::session::{CookieSessionBackend, SessionStorage};
use actix_web::{http::Method, middleware, server, App};
use db_conn::DbExecutor;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use std::env;

const DB_ARBITER_COUNT: usize = 3;
const SERVER_ADDRESS: &str = "127.0.0.1:8088";

fn main() {
    setup_env();
    let sys = actix::System::new("shot_log");
    let pool = setup_db_pool();
    let addr = SyncArbiter::start(DB_ARBITER_COUNT, move || DbExecutor(pool.clone()));

    server::new(move || {
        let state = app_state::AppState::new(addr.clone());

        let session_storage =
            SessionStorage::new(CookieSessionBackend::private(&[0; 32]).secure(false));

        App::with_state(state)
            .middleware(middleware::Logger::default())
            .middleware(session_storage)
            .handler(
                "/assets",
                actix_web::fs::StaticFiles::new("./assets").unwrap(),
            )
            .resource("/", |r| r.with(home::index))
            .resource("/login", |r| {
                r.method(Method::GET).with(sessions::login_form);
                r.method(Method::POST).with(sessions::login)
            })
            // .resource("/brands", |r| r.f(brands::index))
            // .resource("/cameras", |r| {
            //     r.route()
            //         .filter(pred::Header("accept", "application/json"))
            //         .f(cameras::drop_down);

            //     r.f(cameras::index)
            // })
            // .resource("/film_formats", |r| r.f(film_formats::index))
            // .resource("/film_stocks", |r| {
            //     r.route()
            //         .filter(pred::Header("accept", "application/json"))
            //         .f(film_stocks::drop_down);

            //     r.f(film_stocks::index)
            // })
            // .resource("/lenses", |r| {
            //     r.route()
            //         .filter(pred::Header("accept", "application/json"))
            //         .method(Method::GET)
            //         .f(lenses::drop_down);

            //     r.f(lenses::index)
            // })
            // .scope("/rolls", |rolls_scope| {
            //     rolls_scope.resource("" , |r| {
            //         r.method(Method::GET).f(rolls::index);
            //         r.method(Method::POST).f(rolls::create);
            //     })
            //     .resource("/new", |r| r.f(rolls::new))
            // })
            // .resource("/logout", |r| {
            //     r.method(Method::DELETE).f(sessions::logout)
            // })
            // .scope("/users", |users_scope| {
            //     users_scope
            //         .resource("", |r| r.method(Method::POST).f(users::create))
            //         .resource("/new", |r| r.method(Method::GET).f(users::new))
            // })
            // .scope("/user_cameras", |user_cameras_scope| {
            //     user_cameras_scope
            //         .resource("", |r| {
            //             r.route()
            //                 .filter(pred::Header("accept", "application/json"))
            //                 .method(Method::GET)
            //                 .f(user_cameras::drop_down);
            //             r.method(Method::GET).f(user_cameras::index);
            //             r.method(Method::POST).f(user_cameras::create);
            //         })
            //         .resource("/new", |r| {
            //             r.method(Method::GET).f(user_cameras::new);
            //         })
            //         .resource("/{user_camera_id}", |r| {
            //             r.method(Method::DELETE).f(user_cameras::destroy)
            //         })
            // })
            // .scope("/user_lenses", |user_lenses_scope| {
            //     user_lenses_scope
            //         .resource("", |r| {
            //             r.method(Method::GET).f(user_lenses::index);
            //             r.method(Method::POST).f(user_lenses::create);
            //         })
            //         .resource("/new", |r| {
            //             r.method(Method::GET).f(user_lenses::new);
            //         })
            //         .resource("/{user_lens_id}", |r| {
            //             r.method(Method::DELETE).f(user_lenses::destroy)
            //         })
            // })
    })
    .bind(SERVER_ADDRESS)
    .expect(&format!("Cannot bind to {}", SERVER_ADDRESS))
    .start();

    println!("Started http server: {}", SERVER_ADDRESS);
    let _ = sys.run();
}

fn setup_env() {
    std::env::set_var("RUST_LOG", "shot_log=debug,actix_web=info");
    if env_logger::init().is_err() {
        panic!("Unable to init logging");
    }
    dotenv::dotenv().ok();
}

fn setup_db_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Start 3 db executor actors
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}
