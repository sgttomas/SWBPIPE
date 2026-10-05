#!/bin/bash
S=WT/scratch/rv94_u7_01; WT=WT; TG=WT/targets/rv94
for L in cand base; do
  M=$WT/rv94/$L/projects/chirality-piping/core/product_physics/Cargo.toml
  $S/cargo_run.sh sweep_${L}_reg $M $TG/pp-$L-reg "I61_U3G2_SWEEP=$S/sweep/sweep_${L}_reg.tsv" --lib zz_i61_u3g2_sweep -- --ignored --nocapture
  $S/cargo_run.sh sweep_${L}_stale $M $TG/pp-$L-stale "RUSTFLAGS=--cfg=i61_u3g2_stale I61_U3G2_SWEEP=$S/sweep/sweep_${L}_stale.tsv" --lib zz_i61_u3g2_sweep -- --ignored --nocapture
done
# the milestone and wire tests, registered, candidate
$S/cargo_run.sh pp_cand_reg_retained $WT/rv94/cand/projects/chirality-piping/core/product_physics/Cargo.toml $TG/pp-cand-reg "I61_U3G2_OUT=$S/live" --lib retained
echo CHAIN_PP_DONE
