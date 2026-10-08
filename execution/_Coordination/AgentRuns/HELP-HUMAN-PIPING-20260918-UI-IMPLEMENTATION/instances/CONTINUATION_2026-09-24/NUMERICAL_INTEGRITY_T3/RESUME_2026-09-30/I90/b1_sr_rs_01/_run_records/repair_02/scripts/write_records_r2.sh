#!/bin/bash
# I90 SR-RS repair 2: copy the run records into R/I90/b1_sr_rs_01/_run_records/repair_02 with placeholder paths only.
set -eu
WT=WT
S=$WT/scratch/i90_b1_sr_rs; X=$S/repair_02
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
D=$R/I90/b1_sr_rs_01/_run_records/repair_02
SAN="python3 $S/scripts/sanitize.py"
rm -rf $D; mkdir -p $D/harness $D/out $D/suites $D/pins $D/mutants/logs $D/fmt $D/scripts
( cd $WT/b1-r && GIT_OPTIONAL_LOCKS=0 git log --format='%H %s' b5cb7faaeb..6e3e4fe219 > $D/commits.txt && GIT_OPTIONAL_LOCKS=0 git diff b5cb7faaeb..6e3e4fe219 > $D/repair_02.diff ) 2>/dev/null
cp $X/harness/rv113_census.rs $D/harness/
$SAN $X/harness/build_probes.py $D/harness/build_probes.py
shasum -a 256 $X/harness/probes_all.json | sed "s#  .*/#  #" > $D/harness/probes_all.sha256
$SAN $X/run_harness.sh $D/harness/run_harness.sh
$SAN $X/compare_r2.py $D/harness/compare_r2.py
for f in census_base.jsonl census_head.jsonl probes_base.jsonl probes_head.jsonl PROBE_TABLE_R2.json compare_r2.out; do cp $X/out/$f $D/out/; done
for f in harness_base.log harness_head.log; do $SAN $X/out/$f $D/out/$f; done
cp $X/census_r2_contract.out $D/suites/
for n in r2_re r2_pp r2_pins; do $SAN $S/logs/$n.log $D/suites/$n.log; cp $S/logs/$n.outcomes $D/suites/; done
cp $S/logs/r1_re.outcomes $D/suites/b5cb_re.outcomes
cp $S/logs/cand_pp.outcomes $D/suites/cc81_pp.outcomes
for n in r2_build1 r2_tests1 r2_tests2 r2_tests3; do $SAN $S/logs/$n.log $D/suites/$n.log; done
$SAN $X/run_r2.out $D/run_r2.out
( cd $S/pins && shasum -a 256 r2/* r1/* && for f in r1/*; do cmp -s $f r2/$(basename $f) && echo "identical $(basename $f)" || echo "DIFFERENT $(basename $f)"; done ) > $D/pins/compare_pins_r2.out
for f in make_schema.py manifest.json mutant_schema_RS.diff run_mutants.sh run.out; do $SAN $X/mutants/$f $D/mutants/$f; done
for f in $X/mutants/logs/*; do $SAN $f $D/mutants/logs/$(basename $f); done
for f in fmt_hunks.py fmt_apply.py; do $SAN $X/$f $D/fmt/$f; done
for f in fmt_retained_precision.rs.diff fmt_retained_precision_contract.rs.diff fmt_touch_retained_precision.rs.txt fmt_touch_retained_precision_contract.rs.txt mine_retained_precision.rs.diff mine_retained_precision_contract.rs.diff; do $SAN $X/$f $D/fmt/$f; done
$SAN $X/base_rs.rs.fmtcheck $D/fmt/base_b5cb_retained_precision.rs.fmtcheck
$SAN $X/base_contract.rs.fmtcheck $D/fmt/base_b5cb_retained_precision_contract.rs.fmtcheck
for f in run_r2.sh run_r2b.sh write_records_r2.sh; do $SAN $X/$f $D/scripts/$f; done
for f in run_suites.sh run_re.sh; do $SAN $S/$f $D/scripts/$f; done
awk '$1 >= "2026-10-08T00:15"' $WT/guard/cargo_jobs.log | grep -E "i90_b1_sr_rs|/b1-r/" > $X/cargo_jobs_i90_r2.log || true
$SAN $X/cargo_jobs_i90_r2.log $D/cargo_jobs_i90_r2.log
MACHINE="/""Users/\|/""private/tmp\|/""var/folders\|Mac[.]ht[.]home\|Ryans[-]Mac""Book\|Mac""Book"  # machine paths and host names, spelled split
if grep -rIl "$MACHINE" $D; then echo "MACHINE PATHS OR HOST NAME REMAIN"; exit 3; fi
if find $D -type l | grep -q .; then echo "SYMLINK"; exit 4; fi
if find $D -type d -name build | grep -q .; then echo "BUILD FOLDER"; exit 5; fi
echo "records written: $(find $D -type f | wc -l | tr -d ' ') files, $(du -sh $D | cut -f1)"
