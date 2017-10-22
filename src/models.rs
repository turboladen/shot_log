use uuid::Uuid;
use schema::{brands, film_formats, film_stocks, users};

#[derive(Identifiable, Queryable, Serialize, Associations)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
}

#[derive(Identifiable, Queryable, Serialize, Associations)]
pub struct FilmFormat {
    pub id: Uuid,
    pub designation: String,
    pub stock_size_value: Option<f64>,
    pub stock_size_unit: Option<String>,
}

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[belongs_to(Brand)]
#[belongs_to(FilmFormat)]
pub struct FilmStock {
    pub id: Uuid,
    pub box_name: String,
    pub box_speed: Option<i32>,
    pub brand_id: Uuid,
    pub film_format_id: Uuid
}

#[derive(Identifiable, Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(Identifiable, Queryable, Serialize)]
#[table_name="users"]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
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
                        let u = CurrentUser { id: user.id, email: user.email };
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
