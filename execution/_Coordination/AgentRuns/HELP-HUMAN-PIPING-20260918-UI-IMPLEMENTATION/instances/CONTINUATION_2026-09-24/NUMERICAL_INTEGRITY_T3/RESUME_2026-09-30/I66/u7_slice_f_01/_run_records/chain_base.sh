#!/bin/bash
S=WT/scratch/i66_u7f; B=$S/base/projects/chirality-piping/core; TG=WT/targets/i66-u7f
$S/cargo_run.sh base_re $B/reporting/result_export/Cargo.toml $TG/re-base -
$S/cargo_run.sh base_pp_reg $B/product_physics/Cargo.toml $TG/pp-base-reg -
$S/cargo_run.sh base_pp_stale $B/product_physics/Cargo.toml $TG/pp-base-stale RUSTFLAGS=--cfg=i61_u3g2_stale
$S/cargo_run.sh base_runner $B/runner/headless/Cargo.toml $TG/runner-base -
echo CHAIN_BASE_DONE
