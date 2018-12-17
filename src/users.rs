use super::template_contexts::FlashContext;
use argon2rs::argon2d_simple;
use db_conn::DbConn;
use diesel::RunQueryDsl;
use models::users::{NewUser, User, UserToSave};
use rocket::http::{Cookie, Cookies};
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use schema::users;
use std::env;

#[get("/users/new")]
pub(crate) fn new(flash: Option<FlashMessage>) -> Template {
    match flash {
        Some(fm) => Template::render("users/form", FlashContext::new(fm)),
        None => Template::render("users/form", ()),
    }
}

#[post("/users", data = "<user_form>")]
pub(crate) fn create(
    conn: DbConn,
    mut cookies: Cookies,
    user_form: Form<NewUser>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let u = user_form.get();

    if u.password != u.password_confirmation {
        return Err(Flash::error(
            Redirect::to("/users/new"),
            "Passwords don't match",
        ));
    }

    if u.password.len() < 8 {
        return Err(Flash::error(
            Redirect::to("/users/new"),
            "Passwords must be >= 8 characters",
        ));
    }

    let hashed_password = password_to_hash(&u.password);

    let user = UserToSave {
        email: u.email.clone(),
        password_hash: hashed_password,
    };

    let user: User = ::diesel::insert_into(users::table)
        .values(&user)
        .get_result(&*conn)
        .expect("Error saving new user");

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));

    Ok(Flash::success(
        Redirect::to("/"),
        format!("Welcome, {}", user.email),
    ))
}

pub(crate) fn password_to_hash(password: &str) -> String {
    let hashed_password = argon2d_simple(password, env::var("SALT").expect("No SALT").as_str());

    let s: String = hashed_password.into_iter().map(|c| *c as char).collect();

    s
}
