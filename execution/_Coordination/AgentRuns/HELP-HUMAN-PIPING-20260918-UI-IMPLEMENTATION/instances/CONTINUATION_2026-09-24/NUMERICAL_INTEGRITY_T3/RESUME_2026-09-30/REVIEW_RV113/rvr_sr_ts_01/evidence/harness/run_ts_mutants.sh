#!/bin/bash
# RV113: the TS mutant schema's runs, one locked vitest job per mutant (RV113_MUT=<id>; NONE = the control).
# Usage: run_ts_mutants.sh <probes json> <ids...>. Each run: the seven test files that import the TS reader, plus the
# reviewer's probe harness; vitest's JSON report per mutant.
WT=WT
S=$WT/scratch/rv113_rvr_01
D=$WT/rv113/ts-mut/projects/chirality-piping/apps/desktop
probes=$1; shift
FILES="src/features/results/retainedPrecision.test.ts src/features/results/retainedPrecisionIntegration.test.tsx src/features/results/retainedPrecisionOutputRefusal.test.tsx src/features/results/outputPolicy.test.ts src/features/result-export/retainedPrecisionResultExport.test.tsx src/features/stress-neutral/retainedPrecisionStressNeutral.test.tsx src/services/retainedPrecisionAnalysisRun.test.ts src/features/results/rv113Census.test.ts"
for id in "$@"; do
  out=$S/ts/mutants/runs/$id; mkdir -p "$out"
  if [ "$id" = NONE ]; then unset RV113_MUT; else export RV113_MUT=$id; fi
  RV113_PROBES=$probes RV113_PROBES_OUT=$out/probes.jsonl \
    $S/tools/run_ts_job.sh mut_$id "$D" vitest run $FILES --reporter=dot --reporter=json --outputFile.json=$out/vitest.json
  echo "$id rc=$? $(date -u '+%FT%TZ')" >> $S/ts/mutants/batch.log
  mv $S/ts/logs/mut_$id.log $out/run.log
done
