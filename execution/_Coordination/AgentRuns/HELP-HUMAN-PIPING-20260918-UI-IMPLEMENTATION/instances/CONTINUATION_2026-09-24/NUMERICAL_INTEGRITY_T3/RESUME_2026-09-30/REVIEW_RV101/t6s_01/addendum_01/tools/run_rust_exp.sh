#!/bin/bash
# RV101: build and run the {:e} oracle through the T3 lock.
cd WT/scratch/rv101_t6s_01/addendum/rv101_exp || exit 2
export CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 TMPDIR=WT/scratch/rv101_t6s_01/addendum/tmp
WT/tools/t3_cargo.sh run --locked --offline --release --target-dir WT/targets/rv101-exp -- WT/scratch/rv101_t6s_01/addendum/words/rv101_random.txt WT/scratch/rv101_t6s_01/addendum/words/rv101_ties17.txt WT/scratch/rv101_t6s_01/addendum/words/rv101_ties16.txt WT/scratch/rv101_t6s_01/addendum/words/rv101_small.txt WT/scratch/rv101_t6s_01/addendum/words/rv101_pow2.txt WT/scratch/rv101_t6s_01/addendum/words/i75_ties16.txt WT/scratch/rv101_t6s_01/addendum/words/vectors.txt > WT/scratch/rv101_t6s_01/addendum/logs/rust_exp.log 2>&1
echo "rc=$?" >> WT/scratch/rv101_t6s_01/addendum/logs/rust_exp.log
