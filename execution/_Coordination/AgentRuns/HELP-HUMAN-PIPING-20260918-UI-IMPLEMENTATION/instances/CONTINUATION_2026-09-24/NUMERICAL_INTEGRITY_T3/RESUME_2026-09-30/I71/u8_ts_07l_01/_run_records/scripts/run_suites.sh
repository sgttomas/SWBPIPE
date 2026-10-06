#!/bin/bash
# I71: the desktop vitest suite, base (d449097085 archive copy) then candidate (WT/f2a-u8), one at a time.
S="$1"; CAND="$2"
for side in base candidate; do
  if [ "$side" = base ]; then D="$S/base/projects/chirality-piping/apps/desktop"; else D="$CAND/projects/chirality-piping/apps/desktop"; fi
  cd "$D" || exit 9
  start=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  TMPDIR="$S/tmp" ../../node_modules/.bin/vitest run --reporter=default --reporter=json --outputFile.json="$S/logs/suite_$side.json" > "$S/logs/suite_$side.log" 2>&1
  rc=$?
  echo "side=$side start=$start end=$(date -u +%Y-%m-%dT%H:%M:%SZ) rc=$rc" >> "$S/logs/suite_runs.txt"
done
