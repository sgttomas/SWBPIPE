#!/bin/bash
# I73 S-I1: pytest under the T3 host rules. Args: label root(P) pytest-args...
set -u
WT=<WT>
VENV=<VENV>
label=$1; root=$2; shift 2
pgrep -f "$WT/guard/memguard.sh" >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
LOG=$WT/scratch/i73_s_i1/logs/$label.log
echo "== $label start $(date -u +%FT%TZ)" > $LOG
( cd $root && TMPDIR=$WT/scratch/i73_s_i1/tmp PYTHONDONTWRITEBYTECODE=1 \
  OPENPIPESTRESS_CHECKED_JSON_BIN=$WT/targets/i73-s-i1/checked-json/release/openpipestress_jcs_ijson \
  OPENPIPESTRESS_UNITS_BIN=$WT/targets/i73-s-i1/units-authority/release/openpipestress_units \
  $VENV/bin/python -m pytest -q -p no:cacheprovider "$@" >> $LOG 2>&1 )
rc=$?
echo "== $label exit=$rc $(date -u +%FT%TZ)" >> $LOG
tail -3 $LOG
exit $rc
