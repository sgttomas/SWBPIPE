#!/bin/bash
# usage: run_iter.sh ITER [VARIANT]   — emit receipts, then run the three unchanged draft readers.
set -u
W=WT; X=$W/scratch/i61_receipt_experiment_02
IT=$1; VAR=${2:-}; O=$X/out/$IT; mkdir -p $O; rm -f $O/*.json
RP=$X/reader/projects/chirality-piping; VENV=VENV
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
cd $X/prod/projects/chirality-piping/core/product_physics
env I61_OUT=$O I61_DEFINITION=$RP/fixtures/results/retained_precision_prepared_ordinary_v1.json CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 \
  perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --manifest-path $X/prod/projects/chirality-piping/core/product_physics/Cargo.toml \
  --target-dir $W/targets/i61-receipt/product_physics --lib i61_emit_receipts -- --nocapture > $X/logs/${IT}_emit.log 2>&1
echo "emit exit=$?"; grep -E "I61_EMITTED|panicked" -A1 $X/logs/${IT}_emit.log | grep -v I51_ | head
FILES=$(ls $O/*.json | grep -v provenance | tr '\n' ' ')
cd $RP; OPENPIPESTRESS_CHECKED_JSON_BIN=$W/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson OPENPIPESTRESS_UNITS_BIN=$W/targets/i52-readers/units/release/openpipestress_units \
  perl -e 'alarm shift; exec @ARGV' 600 $VENV/bin/python $X/harness_py.py $RP $FILES > $X/logs/${IT}_py.log 2>&1; cat $X/logs/${IT}_py.log | cut -c1-300
cd $RP/core/reporting/result_export; I61_RECEIPTS=$(echo $FILES | tr ' ' ':') CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_NET_OFFLINE=true perl -e 'alarm shift; exec @ARGV' 1200 \
  cargo test --locked --offline --manifest-path $RP/core/reporting/result_export/Cargo.toml --target-dir $W/targets/i61-receipt/result_export --test i61_receipt_probe -- --nocapture > $X/logs/${IT}_rs.log 2>&1
grep -E "I61_RS|^error" $X/logs/${IT}_rs.log
echo $FILES | tr ' ' '\n' > $X/out/ts_receipts.txt; rm -f $X/out/ts_results.txt
cd $RP/apps/desktop; perl -e 'alarm shift; exec @ARGV' 600 npm test -- src/features/results/i61Probe.test.ts --maxWorkers=2 > $X/logs/${IT}_ts.log 2>&1
cat $X/out/ts_results.txt; cp $X/out/ts_results.txt $X/logs/${IT}_ts_results.txt
