#!/bin/bash
# RV101 ADDENDUM_01: the ten T6S test files and tsc on fdcdb5e024, one job under the T3 lock.
export TMPDIR=WT/scratch/rv101_t6s_01/addendum/tmp
echo "$(date -u '+%FT%TZ') START RV101 addendum T6S files + tsc (locked)" >> WT/guard/cargo_jobs.log
cd WT/rv101/cand3/projects/chirality-piping/apps/desktop
../../node_modules/.bin/vitest run --maxWorkers=6 --reporter=default --reporter=json --outputFile.json=WT/scratch/rv101_t6s_01/addendum/logs/t6s_files.json src/features/stress-neutral/retainedPrecisionStressNeutral.test.tsx src/features/results/outputPolicy.test.ts src/features/results/retainedPrecisionOutputRefusal.test.tsx src/features/results/loadReferenceOutputRefusal.test.tsx src/features/stress-neutral/StressNeutralExportPanel.test.tsx src/features/result-export/ResultExportPanel.test.tsx src/features/result-export/retainedPrecisionResultExport.test.tsx src/features/result-export/resultExportAdapter.test.ts src/features/result-export/physicsResultExport.test.ts src/features/results/retainedPrecisionIntegration.test.tsx > WT/scratch/rv101_t6s_01/addendum/logs/t6s_files.log 2>&1
echo "vitest rc=$?" >> WT/scratch/rv101_t6s_01/addendum/logs/t6s_files.log
../../node_modules/.bin/tsc --noEmit -p . > WT/scratch/rv101_t6s_01/addendum/logs/tsc.log 2>&1
echo "tsc rc=$?" >> WT/scratch/rv101_t6s_01/addendum/logs/tsc.log
echo "$(date -u '+%FT%TZ') END RV101 addendum T6S files + tsc (locked)" >> WT/guard/cargo_jobs.log
