#!/usr/bin/env bash
step_build_and_run() {
    log "Building server (release mode)..."
    cargo build --release || err "Build failed"
    
    [ "$AUTO_MODE" != "--auto" ] && read -rp "Press ENTER to run the server..." _
    
    log "Starting server..."
    MODEL_PATH="$MODEL_PATH" PREFER_CUDA=true N_GPU_LAYERS=-1 "./target/release/$PROJECT_NAME"
    
    popd >/dev/null
}
