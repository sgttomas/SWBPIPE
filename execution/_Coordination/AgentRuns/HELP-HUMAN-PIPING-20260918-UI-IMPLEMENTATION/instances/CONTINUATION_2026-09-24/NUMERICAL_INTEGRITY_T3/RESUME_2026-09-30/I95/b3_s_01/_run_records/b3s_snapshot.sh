#!/bin/bash
# I95 B3-S: the disposable basis snapshot (I82's b1_snapshot.sh with the basis and scratch rebound).
# Extracts main 2007709549's projects/chirality-piping/{core,schemas,fixtures} into WT/scratch/i95_b3_s/snap.
# Git reads only (GIT_OPTIONAL_LOCKS=0); no checkout, no Git write.
# Usage: I95_WT=<WT> b3s_snapshot.sh
set -euo pipefail
T=${I95_WT:?}; S=$T/scratch/i95_b3_s
rm -rf $S/snap; mkdir -p $S/snap $S/tmp
cd $T/numerics && GIT_OPTIONAL_LOCKS=0 git archive 2007709549e9701b302e0eb62a1474d82acc1c40 \
  projects/chirality-piping/core projects/chirality-piping/schemas projects/chirality-piping/fixtures | tar -x -C $S/snap
