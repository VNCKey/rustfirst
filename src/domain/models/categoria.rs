use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoriaPartial {
    #[schema(value_type = String)]
    pub id: Thing,
    pub nombre: String,
    pub descripcion: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewCategoria {
    pub nombre: String,
    pub descripcion: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteCategoria {
    #[schema(value_type = String)]
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
}
