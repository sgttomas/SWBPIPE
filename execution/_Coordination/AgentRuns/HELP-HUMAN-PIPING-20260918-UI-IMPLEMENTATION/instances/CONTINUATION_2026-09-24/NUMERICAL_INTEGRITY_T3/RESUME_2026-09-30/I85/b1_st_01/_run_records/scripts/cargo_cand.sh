#!/bin/bash
# I85 B1-ST: one cargo job through the T3 wrapper, in the candidate (WT/b1) PP crate, registered build.
# Usage: cargo_cand.sh <log name> <crate dir relative to P/core> <cargo args...>
WT=WT; S=$WT/scratch/i85_b1_st
name=$1; crate=$2; shift 2
cd $WT/b1/projects/chirality-piping/core/$crate || exit 9
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$WT/targets/i85-b1-st $WT/tools/t3_cargo.sh "$@" > $S/logs/$name.log 2>&1
rc=$?; echo "rc=$rc" >> $S/logs/$name.log; echo "$name rc=$rc"; exit $rc
