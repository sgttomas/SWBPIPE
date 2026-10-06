#!/bin/bash
WT=WT
S=$WT/scratch/rv101_t6s_01
P=$WT/rv101/cand/projects/chirality-piping
export TMPDIR=$S/tmp CARGO_BUILD_JOBS=4
(cd $P/core/serialization/canonical_json && env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS $WT/tools/t3_cargo.sh build --locked --offline --release --features checked-cli --bin openpipestress_jcs_ijson --target-dir $WT/targets/rv101/checked-json) > $S/logs/build_checked_json.log 2>&1; echo "rc=$?" >> $S/logs/build_checked_json.log
(cd $P/core/units && env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS $WT/tools/t3_cargo.sh build --locked --offline --release --features cli --bin openpipestress_units --target-dir $WT/targets/rv101/units-authority) > $S/logs/build_units.log 2>&1; echo "rc=$?" >> $S/logs/build_units.log
