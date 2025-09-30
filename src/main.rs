mod logger;
mod config;
mod database;
mod entity;
mod server;
mod app;

use crate::entity::sys_user::Column;
use axum::response::IntoResponse;
use tokio::net::TcpListener;
use axum::{debug_handler, routing, Router, extract::State, http::StatusCode, Json};
use sea_orm::prelude::*;
use entity::prelude::*;
use crate::app::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()>
{

    let router = Router::new()
        .route("/",routing::get(index))
        .route("/users",routing::get(query_users));

    app::run(router).await
}

#[debug_handler] //出错时直接指出错误 棒棒棒
async fn index() -> &'static str
{
    "Hello, Rust!"
}

#[debug_handler]
async fn query_users(State(AppState{ db }): State<AppState>) -> impl IntoResponse
{
    let users = SysUser::find()
        // .filter(Column::Gender.eq("男"))
        .all(&db)
        .await.unwrap();

    axum::Json(users)
}

