#!/bin/bash
# I85 B1-SP I3: copy the I3 step's scripts, checks and logs into R/I85/b1_sp_01/_run_records/i3_01/,
# sanitized (placeholder paths only; no symlink; no folder named build). After write_records.sh (ST).
# Two rounds: r105e (head 105e1a78c6: suites, pins, 35 mutants) and the head 03f55e7178 (suites,
# pins, RV109's six and O1).
set -eu
WT=WT; S=$WT/scratch/i85_b1_st; X=$S/sp03
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I85/b1_sp_01
RR=$R/_run_records/i3_01
BASE=2ba2f81863003447456ec8e9c7d2dd0dbc98ea2d; HEAD=03f55e71786a58ce9ed61863813e2410a9f53827
san() { python3 $S/sanitize.py "$@"; }
rm -rf $RR; mkdir -p $RR/scripts $RR/checks/r105e $RR/suites/r105e $RR/witness $RR/pins $RR/tests $RR/mutants/r105e $RR/mutants/head $RR/diffs $RR/cargo
# Scripts.
for f in cargo_cand.sh sanitize.py; do san $S/$f $RR/scripts/$f; done
san $S/sp02/post_i3.py $RR/scripts/post_i3.py; san $S/sp02/wc2_pins.json $RR/scripts/wc2_pins.json
for f in sp_suites.sh run_base.sh run_head.sh mutants_sp.py mutant_ids.txt write_records_i3.sh; do san $X/$f $RR/scripts/$f; done
for f in screen_i3.sh; do san $X/rec/$f $RR/scripts/$f; done
for f in rv2_patch_identity pin_literals guard_text_identity reversed_input_identity witness_lines suite_delta mutant_table carry_over; do
  san $X/rec/$f.py $RR/scripts/$f.py; san $X/rec/$f.out $RR/checks/$f.out; done
for f in suite_delta witness_lines mutant_table; do san $X/r105e/$f.out $RR/checks/r105e/$f.out; done
# Suites: the base (2ba2f81863, archive S/base_i3) and the head; r105e's head outcomes kept.
for f in base_reg_pp base_stale_pp base_reg_runner cand_reg_pp cand_stale_pp cand_reg_runner cand_reg_re_carriers; do
  san --filter $X/suites/$f.log $RR/suites/$f.filtered.log; cp $X/suites/$f.outcomes $X/suites/$f.warnings $RR/suites/; done
for f in cand_reg_pp cand_stale_pp cand_reg_runner cand_reg_re_carriers; do cp $X/r105e/suites/$f.outcomes $RR/suites/r105e/; done
san $X/run_base.out $RR/suites/run_base.out; san $X/run_head.out $RR/suites/run_head.out; san $X/r105e/run_head.out $RR/suites/r105e/run_head.out
for f in base_reg_witness cand_reg_witness; do san $X/suites/$f.log $RR/witness/$f.log; done
cp $X/rec/base_witness.lines $X/rec/cand_witness.lines $RR/witness/
# Pins.
cp $X/pins_base/pins.sha256 $RR/pins/pins_base_2ba2f81863.sha256; cp $X/r105e/pins_head/pins.sha256 $RR/pins/pins_head_105e1a78c6.sha256
cp $X/pins_head/pins.sha256 $RR/pins/pins_head_03f55e7178.sha256
san --filter $X/logs/pins_base.log $RR/pins/pins_base.filtered.log; san --filter $X/logs/pins_head.log $RR/pins/pins_head.filtered.log
# The b1_sp tests at the head, with their prints (SF-1 records, SF-2 headlines, the pins).
grep -aE "B1_SP_|^test |^test result|Running|panicked" $X/logs/i3_sf2_restructure2.log > $X/rec/b1_sp_head.lines
san $X/rec/b1_sp_head.lines $RR/tests/b1_sp_head.lines
# Mutants: r105e (35) and the head (RV109's six and O1).
for d in r105e/mutants:r105e mutants:head; do src=$X/${d%%:*}; dst=$RR/mutants/${d#*:}
  san $src/mutants.json $dst/mutants.json; san $src/run.out $dst/run.out
  for f in $src/mutant_*.log; do san --filter $f $dst/$(basename $f .log).filtered.log; done; done
# Diffs and commits.
cd $WT/b1
GIT_OPTIONAL_LOCKS=0 git diff $BASE $HEAD -- projects/chirality-piping/core > $X/rec/i3_src.diff.raw
san $X/rec/i3_src.diff.raw $RR/diffs/i3_src_2ba2f81863__03f55e7178.diff
cmp -s $X/rec/i3_src.diff.raw $RR/diffs/i3_src_2ba2f81863__03f55e7178.diff && echo "diff unchanged by sanitizing"
GIT_OPTIONAL_LOCKS=0 git diff --stat=200 $BASE $HEAD > $RR/diffs/i3_2ba2f81863__03f55e7178.stat
GIT_OPTIONAL_LOCKS=0 git log --format='%H %s' $BASE..$HEAD > $RR/commits.txt
( cd projects/chirality-piping && shasum -a 256 fixtures/results/retained_precision_w_c2_successor_*.json ) > $RR/diffs/w_c2_fixtures.sha256
# My cargo job lines since the I3 step began.
awk '$1 >= "2026-10-08T00:00:00Z"' $WT/guard/cargo_jobs.log | grep -E "i85_b1_st|/t3/b1/" > $X/rec/cargo_jobs_i3.raw || true
san $X/rec/cargo_jobs_i3.raw $RR/cargo/cargo_jobs_i85_i3.log
if find $RR -type l | grep -q .; then echo "SYMLINK IN RECORDS"; exit 9; fi
if find $RR -type d -name build | grep -q .; then echo "BUILD FOLDER IN RECORDS"; exit 9; fi
echo "records written: $(find $RR -type f | wc -l | tr -d ' ') files"
