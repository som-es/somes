mod delegate;
use axum::{
    routing::{delete, get, post},
    Router,
};
pub use delegate::*;

mod vote_result;
use somes_common_lib::{DELEGATE, VOTE_RESULT};
pub use vote_result::*;

use crate::server::AppState;

pub fn create_bookmark_router() -> Router<AppState> {
    Router::new()
        .route(DELEGATE, post(add_user_delegate_bookmark))
        .route(DELEGATE, get(delegate_bookmarks_by_user))
        .route(DELEGATE, delete(remove_user_delegate_bookmark))
        .route(VOTE_RESULT, delete(remove_user_vote_result_bookmark))
        .route(VOTE_RESULT, get(user_vote_result_booksmarks))
        .route(VOTE_RESULT, post(add_user_vote_result_bookmark))
}
