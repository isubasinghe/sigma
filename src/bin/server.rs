use capnp::serialize_packed;
use std::net::ToSocketAddrs;
// use std::time::Duration;
use tokio::io::AsyncReadExt;

// use opentelemetry::sdk::export::trace::stdout;
use opentelemetry::sdk::trace::{self, Sampler};
use tracing::{error, info, info_span, warn};

use opentelemetry::sdk::Resource;
use opentelemetry::KeyValue;
use sigma::shared::proto_capnp::*;
use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

const PORT: u16 = 3001;
const MAX_RETRIES: u64 = 10;

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
    info_span!(
        "data",
        "TraceId" = "a2e4f3r12i83rc21",
        "Starting on port {}",
        PORT
    );

    let addr = ("0.0.0.0", PORT).to_socket_addrs().unwrap().next().unwrap();

    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            error!("Unable to bind due to {}", e);
            return;
        }
    };

    loop {
        let (mut sock, _) = match listener.accept().await {
            Ok(v) => v,
            Err(e) => {
                warn!("Unable to accept due to {}, skipping", e);
                continue;
            }
        };
        tokio::spawn(async move {
            let mut retries: u64 = 0;

            loop {
                let sz = match sock.read_u16().await {
                    Ok(sz) => sz,
                    Err(e) => {
                        if retries == MAX_RETRIES {
                            error!("MAX_RETRIES exceeded, dropping client due to {}", e);
                            return;
                        }
                        warn!("Error reading u16 due to {}", e);
                        continue;
                    }
                };
                let sz = usize::from(sz);
                let mut buf: Vec<u8> = Vec::with_capacity(sz);
                // TODO: figure out what this return value is
                let _data = match sock.read_exact(&mut buf).await {
                    Ok(data) => data,
                    Err(e) => {
                        // TODO: drop client if bytes read != sz
                        if retries == MAX_RETRIES {
                            return;
                        }
                        warn!("Unable to read {} bytes due to {}", sz, e);
                        continue;
                    }
                };

                let buf = std::io::BufReader::new(buf.as_slice());
                let message_reader = match serialize_packed::read_message(
                    buf,
                    ::capnp::message::ReaderOptions::new(),
                ) {
                    Ok(msg_reader) => msg_reader,
                    Err(e) => {
                        if retries == MAX_RETRIES {
                            error!(
                                "Unable to setup message reader due to {}, dropping connection",
                                e
                            );
                            return;
                        }
                        warn!("Unable to setup message reader due to {}", e);
                        continue;
                    }
                };
                let request = match message_reader.get_root::<request::Reader>() {
                    Ok(r) => r,
                    Err(e) => {
                        if retries == MAX_RETRIES {
                            error!(
                                "Unable to get message root due to {}, dropping connection",
                                e
                            );
                            return;
                        }
                        warn!("Unable to get message root due to {}", e);
                        continue;
                    }
                };
                let request = match request.which() {
                    Ok(r) => r,
                    Err(e) => {
                        if retries == MAX_RETRIES {
                            error!("Unable to obtain correct union variant due to {}, dropping connection", e);
                            return;
                        }
                        warn!("Unable to obtain correct enum variant due to {}", e);
                        continue;
                    }
                };
                match request {
                    request::Which::RegisterClient(c) => {}
                    request::Which::MakeLock(_l) => {}
                }

                retries = 0;
            }
        });
    }
}
