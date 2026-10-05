#!/bin/bash
set -u
W=WT; S=$W/scratch/i61_u1_serializer_02; C=$W/f2a-serializer/projects/chirality-piping/core
run() {
  pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  echo "== $1 start $(date -u +%FT%TZ)"
  ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${4:-} > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"
  awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sort > $S/logs/$1.outcomes
}
run final2_pp $C/product_physics/Cargo.toml $W/targets/i61-u1/product_physics
run final2_runner $C/runner/headless/Cargo.toml $W/targets/i61-u1/runner
run final2_fk_lib $C/solver/frame_kernel/Cargo.toml $W/targets/i61-u1/fk --lib
( cd $C/product_physics && env CARGO_BUILD_JOBS=4 perl -e 'alarm shift; exec @ARGV' 1200 cargo build --locked --offline --lib --target-dir $W/targets/i61-u1/product_physics > $S/logs/final2_build.log 2>&1 ); echo "== final2_build exit=$?"
grep -E "^warning: " $S/logs/final2_build.log | sort > $S/logs/final2_build.warnings
