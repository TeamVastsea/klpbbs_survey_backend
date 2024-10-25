mod ping;
pub mod error;
mod oauth;
mod user;
mod survey;

use axum::{Router};
use axum::routing::get;

pub fn all_routers() -> Router {
    Router::new()
        .route("/", get(ping::ping))
        .route("/user", get(user::get_user_info).delete(user::invalidate_token))
        .route("/user/:other", get(user::get_other_user_info))
        .nest("/oauth", oauth::get_oauth_routers())
        .nest("/survey", survey::get_survey_routes())
}