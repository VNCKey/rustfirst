use axum::Router;

use crate::db::AppState;
use axum::routing::{delete, get};

use crate::presentation::handlers::categorias::{
    crear_categoria, eliminar_categoria, get_categorias,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/categorias", get(get_categorias).post(crear_categoria))
        .route("/categorias/{id}", delete(eliminar_categoria))
}
