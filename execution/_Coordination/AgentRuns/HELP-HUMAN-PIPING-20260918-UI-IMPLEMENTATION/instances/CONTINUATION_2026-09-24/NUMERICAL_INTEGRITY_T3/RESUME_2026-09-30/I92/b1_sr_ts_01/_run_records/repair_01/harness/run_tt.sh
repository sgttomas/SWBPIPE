#!/bin/bash
# I92 repair 01: one vitest run of the TS test file (optionally filtered) in a scratch copy. Usage: run_tt.sh <label> <copy> [-t pattern]
set -u
S2=S2
LABEL=$1; COPY=$2; shift 2; D=$S2/$COPY/projects/chirality-piping/apps/desktop
mkdir -p $S2/tmp/$LABEL
cd $D && env TMPDIR=$S2/tmp/$LABEL ../../node_modules/.bin/vitest run src/features/results/retainedPrecision.test.ts "$@" > $S2/logs/tt_$LABEL.log 2>&1
rc=$?; rm -rf "${D:?}/node_modules/.vite"; rmdir "${D:?}/node_modules/.vite-temp" "${D:?}/node_modules" 2>/dev/null
echo "tt $LABEL rc=$rc $(date -u +%FT%TZ)" >> $S2/logs/jobs.txt; exit $rc
