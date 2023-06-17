#![feature(result_option_inspect)]

mod app;
mod contexts;
mod libs;
mod settings;
mod telemetry;

#[tokio::main]
async fn main() {
    dotenv::from_filename("config/env/.env").ok();

    std::panic::set_hook(Box::new(tracing_panic::panic_hook));

    let settings = settings::Settings::new();

    if settings.telemetry_enabled {
        telemetry::setup_tracing("http:api");
        tracing::debug!("telemetry enabled");
    }

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app::App::http(settings).await.into_make_service())
        .await
        .unwrap();
}
