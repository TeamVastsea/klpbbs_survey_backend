use axum::routing::get;
use axum::Router;

mod submit;
mod query;
mod summarize;

pub fn get_submit_routes() -> Router {
    Router::new()
        .route("/", get(query::get_by_user).post(submit::submit).patch(submit::finish))
        .route("/:id", get(query::get_by_id).post(submit::confirm).patch(submit::rejudge))
        .route("/search", get(query::search_answer))
}