#!/bin/zsh
# RV91: run the reviewer probes in one lane. Usage: run_review.sh <lane> <tag>
set -u
T3=WT
S=$T3/scratch/rv91_u6d
export TMPDIR=$S/tmp
L=$1; TAG=$2
D=$T3/rv91/$L/projects/chirality-piping/apps/desktop
cp $S/review/zzRV91Review.test.tsx $D/src/features/results/
cd $D
RV91_REVIEW_OUT=$S/review/review_$TAG.json RV91_EXTRA_CASES=$S/parity/extra_cases.json ../../node_modules/.bin/vitest run --reporter=verbose src/features/results/zzRV91Review.test.tsx > $S/review/review_$TAG.log 2>&1
echo "exit=$?" >> $S/review/review_$TAG.log
rm -f $D/src/features/results/zzRV91Review.test.tsx
