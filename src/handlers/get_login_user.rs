use actix::prelude::*;
use actix_web::error::ErrorUnauthorized;
use actix_web::*;
use db_conn::DbExecutor;
use diesel::prelude::*;
use models::users::User;

pub struct GetLoginUser {
    pub email: String,
}

impl Message for GetLoginUser {
    type Result = Result<User, Error>;
}

impl Handler<GetLoginUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: GetLoginUser, _: &mut Self::Context) -> Self::Result {
        use schema::users::dsl::email;
        use schema::users::table as users;

        let conn: &PgConnection = &self.0.get().unwrap();

        let result = users
            .filter(email.eq(&msg.email))
            .first::<User>(&*conn)
            .map_err(|_| ErrorUnauthorized("No user with given email"))?;

        Ok(result)
    }
}