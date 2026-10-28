#!/bin/bash
# Build the M3 judge's runner from a clean checkout (both routes).
#   build.sh <checkout root> <cargo target dir> <binary out>
# The runner is the checkout's own committed m3_judge/runner (Cargo.toml, Cargo.lock, build.rs, src/main.rs); it links
# ../../../../core/product_physics of that checkout. Every build input must be clean in git: core/ (the product and all
# its path dependencies), fixtures/, schemas/ and validation/ (files the product embeds, the runner sources and its
# Cargo.lock); the runner links nothing under apps/. A dirty input refuses the build (exit 3); M3J_ALLOW_DIRTY=1 builds
# anyway with the head marked -dirty, which the judging pass refuses. The head baked in is the checkout's commit.
# Cargo is $M3J_CARGO (default: cargo; a host wrapper may be named) and runs with --locked. Prints the binary's sha256
# and the head.
set -eu
[ $# -eq 3 ] || { echo "usage: build.sh <checkout root> <cargo target dir> <binary out>" >&2; exit 2; }
ROOT=$(cd "$1" && pwd -P)
TARGET=$2
OUT=$3
P=projects/chirality-piping
DEST=$ROOT/$P/validation/references/m3_judge/runner
[ -f "$ROOT/$P/core/product_physics/Cargo.toml" ] || { echo "no product_physics crate under $ROOT" >&2; exit 2; }
[ -f "$DEST/Cargo.toml" ] && [ -f "$DEST/build.rs" ] || { echo "the checkout has no committed M3 judge runner at $DEST" >&2; exit 2; }
HEAD_SHA=$(git -C "$ROOT" rev-parse --verify HEAD^{commit})
DIRTY=$(git -C "$ROOT" status --porcelain --untracked-files=all -- "$P/core" "$P/fixtures" "$P/schemas" "$P/validation")
if [ -n "$DIRTY" ]; then
  if [ "${M3J_ALLOW_DIRTY:-}" = 1 ]; then
    HEAD_SHA=$HEAD_SHA-dirty
  else
    echo "refusing: build inputs of the runner are not clean at $HEAD_SHA:" >&2
    echo "$DIRTY" | head -20 >&2
    exit 3
  fi
fi
export CARGO_TARGET_DIR=$TARGET
export M3J_PRODUCT_HEAD=$HEAD_SHA
${M3J_CARGO:-cargo} build --release --locked --manifest-path "$DEST/Cargo.toml"
cp "$TARGET/release/m3_judge_runner" "$OUT"
shasum -a 256 "$OUT"
echo "built from $ROOT at $M3J_PRODUCT_HEAD"
