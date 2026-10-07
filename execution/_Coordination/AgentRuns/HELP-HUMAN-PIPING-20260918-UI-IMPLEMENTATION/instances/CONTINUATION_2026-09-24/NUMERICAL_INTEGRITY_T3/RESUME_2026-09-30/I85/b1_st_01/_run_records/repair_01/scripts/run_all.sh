#!/bin/bash
# I85 B1-ST repair 1: the evidence at head 98a77c716e, one cargo job at a time through
# WT/tools/t3_cargo.sh (via run_suites.sh, cargo_cand.sh and mutants.py).
set -u
WT=WT; S=$WT/scratch/i85_b1_st
echo "== start $(date -u +%FT%TZ)"
$S/run_suites.sh cand_reg_pp cand_stale_pp cand_reg_witness cand_reg_re_carriers
mkdir -p $S/logs/mutants
VENV/bin/python $S/mutants.py $S/mut/projects/chirality-piping $S/logs/mutants $WT/targets/i85-b1-st/mut $WT/tools/t3_cargo.sh $S/tmp
echo "== mutants rc=$? $(date -u +%FT%TZ)"
