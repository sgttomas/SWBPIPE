#!/bin/bash
# I90 B1-SR-RS: copy the run records into R/I90/b1_sr_rs_01/_run_records with placeholder paths only.
set -eu
WT=WT
S=$WT/scratch/i90_b1_sr_rs
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
D=$R/I90/b1_sr_rs_01/_run_records
SAN="python3 $S/scripts/sanitize.py"
rm -rf $D; mkdir -p $D/census $D/suites $D/pins $D/mutants/logs $D/scripts
( cd $WT/b1-r && GIT_OPTIONAL_LOCKS=0 git log --format='%H %s' 262bd687f0..cc81e78801 > $D/commits.txt && GIT_OPTIONAL_LOCKS=0 git diff 262bd687f0..cc81e78801 > $D/sr_rs.diff ) 2>/dev/null
cp $S/census/census.py $S/census/census.out $S/census/census_head.out $D/census/
$SAN $S/census/base_re.log $D/census/base_re.log
$SAN $S/census/aligned_re.log $D/census/aligned_re.log
$SAN $S/logs/cand_re.log $D/census/head_re.log
for side in base cand; do
  for s in re pp pins; do $SAN $S/logs/${side}_$s.log $D/suites/${side}_$s.log; cp $S/logs/${side}_$s.outcomes $D/suites/; done
  $SAN --filter $S/logs/${side}_headless.log $D/suites/${side}_headless.filtered.log; cp $S/logs/${side}_headless.outcomes $D/suites/
done
$SAN $S/logs/run_base.out $D/suites/run_base.out
$SAN $S/logs/run_cand_pp.out $D/suites/run_cand_pp.out
$SAN $S/logs/run_cand_re2.out $D/suites/run_cand_re.out
cp $S/pins/pins.sha256 $D/pins/
$S/compare_pins.sh > $D/pins/compare_pins.out 2>&1
$SAN $S/mutants/mutants.py $D/mutants/mutants.py
cp $S/mutants/results.jsonl $D/mutants/
$SAN $S/mutants/run.out $D/mutants/run.out
for f in $S/mutants/logs/*.log; do $SAN $f $D/mutants/logs/$(basename $f); done
for f in run_suites.sh run_re.sh compare_pins.sh; do $SAN $S/$f $D/scripts/$f; done
for f in sanitize.py write_records.sh; do $SAN $S/scripts/$f $D/scripts/$f; done
grep -E "i90_b1_sr_rs|/b1-r/" $WT/guard/cargo_jobs.log > $S/cargo_jobs_i90.log || true
$SAN $S/cargo_jobs_i90.log $D/cargo_jobs_i90.log
MACHINE="/""Users/\|/""private/tmp\|/""var/folders"  # machine paths, spelled split: none may remain in the records
if grep -rIl "$MACHINE" $D; then echo "MACHINE PATHS REMAIN"; exit 3; fi
echo "records written: $(find $D -type f | wc -l | tr -d ' ') files"
