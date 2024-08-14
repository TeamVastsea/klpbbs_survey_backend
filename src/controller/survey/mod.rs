mod query;

use axum::Router;
use axum::routing::get;

pub fn get_survey_routes() -> Router {
    Router::new()
        .route("/", get(query::query_surveys))
        .route("/:id", get(query::query_by_id))
}