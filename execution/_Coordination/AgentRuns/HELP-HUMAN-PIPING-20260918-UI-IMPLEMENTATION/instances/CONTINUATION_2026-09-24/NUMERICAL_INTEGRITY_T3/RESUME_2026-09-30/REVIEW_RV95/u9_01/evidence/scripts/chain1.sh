#!/bin/bash
# RV95 chain 1: PP registered and Stale (full suite and the 324-output sweep), result_export, runner. One cargo job at a time.
T3=WT
S=$T3/scratch/rv95_u9_01; TG=$T3/targets; C=$T3/rv95/projects/chirality-piping/core; R=$S/scripts/cargo_run.sh
$R pp_reg_full $C/product_physics/Cargo.toml $TG/rv95/pp "I61_U3G2_OUT=$S/live"
$R pp_reg_sweep $C/product_physics/Cargo.toml $TG/rv95/pp "I61_U3G2_SWEEP=$S/sweep/sweep_reg.tsv" --lib zz_i61_u3g2_sweep -- --ignored --nocapture
$R pp_stale_full $C/product_physics/Cargo.toml $TG/rv95-stale/pp "RUSTFLAGS=--cfg=rv95_stale"
$R pp_stale_sweep $C/product_physics/Cargo.toml $TG/rv95-stale/pp "RUSTFLAGS=--cfg=rv95_stale I61_U3G2_SWEEP=$S/sweep/sweep_stale.tsv" --lib zz_i61_u3g2_sweep -- --ignored --nocapture
$R re_full $C/reporting/result_export/Cargo.toml $TG/rv95/re -
$R runner_reg $C/runner/headless/Cargo.toml $TG/rv95/runner -
$R re_stale $C/reporting/result_export/Cargo.toml $TG/rv95-stale/re "RUSTFLAGS=--cfg=rv95_stale"
$R runner_stale $C/runner/headless/Cargo.toml $TG/rv95-stale/runner "RUSTFLAGS=--cfg=rv95_stale"
echo CHAIN1_DONE
