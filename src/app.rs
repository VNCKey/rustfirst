use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::db::AppState;
use crate::presentation::docs::ApiDoc;
use crate::presentation::routes::routes;

pub fn build_app(state: AppState) -> Router {
    routes()
        .with_state(state)
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
}
