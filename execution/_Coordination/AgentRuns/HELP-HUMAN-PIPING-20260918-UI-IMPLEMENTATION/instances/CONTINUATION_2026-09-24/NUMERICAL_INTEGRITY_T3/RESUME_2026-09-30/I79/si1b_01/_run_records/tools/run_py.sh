#!/bin/bash
# I79 T3-SI1b: pytest under the T3 host rules. Args: label root(P) pytest-args...
set -u
WT=WT
VENV=VENV
label=$1; root=$2; shift 2
pgrep -f "$WT/guard/memguard.sh" >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
LOG=$WT/scratch/i79_si1b/logs/$label.log
echo "== $label start $(date -u +%FT%TZ)" > $LOG
( cd $root && TMPDIR=$WT/scratch/i79_si1b/tmp PYTHONDONTWRITEBYTECODE=1 \
  $VENV/bin/python -m pytest -q -p no:cacheprovider "$@" >> $LOG 2>&1 )
rc=$?
echo "== $label exit=$rc $(date -u +%FT%TZ)" >> $LOG
tail -3 $LOG
exit $rc
