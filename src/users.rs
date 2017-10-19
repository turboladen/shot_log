use argon2rs::argon2d_simple;
use db_conn::DbConn;
use diesel::LoadDsl;
use models::{User, NewUser, UserToSave};
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket_contrib::Template;
use schema::*;

#[derive(Serialize)]
struct TemplateContext {
    email: String,
}

#[get("/users/new")]
fn new() -> Template {
    Template::render("users/form", ())
}

#[post("/users", data = "<user_form>")]
fn create(conn: DbConn, mut cookies: Cookies, user_form: Form<NewUser>) -> Template {
    let u = user_form.get();
    let hashed_password = password_to_hash(&u.password);

    let user = UserToSave {
        email: u.email.clone(),
        password_hash: hashed_password
    };

    let user: User = ::diesel::insert(&user).into(users::table)
        .get_result(&*conn)
        .expect("Error saving new user");

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));

    let context = TemplateContext {
        email: user.email,
    };

    Template::render("users/welcome", context)
}

pub fn password_to_hash(password: &str) -> String {
    let hashed_password = argon2d_simple(password, env!("SALT"));

    let s: String = hashed_password.into_iter().map(|c| *c as char).collect();

    s
}
