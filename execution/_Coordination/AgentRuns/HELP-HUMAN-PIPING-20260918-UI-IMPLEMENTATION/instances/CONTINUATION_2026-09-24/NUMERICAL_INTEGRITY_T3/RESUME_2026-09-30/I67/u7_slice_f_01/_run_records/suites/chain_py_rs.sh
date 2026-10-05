#!/bin/bash
WT=WT; S=$WT/scratch/i67_u6d/r7; P=$WT/f2a-u7/projects/chirality-piping
$S/run_py.sh py_retained $P tests/test_retained_precision_contract.py tests/test_retained_precision_carriers.py tests/test_retained_precision_schema.py
$S/cargo_run.sh cand_re $P/core/reporting/result_export/Cargo.toml $WT/targets/i67-u7f/re-cand
echo CHAIN_DONE > $S/chain.done
