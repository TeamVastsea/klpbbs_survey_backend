mod auth;
pub mod error;
pub mod oauth;
mod question;

use axum::{Router};
use axum::routing::get;
use crate::controller::auth::{get_token, get_user};

pub fn all_routers() -> Router {
    Router::new()
        .route("/user", get(get_user).post(get_token))
        .nest("/oauth", oauth::get_oauth_routers())
        .nest("/question", question::get_question_routers())
}