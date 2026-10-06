#!/bin/bash
# RV99 cargo wrapper: rc.sh <crate_dir> <target_subdir> <log_name> <cargo args...>
WT=<WT>
S=$WT/scratch/rv99_s_i1_01
dir=$1; tgt=$2; log=$3; shift 3
export CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR=$WT/targets/rv99/$tgt TMPDIR=$S/tmp
cd "$dir" || exit 2
"$WT/tools/t3_cargo.sh" "$@" > "$S/logs/$log.txt" 2>&1
rc=$?
echo "rc=$rc" >> "$S/logs/$log.txt"
grep -E '^test result:|Running |panicked|error(\[|:)' "$S/logs/$log.txt" > "$S/logs/$log.summary.txt"
echo "rc=$rc" >> "$S/logs/$log.summary.txt"
exit $rc
