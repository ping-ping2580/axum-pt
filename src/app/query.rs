use crate::app::error::ApiError;
use axum::extract::FromRequestParts;
use axum_valid::HasValidate;

#[derive(Debug, Clone, Copy, Default, FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(ApiError))]
pub struct Query<T> (pub T);

impl<T> HasValidate for Query<T>
{
    type Validate = T;

    fn get_validate(&self) -> &Self::Validate
    {
        &self.0
    }

}