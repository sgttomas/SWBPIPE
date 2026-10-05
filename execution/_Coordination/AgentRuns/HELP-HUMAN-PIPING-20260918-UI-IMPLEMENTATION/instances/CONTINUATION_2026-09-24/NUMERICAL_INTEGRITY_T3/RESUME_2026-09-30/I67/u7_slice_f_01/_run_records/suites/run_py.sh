#!/bin/bash
# I67 U7 slice F part 2: Python tests as I66's run_py.sh. Args: label root(P) tests...
set -u
WT=WT; S=$WT/scratch/i67_u6d/r7
VENV=REPO_ROOT/projects/chirality-piping/.venv
label=$1; root=$2; shift 2
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
echo "== $label start $(date -u +%FT%TZ)"
( cd $root && TMPDIR=$WT/scratch/i67_u6d/tmp PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN=$WT/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson \
  OPENPIPESTRESS_UNITS_BIN=$WT/targets/i52-readers/units/release/openpipestress_units \
  perl -e 'alarm shift; exec @ARGV' 1800 $VENV/bin/python -m pytest -q -p no:cacheprovider -rA "$@" > $S/logs/$label.log 2>&1 )
echo "== $label exit=$? $(date -u +%FT%TZ)"; tail -3 $S/logs/$label.log
