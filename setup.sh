#!/usr/bin/env bash
set -euo pipefail
shopt -s expand_aliases

AUTO_MODE=${1:-""}  # pass --auto for sequential build

# Paths
PROJECT_NAME="laurus-llm-rust"
PROJECT_DIR="$(pwd)/${PROJECT_NAME}"
LLAMA_CPP_DIR="${PROJECT_DIR}/llama.cpp"
MODEL_DIR="${PROJECT_DIR}/model"
MODEL_URL="https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf"
MODEL_PATH="${MODEL_DIR}/tinyllama.gguf"
RUST_TOOLCHAIN="stable"

# Include helpers
log() { echo -e "\033[1;34m[INFO]\033[0m $*"; }
warn() { echo -e "\033[1;33m[WARN]\033[0m $*"; }
err() { echo -e "\033[1;31m[ERR]\033[0m $*" >&2; exit 1; }
pause() { [ "$AUTO_MODE" != "--auto" ] && read -rp "Press Enter to continue..."; }

# Load steps
for step in steps/*.sh; do
    # shellcheck disable=SC1090
    source "$step"
done

# ================================================================
# Generate recursive file/folder list from script root
# ================================================================
step_generate_file_tree() {
    local script_dir
    script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    local output_file="$script_dir/file_tree.txt"
    
    # Folders to ignore
    local ignore_dirs=(
        ".git"
        "target"
        "__pycache__"
        "node_modules"
        ".idea"
        ".vscode"
    )
    
    log "Generating file/folder tree for $script_dir, ignoring: ${ignore_dirs[*]}"
    
    # Build find exclude arguments
    local exclude_args=()
    for d in "${ignore_dirs[@]}"; do
        exclude_args+=(-path "$script_dir/$d" -prune -o)
    done
    
    # Run find with exclusions
    find "$script_dir" "${exclude_args[@]}" -print | sort > "$output_file"
    
    log "✅ File tree saved to $output_file"
}



auto() {
    step_check_cuda
    step_install_deps
    step_install_rust
    step_setup_rust_project
    step_build_llama_cpp
    step_download_model
    step_build_and_run
}

menu() {
    clear
    echo "==============================="
    echo " 🚀 Laurus LLM Rust Setup"
    echo "==============================="
    echo
    echo "1) Check CUDA + NVIDIA"
    echo "2) Install Dependencies"
    echo "3) Install Rust"
    echo "4) Setup Rust Project"
    echo "5) Build llama.cpp"
    echo "6) Download Model"
    echo "7) Build & Run Server"
    echo "x) Auto"
    echo ""
    echo "11) File tree"
    echo "0) Exit"
    echo
}

if [ "$AUTO_MODE" = "--auto" ]; then
    auto
    exit 0
fi

while true; do
    menu
    read -rp "Select step [0-7]: " choice
    case "$choice" in
        1) step_check_cuda; pause ;;
        2) step_install_deps; pause ;;
        3) step_install_rust; pause ;;
        4) step_setup_rust_project; pause ;;
        5) step_build_llama_cpp; pause ;;
        6) step_download_model; pause ;;
        7) step_build_and_run; pause ;;
        x) auto; pause ;;
        11) step_generate_file_tree; pause ;;
        0) log "Exiting."; exit 0 ;;
        *) warn "Invalid choice"; pause ;;
    esac
done
