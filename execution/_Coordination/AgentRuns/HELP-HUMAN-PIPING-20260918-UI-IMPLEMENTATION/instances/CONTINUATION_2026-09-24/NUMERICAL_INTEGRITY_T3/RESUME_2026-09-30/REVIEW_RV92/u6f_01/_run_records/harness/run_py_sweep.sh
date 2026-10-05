#!/bin/zsh
WT=WT; S=$WT/scratch/rv92_u6f
VENV=REPO_ROOT/projects/chirality-piping/.venv
export TMPDIR=$S/tmp PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN=$WT/targets/rv92/cli/release/openpipestress_jcs_ijson OPENPIPESTRESS_UNITS_BIN=$WT/targets/rv92/cli/release/openpipestress_units
L=$1
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
cd $WT/rv92/$L/projects/chirality-piping && $VENV/bin/python $S/rv92_py_sweep.py $WT/rv92/$L/projects/chirality-piping $S/sweep_inputs $S/sweep_py_$L.jsonl > $S/sweep_py_$L.log 2>&1; echo "exit=$?" >> $S/sweep_py_$L.log
