use chrono::DateTime;
use chrono::offset::Utc;
use schema::users;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Identifiable, Queryable, Serialize, Hash)]
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

#[cfg(test)]
pub mod test {
    use chrono::DateTime;
    use chrono::offset::Utc;
    use super::super::super::db_conn;
    use super::super::super::models::users::{User, UserToSave};
    use uuid::Uuid;

    static TEST_USER_EMAIL: &'static str = "test@shot_log.com";
    static TEST_USER_PASSWORD: &'static str = "asdfQWER1234";

    pub struct TestUser {
        pub id: Uuid,
        pub email: String,
        pub password: String,
        pub password_hash: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    pub fn build_test_user() -> TestUser {
        use diesel::{ExpressionMethods, FilterDsl, FirstDsl, LoadDsl};
        use schema::users::dsl::email;
        use schema::users::table as users;

        let pool = db_conn::init_pool();
        let conn = pool.get().unwrap();
        let hashed_password = super::super::super::users::password_to_hash(TEST_USER_PASSWORD);

        let user = match users.filter(email.eq(TEST_USER_EMAIL)).first::<User>(&*conn) {
            Ok(u) => u,
            Err(_) => {
                let user_to_save = UserToSave {
                    email: String::from(TEST_USER_EMAIL),
                    password_hash: hashed_password.clone()
                };

                let u: User = ::diesel::insert(&user_to_save).into(users)
                    .get_result(&*conn)
                    .expect("Error saving test user");

                u
            }
        };

        TestUser {
            id: user.id,
            email: user.email,
            password: String::from(TEST_USER_PASSWORD),
            password_hash: hashed_password,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
