use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum::{http::StatusCode, Json};
use lazy_static::lazy_static;
use serde::Serialize;
use std::env;
use std::net::SocketAddr;
use tokio::signal;
use tracer::info;

lazy_static! {
    static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT").map_or(None, |data| Some(
        data.parse().expect("DEPTH_LIMIT is not a number")
    ));
    static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
        .map_or(None, |data| {
            Some(data.parse().expect("COMPLEXITY_LIMIT is not a number"))
        });
}

#[derive(Serialize)]
struct Health {
    healthy: bool,
}

pub(crate) async fn health() -> impl IntoResponse {
    let health = Health { healthy: true };

    (StatusCode::OK, Json(health))
}

#[allow(unused)]
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracer::opentelemetry::global::shutdown_tracer_provider();
}

pub async fn run(port: u16) {
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Service starting at address: {}", address);

    let app = Router::new().route("/", get(health));

    axum_server::bind(address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
