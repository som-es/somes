mod settings;
mod token;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use somes_common_lib::{PUSH_SETTINGS, PUSH_TOKEN, PUSH_TOKENS};

use crate::AppState;
pub use settings::*;
pub use token::*;

pub fn create_push_notification_router() -> Router<AppState> {
    Router::new()
        .route(PUSH_TOKEN, post(register_push_token_route))
        .route(PUSH_TOKEN, delete(remove_push_token_route))
        .route(PUSH_TOKENS, get(user_push_tokens_route))
        .route(PUSH_SETTINGS, get(get_notification_settings_route))
        .route(PUSH_SETTINGS, put(update_notification_settings_route))
}
