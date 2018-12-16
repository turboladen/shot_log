extern crate actix;
extern crate actix_web;
extern crate argon2rs;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate handlebars;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate uuid;

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod db_conn;
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
// mod sessions;
// mod user_cameras;
// mod user_lenses;
// mod users;

use actix::prelude::*;
use actix_web::{pred, server, App, HttpRequest, State};
use actix_web::http::Method;
use actix_web::middleware::session::{RequestSession, SessionStorage, CookieSessionBackend};
use db_conn::DbExecutor;
use dotenv::dotenv;
use handlebars::Handlebars;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use std::env;

const DB_ARBITER_COUNT: usize = 3;

pub(crate) struct AppState {
    pub(crate) db: Addr<DbExecutor>,
    pub(crate) template: handlebars::Handlebars,
}

fn main() {
    let sys = actix::System::new("shotlog");
    let pool = setup_db_pool();
    let addr = SyncArbiter::start(DB_ARBITER_COUNT, move || DbExecutor(pool.clone()));

    server::new(|| create_app)
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

fn register_handlebars() -> Handlebars {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("home", "home.html.hbs");

    handlebars
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

fn create_app() -> App {
    App::new()
        .with_state(AppState { template: register_handlebars() })
        .middleware(
            SessionStorage::new(
             CookieSessionBackend::signed(&[0; 32])
                .secure(false)
            )
        )
        .resource("/", |r| {
            r.f(home::index)
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
        // .resource("/login", |r| {
        //     r.method(Method::GET).f(sessions::login_form);
        //     r.method(Method::POST).f(sessions::login)
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
}

// fn rocket() -> Rocket {
//     dotenv().ok();

//     let routes = routes![
//         home::index,
//         home::index_no_user,
//         home::files,
//     ];

//     rocket::ignite()
//         .manage(db_conn::init_pool())
//         .attach(Template::fairing())
//         .mount("/", routes)
// }
