use crate::db::AppState;
use crate::domain::models::categoria::{CategoriaPartial, NewCategoria};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

#[utoipa::path(
    get,
    path = "/categorias",
    responses(
        (status = 200, description = "Lista de categorias", body = [CategoriaPartial])
    )
)]
pub async fn get_categorias(State(state): State<AppState>) -> Json<Vec<CategoriaPartial>> {
    let categorias: Vec<CategoriaPartial> = state.db.select("categorias").await.unwrap_or_default();
    Json(categorias)
}

#[utoipa::path(
    post,
    path = "/categorias",
    responses(
        (status = 200, description = "Categoria creada", body = CategoriaPartial)
    )
)]
pub async fn crear_categoria(
    State(state): State<AppState>,
    Json(data): Json<NewCategoria>,
) -> Json<Option<CategoriaPartial>> {
    let categoria: Option<CategoriaPartial> = state
        .db
        .create("categorias")
        .content(data)
        .await
        .unwrap_or_default();

    Json(categoria)
}

#[utoipa::path(
    delete,
    path = "/categorias/{id}",
    params(
        ("id" = String, Path, description = "ID de la categoría a eliminar")
    ),
    responses(
        (status = 200, description = "Categoría eliminada", body = CategoriaPartial),
        (status = 404, description = "Categoría no encontrada")
    )
)]
pub async fn eliminar_categoria(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<CategoriaPartial>, StatusCode> {
    let key = format!("categorias:{}", id);

    let categorias: Vec<CategoriaPartial> = state.db.delete(key).await.unwrap_or_default();

    match categorias.into_iter().next() {
        Some(cat) => Ok(Json(cat)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
