use argon2rs::argon2d_simple;
use db_conn::DbConn;
use diesel::ExecuteDsl;
use models::{NewUser, UserToSave};
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
fn create(conn: DbConn, user_form: Form<NewUser>) -> Template {
    let u = user_form.get();
    let hashed_password = argon2d_simple(&u.password, env!("SALT"));
    let hashed_password: String = hashed_password.into_iter().map(|c| *c as char).collect();

    let user = UserToSave {
        email: u.email.clone(),
        password_hash: hashed_password
    };

    ::diesel::insert(&user).into(users::table)
        .execute(&*conn)
        .expect("Error saving new user");

    let context = TemplateContext {
        email: user.email,
    };

    Template::render("users/welcome", context)
}
