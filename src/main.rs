mod config;
mod model;
mod server;

use crate::config::AppConfig;
use crate::server::Server;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = AppConfig::from_env();
    info!(model=%config.model_path, "Starting server");

    let server = Server::new(config).await?;
    server.run().await?;

    Ok(())
}
