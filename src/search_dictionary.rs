use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_valid::Valid;
use crud::query_dictionary;
use hyper::StatusCode;
use model::QueryResult;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{error, info};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Validate)]
pub struct SearchParams {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1))]
    pub page_size: u64,
}

pub async fn search_dictionary(
    Valid(Query(SearchParams { page_size, page })): Valid<Query<SearchParams>>,
    Path((version, query)): Path<(String, String)>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<QueryResult>, (StatusCode, &'static str)> {
    info!(?page_size, ?page, "Searching '{}'. ", &query);

    match &*version {
        "v1" => {
            if !query.is_empty() {
                query_dictionary(&query, page_size, page, &db)
                    .await
                    .map(Json)
                    .map_err(|err| {
                        error!("{:?}", err);
                        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error. ")
                    })
            } else {
                Err((StatusCode::BAD_REQUEST, "Query Word should not be empty"))
            }
        }
        _ => Err((StatusCode::BAD_REQUEST, "API version invalid. ")),
    }
}
