#!/bin/bash
# I76 T6S: the Python sweep, base or candidate. Args: label root(P) [pytest args...]
set -u
WT=WT; S=$WT/scratch/i76_t6s
VENV=VENV
label=$1; root=$2; shift 2
pgrep -f "$WT/guard/memguard.sh" >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
mkdir -p $S/tmp/$label; rm -rf $S/tmp/$label/*
echo "== $label start $(date -u +%FT%TZ)"
( cd $root && GIT_CEILING_DIRECTORIES=$S GIT_OPTIONAL_LOCKS=0 I76_LABEL=$label PYTHONDONTWRITEBYTECODE=1 TMPDIR=$S/tmp/$label PATH=$S/shim:$PATH \
  OPENPIPESTRESS_CHECKED_JSON_BIN=$WT/targets/i76-t6s-base/checked-json/release/openpipestress_jcs_ijson \
  OPENPIPESTRESS_UNITS_BIN=$WT/targets/i76-t6s-base/units-authority/release/openpipestress_units \
  perl -e 'alarm shift; exec @ARGV' 3600 $VENV/bin/python -m pytest -q -p no:cacheprovider -rA \
  --basetemp=$S/tmp/$label/basetemp --junitxml=$S/logs/$label.junit.xml "$@" > $S/logs/$label.log 2>&1 )
rc=$?; echo "== $label exit=$rc $(date -u +%FT%TZ)"; tail -3 $S/logs/$label.log
