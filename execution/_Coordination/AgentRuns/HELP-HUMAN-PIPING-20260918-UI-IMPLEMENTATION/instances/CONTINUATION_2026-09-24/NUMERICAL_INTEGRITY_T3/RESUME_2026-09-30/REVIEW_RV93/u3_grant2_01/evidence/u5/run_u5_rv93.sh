#!/bin/bash
# RV93: U5 on the successor documents RV93 obtained from the actual Direct entry (registered build).
# Placeholders: WT (the t3 workspace), R (NUM's RESUME_2026-09-30), S = WT/scratch/rv93_u3_grant2_01.
set -eu
S=WT/scratch/rv93_u3_grant2_01
python3 R/REVIEW_RV86/u5_reference_01/_run_records/make_extract.py WT/scratch/i50_first_publishing/runtime02/pp_debug_final.log $S/u5/extract_regenerated.log
cmp $S/u5/extract_regenerated.log R/REVIEW_RV86/u5_reference_01/_run_records/i50_oracle_inputs_extract.log
OPENPIPESTRESS_CHECKED_JSON_BIN=WT/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson \
OPENPIPESTRESS_UNITS_BIN=WT/targets/i52-readers/units/release/openpipestress_units TMPDIR=$S/tmp \
perl -e 'alarm shift; exec @ARGV' 900 python3 R/I61/u5_reference_01/_run_records/u5_compare.py \
  R/I50/first_publishing_component_02/named_oracle.py R/REVIEW_RV86/u5_reference_01/_run_records/i50_oracle_inputs_extract.log \
  WT/rv93/cand/projects/chirality-piping $S/u5/u5_report.json \
  $S/out_reg/u1_milestone_sparse_interactive.json $S/out_reg/u1_milestone_dense_scrutiny.json > $S/u5/u5_run.log 2>&1
cmp $S/u5/u5_report.json R/I61/u5_reference_01/_run_records/u5_report.json
cmp $S/u5/u5_run.log R/I61/u5_reference_01/_run_records/u5_run.log
