use argon2rs::argon2d_simple;
use db_conn::DbConn;
use diesel::LoadDsl;
use models::users::{User, NewUser, UserToSave};
use rocket::http::{Cookie, Cookies};
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;
use schema::users;

#[derive(Serialize)]
struct FlashContext<'a> {
    flash_message: &'a str,
    flash_type: &'a str,
}

#[get("/users/new")]
fn new(flash: Option<FlashMessage>) -> Template {
    match flash {
        Some(msg) => {
            let context = FlashContext { flash_message: msg.msg(), flash_type: "danger" };
            Template::render("users/form", context)
        },
        None => Template::render("users/form", ())
    }
}

#[post("/users", data = "<user_form>")]
fn create(conn: DbConn, mut cookies: Cookies, user_form: Form<NewUser>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let u = user_form.get();

    if &u.password != &u.password_confirmation {
        return Err(Flash::error(Redirect::to("/users/new"), "Passwords don't match"))
    }

    if u.password.len() < 8 {
        return Err(Flash::error(Redirect::to("/users/new"), "Passwords must be >= 8 characters"))
    }

    let hashed_password = password_to_hash(&u.password);

    let user = UserToSave {
        email: u.email.clone(),
        password_hash: hashed_password
    };

    let user: User = ::diesel::insert(&user).into(users::table)
        .get_result(&*conn)
        .expect("Error saving new user");

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));

    Ok(Flash::success(Redirect::to("/"), format!("Welcome, {}", user.email)))
}

pub fn password_to_hash(password: &str) -> String {
    let hashed_password = argon2d_simple(password, env!("SALT"));

    let s: String = hashed_password.into_iter().map(|c| *c as char).collect();

    s
}

#[cfg(test)]
pub mod test {
    use chrono::DateTime;
    use chrono::offset::Utc;
    use super::super::db_conn;
    use super::super::models::users::{User, UserToSave};
    use uuid::Uuid;

    static TEST_USER_EMAIL: &'static str = "test@shot_log.com";
    static TEST_USER_PASSWORD: &'static str = "asdfQWER1234";

    #[derive(Debug)]
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
        let hashed_password = super::password_to_hash(TEST_USER_PASSWORD);

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
