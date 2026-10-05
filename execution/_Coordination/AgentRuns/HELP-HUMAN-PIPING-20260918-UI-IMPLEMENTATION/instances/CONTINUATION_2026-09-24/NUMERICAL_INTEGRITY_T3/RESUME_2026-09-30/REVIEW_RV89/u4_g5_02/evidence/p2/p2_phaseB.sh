#!/bin/bash
T=WT; S=WT/scratch/rv89_u4_g5; TG=$T/targets/rv89
PPC=$T/rv89/cand2/projects/chirality-piping/core/product_physics
until grep -q "P2A DONE" $S/logs/p2_phaseA.out; do sleep 5; done
for w in witness_w1_milestone witness_w2_cap_maximal witness_w2b_cap_maximal_solvable witness_w3_exact_selected witness_w4_preparation_refusal witness_w6_force_scaled witness_w7_fault_fallbacks witness_headroom_w1_at_one_mebibyte; do
  RUST_TEST_THREADS=1 $S/run.sh p2_$w $PPC/Cargo.toml $TG/cand --lib retained_memory::witness_tests::$w -- --ignored --exact --test-threads=1 --nocapture
done
$S/run.sh p2_challenge $PPC/Cargo.toml $TG/cand --test retained_memory_challenge -- --nocapture --test-threads=1
cp $S/p2/rv89_p2_probe_tests.rs $PPC/src/rv89_p2_probe_tests.rs
grep -q rv89p2 $PPC/src/retained_memory.rs || printf '\n#[cfg(test)]\n#[path = "rv89_p2_probe_tests.rs"]\nmod rv89p2;\n' >> $PPC/src/retained_memory.rs
$S/run.sh p2_probe $PPC/Cargo.toml $TG/cand --lib rv89p2 -- --nocapture --test-threads=1
echo "mutants i65 start $(date +%T)"
( cd WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I65/u4_g5_01/part2/_run_records && pgrep -f memguard.sh >/dev/null && env TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 python3 mutants_g5_part2.py $T/rv89/pristine2/projects/chirality-piping/core $T/rv89/mut2/projects/chirality-piping/core $TG/mut2 > $S/p2/mutants_i65_p2_rerun.jsonl 2> $S/logs/p2_mut_i65.err )
echo "mutants i65 exit=$? $(date +%T)"
( cd $T && pgrep -f memguard.sh >/dev/null && env TMPDIR=$S/tmp python3 $S/p2/mutants_rv89_p2.py $T/rv89/pristine2/projects/chirality-piping/core $T/rv89/mut2/projects/chirality-piping/core $TG/mut2 > $S/p2/mutants_rv89_p2.jsonl 2> $S/logs/p2_mut_rv89.err )
echo "mutants rv89 exit=$? $(date +%T)"
echo P2B DONE
