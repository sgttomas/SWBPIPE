#!/bin/bash
# I90 SR-RS repair 2: after run_r2.sh's last job has gone, the mutant schema (one heavy job of mine at a time).
S=WT/scratch/i90_b1_sr_rs
while pgrep -f "$S/repair_02/run_r2.sh" >/dev/null; do sleep 10; done
$S/repair_02/mutants/run_mutants.sh > $S/repair_02/mutants/run.out 2>&1
echo "mutants done"
