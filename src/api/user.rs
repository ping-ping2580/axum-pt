use axum::{debug_handler, Router, routing};
use axum::extract::State;
use axum::response::IntoResponse;
use crate::app::AppState;
use crate::entity::prelude::SysUser;
use sea_orm::prelude::*;
use crate::entity::prelude::*;
use crate::entity::sys_user;
use crate::error::ApiResult;
use crate::response::ApiResponse;

pub fn create_user() -> Router<AppState>
{
    Router::new()
        .route("/", routing::get(query_users))
}


#[debug_handler]
async fn query_users(State(AppState{ db }): State<AppState>) -> ApiResult<ApiResponse<Vec<sys_user::Model>>>
{
    let users = SysUser::find()
        // .filter(Column::Gender.eq("男"))
        .all(&db)
        .await.unwrap();

    Ok(ApiResponse::ok("ok", Some(users)))
}
