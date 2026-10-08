#!/bin/bash
# I91 repair 02: one locked job: RV113's harness (census and its three probe sets) and my census, on one P tree.
# Usage: job_r02.sh <label> <P root>
set -u
source WT/scratch/i91_b1_sr_py/env.sh
LABEL=$1; P=$2; O=$S/r02/$LABEL; mkdir -p $O
/usr/bin/lockf -k $WT/guard/cargo_job.lock /bin/bash -c "
cd $S
date -u '+%FT%TZ start' > $O/stamps.txt
$VENV/bin/python $S/r02/rv113/rv113_py_harness.py $P census $O/census_rv113.jsonl > $O/log.txt 2>&1
for set in v5 fg rp; do $VENV/bin/python $S/r02/rv113/rv113_py_harness.py $P probes $S/r02/rv113/probes_\$set.json $O/probes_\$set.jsonl >> $O/log.txt 2>&1; done
$VENV/bin/python $S/census.py $P $P/fixtures/results/retained_precision_cases.json $O/census_i91.json >> $O/log.txt 2>&1
date -u '+%FT%TZ end' >> $O/stamps.txt
"
echo "rc=$?" >> $O/stamps.txt
