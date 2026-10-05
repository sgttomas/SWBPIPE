#!/bin/bash
set -u
W=WT; S=$W/scratch/i61_u1_serializer_02; C=$W/f2a-serializer/projects/chirality-piping/core
rm -f $S/logs/mutants_summary.txt; python3 $S/mutants.py; echo "mutants done $(date -u +%T)"
run() {
  pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  echo "== $1 start $(date -u +%FT%TZ)"
  ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"
  awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sort > $S/logs/$1.outcomes
}
run final3_pp $C/product_physics/Cargo.toml $W/targets/i61-u1/product_physics
run final3_runner $C/runner/headless/Cargo.toml $W/targets/i61-u1/runner
( cd $C/product_physics && env CARGO_BUILD_JOBS=4 perl -e 'alarm shift; exec @ARGV' 1200 cargo build --locked --offline --lib --target-dir $W/targets/i61-u1/product_physics > $S/logs/final3_build.log 2>&1 ); echo "== final3_build exit=$?"
grep -E "^warning: " $S/logs/final3_build.log | sort > $S/logs/final3_build.warnings
