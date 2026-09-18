mod delegate;
pub use delegate::*;
use utoipa_axum::{router::OpenApiRouter, routes};

mod vote_result;
pub use vote_result::*;

use crate::AppState;

pub fn create_bookmark_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(add_user_delegate_bookmark))
        .routes(routes!(update_user_delegate_bookmark))
        .routes(routes!(delegate_bookmarks_by_user))
        .routes(routes!(remove_user_delegate_bookmark))
        .routes(routes!(remove_user_vote_result_bookmark))
        .routes(routes!(user_vote_result_booksmarks))
        .routes(routes!(add_user_vote_result_bookmark))
}
