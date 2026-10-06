#!/bin/bash
# usage: run_vitest_suite.sh <cand|base>
WT=WT
S=$WT/scratch/rv101_t6s_01
c=$1
export TMPDIR=$S/tmp
cd $WT/rv101/$c/projects/chirality-piping/apps/desktop || exit 2
../../node_modules/.bin/vitest run --maxWorkers=5 --reporter=dot --reporter=json --outputFile.json=$S/out/${c}_vitest.json > $S/logs/${c}_vitest.log 2>&1
echo "vitest rc=$?" >> $S/logs/${c}_vitest.log
../../node_modules/.bin/tsc --noEmit -p . > $S/logs/${c}_tsc.log 2>&1
echo "tsc rc=$?" >> $S/logs/${c}_tsc.log
