#!/bin/bash
T=WT; U=$T/scratch/rv89_u4_g7_01/u7; TG=$T/targets/rv89_u7
C=$T/rv89_u7/base/projects/chirality-piping/core; PM=$C/product_physics/Cargo.toml; RP=$C/reporting/result_export/src/retained_precision.rs
$U/run.sh u7_law $PM $TG/base --lib -- retained_memory --nocapture --test-threads=1
for w in witness_w1_milestone witness_w2_cap_maximal witness_w2_deep_milestone_publishes witness_w2b_cap_maximal_solvable witness_w3_exact_selected witness_w4_preparation_refusal witness_w6_force_scaled witness_w7_fault_fallbacks witness_headroom_w1_at_one_mebibyte; do
  RUST_TEST_THREADS=1 $U/run.sh u7_$w $PM $TG/base --lib -- retained_memory::witness_tests::$w --ignored --exact --nocapture --test-threads=1
done
RUST_TEST_THREADS=1 $U/run.sh u7_challenge $PM $TG/base --test retained_memory_challenge -- --nocapture --test-threads=1
$U/run.sh u7_pp $PM $TG/base
$U/run.sh u7_runner $C/runner/headless/Cargo.toml $TG/base-runner
cp $T/scratch/rv89_u4_g5/zz_rv89_sweep.rs $C/product_physics/tests/zz_rv89_sweep.rs
RV89_SWEEP_INPUTS=$T/scratch/rv89_u4_g5/inputs RV89_SWEEP_OUT=$U/sweep_u7.tsv $U/run.sh u7_sweep $PM $TG/base --test zz_rv89_sweep -- --ignored --nocapture
rm -f $C/product_physics/tests/zz_rv89_sweep.rs
# the eligibility probe: as committed (flag true), then with only the flag false
cp $U/rv89_u7_alloc_probe.rs $C/product_physics/src/rv89_u7_alloc_probe.rs
printf '\n#[cfg(test)]\n#[path = "rv89_u7_alloc_probe.rs"]\nmod rv89_u7_alloc_probe;\n' >> $C/product_physics/src/lib.rs
grep -c "const IMPLEMENTATION_COMPLETE: bool = true;" $RP
RUST_TEST_THREADS=1 $U/run.sh u7_probe_true $PM $TG/base --lib -- rv89u7_eligibility_switch_allocations --nocapture --test-threads=1
sed -i '' 's/^const IMPLEMENTATION_COMPLETE: bool = true;$/const IMPLEMENTATION_COMPLETE: bool = false;/' $RP; grep -c "const IMPLEMENTATION_COMPLETE: bool = false;" $RP
RUST_TEST_THREADS=1 $U/run.sh u7_probe_false $PM $TG/base --lib -- rv89u7_eligibility_switch_allocations --nocapture --test-threads=1
# restore
rm -f $C/product_physics/src/rv89_u7_alloc_probe.rs
( cd $T/numerics && GIT_OPTIONAL_LOCKS=0 git archive cfda60403f projects/chirality-piping/core/product_physics/src/lib.rs projects/chirality-piping/core/reporting/result_export/src/retained_precision.rs | tar -x -C $T/rv89_u7/base )
echo PHASEU DONE
