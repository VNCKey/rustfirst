use crate::db::AppState;
use crate::presentation::models::productCreate::ProductoCreate;
use crate::presentation::models::producto::ProductoPartial;
use axum::{extract::State, Json};
use axum::{http::StatusCode, response::IntoResponse};

#[utoipa::path(
    get,
    path = "/productos",
    responses(
        (status = 200, description = "Lista de productos", body = [ProductoPartial])
    )
)]
pub async fn get_productos(State(state): State<AppState>) -> Json<Vec<ProductoPartial>> {
    let productos: Vec<ProductoPartial> = state.db.select("productos").await.unwrap_or_default();
    Json(productos)
}

#[utoipa::path(
    post,
    path = "/productos",
    request_body = ProductoCreate,
    responses(
        (status = 201, description = "Producto creado", body = ProductoPartial)
    )
)]
pub async fn create_producto(
    State(state): State<AppState>,
    Json(producto): Json<ProductoCreate>,
) -> impl IntoResponse {
    // Forzamos explícitamente que lo que devuelve Surreal es un Option<ProductoPartial>
    let created: Result<Option<ProductoPartial>, surrealdb::Error> =
        state.db.create("productos").content(producto).await;

    match created {
        Ok(Some(p)) => (StatusCode::CREATED, Json(p)).into_response(),
        Ok(None) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "No se pudo crear el producto",
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al insertar el producto en la base de datos: {:?}", e),
        )
            .into_response(),
    }
}
