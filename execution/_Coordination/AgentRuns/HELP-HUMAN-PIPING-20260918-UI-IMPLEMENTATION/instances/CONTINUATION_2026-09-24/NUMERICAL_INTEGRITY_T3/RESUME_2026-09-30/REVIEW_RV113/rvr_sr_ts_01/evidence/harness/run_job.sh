#!/bin/bash
# RV113: one cargo job through the T3 lock. Usage: run_job.sh <label> <crate dir> <target dir> <cargo args...>
# Logs to WT/scratch/rv113_rvr_01/logs/<label>.log with start/end stamps and the exit code.
WT=WT
S=$WT/scratch/rv113_rvr_01
label=$1; dir=$2; target=$3; shift 3
mkdir -p "$S/logs" "$S/tmp"
log=$S/logs/$label.log
export TMPDIR=$S/tmp
export CARGO_TARGET_DIR=$target
{
  echo "# rv113 job $label"
  echo "# start $(date -u '+%FT%TZ') dir=${dir#$WT/} target=${target#$WT/} args: $*"
} > "$log"
cd "$dir" || exit 90
"$WT/tools/t3_cargo.sh" "$@" >> "$log" 2>&1
rc=$?
echo "# end $(date -u '+%FT%TZ') rc=$rc" >> "$log"
exit $rc
