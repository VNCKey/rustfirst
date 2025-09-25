use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductoPartial {
    #[schema(value_type = String)]
    pub id: Thing,
    #[schema(value_type = String)]
    pub categoria_id: Thing,
    pub nombre: String,
    pub descripcion: String,
    pub images: String,
    pub precio: serde_json::Value,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductoCreate {
    pub categoria_id: String,
    pub nombre: String,
    pub descripcion: String,
    pub images: String,
    pub precio: i32,
}
