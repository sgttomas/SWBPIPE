#!/bin/bash
# I92 repair 01: RV113's TS harness (census over 07m and the 135 probes) in one copy, as one vitest job.
# Usage (through WT/tools/t3_slot.sh): run_census.sh <label> <copy>
set -u
S2=S2
LABEL=$1; COPY=$2; D=$S2/$COPY/projects/chirality-piping/apps/desktop
cp $S2/harness/rv113Census.test.ts $D/src/features/results/rv113Census.test.ts
mkdir -p $S2/tmp/$LABEL
cd $D && env TMPDIR=$S2/tmp/$LABEL RV113_OUT=$S2/logs/census_$LABEL.jsonl RV113_PROBES=$S2/harness/probes_all.json RV113_PROBES_OUT=$S2/logs/probes_$LABEL.jsonl \
  ../../node_modules/.bin/vitest run src/features/results/rv113Census.test.ts > $S2/logs/census_$LABEL.log 2>&1
rc=$?
rm -f $D/src/features/results/rv113Census.test.ts; rm -rf "${D:?}/node_modules/.vite"; rmdir "${D:?}/node_modules/.vite-temp" "${D:?}/node_modules" 2>/dev/null
echo "census $LABEL rc=$rc $(date -u +%FT%TZ)" >> $S2/logs/jobs.txt
exit $rc
