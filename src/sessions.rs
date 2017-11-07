use db_conn::DbConn;
use diesel::*;
use models::users::{CurrentUser, User, LoginUser};
use rocket::http::{Cookie, Cookies};
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;
use schema::users::table as users;
use super::template_contexts::{EmptyResourceContext, FlashContext};

#[get("/login")]
fn login_form(flash: Option<FlashMessage>) -> Template {
    match flash {
        Some(fm) => {
            let context = EmptyResourceContext {
                current_user: None,
                flash: Some(FlashContext::new(fm)),
            };

            Template::render("sessions/form", context)
        },
        None => Template::render("sessions/form", ()),
    }
}

#[post("/login", data = "<login_form>")]
fn login(conn: DbConn, mut cookies: Cookies, login_form: Form<LoginUser>) -> Result<Redirect, Flash<Redirect>> {
    use schema::users::dsl::email;
    let form = login_form.get();

    match users.filter(email.eq(&form.email)).first::<User>(&*conn) {
        Ok(user) => {
            let hashed_password = ::users::password_to_hash(&form.password);

            if user.password_hash == hashed_password {
                cookies.add_private(Cookie::new("user_id", user.id.to_string()));
                Ok(Redirect::to("/user_cameras"))
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

#[cfg(test)]
mod tests {
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;
    use super::super::models::users::test::build_test_user;

    #[test]
    fn test_login_good() {
        let test_user = build_test_user();
        let client = Client::new(super::super::rocket()).expect("valid rocket instance");
        let body = format!("email={}&password={:?}", test_user.email, test_user.password);

        let response = client.post("/login")
            .header(ContentType::Form)
            .body(body)
            .dispatch();

        assert_eq!(response.status(), Status::SeeOther);
    }

    // TODO: Following the Location URL doesn't result in the flash
    // (which would contain the "Invalid password" message. We can't check
    // the cookies on the client. There's no real way to validate this.
    #[test]
    fn test_login_bad_password() {
        let test_user = build_test_user();
        let client = Client::new(super::super::rocket()).expect("valid rocket instance");
        let body = format!("email={}&password={:?}", test_user.email, "blargh");

        let response = client.post("/login")
            .header(ContentType::Form)
            .body(body)
            .dispatch();

        assert_eq!(response.status(), Status::SeeOther);

        // let new_location = response.headers()
        //     .get_one("Location")
        //     .expect("No Location header found");

        // let mut response = client.get(new_location).dispatch();
        // let response_body = response.body_string().unwrap();
        // println!("response body: {}", &response_body);

        // assert!(&response_body.contains("Invalid password"));
    }

    // TODO: Following the Location URL doesn't result in the flash
    // (which would contain the "No user with email" message. We can't check
    // the cookies on the client. There's no real way to validate this.
    #[test]
    fn test_login_bad_email() {
        let client = Client::new(super::super::rocket()).expect("valid rocket instance");
        let body = format!("email={}&password={:?}", "meow@meow.com", "blargh");

        let response = client.post("/login")
            .header(ContentType::Form)
            .body(body)
            .dispatch();

        assert_eq!(response.status(), Status::SeeOther);

        // let new_location = response.headers()
        //     .get_one("Location")
        //     .expect("No Location header found");

        // let mut response = client.get(new_location).dispatch();
        // let response_body = response.body_string().unwrap();
        // println!("response body: {}", &response_body);

        // assert!(&response_body.contains("Invalid password"));
    }
}
