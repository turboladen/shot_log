use models::users::CurrentUser;
use rocket::request::FlashMessage;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::path::{Path, PathBuf};
use super::template_contexts::{EmptyResourceContext, FlashContext};

#[get("/", format="text/html")]
fn index(current_user: CurrentUser) -> Template {
    let context = EmptyResourceContext {
        current_user: Some(current_user),
        flash: None,
    };

    Template::render("home", context)
}

#[get("/", format="text/html", rank = 2)]
fn index_no_user(flash: Option<FlashMessage>) -> Template {
    match flash {
        Some(fm) => {
            let context = EmptyResourceContext {
                current_user: None,
                flash: Some(FlashContext::new(fm)),
            };

            Template::render("home", context)
        },
        None => Template::render("home", ()),
    }
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("vendor/").join(file)).ok()
}
