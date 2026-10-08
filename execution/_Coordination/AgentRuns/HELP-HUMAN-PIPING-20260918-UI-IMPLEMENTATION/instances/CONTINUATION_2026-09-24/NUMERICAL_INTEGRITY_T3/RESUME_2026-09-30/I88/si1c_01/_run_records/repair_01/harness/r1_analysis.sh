#!/bin/bash
# I88 repair round 1: the differential analysis, under the T3 lock (scratch tool).
set -u
WT=WT
S=$WT/scratch/i88_si1c
export TMPDIR=$S/tmp
cd $S
echo "$(date -u '+%FT%TZ') begin r1_analysis" >> $S/logs/driver.log
VENV/bin/python harness/si1c_r1_delta.py $S/dumps_r1/cand_r0 $S/dumps_r1/cand $S/dumps_r1/base $S/dumps_r1/ibase $S/reports/r1_delta_report.json > $S/logs/r1/delta.log 2>&1
echo "delta rc=$?" >> $S/logs/r1/delta.log
VENV/bin/python harness/si1c_compare.py $S $S/trees/cand/projects/chirality-piping/schemas/rule_check_run_result.schema.json $S/reports/r1_diff_report.json $S/dumps_r1 > $S/logs/r1/compare.log 2>&1
echo "compare rc=$?" >> $S/logs/r1/compare.log
echo "$(date -u '+%FT%TZ') end r1_analysis" >> $S/logs/driver.log
