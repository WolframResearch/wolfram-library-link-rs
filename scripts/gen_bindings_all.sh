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

DOCKER_PLATFORM="${DOCKER_PLATFORM:-linux/amd64}"

echo "Generating bindings in Docker for target: $RUST_TARGET (SystemID: $SYSTEM_ID) using image: $IMAGE (platform: $DOCKER_PLATFORM)"

# Prepare mount option if a local Wolfram app directory is set
MOUNT_WOLFRAM=""
if [ -n "${WOLFRAM_APP_DIRECTORY:-}" ]; then
  MOUNT_WOLFRAM="-v \"$WOLFRAM_APP_DIRECTORY\":\"$WOLFRAM_APP_DIRECTORY\":ro -e WOLFRAM_APP_DIRECTORY=\"$WOLFRAM_APP_DIRECTORY\""
fi

# Snapshot existing generated bindings (record path + size) so we can detect new/changed/removed files later.
SNAP_BEFORE=$(mktemp)
find wolfram-library-link-sys/generated -type f -name 'LibraryLink_bindings.rs' -print0 2>/dev/null | \
  xargs -0 -I{} sh -c 'printf "%s %s\n" "$(wc -c <"{}")" "{}"' | sort > "$SNAP_BEFORE" || true

echo "[info] snapshot before generation: $(wc -l < "$SNAP_BEFORE") files"

LOGFILE="$(mktemp -t gen-bindings-XXXX).log"

echo "[info] docker output will be written to: $LOGFILE"

# Run docker and capture stdout/stderr to $LOGFILE so we can inspect the xtask output.
docker run --platform "$DOCKER_PLATFORM" --rm -i \
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
    echo 'gen-bindings completed'; \
    chown -R $(id -u):$(id -g) /work || true" > "$LOGFILE" 2>&1 || true

DOCKER_RC=$?
echo "[debug] docker run exit code: $DOCKER_RC" | tee -a "$LOGFILE"

if [ $DOCKER_RC -ne 0 ]; then
  echo "[error] docker run failed; last 200 lines of log:" >&2
  tail -n 200 "$LOGFILE" >&2 || true
  rm -f "$SNAP_BEFORE" "$LOGFILE"
  exit $DOCKER_RC
fi

# Snapshot after generation (path + size)
SNAP_AFTER=$(mktemp)
find wolfram-library-link-sys/generated -type f -name 'LibraryLink_bindings.rs' -print0 2>/dev/null | \
  xargs -0 -I{} sh -c 'printf "%s %s\n" "$(wc -c <"{}")" "{}"' | sort > "$SNAP_AFTER" || true

# Compute new/removed/changed files
NEW_FILES=$(comm -13 <(awk '{print $2}' "$SNAP_BEFORE") <(awk '{print $2}' "$SNAP_AFTER") | sed '/^$/d')
REMOVED_FILES=$(comm -23 <(awk '{print $2}' "$SNAP_BEFORE") <(awk '{print $2}' "$SNAP_AFTER") | sed '/^$/d')
CHANGED_FILES=$(awk 'NR==FNR{a[$2]=$1;next} { if ($2 in a && a[$2]!=$1) print $2 }' "$SNAP_BEFORE" "$SNAP_AFTER" | sed '/^$/d')

if [ -z "$NEW_FILES" ] && [ -z "$CHANGED_FILES" ]; then
  echo "[error] No new or changed binding files were generated." >&2
  echo "Files before generation: $(wc -l < "$SNAP_BEFORE")" >&2
  echo "Files after  generation: $(wc -l < "$SNAP_AFTER")" >&2
  echo "Last 200 lines of docker log:" >&2
  tail -n 200 "$LOGFILE" >&2 || true
  rm -f "$SNAP_BEFORE" "$SNAP_AFTER" "$LOGFILE"
  exit 2
fi

echo "[info] new generated binding files:" 
echo "$NEW_FILES"
if [ -n "$CHANGED_FILES" ]; then
  echo "[info] changed binding files:" 
  echo "$CHANGED_FILES"
fi
if [ -n "$REMOVED_FILES" ]; then
  echo "[warn] removed binding files:" 
  echo "$REMOVED_FILES"
fi

# Validate new/changed files are non-empty
BAD=0
for f in $(echo -e "$NEW_FILES\n$CHANGED_FILES" | sed '/^$/d'); do
  if [ ! -s "$f" ]; then
    echo "[error] generated file exists but is empty: $f" >&2
    BAD=1
  fi
done

rm -f "$SNAP_BEFORE" "$SNAP_AFTER" "$LOGFILE"

if [ $BAD -ne 0 ]; then
  echo "[error] one or more generated binding files were empty" >&2
  exit 3
fi

echo "Generated bindings for target $RUST_TARGET (SystemID: $SYSTEM_ID)."

echo "Done. Please review the generated files under wolfram-library-link-sys/generated/ and commit them."
