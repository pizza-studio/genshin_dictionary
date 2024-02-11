use axum::{routing::get, Extension, Router};

use sqlx::PgPool;

use tower_http::cors::CorsLayer;

use super::search_dictionary::search_dictionary;

pub fn app(db: PgPool) -> Router {
    Router::new()
        .route("/:version/translations/:query", get(search_dictionary))
        .layer(Extension(db))
        .layer(CorsLayer::permissive())
}
