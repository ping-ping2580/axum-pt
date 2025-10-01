use axum::Router;
use crate::app::AppState;
use crate::error::{ApiError, ApiResult};

mod user;

pub fn create_router() -> Router<AppState>
{
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/users", user::create_user())
                .fallback(async || -> ApiResult<()>
                {
                    tracing::warn!("Not found");
                    Err(ApiError::NotFound)
                })
                .method_not_allowed_fallback(async || -> ApiResult<()>{
                        tracing::warn!("Not found");
                        Err(ApiError::MethodNotAllowed)
                })
        )
}

