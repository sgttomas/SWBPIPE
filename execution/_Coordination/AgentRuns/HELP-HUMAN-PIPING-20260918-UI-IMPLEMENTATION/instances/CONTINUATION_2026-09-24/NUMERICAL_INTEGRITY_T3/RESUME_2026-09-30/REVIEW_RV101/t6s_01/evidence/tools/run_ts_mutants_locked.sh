#!/bin/bash
# RV101: the remaining I75 mutants (part 2) and RV101's own, as one job under the T3 host lock (ROOT's rule of 2026-10-06).
export TMPDIR=WT/scratch/rv101_t6s_01/tmp
M=WT/rv101/mut/projects/chirality-piping
echo "$(date -u '+%FT%TZ') START RV101 TS mutant loop (locked)" >> WT/guard/cargo_jobs.log
python3 WT/scratch/rv101_t6s_01/tools/i75_mutants_final.py $M WT/scratch/rv101_t6s_01/out/mut_i75_part2 M19,M20,M21,M22,M23,M24,M25,M26,M27,M28,M29,M30,M31,M32,M33,M34,R01,R02,R03,R04,R05,R06,R07,R08,R09,R10,R11,R12,R13,R14,R15,R16,R17,R18,R19,R20 > WT/scratch/rv101_t6s_01/logs/mut_i75_part2.log 2>&1; echo "i75 rc=$?" >> WT/scratch/rv101_t6s_01/logs/mut_i75_part2.log
python3 WT/scratch/rv101_t6s_01/tools/rv101_ts_mutants.py $M WT/scratch/rv101_t6s_01/out/mut_rv101 > WT/scratch/rv101_t6s_01/logs/mut_rv101.log 2>&1; echo "rv101 rc=$?" >> WT/scratch/rv101_t6s_01/logs/mut_rv101.log
echo "$(date -u '+%FT%TZ') END RV101 TS mutant loop (locked)" >> WT/guard/cargo_jobs.log
