#!/bin/zsh
# I67 U6d round 04: collect records into NUM/R/I67/u6d_typescript_04/_run_records (placeholder paths only).
set -eu
T3=WT; REPO=REPO_ROOT; S=$T3/scratch/i67_u6d
R=$T3/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I67/u6d_typescript_04; O=$R/_run_records; mkdir -p $O
san() { sed -e "s|$T3|WT|g" -e "s|$REPO|REPO_ROOT|g" -e "s|SYSTEM_TMP \"']*|SYSTEM_TMP|g" "$@"; }
export GIT_OPTIONAL_LOCKS=0
cd $T3/f2a-carriers
git rev-parse HEAD > $O/basis.txt
git diff -- projects/chirality-piping | san > $O/candidate.diff
shasum -a 256 $(git diff --name-only -- projects/chirality-piping) | san > $O/changed_files_sha256.txt
git status --short --untracked-files=all -- projects/chirality-piping 2>/dev/null | san > $O/worktree_status.txt || true
{ shasum -a 256 projects/chirality-piping/fixtures/results/retained_precision_carrier_cases.json; git diff --quiet -- projects/chirality-piping/fixtures && echo "fixtures: no change (git diff --quiet)"; } > $O/case_file_sha256_unchanged.txt
for f in r4_pipeline.sh run_suite.sh collect_r4.sh; do san $S/$f > $O/$f; done
for f in mutants_r4.py mutants_r4.json mutants_r4.log; do san $S/mut/$f > $O/$f; done
for run in base cand; do d=$S/r4/$run; san $d/exit.txt > $O/${run}_exit.txt; san $d/node_version.txt > $O/${run}_versions.txt; san $d/tsc.out > $O/${run}_tsc.out
python3 -c "
import json
d=json.load(open('$d/vitest.json'))
rows=sorted((f['name'].split('/src/',1)[1], a['fullName'], a['status']) for f in d['testResults'] for a in f['assertionResults'])
print(json.dumps({k:d[k] for k in d if k.startswith('num') or k=='success'}))
print(json.dumps({'failed_suites': [[f['name'].split('/src/',1)[1], (f.get('message') or '')[:200]] for f in d['testResults'] if f['status'] != 'passed']}))
for r in rows: print('\t'.join(r))
" | san > $O/${run}_outcomes.tsv; done
san $S/records/compare_outcomes_r4.py > $O/compare_outcomes_r4.py
python3 $S/records/compare_outcomes_r4.py $S/r4/base/vitest.json $S/r3/cand/vitest.json $S/r4/cand/vitest.json | san > $O/compare_outcomes.txt
san $S/r4/py.log > $O/python_shared_consumers.log
san $S/r4/lane_mut4_vs_worktree.txt > $O/lane_mut4_vs_worktree.txt
{ echo "worktree: WT/f2a-carriers at 6383e8e70e (codex/piping-f2a-carriers-20261004; I66's post-U6f round committed by ROOT), uncommitted"; echo "node_modules: the existing symlink P/node_modules -> REPO_ROOT/projects/chirality-piping/node_modules; no install"; echo "WASM: the existing copies in apps/desktop/public; nothing built"; echo "base lane: WT/scratch/i67_u6d/lanes/base4 = git archive 6383e8e70e; mutant lane: lanes/mut4 (the candidate)"; echo "integration-file comparison: round 03's candidate run (its file equals 76477534f6's, sha256 3437bceb...)"; echo "Python: REPO_ROOT/projects/chirality-piping/.venv with the I52 checked-JSON and units CLIs (WT/targets/i52-readers), PYTHONDONTWRITEBYTECODE=1, -p no:cacheprovider"; echo "TMPDIR=WT/scratch/i67_u6d/tmp"; cat $S/r4/cand/node_version.txt; } | san > $O/runtime.txt
python3 -c "
import json,gzip
d=json.load(open('$S/r3/cand/vitest.json'))
rows=sorted((f['name'].split('/src/',1)[1], a['fullName'], a['status']) for f in d['testResults'] for a in f['assertionResults'] if 'retainedPrecisionIntegration' in f['name'])
for r in rows: print('\t'.join(r))
" > $O/round03_integration_outcomes.tsv
grep -rlE "/Us[e]rs/|/priv[a]te/" $O && { echo LEAK; exit 1; } || echo collected
