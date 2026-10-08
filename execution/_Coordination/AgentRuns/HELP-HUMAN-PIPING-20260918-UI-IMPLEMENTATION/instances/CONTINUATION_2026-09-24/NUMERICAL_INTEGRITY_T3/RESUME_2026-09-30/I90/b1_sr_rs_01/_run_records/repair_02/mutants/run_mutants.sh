#!/bin/bash
# I90 SR-RS repair 2: build the mutant schema once (one cargo job through t3_cargo.sh), then run RE's
# contract and lib test binaries for the control and each mutant, each run in a T3 lock slot (t3_slot.sh).
set -u
WT=WT
S=$WT/scratch/i90_b1_sr_rs; M=$S/repair_02/mutants; L=$M/logs; mkdir -p $L
RE=$S/repair_02/mut/projects/chirality-piping/core/reporting/result_export
cd $RE && unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS I90_MUT
export TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 CARGO_TARGET_DIR=$WT/targets/i90-b1-sr-rs/mut-r2
$WT/tools/t3_cargo.sh test --locked --offline --no-run --test retained_precision_contract --lib > $L/build.log 2>&1; echo "build rc=$?"
BINS=$(grep -o "(/[^)]*deps/[^)]*)" $L/build.log | tr -d '()')
echo "$BINS" > $L/binaries.txt
for id in control $(python3 -c "import json;print(' '.join(m['id'] for m in json.load(open('$M/manifest.json'))))"); do
  : > $L/$id.log
  for bin in $BINS; do
    if [ $id = control ]; then
      $WT/tools/t3_slot.sh env RUST_TEST_THREADS=2 $bin >> $L/$id.log 2>&1
    else
      $WT/tools/t3_slot.sh env I90_MUT=$id RUST_TEST_THREADS=2 $bin >> $L/$id.log 2>&1
    fi
  done
  echo "$id: $(grep -c '^test .* ok$' $L/$id.log) ok, $(grep -c '^test .* FAILED$' $L/$id.log) failed: $(grep '^test .* FAILED$' $L/$id.log | awk '{print $2}' | tr '\n' ' ')"
done
