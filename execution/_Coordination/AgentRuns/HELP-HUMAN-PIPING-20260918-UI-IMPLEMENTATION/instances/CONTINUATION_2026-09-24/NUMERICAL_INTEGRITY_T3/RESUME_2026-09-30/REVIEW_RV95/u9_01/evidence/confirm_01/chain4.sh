#!/bin/bash
# RV95 confirmation (F = 20dd3d929d): PP registered full suite, then the C2 mutant rerun is done separately. One cargo job.
T3=WT
S=$T3/scratch/rv95_u9_01; TG=$T3/targets; C=$T3/rv95/projects/chirality-piping/core; R=$S/scripts/cargo_run.sh
mkdir -p $S/live3
$R pp_reg_full_F $C/product_physics/Cargo.toml $TG/rv95/pp "I61_U3G2_OUT=$S/live3"
echo CHAIN4_DONE
