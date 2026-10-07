#!/bin/bash
# I90 SR-RS repair 1: the RE suite (census lines) and the c = 1 pins at the repair head, then RV113's three mutants.
S=WT/scratch/i90_b1_sr_rs
$S/run_suites.sh r1_re r1_pins
cd $S/repair_01/mutants && rm -f results.jsonl && python3 mutants_r1.py > run.out 2>&1; echo "mutants rc=$?"
