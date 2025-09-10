use axum::{routing::{get, post}, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize, Deserialize)]
struct Health { status: &'static str }

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/health", get(|| async { Json(Health { status: "ok" }) }))
        .route("/v1/stations/register", post(dummy))
        .route("/v1/jobs", post(dummy))
        .route("/v1/results", post(dummy));

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    tracing::info!("server listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn dummy(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"ok": true, "echo": payload}))
}
