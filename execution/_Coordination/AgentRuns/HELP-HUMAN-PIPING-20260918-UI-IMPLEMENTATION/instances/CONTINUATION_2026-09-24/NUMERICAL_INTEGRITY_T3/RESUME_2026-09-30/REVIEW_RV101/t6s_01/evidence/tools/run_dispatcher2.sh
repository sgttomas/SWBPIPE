#!/bin/bash
cd WT/scratch/rv101_t6s_01 && GIT_OPTIONAL_LOCKS=0 VENV/bin/python WT/scratch/rv101_t6s_01/tools/dispatcher_oracle.py WT/rv101/cand/projects/chirality-piping WT/rv101/base/projects/chirality-piping WT/t6-outputs 2033260c57 WT/scratch/rv101_t6s_01/out/dispatcher2 WT/scratch/rv101_t6s_01/out/so2_docs WT/scratch/rv101_t6s_01/out/r4_docs > WT/scratch/rv101_t6s_01/logs/dispatcher2.log 2>&1
echo "rc=$?" >> WT/scratch/rv101_t6s_01/logs/dispatcher2.log
