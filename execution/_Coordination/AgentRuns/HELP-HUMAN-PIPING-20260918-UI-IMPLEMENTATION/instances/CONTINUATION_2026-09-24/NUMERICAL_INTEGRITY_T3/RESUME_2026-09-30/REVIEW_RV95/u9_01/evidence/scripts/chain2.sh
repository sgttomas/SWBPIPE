#!/bin/bash
# RV95 chain 2 (PR head 92a5a9da1c): result_export, PP registered sweep + retained lib tests, Stale sweep, wasm32 checks. One cargo job at a time.
T3=WT
S=$T3/scratch/rv95_u9_01; TG=$T3/targets; C=$T3/rv95/projects/chirality-piping/core; R=$S/scripts/cargo_run.sh
mkdir -p $S/live2
$R re_full_92a5 $C/reporting/result_export/Cargo.toml $TG/rv95/re -
$R pp_reg_sweep_92a5 $C/product_physics/Cargo.toml $TG/rv95/pp "I61_U3G2_SWEEP=$S/sweep/sweep_reg_92a5.tsv" --lib zz_i61_u3g2_sweep -- --ignored --nocapture
$R pp_reg_retained_92a5 $C/product_physics/Cargo.toml $TG/rv95/pp "I61_U3G2_OUT=$S/live2" --lib retained
$R pp_stale_sweep_92a5 $C/product_physics/Cargo.toml $TG/rv95-stale/pp "RUSTFLAGS=--cfg=rv95_stale I61_U3G2_SWEEP=$S/sweep/sweep_stale_92a5.tsv" --lib zz_i61_u3g2_sweep -- --ignored --nocapture
for crate in reporting/result_export product_physics loads/self_weight_wasm model_operations/operation_applier; do
  ps -p 5387 >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  l=wasm_check_$(basename $crate)
  echo "== $l start $(date -u +%FT%TZ)"
  ( cd $C/$crate && env -u RUSTFLAGS TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 perl -e 'alarm shift; exec @ARGV' 1800 cargo check --locked --offline --target wasm32-unknown-unknown --lib --target-dir $TG/rv95-wasm > $S/logs/$l.log 2>&1 )
  echo "== $l exit=$? $(date -u +%FT%TZ)"
done
echo CHAIN2_DONE
