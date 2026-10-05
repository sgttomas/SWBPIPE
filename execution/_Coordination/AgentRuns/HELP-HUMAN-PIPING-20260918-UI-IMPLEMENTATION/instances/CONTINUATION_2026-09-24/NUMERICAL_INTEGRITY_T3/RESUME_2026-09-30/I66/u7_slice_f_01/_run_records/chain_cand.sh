#!/bin/bash
S=WT/scratch/i66_u7f; W=WT/f2a-u7/projects/chirality-piping/core; TG=WT/targets/i66-u7f
$S/cargo_run.sh cand_re $W/reporting/result_export/Cargo.toml $TG/re-cand -
$S/cargo_run.sh cand_pp_reg $W/product_physics/Cargo.toml $TG/pp-cand-reg -
$S/cargo_run.sh cand_pp_stale $W/product_physics/Cargo.toml $TG/pp-cand-stale RUSTFLAGS=--cfg=i61_u3g2_stale
$S/cargo_run.sh cand_runner $W/runner/headless/Cargo.toml $TG/runner-cand -
mkdir -p $S/sweep
for L in base cand; do
  $S/cargo_run.sh sweep_${L}_reg $S/$L/projects/chirality-piping/core/product_physics/Cargo.toml $TG/sweep-$L-reg "I61_U3G2_SWEEP=$S/sweep/sweep_${L}_reg.tsv" --lib zz_i61_u3g2_sweep -- --ignored --nocapture
  $S/cargo_run.sh sweep_${L}_stale $S/$L/projects/chirality-piping/core/product_physics/Cargo.toml $TG/sweep-$L-stale "RUSTFLAGS=--cfg=i61_u3g2_stale I61_U3G2_SWEEP=$S/sweep/sweep_${L}_stale.tsv" --lib zz_i61_u3g2_sweep -- --ignored --nocapture
done
echo CHAIN_CAND_DONE
