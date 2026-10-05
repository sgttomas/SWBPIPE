#!/bin/bash
T=WT
S=$T/scratch/rv89_u4_g5; TG=$T/targets/rv89
PR=$T/rv89/reg3/projects/chirality-piping/core; PC=$T/rv89/cand3/projects/chirality-piping/core
export RV89_SWEEP_INPUTS=$S/inputs
RV89_SWEEP_OUT=$S/g6/sweep_reg.tsv $S/run.sh g6_reg_sweep $PR/product_physics/Cargo.toml $TG/reg --test zz_rv89_sweep -- --ignored --nocapture
$S/run.sh g6_reg_pp $PR/product_physics/Cargo.toml $TG/reg
RV89_SWEEP_OUT=$S/g6/sweep_cand.tsv $S/run.sh g6_cand_sweep $PC/product_physics/Cargo.toml $TG/cand --test zz_rv89_sweep -- --ignored --nocapture
$S/run.sh g6_reg_runner $PR/runner/headless/Cargo.toml $TG/reg-runner
echo G6A DONE
