mod auth;
pub mod error;
mod oauth;

use axum::{Router};
use axum::routing::get;
use crate::controller::auth::{get_token, get_user_id};

pub fn all_routers() -> Router {
    Router::new()
        .route("/user", get(get_user_id).post(get_token))
        .nest("/oauth", oauth::get_oauth_routers())
}