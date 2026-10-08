#!/bin/bash
# RV113: the control and every mutant in MUTANTS.json, each as its own locked job (run_py_mutants.sh).
S=WT/scratch/rv113_rvr_01
sel=${1:-b1}
ids=(NONE $(python3 -c "import json;print(' '.join(m['id'] for m in json.load(open('$S/py/mutants/MUTANTS.json'))))"))
echo "${#ids[@]} runs: ${ids[*]}"
exec "$S/tools/run_py_mutants.sh" "$sel" "${ids[@]}"
