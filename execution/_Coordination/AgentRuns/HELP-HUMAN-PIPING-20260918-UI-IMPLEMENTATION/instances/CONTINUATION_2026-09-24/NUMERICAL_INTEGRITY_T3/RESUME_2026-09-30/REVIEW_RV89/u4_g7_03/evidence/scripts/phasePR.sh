#!/bin/bash
T=WT; P=$T/scratch/rv89_u4_g7_01/pr; TG=$T/targets/rv89_pr
C=$T/rv89_pr/base/projects/chirality-piping/core; PM=$C/product_physics/Cargo.toml
$P/run.sh pr_law $PM $TG/base --lib -- retained_memory --nocapture --test-threads=1
for w in witness_w1_milestone witness_w2_cap_maximal witness_w2_deep_milestone_publishes witness_w2b_cap_maximal_solvable witness_w3_exact_selected witness_w4_preparation_refusal witness_w6_force_scaled witness_w7_fault_fallbacks witness_headroom_w1_at_one_mebibyte; do
  RUST_TEST_THREADS=1 $P/run.sh pr_$w $PM $TG/base --lib -- retained_memory::witness_tests::$w --ignored --exact --nocapture --test-threads=1
done
RUST_TEST_THREADS=1 $P/run.sh pr_challenge $PM $TG/base --test retained_memory_challenge -- --nocapture --test-threads=1
$P/run.sh pr_pp $PM $TG/base
$P/run.sh pr_runner $C/runner/headless/Cargo.toml $TG/base-runner
# instrumented: counters on validate_in (loop as committed), the probe and the counting sweep
python3 $P/instrument_pr.py $C new || exit 1
cp $P/rv89_pr_probe.rs $C/product_physics/src/rv89_pr_probe.rs
printf '\n#[cfg(test)]\n#[path = "rv89_pr_probe.rs"]\nmod rv89_pr_probe;\n' >> $C/product_physics/src/lib.rs
cp $P/zz_rv89_pr_sweep.rs $C/product_physics/tests/zz_rv89_pr_sweep.rs
RUST_TEST_THREADS=1 $P/run.sh pr_probe_new $PM $TG/base --lib -- rv89pr_validate_in_reach_and_allocations --nocapture --test-threads=1
RV89_SWEEP_INPUTS=$T/scratch/rv89_u4_g5/inputs RV89_SWEEP_OUT=$P/sweep_pr.tsv $P/run.sh pr_sweep $PM $TG/base --test zz_rv89_pr_sweep -- --ignored --nocapture
# the loop header set back to PR1080's parent, counters kept
( cd $T/numerics && GIT_OPTIONAL_LOCKS=0 git archive 6b9bb19a5f projects/chirality-piping/core/reporting/result_export/src/source_blocks.rs | tar -x -C $T/rv89_pr/base )
python3 $P/instrument_pr.py $C old || exit 1
RUST_TEST_THREADS=1 $P/run.sh pr_probe_old $PM $TG/base --lib -- rv89pr_validate_in_reach_and_allocations --nocapture --test-threads=1
# restore
rm -f $C/product_physics/src/rv89_pr_probe.rs $C/product_physics/tests/zz_rv89_pr_sweep.rs
( cd $T/numerics && GIT_OPTIONAL_LOCKS=0 git archive 6b9bb19a5f projects/chirality-piping/core/reporting/result_export/src/source_blocks.rs projects/chirality-piping/core/product_physics/src/lib.rs | tar -x -C $T/rv89_pr/base )
echo PHASEPR DONE
