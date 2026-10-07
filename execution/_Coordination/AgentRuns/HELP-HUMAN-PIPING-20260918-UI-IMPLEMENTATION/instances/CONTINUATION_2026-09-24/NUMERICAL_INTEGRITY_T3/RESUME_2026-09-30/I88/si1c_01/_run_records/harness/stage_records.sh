#!/bin/bash
# I88: stage sanitized run records (scratch tool). Usage: stage_records.sh
set -eu
WT=WT
S=$WT/scratch/i88_si1c
RS=$S/records_stage/_run_records
san() { python3 "$S/harness/sanitize_copy.py" "$1" "$2"; }
mkdir -p $RS/{harness,instrumented_base,differential,suites,mutants,host}
for f in si1c_family.rs rv104_run_full.rs si1c_instrument_base.py si1c_compare.py si1c_status_tally.py \
         si1c_mutants.py n5_comment_only.py si1c_driver.sh wt_check.sh sanitize_copy.py stage_records.sh; do
  san "$S/harness/$f" "$RS/harness/$f"
done
if [ -d "$S/trees" ]; then cd "$S/trees"
{ diff -u base/projects/chirality-piping/core/rules/expression_evaluator/src/lib.rs \
          ibase/projects/chirality-piping/core/rules/expression_evaluator/src/lib.rs || true
  diff -u base/projects/chirality-piping/core/rules/rule_check_runner/src/lib.rs \
          ibase/projects/chirality-piping/core/rules/rule_check_runner/src/lib.rs || true; } > "$S/tmp/ibase.diff"
san "$S/tmp/ibase.diff" "$RS/instrumented_base/ibase_vs_base.diff"; fi
for f in diff_report.json status_transitions.json bounded_lines.json; do san "$S/reports/$f" "$RS/differential/$f"; done
san "$S/records_stage/dump_sha256.txt" "$RS/differential/dump_sha256.txt"
[ -f "$S/records_stage/n5_code_only_check.txt" ] && san "$S/records_stage/n5_code_only_check.txt" "$RS/differential/n5_code_only_check.txt"
for t in base cand; do
  for c in expression_evaluator rule_check_runner rule_pack_document; do
    san "$S/logs/suite_${t}_$c.log" "$RS/suites/suite_${t}_$c.log"
    san "$S/logs/list_${t}_$c.log" "$RS/suites/list_${t}_$c.log"
  done
  san "$S/logs/pytest_$t.log" "$RS/suites/pytest_$t.log"
done
for c in expression_evaluator rule_check_runner rule_pack_document; do san "$S/logs/wt_$c.log" "$RS/suites/wt_$c.log"; done
san "$S/logs/fmt_cand_ee.log" "$RS/suites/fmt_cand_ee.log"
for f in "$S"/records_stage/suites/rustfmt_check_*.txt; do san "$f" "$RS/suites/$(basename "$f")"; done
[ -f "$S/records_stage/test_name_delta.txt" ] && san "$S/records_stage/test_name_delta.txt" "$RS/suites/test_name_delta.txt"
san "$S/logs/driver.log" "$RS/host/driver.log"
for f in bins_checked_json bins_units; do san "$S/logs/$f.log" "$RS/host/$f.log"; done
grep -E "i88|I88" "$WT/guard/cargo_jobs.log" > "$S/tmp/cargo_jobs_i88.txt" || true
san "$S/tmp/cargo_jobs_i88.txt" "$RS/host/cargo_jobs_i88.txt"
if [ -f "$S/reports/mutants.json" ]; then
  san "$S/reports/mutants.json" "$RS/mutants/mutants.json"
  [ -f "$S/records_stage/mutant_logs_summary.txt" ] && san "$S/records_stage/mutant_logs_summary.txt" "$RS/mutants/mutant_logs_summary.txt"
  [ -f "$S/records_stage/mutant_dump_identity.txt" ] && san "$S/records_stage/mutant_dump_identity.txt" "$RS/mutants/mutant_dump_identity.txt"
fi
cp "$S/records_stage/harness_sha256.txt" "$RS/harness/harness_sha256.txt"
echo staged
