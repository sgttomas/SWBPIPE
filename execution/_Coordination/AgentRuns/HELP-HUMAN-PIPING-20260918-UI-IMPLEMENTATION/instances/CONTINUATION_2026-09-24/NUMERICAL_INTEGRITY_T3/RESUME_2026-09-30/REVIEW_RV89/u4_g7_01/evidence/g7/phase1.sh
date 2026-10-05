#!/bin/bash
T=WT; G=$T/scratch/rv89_u4_g7_01; TG=$T/targets/rv89_g7
C=$T/rv89_g7/base/projects/chirality-piping/core; PM=$C/product_physics/Cargo.toml
# 1. law tests, printing the record (identity, inputs, layouts, profile)
$G/run.sh g7_law $PM $TG/base --lib -- retained_memory --nocapture --test-threads=1
# 2. witnesses, one process each
for w in witness_w1_milestone witness_w2_cap_maximal witness_w2_deep_milestone_publishes witness_w2b_cap_maximal_solvable witness_w3_exact_selected witness_w4_preparation_refusal witness_w6_force_scaled witness_w7_fault_fallbacks witness_headroom_w1_at_one_mebibyte; do
  RUST_TEST_THREADS=1 $G/run.sh g7_$w $PM $TG/base --lib -- retained_memory::witness_tests::$w --ignored --exact --nocapture --test-threads=1
done
# 3. the challenge
RUST_TEST_THREADS=1 $G/run.sh g7_challenge $PM $TG/base --test retained_memory_challenge -- --nocapture --test-threads=1
# 4. suites
$G/run.sh g7_pp $PM $TG/base
$G/run.sh g7_runner $C/runner/headless/Cargo.toml $TG/base-runner
# 5. my sweep (test file added after the suites)
cp $T/scratch/rv89_u4_g5/zz_rv89_sweep.rs $C/product_physics/tests/zz_rv89_sweep.rs
RV89_SWEEP_INPUTS=$T/scratch/rv89_u4_g5/inputs RV89_SWEEP_OUT=$G/sweep_g7.tsv $G/run.sh g7_sweep $PM $TG/base --test zz_rv89_sweep -- --ignored --nocapture
echo PHASE1 DONE
