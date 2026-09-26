#!/usr/bin/env bash
# Build the out-of-repository scratch crate that compiles
# apps/desktop/src-tauri/src/model_document_migration.rs verbatim (#[path]).
# Usage: make_harness.sh <repo_root> <harness_dir>
# The module needs only serde + serde_json; nothing is stubbed.
set -euo pipefail
REPO="$1"; H="$2"
MOD="$REPO/projects/chirality-piping/apps/desktop/src-tauri/src/model_document_migration.rs"
test -f "$MOD"
mkdir -p "$H/src"
cat > "$H/Cargo.toml" <<'TOML'
[package]
name = "t1_wp2_native_harness"
version = "0.0.0"
edition = "2021"
publish = false

[lib]
path = "src/lib.rs"

# Versions pinned to apps/desktop/src-tauri/Cargo.lock; features as in its Cargo.toml.
[dependencies]
serde = { version = "=1.0.228", features = ["derive"] }
serde_json = { version = "=1.0.149", features = ["float_roundtrip"] }

[workspace]
TOML
cat > "$H/src/lib.rs" <<RS
//! Scratch harness: the src-tauri module source, included verbatim.
#[path = "$MOD"]
pub mod model_document_migration;
RS
