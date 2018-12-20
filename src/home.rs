use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, error::ErrorInternalServerError};
use app_state::AppState;
use super::template_contexts::EmptyResourceContext;
use models::users::CurrentUser;

pub(crate) fn index((req, current_user): (HttpRequest<AppState>, Option<CurrentUser>)) -> ActixResult<HttpResponse> {
    let render_result = match current_user {
        Some(cu) => {
            let context = EmptyResourceContext {
                current_user: Some(cu),
                flash: None,
            };

            req.state().template.render("home", &context)
        },
        None => {
            debug!("No current user.");
            req.state().template.render("home", &())
        }
    };

    let body = render_result
        .map_err(|e| {
            debug!("Failed to render template: {}", e.to_string());
            ErrorInternalServerError(e)
        })?;

    Ok(HttpResponse::Ok().body(&body))
}
