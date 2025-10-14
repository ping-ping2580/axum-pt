use std::net::SocketAddr;
use axum::{debug_handler, routing, Extension, Router};
use axum::extract::{ConnectInfo, State};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::app::AppState;
use crate::app::error::{ApiError, ApiResult};
use crate::app::middleware::get_auth_layer;
use crate::app::valid::ValidJson;
use crate::entity::prelude::*;
use sea_orm::prelude::*;
use crate::app::auth::Principal;
use crate::app::utils::varify_password;
use crate::entity::sys_user;
use crate::app::auth::get_jwt;
use crate::app::response::ApiResponse;

pub fn create_router() ->  Router<AppState>
{
    Router::new()
        .route("/user-info", routing::get(get_user_info))
        .route_layer(get_auth_layer())
        .route("/login", routing::post(login))
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct LoginParams
{
    #[validate(length(min = 3, max = 16, message = "用户名长度为3-16"))]
    pub account: String,
    #[validate(length(min = 6, max = 16, message = "密码长度为6-16"))]
    pub password: String,
}

#[derive(Deserialize, Debug, Clone, Validate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult
{
    pub access_token: String,
}

#[debug_handler]
#[tracing::instrument(name = "login", skip_all, fields(account = %params.account, ip = %addr.ip()))]
async fn login(
    State(AppState{ db }): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<LoginParams>,
) -> ApiResult<ApiResponse<LoginResult>>
{
    tracing::info!("开始处理登陆逻辑...");
    let user = SysUser::find()
        .filter(sys_user::Column::Account.eq(&params.account))
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("账户或密码不正确")))?;

    if !varify_password(&params.password, &user.password)?
    {
        return Err(ApiError::Biz(String::from("账户或密码不正确")));
    }

    let principal = Principal
    {
        id: user.id,
        name: user.name,
    };

    let access_token = get_jwt().encode(principal)?;

    tracing::info!("登陆成功, JWT token{access_token}");

    Ok(ApiResponse::ok("登陆成功", Some(LoginResult { access_token })))
}

#[debug_handler]
async fn get_user_info(
    // State(AppState{ db }): State<AppState>,
    Extension(principal): Extension<Principal>,
) -> ApiResult<ApiResponse<Principal>>
{
    Ok(ApiResponse::ok("ok", Some(principal)))
}