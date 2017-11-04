use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use schema::users;

#[derive(Identifiable, Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Identifiable, Queryable, Serialize)]
#[table_name="users"]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

use rocket::{State, Outcome};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use diesel::*;
use db_conn::DbConn;

impl<'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<CurrentUser, ()> {
        use schema::users::table as users;
        let mut cookies = request.cookies();

        match cookies.get_private("user_id") {
            Some(user_id_cookie) => {
                let user_id = match Uuid::parse_str(user_id_cookie.value()) {
                    Ok(id) => id,
                    Err(_) => return Outcome::Forward(()),
                };

                let pool = request.guard::<State<::db_conn::Pool>>()?;

                let conn = match pool.get() {
                    Ok(conn) => DbConn(conn),
                    Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ()))
                };

                match users.find(user_id).first::<User>(&*conn) {
                    Ok(user) => {
                        let u = CurrentUser {
                            id: user.id,
                            email: user.email,
                            created_at: user.created_at,
                            updated_at: user.updated_at
                        };
                        Outcome::Success(u)
                    },
                    Err(_) => Outcome::Forward(())
                }
            },
            None => {
                info!("No cookie in cookies");
                Outcome::Forward(())
            },
        }
    }
}

#[derive(FromForm)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(FromForm)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct UserToSave {
    pub email: String,
    pub password_hash: String,
}
