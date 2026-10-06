#!/bin/bash
# usage: run_re_suite.sh <cand|base> <logname> [extra cargo test args...]
WT=WT
S=$WT/scratch/rv101_t6s_01
c=$1; log=$2; shift 2
export CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$WT/targets/rv101/$c TMPDIR=$S/tmp
cd $WT/rv101/$c/projects/chirality-piping/core/reporting/result_export || exit 2
$WT/tools/t3_cargo.sh test --locked --offline --no-fail-fast "$@" > $S/logs/$log 2>&1
echo "rc=$?" >> $S/logs/$log
