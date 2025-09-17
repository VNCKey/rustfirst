pub mod productos;

use crate::db::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new().merge(productos::routes())
}
