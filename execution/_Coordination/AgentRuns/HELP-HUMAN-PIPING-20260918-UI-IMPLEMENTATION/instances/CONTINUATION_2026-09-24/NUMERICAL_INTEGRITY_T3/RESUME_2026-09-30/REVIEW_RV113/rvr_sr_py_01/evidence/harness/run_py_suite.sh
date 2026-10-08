#!/bin/bash
# Usage: run_py_suite.sh <label> <P root>   -- I83's 27-file Python sweep, one locked pytest job, junit to py/logs/<label>.xml
S=WT/scratch/rv113_rvr_01
label=$1; P=$2
mapfile -t FILES < "$S/tools/py_sweep_files.txt" 2>/dev/null || FILES=($(cat "$S/tools/py_sweep_files.txt"))
exec "$S/tools/run_py_job.sh" "$label" "$P" -m pytest -p no:cacheprovider --basetemp="$S/py/tmp/$label/basetemp" -q -rs -rf -v --junitxml="$S/py/logs/$label.xml" "${FILES[@]}"
