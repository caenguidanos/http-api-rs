use axum::http::HeaderValue;

pub struct Settings {
    pub port: u16,
    pub cors_origins: Vec<HeaderValue>,
}

impl Settings {
    pub fn new() -> Self {
        let cors_origins: Vec<HeaderValue> = std::env::var("CORS_ORIGIN")
            .expect("CORS_ORIGIN")
            .split(',')
            .map(|origin| origin.parse::<HeaderValue>().expect("Valid CORS origin value"))
            .collect();

        Self {
            port: 8080,
            cors_origins,
        }
    }
}
