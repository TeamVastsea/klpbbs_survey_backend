use axum::Router;
use axum::routing::get;

mod page;
mod question;

pub fn get_question_routers() -> Router {
    Router::new()
        .route("/", get(page::get_page))
        .route("/:question", get(question::get_question))
}