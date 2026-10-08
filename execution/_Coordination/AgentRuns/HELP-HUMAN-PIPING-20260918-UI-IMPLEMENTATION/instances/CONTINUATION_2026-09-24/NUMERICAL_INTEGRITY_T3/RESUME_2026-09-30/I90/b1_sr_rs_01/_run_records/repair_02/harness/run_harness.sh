#!/bin/bash
# I90 SR-RS repair 2: RV113's harness (census over 07m, and its 135 probes) on one archive, one cargo job.
# Usage: run_harness.sh <side> (base = b5cb7faaeb archive; head = the repair head archive)
set -u
WT=WT
S=$WT/scratch/i90_b1_sr_rs; side=$1
RE=$S/repair_02/$side/projects/chirality-piping/core/reporting/result_export
cd $RE && unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
RV113_OUT=$S/repair_02/out/census_$side.jsonl RV113_PROBES=$S/repair_02/harness/probes_all.json RV113_PROBES_OUT=$S/repair_02/out/probes_$side.jsonl \
TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$WT/targets/i90-b1-sr-rs/harness-$side \
  $WT/tools/t3_cargo.sh test --locked --offline --test rv113_census > $S/repair_02/out/harness_$side.log 2>&1
echo "harness $side rc=$?"
