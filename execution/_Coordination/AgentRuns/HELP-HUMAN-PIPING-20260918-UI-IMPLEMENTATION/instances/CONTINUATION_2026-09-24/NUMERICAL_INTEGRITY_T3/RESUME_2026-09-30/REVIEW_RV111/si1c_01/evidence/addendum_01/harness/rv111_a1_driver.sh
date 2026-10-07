#!/bin/bash
# RV111 addendum 01 driver: harness on the repaired head, the committed suites, pytest, then the mutants.
# Every cargo command goes through WT/tools/t3_cargo.sh (--offline --locked) via rv111_job.sh or
# rv111_a1_mutants.py; pytest runs under the T3 lock.
WT=WT
S=$WT/scratch/rv111_si1c_01
J=$S/harness/rv111_job.sh
P=projects/chirality-piping
VENV=VENV
export TMPDIR=$S/tmp
cd "$S" || exit 2
H=$WT/rv111/head2/$P/core/rules/rule_check_runner/tests
cp "$S/harness/rv111_si1c.rs" "$S/harness/rv111_probe.rs" "$H/"
$J head2 rule_check_runner a1_harness -- test --offline --locked --test rv111_si1c --test rv111_probe -- --test-threads=2
echo "harness rc=$?"
rm -f "$H/rv111_si1c.rs" "$H/rv111_probe.rs"
$J head2 expression_evaluator a1_suite_ee -- test --offline --locked --no-fail-fast; echo "suite ee rc=$?"
$J head2 rule_check_runner a1_suite_rcr -- test --offline --locked --no-fail-fast; echo "suite rcr rc=$?"
$J head2 rule_pack_document a1_suite_rpd -- test --offline --locked --no-fail-fast; echo "suite rpd rc=$?"
( cd "$WT/rv111/head2/$P" && TMPDIR=$S/tmp OPENPIPESTRESS_CHECKED_JSON_BIN=$S/tmp/unused_checked_json_bin \
  OPENPIPESTRESS_UNITS_BIN=$S/tmp/unused_units_bin PYTHONDONTWRITEBYTECODE=1 \
  /usr/bin/lockf -k "$WT/guard/cargo_job.lock" "$VENV/bin/python" -m pytest -v -p no:cacheprovider tests/test_rule_interval.py \
  > "$S/logs/a1_pytest.log" 2>&1 ); echo "pytest rc=$?"
"$VENV/bin/python" "$S/harness/rv111_a1_mutants.py" > "$S/logs/a1_mutants.log" 2>&1; echo "mutants rc=$?"
