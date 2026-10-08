#!/bin/bash
# I86 item 4, one input: its six runs (two modes x value, direct, witness), each a separate
# process under /usr/bin/time -l. Called by timing2.sh under one host lock per input.
# Usage: timing_input.sh <binary> <out_dir> <rep> <input.json>
set -u
WT=${WT:?set WT to the T3 worktree root (placeholder WT)}
S=$WT/scratch/i86_b1_w
BIN=$1; OUT=$2; REP=$3; input=$4
cd "$S/arch/projects/chirality-piping/core/product_physics" || exit 1
export I86_QUIET=1 I86_OUT=$S/out TMPDIR=$S/tmp
label=$(basename "$input" .json)
for mode in sparse_interactive dense_scrutiny; do
  for path in value direct witness; do
    name="${label}__${mode}__${path}__r$REP"
    echo "$(date -u '+%FT%TZ') begin $name" >> "$OUT/timing_index.log"
    I86_FILE="$input" I86_MODE=$mode I86_PATH=$path \
      /usr/bin/time -l "$BIN" retained_memory::zz_i86_probe::zz_i86_once --exact --ignored --nocapture --test-threads=1 > "$OUT/$name.log" 2>&1
    echo "$(date -u '+%FT%TZ') end $name rc=$?" >> "$OUT/timing_index.log"
  done
done
