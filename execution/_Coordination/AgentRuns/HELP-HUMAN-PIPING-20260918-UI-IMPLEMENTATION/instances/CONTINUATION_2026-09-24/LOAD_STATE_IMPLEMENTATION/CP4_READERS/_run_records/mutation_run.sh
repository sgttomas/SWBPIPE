#!/usr/bin/env bash
# CP4_READERS mutation run. $1 = scratch copy of WORKING_ROOT, $2 = RR, $3 = worktree WORKING_ROOT (helper binaries only).
set -u
SCR=$1; RR=$2; WR=$3
export PYTHONDONTWRITEBYTECODE=1 CARGO_TARGET_DIR=${CARGO_TARGET_DIR:?}
export OPENPIPESTRESS_CHECKED_JSON_BIN=$WR/core/serialization/canonical_json/target/checked-json/release/openpipestress_jcs_ijson
export OPENPIPESTRESS_UNITS_BIN=$WR/core/units/target/units-authority/release/openpipestress_units
PYFILE=core/analysis_runs/load_reference_evidence.py; RSFILE=core/reporting/result_export/src/load_reference.rs
( cd "$SCR" && sha256sum $PYFILE $RSFILE ) > "$RR/mutation_prehash.txt"
for m in $(python3 "$RR/mutants.py" "$SCR" x list); do
  cp "$SCR/$PYFILE" "$SCR/$PYFILE.orig"; cp "$SCR/$RSFILE" "$SCR/$RSFILE.orig"
  python3 "$RR/mutants.py" "$SCR" "$m" apply
  case $m in
    PY-*) ( cd "$SCR" && timeout 2400 "$PYTHON" -m pytest -q -p no:cacheprovider tests/test_load_reference_readers.py 2>&1 ) > "$RR/mutants/$m.log";;
    RS-*) ( cd "$SCR/core/reporting/result_export" && timeout 3000 cargo +1.97.1 test --locked --offline -j 2 --test load_reference_contract 2>&1 ) > "$RR/mutants/$m.log";;
  esac
  echo "$m exit=$? $(grep -E '^[0-9]+ (passed|failed)|[0-9]+ failed|^test result' "$RR/mutants/$m.log" | tail -1)"
  mv "$SCR/$PYFILE.orig" "$SCR/$PYFILE"; mv "$SCR/$RSFILE.orig" "$SCR/$RSFILE"
done
( cd "$SCR" && sha256sum $PYFILE $RSFILE ) > "$RR/mutation_posthash.txt"
cmp "$RR/mutation_prehash.txt" "$RR/mutation_posthash.txt" && echo "restored: scratch hashes equal the pre-mutation hashes"
