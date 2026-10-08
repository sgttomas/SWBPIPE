#!/bin/bash
# RV109 round 2: copy the evidence into the records, machine paths replaced (sanitize.py).
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
E=$R/REVIEW_RV109/rvp_round2_01/evidence
san() { python3 $S/tools/sanitize.py "$@"; }
mkdir -p $E/suites $E/probe $E/wc2 $E/i3 $E/mutants $E/headline $E/ledger $E/tools
for f in r2_pp_reg_i1 r2_pp_reg_head r2_pp_stale_i1 r2_pp_stale_head r2_runner_i1 r2_runner_head r2_re_i1 r2_re_head; do
  san $S/logs/$f.log $E/suites/$f.filtered.log --filter '^test |^test result|Running |RV109_JOB|panicked at|^failures:|^    [a-z_:0-9]+$'
done
for f in r2_wit_i1 r2_wit_head; do san $S/logs/$f.log $E/suites/$f.filtered.log --filter 'I65_G5_WITNESS|^test result|RV109_JOB'; done
for f in out_i1 out_head stage_i1 stage_head; do san $S/r2/probe/$f.jsonl $E/probe/$f.jsonl; done
san $S/logs/r2_probe_head.log $E/probe/probe_head_rv109_lines.log --filter 'RV109_(WC2|ONE|R10) |^test result|RV109_JOB'
san $S/logs/r2_probe_i1.log $E/probe/probe_i1_rv109_lines.log --filter 'RV109_R10 |^test result|RV109_JOB'
san $S/logs/r2_i3_pp.log $E/i3/i3_pp.filtered.log --filter '^test .*FAILED|^test result|RV109_JOB|panicked at|^  left:|^ right:|assertion|^failures:|^    [a-z_:0-9]+$'
san $S/logs/r2_i3_re.log $E/i3/i3_re.filtered.log --filter '^test |^test result|RV109_JOB|Running'
san $S/logs/r2_i3_docs.log $E/i3/i3_docs.log --filter 'RV109_DOC|^test result|RV109_JOB'
san $S/logs/r2_i3_probe.log $E/i3/i3_probe_rv109_lines.log --filter 'RV109_WC2 |^test result|RV109_JOB'
for f in $S/r2/tools/*.py $S/r2/*.sh $S/r2/sp2_mutants.py $S/runjob.sh $S/tools/suite_diff.py $S/tools/mutant_summary.py $S/tools/sanitize.py; do
  san $f $E/tools/$(basename $f)
done
if [ -d $S/r2/mutants ]; then
  for f in $S/r2/mutants/*.log; do san $f $E/mutants/$(basename $f .log).filtered.log --filter '^test .*FAILED|^test result|RV109_JOB|panicked at|^  left:|^ right:|assertion|error\[|^error'; done
  cp $S/r2/mutants/list*.tsv $S/r2/mutants/restored_src*.sha256 $E/mutants/ 2>/dev/null
fi
cp $S/r2/mutants/summary_a.md $S/r2/mutants/summary_b.md $S/r2/i3mut/summary_i3.txt $E/mutants/
mkdir -p $E/mutants/i3_merge
for f in $S/r2/i3mut/*.log; do san $f $E/mutants/i3_merge/$(basename $f .log).filtered.log --filter 'RV109_WC2 |^test result|RV109_JOB'; done
cp $S/r2/i3mut/restored_src.sha256 $E/mutants/i3_merge/
for f in $S/r2/evidence/*.txt $S/r2/evidence/*.md; do :; done
cp $S/r2/evidence/suite_diff.txt $E/suites/suite_diff_I1__603e238517.txt
cp $S/r2/evidence/stage_compare.txt $S/r2/evidence/probe_compare.txt $E/probe/
cp $S/r2/evidence/wc2_head_summary.txt $S/r2/evidence/n16_record_diff.txt $E/wc2/
cp $S/r2/evidence/headline_py_base_reader.txt $E/headline/
cp $S/r2/evidence/wc2_i3_summary.txt $S/r2/i3/overlay.sha256 $E/i3/
cp $S/r2/evidence/ledger_sp.md $E/ledger/
(cd $S/r2/probe/wc2 && shasum -a 256 *.json) > $E/wc2/head_wc2_documents.sha256
(cd $S/r2/i3/wc2 && shasum -a 256 *.json) > $E/i3/i3_wc2_documents.sha256
san $S/probe/fixture_files.txt $E/probe/fixture_files.txt
for f in zz_rv109_probe.rs zz_rv109_rev.i1.rs zz_rv109_rev.head.rs; do san $S/r2/probe/$f $E/probe/$f; done
grep -rl "~" $E && echo "UNSANITIZED" || echo "sanitized: no machine path"
