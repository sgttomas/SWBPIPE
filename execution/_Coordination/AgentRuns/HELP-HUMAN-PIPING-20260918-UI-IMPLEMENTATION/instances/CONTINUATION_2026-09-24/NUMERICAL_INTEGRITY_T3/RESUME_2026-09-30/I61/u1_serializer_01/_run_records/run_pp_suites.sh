#!/bin/bash
# Protected byte control 1: PP lib + integration suites, base (NUM 43a6368c21 archive) then candidate.
set -u
W=WT; S=$W/scratch/i61_u1_serializer_01
run() { # label manifest target
  pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  echo "== $1 start $(date -u +%FT%TZ)"
  ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"
  awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sort > $S/logs/$1.outcomes
  grep -E "Running |test result" $S/logs/$1.log | paste - - | sed 's#'$W'#WT#g' | awk '{print}' > $S/logs/$1.summary
}
build() { # label manifest target: the production (non-test) library build and its warnings
  pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 perl -e 'alarm shift; exec @ARGV' 1200 cargo build --locked --offline --lib --manifest-path $2 --target-dir $3 > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$?"
  grep -E "^warning: " $S/logs/$1.log | sort > $S/logs/$1.warnings
}
build base_build $S/lane/projects/chirality-piping/core/product_physics/Cargo.toml $W/targets/i61-u1/base-pp
build cand_build $W/f2a-serializer/projects/chirality-piping/core/product_physics/Cargo.toml $W/targets/i61-u1/product_physics
run base_pp $S/lane/projects/chirality-piping/core/product_physics/Cargo.toml $W/targets/i61-u1/base-pp
run cand_pp $W/f2a-serializer/projects/chirality-piping/core/product_physics/Cargo.toml $W/targets/i61-u1/product_physics
