#!/bin/bash
# I85 B1-ST: copy the run's scripts and logs into the records, sanitized (placeholder paths only).
set -eu
WT=WT; S=$WT/scratch/i85_b1_st; L=$S/logs
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I85/b1_st_01
RR=$R/_run_records
san() { python3 $S/sanitize.py "$@"; }
rm -rf $RR; mkdir -p $RR/scripts $RR/guards $RR/suites/round1 $RR/witness $RR/pins $RR/mutants/round1 $RR/build
for f in cargo_cand.sh run_suites.sh run_pins.sh mutants.py sanitize.py write_records.sh; do san $S/$f $RR/scripts/$f; done
for f in guards01_pp guards02_re_carriers guards03_pp_head guards04_re_carriers_head; do san $L/$f.log $RR/guards/$f.log; done
for f in cand_reg_pp base_reg_pp cand_stale_pp base_stale_pp; do san $L/$f.log $RR/suites/$f.log; cp $L/$f.outcomes $L/$f.warnings $RR/suites/; done
for f in cand_reg_runner base_reg_runner; do san --filter $L/$f.log $RR/suites/$f.filtered.log; cp $L/$f.outcomes $L/$f.warnings $RR/suites/; done
san $L/run_suites.out $RR/suites/run_suites.out; san $L/run_suites_2.out $RR/suites/run_suites_2.out
for f in cand_reg_pp cand_stale_pp; do san $L/round1/$f.log $RR/suites/round1/$f.log; cp $L/round1/$f.outcomes $L/round1/$f.warnings $RR/suites/round1/; done
for f in cand_reg_witness base_reg_witness cand_stale_witness; do san $L/$f.log $RR/witness/$f.log; done
cp $L/base_witness.lines $L/cand_witness.lines $RR/witness/
for f in pins_cand pins_base; do san $L/$f.log $RR/pins/$f.log; done
san $L/run_pins.out $RR/pins/run_pins.out; cp $S/pins/pins.sha256 $RR/pins/
san $L/mutants/mutants.json $RR/mutants/mutants.json; san $L/mutants/mutants.out $RR/mutants/mutants.out
san $L/round1/mutants/mutants.json $RR/mutants/round1/mutants.json
for f in $L/mutants/mutant_*.log; do san --filter $f $RR/mutants/$(basename $f .log).filtered.log; done
san $L/build01_norun.log $RR/build/build01_norun.log; san $L/lib01.log $RR/build/lib01.log
grep -E "i85_b1_st|/t3/b1/" $WT/guard/cargo_jobs.log > $S/cargo_jobs_i85.raw; san $S/cargo_jobs_i85.raw $RR/cargo_jobs_i85.log
T=$WT/targets/i85-b1-st
{ for d in cand:. base:base stale:stale base-stale:base-stale mut:mut; do n=${d%%:*}; p=${d#*:}; for f in $T/$p/debug/build/open_pipe_stress_product_physics-*/output; do echo "$n $(grep -o 'OPS_RETAINED_BUILD_IDENTITY=.*' $f)"; done; done; } | sort -u > $S/build_identities.raw
san $S/build_identities.raw $RR/build/build_identities.txt
cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git diff 47a3bdfcf5 a8e719f5b4 -- projects/chirality-piping/core > $S/st.diff.raw && san $S/st.diff.raw $RR/st.diff
GIT_OPTIONAL_LOCKS=0 git log --format='%H %s' 47a3bdfcf5..a8e719f5b4 > $RR/commits.txt
cmp -s $S/st.diff.raw $RR/st.diff && echo "st.diff unchanged by sanitizing"
