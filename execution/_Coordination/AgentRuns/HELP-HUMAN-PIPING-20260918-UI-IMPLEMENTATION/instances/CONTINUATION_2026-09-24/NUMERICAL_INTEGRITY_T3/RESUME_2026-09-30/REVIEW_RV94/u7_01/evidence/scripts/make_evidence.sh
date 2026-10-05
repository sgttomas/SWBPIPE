#!/bin/bash
# RV94: copy sanitized evidence into the review folder. Placeholder paths only.
set -eu
WT=WT
REPO=REPO_ROOT
S=$WT/scratch/rv94_u7_01
OUT=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/REVIEW_RV94/u7_01
E=$OUT/evidence
mkdir -p $E/scripts $E/dumps $E/compare $E/mutants $E/sweep $E/u5 $E/suites $E/impl_mutants
san() { sed -e "s#$WT#WT#g" -e "s#$REPO#REPO_ROOT#g" -e "s#SYSTEM_TMP_REDACTED" ]*#SYSTEM_TMP_REDACTED#g" "$1" > "$2"; }
for f in rv94_gen.py rv94_oracle.py rv94_compare.py rv94_compare_ts.py rv94_dump_py.py zz_rv94_dump.rs zzRV94Dump.test.tsx rv94_mutants.py cargo_run.sh run_py.sh run_ts_suite.sh chain_pp.sh chain_rs2.sh chain_ts2.sh make_evidence.sh; do san $S/$f $E/scripts/$f; done
for f in py_base.jsonl py_cand.jsonl rs_base.jsonl rs_cand.jsonl ts_base.jsonl ts_cand.jsonl ts_base.jsonl.ipc.jsonl ts_cand.jsonl.ipc.jsonl py_m09.jsonl; do san $S/$f $E/dumps/$f; done
shasum -a 256 $S/inputs.jsonl | sed "s#$S/##" > $E/dumps/inputs_sha256.txt
for f in cmp_py.txt cmp_rs.txt cmp_ts.txt; do san $S/$f $E/compare/$f; done
for f in mutants_rv94_py.json mutants_rv94_rs.json mutants_rv94_ts.json; do san $S/$f $E/mutants/$f; done
for f in $S/impl/*.json; do san $f $E/impl_mutants/$(basename $f); done
for f in $S/impl/logs/*.log; do san $f $E/impl_mutants/$(basename $f); done
for f in mutants_f.py mutants_f2.py mutants_r5.py mutants_i66.py pyrs_mutants.py; do san $S/impl/$f $E/impl_mutants/$f; done
cp $S/sweep/*.tsv $E/sweep/; shasum -a 256 $S/sweep/*.tsv $S/live/*.json | sed "s#$S/##" > $E/sweep/sha256.txt
for f in $S/u5/*; do san $f $E/u5/$(basename $f); done
san $S/ts_base_outcomes.tsv $E/suites/ts_base_outcomes.tsv; san $S/ts_cand_outcomes.tsv $E/suites/ts_cand_outcomes.tsv; san $S/ts_base_vs_cand_titles.json $E/suites/ts_base_vs_cand_titles.json
for L in base cand; do cat $S/ts_${L}_full/exit.txt > $E/suites/ts_${L}_exit.txt; san $S/ts_${L}_full/tsc.out $E/suites/ts_${L}_tsc.out; done
san $S/py_base_outcomes.txt $E/suites/py_base_retained_outcomes.txt; san $S/py_cand_outcomes.txt $E/suites/py_cand_retained_outcomes.txt
for f in $S/logs/*.outcomes; do san $f $E/suites/$(basename $f); done
echo done
