#!/bin/bash
# I86 item 4 (restart): one host lock (lockf -k WT/guard/cargo_job.lock) per input, held for
# that input's six single-run processes (timing_input.sh), so no other T3 job runs between them.
# Usage: timing2.sh <binary> <out_dir> <rep> <input.json>...
set -u
WT=${WT:?set WT to the T3 worktree root (placeholder WT)}
S=$WT/scratch/i86_b1_w
BIN=$1; OUT=$2; REP=$3; shift 3
for input in "$@"; do
  echo "$(date -u '+%FT%TZ') lock-wait $(basename "$input" .json)" >> "$OUT/timing_index.log"
  /usr/bin/lockf -k "$WT/guard/cargo_job.lock" /bin/bash "$S/timing_input.sh" "$BIN" "$OUT" "$REP" "$input"
done
echo "$(date -u '+%FT%TZ') all done" >> "$OUT/timing_index.log"
