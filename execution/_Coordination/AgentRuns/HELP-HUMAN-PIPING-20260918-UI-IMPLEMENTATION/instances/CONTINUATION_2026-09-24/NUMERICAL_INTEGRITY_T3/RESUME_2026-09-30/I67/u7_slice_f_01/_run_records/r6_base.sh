#!/bin/zsh
S=WT/scratch/i67_u6d
$S/run_suite.sh $S/lanes/base6/projects/chirality-piping/apps/desktop $S/r6/base
$S/run_suite.sh $S/lanes/part1/projects/chirality-piping/apps/desktop $S/r6/part1
echo done > $S/r6/bases.done
