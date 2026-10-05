#!/bin/zsh
# RV92: sequential cargo jobs after the Rust sweep (one cargo job at a time).
WT=WT; S=$WT/scratch/rv92_u6f
VENV=REPO_ROOT/projects/chirality-piping/.venv
export TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; } }
RE=projects/chirality-piping/core/reporting/result_export
for L in base cand; do guard
  (cd $WT/rv92/$L/$RE && RV92_SWEEP=$S/extra_inputs RV92_OUT=$S/extra_rust_$L.jsonl CARGO_TARGET_DIR=$WT/targets/rv92/re_$L cargo test --locked --offline --test zz_rv92_sweep > $S/extra_rust_$L.log 2>&1)
done
guard
(cd $WT/rv92/cand/$RE && RV92_SURVIVAL=$S/survival RV92_STEP=a CARGO_TARGET_DIR=$WT/targets/rv92/re_cand cargo test --locked --offline --test zz_rv92_survival > $S/survival_rust_a.log 2>&1)
(cd $WT/rv92/cand/$RE && RV92_SURVIVAL=$S/survival RV92_STEP=b CARGO_TARGET_DIR=$WT/targets/rv92/re_cand cargo test --locked --offline --test zz_rv92_survival > $S/survival_rust_b.log 2>&1)
(cd $WT/rv92/cand/projects/chirality-piping && PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN=$WT/targets/rv92/cli/release/openpipestress_jcs_ijson OPENPIPESTRESS_UNITS_BIN=$WT/targets/rv92/cli/release/openpipestress_units $VENV/bin/python $S/rv92_py_survival.py $WT/rv92/cand/projects/chirality-piping $S/survival b > $S/survival_py_b.log 2>&1)
(cd $WT/rv92/cand/projects/chirality-piping/apps/desktop && RV92_SURVIVAL=$S/survival RV92_STEP=b ../../node_modules/.bin/vitest run --maxWorkers=1 src/zzRV92Survival.test.ts > $S/survival_ts_b.log 2>&1)
echo "survival done $(date -u +%FT%TZ)"
guard
(cd $WT/rv92/merge/projects/chirality-piping/core/product_physics && CARGO_TARGET_DIR=$WT/targets/rv92/pp_merge cargo test --locked --offline --no-fail-fast --lib -- retained_wire_tests retained_facade_tests > $S/pp_merge.log 2>&1; echo "exit=$?" >> $S/pp_merge.log)
echo "pp done $(date -u +%FT%TZ)"
for L in cand base; do guard
  (cd $WT/rv92/$L/projects/chirality-piping/core/runner/headless && CARGO_TARGET_DIR=$WT/targets/rv92/runner_$L cargo test --locked --offline --no-fail-fast > $S/runner_$L.log 2>&1; echo "exit=$?" >> $S/runner_$L.log)
done
echo "chain done $(date -u +%FT%TZ)"
guard
(cd $WT/rv92/cand/$RE && RV92_EXTRA_OUT=$S/extra_rust_checks_debug.jsonl CARGO_TARGET_DIR=$WT/targets/rv92/re_cand cargo test --locked --offline --test zz_rv92_extra > $S/extra_rust_checks_debug.log 2>&1)
(cd $WT/rv92/cand/$RE && RV92_EXTRA_OUT=$S/extra_rust_checks_release.jsonl CARGO_TARGET_DIR=$WT/targets/rv92/re_cand_release cargo test --locked --offline --release --test zz_rv92_extra > $S/extra_rust_checks_release.log 2>&1)
echo "extra done $(date -u +%FT%TZ)"
