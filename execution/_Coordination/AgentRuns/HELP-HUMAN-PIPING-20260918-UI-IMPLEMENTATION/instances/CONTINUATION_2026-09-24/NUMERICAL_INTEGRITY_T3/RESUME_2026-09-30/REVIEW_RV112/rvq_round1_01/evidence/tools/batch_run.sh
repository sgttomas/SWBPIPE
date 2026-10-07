#!/bin/bash
# RV112: run under ONE host-lock acquisition: each named mutant over PP's whole lib suite, one after another.
# Usage (via lockf): batch_run.sh <bin> <mutant ids...>
WT=WT
S=$WT/scratch/rv112_rvq_01
PP=$WT/rv112/mut/projects/chirality-piping/core/product_physics
BIN=$1; shift
cd $PP || exit 2
for m in "$@"; do
  env RV112_MUT="$m" CARGO_MANIFEST_DIR=$PP TMPDIR=$S/tmp RUST_TEST_THREADS=2 RUST_BACKTRACE=0 "$BIN" --skip zz_rv112 > $S/mutants/$m.log 2>&1
  echo $? > $S/mutants/$m.rc
done
