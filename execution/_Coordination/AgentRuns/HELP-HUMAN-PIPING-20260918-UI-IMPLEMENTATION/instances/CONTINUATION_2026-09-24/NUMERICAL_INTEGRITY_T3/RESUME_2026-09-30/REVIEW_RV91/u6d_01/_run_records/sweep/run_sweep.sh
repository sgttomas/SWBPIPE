#!/bin/zsh
# RV91: run the sweep in one lane. Usage: run_sweep.sh <lane>
set -u
T3=WT
S=$T3/scratch/rv91_u6d
export TMPDIR=$S/tmp
L=$1
D=$T3/rv91/$L/projects/chirality-piping/apps/desktop
cp $S/sweep/zzRV91Sweep.test.tsx $D/src/features/results/
cd $D
RV91_INPUT_ROOT=$T3/rv91/cand/projects/chirality-piping RV91_SWEEP_OUT=$S/sweep/sweep_$L.jsonl ../../node_modules/.bin/vitest run src/features/results/zzRV91Sweep.test.tsx > $S/sweep/sweep_$L.log 2>&1
echo "exit=$?" >> $S/sweep/sweep_$L.log
rm -f $D/src/features/results/zzRV91Sweep.test.tsx
