#!/bin/zsh
# RV92 confirmation: sequential cargo jobs on the post-U6f head (one cargo job at a time).
WT=WT; S=$WT/scratch/rv92_u6f
VENV=REPO_ROOT/projects/chirality-piping/.venv
export TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; } }
RE=$WT/rv92/cand2/projects/chirality-piping/core/reporting/result_export
guard; (cd $RE && RV92_SURVIVAL=$S/r2/survival RV92_STEP=a CARGO_TARGET_DIR=$WT/targets/rv92/re_cand2 cargo test --locked --offline --test zz_rv92_survival > $S/r2/survival_rust_a.log 2>&1)
guard; (cd $RE && RV92_SURVIVAL=$S/r2/survival RV92_STEP=b CARGO_TARGET_DIR=$WT/targets/rv92/re_cand2 cargo test --locked --offline --test zz_rv92_survival > $S/r2/survival_rust_b.log 2>&1)
echo "survival rust done $(date -u +%FT%TZ)"
guard; (cd $RE && RV92_PROBES=$S/r2/probes RV92_OUT=$S/r2/parity_rust.jsonl CARGO_TARGET_DIR=$WT/targets/rv92/re_cand2 cargo test --locked --offline --test zz_rv92_parity > $S/r2/parity_rust.log 2>&1; echo "exit=$?" >> $S/r2/parity_rust.log)
echo "parity rust done $(date -u +%FT%TZ)"
guard; (cd $RE && RV92_SWEEP=$S/sweep_inputs RV92_OUT=$S/r2/sweep_rust_cand2.jsonl CARGO_TARGET_DIR=$WT/targets/rv92/re_cand2 cargo test --locked --offline --test zz_rv92_sweep > $S/r2/sweep_rust_cand2.log 2>&1; echo "exit=$?" >> $S/r2/sweep_rust_cand2.log)
echo "sweep rust done $(date -u +%FT%TZ)"
