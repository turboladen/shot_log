use super::template_contexts::ListResourcesContext;
use crate::app_state::AppState;
use crate::handlers::GetBrands;
use crate::models::users::CurrentUser;
use actix_web::{
    error::ErrorInternalServerError, HttpRequest, HttpResponse, Result as ActixResult,
};
use futures::Future;

pub(crate) fn index(
    (req, current_user): (HttpRequest<AppState>, CurrentUser),
) -> ActixResult<HttpResponse> {
    let brands = req.state().db.send(GetBrands).wait()??;

    let context = ListResourcesContext {
        current_user: Some(current_user),
        flash: None,
        name: "Brands",
        resources: brands,
    };

    let render_result = req.state().template.render("brands/index", &context);

    let body = render_result.map_err(|e| {
        debug!("Failed to render template: {}", e.to_string());
        ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().body(&body))
}
