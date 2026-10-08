#!/bin/bash
# Usage: suites.sh <label> <P root> [py|ts|tsc|rs ...]
# Heavy runs under the shared T3 lock (Python, vitest, tsc); cargo through t3_cargo.sh (which takes the lock itself).
set -u
source "$(dirname "$0")/env.sh"
LABEL=$1; PROOT=$2; shift 2
LOCK=$WT/guard/cargo_job.lock
SWEEP="tests/test_adapter_framework_contract.py tests/test_analysis_run_schema.py tests/test_analysis_run_records.py tests/test_analysis_status_schema.py tests/test_analysis_run_compatibility.py tests/test_comparison_contracts.py tests/test_handoff_package_schema.py tests/test_handoff_export_workflow.py tests/test_load_reference_headless_artifacts.py tests/test_invented_example_models.py tests/test_load_reference_schema.py tests/test_load_reference_source_schema.py tests/test_persistence_schema.py tests/test_physics_consumer_contract.py tests/test_preview_physics_consumer_contract.py tests/test_precision_consumer_contract.py tests/test_project_persistence_service.py tests/test_result_export_v0_2.py tests/test_stress_neutral_physics_source.py tests/test_results_schema.py tests/test_source_block_schema_contract.py tests/test_stress_neutral_precision.py tests/test_stress_neutral_export_package.py tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py tests/test_retained_precision_carriers.py tests/test_results_dispatcher_v0_3.py"
mkdir -p $S/logs/$LABEL $S/tmp/$LABEL
for what in "$@"; do
  case $what in
    py)
      cd "$PROOT" && /usr/bin/lockf -k $LOCK env TMPDIR=$S/tmp/$LABEL $VENV/bin/python -m pytest -p no:cacheprovider --basetemp=$S/tmp/$LABEL/basetemp -q -rs -rf -v --junitxml=$S/logs/$LABEL/py.xml $SWEEP > $S/logs/$LABEL/py.log 2>&1
      echo "py rc=$?" >> $S/logs/$LABEL/rc.txt ;;
    ts)
      cd "$PROOT/apps/desktop" && /usr/bin/lockf -k $LOCK env TMPDIR=$S/tmp/$LABEL ../../node_modules/.bin/vitest run --reporter=dot --reporter=json --outputFile.json=$S/logs/$LABEL/vitest.json > $S/logs/$LABEL/vitest.log 2>&1
      echo "ts rc=$?" >> $S/logs/$LABEL/rc.txt
      [ -L "$PROOT/apps/desktop/node_modules" ] || { rm -rf "$PROOT/apps/desktop/node_modules/.vite"; rmdir "$PROOT/apps/desktop/node_modules" 2>/dev/null; } ;;
    tsc)
      cd "$PROOT/apps/desktop" && /usr/bin/lockf -k $LOCK env TMPDIR=$S/tmp/$LABEL ../../node_modules/.bin/tsc --noEmit -p tsconfig.json > $S/logs/$LABEL/tsc.log 2>&1
      echo "tsc rc=$?" >> $S/logs/$LABEL/rc.txt ;;
    rs)
      cd "$PROOT/core/reporting/result_export" && env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$WT/targets/i83-b6/rx TMPDIR=$S/tmp/$LABEL $WT/tools/t3_cargo.sh test --locked --offline --no-fail-fast > $S/logs/$LABEL/rs.log 2>&1
      echo "rs rc=$?" >> $S/logs/$LABEL/rc.txt ;;
  esac
done
echo done >> $S/logs/$LABEL/rc.txt
