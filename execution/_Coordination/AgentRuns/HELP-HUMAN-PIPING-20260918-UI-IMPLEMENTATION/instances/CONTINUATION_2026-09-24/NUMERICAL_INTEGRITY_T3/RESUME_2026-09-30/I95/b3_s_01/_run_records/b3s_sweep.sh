#!/bin/bash
# I95 B3-S: every priced point (D1's model caps, a = c, L = c*l): I82's chain (mc_chain) and the four variants.
# Usage: I95_WT=<WT> b3s_sweep.sh > sweep.log
set -euo pipefail
T=${I95_WT:?}; S=$T/scratch/i95_b3_s
for ch in mc_chain ur urc er erc; do
  for c in 1 2 3; do
    CHAIN=$ch $S/tools/b3s_run.sh c$c "{\"l\": 128, \"c\": $c}" 2>/dev/null | tail -1
  done
done
