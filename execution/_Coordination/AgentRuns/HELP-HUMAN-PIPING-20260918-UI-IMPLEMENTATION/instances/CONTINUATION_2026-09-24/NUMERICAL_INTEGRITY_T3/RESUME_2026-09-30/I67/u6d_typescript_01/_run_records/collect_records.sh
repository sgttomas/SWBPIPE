#!/bin/zsh
# I67 U6d: collect the run records into NUM/R/I67/u6d_typescript_01/_run_records with
# placeholder paths only (WT, REPO_ROOT). Reads scratch and the worktree; Git reads use
# GIT_OPTIONAL_LOCKS=0 and never write.
set -eu
T3=WT
REPO=REPO_ROOT
S=$T3/scratch/i67_u6d
R=$T3/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I67/u6d_typescript_01
O=$R/_run_records
mkdir -p $O
san() { sed -e "s|$T3|WT|g" -e "s|$REPO|REPO_ROOT|g" -e "s|SYSTEM_TMP \"']*|SYSTEM_TMP|g" "$@"; }
export GIT_OPTIONAL_LOCKS=0
cd $T3/f2a-carriers-ts
P=projects/chirality-piping/apps/desktop/src
NEW=($P/features/results/retainedPrecisionStanding.ts $P/features/results/retainedPrecisionIntegration.test.tsx $P/services/retainedPrecisionAnalysisRun.test.ts $P/features/results/retainedPrecisionOutputRefusal.test.tsx)
TRACKED=($(git diff --name-only -- projects/chirality-piping))
{ git diff -- projects/chirality-piping; for f in $NEW; do git diff --no-index -- /dev/null $f || true; done; } | san > $O/candidate.diff
shasum -a 256 $TRACKED $NEW | san > $O/changed_files_sha256.txt
git status --short --untracked-files=all -- projects/chirality-piping 2>/dev/null | san > $O/worktree_status.txt || true
git rev-parse HEAD > $O/basis.txt
for f in run_suite.sh pipeline.sh collect_records.sh; do san $S/$f > $O/$f; done
san $S/sweep/zzI67Sweep.test.ts > $O/zzI67Sweep.test.ts
for f in sweep_base.tsv sweep_cand.tsv sweep_base.log sweep_cand.log; do san $S/sweep/$f > $O/$f; done
gzip -9 -c $S/sweep/sweep_base.jsonl > $O/sweep_base.jsonl.gz; gzip -9 -c $S/sweep/sweep_cand.jsonl > $O/sweep_cand.jsonl.gz
python3 $S/records/compare_sweep.py $S/sweep/sweep_base $S/sweep/sweep_cand | san > $O/sweep_compare.txt
for f in compare_outcomes.py compare_sweep.py collision_check.py COLLISIONS.json; do san $S/records/$f > $O/$f; done
san $S/proposed/knownSemanticLimitations.test.ts.patch > $O/proposed_knownSemanticLimitations.test.ts.patch
san $S/mut/mutants.py > $O/mutants.py
for f in $S/mut/mutants_round*.json $S/mut/mutants_round*.log $S/mut/mutants_final.json $S/mut/mutants_final.log; do san $f > $O/$(basename $f); done
for run in base/run1 cand/final cand/lane_patched_final; do
  d=$S/$run; n=$(echo $run | tr / _)
  [ -d $d ] || continue
  san $d/exit.txt > $O/${n}_exit.txt; san $d/node_version.txt > $O/${n}_versions.txt; san $d/tsc.out > $O/${n}_tsc.out
  python3 -c "
import json,sys
d=json.load(open('$d/vitest.json'))
rows=sorted((f['name'].split('/src/',1)[1], a['fullName'], a['status']) for f in d['testResults'] for a in f['assertionResults'])
print(json.dumps({k:d[k] for k in d if k.startswith('num') or k=='success'}))
for r in rows: print('\t'.join(r))
" > $O/${n}_outcomes.tsv
done
[ -d $S/cand/final ] && python3 $S/records/compare_outcomes.py $S/base/run1/vitest.json $S/cand/final/vitest.json > $O/compare_base_vs_worktree.txt
[ -d $S/cand/lane_patched_final ] && python3 $S/records/compare_outcomes.py $S/base/run1/vitest.json $S/cand/lane_patched_final/vitest.json > $O/compare_base_vs_patched_lane.txt
{ echo "node_modules: symlink WT/f2a-carriers-ts/projects/chirality-piping/node_modules -> $(readlink $T3/f2a-carriers-ts/projects/chirality-piping/node_modules)"; echo "desktop node_modules: none at REPO_ROOT (none linked); vitest wrote its cache under the ignored apps/desktop/node_modules/.vite"; echo "WASM: copied from WT/f2a-readers/projects/chirality-piping/apps/desktop/public/{wasm-engine,self-weight-engine} (never built)"; (cd $T3/f2a-carriers-ts/projects/chirality-piping/apps/desktop && shasum -a 256 public/*/*); echo "TMPDIR=WT/scratch/i67_u6d/tmp"; } | san > $O/runtime.txt
grep -rlE "/Us[e]rs/|/priv[a]te/" $O && { echo "MACHINE PATH LEAK"; exit 1; } || true
echo collected
