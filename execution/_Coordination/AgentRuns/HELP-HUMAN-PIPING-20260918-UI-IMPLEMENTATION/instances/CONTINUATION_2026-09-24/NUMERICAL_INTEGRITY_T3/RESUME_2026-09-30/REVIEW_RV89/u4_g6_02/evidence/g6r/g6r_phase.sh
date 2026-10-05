#!/bin/bash
T=WT; S=WT/scratch/rv89_u4_g5; TG=$T/targets/rv89
PR=$T/rv89/reg4/projects/chirality-piping/core; PC=$T/rv89/cand4/projects/chirality-piping/core
export RV89_SWEEP_INPUTS=$S/inputs
RV89_SWEEP_OUT=$S/g6r/sweep_reg.tsv $S/run.sh g6r_reg_sweep $PR/product_physics/Cargo.toml $TG/reg --test zz_rv89_sweep -- --ignored --nocapture
$S/run.sh g6r_reg_pp $PR/product_physics/Cargo.toml $TG/reg
$S/run.sh g6r_reg_runner $PR/runner/headless/Cargo.toml $TG/reg-runner
RV89_SWEEP_OUT=$S/g6r/sweep_cand.tsv $S/run.sh g6r_cand_sweep $PC/product_physics/Cargo.toml $TG/cand --test zz_rv89_sweep -- --ignored --nocapture
$S/run.sh g6r_cand_pp $PC/product_physics/Cargo.toml $TG/cand
$S/run.sh g6r_cand_runner $PC/runner/headless/Cargo.toml $TG/cand-runner
# instrument the registered copy and mount RV89's probe
python3 - "$PR/product_physics/src/lib.rs" <<'PY'
import sys
p=sys.argv[1]; s=open(p).read()
anchor="        BasisStiffness::RangeDeferred(error) => Err(OrdinaryFailure::Formation(error.clone())),\n    };\n"
assert s.count(anchor)==1
s=s.replace(anchor, anchor+"    #[cfg(test)]\n    RV89_ATTEMPTS.with(|c| c.set(c.get() + 1));\n")
s+="\n#[cfg(test)]\nthread_local! { pub(crate) static RV89_ATTEMPTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }\n"
open(p,'w').write(s)
PY
cp $S/g6r/rv89_g6_probe_tests.rs $PR/product_physics/src/rv89_g6_probe_tests.rs
printf '\n#[cfg(test)]\n#[path = "rv89_g6_probe_tests.rs"]\nmod rv89g6;\n' >> $PR/product_physics/src/retained_memory.rs
$S/run.sh g6r_probe $PR/product_physics/Cargo.toml $TG/reg --lib rv89g6 -- --nocapture --test-threads=1
RUSTFLAGS="--cfg rv89_stale" $S/run.sh g6r_stale_flags $PR/product_physics/Cargo.toml $TG/reg-flags --lib -- rv89g6_identity rv89g6r_required the_registered_profile admit_grants admission_bound --nocapture --test-threads=1
$S/run.sh g6r_witness_deep $PR/product_physics/Cargo.toml $TG/reg --lib retained_memory::witness_tests::witness_w2_deep_milestone_publishes -- --ignored --exact --test-threads=1 --nocapture
echo "mutants start $(date +%T)"
( cd WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I65/u4_g6_01/_run_records_g6r && pgrep -f memguard.sh >/dev/null && env TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 python3 mutants_g6.py $T/rv89/cand4/projects/chirality-piping/core $T/rv89/mutU4/projects/chirality-piping/core $TG/mutU G6R,G6 > $S/g6r/mutants_i65_g6r_rerun.jsonl 2> $S/logs/g6r_mut_i65.err )
( cd $T && pgrep -f memguard.sh >/dev/null && env TMPDIR=$S/tmp python3 $S/g6r/mutants_rv89_g6r.py $T/rv89/pristR4/projects/chirality-piping/core $T/rv89/mutR4/projects/chirality-piping/core $TG/mutR > $S/g6r/mutants_rv89_g6r.jsonl 2> $S/logs/g6r_mut_rv89.err )
echo "mutants done $(date +%T)"
echo G6R DONE
