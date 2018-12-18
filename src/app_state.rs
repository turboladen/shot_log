use actix::prelude::*;
use db_conn::DbExecutor;
use handlebars::Handlebars;

pub(crate) struct AppState {
    pub(crate) db: Addr<DbExecutor>,
    pub(crate) template: Handlebars,
}

impl AppState {
    pub(crate) fn new(addr: Addr<DbExecutor>) -> Self {
        AppState {
            db: addr.clone(),
            template: register_handlebars(),
        }
    }
}

fn register_handlebars() -> Handlebars {
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html.hbs", "./templates/").unwrap();
    debug!("TEMPLATES: {:#?}", handlebars.get_templates().keys());

    handlebars
}
