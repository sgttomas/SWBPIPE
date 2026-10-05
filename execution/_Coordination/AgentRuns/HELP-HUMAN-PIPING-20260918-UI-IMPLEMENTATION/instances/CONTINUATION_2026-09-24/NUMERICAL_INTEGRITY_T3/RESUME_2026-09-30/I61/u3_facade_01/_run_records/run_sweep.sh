#!/bin/bash
set -u
W=WT; S=$W/scratch/i61_u3_facade_01
for pair in base:base-pp cand:product_physics; do
  a=${pair%%:*}; t=${pair##*:}
  pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  M=$S/$a/projects/chirality-piping/core/product_physics/Cargo.toml
  echo "== sweep_$a start $(date -u +%FT%TZ)"
  ( cd $(dirname $M) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 I61_U3_SWEEP=$S/sweep/sweep_$a.tsv perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --manifest-path $M --target-dir $W/targets/i61-u3/$t --test zz_i61_u3_fixture_sweep -- --ignored --nocapture > $S/logs/sweep_$a.log 2>&1 ); echo "== sweep_$a exit=$? $(date -u +%FT%TZ)"
done
