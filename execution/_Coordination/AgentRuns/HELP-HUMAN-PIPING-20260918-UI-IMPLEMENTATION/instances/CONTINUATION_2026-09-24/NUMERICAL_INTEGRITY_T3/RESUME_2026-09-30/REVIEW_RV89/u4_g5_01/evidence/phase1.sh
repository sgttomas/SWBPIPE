#!/bin/bash
T=WT
S=$T/scratch/rv89_u4_g5; R=$T/rv89; TG=$T/targets/rv89
PPB=$R/base/projects/chirality-piping/core/product_physics/Cargo.toml
PPC=$R/cand/projects/chirality-piping/core/product_physics/Cargo.toml
export RV89_SWEEP_INPUTS=$S/inputs
RV89_SWEEP_OUT=$S/sweep_base.tsv $S/run.sh base_sweep $PPB $TG/base --test zz_rv89_sweep -- --ignored --nocapture
$S/run.sh base_pp $PPB $TG/base
RV89_SWEEP_OUT=$S/sweep_cand.tsv $S/run.sh cand_sweep $PPC $TG/cand --test zz_rv89_sweep -- --ignored --nocapture
$S/run.sh cand_pp $PPC $TG/cand
echo PHASE1 DONE
