use crate::domain::models::{
    categoria::{CategoriaPartial, NewCategoria},
    producto::{ProductoCreate, ProductoPartial},
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(

				//Categorias
        crate::presentation::handlers::categorias::get_categorias,
        crate::presentation::handlers::categorias::crear_categoria,
        crate::presentation::handlers::categorias::eliminar_categoria,

				//Productos
        crate::presentation::handlers::productos::get_productos,
        crate::presentation::handlers::productos::create_producto,
    ),
    components(schemas(ProductoPartial,CategoriaPartial,NewCategoria,ProductoCreate)),
    tags(
        (name = "Categorias", description = "Operaciones relacionadas con categorias"),
        (name = "Productos", description = "Operaciones relacionadas con productos"),
    )
)]
pub struct ApiDoc;
