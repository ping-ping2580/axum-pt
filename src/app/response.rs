use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T>
{
    pub code: i32,
    pub msg: String,
    // #[serde(skip_serializing_if = "Option::is_some")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
{
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self
    {
        Self{ code, msg, data }
    }

    pub fn ok<M: AsRef<str>>(msg: M, data: Option<T>) -> Self
    {
        Self::new(0, String::from(msg.as_ref()), data)
    }

    pub fn err<M: AsRef<str>>(msg: M) -> Self
    {
        Self::new(1, String::from(msg.as_ref()), None)
    }

}

impl<T: Serialize> IntoResponse for ApiResponse<T>
{
    fn into_response(self) -> Response
    {
        axum::Json(self).into_response()
    }
}
