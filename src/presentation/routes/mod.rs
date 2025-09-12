pub mod hello;
pub mod productos;

pub use hello::hello;
pub use productos::get_productos;

use crate::db::AppState;
use axum::{routing::get, Router};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/hello", get(hello))
        .route("/productos", get(get_productos))
}
