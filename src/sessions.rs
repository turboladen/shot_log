use actix_web::middleware::session::RequestSession;
use actix_web::{
    error::ErrorInternalServerError, Form, HttpRequest, HttpResponse, Result as ActixResult,
};
use app_state::AppState;
use flash_message::{self, FlashMessage};
use futures::Future;
use handlers::GetLoginUser;
// use diesel::*;
// use models::users::{CurrentUser, LoginUser, User};
use models::users::LoginUser;
// use rocket::http::{Cookie, Cookies};
// use rocket::request::{FlashMessage, Form};
// use rocket::response::{Flash, Redirect};
// use schema::users::table as users;
use super::template_contexts::{EmptyResourceContext, FlashContext};
use flash_message::get_flash;
use route_helpers;

pub(crate) fn login_form(req: HttpRequest<AppState>) -> ActixResult<HttpResponse> {
    let render_result = match get_flash(&req)? {
        Some(fm) => {
            let context = EmptyResourceContext {
                current_user: None,
                flash: Some(FlashContext::new(fm)),
            };

            req.state().template.render("sessions/form", &context)
        }
        None => req.state().template.render("sessions/form", &()),
    };

    let body = render_result.map_err(|e| {
        debug!("Failed to render template: {}", e.to_string());
        ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().body(&body))
}

pub(crate) fn login(
    (req, form): (HttpRequest<AppState>, Form<LoginUser>),
) -> ActixResult<HttpResponse> {
    let user_result = req
        .state()
        .db
        .send(GetLoginUser {
            email: form.email.clone(),
        })
        .wait()?;

    match user_result {
        Ok(user) => {
            let hashed_password = ::users::password_to_hash(&form.password);

            if user.password_hash == hashed_password {
                req.session().set("user_id", user.id.to_string());
                Ok(route_helpers::redirect_to("/user_cameras"))
            } else {
                let message = FlashMessage::error("Invalid password");
                flash_message::set_flash(&req, message);
                Ok(route_helpers::redirect_to("/login"))
            }
        }
        Err(e) => Ok(e.into()),
    }
}

// #[delete("/logout")]
// pub(crate) fn logout(_current_user: CurrentUser, mut cookies: Cookies) -> Flash<Redirect> {
//     cookies.remove_private(Cookie::named("user_id"));

//     Flash::success(Redirect::to("/"), "Bye!")
// }

#[cfg(test)]
mod tests {
    use super::super::models::users::test::build_test_user;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn test_login_good() {
        let test_user = build_test_user();
        let client = Client::new(super::super::rocket()).expect("valid rocket instance");
        let body = format!(
            "email={}&password={:?}",
            test_user.email, test_user.password
        );

        let response = client
            .post("/login")
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

        let response = client
            .post("/login")
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

        let response = client
            .post("/login")
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
