#!/bin/bash
# RV113: my copies lacked P/execution/; the two handoff test files read PKG-15's working files from it (identical at
# I1 and the head). Add that folder (git archive, read-only) to both copies, then rerun those two files under the lock.
WT=WT
S=$WT/scratch/rv113_rvr_01
PKG="projects/chirality-piping/execution/PKG-15_Handoff and External Prover Workflow"
for pair in "262bd687f0 py-i1" "11cc14e3e65363790f43943b09933a57f4da28e4 py-head"; do
  set -- $pair
  (cd $WT/b1-p && GIT_OPTIONAL_LOCKS=0 git archive --format=tar "$1" -- "$PKG") | tar -x -C "$WT/rv113/$2"
  echo "$2: $(find "$WT/rv113/$2/$PKG" -type f | wc -l) files"
done
for v in i1 head; do
  P=$WT/rv113/py-$v/projects/chirality-piping
  $S/tools/run_py_job.sh suite_${v}_handoff $P -m pytest -p no:cacheprovider --basetemp=$S/py/tmp/suite_${v}_handoff/basetemp -q -rs -rf -v --junitxml=$S/py/logs/suite_${v}_handoff.xml tests/test_handoff_package_schema.py tests/test_handoff_export_workflow.py
  echo "handoff $v rc=$?"
done
