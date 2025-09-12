// src/routes/hello.rs
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HelloResponse {
    message: String,
}

#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (status = 200, description = "Un saludo", body = HelloResponse)
    )
)]
pub async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, world!".to_string(),
    })
}
