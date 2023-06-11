use axum::http::{header, StatusCode};
use axum::response::IntoResponse;

pub struct JsonResponse;

impl JsonResponse {
    pub fn with_status<T: serde::Serialize>(status: StatusCode, body: T) -> impl IntoResponse {
        let _e = tracing::debug_span!("Response as JSON").entered();

        let body = serde_json::to_string(&body).unwrap_or("{}".to_string());
        tracing::debug!("{body}");

        (status, [(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())], body)
    }
}
