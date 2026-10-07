#!/bin/bash
# I82 B1-S: the disposable basis snapshot the Python chain reads (Git reads only; no checkout, no Git write).
# Extracts main d8c88774d0's projects/chirality-piping/{core,schemas,fixtures} into WT/scratch/i82_b1_study/snap.
# Run before b1_text_base.sh; delete the snapshot afterwards (rm -rf WT/scratch/i82_b1_study/snap).
# Usage: I82_WT=<WT> b1_snapshot.sh
set -euo pipefail
T=${I82_WT:?}; S=$T/scratch/i82_b1_study
rm -rf $S/snap; mkdir -p $S/snap $S/tmp
cd $T/numerics && GIT_OPTIONAL_LOCKS=0 git archive d8c88774d0a73bc99fe9c6e906db6296162f8953 \
  projects/chirality-piping/core projects/chirality-piping/schemas projects/chirality-piping/fixtures | tar -x -C $S/snap
