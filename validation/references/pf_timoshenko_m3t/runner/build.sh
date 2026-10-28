#!/bin/bash
# Build the PF M3-T runner against the product_physics crate of a checkout.
#   build.sh <checkout root> <cargo target dir> <binary out>
# The crate links ../../../../core/product_physics, the product of the checkout it sits in. For another
# checkout (for example a detached scratch worktree of a product commit), Cargo.toml, Cargo.lock, build.rs and
# src/main.rs are first copied to the same path inside that checkout; a different existing copy is refused.
# Cargo is $PFM3T_CARGO (default: cargo; a host wrapper may be named) and runs with --locked, so no
# Cargo.lock changes. Prints the binary's sha256 and the checkout's commit.
set -eu
[ $# -eq 3 ] || { echo "usage: build.sh <checkout root> <cargo target dir> <binary out>" >&2; exit 2; }
HERE=$(cd "$(dirname "$0")" && pwd -P)
ROOT=$(cd "$1" && pwd -P)
TARGET=$2
OUT=$3
REL=projects/chirality-piping/validation/references/pf_timoshenko_m3t/runner
[ -f "$ROOT/projects/chirality-piping/core/product_physics/Cargo.toml" ] || { echo "no product_physics crate under $ROOT" >&2; exit 2; }
DEST=$ROOT/$REL
if [ "$(cd "$DEST" 2>/dev/null && pwd -P)" != "$HERE" ]; then
  for f in Cargo.toml Cargo.lock build.rs src/main.rs; do
    if [ -e "$DEST/$f" ] && ! cmp -s "$HERE/$f" "$DEST/$f"; then
      echo "refusing: $DEST/$f exists and differs from $HERE/$f" >&2
      exit 2
    fi
  done
  mkdir -p "$DEST/src"
  for f in Cargo.toml Cargo.lock build.rs src/main.rs; do cp "$HERE/$f" "$DEST/$f"; done
fi
export CARGO_TARGET_DIR=$TARGET
# the product commit the runner is built from, baked into each record (build.rs)
HEAD_SHA=$(git -C "$ROOT" rev-parse HEAD 2>/dev/null || echo unknown)
[ -n "$(git -C "$ROOT" status --porcelain -- projects/chirality-piping/core 2>/dev/null)" ] && HEAD_SHA=$HEAD_SHA-dirty
export PFM3T_PRODUCT_HEAD=$HEAD_SHA
${PFM3T_CARGO:-cargo} build --release --locked --manifest-path "$DEST/Cargo.toml"
cp "$TARGET/release/pft_m3t_runner" "$OUT"
shasum -a 256 "$OUT"
echo "built against $ROOT at $PFM3T_PRODUCT_HEAD"
