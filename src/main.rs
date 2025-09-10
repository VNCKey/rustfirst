use std::net::SocketAddr;

use axum::{routing::get, Json, Router};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
#[derive(Serialize, ToSchema)]
struct HelloResponse {
    message: String,
}

#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (status = 200, description = "Un saludo", body = HelloResponse)
    )
)]
async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, world!".to_string(),
    })
}

#[derive(OpenApi)]
#[openapi(
	paths(hello), 
	components(schemas(HelloResponse)),
	tags(
		(name = "Ejemplo", description = "API de ejemplo")
	)
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let socket_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();

    let app = Router::new()
        .route("/hello", get(hello))
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()));

    println!("🚀 Servidor corriendo en http://{}", socket_address);
    println!("📑 Swagger UI en http://{}/docs", socket_address);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
