#!/bin/bash
# I88 repair round 1, mutants part 2 (scratch tool): the runner mutants first (RV111's N06 and N09
# among them), then SI1b's remaining mutants without new equivalence dumps (the evaluator's non-test
# code is byte-identical to 7f233b2e01, so round 0's dump evidence stands for them).
set -u
S=WT/scratch/i88_si1c
export TMPDIR=$S/tmp SI1C_CAND_DUMPS=$S/dumps/cand
cd $S
python3 harness/si1c_mutants.py $S/reports/r1_mutants_part2a.json \
  RV111_N06_dedup_removed RV111_N09_trim_dropped N11_note_replaces_an_existing_note N12_note_prepended \
  N1_n4_input_check_removed N2_n4_keeps_the_value N3_limit_message_reverted N4_n4_blocks_an_unreferenced_input \
  N5_raw_value_not_tested_before_normalization N6_raw_non_finite_in_another_unit_supplied N7_note_not_set \
  N8_limit_raw_check_removed N9_limit_normalized_check_removed N10_formula_block_missing_returns_to_evaluator \
  > $S/logs/r1/mutants_part2a.log 2>&1
SI1C_SKIP_EQUIV="carried from round 0: EE non-test code identical to 7f233b2e01" \
python3 harness/si1c_mutants.py $S/reports/r1_mutants_part2b.json \
  SI1B_I79_T1_nan_argument_code_out_of_range SI1B_I79_T2_exact_lookup_also_blocks_nan \
  SI1B_I79_Q4_quotient_check_before_unit_check SI1B_RV104_R1_ratio_check_nan_only \
  SI1B_RV104_R2_overblock_dimensionless_divisor SI1B_RV104_R3_overblock_derived_quotient \
  SI1B_RV104_R4_ratio_subject_changed SI1B_RV104_R5_ratio_message_changed SI1B_RV104_R6_ratio_block_continues_with_zero \
  SI1B_RV104_S1_step_subject_changed SI1B_RV104_S2_step_pushes_but_continues SI1B_RV104_I1_interpolate_nan_silent_first_row \
  SI1B_RV104_I2_interpolate_pushes_but_continues SI1B_RV104_N1_nan_message_changed \
  > $S/logs/r1/mutants_part2b.log 2>&1
