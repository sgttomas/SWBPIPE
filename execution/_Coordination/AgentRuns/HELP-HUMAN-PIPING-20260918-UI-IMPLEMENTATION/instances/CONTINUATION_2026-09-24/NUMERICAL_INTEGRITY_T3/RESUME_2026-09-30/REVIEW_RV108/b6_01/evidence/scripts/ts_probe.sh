#!/bin/bash
# RV108: run the TS probe harness in a probe copy (pbase|pcand) over a probe file.
source S/env.sh
WHICH=$1; IN=$2; OUT=$3; LOG=$4
cd $WT/rv108/$WHICH/$P/apps/desktop && env TMPDIR=$S/tmp RV108_IN=$IN RV108_OUT=$OUT NODE_OPTIONS=--max-old-space-size=8192 ../../node_modules/.bin/vitest run src/features/results/zzRv108Probe.test.ts > $LOG 2>&1
echo "rc=$?" >> $LOG
