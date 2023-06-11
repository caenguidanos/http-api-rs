use axum::http::{Request, StatusCode};
use axum::routing::get;
use axum::{http, Router};
use hyper::Body;
use tower_http::cors::CorsLayer;
use tower_http::trace;

use crate::contexts;

pub struct HttpServer;

impl HttpServer {
    pub async fn unsafe_serve(settings: &crate::settings::Settings) {
        let mut app = Router::new()
            .nest("/ecommerce", contexts::ecommerce::HttpContext::unsafe_compose().await)
            .route("/healthz", get(|| async { (StatusCode::OK, "OK") }));

        app = app
            .layer(Self::cors(settings))
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(|request: &Request<Body>| {
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
                    })
                    .on_request(trace::DefaultOnRequest::new().level(tracing::Level::DEBUG))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::DEBUG)),
            )
            .fallback(|| async { (StatusCode::NOT_FOUND, "NOT_FOUND") });

        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], settings.port));
        tracing::debug!("listening on {}", addr);
        axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
    }

    fn cors(settings: &crate::settings::Settings) -> CorsLayer {
        CorsLayer::new()
            .allow_origin(settings.cors_origins.clone())
            .allow_methods([http::Method::GET, http::Method::PUT])
            .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
    }
}
