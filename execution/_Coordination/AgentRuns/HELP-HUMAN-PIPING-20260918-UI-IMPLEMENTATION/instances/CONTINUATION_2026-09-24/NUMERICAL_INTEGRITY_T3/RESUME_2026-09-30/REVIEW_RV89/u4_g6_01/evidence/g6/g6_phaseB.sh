#!/bin/bash
T=WT; S=WT/scratch/rv89_u4_g5; TG=$T/targets/rv89; PPC=WT/rv89/reg3/projects/chirality-piping/core/product_physics
export RV89_SWEEP_INPUTS=$S/inputs
$S/run.sh g6_probe $PPC/Cargo.toml $TG/reg --lib rv89g6 -- --nocapture --test-threads=1
for w in witness_w1_milestone witness_w2_cap_maximal witness_w2_deep_milestone_publishes witness_w2b_cap_maximal_solvable witness_w3_exact_selected witness_w4_preparation_refusal witness_w6_force_scaled witness_w7_fault_fallbacks witness_headroom_w1_at_one_mebibyte; do
  $S/run.sh g6_$w $PPC/Cargo.toml $TG/reg --lib retained_memory::witness_tests::$w -- --ignored --exact --test-threads=1 --nocapture
done
RUSTFLAGS="--cfg rv89_stale" $S/run.sh g6_stale_flags $PPC/Cargo.toml $TG/reg-flags --lib -- rv89g6_identity the_registered_profile admit_grants registered_g_c --nocapture --test-threads=1
CARGO_PROFILE_DEV_OPT_LEVEL=1 $S/run.sh g6_stale_opt1 $PPC/Cargo.toml $TG/reg-opt1 --lib -- rv89g6_identity the_registered_profile admit_grants registered_g_c --nocapture --test-threads=1
echo "mutants i65 start $(date +%T)"
( cd WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I65/u4_g6_01/_run_records && pgrep -f memguard.sh >/dev/null && env TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 python3 mutants_g6.py $T/rv89/cand3/projects/chirality-piping/core $T/rv89/mutU/projects/chirality-piping/core $TG/mutU G6,RV,P2 > $S/g6/mutants_i65_g6_rerun.jsonl 2> $S/logs/g6_mut_i65.err )
echo "mutants i65 exit=$? $(date +%T)"
( cd $T && pgrep -f memguard.sh >/dev/null && env TMPDIR=$S/tmp python3 $S/g6/mutants_rv89_g6.py $T/rv89/pristR/projects/chirality-piping/core $T/rv89/mutR/projects/chirality-piping/core $TG/mutR > $S/g6/mutants_rv89_g6.jsonl 2> $S/logs/g6_mut_rv89.err )
echo "mutants rv89 exit=$? $(date +%T)"
echo G6B DONE
