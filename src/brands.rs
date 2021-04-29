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

#[cfg(test)]
mod tests {
    use crate::app_state::AppState;
    use crate::models::users::CurrentUser;
    use actix_web::{http, test};
    use chrono::offset::Utc;
    use uuid::Uuid;

    #[test]
    fn meow() {
        let current_user = CurrentUser {
            id: Uuid::new_v4(),
            email: "test@test.com".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        // let code = actix::System::run(|| {
            let addr = crate::app_state::build_initial_addr();
            let state = AppState::new(addr.clone());
            // let response = test::TestRequest::with_state(state)
            let request = test::TestRequest::default()
                // .run(&super::index)
                .finish();

            let response = super::index((request, current_user)).unwrap();

            assert_eq!(response.status(), http::StatusCode::OK);
        // });
        // std::process::exit(code);
    }
}
