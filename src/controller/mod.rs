mod auth;
pub mod error;
pub mod oauth;
mod question;
mod survey;
mod answer;
mod judge;
mod refresh;

use axum::{Router};
use axum::routing::get;
use crate::controller::auth::{get_token, get_user};

pub fn all_routers() -> Router {
    Router::new()
        .route("/user", get(get_user).post(get_token))
        .route("/refresh", get(refresh::refresh_cache))
        .nest("/oauth", oauth::get_oauth_routers())
        .nest("/question", question::get_question_routers())
        .nest("/survey", survey::get_survey_routes())
        .nest("/answer", answer::get_answer_routes())
        .nest("/judge", judge::get_judge_routers())
}