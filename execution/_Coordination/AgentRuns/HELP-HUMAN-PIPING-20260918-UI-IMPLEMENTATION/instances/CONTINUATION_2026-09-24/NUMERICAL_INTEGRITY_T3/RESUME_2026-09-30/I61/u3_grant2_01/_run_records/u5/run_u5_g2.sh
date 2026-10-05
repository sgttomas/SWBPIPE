#!/bin/bash
# I61 U3 grant 2, deliverable 6: U5 rerun (records/test lane; no product code, no cargo).
# Placeholders: WT (the t3 workspace), R (NUM's RESUME_2026-09-30).
# Inputs: the successor bytes the actual Direct entry published in the registered build, written by
# the committed u3g2_direct_entry_publishes_the_pinned_successor with I61_U3G2_OUT; RV86's committed
# 7,240-byte extract; u5_compare.py with RV86's two-line extract pin (otherwise unchanged).
set -eu
S=WT/scratch/i61_u3_grant2_01
# The extract still derives from I50's hash-pinned full log (make_extract.py asserts 5ced66b5...).
python3 R/REVIEW_RV86/u5_reference_01/_run_records/make_extract.py WT/scratch/i50_first_publishing/runtime02/pp_debug_final.log $S/u5/extract_regenerated.log
cmp $S/u5/extract_regenerated.log R/REVIEW_RV86/u5_reference_01/_run_records/i50_oracle_inputs_extract.log
OPENPIPESTRESS_CHECKED_JSON_BIN=WT/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson \
OPENPIPESTRESS_UNITS_BIN=WT/targets/i52-readers/units/release/openpipestress_units \
perl -e 'alarm shift; exec @ARGV' 900 python3 R/I61/u5_reference_01/_run_records/u5_compare.py \
  R/I50/first_publishing_component_02/named_oracle.py \
  R/REVIEW_RV86/u5_reference_01/_run_records/i50_oracle_inputs_extract.log \
  WT/f2a-memory/projects/chirality-piping \
  $S/u5/u5_report.json $S/out/reg/u3g2_successor_sparse_interactive.json $S/out/reg/u3g2_successor_dense_scrutiny.json > $S/u5/u5_run.log 2>&1
cmp $S/u5/u5_report.json R/I61/u5_reference_01/_run_records/u5_report.json
cmp $S/u5/u5_run.log R/I61/u5_reference_01/_run_records/u5_run.log
