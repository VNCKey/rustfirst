use crate::presentation::models::producto::ProductoPartial;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::presentation::handlers::productos::get_productos,
        crate::presentation::handlers::productos::create_producto,
    ),
    components(schemas(ProductoPartial)),
    tags(
        (name = "Productos", description = "Operaciones relacionadas con productos")
    )
)]
pub struct ApiDoc;
