use crate::db_conn::DbExecutor;
use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use handlebars::Handlebars;
use std::env;

const DB_ARBITER_COUNT: usize = 3;

pub(crate) struct AppState {
    pub(crate) db: Addr<DbExecutor>,
    pub(crate) template: Handlebars,
}

impl AppState {
    pub(crate) fn new(addr: Addr<DbExecutor>) -> Self {
        AppState {
            db: addr,
            template: register_handlebars(),
        }
    }
}

pub(crate) fn build_initial_addr() -> Addr<DbExecutor> {
    let pool = setup_db_pool();
    SyncArbiter::start(DB_ARBITER_COUNT, move || DbExecutor(pool.clone()))
}

fn setup_db_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").unwrap_or("postgres://localhost/shot_log".into());

    // Start 3 db executor actors
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

fn register_handlebars() -> Handlebars {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html.hbs", "./templates/")
        .unwrap();
    debug!("TEMPLATES: {:#?}", handlebars.get_templates().keys());

    handlebars
}
