use axum::Router;
use std::net::SocketAddr;

pub async fn run_server(app: Router) -> anyhow::Result<()> {
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
