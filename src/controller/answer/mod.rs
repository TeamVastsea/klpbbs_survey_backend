mod check;
mod submit;

use axum::Router;
use axum::routing::get;

pub fn get_answer_routes() -> Router {
    Router::new()
        .route("/", get(check::check_record).post(submit::submit_answer))
}