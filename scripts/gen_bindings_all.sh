#!/usr/bin/env bash
# Generate wolfram-library-link-sys bindings for all platforms
# Requires: Rust nightly, clang, local Wolfram installation
#
# Usage: ./scripts/gen_bindings_all.sh

set -euo pipefail

TARGETS=(
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-msvc"
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
)

for target in "${TARGETS[@]}"; do
    echo "Generating bindings for $target..."
    cargo +nightly xtask gen-bindings --target "$target"
done

echo "Done. Review files under wolfram-library-link-sys/generated/ and commit."
