#!/usr/bin/env bash
# Generate wolfram-library-link-sys bindings for multiple platforms using Docker
#
# This script automates running `cargo +nightly xtask gen-bindings` inside a Docker
# container for a set of Linux targets. It is primarily useful for generating the
# bindings for Linux system IDs (x86_64, aarch64). macOS bindings must be generated
# on macOS hosts because Wolfram Mathematica is not distributable and not available
# inside Docker images.
#
# Usage:
#   ./scripts/gen_bindings_all.sh
#
# Environment variables:
#   WOLFRAM_APP_DIRECTORY - path inside the container where the Wolfram app is mounted.
#                           The script will mount the host $WOLFRAM_APP_DIRECTORY into
#                           the container at the same path. You must ensure the path
#                           points to a valid Mathematica/Wolfram installation on the
#                           host machine.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# Simplify to a single Linux target (x86_64) using WolframResearch's Wolfram Engine image.
RUST_TARGET="x86_64-unknown-linux-gnu"
SYSTEM_ID="Linux-x86-64"

# Docker image to use. Default to WolframResearch's Wolfram Engine image. You can
# override by setting WOLFRAM_DOCKER_IMAGE in the environment.
DEFAULT_IMAGE="wolframresearch/wolframengine:latest"
IMAGE="${WOLFRAM_DOCKER_IMAGE:-$DEFAULT_IMAGE}"

echo "Generating bindings in Docker for target: $RUST_TARGET (SystemID: $SYSTEM_ID) using image: $IMAGE"

# Mount WOLFRAM_APP_DIRECTORY if provided; otherwise assume the Docker image includes the runtime.
MOUNT_WOLFRAM=""
if [ -n "${WOLFRAM_APP_DIRECTORY:-}" ]; then
  MOUNT_WOLFRAM="-v \"$WOLFRAM_APP_DIRECTORY\":\"$WOLFRAM_APP_DIRECTORY\":ro -e WOLFRAM_APP_DIRECTORY=\"$WOLFRAM_APP_DIRECTORY\""
fi

docker run --rm -i \
  -v "$ROOT_DIR":/work -w /work \
  $MOUNT_WOLFRAM \
  $IMAGE /bin/bash -lc "set -euo pipefail; \
    # Ensure Rust toolchain is available in the container; install if necessary.\
    if ! command -v rustup >/dev/null 2>&1; then \
      echo 'Installing Rust...'; apt-get update && apt-get install -y --no-install-recommends curl build-essential pkg-config libssl-dev clang cmake wget git && \
      curl https://sh.rustup.rs -sSf | sh -s -- -y; export PATH=\$HOME/.cargo/bin:\$PATH; \
    else \
      export PATH=\$HOME/.cargo/bin:\$PATH; \
    fi; \
    rustup default stable || true; rustup toolchain install nightly || true; rustup target add $RUST_TARGET || true; \
    cargo install --locked cargo-make || true; \
    echo 'Running xtask gen-bindings...'; cargo +nightly xtask gen-bindings --target $RUST_TARGET; \
    chown -R $(id -u):$(id -g) /work || true"

echo "Generated bindings for target $RUST_TARGET (SystemID: $SYSTEM_ID)."

echo "Done. Please review the generated files under wolfram-library-link-sys/generated/ and commit them."
