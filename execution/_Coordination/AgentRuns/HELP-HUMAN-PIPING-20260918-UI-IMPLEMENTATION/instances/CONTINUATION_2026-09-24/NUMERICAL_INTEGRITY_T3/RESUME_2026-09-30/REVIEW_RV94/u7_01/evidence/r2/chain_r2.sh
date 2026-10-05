#!/bin/bash
S=WT/scratch/rv94_u7_01; WT=WT; TG=WT/targets/rv94; C=$WT/rv94/cand2/projects/chirality-piping/core
for L in prev cand2; do $S/cargo_run.sh r2_dump_rs_$L $WT/rv94/$L/projects/chirality-piping/core/reporting/result_export/Cargo.toml $TG/re-$L "RV94_INPUTS=$S/r2/inputs.jsonl RV94_OUT=$S/r2/rs_$L.jsonl" --test zz_rv94_dump -- --ignored --nocapture; done
$S/cargo_run.sh r2_re_cand2 $C/reporting/result_export/Cargo.toml $TG/re-cand2 -
$S/cargo_run.sh r2_sweep_reg $C/product_physics/Cargo.toml $TG/pp-cand2-reg "I61_U3G2_SWEEP=$S/r2/sweep_cand2_reg.tsv" --lib zz_i61_u3g2_sweep -- --ignored --nocapture
$S/cargo_run.sh r2_sweep_stale $C/product_physics/Cargo.toml $TG/pp-cand2-stale "RUSTFLAGS=--cfg=i61_u3g2_stale I61_U3G2_SWEEP=$S/r2/sweep_cand2_stale.tsv" --lib zz_i61_u3g2_sweep -- --ignored --nocapture
$S/cargo_run.sh r2_pp_reg_full $C/product_physics/Cargo.toml $TG/pp-cand2-reg "I61_U3G2_OUT=$S/r2/live"
$S/cargo_run.sh r2_pp_stale_full $C/product_physics/Cargo.toml $TG/pp-cand2-stale "RUSTFLAGS=--cfg=i61_u3g2_stale"
$S/cargo_run.sh r2_runner $C/runner/headless/Cargo.toml $TG/runner-cand2 -
echo CHAIN_R2_DONE
