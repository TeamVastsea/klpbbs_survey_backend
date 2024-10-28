use axum::routing::get;
use axum::Router;

mod submit;
mod query;

pub fn get_submit_routes() -> Router {
    Router::new()
        .route("/", get(query::get_by_user).post(submit::submit).patch(submit::finish))
}