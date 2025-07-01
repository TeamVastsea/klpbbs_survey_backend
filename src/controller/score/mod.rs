use axum::routing::get;
use axum::Router;

mod submit;
mod query;

pub fn get_submit_routes() -> Router {
    Router::new()
        .route("/", get(query::get_by_user).post(submit::submit).patch(submit::finish))
        .route("/{id}", get(query::get_by_id).post(submit::confirm).patch(submit::rejudge))
        .route("/{id}/export", get(query::export_by_id))
        .route("/search", get(query::search_answer))
}