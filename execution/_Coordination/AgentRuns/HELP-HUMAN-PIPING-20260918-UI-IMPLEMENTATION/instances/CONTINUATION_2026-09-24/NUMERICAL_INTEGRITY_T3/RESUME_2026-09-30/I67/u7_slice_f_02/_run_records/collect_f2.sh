#!/bin/zsh
# I67 U7 slice F follow-on: collect records into NUM/R/I67/u7_slice_f_02/_run_records (placeholder paths only).
set -eu
T3=WT; REPO=REPO_ROOT; S=$T3/scratch/i67_u6d
NUM=$T3/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
R=$NUM/I67/u7_slice_f_02; O=$R/_run_records; mkdir -p $O
san() { sed -e "s|$T3|WT|g" -e "s|$REPO|REPO_ROOT|g" -e "s|SYSTEM_TMP \"']*|SYSTEM_TMP|g" "$@"; }
export GIT_OPTIONAL_LOCKS=0
W=$T3/f2a-u7; cd $W
git rev-parse HEAD > $O/basis.txt
git diff -- projects/chirality-piping | san > $O/candidate.diff
shasum -a 256 $(git diff --name-only -- projects/chirality-piping) | san > $O/changed_files_sha256.txt
git status --short --untracked-files=all -- projects/chirality-piping 2>/dev/null | san > $O/worktree_status.txt || true
san $S/r8/lane_cand7_vs_cfda60403f.txt > $O/lane_base_cand7_vs_cfda60403f.txt
for run in r7/final r8/final; do d=$S/$run; t=${run//\//_}
  san $d/exit.txt > $O/${t}_exit.txt; san $d/node_version.txt > $O/${t}_versions.txt; san $d/tsc.out > $O/${t}_tsc.out
  python3 -c "
import json
d=json.load(open('$d/vitest.json'))
rows=sorted((f['name'].split('/src/',1)[1], a['fullName'], a['status']) for f in d['testResults'] for a in f['assertionResults'])
print(json.dumps({k:d[k] for k in d if k.startswith('num') or k=='success'}))
for r in rows: print('\t'.join(r))
" | san > $O/${t}_outcomes.tsv; done
python3 $S/records/compare_outcomes.py $S/r7/final/vitest.json $S/r8/final/vitest.json | san > $O/compare_cfda60403f_vs_followon.txt
san $S/records/compare_outcomes.py > $O/compare_outcomes.py
for f in run_suite.sh r8_final.sh collect_f2.sh; do san $S/$f > $O/$f; done
for f in mutants_f2.py mutants_f2.json mutants_f2.log; do san $S/mut/$f > $O/$f; done
{ echo "mutant lane WT/scratch/i67_u6d/lanes/mut7 vs the worktree:"; for f in $(cat $S/r8/changed.txt); do cmp -s $S/lanes/mut7/$f $W/$f && echo "same $f" || echo "DIFF $f"; done; } > $O/lane_mut7_vs_worktree.txt
for f in ts_cand8.json ts_cand8.json.log oracle_dump_vs_f01.txt compare_ts_oracle_f02.py ts_oracle_diff_f02.txt; do san $S/r8/$f > $O/$f; done
{ echo "worktree: WT/f2a-u7 at $(cut -c1-10 $O/basis.txt) (codex/piping-f2a-u7-20261004), uncommitted follow-on"
  echo "lanes: cand7 (its 18 slice F files hash-equal to cfda60403f, lane_base_cand7_vs_cfda60403f.txt) is the base; cand8 = cand7 + the 2 follow-on files; mut7 = cand8"
  echo "no install, no build, no Cargo or Python; node_modules is the existing untracked symlink; WASM: the lanes' byte copies of the prebuilt ignored public/ files (as u7_slice_f_01/runtime.txt)"
  echo "TMPDIR=WT/scratch/i67_u6d/tmp"; cat $S/r8/final/node_version.txt; } | san > $O/runtime.txt
grep -rlE "/Us[e]rs/|/priv[a]te/" $O && { echo LEAK; exit 1; } || echo collected
