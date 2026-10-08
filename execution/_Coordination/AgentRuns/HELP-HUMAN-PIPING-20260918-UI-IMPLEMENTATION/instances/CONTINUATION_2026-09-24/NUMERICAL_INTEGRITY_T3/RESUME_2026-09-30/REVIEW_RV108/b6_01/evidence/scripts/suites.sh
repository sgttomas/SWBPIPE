#!/bin/bash
# RV108 suites: base and cand copies, base against head. Cargo through t3_cargo.sh; Python, vitest and tsc under the T3 lock.
set -u
source S/env.sh
LOCK=$WT/guard/cargo_job.lock
TGT=$WT/targets/rv108-b6
export OPENPIPESTRESS_UNITS_BIN=$TGT/units-authority/release/openpipestress_units
export OPENPIPESTRESS_CHECKED_JSON_BIN=$TGT/checked-json/release/openpipestress_jcs_ijson
export PYTHONDONTWRITEBYTECODE=1
SWEEP=$(cat $S/sweep.txt)
log() { echo "$(date -u '+%FT%TZ') $*" >> $S/logs/suites_progress.txt; }
cargo_env() { env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 TMPDIR=$S/tmp "$@"; }
for what in "$@"; do
  log "start $what"
  case $what in
    clis)
      (cd $WT/rv108/base/$P/core/serialization/canonical_json && cargo_env $WT/tools/t3_cargo.sh build --locked --offline --release --features checked-cli --bin openpipestress_jcs_ijson --target-dir $TGT/checked-json) > $S/logs/clis_checked.log 2>&1; log "checked-json rc=$?"
      (cd $WT/rv108/base/$P/core/units && cargo_env $WT/tools/t3_cargo.sh build --locked --offline --release --features cli --bin openpipestress_units --target-dir $TGT/units-authority) > $S/logs/clis_units.log 2>&1; log "units rc=$?" ;;
    rs-base|rs-cand)
      L=${what#rs-}; (cd $WT/rv108/$L/$P/core/reporting/result_export && cargo_env CARGO_TARGET_DIR=$TGT/rx $WT/tools/t3_cargo.sh test --locked --offline --no-fail-fast) > $S/logs/rs_$L.log 2>&1; log "rs $L rc=$?" ;;
    py-base|py-cand)
      L=${what#py-}; mkdir -p $S/tmp/py_$L; (cd $WT/rv108/$L/$P && /usr/bin/lockf -k $LOCK env TMPDIR=$S/tmp/py_$L $VENV/bin/python -m pytest -p no:cacheprovider --basetemp=$S/tmp/py_$L/basetemp -q -rs -rf -v --junitxml=$S/logs/py_$L.xml $SWEEP) > $S/logs/py_$L.log 2>&1; log "py $L rc=$?" ;;
    ts-base|ts-cand)
      L=${what#ts-}; (cd $WT/rv108/$L/$P/apps/desktop && /usr/bin/lockf -k $LOCK env TMPDIR=$S/tmp ../../node_modules/.bin/vitest run --reporter=dot --reporter=json --outputFile.json=$S/logs/vitest_$L.json) > $S/logs/vitest_$L.log 2>&1; log "ts $L rc=$?" ;;
    tsc-base|tsc-cand)
      L=${what#tsc-}; (cd $WT/rv108/$L/$P/apps/desktop && /usr/bin/lockf -k $LOCK env TMPDIR=$S/tmp ../../node_modules/.bin/tsc --noEmit -p tsconfig.json) > $S/logs/tsc_$L.log 2>&1; log "tsc $L rc=$?" ;;
  esac
done
log "all done: $*"
