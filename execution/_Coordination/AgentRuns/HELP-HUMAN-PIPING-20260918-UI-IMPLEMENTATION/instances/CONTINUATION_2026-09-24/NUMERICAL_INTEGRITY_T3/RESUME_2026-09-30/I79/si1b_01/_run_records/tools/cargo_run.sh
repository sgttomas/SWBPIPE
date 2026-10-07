#!/bin/bash
# I79 T3-SI1b: run one cargo command through the T3 host lock.
# Usage: cargo_run.sh <label> <crate dir> <target dir> <cargo args...>
set -u
WT=WT
label=$1; dir=$2; tgt=$3; shift 3
LOG=$WT/scratch/i79_si1b/logs/$label.log
cd "$dir" || exit 2
echo "== $label start $(date -u +%FT%TZ) dir=$dir" > "$LOG"
env RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 \
  CARGO_TARGET_DIR="$tgt" TMPDIR=$WT/scratch/i79_si1b/tmp \
  $WT/tools/t3_cargo.sh "$@" >> "$LOG" 2>&1
rc=$?
echo "== $label exit=$rc $(date -u +%FT%TZ)" >> "$LOG"
exit $rc
