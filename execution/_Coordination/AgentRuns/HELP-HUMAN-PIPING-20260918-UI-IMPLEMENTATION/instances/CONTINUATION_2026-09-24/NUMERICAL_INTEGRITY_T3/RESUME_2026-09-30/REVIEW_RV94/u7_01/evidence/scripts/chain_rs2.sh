#!/bin/bash
# Waits for RV94's own Rust mutants, then: I66 rs-phase mutants, I67 pyrs scope mutants, PP full suites, runner.
S=WT/scratch/rv94_u7_01; WT=WT; I=WT/scratch/rv94_u7_01/impl; TG=WT/targets/rv94
while [ ! -f $S/mutants_rv94_rs.json ]; do sleep 20; done
cd $I && python3 mutants_i66.py i66_rs rs > $I/logs/impl_i66_rs.log 2>&1
while [ ! -f $S/mutants_rv94_py.json ]; do sleep 20; done
cd $I && python3 mutants_i66.py i66_py py > $I/logs/impl_i66_py.log 2>&1
cd $I && python3 pyrs_mutants.py $I/impl_pyrs.json > $I/logs/impl_pyrs.log 2>&1
for L in cand base; do
  M=$WT/rv94/$L/projects/chirality-piping/core/product_physics/Cargo.toml
  $S/cargo_run.sh pp_${L}_reg_full $M $TG/pp-$L-reg -
done
$S/cargo_run.sh pp_cand_stale_full $WT/rv94/cand/projects/chirality-piping/core/product_physics/Cargo.toml $TG/pp-cand-stale "RUSTFLAGS=--cfg=i61_u3g2_stale"
for L in cand base; do
  $S/cargo_run.sh runner_$L $WT/rv94/$L/projects/chirality-piping/core/runner/headless/Cargo.toml $TG/runner-$L -
done
echo CHAIN_RS2_DONE
