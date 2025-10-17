use crate::server::state::AppState;
use axum::{extract::State, Json};
use serde::Serialize;
use std::sync::atomic::Ordering;

#[derive(Serialize)]
pub struct StatusResponse {
    pub model_path: String,
    pub active_requests: usize,
    pub reloading: bool,
}

pub struct StatusHandler;

impl StatusHandler {
    pub async fn handle(State(state): State<AppState>) -> Json<StatusResponse> {
        let guard = state.shared.read();
        Json(StatusResponse {
            model_path: guard.model_path.clone(),
            active_requests: state.active_requests.load(Ordering::SeqCst),
            reloading: state.blocking_reload.load(Ordering::SeqCst),
        })
    }
}
