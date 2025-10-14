use crate::app::error::ApiError;
use axum::extract::FromRequestParts;
use axum_valid::HasValidate;

#[derive(Debug, Clone, Default, FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(ApiError))]
pub struct Path<T>(pub T);

impl<T> HasValidate  for Path<T>
{
    type Validate = T;
    fn get_validate(&self) -> &Self::Validate
    {
        &self.0
    }
}