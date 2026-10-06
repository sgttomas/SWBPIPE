#!/bin/bash
# I70 records-only probe runs in the scratch probe copy (never the candidate).
# usage: run_probes.sh <mutant>...   (each of: none RM1 RM2 RM3 RM4 TMA)
WT="${WT:?set WT to the T3 root}"
S=$WT/scratch/i70_u8
P=$S/probe/projects/chirality-piping
RX=$P/core/reporting/result_export
SRC=$RX/src/retained_precision.rs
CORPUS=$P/fixtures/results/retained_precision_cases.json
mkdir -p $S/pristine $S/logs/probes
[ -f $S/pristine/retained_precision.rs ] || cp $SRC $S/pristine/retained_precision.rs
[ -f $S/pristine/retained_precision_cases.json ] || cp $CORPUS $S/pristine/retained_precision_cases.json
for name in "$@"; do
  cp $S/pristine/retained_precision.rs $SRC
  cp $S/pristine/retained_precision_cases.json $CORPUS
  python3 $S/scripts/mutate.py $P $name || exit 2
  shasum -a 256 $SRC $CORPUS $RX/tests/retained_precision_contract.rs > $S/logs/probes/$name.inputs
  (cd $RX && env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 \
     CARGO_TARGET_DIR=$WT/targets/i70-u8 $WT/tools/t3_cargo.sh test --locked --offline \
     --test retained_precision_contract -- --show-output) > $S/logs/probes/$name.log 2>&1
  echo "rc=$?" > $S/logs/probes/$name.rc
done
# restore and check
cp $S/pristine/retained_precision.rs $SRC
cp $S/pristine/retained_precision_cases.json $CORPUS
shasum -a 256 $SRC $CORPUS
