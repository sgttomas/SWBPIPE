#!/bin/zsh
# I67 U6d post-U6f round 04: controls on the frozen candidate, in sequence (scratch only).
set -u
T3=WT; S=$T3/scratch/i67_u6d
REPO=REPO_ROOT
export TMPDIR=$S/tmp
WTD=$T3/f2a-carriers/projects/chirality-piping/apps/desktop
rsync -a --delete --exclude node_modules --exclude public $WTD/src/ $S/lanes/mut4/projects/chirality-piping/apps/desktop/src/
diff -rq $WTD/src $S/lanes/mut4/projects/chirality-piping/apps/desktop/src > $S/r4/lane_mut4_vs_worktree.txt
$S/run_suite.sh $WTD $S/r4/cand
pgrep -f memguard.sh >/dev/null || { echo NOGUARD > $S/r4/py.log; exit 9; }
(cd $T3/f2a-carriers/projects/chirality-piping && PYTHONDONTWRITEBYTECODE=1 \
  OPENPIPESTRESS_CHECKED_JSON_BIN=$T3/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson \
  OPENPIPESTRESS_UNITS_BIN=$T3/targets/i52-readers/units/release/openpipestress_units \
  perl -e 'alarm shift; exec @ARGV' 1200 $REPO/projects/chirality-piping/.venv/bin/python -m pytest -q -p no:cacheprovider \
  tests/test_retained_precision_carriers.py -k "shared or declared" > $S/r4/py.log 2>&1; echo "exit=$?" >> $S/r4/py.log)
(cd $S/mut && python3 mutants_r4.py mutants_r4.json > mutants_r4.log 2>&1)
echo done > $S/r4/done
