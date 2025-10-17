#!/usr/bin/env bash
step_build_llama_cpp() {
    log "Preparing to clone/build llama.cpp..."
    
    LLAMA_FORCE_CUDA="${LLAMA_FORCE_CUDA:-}"
    LLAMA_FORCE_CPU="${LLAMA_FORCE_CPU:-}"
    LLAMA_CLEAN_BUILD="${LLAMA_CLEAN_BUILD:-}"
    LLAMA_FORCE_CURL="${LLAMA_FORCE_CURL:-}"
    LLAMA_FORCE_NO_CURL="${LLAMA_FORCE_NO_CURL:-}"
    
    if [ ! -d "$LLAMA_CPP_DIR" ]; then
        log "Cloning llama.cpp (shallow)..."
        git clone --depth 1 https://github.com/ggerganov/llama.cpp.git "$LLAMA_CPP_DIR" || err "git clone failed"
    else
        log "Updating llama.cpp..."
        pushd "$LLAMA_CPP_DIR" >/dev/null
        git fetch --all --prune || warn "git fetch failed"
        git reset --hard origin/HEAD || warn "git reset failed"
        git pull --ff --rebase || warn "git pull skipped or failed"
        popd >/dev/null
    fi
    
    pushd "$LLAMA_CPP_DIR" >/dev/null
    
    _nvcc_version=$(nvcc --version 2>/dev/null | grep "release" | head -n1 | awk '{print $6}' || echo "")
    _cmake_version=$(cmake --version 2>/dev/null | head -n1 | awk '{print $3}' || echo "")
    
    GGML_CUDA="OFF"
    if [ -n "$LLAMA_FORCE_CPU" ]; then
        GGML_CUDA="OFF"
        elif [ -n "$LLAMA_FORCE_CUDA" ] || [ -n "$_nvcc_version" ]; then
        GGML_CUDA="ON"
    fi
    
    LLAMA_CURL="OFF"
    _detect_curl() {
        command -v curl-config >/dev/null && curl-config --libs >/dev/null && return 0
        command -v pkg-config >/dev/null && pkg-config --exists libcurl && return 0
        [ -f /usr/include/curl/curl.h ] || [ -f /usr/local/include/curl/curl.h ] && return 0
        return 1
    }
    
    if [ -n "$LLAMA_FORCE_NO_CURL" ]; then
        LLAMA_CURL="OFF"
        elif [ -n "$LLAMA_FORCE_CURL" ] && _detect_curl; then
        LLAMA_CURL="ON"
        elif _detect_curl; then
        LLAMA_CURL="ON"
    fi
    
    BUILD_DIR="build"
    [ -n "$LLAMA_CLEAN_BUILD" ] && rm -rf "$BUILD_DIR"
    mkdir -p "$BUILD_DIR"
    
    CMAKE_CUDA_ARCHS="75"
    CMAKE_CUDA_FLAGS="-Wno-deprecated-gpu-targets"
    
    cmake -S . -B "$BUILD_DIR" \
    -DGGML_CUDA="$GGML_CUDA" \
    -DCMAKE_CUDA_ARCHITECTURES="$CMAKE_CUDA_ARCHS" \
    -DCMAKE_CUDA_FLAGS="$CMAKE_CUDA_FLAGS" \
    -DLLAMA_CURL="$LLAMA_CURL" \
    -DLLAMA_CUBLAS=OFF \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_POSITION_INDEPENDENT_CODE=ON \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON || warn "CMake failed"
    
    cmake --build "$BUILD_DIR" --config Release --parallel "$(nproc)" || warn "Build failed"
    
    popd >/dev/null
    cd "$PROJECT_DIR"
}
