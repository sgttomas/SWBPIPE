#!/bin/bash
# RV104 ADDENDUM_01 scratch chain (not repository content). Every cargo via the T3 lock.
set -u
WT=WT
S=$WT/scratch/rv104_si1b_01; A=$S/a1; P=projects/chirality-piping
export TMPDIR=$S/tmp
LOG=$A/logs/chain.log
run() { local label=$1 dir=$2 target=$3; shift 3
  echo "$(date -u '+%FT%TZ') begin $label" >> $LOG
  (cd "$dir" && CARGO_TARGET_DIR=$WT/targets/$target "$WT/tools/t3_cargo.sh" "$@" > "$A/logs/$label.log" 2>&1)
  echo "$(date -u '+%FT%TZ') end $label rc=$?" >> $LOG; }
# 1. Committed suites at the repair (no harness files present yet), and fmt.
for crate in expression_evaluator rule_check_runner rule_pack_document; do
  run "suite_rep_$crate" "$WT/rv104/rep/$P/core/rules/$crate" rv104-rep test --offline --locked
done
run fmt_rep_ee "$WT/rv104/rep/$P/core/rules/expression_evaluator" rv104-rep fmt --check
(cd "$WT/rv104/rep/$P/core/rules/expression_evaluator" && CARGO_TARGET_DIR=$WT/targets/rv104-rep "$WT/tools/t3_cargo.sh" test --offline --locked --lib -- --list 2>/dev/null | grep -E ": test$" | sort > $A/logs/list_rep_ee_lib.txt)
(cd "$WT/rv104/prev/$P/core/rules/expression_evaluator" && CARGO_TARGET_DIR=$WT/targets/rv104-prev "$WT/tools/t3_cargo.sh" test --offline --locked --lib -- --list 2>/dev/null | grep -E ": test$" | sort > $A/logs/list_prev_ee_lib.txt)
# 2. Mutants at the repair.
echo "$(date -u '+%FT%TZ') begin mutants" >> $LOG
python3 $A/rv104_mutants_a1.py $WT $A/reports/mutants_a1.json > $A/logs/mutants_driver.log 2>&1
echo "$(date -u '+%FT%TZ') end mutants rc=$?" >> $LOG
# 3. The differential harnesses on 966113396e (prev) and 0730c87aef (rep).
for side in prev rep; do
  cp $S/harness/rv104_ee_diff.rs $S/harness/rv104_run_diff.rs "$WT/rv104/$side/$P/core/rules/rule_check_runner/tests/"
  RV104_EE_OUT=$A/dumps/ee_$side.txt run "ee_$side" "$WT/rv104/$side/$P/core/rules/rule_check_runner" "rv104-$side" test --offline --locked --test rv104_ee_diff
  RV104_RUN_OUT=$A/dumps/run_$side.txt run "run_$side" "$WT/rv104/$side/$P/core/rules/rule_check_runner" "rv104-$side" test --offline --locked --test rv104_run_diff
done
echo "$(date -u '+%FT%TZ') chain done" >> $LOG
