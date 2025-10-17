use crate::server::state::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use std::collections::HashMap;

pub struct ModesHandler;

impl ModesHandler {
    /// List all modes
    pub async fn list(State(state): State<AppState>) -> Json<HashMap<String, String>> {
        let modes = state.modes.read().clone();
        Json(modes)
    }

    /// Add or update a mode
    pub async fn add(
        State(state): State<AppState>,
        Json(req): Json<ModeRequest>,
    ) -> Json<ModeResponse> {
        state.set_mode(req.name.clone(), req.system_prompt.clone());
        Json(ModeResponse {
            name: req.name,
            system_prompt: req.system_prompt,
        })
    }

    /// Delete a mode
    pub async fn delete(
        State(state): State<AppState>,
        Path(name): Path<String>,
    ) -> Json<serde_json::Value> {
        let removed = state.modes.write().remove(&name);
        if removed.is_some() {
            Json(serde_json::json!({"status": "ok", "removed": name}))
        } else {
            Json(serde_json::json!({"status": "not_found", "name": name}))
        }
    }
}
