use crate::server::state::AppState;
use anyhow::Result;
use async_stream::stream;
use axum::{
    extract::State,
    response::sse::{Event, Sse},
    Json,
};
use futures::stream::Stream;
use serde::Deserialize;
use std::sync::atomic::Ordering;
use tokio::task;
use tracing::error;

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub prompt: String,
    pub max_tokens: Option<usize>,
}

pub struct GenerateHandler;
impl GenerateHandler {
    pub async fn handle(
        State(state): State<AppState>,
        Json(req): Json<GenerateRequest>,
    ) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
        while state.blocking_reload.load(Ordering::SeqCst) {
            state.notify_reload_done.notified().await;
        }

        state.active_requests.fetch_add(1, Ordering::SeqCst);

        let guard = state.shared.read();
        let mut session = match guard.model.session_builder(&guard.backend).build() {
            Ok(sess) => sess,
            Err(e) => {
                error!(?e, "Failed to create session");
                state.active_requests.fetch_sub(1, Ordering::SeqCst);
                return Sse::new(futures::stream::once(async move {
                    Ok(Event::default().data(format!("Session error: {:?}", e)))
                }));
            }
        };

        let full_prompt = state.full_prompt(&req.prompt, req.mode.as_deref());

        let max_tokens = req.max_tokens.unwrap_or(128) as i32;

        let output_stream = stream! {
            match session.eval_stream(&full_prompt, max_tokens) {
                Ok(mut token_stream) => {
                    while let Some(token_result) = token_stream.next() {
                        match token_result {
                            Ok(token) => yield Ok(Event::default().data(token)),
                            Err(e) => { yield Ok(Event::default().data(format!("Error: {:?}", e))); break; }
                        }
                        task::yield_now().await;
                    }
                }
                Err(e) => yield Ok(Event::default().data(format!("Evaluation failed: {:?}", e))),
            }

            if state.active_requests.fetch_sub(1, Ordering::SeqCst) <= 1 {
                state.notify_no_active.notify_waiters();
            }
        };

        Sse::new(output_stream)
    }
}
