#!/bin/zsh
# I67 U6d follow-up round 03 on the merged carriers head: controls in sequence (scratch only).
set -u
T3=WT; S=$T3/scratch/i67_u6d
REPO=REPO_ROOT
export TMPDIR=$S/tmp
WTD=$T3/f2a-carriers/projects/chirality-piping/apps/desktop
rsync -a --delete --exclude node_modules --exclude public $WTD/src/ $S/lanes/mut3/projects/chirality-piping/apps/desktop/src/
diff -rq $WTD/src $S/lanes/mut3/projects/chirality-piping/apps/desktop/src > $S/r3/lane_mut3_vs_worktree.txt
# 1. The whole desktop suite and tsc on the candidate.
$S/run_suite.sh $WTD $S/r3/cand
# 2. Python's shared-file consumers on the same tree (parity cross-check; read-only run, no bytecode).
pgrep -f memguard.sh >/dev/null || { echo NOGUARD > $S/r3/py.log; exit 9; }
(cd $T3/f2a-carriers/projects/chirality-piping && PYTHONDONTWRITEBYTECODE=1 \
  OPENPIPESTRESS_CHECKED_JSON_BIN=$T3/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson \
  OPENPIPESTRESS_UNITS_BIN=$T3/targets/i52-readers/units/release/openpipestress_units \
  perl -e 'alarm shift; exec @ARGV' 1200 $REPO/projects/chirality-piping/.venv/bin/python -m pytest -q -p no:cacheprovider \
  tests/test_retained_precision_carriers.py -k "shared or declared" > $S/r3/py.log 2>&1; echo "exit=$?" >> $S/r3/py.log)
# 3. The mutant programme on the frozen candidate.
(cd $S/mut && python3 mutants_r3.py mutants_r3.json > mutants_r3.log 2>&1)
echo done > $S/r3/done
