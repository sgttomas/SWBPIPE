#!/bin/bash
# RV113: rerun only the reviewer probes for the named mutants (the batch's harness step, probes only).
# Usage: run_mutant_probes.sh <probes json> <ids...>
WT=WT
S=$WT/scratch/rv113_rvr_01
D=$WT/targets/rv113-mut/debug/deps
RE=$WT/rv113/mut/projects/chirality-piping/core/reporting/result_export
probes=$1; shift
CENSUS="$D/$(ls "$D" | grep -E '^rv113_census-[0-9a-f]+$' | head -1)"
export TMPDIR=$S/tmp
cd "$RE" || exit 90
for id in "$@"; do
  out=$S/mutants/runs/$id
  if [ "$id" = NONE ]; then unset RV113_MUT; else export RV113_MUT=$id; fi
  RV113_PROBES=$probes RV113_PROBES_OUT=$out/probes.jsonl nice -n 10 "$CENSUS" rv113_probes --exact > "$out/probes_rerun.log" 2>&1
  echo "probes rerun rc=$? $(date -u '+%FT%TZ')" >> "$out/run.log"
done
