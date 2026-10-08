#!/bin/bash
# I92 repair 01: the whole desktop vitest suite and tsc --noEmit in one scratch copy, run through WT/tools/t3_slot.sh
# (one heavy job of mine at a time). Usage: suites.sh <label> <copy> ts|tsc
set -u
S2=S2
LABEL=$1; COPY=$2; WHAT=$3; D=$S2/$COPY/projects/chirality-piping/apps/desktop
mkdir -p $S2/tmp/$LABEL $S2/logs/$LABEL
echo "$WHAT start $(date -u +%FT%TZ)" >> $S2/logs/$LABEL/rc.txt
cd $D
if [ "$WHAT" = ts ]; then env TMPDIR=$S2/tmp/$LABEL ../../node_modules/.bin/vitest run --reporter=dot --reporter=json --outputFile.json=$S2/logs/$LABEL/vitest.json > $S2/logs/$LABEL/vitest.log 2>&1
else env TMPDIR=$S2/tmp/$LABEL ../../node_modules/.bin/tsc --noEmit -p tsconfig.json > $S2/logs/$LABEL/tsc.log 2>&1; fi
rc=$?; echo "$WHAT rc=$rc end $(date -u +%FT%TZ)" >> $S2/logs/$LABEL/rc.txt
rm -rf "${D:?}/node_modules/.vite"; rmdir "${D:?}/node_modules/.vite-temp" "${D:?}/node_modules" 2>/dev/null
exit $rc
