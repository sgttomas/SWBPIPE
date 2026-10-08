#!/bin/bash
# I90 SR-RS repair 2: RV113's harness (census and probes) on b5cb7faaeb and on the head, then RE's suite, PP's suite and the c = 1 pins at the head.
S=WT/scratch/i90_b1_sr_rs
$S/repair_02/run_harness.sh base
$S/repair_02/run_harness.sh head
$S/run_suites.sh r2_re r2_pins r2_pp
