#!/bin/bash
# I61 U7 slice L suites on the lane (= WT/f2a-u7 candidate; its milestone fixtures are the live bytes, identical).
set -u
T=WT; S=$T/scratch/i61_u7_slice_l_01; L=$S/lane/projects/chirality-piping
VENV=REPO_ROOT/P/.venv
export TMPDIR=$S/tmp PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN=$T/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson OPENPIPESTRESS_UNITS_BIN=$T/targets/i52-readers/units/release/openpipestress_units
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
SWEEP="tests/test_adapter_framework_contract.py tests/test_analysis_run_schema.py tests/test_analysis_run_records.py tests/test_analysis_status_schema.py tests/test_analysis_run_compatibility.py tests/test_comparison_contracts.py tests/test_handoff_package_schema.py tests/test_handoff_export_workflow.py tests/test_load_reference_headless_artifacts.py tests/test_invented_example_models.py tests/test_load_reference_schema.py tests/test_load_reference_source_schema.py tests/test_persistence_schema.py tests/test_physics_consumer_contract.py tests/test_preview_physics_consumer_contract.py tests/test_precision_consumer_contract.py tests/test_project_persistence_service.py tests/test_result_export_v0_2.py tests/test_stress_neutral_physics_source.py tests/test_results_schema.py tests/test_source_block_schema_contract.py tests/test_stress_neutral_precision.py tests/test_stress_neutral_export_package.py tests/test_retained_precision_contract.py"
RETAINED="tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py tests/test_retained_precision_carriers.py"
case "$1" in
  py_retained) guard; (cd $L && perl -e 'alarm shift; exec @ARGV' 1800 $VENV/bin/python -m pytest -q -p no:cacheprovider --basetemp=$S/tmp/pyr $RETAINED > $S/logs/py_retained.log 2>&1); echo "py_retained exit=$?"; tail -1 $S/logs/py_retained.log ;;
  py_sweep) guard; (cd $L && perl -e 'alarm shift; exec @ARGV' 3600 $VENV/bin/python -m pytest -q -p no:cacheprovider -rA --basetemp=$S/tmp/pys $SWEEP $RETAINED > $S/logs/py_sweep.log 2>&1); echo "py_sweep exit=$?"; tail -1 $S/logs/py_sweep.log ;;
  py_sweep_wt) guard; (cd $T/f2a-u7/projects/chirality-piping && perl -e 'alarm shift; exec @ARGV' 3600 $VENV/bin/python -m pytest -q -p no:cacheprovider -rA --basetemp=$S/tmp/pyw $SWEEP $RETAINED > $S/logs/py_sweep_wt.log 2>&1); echo "py_sweep_wt exit=$?"; tail -1 $S/logs/py_sweep_wt.log ;;
  re) guard; (cd $L/core/reporting/result_export && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 2400 cargo test --locked --offline --no-fail-fast --target-dir $T/targets/i61-u7l/re > $S/logs/re.log 2>&1); echo "re exit=$?"; grep "^test result" $S/logs/re.log | awk '{p+=$4; f+=$6; i+=$8} END {print p" passed, "f" failed, "i" ignored"}' ;;
  vitest) (cd $L/apps/desktop && perl -e 'alarm shift; exec @ARGV' 3600 ../../node_modules/.bin/vitest run --maxWorkers=2 > $S/logs/vitest.log 2>&1); echo "vitest exit=$?"; grep -E "Test Files|Tests " $S/logs/vitest.log ;;
  tsc) (cd $L/apps/desktop && perl -e 'alarm shift; exec @ARGV' 1800 ../../node_modules/.bin/tsc --noEmit -p tsconfig.json > $S/logs/tsc.log 2>&1); echo "tsc exit=$?"; wc -l < $S/logs/tsc.log ;;
esac
