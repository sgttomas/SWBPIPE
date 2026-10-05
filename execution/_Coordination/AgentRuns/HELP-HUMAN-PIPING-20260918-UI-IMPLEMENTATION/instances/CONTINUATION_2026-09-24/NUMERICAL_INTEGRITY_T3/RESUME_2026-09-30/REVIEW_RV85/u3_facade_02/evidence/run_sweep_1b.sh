#!/bin/bash
set -u
W=WT
S=$W/scratch/rv85_u3_facade_02; L=$S/logs
for t in "$@"; do
  pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING" >> $L/progress.txt; exit 9; }
  echo "== sweep_$t start $(date -u +%FT%TZ)" >> $L/progress.txt
  extra=""; [ $t = cand ] && extra="--test zz_rv85_carrier"
  ( cd $W/rv85/$t/projects/chirality-piping/core/product_physics && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 RV85_SWEEP_LIST=$S/sweep/list.txt RV85_SWEEP_OUT=$S/sweep/sweep_$t.tsv RV85_CARRIER_OUT=$S/sweep/carrier_$t.tsv perl -e 'alarm shift; exec @ARGV' 2400 cargo test --locked --offline --target-dir $W/targets/rv85/g1b/$t-pp --test zz_rv85_sweep $extra > $L/sweep_$t.log 2>&1 ); echo "== sweep_$t exit=$? $(date -u +%FT%TZ)" >> $L/progress.txt
done
