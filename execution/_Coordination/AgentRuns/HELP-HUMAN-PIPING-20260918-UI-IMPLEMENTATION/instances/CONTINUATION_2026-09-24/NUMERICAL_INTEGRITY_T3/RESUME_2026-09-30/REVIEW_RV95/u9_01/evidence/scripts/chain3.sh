#!/bin/bash
# RV95 chain 3 (PR head 6d8f8a82b2): the Rust reader dump, the PHYS-R4 Direct sample (registered and Stale),
# PP full suites (registered and Stale), runner. One cargo job at a time.
T3=WT
S=$T3/scratch/rv95_u9_01; TG=$T3/targets; C=$T3/rv95/projects/chirality-piping/core; R=$S/scripts/cargo_run.sh
cp $S/scripts/zz_rv95_dump.rs $C/reporting/result_export/tests/zz_rv95_dump.rs
cp $S/scripts/zz_rv95_physr4.rs $C/product_physics/tests/zz_rv95_physr4.rs
$R rs_reader_dump $C/reporting/result_export/Cargo.toml $TG/rv95/re "RV95_IN=$S/live2 RV95_OUT=$S/readers/rs.json" --test zz_rv95_dump -- --ignored --nocapture
$R physr4_reg $C/product_physics/Cargo.toml $TG/rv95/pp - --test zz_rv95_physr4 zz_rv95_phys_r4_direct -- --nocapture
$R physr4_stale $C/product_physics/Cargo.toml $TG/rv95-stale/pp "RUSTFLAGS=--cfg=rv95_stale" --test zz_rv95_physr4 zz_rv95_phys_r4_direct -- --nocapture
rm $C/reporting/result_export/tests/zz_rv95_dump.rs $C/product_physics/tests/zz_rv95_physr4.rs
$R pp_reg_full_6d8f $C/product_physics/Cargo.toml $TG/rv95/pp "I61_U3G2_OUT=$S/live2"
$R pp_stale_full_6d8f $C/product_physics/Cargo.toml $TG/rv95-stale/pp "RUSTFLAGS=--cfg=rv95_stale"
$R runner_reg_6d8f $C/runner/headless/Cargo.toml $TG/rv95/runner -
echo CHAIN3_DONE
