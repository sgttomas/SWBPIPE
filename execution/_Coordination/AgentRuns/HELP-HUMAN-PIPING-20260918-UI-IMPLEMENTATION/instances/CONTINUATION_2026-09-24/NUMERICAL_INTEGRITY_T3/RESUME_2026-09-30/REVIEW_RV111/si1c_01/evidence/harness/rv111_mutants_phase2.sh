#!/bin/bash
# RV111: phase 2 of the mutant programme (a trimmed set, after lock contention made the full set too slow),
# then RV111's differential on every survivor. Every cargo command goes through t3_cargo.sh (rv111_mutants.py).
WT=WT
S=$WT/scratch/rv111_si1c_01
VENV=VENV
export TMPDIR=$S/tmp
cd "$S" || exit 2
"$VENV/bin/python" "$S/harness/rv111_mutants.py" --only I07_four_unchecked_jointly,N02_raw_other_unit_supplied,N03_note_dropped,N04_value_kept,N06_dedup_removed,N07_block_after_mode_dispatch,N08_unreferenced_input_blocks,N09_trim_dropped,L01_limit_raw_check_removed,L02_limit_normalized_check_removed,L04_limit_as_missing,X01_step_nan_check_removed,X02_interpolate_nan_check_removed,X03_ratio_check_infinite_only,X04_nan_argument_code_out_of_range
echo "phase2 kill run rc=$?"
survivors=$("$VENV/bin/python" -c "
import json; r=json.load(open('$S/mutants/mutants.json'))
print(','.join(sorted(k for k,v in r.items() if v.get('killed') is False)))")
echo "survivors: $survivors"
"$VENV/bin/python" "$S/harness/rv111_mutants.py" --diff "$survivors"
echo "diff run rc=$?"
