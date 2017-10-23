use models::user::CurrentUser;
use rocket_contrib::Template;

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

