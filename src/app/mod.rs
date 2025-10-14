use axum::Router;
use sea_orm::DatabaseConnection;
use crate::config;
// use tracing::log::logger;

mod logger;
mod database;
mod server;
pub mod response;
pub mod error;

pub mod latency;
pub mod common;
pub mod serde;
pub mod query;
pub mod path;
pub mod json;
pub mod valid;
pub mod validation;
pub mod id;
pub mod enumeration;
pub mod auth;
pub mod middleware;
pub mod utils;

#[derive(Clone)]
pub struct AppState
{
    pub db: DatabaseConnection,
}

impl AppState
{
    pub fn new(db: DatabaseConnection) -> Self
    {
        Self { db }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()>
{
    logger::init();
    id::init()?;
    tracing::info!("Starting app.server..");

    let db = database::init().await?;
    let state = AppState::new(db);
    let server = server::Server::new(config::get().server());

    server.start(state, router).await
}