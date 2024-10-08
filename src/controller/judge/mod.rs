mod start;
mod confirm;

use axum::routing::get;
use axum::Router;

pub fn get_judge_routers() -> Router {
    Router::new()
        .route("/", get(start::auto_judge).post(confirm::confirm_judge))
}