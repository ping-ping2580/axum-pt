use axum::response::{ IntoResponse, Response};
use axum::http::StatusCode;
use sea_orm::sqlx;
use crate::response::ApiResponse;

pub type ApiResult<T> = Result<T, ApiError>;
#[derive(Debug, thiserror::Error)]
pub enum ApiError
{
    #[error("服务器迷路了")]
    NotFound,
    #[error("数据库异常: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("请求方法不支持")]
    MethodNotAllowed,
    #[error("{0}")]
    Biz(String),
    #[error("错误: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ApiError
{
    pub fn status_code(&self) -> StatusCode
    {
        match self
        {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz(_) => StatusCode::OK,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError
{
    fn into_response(self) -> Response
    {
        let status_code = self.status_code();
        let body = axum::Json(ApiResponse::<()>::err(self.to_string()));

        (status_code, body).into_response()
    }
}