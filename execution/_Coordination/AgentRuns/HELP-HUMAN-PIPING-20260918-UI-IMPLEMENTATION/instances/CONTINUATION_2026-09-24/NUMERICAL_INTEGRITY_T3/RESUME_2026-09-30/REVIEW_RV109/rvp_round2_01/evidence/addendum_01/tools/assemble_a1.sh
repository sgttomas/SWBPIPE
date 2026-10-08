#!/bin/bash
# RV109 round 2, addendum 01: copy the evidence into the records, machine paths replaced (sanitize.py).
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
X=$S/a1
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
E=$R/REVIEW_RV109/rvp_round2_01/evidence/addendum_01
san() { python3 $S/tools/sanitize.py "$@"; }
mkdir -p $E/suites $E/probe $E/mutants $E/tools $E/ledger
for a in pp_reg pp_stale runner re; do for rev in i3 head; do
  san $S/logs/a1_${a}_$rev.log $E/suites/a1_${a}_$rev.filtered.log --filter '^test |^test result|Running |RV109_JOB|panicked at|^failures:|^    [a-z_:0-9]+$'
done; done
for rev in i3 head; do san $S/logs/a1_wit_$rev.log $E/suites/a1_wit_$rev.filtered.log --filter 'I65_G5_WITNESS|^test result|RV109_JOB'; done
san $X/suite_diff.txt $E/suites/suite_diff_I3__03f55e7178.txt
for rev in i3 head; do
  san $X/probe/$rev/out.jsonl $E/probe/out_$rev.jsonl
  san $X/probe/$rev/stage.jsonl $E/probe/stage_$rev.jsonl
  san $S/logs/a1_probe_$rev.log $E/probe/probe_${rev}_rv109_lines.log --filter 'RV109_(WC2|ONE|R10|REV|DOC) |^test result|RV109_JOB'
done
for f in probe_compare.txt stage_compare.txt wc2_head_summary.txt n16_head.txt py_reader_reversed.txt; do san $X/$f $E/probe/$f; done
for f in zz_rv109_probe.rs zz_rv109_rev.head.rs; do san $X/probe/$f $E/probe/$f; done
(cd $X/probe && shasum -a 256 i3/doc/*.json head/doc/*.json) > $E/probe/w_c2_documents.sha256
san $X/mutants/summary.md $E/mutants/summary.md
cp $X/mutants/list.tsv $X/mutants/restored_src.sha256 $E/mutants/
for m in $(cut -f1 $X/mutants/list.tsv) pristine; do san $X/mutants/$m.log $E/mutants/$m.filtered.log --filter '^test .*FAILED|^test result|RV109_JOB|panicked at|^  left:|^ right:|assertion'; done
for f in $X/a1_jobs.sh $X/a1_mutants.py $X/assemble_a1.sh $S/r2/sp2_mutants.py $S/r2/tools/ledger.py $S/r2/tools/ledger_table.py $S/r2/tools/stage_compare2.py $S/r2/tools/wc2_summary.py $S/runjob.sh; do san $f $E/tools/$(basename $f); done
san $X/ledger_raw.md $E/ledger/ledger_raw.md
awk '$1>="2026-10-08T01:40:00Z"' $WT/guard/cargo_jobs.log | grep 'rv109/a1_\|rv109_rvp_01' > $X/cargo_jobs_a1.log
san $X/cargo_jobs_a1.log $E/cargo_jobs_rv109_a1.log
find $E -type l | head -1 | grep -q . && echo "SYMLINK PRESENT" || echo "no symlink"
grep -rl "~" $E && echo "UNSANITIZED" || echo "sanitized: no machine path"
grep -rli "$(hostname -s)" $E && echo "HOSTNAME PRESENT" || echo "no hostname"
grep -rlE '[.]l[o]cal([^a-z]|$)' $E && echo "dot-l0cal PRESENT" || echo "no dot-l0cal"
