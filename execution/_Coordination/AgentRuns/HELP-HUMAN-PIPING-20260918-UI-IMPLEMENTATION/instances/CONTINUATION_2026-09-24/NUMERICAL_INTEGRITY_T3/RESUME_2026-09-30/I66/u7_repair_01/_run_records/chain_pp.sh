#!/bin/bash
S=WT/scratch/i66_u7r; TG=WT/targets/i66-u7f
for L in base candx; do
  C=$S/$L/projects/chirality-piping/core
  $S/cargo_run.sh ${L}_pp_reg $C/product_physics/Cargo.toml $TG/pp-$L-reg-r -
  $S/cargo_run.sh ${L}_pp_stale $C/product_physics/Cargo.toml $TG/pp-$L-stale-r RUSTFLAGS=--cfg=i61_u3g2_stale
  $S/cargo_run.sh ${L}_runner $C/runner/headless/Cargo.toml $TG/runner-$L-r -
done
echo PP_CHAIN_DONE
