#!/bin/bash
# Usage: run_py.sh <P root> <label> <sweep|retained>
# Runs the Python suites with the two CLI authorities; scratch-only temp; no cache or bytecode writes.
set -u
WT=WT
VENV=VENV
S=$WT/scratch/i69_u8
PROOT=$1; LABEL=$2; WHAT=$3
SWEEP="tests/test_adapter_framework_contract.py tests/test_analysis_run_schema.py tests/test_analysis_run_records.py tests/test_analysis_status_schema.py tests/test_analysis_run_compatibility.py tests/test_comparison_contracts.py tests/test_handoff_package_schema.py tests/test_handoff_export_workflow.py tests/test_load_reference_headless_artifacts.py tests/test_invented_example_models.py tests/test_load_reference_schema.py tests/test_load_reference_source_schema.py tests/test_persistence_schema.py tests/test_physics_consumer_contract.py tests/test_preview_physics_consumer_contract.py tests/test_precision_consumer_contract.py tests/test_project_persistence_service.py tests/test_result_export_v0_2.py tests/test_stress_neutral_physics_source.py tests/test_results_schema.py tests/test_source_block_schema_contract.py tests/test_stress_neutral_precision.py tests/test_stress_neutral_export_package.py tests/test_retained_precision_contract.py"
RETAINED="tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py tests/test_retained_precision_carriers.py"
if [ "$WHAT" = sweep ]; then FILES="$SWEEP tests/test_retained_precision_schema.py tests/test_retained_precision_carriers.py"; else FILES="$RETAINED"; fi
mkdir -p $S/tmp/$LABEL $S/logs
cd "$PROOT"
env OPENPIPESTRESS_UNITS_BIN=$WT/targets/i69-u8/units-authority/release/openpipestress_units \
    OPENPIPESTRESS_CHECKED_JSON_BIN=$WT/targets/i69-u8/checked-json/release/openpipestress_jcs_ijson \
    PYTHONDONTWRITEBYTECODE=1 TMPDIR=$S/tmp/$LABEL \
    $VENV/bin/python -m pytest -p no:cacheprovider --basetemp=$S/tmp/$LABEL/basetemp -q -rs -rf -v $FILES > $S/logs/$LABEL.log 2>&1
rc=$?
echo "rc=$rc" >> $S/logs/$LABEL.log
exit $rc
