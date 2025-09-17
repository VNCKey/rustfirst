use axum::{routing::get, Router};

use crate::db::AppState;
use crate::presentation::handlers::productos::{create_producto, get_productos};

pub fn routes() -> Router<AppState> {
    Router::new().route("/productos", get(get_productos).post(create_producto))
}
