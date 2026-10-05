#!/bin/zsh
S=WT/scratch/i67_u6d
$S/run_suite.sh $S/lanes/cand7/projects/chirality-piping/apps/desktop $S/r7/final
echo done > $S/r7/final.done
