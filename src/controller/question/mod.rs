use axum::routing::get;
use axum::Router;

mod page;
mod question;
mod modify;

pub fn get_question_routers() -> Router {
    Router::new()
        .route("/", get(page::get_page).post(modify::new_question).put(modify::modify_question))
        .route("/:question", get(question::get_question))
}