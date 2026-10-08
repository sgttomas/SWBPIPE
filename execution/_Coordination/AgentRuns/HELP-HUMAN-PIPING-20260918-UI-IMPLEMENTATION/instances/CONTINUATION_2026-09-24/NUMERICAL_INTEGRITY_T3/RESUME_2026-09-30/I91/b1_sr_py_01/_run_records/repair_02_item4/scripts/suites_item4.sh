#!/bin/bash
# Usage: suites_item4.sh <label> <P root>. I83's Python set (I69's set plus the retained files), as one T3 slot job (t3_slot.sh),
# with I91's own builds of the two CLI authorities.
set -u
source WT/scratch/i91_b1_sr_py/env.sh
LABEL=$1; PROOT=$2
LOCK=$WT/guard/cargo_job.lock
SWEEP="tests/test_adapter_framework_contract.py tests/test_analysis_run_schema.py tests/test_analysis_run_records.py tests/test_analysis_status_schema.py tests/test_analysis_run_compatibility.py tests/test_comparison_contracts.py tests/test_handoff_package_schema.py tests/test_handoff_export_workflow.py tests/test_load_reference_headless_artifacts.py tests/test_invented_example_models.py tests/test_load_reference_schema.py tests/test_load_reference_source_schema.py tests/test_persistence_schema.py tests/test_physics_consumer_contract.py tests/test_preview_physics_consumer_contract.py tests/test_precision_consumer_contract.py tests/test_project_persistence_service.py tests/test_result_export_v0_2.py tests/test_stress_neutral_physics_source.py tests/test_results_schema.py tests/test_source_block_schema_contract.py tests/test_stress_neutral_precision.py tests/test_stress_neutral_export_package.py tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py tests/test_retained_precision_carriers.py tests/test_results_dispatcher_v0_3.py"
mkdir -p $S/logs/$LABEL $S/tmp/$LABEL
cd "$PROOT" && $WT/tools/t3_slot.sh env TMPDIR=$S/tmp/$LABEL $VENV/bin/python -m pytest -p no:cacheprovider --basetemp=$S/tmp/$LABEL/basetemp -q -rs -rf -v --junitxml=$S/logs/$LABEL/py.xml $SWEEP > $S/logs/$LABEL/py.log 2>&1
echo "py rc=$?" >> $S/logs/$LABEL/rc.txt
echo done >> $S/logs/$LABEL/rc.txt
