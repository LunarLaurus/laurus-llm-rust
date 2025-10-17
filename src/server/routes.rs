use crate::server::handlers::{GenerateHandler, StatusHandler, SwitchModelHandler};
use crate::server::state::AppState;
use axum::routing::{get, post, put};
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/generate", post(GenerateHandler::handle))
        .route("/switch_model", put(SwitchModelHandler::handle))
        .route("/status", get(StatusHandler::handle))
        .route("/modes", get(ModesHandler::list).post(ModesHandler::add))
}
