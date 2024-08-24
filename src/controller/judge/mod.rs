mod start;

use axum::routing::get;
use axum::Router;

pub fn get_judge_routers() -> Router {
    Router::new()
        .route("/", get(start::auto_judge))
}