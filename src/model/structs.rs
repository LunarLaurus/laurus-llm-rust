use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub prompt: String,
    pub max_tokens: Option<usize>,
    pub mode: Option<String>,
}

#[derive(Deserialize)]
pub struct ModeRequest {
    pub name: String,
    pub system_prompt: String,
}

#[derive(Serialize)]
pub struct ModeResponse {
    pub name: String,
    pub system_prompt: String,
}
