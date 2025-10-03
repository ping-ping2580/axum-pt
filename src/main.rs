mod logger;
mod config;
mod database;
mod entity;
mod server;
mod app;
mod api;
mod error;
mod response;
mod Latency;
mod common;
mod serde;

use crate::api::create_router;
use axum::response::IntoResponse;
use sea_orm::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()>
{
    app::run(api::create_router()).await
}

