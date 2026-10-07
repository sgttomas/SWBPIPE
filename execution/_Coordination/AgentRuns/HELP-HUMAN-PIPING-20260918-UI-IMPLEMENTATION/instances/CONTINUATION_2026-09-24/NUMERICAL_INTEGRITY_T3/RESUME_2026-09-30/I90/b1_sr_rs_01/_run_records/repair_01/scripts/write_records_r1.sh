#!/bin/bash
# I90 SR-RS repair 1: copy the run records into R/I90/b1_sr_rs_01/_run_records/repair_01 with placeholder paths only.
set -eu
WT=WT
S=$WT/scratch/i90_b1_sr_rs
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
D=$R/I90/b1_sr_rs_01/_run_records/repair_01
SAN="python3 $S/scripts/sanitize.py"
rm -rf $D; mkdir -p $D/mutants/logs $D/suites $D/pins $D/scripts
( cd $WT/b1-r && GIT_OPTIONAL_LOCKS=0 git log --format='%H %s' cc81e78801..b5cb7faaeb > $D/commits.txt && GIT_OPTIONAL_LOCKS=0 git diff cc81e78801..b5cb7faaeb > $D/repair_01.diff ) 2>/dev/null
$SAN $S/repair_01/mutants/mutants_r1.py $D/mutants/mutants_r1.py
cp $S/repair_01/mutants/results.jsonl $D/mutants/
$SAN $S/repair_01/mutants/run.out $D/mutants/run.out
for f in $S/repair_01/mutants/logs/*.log; do $SAN $f $D/mutants/logs/$(basename $f); done
$SAN $S/logs/r1_re.log $D/suites/r1_re.log
cp $S/logs/r1_re.outcomes $D/suites/
cp $S/logs/cand_re.outcomes $D/suites/cc81_re.outcomes
$SAN $S/logs/r1_pins.log $D/suites/r1_pins.log
cp $S/logs/r1_pins.outcomes $D/suites/
$SAN $S/logs/r1_b1tests.log $D/suites/r1_b1tests.log
$SAN $S/repair_01/run_r1.out $D/run_r1.out
cp $S/repair_01/census_r1.out $D/
( cd $S/pins && shasum -a 256 r1/* cand/* && for f in cand/*; do cmp -s $f r1/$(basename $f) && echo "identical $(basename $f)" || echo "DIFFERENT $(basename $f)"; done ) > $D/pins/compare_pins_r1.out
for f in run_suites.sh run_re.sh; do $SAN $S/$f $D/scripts/$f; done
for f in run_r1.sh write_records_r1.sh; do $SAN $S/repair_01/$f $D/scripts/$f; done
awk '$1 >= "2026-10-07T17:40"' $WT/guard/cargo_jobs.log | grep -E "i90_b1_sr_rs|/b1-r/" > $S/repair_01/cargo_jobs_i90_r1.log || true
$SAN $S/repair_01/cargo_jobs_i90_r1.log $D/cargo_jobs_i90_r1.log
MACHINE="/""Users/\|/""private/tmp\|/""var/folders"
if grep -rIl "$MACHINE" $D; then echo "MACHINE PATHS REMAIN"; exit 3; fi
echo "records written: $(find $D -type f | wc -l | tr -d ' ') files"
