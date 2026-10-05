#!/bin/bash
T=WT
S=$T/scratch/rv89_u4_g5; R=$T/rv89; TG=$T/targets/rv89
for side in base cand; do
  $S/run.sh ${side}_runner $R/$side/projects/chirality-piping/core/runner/headless/Cargo.toml $TG/${side}-runner
  $S/run.sh ${side}_rx $R/$side/projects/chirality-piping/core/reporting/result_export/Cargo.toml $TG/${side}-rx
done
echo PHASE2 DONE
