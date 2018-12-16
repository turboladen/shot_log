//! Db executor actor

use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

// use schema;

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
