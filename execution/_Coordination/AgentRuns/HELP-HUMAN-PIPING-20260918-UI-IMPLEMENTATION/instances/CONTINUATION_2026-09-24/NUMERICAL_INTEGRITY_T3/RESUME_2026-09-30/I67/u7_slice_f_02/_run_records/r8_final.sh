#!/bin/zsh
S=WT/scratch/i67_u6d
$S/run_suite.sh $S/lanes/cand8/projects/chirality-piping/apps/desktop $S/r8/final
echo done > $S/r8/final.done
