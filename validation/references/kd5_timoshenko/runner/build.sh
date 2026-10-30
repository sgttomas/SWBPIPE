#!/bin/bash
# Build the M3-T runner against the product_physics crate of a checkout.
#   build.sh <checkout root> <cargo target dir> <binary out>
# The crate links ../../../../core/product_physics, the product of the checkout it sits in. For another
# checkout (for example a detached scratch worktree of a product commit), Cargo.toml, Cargo.lock and
# src/main.rs are first copied to the same path inside that checkout; a different existing copy is refused.
# Cargo is $KD5T_CARGO (default: cargo; a host wrapper may be named) and runs with --locked, so no
# Cargo.lock changes. Prints the binary's sha256 and the checkout's commit.
set -eu
[ $# -eq 3 ] || { echo "usage: build.sh <checkout root> <cargo target dir> <binary out>" >&2; exit 2; }
HERE=$(cd "$(dirname "$0")" && pwd -P)
ROOT=$(cd "$1" && pwd -P)
TARGET=$2
OUT=$3
REL=projects/chirality-piping/validation/references/kd5_timoshenko/runner
[ -f "$ROOT/projects/chirality-piping/core/product_physics/Cargo.toml" ] || { echo "no product_physics crate under $ROOT" >&2; exit 2; }
DEST=$ROOT/$REL
if [ "$(cd "$DEST" 2>/dev/null && pwd -P)" != "$HERE" ]; then
  for f in Cargo.toml Cargo.lock src/main.rs; do
    if [ -e "$DEST/$f" ] && ! cmp -s "$HERE/$f" "$DEST/$f"; then
      echo "refusing: $DEST/$f exists and differs from $HERE/$f" >&2
      exit 2
    fi
  done
  mkdir -p "$DEST/src"
  for f in Cargo.toml Cargo.lock src/main.rs; do cp "$HERE/$f" "$DEST/$f"; done
fi
export CARGO_TARGET_DIR=$TARGET
${KD5T_CARGO:-cargo} build --release --locked --manifest-path "$DEST/Cargo.toml"
cp "$TARGET/release/kd5t_runner" "$OUT"
shasum -a 256 "$OUT"
echo "built against $ROOT at $(git -C "$ROOT" rev-parse HEAD 2>/dev/null || echo unknown)"
