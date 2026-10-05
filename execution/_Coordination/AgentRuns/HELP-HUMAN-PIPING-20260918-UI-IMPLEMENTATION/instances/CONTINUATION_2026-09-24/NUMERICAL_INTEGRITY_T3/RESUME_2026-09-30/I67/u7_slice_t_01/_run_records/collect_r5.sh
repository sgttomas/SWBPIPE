#!/bin/zsh
# I67 U7 slice T: collect records into NUM/R/I67/u7_slice_t_01/_run_records (placeholder paths only).
set -eu
T3=WT; REPO=REPO_ROOT; S=$T3/scratch/i67_u6d
R=$T3/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I67/u7_slice_t_01; O=$R/_run_records; mkdir -p $O
san() { sed -e "s|$T3|WT|g" -e "s|$REPO|REPO_ROOT|g" -e "s|SYSTEM_TMP \"']*|SYSTEM_TMP|g" "$@"; }
export GIT_OPTIONAL_LOCKS=0
cd $T3/f2a-u7
git rev-parse HEAD > $O/basis.txt
git diff -- projects/chirality-piping | san > $O/candidate.diff
shasum -a 256 $(git diff --name-only -- projects/chirality-piping) | san > $O/changed_files_sha256.txt
git status --short --untracked-files=all -- projects/chirality-piping 2>/dev/null | san > $O/worktree_status.txt || true
{ git diff --quiet -- projects/chirality-piping/fixtures projects/chirality-piping/core projects/chirality-piping/tests projects/chirality-piping/schemas && echo "fixtures, core, tests, schemas: no change (git diff --quiet)"; git diff --quiet -- projects/chirality-piping/apps/desktop/src/features/results/retainedPrecision.ts && echo "TS reader retainedPrecision.ts (the flag): no change"; } > $O/out_of_fence_unchanged.txt
cp $S/r5/declared_difference_D-U7-4.draft.json $O/declared_difference_D-U7-4.draft.json
for f in r5_pipeline.sh run_suite.sh collect_r5.sh; do san $S/$f > $O/$f; done
for f in mutants_r5.py mutants_r5.json mutants_r5.log; do san $S/mut/$f > $O/$f; done
for run in base cand cand_final; do d=$S/r5/$run; [ -d $d ] || continue; san $d/exit.txt > $O/${run}_exit.txt; san $d/node_version.txt > $O/${run}_versions.txt; san $d/tsc.out > $O/${run}_tsc.out
python3 -c "
import json
d=json.load(open('$d/vitest.json'))
rows=sorted((f['name'].split('/src/',1)[1], a['fullName'], a['status']) for f in d['testResults'] for a in f['assertionResults'])
print(json.dumps({k:d[k] for k in d if k.startswith('num') or k=='success'}))
for r in rows: print('\t'.join(r))
" | san > $O/${run}_outcomes.tsv; done
san $S/records/compare_outcomes.py > $O/compare_outcomes.py
python3 $S/records/compare_outcomes.py $S/r5/base/vitest.json $S/r5/cand_final/vitest.json | san > $O/compare_base_vs_candidate.txt
san $S/r5/lane_mut5_vs_worktree.txt > $O/lane_mut5_vs_worktree.txt
for f in sweep_base5.tsv sweep_mut5.tsv sweep_base5.log sweep_mut5.log; do san $S/r5/$f > $O/$f; done
san $S/sweep/zzI67Sweep.test.ts > $O/zzI67Sweep.test.ts; san $S/records/compare_sweep.py > $O/compare_sweep.py
python3 $S/records/compare_sweep.py $S/r5/sweep_base5 $S/r5/sweep_mut5 | san > $O/sweep_compare.txt
{ echo "worktree: WT/f2a-u7 at 071eec5c04 (codex/piping-f2a-u7-20261004), uncommitted"; echo "node_modules: an untracked symlink WT/f2a-u7/projects/chirality-piping/node_modules -> REPO_ROOT/projects/chirality-piping/node_modules (created here; never committed); no install"; echo "WASM: copied from WT/f2a-readers/projects/chirality-piping/apps/desktop/public/{wasm-engine,self-weight-engine} into the worktree's ignored public/ (hashes below); nothing built"; (cd $T3/f2a-u7/projects/chirality-piping/apps/desktop && shasum -a 256 public/*/*); echo "base lane: WT/scratch/i67_u6d/lanes/base5 = git archive 071eec5c04; mutant lane: lanes/mut5 (the candidate)"; echo "TMPDIR=WT/scratch/i67_u6d/tmp"; cat $S/r5/cand_final/node_version.txt; } | san > $O/runtime.txt
grep -rlE "/Us[e]rs/|/priv[a]te/" $O && { echo LEAK; exit 1; } || echo collected
