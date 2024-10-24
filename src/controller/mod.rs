mod ping;
pub mod error;
mod oauth;
mod user;

use axum::{Router};
use axum::routing::get;

pub fn all_routers() -> Router {
    Router::new()
        .route("/", get(ping::ping))
        .route("/user", get(user::get_user_info))
        .route("/user/:other", get(user::get_other_user_info))
        .nest("/oauth", oauth::get_oauth_routers())
}