#!/bin/zsh
# I61 U7 slice L: RV92's survival chain on the live successors (the lane's milestone files are the
# PP Direct entry's live bytes). Step a in each language, then step b in each. One cargo job at a time.
T=WT; S=$T/scratch/i61_u7_slice_l_01; L=$S/lane/projects/chirality-piping; D=$S/survival
export CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 OPENPIPESTRESS_CHECKED_JSON_BIN=$T/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson OPENPIPESTRESS_UNITS_BIN=$T/targets/i52-readers/units/release/openpipestress_units
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; } }
RE=$L/core/reporting/result_export
for step in a b; do
  guard; (cd $RE && env -u RUSTFLAGS RV92_SURVIVAL=$D RV92_STEP=$step perl -e 'alarm shift; exec @ARGV' 1800 cargo test --locked --offline --target-dir $T/targets/i61-u7l/re --test zz_rv92_survival > $S/logs/survival_rust_$step.log 2>&1); echo "rust $step exit=$?"
  (cd $L && PYTHONDONTWRITEBYTECODE=1 python3 $S/harness/rv92_py_survival.py $L $D $step > $S/logs/survival_py_$step.log 2>&1); echo "py $step exit=$?"
  (cd $L/apps/desktop && RV92_SURVIVAL=$D RV92_STEP=$step perl -e 'alarm shift; exec @ARGV' 1200 ../../node_modules/.bin/vitest run --maxWorkers=1 src/zzRV92Survival.test.ts > $S/logs/survival_ts_$step.log 2>&1); echo "ts $step exit=$?"
done
