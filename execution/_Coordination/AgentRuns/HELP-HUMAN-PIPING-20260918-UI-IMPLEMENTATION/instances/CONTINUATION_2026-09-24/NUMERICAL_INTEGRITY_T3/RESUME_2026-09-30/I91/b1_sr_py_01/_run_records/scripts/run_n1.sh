#!/bin/bash
source WT/scratch/i91_b1_sr_py/env.sh
cd WT/scratch/i91_b1_sr_py
$VENV/bin/python WT/scratch/i91_b1_sr_py/n1_diff.py WT/scratch/i91_b1_sr_py/base/projects/chirality-piping WT/scratch/i91_b1_sr_py/base/projects/chirality-piping/fixtures/results/retained_precision_cases.json WT/scratch/i91_b1_sr_py/n1/base.json --reader > WT/scratch/i91_b1_sr_py/n1/base.log 2>&1
echo "base rc=$?" >> WT/scratch/i91_b1_sr_py/n1/rc.txt
$VENV/bin/python WT/scratch/i91_b1_sr_py/n1_diff.py WT/b1-p/projects/chirality-piping WT/b1-p/projects/chirality-piping/fixtures/results/retained_precision_cases.json WT/scratch/i91_b1_sr_py/n1/head.json --reader > WT/scratch/i91_b1_sr_py/n1/head.log 2>&1
echo "head rc=$?" >> WT/scratch/i91_b1_sr_py/n1/rc.txt
echo done >> WT/scratch/i91_b1_sr_py/n1/rc.txt
