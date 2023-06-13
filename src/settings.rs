use axum::http::HeaderValue;

pub struct Settings {
    pub port: u16,
    pub cors_origins: Vec<HeaderValue>,
    pub telemetry_enabled: bool,
}

impl Settings {
    pub fn new() -> Self {
        let cors_origins: Vec<HeaderValue> = std::env::var("CORS_ORIGIN")
            .expect("CORS_ORIGIN")
            .split(',')
            .map(|origin| origin.parse::<HeaderValue>().expect("Valid CORS origin value"))
            .collect();

        let telemetry_enabled: bool = std::env::var("TELEMETRY_ENABLED")
            .unwrap_or(String::from("false"))
            .parse::<bool>()
            .unwrap_or(false);

        Self {
            port: 8080,
            cors_origins,
            telemetry_enabled,
        }
    }
}
