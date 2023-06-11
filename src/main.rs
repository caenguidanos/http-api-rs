#![feature(result_option_inspect)]

use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::sdk::trace;
use opentelemetry::sdk::trace::{RandomIdGenerator, Sampler};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod contexts;
mod libs;
mod server;
mod settings;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let settings = settings::Settings::new();

    init_telemetry();

    std::panic::set_hook(Box::new(tracing_panic::panic_hook));

    server::HttpServer::unsafe_serve(&settings).await;
}

fn init_telemetry() {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default()),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(telemetry)
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
}
