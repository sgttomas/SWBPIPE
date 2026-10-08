#!/bin/bash
# RV108: run the Rust probe harness over a probe file (release, own target), through the T3 lock.
source S/env.sh
IN=$1; OUT=$2; LOG=$3
cd $WT/rv108/pcand/$P/core/reporting/result_export && env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=1 TMPDIR=$S/tmp CARGO_TARGET_DIR=$WT/targets/rv108-b6/rx-probe RV108_IN=$IN RV108_OUT=$OUT $WT/tools/t3_cargo.sh test --locked --offline --release --test zz_rv108_probe > $LOG 2>&1
echo "rc=$?" >> $LOG
