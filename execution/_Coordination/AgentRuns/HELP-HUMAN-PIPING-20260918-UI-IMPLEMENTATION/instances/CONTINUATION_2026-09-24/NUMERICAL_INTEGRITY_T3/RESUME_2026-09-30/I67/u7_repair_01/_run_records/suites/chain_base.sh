#!/bin/bash
# I67 U7 repair: base runs at 8c84e7ae14 (lane base9): vitest+tsc, Python retained suites, result_export.
WT=WT; S=$WT/scratch/i67_u6d; P=$S/lanes/base9/projects/chirality-piping
$S/run_suite.sh $P/apps/desktop $S/r9/base
$S/r9/run_py.sh py_base $P tests/test_retained_precision_contract.py tests/test_retained_precision_carriers.py tests/test_retained_precision_schema.py
$S/r9/cargo_run.sh re_base $P/core/reporting/result_export/Cargo.toml $WT/targets/i67-u7f/re-base9
echo done > $S/r9/base.done
