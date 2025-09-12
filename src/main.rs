use std::net::SocketAddr;

use surrealdb::sql::Thing;

use std::sync::Arc;
// USAMOS AXUM
use axum::{routing::get, Json, Router};
// PARA EL JSON
use serde::{Serialize, Deserialize};
// SWAGGER
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

// TOKIO
use tokio;

// CONECTARSE CON SURREALDB CLOUD 
use surrealdb::Surreal;
use surrealdb::engine::any;
use surrealdb::opt::auth::Root;


use config::{Config, File};


#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    namespace: String,
    name: String,
    user: String,
    pass: String,
}

#[derive(Debug, Deserialize)]
struct Settings {
    database: DatabaseConfig,
}


fn load_config() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("config/default")) // base
        .add_source(File::with_name("config/production").required(false)) // si existe
        .add_source(config::Environment::default().separator("__")) // sobrescribir con env vars
        .build()?;

    settings.try_deserialize()
}



#[derive(Clone)]
struct AppState {
    db: Arc<Surreal<any::Any>>,
}




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
	paths(hello, get_productos), 
	components(schemas(HelloResponse, ProductoPartial)),
	tags(
		(name = "Ejemplo", description = "API de ejemplo"),
		(name = "Productos", description = "Operaciones relacionadas con productos")
	)
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {


		let settings = load_config()?;

    println!("Conectando a: {}", settings.database.url);
    println!("Namespace: {}", settings.database.namespace);
    println!("DB: {}", settings.database.name);


		let db =
        any::connect(&settings.database.url).await?;

		db.use_ns(&settings.database.namespace).use_db(&settings.database.name).await?;
		db.signin(Root {
        username: &settings.database.user,
        password: &settings.database.pass,
    })
    .await?;

		let state = AppState {
				db: Arc::new(db),
		};
    let socket_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();

    let app = Router::new()
        .route("/hello", get(hello))
				.route("/productos", get(get_productos))
				.with_state(state)
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()));

    println!("🚀 Servidor corriendo en http://{}", socket_address);
    println!("📑 Swagger UI en http://{}/docs", socket_address);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

		Ok(())
}



#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct RecordId {
    tb: String,
    id: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct ProductoPartial {
		#[schema(value_type =String)]
    categoria_id: Thing,
    nombre: String,
    descripcion: String,
    images: String,
    precio: serde_json::Value,
    // Ajusta el tipo de time según lo que recibas realmente
}


#[utoipa::path(
    get,
    path = "/productos",
    responses(
        (status = 200, description = "Lista de productos", body = [ProductoPartial])
    )
)]

async fn get_productos(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<Vec<ProductoPartial>> {
    let productos: Vec<ProductoPartial> = state
        .db
        .select("productos")
        .await
        .unwrap_or_default();

    // Imprime el primer producto para ver la estructura real de los campos
    if let Some(p) = productos.first() {
        println!("{:#?}", p);
    }

    Json(productos)
}
