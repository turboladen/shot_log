use actix_web::{HttpRequest, HttpResponse, State};
use super::AppState;
use super::template_contexts::{EmptyResourceContext, FlashContext};
use models::users::CurrentUser;
// use rocket::request::FlashMessage;
// use rocket::response::NamedFile;
// use rocket_contrib::Template;
use std::path::{Path, PathBuf};

// #[get("/", format = "text/html")]
// pub(crate) fn index(current_user: CurrentUser) -> Template {
//     let context = EmptyResourceContext {
//         current_user: Some(current_user),
//         flash: None,
//     };

//     Template::render("home", context)
// }
pub(crate) fn index(state: State<AppState>) -> HttpResponse {
    let context = EmptyResourceContext {
        current_user: Some(current_user),
        flash: None,
    };

    // Template::render("home", context)
    state.template.render("home", context)
}

// #[get("/", format = "text/html", rank = 2)]
// pub(crate) fn index_no_user(flash: Option<FlashMessage>) -> Template {
//     match flash {
//         Some(fm) => {
//             let context = EmptyResourceContext {
//                 current_user: None,
//                 flash: Some(FlashContext::new(fm)),
//             };

//             Template::render("home", context)
//         }
//         None => Template::render("home", ()),
//     }
// }

// #[get("/<file..>")]
pub(crate) fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("vendor/").join(file)).ok()
}
