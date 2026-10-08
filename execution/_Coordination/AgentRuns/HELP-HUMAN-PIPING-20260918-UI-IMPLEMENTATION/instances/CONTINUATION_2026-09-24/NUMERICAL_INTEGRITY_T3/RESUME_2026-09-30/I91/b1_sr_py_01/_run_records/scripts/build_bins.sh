#!/bin/bash
# Builds the two CLI authorities for pytest into I91's own target, through the T3 cargo wrapper.
set -u
WT=WT
S=$WT/scratch/i91_b1_sr_py
P=$WT/b1-p/projects/chirality-piping
TG=$WT/targets/i91-b1-sr-py
export TMPDIR=$S/tmp
cd $P/core/serialization/canonical_json && env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 $WT/tools/t3_cargo.sh build --locked --offline --release --features checked-cli --bin openpipestress_jcs_ijson --target-dir $TG/checked-json > $S/logs/build_checked_json.log 2>&1
echo "checked-json rc=$?" >> $S/logs/build_bins.rc
cd $P/core/units && env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 $WT/tools/t3_cargo.sh build --locked --offline --release --features cli --bin openpipestress_units --target-dir $TG/units-authority > $S/logs/build_units.log 2>&1
echo "units rc=$?" >> $S/logs/build_bins.rc
echo done >> $S/logs/build_bins.rc
