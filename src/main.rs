#![feature(result_option_inspect)]

mod app;
mod contexts;
mod libs;
mod settings;
mod telemetry;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let settings = settings::Settings::new();

    if settings.telemetry_enabled {
        telemetry::init_service("http:api");
    }

    std::panic::set_hook(Box::new(tracing_panic::panic_hook));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], settings.port));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app::App::build(&settings).await.into_make_service())
        .await
        .unwrap();
}
