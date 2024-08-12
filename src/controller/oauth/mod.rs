use axum::Router;
use axum::routing::get;

pub mod callback;

pub fn get_oauth_routers() -> Router {
    Router::new()
        .route("/", get(callback::oauth_callback))
}