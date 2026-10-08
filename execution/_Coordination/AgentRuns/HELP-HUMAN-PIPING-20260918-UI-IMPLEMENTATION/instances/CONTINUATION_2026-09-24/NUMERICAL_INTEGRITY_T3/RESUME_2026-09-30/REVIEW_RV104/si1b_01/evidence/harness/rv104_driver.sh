#!/bin/bash
# RV104 scratch driver (not repository content): runs RV104's cargo jobs in
# sequence, each through the T3 host lock (WT/tools/t3_cargo.sh).
# Usage: rv104_driver.sh STEP...   (steps: ee_base run_cand run_base suites graft)
set -u
WT=WT
S=$WT/scratch/rv104_si1b_01
P=projects/chirality-piping
export TMPDIR=$S/tmp
LOG=$S/logs/driver.log

run() { # label crate_dir target_name cargo-args...
  local label=$1 dir=$2 target=$3; shift 3
  echo "$(date -u '+%FT%TZ') begin $label" >> "$LOG"
  (cd "$dir" && CARGO_TARGET_DIR=$WT/targets/$target "$WT/tools/t3_cargo.sh" "$@" > "$S/logs/$label.log" 2>&1)
  local rc=$?
  echo "$(date -u '+%FT%TZ') end $label rc=$rc" >> "$LOG"
}

for step in "$@"; do
  case $step in
    ee_base)
      RV104_EE_OUT=$S/dumps/ee_base.txt run ee_base "$WT/rv104/base/$P/core/rules/rule_check_runner" rv104-base \
        test --offline --locked --test rv104_ee_diff ;;
    run_cand)
      RV104_RUN_OUT=$S/dumps/run_cand.txt run run_cand "$WT/rv104/cand/$P/core/rules/rule_check_runner" rv104-cand \
        test --offline --locked --test rv104_run_diff ;;
    run_base)
      RV104_RUN_OUT=$S/dumps/run_base.txt run run_base "$WT/rv104/base/$P/core/rules/rule_check_runner" rv104-base \
        test --offline --locked --test rv104_run_diff ;;
    suites)
      # The committed suites only: RV104's harness files are moved out first.
      for side in base cand; do
        mkdir -p "$S/harness_parked/$side"
        mv "$WT/rv104/$side/$P/core/rules/rule_check_runner/tests/rv104_"*.rs "$S/harness_parked/$side/" 2>/dev/null
        for crate in expression_evaluator rule_check_runner rule_pack_document; do
          run "suite_${side}_${crate}" "$WT/rv104/$side/$P/core/rules/$crate" "rv104-$side" \
            test --offline --locked
        done
      done ;;
    graft)
      run graft_ee_lib "$WT/rv104/graft/$P/core/rules/expression_evaluator" rv104-graft \
        test --offline --locked --lib
      run graft_rcr "$WT/rv104/graft/$P/core/rules/rule_check_runner" rv104-graft \
        test --offline --locked --test point_path_non_finite_run ;;
    fmt)
      run fmt_cand_ee "$WT/rv104/cand/$P/core/rules/expression_evaluator" rv104-cand fmt --check ;;
    *) echo "unknown step $step" >> "$LOG" ;;
  esac
done
echo "$(date -u '+%FT%TZ') driver done: $*" >> "$LOG"
