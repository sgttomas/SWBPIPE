#!/bin/zsh
# I75 REPAIR_01: the tie probe and mutants in the probe copy, then tsc, the T6S test files and the full desktop suite in the worktree.
# Runs whole under lockf -k WT/guard/cargo_job.lock (the caller takes the lock).
S=$1; WTD=$2; OUT=$S/repair_out; mkdir -p $OUT
echo "probe start $(date -u +%FT%TZ)" > $OUT/times.txt
( cd $S/probe/projects/chirality-piping/apps/desktop && I75_TIES=$S/ties ../../node_modules/.bin/vitest run src/features/results/zz_i75_tie_probe.test.ts src/features/stress-neutral/retainedPrecisionStressNeutral.test.tsx > $OUT/probe.log 2>&1 ); echo "probe rc=$? $(date -u +%FT%TZ)" >> $OUT/times.txt
cp $S/ties/tie_probe_summary.json $OUT/tie_probe_summary.json
echo "mutants start $(date -u +%FT%TZ)" >> $OUT/times.txt
python3 $S/ties/mutants_repair_01.py $S/probe/projects/chirality-piping $OUT/mutants > $OUT/mutants.out 2>&1; echo "mutants rc=$? end $(date -u +%FT%TZ)" >> $OUT/times.txt
cd $WTD || exit 9
../../node_modules/.bin/tsc --noEmit -p . > $OUT/tsc.log 2>&1; echo "tsc rc=$? $(date -u +%FT%TZ)" >> $OUT/times.txt
../../node_modules/.bin/vitest run src/features/stress-neutral/retainedPrecisionStressNeutral.test.tsx src/features/results/outputPolicy.test.ts \
  src/features/results/retainedPrecisionOutputRefusal.test.tsx src/features/results/loadReferenceOutputRefusal.test.tsx \
  src/features/stress-neutral/StressNeutralExportPanel.test.tsx src/features/result-export/ResultExportPanel.test.tsx \
  src/features/result-export/retainedPrecisionResultExport.test.tsx src/features/result-export/resultExportAdapter.test.ts \
  src/features/result-export/physicsResultExport.test.ts src/features/results/retainedPrecisionIntegration.test.tsx > $OUT/t6s_files.log 2>&1
echo "t6s_files rc=$? $(date -u +%FT%TZ)" >> $OUT/times.txt
../../node_modules/.bin/vitest run --reporter=default --reporter=json --outputFile.json=$OUT/repair_vitest.json > $OUT/full_suite.log 2>&1
echo "full rc=$? $(date -u +%FT%TZ)" >> $OUT/times.txt
