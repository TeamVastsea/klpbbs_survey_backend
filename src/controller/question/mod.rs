use axum::Router;
use axum::routing::get;

mod query;
mod modify;

pub fn get_question_routers() -> Router {
    Router::new()
        .route("/", get(query::get_question_by_page).post(modify::new_question)
            .put(modify::modify_question).patch(modify::swap_question))
        .route("/:question", get(query::get_question))
}