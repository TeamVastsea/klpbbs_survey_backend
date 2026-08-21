use axum::Router;
use axum::routing::get;

mod modify;
mod query;

pub fn get_question_routers() -> Router {
    Router::new()
        .route(
            "/",
            get(query::get_question_by_page)
                .post(modify::new_question)
                .put(modify::modify_question)
                .patch(modify::swap_question),
        )
        .route("/{id}", axum::routing::delete(modify::delete_question))
}
