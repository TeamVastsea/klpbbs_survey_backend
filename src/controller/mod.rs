mod ping;
pub mod error;
mod oauth;
mod user;
mod survey;
mod page;
mod question;
mod score;

use axum::routing::get;
use axum::Router;

pub fn all_routers() -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
        .route("/user", get(user::get_user_info).delete(user::invalidate_token))
        .route("/user/{other}", get(user::get_other_user_info))
        .nest("/oauth", oauth::get_oauth_routers())
        .nest("/survey", survey::get_survey_routes())
        .nest("/page", page::get_page_routes())
        .nest("/question", question::get_question_routers())
        .nest("/score", score::get_submit_routes())
}