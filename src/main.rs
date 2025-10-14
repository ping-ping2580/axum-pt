mod api;
mod app;
mod config;
mod entity;
// use axum::response::IntoResponse;
// use sea_orm::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()>
{
    app::run(api::create_router()).await
}

