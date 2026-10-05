#!/bin/zsh
# RV95: vitest (JSON reporter) + tsc in a desktop dir. Usage: run_ts.sh <desktop_dir> <out_dir> [vitest args...]
set -u
DESK=$1; OUT=$2; shift 2
mkdir -p $OUT
export TMPDIR=WT/scratch/rv95_u9_01/tmp
ps -p 5387 >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
cd $DESK
start=$(date +%s)
../../node_modules/.bin/vitest run --maxWorkers=4 --reporter=json --outputFile=$OUT/vitest.json "$@" > $OUT/vitest.stdout 2> $OUT/vitest.stderr
echo "vitest_exit=$?" > $OUT/exit.txt
../../node_modules/.bin/tsc -p tsconfig.json --noEmit > $OUT/tsc.out 2>&1
echo "tsc_exit=$?" >> $OUT/exit.txt
echo "seconds=$(( $(date +%s) - start ))" >> $OUT/exit.txt
