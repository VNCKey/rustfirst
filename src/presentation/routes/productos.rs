use crate::db::AppState;
use crate::presentation::models::producto::ProductoPartial;
use axum::{extract::State, Json};

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
