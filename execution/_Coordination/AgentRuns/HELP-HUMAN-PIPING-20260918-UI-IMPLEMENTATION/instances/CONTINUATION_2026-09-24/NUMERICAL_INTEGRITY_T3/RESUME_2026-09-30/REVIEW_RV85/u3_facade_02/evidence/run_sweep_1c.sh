#!/bin/bash
set -u
W=WT
S=$W/scratch/rv85_u3_facade_02; L=$S/c1c/logs
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING" >> $L/progress.txt; exit 9; }
echo "== sweep_c1c start $(date -u +%FT%TZ)" >> $L/progress.txt
( cd $W/rv85/c1c/projects/chirality-piping/core/product_physics && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 RV85_SWEEP_LIST=$S/sweep/list.txt RV85_SWEEP_OUT=$S/c1c/sweep/sweep_c1c.tsv RV85_CARRIER_OUT=$S/c1c/sweep/carrier_c1c.tsv perl -e 'alarm shift; exec @ARGV' 2400 cargo test --locked --offline --target-dir $W/targets/rv85/g1c/cand-pp --test zz_rv85_sweep --test zz_rv85_carrier > $L/sweep_c1c.log 2>&1 ); echo "== sweep_c1c exit=$? $(date -u +%FT%TZ)" >> $L/progress.txt
