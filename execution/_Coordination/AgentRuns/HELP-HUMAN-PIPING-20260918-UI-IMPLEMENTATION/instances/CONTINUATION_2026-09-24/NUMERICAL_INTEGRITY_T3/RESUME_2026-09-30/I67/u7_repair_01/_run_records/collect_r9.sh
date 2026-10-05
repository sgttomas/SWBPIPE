#!/bin/zsh
# I67 U7 repair (u7_repair_01): collect records into NUM/R/I67/u7_repair_01/_run_records (placeholder paths only).
set -eu
T3=WT; REPO=REPO_ROOT; S=$T3/scratch/i67_u6d
NUM=$T3/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
R=$NUM/I67/u7_repair_01; O=$R/_run_records; mkdir -p $O/suites $O/oracle $O/mutants
san() { sed -e "s|$T3|WT|g" -e "s|$REPO|REPO_ROOT|g" -e "s|SYSTEM_TMP \"']*|SYSTEM_TMP|g" "$@"; }
export GIT_OPTIONAL_LOCKS=0
W=$T3/f2a-u7; cd $W
git rev-parse HEAD > $O/basis.txt
git diff -- projects/chirality-piping | san > $O/candidate.diff
shasum -a 256 $(git diff --name-only -- projects/chirality-piping) | san > $O/changed_files_sha256.txt
git diff --numstat -- projects/chirality-piping > $O/changed_files_numstat.txt
git status --short --untracked-files=all -- projects/chirality-piping 2>/dev/null | san > $O/worktree_status.txt || true
{ echo "lanes vs the worktree (the 9 changed files):"; for L in cand9 mut9 pyrs9; do for f in $(cat $S/r9/changed.txt); do [ -f $S/lanes/$L/$f ] || continue; cmp -s $S/lanes/$L/$f $W/$f && echo "$L same $f" || echo "$L DIFF $f"; done; done; } > $O/lanes_vs_worktree.txt
for run in base cand; do d=$S/r9/$run
  san $d/exit.txt > $O/suites/vitest_${run}_exit.txt; san $d/node_version.txt > $O/suites/vitest_${run}_versions.txt; san $d/tsc.out > $O/suites/tsc_${run}.out
  python3 -c "
import json
d=json.load(open('$d/vitest.json'))
rows=sorted((f['name'].split('/src/',1)[1], a['fullName'], a['status']) for f in d['testResults'] for a in f['assertionResults'])
print(json.dumps({k:d[k] for k in d if k.startswith('num') or k=='success'}))
for r in rows: print('\t'.join(r))
" | san > $O/suites/vitest_${run}_outcomes.tsv; done
python3 $S/records/compare_outcomes.py $S/r9/base/vitest.json $S/r9/cand/vitest.json | san > $O/suites/vitest_compare.txt
san $S/records/compare_outcomes.py > $O/suites/compare_outcomes.py
for l in py_base py_cand py_cand2; do grep -E "^(PASSED|FAILED|ERROR) " $S/r9/logs/$l.log | sort | san > $O/suites/$l.outcomes; tail -1 $S/r9/logs/$l.log | san > $O/suites/$l.summary.txt; done
{ echo "Python retained suites, base vs candidate (outcome lines only in one side):"; diff $O/suites/py_base.outcomes $O/suites/py_cand2.outcomes || true; } > $O/suites/py_compare.txt
for l in re_base re_cand re_cand2; do san $S/r9/logs/$l.outcomes > $O/suites/$l.outcomes; done
{ echo "result_export, base vs candidate:"; diff $O/suites/re_base.outcomes $O/suites/re_cand2.outcomes || true; echo "base: $(grep -c ' ok$' $S/r9/logs/re_base.outcomes) ok, $(grep -c 'FAILED$' $S/r9/logs/re_base.outcomes) failed; cand (final): $(grep -c ' ok$' $S/r9/logs/re_cand2.outcomes) ok, $(grep -c 'FAILED$' $S/r9/logs/re_cand2.outcomes) failed"; } > $O/suites/re_compare.txt
for f in run_suite.sh collect_r9.sh; do san $S/$f > $O/$f; done
for f in run_py.sh cargo_run.sh chain_base.sh chain_cand.sh chain_cand2.sh; do san $S/r9/$f > $O/suites/$f; done
for f in run_dump9.sh compare_repair_ts.py ts_oracle_diff_repair.txt ts_base9.json ts_cand9.json ts_base9.json.log ts_cand9.json.log; do san $S/oracle9/$f > $O/oracle/$f; done
san $S/oracle7/zzI67U7fOracle.test.ts > $O/oracle/zzI67U7fOracle.test.ts
{ echo "inputs regenerated with I66's gen_inputs.py (not copied, about 50 MB each):"; shasum -a 256 $S/oracle9/inputs_base9.json $S/oracle9/inputs_cand9.json; echo "base lane = 8c84e7ae14: 381 inputs, equal to I66 u7_repair_01's inputs_sha256 (104fb714...); candidate: 386 = those 381 unchanged plus 5 new"; } | san > $O/oracle/inputs_sha256.txt
for f in mutants_r9.py mutants_r9.json mutants_r9.log pyrs_mutants_r9.py pyrs_mutants_r9.json pyrs_mutants_r9.log; do [ -f $S/mut/$f ] && san $S/mut/$f > $O/mutants/$f; done
{ echo "worktree: WT/f2a-u7 at $(cut -c1-10 $O/basis.txt) (codex/piping-f2a-u7-20261004), uncommitted"
  echo "lanes (WT/scratch/i67_u6d/lanes): base9 = git archive 8c84e7ae14 of projects/chirality-piping; cand9 = base9 without execution/ (a symlink to base9's) plus the 9 changed files; mut9 = cand9 (TS mutants); pyrs9 = cand9 without apps/ and execution/ (Python/Rust mutants)"
  echo "node_modules: the existing untracked symlink in the worktree; lanes link the same target; no install"
  echo "WASM: no build; base9 carries byte copies of the worktree's ignored prebuilt public/ files (hashes below)"
  (cd $S/lanes/base9/projects/chirality-piping/apps/desktop && shasum -a 256 public/wasm-engine/* public/self-weight-engine/*)
  echo "Python: REPO_ROOT/projects/chirality-piping/.venv with the I52 CLIs; PYTHONDONTWRITEBYTECODE=1; -p no:cacheprovider"
  echo "Cargo: default toolchain, --locked --offline, CARGO_BUILD_JOBS=4, RUST_TEST_THREADS=2, one job at a time; target dirs WT/targets/i67-u7f/{re-base9,re-cand9,re-mut9}"
  echo "TMPDIR=WT/scratch/i67_u6d/tmp"; cat $S/r9/cand/node_version.txt; } | san > $O/runtime.txt
grep -rlE "/Us[e]rs/|/priv[a]te/" $O && { echo LEAK; exit 1; } || echo collected
