#!/bin/bash
export TMPDIR=WT/scratch/rv89_u4_g5/tmp GIT_OPTIONAL_LOCKS=0
P=WT/rv89/pristine/projects/chirality-piping/core/product_physics
C=WT/rv89/mut/projects/chirality-piping/core/product_physics
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
( cd $C && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 cargo test --locked --offline --lib --target-dir WT/targets/rv89/mut retained_memory > WT/scratch/rv89_u4_g5/logs/mut_baseline.log 2>&1 ); echo "baseline exit=$? $(date +%T)"; grep "^test result" WT/scratch/rv89_u4_g5/logs/mut_baseline.log
echo "i65 start $(date +%T)"
python3 WT/scratch/rv89_u4_g5/mutants_g5_i65_copy.py $P $C WT/targets/rv89/mut > WT/scratch/rv89_u4_g5/mutants_i65_rerun.jsonl 2> WT/scratch/rv89_u4_g5/logs/mutants_i65.err
echo "i65 exit=$? $(date +%T)"
python3 WT/scratch/rv89_u4_g5/mutants_rv89.py $P $C WT/targets/rv89/mut > WT/scratch/rv89_u4_g5/mutants_rv89.jsonl 2> WT/scratch/rv89_u4_g5/logs/mutants_rv89.err
echo "rv89 exit=$? $(date +%T)"
echo PHASE4 DONE
