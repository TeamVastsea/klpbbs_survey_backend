use axum::Router;
use axum::routing::get;

mod callback;

pub fn get_oauth_routers() -> Router {
    Router::new()
        .route("/callback", get(callback::oauth_callback))
}