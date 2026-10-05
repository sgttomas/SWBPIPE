#!/bin/bash
T=WT
S=$T/scratch/rv89_u4_g5; TG=$T/targets/rv89; PPC=$T/rv89/reg3/projects/chirality-piping/core/product_physics
until grep -q "G6B DONE" $S/logs/g6_phaseB.out; do sleep 5; done
$S/run.sh g6_rel_identity $PPC/Cargo.toml $TG/reg-release --release --lib -- rv89g6_identity the_registered_profile admit_grants --nocapture --test-threads=1
for w in witness_w1_milestone witness_w2_cap_maximal witness_w2_deep_milestone_publishes witness_w2b_cap_maximal_solvable witness_w3_exact_selected witness_w4_preparation_refusal witness_w6_force_scaled witness_w7_fault_fallbacks witness_headroom_w1_at_one_mebibyte; do
  $S/run.sh g6_rel_$w $PPC/Cargo.toml $TG/reg-release --release --lib retained_memory::witness_tests::$w -- --ignored --exact --test-threads=1 --nocapture
done
echo G6C DONE
