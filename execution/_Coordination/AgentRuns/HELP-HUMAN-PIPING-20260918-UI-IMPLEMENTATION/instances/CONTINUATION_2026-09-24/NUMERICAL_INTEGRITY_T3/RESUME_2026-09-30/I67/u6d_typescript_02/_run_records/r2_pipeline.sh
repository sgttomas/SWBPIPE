#!/bin/zsh
# I67 U6d repair round 02: controls on the frozen repaired candidate, in sequence (scratch only).
set -u
T3=WT; S=$T3/scratch/i67_u6d
export TMPDIR=$S/tmp
WTD=$T3/f2a-carriers-ts/projects/chirality-piping/apps/desktop
rsync -a --delete --exclude node_modules --exclude public $WTD/src/ $S/lanes/mut2/projects/chirality-piping/apps/desktop/src/
diff -rq $WTD/src $S/lanes/mut2/projects/chirality-piping/apps/desktop/src > $S/r2/lane_mut2_vs_worktree.txt
$S/run_suite.sh $WTD $S/r2/cand
(cd $S/mut && python3 mutants_r2.py mutants_r2.json > mutants_r2.log 2>&1)
echo done > $S/r2/done
