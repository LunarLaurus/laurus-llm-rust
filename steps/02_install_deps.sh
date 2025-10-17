#!/usr/bin/env bash
step_install_deps() {
    log "Installing system dependencies..."
    sudo apt update -y
    sudo apt install -y build-essential cmake git pkg-config libssl-dev curl unzip clang llvm libclang-dev libcurl4-openssl-dev pkg-config
    log "✅ Dependencies installed"
}
