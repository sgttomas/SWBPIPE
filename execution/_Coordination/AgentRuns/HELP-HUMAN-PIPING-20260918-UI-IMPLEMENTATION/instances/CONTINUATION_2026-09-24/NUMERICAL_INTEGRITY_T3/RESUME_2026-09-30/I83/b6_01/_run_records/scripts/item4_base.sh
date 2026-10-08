#!/bin/bash
# Item 4 before/after: the R34 corpus mutants on 07l against the BASE Python harness (which reads `expected`).
set -u
source "$(dirname "$0")/env.sh"
MB=$S/mutbase/projects/chirality-piping; C=$MB/fixtures/results/retained_precision_cases.json
cp $S/base/projects/chirality-piping/fixtures/results/retained_precision_cases.json $C.orig
for mid in C1 C2; do
  $VENV/bin/python - "$C" "$mid" <<'PY'
import json, sys
p, mid = sys.argv[1], sys.argv[2]
c = json.loads(open(p + ".orig").read())
i, code = (277, "SOURCE_NUMERICAL_QUALITY_INVALID") if mid == "C1" else (139, "SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS")
c["mutations"][i]["expected_by_reader"]["python"] = {"gate": "G7", "code": code}
open(p, "w").write(json.dumps(c, indent=2) + "\n")
PY
  cd $MB && /usr/bin/lockf -k $WT/guard/cargo_job.lock $VENV/bin/python -m pytest -p no:cacheprovider --basetemp=$S/tmp/mutbase_$mid -q -rf tests/test_retained_precision_contract.py -k "snapshot_07 or (first_failure_controls and g7)" > $S/logs/mutants/base_harness_$mid.log 2>&1
  echo "$mid base-harness rc=$?" >> $S/logs/mutants/base_harness.txt
done
cp $C.orig $C && rm $C.orig
