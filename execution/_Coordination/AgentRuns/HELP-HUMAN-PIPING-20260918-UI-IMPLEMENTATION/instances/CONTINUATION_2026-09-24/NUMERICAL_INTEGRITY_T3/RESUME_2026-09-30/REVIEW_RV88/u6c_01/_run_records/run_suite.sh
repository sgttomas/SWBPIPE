#!/bin/bash
# RV88 U6c: the 24-file schema sweep plus the retained schema test, in one lane. Args: lane_P out_log
set -u
WT=WT
VENV=REPO_ROOT/projects/chirality-piping/.venv
pgrep -f memguard.sh >/dev/null || { echo NOGUARD; exit 9; }
export TMPDIR=$WT/scratch/rv88_u6/u6d/tmp PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN=$WT/targets/rv88/canonical_json/release/openpipestress_jcs_ijson OPENPIPESTRESS_UNITS_BIN=$WT/targets/rv88/units/release/openpipestress_units
FILES=(tests/test_adapter_framework_contract.py tests/test_analysis_run_schema.py tests/test_analysis_run_records.py tests/test_analysis_status_schema.py tests/test_analysis_run_compatibility.py tests/test_comparison_contracts.py tests/test_handoff_package_schema.py tests/test_handoff_export_workflow.py tests/test_load_reference_headless_artifacts.py tests/test_invented_example_models.py tests/test_load_reference_schema.py tests/test_load_reference_source_schema.py tests/test_persistence_schema.py tests/test_physics_consumer_contract.py tests/test_preview_physics_consumer_contract.py tests/test_precision_consumer_contract.py tests/test_project_persistence_service.py tests/test_result_export_v0_2.py tests/test_stress_neutral_physics_source.py tests/test_results_schema.py tests/test_source_block_schema_contract.py tests/test_stress_neutral_precision.py tests/test_stress_neutral_export_package.py tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py)
cd $1
present=(); for f in "${FILES[@]}"; do [ -f "$f" ] && present+=("$f"); done
perl -e 'alarm shift; exec @ARGV' 2400 $VENV/bin/python -m pytest -q -p no:cacheprovider -rA "${present[@]}" > $2 2>&1
echo "rc=$? files=${#present[@]}"; tail -1 $2
