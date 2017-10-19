use db_conn::DbConn;
use diesel::FindDsl;
use models::{User, LoginUser};
use diesel::*;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket_contrib::Template;
use schema::users::table as users;
use uuid::Uuid;

#[derive(Serialize)]
struct TemplateContext {
    email: String,
}

#[get("/", format="text/html")]
fn index(conn: DbConn, mut cookies: Cookies) -> Template {
    match cookies.get_private("user_id") {
        Some(user_id_cookie) => {
            info!("Got a cookie");
            let user_id = Uuid::parse_str(user_id_cookie.value()).expect("Couldn't parse UUID");

            match users.find(user_id).first::<User>(&*conn) {
                Ok(user) => {
                    info!("Got a user from the cookie");
                    let context = TemplateContext { email: user.email };
                    Template::render("home", context)
                },
                Err(_) => Template::render("home", ""),
            }
        },
        None => {
            info!("No cookie in cookies");
            Template::render("home", "")
        },
    }
}

#[get("/", format="text/html", rank = 2)]
fn index_no_user() -> Template {
    info!("No cookie");
    Template::render("home", "")
}

#[get("/login")]
fn login_form() -> Template {
    Template::render("login/form", ())
}

#[post("/login", data = "<login_form>")]
fn do_login(conn: DbConn, mut cookies: Cookies, login_form: Form<LoginUser>) -> Template {
    use schema::users::dsl::email;
    let form = login_form.get();

    match users.filter(email.eq(&form.email)).first::<User>(&*conn) {
        Ok(user) => {
            info!("User matched");
            let hashed_password = ::users::password_to_hash(&form.password);

            if user.password_hash == hashed_password {
                info!("password matched");
                cookies.add_private(Cookie::new("user_id", user.id.to_string()));

                let context = TemplateContext { email: user.email };
                Template::render("home", context)
            } else {
                info!("bad password");
                let context = TemplateContext { email: "bad pass".to_string() };
                Template::render("home", context)
            }
        },
        Err(_) => {
            let context = TemplateContext { email: "No user".to_string() };
            Template::render("home", context)
        }
    }
}
