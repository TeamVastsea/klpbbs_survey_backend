use axum::Router;
use axum::routing::get;

mod modify;
mod query;

pub fn get_page_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(query::get_page_by_index)
                .post(modify::new_page)
                .put(modify::modify_page),
        )
        .route("/{id}", axum::routing::delete(modify::delete_page))
}
