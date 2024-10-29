use axum::routing::get;
use axum::Router;

mod query;
mod modify;

pub fn get_page_routes() -> Router {
    Router::new()
        .route("/", get(query::get_page_by_index).post(modify::new_page).put(modify::modify_page))
    // .route("/:id", get(query::get_page))
}