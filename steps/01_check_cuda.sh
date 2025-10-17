#!/usr/bin/env bash
step_check_cuda() {
    log "Checking NVIDIA GPU + CUDA setup..."
    command -v nvidia-smi >/dev/null || err "NVIDIA drivers missing"
    command -v nvcc >/dev/null || err "CUDA toolkit (nvcc) missing"
    log "✅ NVIDIA GPU detected"
}
