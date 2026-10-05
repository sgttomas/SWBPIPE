#!/bin/zsh
# I67 U6d repair round 02: the existing-identity sweep, base 9555b6ffc2 vs the repaired candidate.
set -u
T3=WT; S=$T3/scratch/i67_u6d
export TMPDIR=$S/tmp
for L in base2 cand2; do
  D=$S/lanes/$L/projects/chirality-piping/apps/desktop
  cp $S/sweep/zzI67Sweep.test.ts $D/src/features/results/
  (cd $D && I67_SWEEP_OUT=$S/r2/sweep_$L ../../node_modules/.bin/vitest run src/features/results/zzI67Sweep.test.ts > $S/r2/sweep_$L.log 2>&1; echo "exit=$?" >> $S/r2/sweep_$L.log)
  rm -f $D/src/features/results/zzI67Sweep.test.ts
done
echo done > $S/r2/sweep.done
