mod ping;
pub mod error;

use axum::{Router};
use axum::routing::get;

pub fn all_routers() -> Router {
    Router::new()
        .route("/", get(ping::ping))
}