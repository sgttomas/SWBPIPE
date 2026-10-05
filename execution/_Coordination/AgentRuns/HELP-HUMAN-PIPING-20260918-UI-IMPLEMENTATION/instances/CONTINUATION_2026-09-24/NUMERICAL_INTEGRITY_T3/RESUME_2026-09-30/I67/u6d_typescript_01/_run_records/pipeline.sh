#!/bin/zsh
# I67 U6d: lane controls in sequence (scratch only).
set -u
T3=WT; S=$T3/scratch/i67_u6d
export TMPDIR=$S/tmp
# 1. Full suite and tsc in the candidate lane (candidate + the proposed out-of-fence pin patch).
$S/run_suite.sh $S/lanes/cand/projects/chirality-piping/apps/desktop $S/cand/lane_patched
# 2. Existing-identity sweep, base then candidate.
for L in base cand; do
  D=$S/lanes/$L/projects/chirality-piping/apps/desktop
  cp $S/sweep/zzI67Sweep.test.ts $D/src/features/results/
  (cd $D && I67_SWEEP_OUT=$S/sweep/sweep_$L ../../node_modules/.bin/vitest run src/features/results/zzI67Sweep.test.ts > $S/sweep/sweep_$L.log 2>&1; echo "exit=$?" >> $S/sweep/sweep_$L.log)
  rm -f $D/src/features/results/zzI67Sweep.test.ts
done
echo pipeline_done > $S/pipeline.done
