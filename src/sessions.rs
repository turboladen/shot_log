use db_conn::DbConn;
use diesel::*;
use models::users::{CurrentUser, User, LoginUser};
use rocket::http::{Cookie, Cookies};
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;
use schema::users::table as users;

#[derive(Serialize)]
struct FlashContext<'a> {
    flash_message: &'a str,
    flash_type: &'a str,
}

#[get("/login")]
fn login_form(flash: Option<FlashMessage>) -> Template {
    match flash {
        Some(msg) => {
            let context = FlashContext { flash_message: msg.msg(), flash_type: "danger" };
            Template::render("login/form", context)
        },
        None => Template::render("login/form", ())
    }
}

#[post("/login", data = "<login_form>")]
fn do_login(conn: DbConn, mut cookies: Cookies, login_form: Form<LoginUser>) -> Result<Redirect, Flash<Redirect>> {
    use schema::users::dsl::email;
    let form = login_form.get();

    match users.filter(email.eq(&form.email)).first::<User>(&*conn) {
        Ok(user) => {
            let hashed_password = ::users::password_to_hash(&form.password);

            if user.password_hash == hashed_password {
                cookies.add_private(Cookie::new("user_id", user.id.to_string()));
                Ok(Redirect::to("/"))
            } else {
                Err(Flash::error(Redirect::to("/login"), "Invalid password"))
            }
        },
        Err(_) => {
            Err(Flash::error(Redirect::to("/login"), format!("No user with email {}", &form.email)))
        }
    }
}

#[delete("/logout")]
fn logout(_current_user: CurrentUser, mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));

    Flash::success(Redirect::to("/"), "Bye!")
}
