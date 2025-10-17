use crate::model::ModelHandle;
use crate::server::state::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;

#[derive(Deserialize)]
pub struct SwitchModelRequest {
    pub model_path: String,
}

#[derive(Serialize)]
pub struct GenerateResponse {
    pub text: String,
}

pub struct SwitchModelHandler;

impl SwitchModelHandler {
    pub async fn handle(
        State(state): State<AppState>,
        Json(req): Json<SwitchModelRequest>,
    ) -> Json<GenerateResponse> {
        if state.blocking_reload.swap(true, Ordering::SeqCst) {
            return Json(GenerateResponse {
                text: "Reload already in progress".into(),
            });
        }

        while state.active_requests.load(Ordering::SeqCst) > 0 {
            state.notify_no_active.notified().await;
        }

        {
            let mut guard = state.shared.write();
            match ModelHandle::load(&req.model_path) {
                Ok(new_model) => *guard = new_model,
                Err(e) => {
                    state.blocking_reload.store(false, Ordering::SeqCst);
                    state.notify_reload_done.notify_waiters();
                    return Json(GenerateResponse {
                        text: format!("Failed to switch model: {:?}", e),
                    });
                }
            }
        }

        state.blocking_reload.store(false, Ordering::SeqCst);
        state.notify_reload_done.notify_waiters();
        Json(GenerateResponse {
            text: "Model switched successfully".into(),
        })
    }
}
