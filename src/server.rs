use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{http, Router};
use hyper::Body;
use tower_http::cors::CorsLayer;
use tower_http::trace;

use crate::contexts;

pub struct HttpServer;

impl HttpServer {
    pub async fn serve(settings: &crate::settings::Settings) {
        let http_trace_layer = trace::TraceLayer::new_for_http()
            .make_span_with(Self::compose_tracing_root_span_for_http)
            .on_request(trace::DefaultOnRequest::new().level(tracing::Level::DEBUG))
            .on_response(trace::DefaultOnResponse::new().level(tracing::Level::DEBUG));

        let app = Router::new()
            .nest("/ecommerce", contexts::ecommerce::HttpContext::compose().await)
            .route("/healthz", get(Self::handle_healthz))
            .layer(Self::cors(settings))
            .layer(http_trace_layer)
            .fallback(Self::handle_fallback);

        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], settings.port.clone()));
        tracing::debug!("listening on {}", addr);

        axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
    }

    fn cors(settings: &crate::settings::Settings) -> CorsLayer {
        CorsLayer::new()
            .allow_origin(settings.cors_origins.clone())
            .allow_methods([http::Method::GET, http::Method::PUT])
            .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
    }

    fn compose_tracing_root_span_for_http(request: &Request<Body>) -> tracing::Span {
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

    async fn handle_healthz() -> impl IntoResponse {
        (StatusCode::OK, "OK")
    }
    async fn handle_fallback() -> impl IntoResponse {
        (StatusCode::NOT_FOUND, "NOT_FOUND")
    }
}
