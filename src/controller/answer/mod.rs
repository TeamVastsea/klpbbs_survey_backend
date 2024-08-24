mod check;
mod submit;
mod get;
mod search;

use axum::routing::get;
use axum::Router;

pub fn get_answer_routes() -> Router {
    Router::new()
        .route("/", get(check::check_record).post(submit::submit_answer))
        .route("/:id", get(get::get_answer))
        .route("/search", get(search::search_answer))
}