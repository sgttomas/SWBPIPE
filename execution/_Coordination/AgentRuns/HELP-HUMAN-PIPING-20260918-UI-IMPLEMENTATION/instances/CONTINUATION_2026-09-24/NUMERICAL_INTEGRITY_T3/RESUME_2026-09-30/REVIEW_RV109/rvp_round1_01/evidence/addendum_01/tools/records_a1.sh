#!/bin/bash
# RV109 ADDENDUM_01: sanitized evidence into the records (placeholder paths only).
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
E=R/REVIEW_RV109/rvp_round1_01/evidence/addendum_01
SAN="python3 $S/tools/sanitize.py"
F='^\s+Running |^\s+Doc-tests |^test |^test result:|^failures:|^    [A-Za-z_:0-9]+$|panicked at|^RV109_JOB|^error|^warning: `'
for n in a1_pp_reg_head a1_pp_stale_head a1_re_carriers_head; do $SAN $S/logs/$n.log $E/suites/$n.filtered.log --filter "$F"; done
pair() { python3 $S/tools/suite_diff.py $S/logs/$1.log $S/logs/$2.log > $S/tmp/sd.txt; $SAN $S/tmp/sd.txt $E/suites/diff_$3.txt; }
pair pp_reg_cand a1_pp_reg_head reg_a8e719f5b4__98a77c716e
pair pp_stale_cand a1_pp_stale_head stale_a8e719f5b4__98a77c716e
pair a1_pp_reg_head a1_pp_stale_head reg__stale_98a77c716e
for n in a1_wit_head a1_wit_stale_head; do $SAN $S/logs/$n.log $E/witness/$n.lines.log --filter '^I65_G5_WITNESS|^test result:|^RV109_JOB|^test retained_memory::witness_tests::'; done
for m in $(cut -f1 $S/a1/mutants/list.txt) pristine; do $SAN $S/a1/mutants/$m.log $E/mutants/$m.filtered.log --filter "$F"; done
for f in $S/a1/mutants/*.apply $S/a1/mutants/*.lib.sha256 $S/a1/mutants/summary.md $S/a1/mutants/restored.sha256 $S/a1/mutants/list.txt; do $SAN $f $E/mutants/$(basename $f); done
$SAN $S/a1/a1_confirm.sh $E/tools/a1_confirm.sh; $SAN $S/a1/records_a1.sh $E/tools/records_a1.sh
for f in mutants.py suite_diff.py mutant_summary.py sanitize.py; do $SAN $S/tools/$f $E/tools/$f; done
$SAN $S/runjob.sh $E/tools/runjob.sh
cp $S/diffs/repair01.diff "$E/diffs/repair_01_a8e719f5b4..98a77c716e.diff"
grep -h '/rv109/head' $WT/guard/cargo_jobs.log > $S/tmp/jobs_a1.txt; $SAN $S/tmp/jobs_a1.txt $E/cargo_jobs_rv109_addendum_01.log
grep -rl "$HOME" $E && echo MACHINE_PATHS_FOUND || echo no_machine_paths
