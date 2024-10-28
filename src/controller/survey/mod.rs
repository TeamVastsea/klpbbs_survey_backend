use axum::routing::get;
use axum::Router;

mod query;
mod modify;

pub fn get_survey_routes() -> Router {
    Router::new()
        .route("/", get(query::query_surveys).post(modify::create_survey).put(modify::modify_survey))
        .route("/:id", get(query::query_by_id))
}