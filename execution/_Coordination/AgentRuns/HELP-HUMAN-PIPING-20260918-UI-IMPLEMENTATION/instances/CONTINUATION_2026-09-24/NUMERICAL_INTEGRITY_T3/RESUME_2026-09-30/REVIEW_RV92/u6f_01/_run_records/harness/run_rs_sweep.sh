#!/bin/zsh
WT=WT; S=$WT/scratch/rv92_u6f
export TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2
for L in base cand; do
  pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  (cd $WT/rv92/$L/projects/chirality-piping/core/reporting/result_export && RV92_SWEEP=$S/sweep_inputs RV92_OUT=$S/sweep_rust_$L.jsonl CARGO_TARGET_DIR=$WT/targets/rv92/re_$L cargo test --locked --offline --test zz_rv92_sweep > $S/sweep_rust_$L.log 2>&1; echo "exit=$?" >> $S/sweep_rust_$L.log)
done
