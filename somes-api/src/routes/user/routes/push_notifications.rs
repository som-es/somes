mod settings;
mod token;

use utoipa_axum::{router::OpenApiRouter, routes};

use crate::AppState;
pub use settings::*;
pub use token::*;

pub fn create_push_notification_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(register_push_token_route))
        .routes(routes!(remove_push_token_route))
        .routes(routes!(user_push_tokens_route))
        .routes(routes!(get_notification_settings_route))
        .routes(routes!(update_notification_settings_route))
}
