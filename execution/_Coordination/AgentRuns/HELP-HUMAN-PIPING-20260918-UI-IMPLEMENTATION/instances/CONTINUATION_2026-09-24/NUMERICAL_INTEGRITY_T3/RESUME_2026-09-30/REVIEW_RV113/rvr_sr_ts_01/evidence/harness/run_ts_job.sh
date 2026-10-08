#!/bin/bash
# RV113: one heavy TS job (tsc or vitest) under the T3 lock. Usage: run_ts_job.sh <label> <desktop dir> <tool: tsc|vitest> <args...>
# Logs to WT/scratch/rv113_rvr_01/ts/logs/<label>.log with start/end stamps and the exit code.
WT=WT
S=$WT/scratch/rv113_rvr_01/ts
NMS=NMS
label=$1; dir=$2; tool=$3; shift 3
log=$S/logs/$label.log
export TMPDIR=$S/tmp
{ echo "# rv113 ts job $label"; echo "# queued $(date -u '+%FT%TZ') dir=${dir#$WT/} tool=$tool args: $*"; } > "$log"
cd "$dir" || exit 90
/usr/bin/lockf -k "$WT/guard/cargo_job.lock" /bin/bash -c 'echo "# start $(date -u "+%FT%TZ")"; "$0" "$@"; rc=$?; echo "# end $(date -u "+%FT%TZ") rc=$rc"; exit $rc' "$NMS/.bin/$tool" "$@" >> "$log" 2>&1
rc=$?
rm -rf "$dir/node_modules/.vite"
exit $rc
