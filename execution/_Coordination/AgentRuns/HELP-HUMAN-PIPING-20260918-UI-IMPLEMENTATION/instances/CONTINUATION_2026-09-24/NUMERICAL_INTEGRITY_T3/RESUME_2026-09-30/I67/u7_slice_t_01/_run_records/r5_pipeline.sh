#!/bin/zsh
# I67 U7 slice T: controls on the frozen candidate, in sequence (scratch only).
set -u
T3=WT; S=$T3/scratch/i67_u6d
export TMPDIR=$S/tmp
WTD=$T3/f2a-u7/projects/chirality-piping/apps/desktop
rsync -a --delete --exclude node_modules --exclude public $WTD/src/ $S/lanes/mut5/projects/chirality-piping/apps/desktop/src/
diff -rq $WTD/src $S/lanes/mut5/projects/chirality-piping/apps/desktop/src > $S/r5/lane_mut5_vs_worktree.txt
$S/run_suite.sh $WTD $S/r5/cand
(cd $S/mut && python3 mutants_r5.py mutants_r5.json > mutants_r5.log 2>&1)
echo done > $S/r5/done
