#!/bin/bash
# RV86: the review's runs (records/test lane; Python only; no Cargo, solver, native or DEC-025 jobs).
# Placeholders: WT (the t3 workspace), NUM (the numerics worktree at fc5d92c56c), R (NUM's RESUME_2026-09-30).
# The reader shells out to I52's prebuilt checked-JSON and units binaries (hashes in I61's inputs_sha256.txt).
set -eu
U=WT/scratch/i61_u5_reference_01; O=WT/scratch/rv86_u5_reference_01; C=R/REVIEW_RV86/u5_reference_01/_run_records
export OPENPIPESTRESS_CHECKED_JSON_BIN=WT/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson
export OPENPIPESTRESS_UNITS_BIN=WT/targets/i52-readers/units/release/openpipestress_units
export PYTHONDONTWRITEBYTECODE=1
ORACLE=R/I50/first_publishing_component_02/named_oracle.py; LOG=WT/scratch/i50_first_publishing/runtime02/pp_debug_final.log
SCRIPT=R/I61/u5_reference_01/_run_records/u5_compare.py; SUCC="$U/u3_successor_sparse_interactive.json $U/u3_successor_dense_scrutiny.json"
# 0. Unchanged rerun, with the NUM reader root and with the recorded WT/f2a-facade reader root (identical reader bytes).
python3 -B $SCRIPT $ORACLE $LOG NUM/projects/chirality-piping $O/rerun_num_report.json $SUCC > $O/rerun_num_run.log
python3 -B $SCRIPT $ORACLE $LOG WT/f2a-facade/projects/chirality-piping $O/rerun_f2a_report.json $SUCC > $O/rerun_f2a_run.log
# 1-5. Independent checks.
python3 -B $C/c1_slices.py $ORACLE $SCRIPT > $C/c1_slices.out
python3 -B $C/c2_classes.py $SUCC $O/rerun_num_report.json > $C/c2_classes.out
python3 -B $C/c3_truths.py $SUCC $O/rerun_num_report.json > $C/c3_truths.out      # also writes c3_truths.rows.json beside itself
python3 -B $C/c4_controls.py $SCRIPT $ORACLE $LOG NUM/projects/chirality-piping $O/controls $SUCC > $C/c4_controls.out
python3 -B $C/c4b_abs_edge.py $SCRIPT $ORACLE $LOG NUM/projects/chirality-piping $O/controls $SUCC > $C/c4b_abs_edge.out
python3 -B $C/c5_ancillary_extrema.py $SUCC > $C/c5_ancillary_extrema.out
# 6. Derive the extract (make_extract.py), then replay from the 7,240-byte extract of the I50 log (mode, request, source, facts per record), via a
#    copy of the script whose only change is the pinned input hash (u5_compare_extract_variant.diff).
python3 -B $C/make_extract.py $LOG $O/extract_check.log   # prints a66a8a49..., equal to $C/i50_oracle_inputs_extract.log
python3 -B $O/u5_compare_extract_variant.py $ORACLE $C/i50_oracle_inputs_extract.log NUM/projects/chirality-piping $O/extract_report.json $SUCC > $O/extract_run.log
