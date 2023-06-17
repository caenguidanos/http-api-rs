use axum::http::StatusCode;
use axum::routing::get;
use axum::{http, Router};
use tower_http::cors::CorsLayer;
use tower_http::trace;

use crate::{contexts, settings, telemetry};

pub struct App;

impl App {
    pub async fn http(settings: settings::Settings) -> Router {
        let ecommerce_http_cx = contexts::ecommerce::HttpContext::new().await;

        Router::new()
            .merge(ecommerce_http_cx.router)
            .route("/healthz", get(|| async { (StatusCode::OK, "OK") }))
            .layer(
                CorsLayer::new()
                    .allow_origin(settings.cors_origin.clone())
                    .allow_methods([http::Method::GET, http::Method::PUT])
                    .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION]),
            )
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(telemetry::setup_http_root_span)
                    .on_request(trace::DefaultOnRequest::new().level(tracing::Level::DEBUG))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::DEBUG)),
            )
            .fallback(|| async { (StatusCode::NOT_FOUND, "NOT_FOUND") })
    }
}
