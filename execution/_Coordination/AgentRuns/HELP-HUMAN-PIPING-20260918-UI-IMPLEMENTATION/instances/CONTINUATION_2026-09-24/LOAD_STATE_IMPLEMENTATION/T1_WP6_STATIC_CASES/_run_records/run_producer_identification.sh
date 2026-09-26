#!/bin/bash
# Identification-only producer runs (T1_WP6_STATIC_CASES). Outputs go to a
# scratch directory and are never committed; no value is read by the package.
# Usage: run_producer_identification.sh <physics_source_connected example binary> <scratch raw dir>
# Run from WORKING_ROOT (projects/chirality-piping).
set -u
EX="$1"; OUT="$2"; mkdir -p "$OUT"
for f in validation/qualification/fixtures/load_reference/*.preview_request.json; do
  c=$(basename "$f" .preview_request.json)
  for m in sparse dense; do
    "$EX" "$m" "$f" > "$OUT/$c.$m.json" 2> "$OUT/$c.$m.err"; echo "$c $m exit=$?"
  done
done
