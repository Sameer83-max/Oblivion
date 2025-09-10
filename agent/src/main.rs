use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "http://localhost:8080")] server: String,
    #[arg(long, default_value = "station-001")] station_id: String,
}

#[derive(Serialize, Deserialize)]
struct Register { station_id: String }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();
    let client = Client::new();

    let url = format!("{}/v1/stations/register", args.server);
    let resp = client.post(&url).json(&Register { station_id: args.station_id.clone() }).send().await?;
    tracing::info!("registered: status={}", resp.status());

    Ok(())
}
