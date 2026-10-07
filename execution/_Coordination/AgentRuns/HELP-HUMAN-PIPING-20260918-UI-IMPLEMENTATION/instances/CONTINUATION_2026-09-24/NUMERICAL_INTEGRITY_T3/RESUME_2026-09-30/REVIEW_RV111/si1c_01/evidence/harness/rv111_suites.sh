#!/bin/bash
# RV111: committed suites on base, candidate and graft (base code + candidate tests), pytest, rustfmt.
# Every cargo command goes through the T3 lock (rv111_job.sh -> t3_cargo.sh, --offline --locked);
# pytest runs under the same lock with the conftest's binary builds bypassed (RV104 N-6).
WT=WT
S=$WT/scratch/rv111_si1c_01
J=$S/harness/rv111_job.sh
P=projects/chirality-piping
VENV=VENV
mkdir -p $S/suites
# The harness is not part of any committed suite.
rm -f $WT/rv111/base/$P/core/rules/rule_check_runner/tests/rv111_si1c.rs $WT/rv111/cand/$P/core/rules/rule_check_runner/tests/rv111_si1c.rs
for tree in base cand graft; do
  $J $tree expression_evaluator suite_${tree}_ee -- test --offline --locked --no-fail-fast; echo "$tree ee rc=$?"
  $J $tree rule_check_runner suite_${tree}_rcr -- test --offline --locked --no-fail-fast; echo "$tree rcr rc=$?"
done
for tree in base cand; do
  $J $tree rule_pack_document suite_${tree}_rpd -- test --offline --locked --no-fail-fast; echo "$tree rpd rc=$?"
done
# pytest: the Python oracle, base and candidate versions, under the lock.
for tree in base cand; do
  ( cd $WT/rv111/$tree/$P && TMPDIR=$S/tmp OPENPIPESTRESS_CHECKED_JSON_BIN=$S/tmp/unused_checked_json_bin \
    OPENPIPESTRESS_UNITS_BIN=$S/tmp/unused_units_bin PYTHONDONTWRITEBYTECODE=1 \
    /usr/bin/lockf -k $WT/guard/cargo_job.lock $VENV/bin/python -m pytest -q -p no:cacheprovider tests/test_rule_interval.py \
    > $S/logs/pytest_$tree.log 2>&1 ); echo "$tree pytest rc=$?"
done
# rustfmt (not cargo): the evaluator crate at the candidate, and the runner's lib on both sides.
for f in core/rules/expression_evaluator/src/lib.rs core/rules/rule_check_runner/tests/point_path_non_finite_run.rs; do
  rustfmt --check --edition 2021 $WT/rv111/cand/$P/$f > $S/logs/rustfmt_cand_$(basename $(dirname $(dirname $f)))_$(basename $f).log 2>&1; echo "rustfmt cand $f rc=$?"
done
for tree in base cand; do
  rustfmt --check --edition 2021 $WT/rv111/$tree/$P/core/rules/rule_check_runner/src/lib.rs > $S/logs/rustfmt_${tree}_rcr_lib.log 2>&1; echo "rustfmt $tree rcr lib rc=$?"
done
