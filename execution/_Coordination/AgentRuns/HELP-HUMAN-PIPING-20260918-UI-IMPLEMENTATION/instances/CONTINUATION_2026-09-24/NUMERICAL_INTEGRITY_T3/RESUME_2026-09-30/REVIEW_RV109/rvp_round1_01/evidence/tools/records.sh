#!/bin/bash
# RV109: copy the sanitized evidence into the records folder (placeholder paths only).
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
E=$R/REVIEW_RV109/rvp_round1_01/evidence
SAN="python3 $S/tools/sanitize.py"
mkdir -p $E/{suites,witness,probe,mutants,tools,diffs}
F='^\s+Running |^\s+Doc-tests |^test |^test result:|^failures:|^    [A-Za-z_:0-9]+$|panicked at|^RV109_JOB|^error|^warning: `'
for n in pp_reg_base pp_reg_cand pp_stale_base pp_stale_cand runner_base runner_cand re_carriers_cand; do
  $SAN $S/logs/$n.log $E/suites/$n.filtered.log --filter "$F"
done
pair() { python3 $S/tools/suite_diff.py $S/logs/$1.log $S/logs/$2.log > $S/tmp/sd.txt; $SAN $S/tmp/sd.txt $E/suites/diff_$1__$2.txt; }
pair pp_reg_base pp_reg_cand
pair pp_stale_base pp_stale_cand
pair runner_base runner_cand
pair pp_reg_base pp_stale_base
pair pp_reg_cand pp_stale_cand
for n in wit_base wit_cand wit_stale_cand; do
  $SAN $S/logs/$n.log $E/witness/$n.lines.log --filter '^I65_G5_WITNESS|^test result:|^RV109_JOB|^test retained_memory::witness_tests::'
done
for f in out_base.jsonl out_cand.jsonl compare.txt fixture_files.txt zz_rv109_probe.rs zz_rv109_rev.base.rs zz_rv109_rev.cand.rs; do
  $SAN $S/probe/$f $E/probe/$f
done
for n in probe_base probe_cand; do
  $SAN $S/logs/$n.log $E/probe/$n.filtered.log --filter '^RV109|^test result:|^test |panicked|^error'
done
for f in runjob.sh phase1_suites.sh phase2_probe.sh phase3_mutants.sh records.sh; do $SAN $S/$f $E/tools/$f; done
for f in suite_diff.py probe_compare.py mutants.py sanitize.py mutant_summary.py; do [ -f $S/tools/$f ] && $SAN $S/tools/$f $E/tools/$f; done
cp $S/diffs/head_vs_base.diff "$E/diffs/head_vs_base_47a3bdfcf5..a8e719f5b4.diff"
cp $S/diffs/testonly.diff "$E/diffs/testonly_4a51783e65..a8e719f5b4.diff"
if [ -d $S/mutants ]; then
  for f in $S/mutants/*.log; do [ -f "$f" ] && $SAN $f $E/mutants/$(basename $f .log).filtered.log --filter "$F"; done
  for f in $S/mutants/*.apply $S/mutants/*.lib.sha256 $S/mutants/summary.md; do [ -f "$f" ] && $SAN $f $E/mutants/$(basename $f); done
fi
for n in r10_head r10_mutant; do $SAN $S/logs/$n.log $E/mutants/$n.filtered.log --filter 'RV109_R10|^test result:|^RV109_JOB|^error'; done
[ -f $S/logs/phase4_restored.sha256 ] && $SAN $S/logs/phase4_restored.sha256 $E/mutants/phase4_restored.sha256
[ -f $S/mutants/restored.sha256 ] && $SAN $S/mutants/restored.sha256 $E/mutants/phase3_restored.sha256
$SAN $S/phase4_r10.sh $E/tools/phase4_r10.sh
grep -h 'rv109' $WT/guard/cargo_jobs.log > $S/tmp/jobs.txt; $SAN $S/tmp/jobs.txt $E/cargo_jobs_rv109.log
grep -rl "$HOME" $E && echo MACHINE_PATHS_FOUND || echo no_machine_paths
