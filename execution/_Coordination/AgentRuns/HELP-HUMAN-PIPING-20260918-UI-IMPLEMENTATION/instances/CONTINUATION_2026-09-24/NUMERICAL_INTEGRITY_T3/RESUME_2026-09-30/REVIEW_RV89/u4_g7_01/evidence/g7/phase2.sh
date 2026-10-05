#!/bin/bash
T=WT; G=$T/scratch/rv89_u4_g7_01; TG=$T/targets/rv89_g7
C=$T/rv89_g7/base/projects/chirality-piping/core; PM=$C/product_physics/Cargo.toml
python3 $G/instrument.py $C || exit 1
cp $T/scratch/rv89_u4_g5/g6r/rv89_g6_probe_tests.rs $C/product_physics/src/rv89_g6_probe_tests.rs
cp $G/rv89_g7_probe_tests.rs $C/product_physics/src/rv89_g7_probe_tests.rs
cp $G/zz_rv89_g7_sweep.rs $C/product_physics/tests/zz_rv89_g7_sweep.rs
RUST_TEST_THREADS=1 $G/run.sh g7_probe $PM $TG/base --lib -- retained_memory::rv89g --nocapture --test-threads=1
RV89_SWEEP_INPUTS=$T/scratch/rv89_u4_g5/inputs RV89_SWEEP_OUT=$G/sweep_g7_inst.tsv $G/run.sh g7_sweep_inst $PM $TG/base --test zz_rv89_g7_sweep -- --ignored --nocapture
echo PHASE2 DONE
