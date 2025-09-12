// src/db.rs
use std::sync::Arc;
use surrealdb::{engine::any, opt::auth::Root, Surreal};

use crate::config::DatabaseConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Surreal<any::Any>>,
}

pub async fn init_db(cfg: &DatabaseConfig) -> anyhow::Result<AppState> {
    let db = any::connect(&cfg.url).await?;

    db.use_ns(&cfg.namespace).use_db(&cfg.name).await?;
    db.signin(Root {
        username: &cfg.user,
        password: &cfg.pass,
    })
    .await?;

    Ok(AppState { db: Arc::new(db) })
}
