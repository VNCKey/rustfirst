// src/models/producto.rs
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductoPartial {
    #[schema(value_type = String)]
    pub categoria_id: Thing,
    pub nombre: String,
    pub descripcion: String,
    pub images: String,
    pub precio: serde_json::Value,
}
