use capnp::serialize_packed;
use message_io::network::{Endpoint, NetEvent, Transport};
use message_io::node;
use sigma::shared::server::*;
use std::net::ToSocketAddrs;
use std::time::Duration;

use opentelemetry::sdk::export::trace::stdout;
use opentelemetry::sdk::trace::{self, Sampler};
use tracing::{error, info, info_span, warn};

use opentelemetry::sdk::Resource;
use opentelemetry::KeyValue;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

const PORT: u16 = 3001;

fn handle_client() {}
#[tokio::main]
async fn main() {
    // Create a new OpenTelemetry pipeline
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "sigma_server",
                )])),
        )
        .install_simple()
        .unwrap(); // stdout::new_pipeline().install_simple();


    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();
    info_span!("data","TraceId" = "a2e4f3r12i83rc21", "Starting on port {}", PORT);


    let addr = ("0.0.0.0", PORT).to_socket_addrs().unwrap().next().unwrap();
    let (handler, listener) = node::split::<()>();
    let (_srv, _hnd) = Server::<()>::new();

    handler
        .network()
        .listen(Transport::FramedTcp, addr)
        .unwrap();
    listener.for_each(move |event| match event.network() {
        NetEvent::Connected(_, _) => {}
        NetEvent::Accepted(endpoint, _) => {
            info!("Accepted connection by {}", endpoint);
        }
        NetEvent::Message(endpoint, data) => {
            let msg_reader = match serialize_packed::read_message(
                data,
                ::capnp::message::ReaderOptions::new(),
            ) {
                Ok(msg) => msg,
                Err(err) => {
                    error!("Could not read message by {} due to {}", endpoint, err);
                    return;
                }
            };
        }
        NetEvent::Disconnected(endpoint) => {
            warn!("Client {} has disconnected", endpoint);
        }
    });
}
