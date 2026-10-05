#!/bin/bash
# RV92: pytest runner. Args: label root(P) tests...
set -u
WT=WT; S=$WT/scratch/rv92_u6f
VENV=REPO_ROOT/projects/chirality-piping/.venv
export TMPDIR=$S/tmp
label=$1; root=$2; shift 2
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
echo "== $label start $(date -u +%FT%TZ)"
( cd $root && PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN=$WT/targets/rv92/cli/release/openpipestress_jcs_ijson \
  OPENPIPESTRESS_UNITS_BIN=$WT/targets/rv92/cli/release/openpipestress_units \
  perl -e 'alarm shift; exec @ARGV' 2400 $VENV/bin/python -m pytest -q -p no:cacheprovider -rA "$@" > $S/logs/$label.log 2>&1 )
echo "== $label exit=$? $(date -u +%FT%TZ)"; tail -3 $S/logs/$label.log
