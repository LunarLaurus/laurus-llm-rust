use crate::model::ModelHandle;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Notify;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub shared: Arc<RwLock<ModelHandle>>,
    pub blocking_reload: Arc<AtomicBool>,
    pub active_requests: Arc<AtomicUsize>,
    pub notify_no_active: Arc<Notify>,
    pub notify_reload_done: Arc<Notify>,
}

impl AppState {
    pub fn new(shared: ModelHandle) -> Self {
        Self {
            shared: Arc::new(RwLock::new(shared)),
            blocking_reload: Arc::new(AtomicBool::new(false)),
            active_requests: Arc::new(AtomicUsize::new(0)),
            notify_no_active: Arc::new(Notify::new()),
            notify_reload_done: Arc::new(Notify::new()),
        }
    }
}
