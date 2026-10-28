#!/bin/bash
# Build the PF M3-T runner from a clean checkout (refutation round 4).
#   build.sh <checkout root> <cargo target dir> <binary out>
# The runner is the checkout's own committed runner/ (Cargo.toml, Cargo.lock, build.rs, src/main.rs); it links
# ../../../../core/product_physics of that checkout. Every build input must be clean in git: core/ (the product and
# all its path dependencies), fixtures/, schemas/ and validation/ (files the product embeds, and the runner sources
# and Cargo.lock); the runner links nothing under apps/. A dirty input refuses the build; PFM3T_ALLOW_DIRTY=1 builds
# anyway with the head marked -dirty, which the judging pass refuses. The head baked in is the checkout's commit.
# Cargo is $PFM3T_CARGO (default: cargo; a host wrapper may be named) and runs with --locked. Prints the binary's
# sha256 and the head.
set -eu
[ $# -eq 3 ] || { echo "usage: build.sh <checkout root> <cargo target dir> <binary out>" >&2; exit 2; }
ROOT=$(cd "$1" && pwd -P)
TARGET=$2
OUT=$3
P=projects/chirality-piping
DEST=$ROOT/$P/validation/references/pf_timoshenko_m3t/runner
[ -f "$ROOT/$P/core/product_physics/Cargo.toml" ] || { echo "no product_physics crate under $ROOT" >&2; exit 2; }
[ -f "$DEST/Cargo.toml" ] && [ -f "$DEST/build.rs" ] || { echo "the checkout has no committed PF M3-T runner at $DEST" >&2; exit 2; }
HEAD_SHA=$(git -C "$ROOT" rev-parse --verify HEAD^{commit})
DIRTY=$(git -C "$ROOT" status --porcelain --untracked-files=all -- "$P/core" "$P/fixtures" "$P/schemas" "$P/validation")
if [ -n "$DIRTY" ]; then
  if [ "${PFM3T_ALLOW_DIRTY:-}" = 1 ]; then
    HEAD_SHA=$HEAD_SHA-dirty
  else
    echo "refusing: build inputs of the runner are not clean at $HEAD_SHA:" >&2
    echo "$DIRTY" | head -20 >&2
    exit 3
  fi
fi
export CARGO_TARGET_DIR=$TARGET
export PFM3T_PRODUCT_HEAD=$HEAD_SHA
${PFM3T_CARGO:-cargo} build --release --locked --manifest-path "$DEST/Cargo.toml"
cp "$TARGET/release/pft_m3t_runner" "$OUT"
shasum -a 256 "$OUT"
echo "built from $ROOT at $PFM3T_PRODUCT_HEAD"
