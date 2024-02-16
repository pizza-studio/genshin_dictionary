use axum::{body::Body, extract::Request, routing::get, Extension, Router};

use sqlx::PgPool;

use tower_http::{
    cors::CorsLayer,
    trace::{
        DefaultMakeSpan, DefaultOnFailure, DefaultOnResponse,
        TraceLayer,
    },
    LatencyUnit,
};
use tracing::{Level, Span};

use super::search_dictionary::search_dictionary;

pub fn app(db: PgPool) -> Router {
    Router::new()
        .route("/:version/translations/:query", get(search_dictionary))
        .layer(Extension(db))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(|request: &Request<Body>, _span: &Span| {
                    let uri = urlencoding::decode(request.uri().path())
                        .unwrap_or(request.uri().path().into());
                    tracing::info!("started {} {}", request.method(), uri)
                })
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                )
                .on_failure(
                    DefaultOnFailure::new()
                        .level(Level::ERROR)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
}
