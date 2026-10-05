#!/bin/bash
# I67 U7 repair: after the one-line extension of I66's D-U7-4 side tests, rerun the candidate's
# Python retained suites and result_export (lane cand9, resynced), then the Python/Rust mutants (one cargo job at a time).
WT=WT; S=$WT/scratch/i67_u6d; P=$S/lanes/cand9/projects/chirality-piping
$S/r9/run_py.sh py_cand2 $P tests/test_retained_precision_contract.py tests/test_retained_precision_carriers.py tests/test_retained_precision_schema.py
$S/r9/cargo_run.sh re_cand2 $P/core/reporting/result_export/Cargo.toml $WT/targets/i67-u7f/re-cand9
echo done > $S/r9/cand2.done
cd $S/mut && python3 pyrs_mutants_r9.py pyrs_mutants_r9.json > pyrs_mutants_r9.log 2>&1
echo done > $S/r9/mut.done
