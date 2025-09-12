mod config;
mod db;
mod presentation;

use presentation::models::producto::ProductoPartial;
use presentation::routes::create_routes;

use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use config::load_config;
use db::init_db;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::presentation::routes::hello::hello,
        crate::presentation::routes::productos::get_productos
    ),
    components(schemas(ProductoPartial)),
    tags(
        (name = "Ejemplo", description = "API de ejemplo"),
        (name = "Productos", description = "Operaciones relacionadas con productos")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Config
    let settings = load_config()?;
    println!("Conectando a: {}", settings.database.url);

    // DB
    let state = init_db(&settings.database).await?;

    // App
    let app = create_routes()
        .with_state(state.clone())
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("🚀 Servidor corriendo en http://{}", addr);
    println!("📑 Swagger UI en http://{}/docs", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app.into_make_service(),
    )
    .await?;

    Ok(())
}
