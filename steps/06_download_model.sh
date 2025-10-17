#!/usr/bin/env bash
step_download_model() {
    mkdir -p "$MODEL_DIR"
    if [ ! -f "$MODEL_PATH" ]; then
        log "Downloading TinyLlama model..."
        curl -L "$MODEL_URL" -o "$MODEL_PATH"
    else
        log "Model already exists at $MODEL_PATH"
    fi
}
