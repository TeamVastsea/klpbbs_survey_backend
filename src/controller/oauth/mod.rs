use axum::routing::get;
use axum::Router;

pub mod callback;

pub fn get_oauth_routers() -> Router {
    Router::new()
        .route("/", get(callback::oauth_callback))
}