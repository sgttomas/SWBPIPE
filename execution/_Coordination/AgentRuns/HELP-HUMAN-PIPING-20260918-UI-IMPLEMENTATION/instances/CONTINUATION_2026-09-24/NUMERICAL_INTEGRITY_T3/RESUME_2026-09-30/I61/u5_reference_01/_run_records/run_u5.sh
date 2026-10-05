#!/bin/bash
# I61 U5: the oracle comparison (records/test lane; no product code, no cargo).
# Placeholders: WT (the t3 workspace), NUM (the numerics worktree), R (NUM's RESUME_2026-09-30).
set -eu
U=WT/scratch/i61_u5_reference_01
# Inputs: U1's pinned successor bytes (written by PP's committed u3_permitted_path_publishes_the_pinned_successor
# with I61_U3_OUT; byte-identical to the stub-archive facade-dispatch output).
OPENPIPESTRESS_CHECKED_JSON_BIN=WT/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson \
OPENPIPESTRESS_UNITS_BIN=WT/targets/i52-readers/units/release/openpipestress_units \
perl -e 'alarm shift; exec @ARGV' 900 python3 R/I61/u5_reference_01/_run_records/u5_compare.py \
  R/I50/first_publishing_component_02/named_oracle.py \
  WT/scratch/i50_first_publishing/runtime02/pp_debug_final.log \
  WT/f2a-facade/projects/chirality-piping \
  $U/u5_report.json $U/u3_successor_sparse_interactive.json $U/u3_successor_dense_scrutiny.json
