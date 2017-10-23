use db_conn::DbConn;
use models::user::{CurrentUser, User, LoginUser};
use diesel::*;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket_contrib::Template;
use schema::users::table as users;
use uuid::Uuid;

#[derive(Serialize)]
struct TemplateContext {
    current_user: CurrentUser,
}

#[get("/", format="text/html")]
fn index(current_user: CurrentUser) -> Template {
    let context = TemplateContext { current_user: current_user };
    Template::render("home", context)
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

                let current_user = CurrentUser { id: user.id, email: user.email };
                let context = TemplateContext { current_user: current_user };
                Template::render("home", context)
            } else {
                info!("bad password");
                let current_user = CurrentUser { id: user.id, email: "bad pass".to_string() };
                let context = TemplateContext { current_user: current_user };
                Template::render("home", context)
            }
        },
        Err(_) => {
            let current_user = CurrentUser { id: Uuid::new_v4(), email: "No user".to_string() };
            let context = TemplateContext { current_user: current_user };
            Template::render("home", context)
        }
    }
}
