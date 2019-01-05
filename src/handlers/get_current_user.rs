use actix::prelude::*;
use actix_web::error::ErrorUnauthorized;
use actix_web::*;
use crate::db_conn::DbExecutor;
use diesel::prelude::*;
use crate::models::users::{CurrentUser, User};
use uuid::Uuid;

/// This is only message that this actor can handle, but it is easy to extend
/// number of messages.
pub struct GetCurrentUser {
    pub id: Uuid,
}

impl Message for GetCurrentUser {
    type Result = Result<CurrentUser, Error>;
}

impl Handler<GetCurrentUser> for DbExecutor {
    type Result = Result<CurrentUser, Error>;

    fn handle(&mut self, msg: GetCurrentUser, _: &mut Self::Context) -> Self::Result {
        // use schema::users::dsl::*;
        use crate::schema::users::table as users;

        let conn: &PgConnection = &self.0.get().unwrap();

        match users.find(msg.id).first::<User>(&*conn) {
            Ok(user) => {
                let u = CurrentUser {
                    id: user.id,
                    email: user.email,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                };
                Ok(u)
            }
            Err(_) => Err(ErrorUnauthorized("sup")),
        }
    }
}
