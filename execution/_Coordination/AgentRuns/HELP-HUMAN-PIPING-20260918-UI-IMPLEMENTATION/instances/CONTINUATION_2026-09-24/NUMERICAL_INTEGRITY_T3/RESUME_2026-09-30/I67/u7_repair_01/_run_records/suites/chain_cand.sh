#!/bin/bash
# I67 U7 repair: candidate runs (lane cand9 = 8c84e7ae14 + the 9 changed files): vitest+tsc, Python retained suites, result_export.
WT=WT; S=$WT/scratch/i67_u6d; P=$S/lanes/cand9/projects/chirality-piping
until [ -f $S/r9/base.done ]; do sleep 5; done
$S/run_suite.sh $P/apps/desktop $S/r9/cand
$S/r9/run_py.sh py_cand $P tests/test_retained_precision_contract.py tests/test_retained_precision_carriers.py tests/test_retained_precision_schema.py
$S/r9/cargo_run.sh re_cand $P/core/reporting/result_export/Cargo.toml $WT/targets/i67-u7f/re-cand9
echo done > $S/r9/cand.done
