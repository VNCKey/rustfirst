mod app;
mod config;
mod db;

mod presentation;
mod server;

use app::build_app;
use config::load_config;
use db::init_db;
use server::run_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = load_config()?;
    println!("Conectando a: {}", settings.database.url);

    let state = init_db(&settings.database).await?;
    let app = build_app(state);

    run_server(app).await?;

    Ok(())
}
