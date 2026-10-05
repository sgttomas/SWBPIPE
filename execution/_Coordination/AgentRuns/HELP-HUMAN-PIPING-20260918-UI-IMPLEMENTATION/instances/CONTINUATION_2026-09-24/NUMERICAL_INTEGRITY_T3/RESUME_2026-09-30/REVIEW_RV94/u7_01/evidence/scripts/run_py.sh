#!/bin/bash
# RV94: Python tests. Args: label root(P) tests...
set -u
WT=WT; S=$WT/scratch/rv94_u7_01
VENV=REPO_ROOT/projects/chirality-piping/.venv
label=$1; root=$2; shift 2
ps -p 5387 >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
echo "== $label start $(date -u +%FT%TZ)"
( cd $root && TMPDIR=$S/tmp PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN=$WT/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson \
  OPENPIPESTRESS_UNITS_BIN=$WT/targets/i52-readers/units/release/openpipestress_units \
  perl -e 'alarm shift; exec @ARGV' 1800 $VENV/bin/python -m pytest -q -p no:cacheprovider --basetemp=$S/tmp/pytest_$label -rA "$@" > $S/logs/$label.log 2>&1 )
echo "== $label exit=$? $(date -u +%FT%TZ)"; tail -3 $S/logs/$label.log
