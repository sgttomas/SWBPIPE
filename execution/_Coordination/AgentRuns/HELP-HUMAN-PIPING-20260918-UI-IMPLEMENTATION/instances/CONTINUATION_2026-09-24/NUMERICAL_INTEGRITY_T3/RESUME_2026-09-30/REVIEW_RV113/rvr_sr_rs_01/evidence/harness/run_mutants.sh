#!/bin/bash
# RV113: run the mutant schema's already-built test binaries (no cargo) once per mutant.
# Usage: run_mutants.sh <probes json> <ids...>   ("NONE" = the control: RV113_MUT unset)
# Per mutant: RE's lib unit tests, the contract test, the public source_blocks test (N-5 ids),
# the reviewer probes and the 07m census. Light runs at nice 10, RUST_TEST_THREADS=2.
WT=WT
S=$WT/scratch/rv113_rvr_01
D=$WT/targets/rv113-mut/debug/deps
RE=$WT/rv113/mut/projects/chirality-piping/core/reporting/result_export
probes=$1; shift
bin() { echo "$D/$(ls "$D" | grep -E "^$1-[0-9a-f]+\$" | head -1)"; }
LIB=$(bin open_pipe_stress_result_export); CONTRACT=$(bin retained_precision_contract)
SB=$(bin source_blocks); CENSUS=$(bin rv113_census)
export TMPDIR=$S/tmp RUST_TEST_THREADS=2
cd "$RE" || exit 90
mkdir -p "$S/mutants/runs"
for id in "$@"; do
  out=$S/mutants/runs/$id; mkdir -p "$out"
  if [ "$id" = NONE ]; then unset RV113_MUT; else export RV113_MUT=$id; fi
  echo "# $id start $(date -u '+%FT%TZ')" > "$out/run.log"
  nice -n 10 "$LIB" > "$out/lib.log" 2>&1; echo "lib rc=$?" >> "$out/run.log"
  nice -n 10 "$CONTRACT" > "$out/contract.log" 2>&1; echo "contract rc=$?" >> "$out/run.log"
  case "$id" in M3*|NONE) nice -n 10 "$SB" > "$out/source_blocks.log" 2>&1; echo "source_blocks rc=$?" >> "$out/run.log";; esac
  RV113_PROBES=$probes RV113_PROBES_OUT=$out/probes.jsonl RV113_OUT=$out/census.jsonl \
    nice -n 10 "$CENSUS" --test-threads 2 > "$out/harness.log" 2>&1; echo "harness rc=$?" >> "$out/run.log"
  echo "# $id end $(date -u '+%FT%TZ')" >> "$out/run.log"
done
