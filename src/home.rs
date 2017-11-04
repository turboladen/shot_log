use models::users::CurrentUser;
use rocket::request::FlashMessage;
use rocket_contrib::Template;

#[derive(Serialize)]
struct TemplateContext {
    current_user: CurrentUser,
}

#[derive(Serialize)]
struct FlashContext<'a> {
    flash: &'a str,
}

#[get("/", format="text/html")]
fn index(current_user: CurrentUser) -> Template {
    let context = TemplateContext { current_user: current_user };
    Template::render("home", context)
}

#[get("/", format="text/html", rank = 2)]
fn index_no_user(flash: Option<FlashMessage>) -> Template {
    match flash {
        Some(msg) => {
            let context = FlashContext { flash: msg.msg() };
            Template::render("home", context)
        },
        None => Template::render("home", ())
    }
}

