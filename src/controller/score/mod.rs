use axum::Router;
use axum::routing::{get, post};

mod submit;
mod query;

pub fn get_submit_routes() -> Router {
    Router::new()
        .route("/", get(query::get_by_user).post(submit::submit).patch(submit::finish))
}