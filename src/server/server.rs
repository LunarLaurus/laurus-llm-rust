use crate::config::AppConfig;
use crate::model::ModelHandle;
use crate::server::routes;
use crate::server::state::AppState;
use anyhow::Result;
use axum::Server as AxumServer;
use std::net::SocketAddr;
use std::sync::Arc;

/// Main server struct, holds config and shared state
pub struct Server {
    pub state: Arc<AppState>,
    pub config: AppConfig,
}

impl Server {
    /// Create a new Server instance
    pub async fn new(config: AppConfig) -> Result<Self> {
        // Load model from path
        let model_handle = ModelHandle::load(&config.model_path)?;
        tracing::info!(model = %config.model_path, "Model loaded successfully");

        // Wrap in AppState and Arc for shared async access
        let state = Arc::new(AppState::new(model_handle));

        Ok(Self { state, config })
    }

    /// Run the server and start listening for requests
    pub async fn run(&self) -> Result<()> {
        let app = routes::routes().with_state(self.state.clone());

        let addr = SocketAddr::from(([0, 0, 0, 0], self.config.port));
        tracing::info!(%addr, "Listening on port {}", self.config.port);

        AxumServer::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
