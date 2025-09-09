use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::any;
use surrealdb::opt::auth::Root;
use tokio;

#[derive(Debug, Serialize, Deserialize)]
struct PersonPartial {
    email: String,
    first_name: String,
}

#[derive(Clone)]
struct AppState {
    db: Arc<Surreal<any::Any>>,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db =
        any::connect("wss://rust-peru-06cculg9mtssp97io08ca4bg0k.aws-use1.surreal.cloud").await?;

    db.use_ns("demo").use_db("surreal_deal_store").await?;

    db.signin(Root {
        username: "root",
        password: "surrealdb",
    })
    .await?;

    let state = AppState { db: Arc::new(db) };

    let app = Router::new()
        .route("/persons", get(get_persons))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_persons(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<Vec<PersonPartial>> {
    let persons: Vec<PersonPartial> = state
        .db
        .select("person") // 👈 lee todos los registros de la tabla "person"
        .await
        .unwrap_or_default();

    Json(persons)
}
