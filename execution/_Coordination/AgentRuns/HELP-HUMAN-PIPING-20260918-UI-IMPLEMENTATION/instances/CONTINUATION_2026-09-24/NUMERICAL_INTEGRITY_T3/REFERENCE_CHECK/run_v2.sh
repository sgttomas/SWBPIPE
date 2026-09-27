#!/bin/sh
# V2 reference check: full pipeline, single-threaded, standard-library Python only.
# Usage (from this folder): sh run_v2.sh <work-dir>
# Bulky intermediates (v2_values.json, ~150 MB) go to <work-dir>; compact outputs to _run_records/.
set -e
W=${1:?work dir}
REF=../REFERENCES/references.json
PY=${PYTHON:-python3}
mkdir -p "$W" _run_records
N="nice -n 19"
$N $PY -B v2_derive.py  $REF "$W/v2_values.json"                        > _run_records/v2_derive.stdout.txt
$N $PY -B v2_mech.py    $REF _run_records/v2_mech.json                   > _run_records/v2_mech.stdout.txt
$N $PY -B v2_compare.py $REF "$W/v2_values.json" _run_records/v2_compare.json > _run_records/v2_compare.stdout.txt
$N $PY -B v2_cancel.py  $REF "$W/v2_values.json" _run_records/v2_cancel.json _run_records/v2_compare.json > _run_records/v2_cancel.stdout.txt
$N $PY -B v2_nc.py      $REF "$W/v2_values.json" _run_records/v2_nc.json      > _run_records/v2_nc.stdout.txt
$N $PY -B v2_defects.py $REF "$W/v2_values.json" _run_records/v2_defects.json > _run_records/v2_defects.stdout.txt
$N $PY -B v2_floor.py   $REF "$W/v2_values.json" _run_records/v2_compare.json _run_records/v2_floor.json > _run_records/v2_floor.stdout.txt
$N $PY -B v2_tail.py    $REF                                             > _run_records/v2_tail.stdout.txt
$N $PY -B v2_trunc.py   $REF                                             > _run_records/v2_trunc.stdout.txt
$N $PY -B v2_zeroscale.py $REF                                         > _run_records/v2_zeroscale.stdout.txt
