use actix_web::{error::ErrorUnauthorized, FromRequest, HttpRequest};
use app_state::AppState;
use chrono::offset::Utc;
use chrono::DateTime;
use diesel::*;
use futures::Future;
use handlers::GetCurrentUser;
// use models::user_cameras::UserCamera;
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
#[table_name = "users"]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// impl CurrentUser {
//     pub fn user_cameras(&self, conn: &DbConn) -> Vec<UserCamera> {
//         UserCamera::belonging_to(self)
//             .get_results::<UserCamera>(&**conn)
//             .expect("Couldn't find associated user cameras")
//     }

//     pub fn user_camera_ids(&self, conn: &DbConn) -> Vec<Uuid> {
//         use schema::user_cameras::dsl::id as user_camera_id;

//         UserCamera::belonging_to(self)
//             .select(user_camera_id)
//             .load::<Uuid>(&**conn)
//             .expect("Couldn't find associated user cameras")
//     }
// }

impl FromRequest<AppState> for CurrentUser {
    type Config = ();
    type Result = Result<CurrentUser, ::actix_web::Error>;

    fn from_request(request: &HttpRequest<AppState>, _: &Self::Config) -> Self::Result {
        // use schema::users::table as users;

        match request.cookie("user_id") {
            Some(user_id_cookie) => {
                let user_id = match Uuid::parse_str(user_id_cookie.value()) {
                    Ok(id) => id,
                    Err(_) => return Err(ErrorUnauthorized("sup")),
                };

                // TODO: don't wait!
                request
                    .state()
                    .db
                    .send(GetCurrentUser { id: user_id })
                    .wait()?
            }
            None => {
                info!("No cookie in cookies");
                Err(ErrorUnauthorized("sup"))
            }
        }
    }
}

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserToSave {
    pub email: String,
    pub password_hash: String,
}

#[cfg(test)]
pub mod test {
    use super::super::super::db_conn;
    use super::super::super::models::users::{User, UserToSave};
    use chrono::offset::Utc;
    use chrono::DateTime;
    use diesel::RunQueryDsl;
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
        use diesel::{ExpressionMethods, QueryDsl};
        use schema::users::dsl::email;
        use schema::users::table as users;

        let pool = db_conn::init_pool();
        let conn = pool.get().unwrap();
        let hashed_password = super::super::super::users::password_to_hash(TEST_USER_PASSWORD);

        let user = match users
            .filter(email.eq(TEST_USER_EMAIL))
            .first::<User>(&*conn)
        {
            Ok(u) => u,
            Err(_) => {
                let user_to_save = UserToSave {
                    email: String::from(TEST_USER_EMAIL),
                    password_hash: hashed_password.clone(),
                };

                let u: User = ::diesel::insert_into(users)
                    .values(&user_to_save)
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
