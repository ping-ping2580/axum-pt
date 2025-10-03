use axum::{debug_handler, Router, routing};
use axum::extract::State;
use axum::response::IntoResponse;
use crate::app::AppState;
use crate::entity::prelude::SysUser;
use sea_orm::prelude::*;
use sea_orm::{QueryOrder, QueryTrait, Condition};
use serde::Deserialize;
use crate::common::{Page, PaginationParams};
use crate::entity::prelude::*;
use crate::entity::sys_user;
use crate::error::ApiResult;
use crate::response::ApiResponse;
use axum::extract::Query;

pub fn create_user() -> Router<AppState>
{
    Router::new()
        .route("/", routing::get(find_page))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams
{
    keyword: Option<String>,
    #[serde(flatten)]
    pagination: PaginationParams
}

#[debug_handler]
async fn find_page(
    State(AppState{ db }): State<AppState>,
    Query(UserQueryParams
    {
              keyword,
              pagination,
    }): Query<UserQueryParams>
) -> ApiResult<ApiResponse<Page<sys_user::Model>>>
{
    let paginator = SysUser::find()
        .apply_if(keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any()
                    .add(sys_user::Column::Name.contains(keyword))
                    .add(sys_user::Column::Account.contains(keyword))
            )
        })
        .order_by_desc(sys_user::Column::CreatedAt)
        .paginate(&db, pagination.size);

    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(pagination.page - 1).await?;

    let page = Page::from_pagination(pagination, total, items);

    Ok(ApiResponse::ok("ok", Some(page)))
}
