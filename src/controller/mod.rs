mod auth;
pub mod error;
pub mod oauth;
mod question;
mod survey;
mod answer;
mod judge;
mod refresh;
mod ping;
mod admin;

use crate::controller::auth::{get_admin_token_info, get_token, get_user, get_user_info};
use axum::routing::get;
use axum::Router;
use crate::controller::admin::get_admin_info;

pub fn all_routers() -> Router {
    Router::new()
        .route("/", get(ping::ping))
        .route("/user", get(get_user).post(get_token))
        .route("/refresh", get(refresh::refresh_cache))
        .route("/admin", get(get_admin_info))
        .route("/token", get(get_user_info))
        .route("/token/admin", get(get_admin_token_info))
        .nest("/oauth", oauth::get_oauth_routers())
        .nest("/question", question::get_question_routers())
        .nest("/survey", survey::get_survey_routes())
        .nest("/answer", answer::get_answer_routes())
        .nest("/judge", judge::get_judge_routers())
}