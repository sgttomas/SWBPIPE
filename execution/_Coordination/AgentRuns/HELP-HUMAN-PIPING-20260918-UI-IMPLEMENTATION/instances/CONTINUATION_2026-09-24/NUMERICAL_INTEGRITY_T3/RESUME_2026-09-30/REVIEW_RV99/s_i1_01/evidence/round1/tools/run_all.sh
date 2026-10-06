#!/bin/bash
# RV99 round 1: every oracle run on the repaired candidate copy.
WT=<WT>
S=$WT/scratch/rv99_s_i1_01; C=$WT/rv99/projects/chirality-piping; R1=$S/r1
VENV=<VENV>
cd $S
$VENV/bin/python -B conv_i73.py $C/fixtures/rule_interval/rule_interval_cases.json $R1/r1_i73cases.json
$VENV/bin/python -B gen_extra.py $R1/r1_extra.json
for name in pass1:rv99_cases.json dense:rv99_dense.json seed1:seed1.json seed2:seed2.json seed3:seed3.json seed4:seed4.json extra:r1/r1_extra.json i73:r1/r1_i73cases.json invalid:r1/r1_invalid.json; do
  n=${name%%:*}; f=${name#*:}
  RV99_CASES=$S/$f RV99_OUT=$R1/out_$n.json $S/rc.sh $C/core/rules/rule_check_runner cand r1_h_$n test --locked --offline --release --test rv99_harness
  echo "$n harness rc=$?"
  $VENV/bin/python -B rv99_check.py $S/$f $R1/out_$n.json $C/core/analysis_runs > $R1/check_$n.txt 2>&1
  grep VIOL $R1/check_$n.txt | sed "s/^/$n /"
done
$S/rc.sh $C/core/rules/rule_check_runner cand r1_probes test --locked --offline --release --test rv99_probes -- --nocapture
grep '^{' $S/logs/r1_probes.txt > $R1/probes_r1.jsonl
