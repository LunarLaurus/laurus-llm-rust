pub mod app_state;
pub mod handlers;
pub mod routes;
pub mod server;
pub mod state;

use crate::config::AppConfig;
use crate::model::ModelHandle;
use crate::server::routes;
use crate::server::state::AppState;
use axum::Router;
