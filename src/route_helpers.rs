use actix_web::{http, HttpResponse};

pub(crate) fn redirect_to(path: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, path)
        .finish()
}
