use anyhow::{anyhow, Result};
use llama_cpp_2::{llama_backend::LlamaBackend, model::LlamaModel};
use reqwest::blocking::Client;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;

/// Represents a loaded Llama model and its backend
pub struct ModelHandle {
    pub model: LlamaModel,
    pub backend: LlamaBackend,
    pub model_path: PathBuf,
}

impl ModelHandle {
    /// Load a model from a Hugging Face repo or local path
    /// Example: load_model("TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF")
    pub fn load_model(name: &str) -> Result<Self> {
        let local_path = Self::model_cache_path(name);

        // Download if missing
        if !local_path.exists() {
            Self::download_from_hf(name, &local_path)?;
        }

        // Load into Llama backend
        let backend = LlamaBackend::init()?;
        let model = LlamaModel::load_from_file(
            &backend,
            local_path.to_str().unwrap(),
            &Default::default(),
        )?;
        info!(model=%local_path.display(), "Loaded model");

        Ok(Self {
            model,
            backend,
            model_path: local_path,
        })
    }

    /// Return standard cache path
    fn model_cache_path(name: &str) -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".laurus_llm/models")
            .join(name)
            .with_extension("gguf")
    }

    /// Download model from Hugging Face repository
    fn download_from_hf(name: &str, dest: &Path) -> Result<()> {
        fs::create_dir_all(dest.parent().unwrap())?;

        // Build URL (assume "resolve/main/{name}.gguf")
        let url = format!(
            "https://huggingface.co/{name}/resolve/main/{name}.gguf",
            name = name
        );

        info!(url, file=%dest.display(), "Downloading model from Hugging Face...");
        let resp = Client::new().get(&url).send()?;
        if !resp.status().is_success() {
            return Err(anyhow!("Failed to download model: HTTP {}", resp.status()));
        }

        let bytes = resp.bytes()?;
        fs::write(dest, bytes)?;
        info!(file=%dest.display(), "Downloaded model successfully");
        Ok(())
    }
}
