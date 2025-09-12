// src/routes/mod.rs
use axum::Router;

mod hello;
mod productos;

pub use hello::hello;
pub use productos::get_productos;

use crate::db::AppState;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/hello", axum::routing::get(hello))
        .route("/productos", axum::routing::get(get_productos))
}
