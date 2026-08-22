pub mod error;
mod oauth;
mod page;
mod ping;
mod question;
mod score;
mod statistics;
mod survey;
mod user;

use axum::Router;
use axum::routing::{delete, get};

pub fn all_routers() -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
        .route("/statistics", get(statistics::get_statistics))
        .route(
            "/user",
            get(user::get_user_info).delete(user::invalidate_token),
        )
        .route("/user/manage", get(user::list_users))
        .route(
            "/user/{other}",
            get(user::get_other_user_info).patch(user::update_user),
        )
        .route(
            "/user/{other}/sessions",
            delete(user::invalidate_user_sessions),
        )
        .nest("/oauth", oauth::get_oauth_routers())
        .nest("/survey", survey::get_survey_routes())
        .nest("/page", page::get_page_routes())
        .nest("/question", question::get_question_routers())
        .nest("/score", score::get_submit_routes())
}
