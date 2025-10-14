use crate::app::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_valid::ValidRejection;

pub type ApiResult<T> = Result<T, ApiError>;
#[derive(Debug, thiserror::Error)]
pub enum ApiError
{
    #[error("服务器迷路了")]
    NotFound,
    #[error("请求方法不支持")]
    MethodNotAllowed,
    #[error("数据库异常: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("查询参数错: {0}")]
    Query(#[from] QueryRejection),
    #[error("路径参数错误: {0}")]
    Path(#[from] PathRejection),
    #[error("body解析错误: {0}")]
    Json(#[from] JsonRejection),
    #[error("参数校验失败: {0}")]
    Validation(String),
    #[error("JWT错误: {0}")]
    JWT(#[from] jsonwebtoken::errors::Error),
    #[error("密码hash错误: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("未授权: {0}")]
    Unauthorized(String),
    #[error("{0}")]
    Biz(String),
    #[error("错误: {0}")]
    Internal(#[from] anyhow::Error),
}

impl From<ValidRejection<ApiError>> for ApiError
{
    fn from(value: ValidRejection<ApiError>) -> Self
    {
        match value
        {
            ValidRejection::Valid(errors) => ApiError::Validation(errors.to_string()),
            ValidRejection::Inner(error) => error
        }
    }
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
            ApiError::JWT(_) | ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Internal(_) | ApiError::DatabaseError(_) |  ApiError::Bcrypt(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Query(_) | ApiError::Json(_) | ApiError::Path(_) | ApiError::Validation(_)=> StatusCode::BAD_REQUEST,
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

impl From<ApiError> for Response
{
    fn from(value: ApiError) -> Self
    {
        value.into_response()
    }
}