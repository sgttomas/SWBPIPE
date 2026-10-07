#!/bin/bash
# I86 B1-SW item 4 (informational; SQ measures): one run per process, one mode per process,
# under /usr/bin/time -l, the registered dev/test lib test binary run directly (not through
# cargo), each process under the T3 host lock (lockf -k WT/guard/cargo_job.lock), with
# I86_QUIET=1 (no seed print serializes the envelope). Paths: value (the ordinary value route
# only), direct (the Direct entry, the permitted path), witness (the private driver at
# R/k = 4 MiB: the furthest W1 phase). Plus the process floor (a filter matching no test).
# Usage: timing.sh <binary> <out_dir> <rep> <input.json>...
set -u
WT=${WT:?set WT to the T3 worktree root (placeholder WT)}
S=$WT/scratch/i86_b1_w
BIN=$1; OUT=$2; REP=$3; shift 3
mkdir -p "$OUT"
cd "$S/arch/projects/chirality-piping/core/product_physics" || exit 1
export I86_QUIET=1 I86_OUT=$S/out TMPDIR=$S/tmp
run() { # name, then env assignments, then args
  local name=$1; shift
  echo "$(date -u '+%FT%TZ') begin $name" >> "$OUT/timing_index.log"
  /usr/bin/lockf -k "$WT/guard/cargo_job.lock" /usr/bin/env "$@" > "$OUT/$name.log" 2>&1
  echo "$(date -u '+%FT%TZ') end $name rc=$?" >> "$OUT/timing_index.log"
}
run "floor_r$REP" /usr/bin/time -l "$BIN" i86_no_such_test_floor --exact --ignored --test-threads=1
for input in "$@"; do
  label=$(basename "$input" .json)
  for mode in sparse_interactive dense_scrutiny; do
    for path in value direct witness; do
      run "${label}__${mode}__${path}__r$REP" I86_FILE="$input" I86_MODE=$mode I86_PATH=$path \
        /usr/bin/time -l "$BIN" retained_memory::zz_i86_probe::zz_i86_once --exact --ignored --nocapture --test-threads=1
    done
  done
done
