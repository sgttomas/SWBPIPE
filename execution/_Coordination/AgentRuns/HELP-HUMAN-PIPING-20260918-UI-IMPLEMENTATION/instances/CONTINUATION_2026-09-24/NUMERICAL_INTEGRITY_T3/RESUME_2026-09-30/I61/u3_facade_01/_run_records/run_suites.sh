#!/bin/bash
set -u
W=WT; S=$W/scratch/i61_u3_facade_01
C=$W/f2a-facade/projects/chirality-piping/core; B=$S/base/projects/chirality-piping/core
run() {
  pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  echo "== $1 start $(date -u +%FT%TZ)"
  ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"
  awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sort > $S/logs/$1.outcomes
}
build() {
  ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 perl -e 'alarm shift; exec @ARGV' 1200 cargo build --locked --offline --lib --manifest-path $2 --target-dir $3 > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$?"
  grep -E "^warning: " $S/logs/$1.log | sort > $S/logs/$1.warnings
}
build base_build $B/product_physics/Cargo.toml $W/targets/i61-u3/base-pp
build cand_build $C/product_physics/Cargo.toml $W/targets/i61-u3/product_physics
run base_pp $B/product_physics/Cargo.toml $W/targets/i61-u3/base-pp
run cand_pp $C/product_physics/Cargo.toml $W/targets/i61-u3/product_physics
run base_runner $B/runner/headless/Cargo.toml $W/targets/i61-u3/base-runner
run cand_runner $C/runner/headless/Cargo.toml $W/targets/i61-u3/runner
