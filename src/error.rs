use axum::response::{ IntoResponse, Response};
use axum::http::StatusCode;
use crate::response::ApiResponse;

pub type ApiResult<T> = Result<T, ApiError>;
#[derive(Debug, thiserror::Error)]
pub enum ApiError
{
    #[error("Not found")]
    NotFound,
    #[error("method not found")]
    MethodNotAllowed,
    #[error("{0}")]
    Biz(String),
    #[error("Internal server error")]
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