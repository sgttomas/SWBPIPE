#!/bin/bash
# RV112: run one cargo job through the T3 host lock (WT/tools/t3_cargo.sh).
# Usage: runjob.sh <name> <crate dir> <target name> <stale 0|1> <cargo args...>
# Logs to S/logs/<name>.log; rc to S/logs/<name>.rc.
WT=WT
S=$WT/scratch/rv112_rvq_01
name=$1; dir=$2; target=$3; stale=$4; shift 4
mkdir -p "$S/logs" "$S/tmp"
cd "$dir" || exit 2
unset RUSTC_WRAPPER
if [ "$stale" = 1 ]; then
  export RUSTFLAGS=--cfg=rv112_stale
  unset CARGO_ENCODED_RUSTFLAGS
else
  unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
fi
export TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$WT/targets/$target
{
  echo "RV112_JOB name=$name dir=$dir target=$target stale=$stale args=$* start=$(date -u '+%FT%TZ')"
  "$WT/tools/t3_cargo.sh" "$@" 2>&1
  rc=$?
  echo "RV112_JOB_END name=$name rc=$rc end=$(date -u '+%FT%TZ')"
} > "$S/logs/$name.log" 2>&1
echo $rc > "$S/logs/$name.rc"
