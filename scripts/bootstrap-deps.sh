#!/usr/bin/env bash
# Clone the sibling `wolfram-expr-rs` and `wstp-rs` repos at the workspace
# root. The outer `Cargo.toml`'s `[patch.crates-io]` block points at these
# paths, so `cargo build` won't resolve without them.
#
# These are *not* git submodules: we keep them as independent checkouts
# so the PR/branch workflow on those upstreams isn't tangled with this
# repo's history. Set `FORK_OWNER` to clone from your own fork instead.
#
# Usage:
#   ./scripts/bootstrap-deps.sh            # clone from WolframResearch upstream
#   FORK_OWNER=sw1sh ./scripts/bootstrap-deps.sh
#   BRANCH=feat/wxf ./scripts/bootstrap-deps.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OWNER="${FORK_OWNER:-WolframResearch}"
BRANCH="${BRANCH:-master}"

clone_or_update() {
    local name="$1"
    local dest="$ROOT/$name"
    local url="git@github.com:${OWNER}/${name}.git"

    if [[ -d "$dest/.git" ]]; then
        echo "[$name] already present; fetching latest"
        git -C "$dest" fetch --all --prune
    else
        echo "[$name] cloning $url → $dest"
        git clone "$url" "$dest"
    fi

    # Best-effort checkout of the requested branch; skip if not present.
    if git -C "$dest" rev-parse --verify --quiet "$BRANCH" >/dev/null; then
        git -C "$dest" checkout "$BRANCH"
    else
        echo "[$name] branch '$BRANCH' not found locally; leaving HEAD as-is"
    fi
}

clone_or_update wolfram-expr-rs
clone_or_update wstp-rs

echo
echo "Done. Sibling checkouts ready:"
echo "  $ROOT/wolfram-expr-rs"
echo "  $ROOT/wstp-rs"
echo
echo "You can now run: cargo build --workspace"
