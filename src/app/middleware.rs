use crate::app::auth::{get_jwt, JWT};
use crate::app::error::ApiError;
use axum::body::Body;
use axum::http::{header, Request, Response};
use std::pin::Pin;
use std::sync::LazyLock;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

static AUTH_LAYER: LazyLock<AsyncRequireAuthorizationLayer<JWTAuth>> = LazyLock::new(||{
    AsyncRequireAuthorizationLayer::new(JWTAuth::new(get_jwt()))
});
#[derive(Debug, Clone)]
pub struct JWTAuth
{
    jwt: &'static JWT
}

// #[derive(Debug)]
impl JWTAuth
{
    pub fn new(jwt: &'static JWT) -> Self
    {
        Self { jwt }
    }
}

impl AsyncAuthorizeRequest<Body> for JWTAuth
{
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = Pin<Box<dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>> + Send + 'static>>;

    fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
        let jwt = self.jwt;
        Box::pin(async move {
            let token = request.headers()
                .get(header::AUTHORIZATION)
                .map(| value| -> Result<_, ApiError> {
                    let token = value.to_str()
                        .map_err(|_| ApiError::Unauthorized(String::from("Authorization请求头不是一个有效的字符串")))?
                        .strip_prefix("Bearer ")
                        .ok_or_else(|| ApiError::Unauthorized(String::from("Authorization请求头必须以Bearer开头")))?;

                    Ok(token)
                })
                .transpose()?
                .ok_or_else(|| ApiError::Unauthorized(String::from("Authorization请求头必须存在")))?;//类似线性代数矩阵的转制

            let principal = jwt.decode(token).map_err(|err| ApiError::Internal(err.into()))?;
            request.extensions_mut().insert(principal);

            Ok(request)
        })


    }
}

pub fn get_auth_layer() -> &'static AsyncRequireAuthorizationLayer<JWTAuth>
{
    &AUTH_LAYER
}

