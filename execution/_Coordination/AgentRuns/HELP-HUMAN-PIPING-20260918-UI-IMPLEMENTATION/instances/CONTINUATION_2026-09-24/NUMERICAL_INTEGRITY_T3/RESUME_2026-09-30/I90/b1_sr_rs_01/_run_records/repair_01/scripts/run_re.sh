#!/bin/bash
# I90: one ad-hoc RE cargo test job in WT/b1-r through the T3 lock. Usage: run_re.sh <log-name> <cargo test args...>
set -u
WT=WT
S=$WT/scratch/i90_b1_sr_rs; name=$1; shift
cd $WT/b1-r/projects/chirality-piping/core/reporting/result_export && unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$WT/targets/i90-b1-sr-rs $WT/tools/t3_cargo.sh test --locked --offline "$@" > $S/logs/$name.log 2>&1
echo "$name rc=$?"
