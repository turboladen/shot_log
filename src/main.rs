#![allow(proc_macro_derive_resolution_fallback)]
#![warn(
    box_pointers,
    // future_incompatible,
    missing_copy_implementations,
    rust_2018_idioms,
    nonstandard_style,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod db_conn;
mod flash_message;

pub(crate) mod app_state;
pub(crate) mod handlers;
pub(crate) mod models;
pub(crate) mod schema;
pub(crate) mod template_contexts;

mod brands;
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

use actix_files::NamedFile;
use actix_session::{CookieSession};
use actix_web::{http::Method, middleware, App, HttpServer};

const SERVER_ADDRESS: &str = "127.0.0.1:8088";

async fn asset(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
}

#[actix_rt::main]
async fn main() {
    setup_env();
    let addr = crate::app_state::build_initial_addr();

    HttpServer::new(|| {
        let state = app_state::AppState::new(addr.clone());

        let session_storage = CookieSession::signed(&[0; 32]).secure(false);

        App::with_state(state)
            .middleware(middleware::Logger::default())
            // .middleware(session_storage)
            // .handler(
            //     "/assets",
            //     actix_web::fs::StaticFiles::new("./assets").unwrap(),
            // )
            .route("/{filename:.*}", web::get().to(asset)))
            .resource("/", |r| r.with(home::index))
            .resource("/login", |r| {
                r.method(Method::GET).with(sessions::login_form);
                r.method(Method::POST).with(sessions::login)
            })
            .resource("/brands", |r| r.method(Method::GET).with(brands::index))
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
    .bind(SERVER_ADDRESS)?
        .run()
        .await
}

fn setup_env() {
    std::env::set_var("RUST_LOG", "shot_log=debug,actix_web=info");
    if env_logger::init().is_err() {
        panic!("Unable to init logging");
    }
    dotenv::dotenv().ok();
}
