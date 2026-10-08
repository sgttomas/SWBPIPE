#!/bin/bash
# RV113: one heavy Python job under the T3 lock, with the reviewer's own authority builds.
# Usage: run_py_job.sh <label> <cwd> <args to python...>   (logs to WT/scratch/rv113_rvr_01/py/logs/<label>.log)
WT=WT
S=$WT/scratch/rv113_rvr_01/py
VENV=VENV
B=$WT/targets/rv113-pybins/release
label=$1; dir=$2; shift 2
log=$S/logs/$label.log
mkdir -p "$S/tmp/$label"
{ echo "# rv113 py job $label"; echo "# queued $(date -u '+%FT%TZ') dir=${dir#$WT/} args: $*"; } > "$log"
cd "$dir" || exit 90
/usr/bin/lockf -k "$WT/guard/cargo_job.lock" /usr/bin/env TMPDIR="$S/tmp/$label" \
  OPENPIPESTRESS_CHECKED_JSON_BIN="$B/openpipestress_jcs_ijson" OPENPIPESTRESS_BINARY64_JSON_BIN="$B/openpipestress_jcs_binary64" OPENPIPESTRESS_UNITS_BIN="$B/openpipestress_units" \
  /bin/bash -c 'echo "# start $(date -u "+%FT%TZ")"; "$0" "$@"; rc=$?; echo "# end $(date -u "+%FT%TZ") rc=$rc"; exit $rc' "$VENV/bin/python" "$@" >> "$log" 2>&1
