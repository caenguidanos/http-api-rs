use axum::http::Request;
use hyper::Body;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::sdk::trace;
use opentelemetry::sdk::trace::{RandomIdGenerator, Sampler};
use opentelemetry::sdk::Resource;
use opentelemetry_semantic_conventions::resource;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn setup_tracing(service_name: &'static str) {
    let service_name_resource = Resource::new(vec![opentelemetry::KeyValue::new(resource::SERVICE_NAME, service_name)]);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(service_name_resource),
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

pub fn setup_http_root_span(request: &Request<Body>) -> tracing::Span {
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
