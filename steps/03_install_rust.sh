#!/usr/bin/env bash
step_install_rust() {
    RUST_TOOLCHAIN="${RUST_TOOLCHAIN:-stable}"
    
    if ! command -v cargo >/dev/null 2>&1; then
        log "Rust/cargo not found. Installing Rust ($RUST_TOOLCHAIN)..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$RUST_TOOLCHAIN" || err "Rust installation failed"
        if [ -f "$HOME/.cargo/env" ]; then
            source "$HOME/.cargo/env"
        else
            warn "$HOME/.cargo/env not found. Add $HOME/.cargo/bin to PATH manually."
        fi
    else
        log "Cargo already installed; skipping installation"
    fi
    
    export PATH="$HOME/.cargo/bin:$PATH"
    
    for rc_file in "$HOME/.bashrc" "$HOME/.zshrc"; do
        if [ -f "$rc_file" ]; then
            grep -qxF 'export PATH="$HOME/.cargo/bin:$PATH"' "$rc_file" || \
            echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$rc_file" && \
            log "Added Cargo to PATH in $rc_file"
        fi
    done
    
    command -v cargo >/dev/null 2>&1 || err "Cargo not found after installation"
    log "Rust version: $(rustc --version || echo 'not found')"
    log "Cargo version: $(cargo --version || echo 'not found')"
}
