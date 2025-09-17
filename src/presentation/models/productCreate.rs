use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductoCreate {
    pub categoria_id: String,
    pub nombre: String,
    pub descripcion: String,
    pub images: String,
    pub precio: i32,
}
