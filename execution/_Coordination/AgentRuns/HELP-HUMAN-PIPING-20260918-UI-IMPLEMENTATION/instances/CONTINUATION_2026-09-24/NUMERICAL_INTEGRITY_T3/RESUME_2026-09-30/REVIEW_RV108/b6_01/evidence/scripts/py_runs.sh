#!/bin/bash
source S/env.sh
export OPENPIPESTRESS_CHECKED_JSON_BIN=$WT/targets/rv108-b6/checked-json/release/openpipestress_jcs_ijson OPENPIPESTRESS_UNITS_BIN=$WT/targets/rv108-b6/units-authority/release/openpipestress_units PYTHONDONTWRITEBYTECODE=1
cd $S
for job in "$@"; do
  which=${job%%:*}; what=${job##*:}
  $VENV/bin/python $S/scripts/run_py.py $WT/rv108/$which/$P $S/probes/$what.json $S/out/py_${which}_$what.jsonl > $S/logs/py_${which}_$what.log 2>&1
  echo "rc=$?" >> $S/logs/py_${which}_$what.log
done
