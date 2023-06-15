use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{http, Router};
use hyper::Body;
use tower_http::cors::CorsLayer;
use tower_http::trace;

use crate::contexts;

pub struct App;

impl App {
    pub async fn build(settings: &crate::settings::Settings) -> Router {
        let http_trace_layer = trace::TraceLayer::new_for_http()
            .make_span_with(Self::compose_http_span)
            .on_request(trace::DefaultOnRequest::new().level(tracing::Level::DEBUG))
            .on_response(trace::DefaultOnResponse::new().level(tracing::Level::DEBUG));

        let ecommerce_http_cx = contexts::ecommerce::HttpContext::new().await;

        Router::new()
            .merge(ecommerce_http_cx.router)
            .route("/healthz", get(Self::handle_healthz))
            .layer(Self::cors(settings))
            .layer(http_trace_layer)
            .fallback(Self::handle_fallback)
    }

    fn cors(settings: &crate::settings::Settings) -> CorsLayer {
        CorsLayer::new()
            .allow_origin(settings.cors_origins.clone())
            .allow_methods([http::Method::GET, http::Method::PUT])
            .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
    }

    async fn handle_healthz() -> impl IntoResponse {
        (StatusCode::OK, "OK")
    }
    async fn handle_fallback() -> impl IntoResponse {
        (StatusCode::NOT_FOUND, "NOT_FOUND")
    }

    fn compose_http_span(request: &Request<Body>) -> tracing::Span {
        let otel_name = format!("{} {}", request.method(), request.uri());

        tracing::span!(
            tracing::Level::INFO,
            "request",
            http.method = %request.method(),
            http.uri = %request.uri(),
            http.version = ?request.version(),
            http.request.headers = ?request.headers(),
            otel.name = %otel_name,
        )
    }
}
