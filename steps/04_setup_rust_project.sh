#!/usr/bin/env bash
step_setup_rust_project() {
    mkdir -p "$PROJECT_DIR/src"
    
    if [ -f "$PROJECT_DIR/main.rs" ]; then
        mv "$PROJECT_DIR/main.rs" "$PROJECT_DIR/src/main.rs"
    fi
    
    if [ ! -f "$PROJECT_DIR/Cargo.toml" ]; then
        log "Generating Cargo.toml..."
        cat > "$PROJECT_DIR/Cargo.toml" <<'EOF'
[package]
name = "laurus-llm-rust"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.6"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
parking_lot = "0.12"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt", "env-filter"] }
llama-cpp-2 = { version = "0.1.122", features = ["cuda"] }
EOF
    else
        log "Cargo.toml exists"
    fi
}
