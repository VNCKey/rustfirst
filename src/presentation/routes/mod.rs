pub mod hello;
pub mod productos;

use hello::hello;
pub use productos::{create_producto, get_productos};

use crate::db::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/hello", get(hello))
        .route("/productos", get(get_productos).post(create_producto))
}
