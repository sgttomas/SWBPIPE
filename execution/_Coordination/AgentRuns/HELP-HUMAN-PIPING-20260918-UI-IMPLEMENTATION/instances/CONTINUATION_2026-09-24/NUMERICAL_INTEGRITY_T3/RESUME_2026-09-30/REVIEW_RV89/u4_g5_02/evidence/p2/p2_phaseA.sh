#!/bin/bash
T=WT
S=$T/scratch/rv89_u4_g5; R=$T/rv89; TG=$T/targets/rv89
PB=$R/base2/projects/chirality-piping/core; PC=$R/cand2/projects/chirality-piping/core
export RV89_SWEEP_INPUTS=$S/inputs
RV89_SWEEP_OUT=$S/p2_sweep_base.tsv $S/run.sh p2_base_sweep $PB/product_physics/Cargo.toml $TG/base --test zz_rv89_sweep -- --ignored --nocapture
RV89_SWEEP_OUT=$S/p2_sweep_cand.tsv $S/run.sh p2_cand_sweep $PC/product_physics/Cargo.toml $TG/cand --test zz_rv89_sweep -- --ignored --nocapture
$S/run.sh p2_cand_pp $PC/product_physics/Cargo.toml $TG/cand
$S/run.sh p2_base_fk $PB/solver/frame_kernel/Cargo.toml $TG/fk-base --lib
$S/run.sh p2_cand_fk $PC/solver/frame_kernel/Cargo.toml $TG/fk-cand --lib
$S/run.sh p2_base_sr $PB/loads/stress_recovery/Cargo.toml $TG/sr-base
$S/run.sh p2_cand_sr $PC/loads/stress_recovery/Cargo.toml $TG/sr-cand
$S/run.sh p2_cand_runner $PC/runner/headless/Cargo.toml $TG/cand-runner
echo P2A DONE
