#!/bin/bash
# I73 S-I1: the Python sweep (piping-pytest: pytest -q tests --dist loadscope) on a
# scratch tree, with cargo removed from PATH so no test can start an unlocked
# cargo job, and the prebuilt checked-JSON and units CLIs supplied.
set -u
WT=<WT>
VENV=<VENV>
label=$1; tree=$2
pgrep -f "$WT/guard/memguard.sh" >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
LOG=$WT/scratch/i73_s_i1/logs/sweep_$label.log
NEWPATH=$(echo "$PATH" | tr ':' '\n' | grep -v "/.cargo/bin" | paste -sd: -)
mkdir -p $WT/scratch/i73_s_i1/tmp/sweep_$label
echo "== sweep $label start $(date -u +%FT%TZ)" > $LOG
( cd $tree/projects/chirality-piping && env PATH="$NEWPATH" TMPDIR=$WT/scratch/i73_s_i1/tmp/sweep_$label PYTHONDONTWRITEBYTECODE=1 \
  OPENPIPESTRESS_CHECKED_JSON_BIN=$WT/targets/i73-s-i1/checked-json/release/openpipestress_jcs_ijson \
  OPENPIPESTRESS_UNITS_BIN=$WT/targets/i73-s-i1/units-authority/release/openpipestress_units \
  perl -e 'alarm shift; exec @ARGV' 5400 $VENV/bin/python -m pytest -q -p no:cacheprovider tests -n 6 --dist loadscope \
  --junitxml=$WT/scratch/i73_s_i1/sweep/junit_$label.xml >> $LOG 2>&1 )
rc=$?
echo "== sweep $label exit=$rc $(date -u +%FT%TZ)" >> $LOG
tail -3 $LOG
exit $rc
