mod start;

use axum::Router;
use axum::routing::get;

pub fn get_judge_routers() -> Router {
    Router::new()
        .route("/", get(start::auto_judge))
}