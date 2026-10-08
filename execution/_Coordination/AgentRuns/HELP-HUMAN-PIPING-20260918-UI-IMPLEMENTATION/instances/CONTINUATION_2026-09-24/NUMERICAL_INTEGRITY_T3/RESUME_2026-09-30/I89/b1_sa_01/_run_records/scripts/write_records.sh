#!/bin/bash
# I89 B1-SA: copy the run records into R/I89/b1_sa_01/_run_records with placeholder paths only
# (sanitize.py), then write SHA256SUMS over RETURN.md and every file under _run_records/.
set -eu
WT=WT
S=$WT/scratch/i89_b1_sa; L=$S/logs
VENV=VENV
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I89/b1_sa_01
O=$R/_run_records
SAN="$VENV/bin/python $S/scripts/sanitize.py"
mkdir -p $O/scripts $O/suites $O/guards $O/pins $O/mutants $O/identity $O/early $O/i2_preview
cd $WT/b1-a
GIT_OPTIONAL_LOCKS=0 git diff 262bd687f0 6b62606778 > $S/tmp/sa.diff
$SAN $S/tmp/sa.diff $O/sa.diff
GIT_OPTIONAL_LOCKS=0 git log --format='%H %P%n%an %aI%n%B' 262bd687f0..HEAD > $S/tmp/commits.txt
$SAN $S/tmp/commits.txt $O/commits.txt
for f in cargo_cand.sh run_suites.sh run_pins.sh mutants.py sanitize.py write_records.sh i2_preview.sh; do $SAN $S/scripts/$f $O/scripts/$f; done
for n in cand_reg_pp base_reg_pp cand_reg_witness base_reg_witness cand_sa_nocapture cand_stale_retained_memory; do
  [ -f $L/$n.log ] && $SAN $L/$n.log $O/suites/$n.log && cp $L/$n.outcomes $O/suites/$n.outcomes
done
for n in cand_reg_runner base_reg_runner; do
  [ -f $L/$n.log ] && $SAN --filter $L/$n.log $O/suites/$n.filtered.log && cp $L/$n.outcomes $O/suites/$n.outcomes
done
for n in cand_reg_pp base_reg_pp cand_reg_runner base_reg_runner; do
  [ -f $L/$n.log ] && grep -E '^warning|^error' $L/$n.log | sort | uniq -c > $O/suites/$n.warnings || true
done
for n in cand_guards_pp cand_guards_re; do [ -f $L/$n.log ] && $SAN $L/$n.log $O/guards/$n.log && cp $L/$n.outcomes $O/guards/$n.outcomes; done
for f in run_suites_base.out run_suites_cand.out run_suites_stale.out run_pins.out suites_diff.txt; do [ -f $L/$f ] && $SAN $L/$f $O/suites/$f; done
[ -f $L/pins_cand.log ] && $SAN $L/pins_cand.log $O/pins/pins_cand.log && $SAN $L/pins_base.log $O/pins/pins_base.log && cp $S/pins/pins.sha256 $O/pins/
for n in cand_identity base_identity build_norun; do [ -f $L/$n.log ] && $SAN $L/$n.log $O/identity/$n.log; done
for n in early_retained_memory early_retained_memory_2; do $SAN $L/$n.log $O/early/$n.log; done
if [ -d $L/mutants ]; then
  for f in $L/mutants/mutant_*.log; do b=$(basename $f .log); $SAN --filter $f $O/mutants/$b.filtered.log; done
  for f in mutants.json mutants.out; do [ -f $L/mutants/$f ] && $SAN $L/mutants/$f $O/mutants/$f; done
fi
$SAN $S/tmp/parked_slots.diff $O/i2_preview/parked_slots.diff
for n in i2_preview_lib i2_unpatched_parked_test; do [ -f $L/$n.log ] && $SAN $L/$n.log $O/i2_preview/$n.log; done
[ -f $L/i2_preview.out ] && $SAN $L/i2_preview.out $O/i2_preview/i2_preview.out
grep -E 'i89_b1_sa|/b1-a/' $WT/guard/cargo_jobs.log > $S/tmp/cargo_jobs_i89.log || true
$SAN $S/tmp/cargo_jobs_i89.log $O/cargo_jobs_i89.log
cd $R && find . -type f ! -name SHA256SUMS | sort | sed 's#^\./##' | xargs shasum -a 256 > $R/SHA256SUMS
echo "records: $(wc -l < $R/SHA256SUMS) files"
