use std::collections::HashMap;
use std::sync::RwLock as StdRwLock; // or parking_lot::RwLock
pub struct AppState {
    pub shared: parking_lot::RwLock<SharedModel>,
    pub blocking_reload: AtomicBool,
    pub active_requests: AtomicUsize,
    pub notify_no_active: tokio::sync::Notify,
    pub notify_reload_done: tokio::sync::Notify,
    pub modes: std::sync::RwLock<HashMap<String, String>>,
    pub current_mode: std::sync::RwLock<Option<String>>,
}

impl AppState {
    // Get a system prompt for a specific mode name
    pub fn get_mode(&self, mode_name: &str) -> Option<String> {
        self.modes.read().get(mode_name).cloned()
    }

    // Get the full prompt for a request (considering current/default mode)
    pub fn full_prompt(&self, prompt: &str, mode_name: Option<&str>) -> String {
        if let Some(mode) = mode_name {
            if let Some(system_prompt) = self.get_mode(mode) {
                return format!("{}\n\n{}", system_prompt, prompt);
            }
        }

        // fallback: use current mode if no mode specified
        if let Some(system_prompt) = self.get_current_prompt() {
            return format!("{}\n\n{}", system_prompt, prompt);
        }

        // default: just return user prompt
        prompt.to_string()
    }
}
