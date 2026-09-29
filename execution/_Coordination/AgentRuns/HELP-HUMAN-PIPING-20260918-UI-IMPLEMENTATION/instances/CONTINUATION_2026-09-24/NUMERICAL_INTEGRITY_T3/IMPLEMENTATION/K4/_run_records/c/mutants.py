"""K4 checkpoint C: the mutants (exact source edits) and their intended kills."""
ALL_TESTS = [
    'adaptive::classification_tests::input_derived_rows_and_s8_ws_far_node_rows_are_classified_as_the_design_says',
    'adaptive::classification_tests::item_6a_floors_force_and_moment_by_phi_where_it_binds_and_nowhere_else',
    'adaptive::classification_tests::stress_scales_and_intensified_factors_match_bit_for_bit',
    'adaptive::classification_tests::the_classification_matches_the_binary64_reimplementation_bit_for_bit',
    'adaptive::classification_tests::the_classifications_boundaries_hold_exactly',
    'adaptive::method_tests::directional_span_under_5a2_was_selected_at_256_and_published_within_its_claim',
    'adaptive::method_tests::e_charge_and_e_estimate_equal_the_emulation_at_every_verification',
    'adaptive::method_tests::e_charge_on_rf_large_at_100_members_at_256',
    'adaptive::method_tests::e_headroom_every_state_pair_is_within_2_to_the_8_of_its_resolution',
    'adaptive::method_tests::every_control_follows_gens_schedule_and_r7s_expectations_honestly',
    'adaptive::method_tests::sd_g5_charge_boundaries_at_p_256_and_p_512_and_the_translation_row',
    'adaptive::method_tests::sd_g5_decides_each_test_exactly_at_its_boundary_in_r7s_order',
    'adaptive::method_tests::sd_g5_lever2s_estimate_residual_is_nonzero_where_the_assembled_one_vanishes',
    'adaptive::method_tests::sd_g5_the_g_check_passes_at_2_to_the_p_minus_16_and_fails_above',
    'adaptive::method_tests::sd_g5_the_gates_bounded_test_at_its_boundary_and_the_best_state',
    'adaptive::references_tests::b1_l_is_exact_with_the_ledger',
    'adaptive::references_tests::k_d5s_d5c_1_controls_and_rf_skew_t_cant_off_122_r1e_04_are_within_1e_minus_9',
    'adaptive::references_tests::n05_and_n06_agree_with_the_intended_basis_and_n05_is_not_np_as_represented_answer',
    'adaptive::references_tests::r1s_cases_through_the_adapter_pass_are_refused_or_are_not_covered_as_section_4_10_lists',
    'adaptive::references_tests::rf_large_at_10_members_is_selected_at_128_and_honest',
    'adaptive::references_tests::rf_large_chain_ax_at_100_members_is_selected_at_128_and_honest',
    'adaptive::references_tests::rf_large_chain_rot_at_100_members_is_selected_at_128_and_honest',
    'adaptive::references_tests::rf_large_cont_ax_at_100_members_is_selected_at_128_and_honest',
    'adaptive::references_tests::rf_large_cont_rot_at_100_members_is_selected_at_128_and_honest',
    'adaptive::references_tests::rf_large_tree_ax_at_100_members_is_selected_at_128_and_honest',
    'adaptive::references_tests::rf_large_tree_rot_at_100_members_is_selected_at_128_and_honest',
    'adaptive::references_tests::the_spring_carried_case_is_evaluated_its_underflowing_actions_published_explicitly',
    'adaptive::references_tests::w1_agrees_with_the_exact_block_oracle_on_the_in_scope_cases',
    'adaptive::scale_tests::blocks_are_the_free_free_components_and_data_follows_7a',
    'adaptive::scale_tests::e_uc_on_rf_large_at_100_members_at_256',
    'adaptive::scale_tests::e_uc_uc_s_and_b_equal_the_emulation_and_never_fall_below_the_norm_per_block',
    'adaptive::scale_tests::e_unit_g_the_bounded_operator_and_e_equal_the_generators_emulation_at_every_precision',
    'adaptive::scale_tests::e_unit_on_rf_large_at_100_members_at_128_and_256',
    'adaptive::scale_tests::factors_l_and_d_bits_are_pinned_and_the_shifted_loop_without_a_shift_reproduces_them',
    'adaptive::tests::a_case_limit_is_exhausted_exactly_where_its_work_ends',
    'adaptive::tests::a_ceiling_case_is_unresolved_with_its_attempts',
    'adaptive::tests::a_structural_zero_kind_is_accepted_through_the_coupled_scale',
    'adaptive::tests::a_sums_rounding_charges_the_limbs_it_rounds',
    'adaptive::tests::an_all_zero_body_agrees_exactly',
    'adaptive::tests::every_published_quantity_takes_part_in_the_stop_rule',
    'adaptive::tests::factor_reuse_is_bit_identical_to_separate_solves',
    'adaptive::tests::failed_and_verification_attempts_are_charged',
    'adaptive::tests::golden_work_counts',
    'adaptive::tests::k4s_files_call_no_binary64_transcendental_or_fused_function',
    'adaptive::tests::k_1e_minus_28_escalates_past_128_and_is_accepted_at_256_against_512',
    'adaptive::tests::n05_and_n06_are_accepted_at_128_against_256',
    'adaptive::tests::probe_v4_s3_y_reference_with_a_chord_component',
    'adaptive::tests::refinement_stops_after_three_corrections_and_the_attempt_escalates',
    'adaptive::tests::retained_states_equal_the_generators_emulation_of_the_method_bit_for_bit',
    'adaptive::tests::runs_and_list_permutations_are_deterministic',
    'adaptive::tests::the_predicate_is_decided_exactly_at_its_boundary',
    'adaptive::tests::the_reactions_only_control_is_rejected_at_128_by_a_reaction',
    'adaptive::tests::the_six_member_skew_run_is_rejected_at_128_and_its_verification_is_reused',
    'assemble::tests::a_prescribed_motion_control_matches_its_exact_reference',
    'assemble::tests::assembled_entries_equal_the_fraction_emulation_bit_for_bit',
    'assemble::tests::at_128_the_element_agrees_with_k_d5s_re_formation_within_64_ulps_of_its_largest_entry',
    'assemble::tests::at_p53_the_axis_aligned_element_agrees_with_the_products_local_stiffness',
    'assemble::tests::every_element_annihilates_the_rigid_motions_to_within_64_units_of_2_to_minus_p',
    'assemble::tests::permuting_every_list_gives_bit_identical_k_rhs_and_encodings',
    'assemble::tests::the_duplicate_operand_entry_is_one_exact_sum_where_a_sequential_fold_loses_the_weak_member',
    'assemble::tests::the_pattern_from_positions_equals_the_connectivity_pattern_where_both_apply',
    'assemble::tests::the_reduced_rhs_enters_prescribed_columns_exactly_and_rounds_once',
    'bound::tests::sd_g5s_searched_profiles_reach_each_boundary_of_7b_to_7d',
    'bound::tests::the_low_precision_stress_keeps_every_bound_above_the_exact_norm_and_m27s_does_not',
    'bound::tests::the_product_of_two_exact_sums_is_exact',
    'bound::tests::v4s_f2_family_bounds_equal_the_emulation_and_are_never_below_the_exact_norm',
    'combine::tests::a_combination_escalates_independently_of_its_operands',
    'combine::tests::a_combination_out_of_budget_is_withheld_and_its_operands_keep_their_standing',
    'combine::tests::a_combination_reuses_its_operands_cached_factor_stops_and_builds',
    'combine::tests::a_combination_runs_its_own_stop_rule',
    'combine::tests::a_combined_prescribed_value_is_published_from_its_exact_sum_rounded_once',
    'combine::tests::b1_c_the_cancelled_1e80_terms_leave_the_1e_minus_8_load_exactly',
    'combine::tests::b1_e_is_caught_the_combination_reproduces_the_truth_that_a_sum_of_states_loses',
    'combine::tests::ceiling_operands_2_to_the_minus_1060_apart_give_the_net_cases_truth',
    'combine::tests::invalid_combinations_are_withheld_with_their_reason',
    'combine::tests::prescribed_values_combine_exactly_and_round_once',
    'directed::tests::a_directed_result_is_never_on_the_wrong_side_and_exact_values_do_not_move',
    'directed::tests::directed_operations_equal_the_fraction_oracle_in_both_directions',
    'factor::tests::a_constructed_ill_conditioned_matrix_whose_pivots_pass_escalates_on_rcond_at_128_only',
    'factor::tests::a_constructed_indefinite_pair_is_a_negative_energy_witness_and_its_definite_neighbour_is_not',
    'factor::tests::a_directional_ground_that_does_not_span_is_not_assessed_and_is_never_refused_as_a_mechanism',
    'factor::tests::a_pivot_that_fails_at_128_escalates_and_passes_at_256',
    'factor::tests::the_ceiling_attempt_uses_its_own_k_as_the_residual_basis',
    'factor::tests::the_p_plus_64_residual_takes_one_correction_on_two_span_and_records_its_basis',
    'factor::tests::the_rcm_port_orders_sparse_directs_small_graphs_as_sparse_direct_does',
    'factor::tests::the_span_decision_is_exact',
    'factor::tests::witnessed_mechanisms_are_refused_before_any_attempt_and_the_rx_companion_solves',
    'ledger::tests::cancelling_contributions_are_exact_per_dof',
    'ledger::tests::canonical_encodings_equal_the_generators_independent_bytes',
    'ledger::tests::the_accessor_nets_once_with_the_quantum_and_a_zero_is_plus_zero',
    'ledger::tests::the_ledger_encoding_is_canonical_and_order_independent',
    'ledger::tests::the_ledger_enters_a_prescribed_coupled_rhs_exactly_even_when_its_tail_decides_a_tie',
    'ledger::tests::the_projection_at_p53_gives_round_on_normal_results_and_the_exact_projection_everywhere',
    'ledger::tests::the_projection_matches_the_fraction_oracle_at_every_precision',
    'recover::tests::a_recovered_action_is_one_exact_expansion_where_a_sequential_fold_loses_a_term',
    'recover::tests::an_exact_zero_is_published_as_plus_zero',
    'recover::tests::every_recovered_kind_matches_its_exact_reference',
    'recover::tests::n06s_torque_and_spring_action_are_nonzero_and_correct',
    'recover::tests::subnormal_underflow_and_overflow_are_published_with_their_outcome',
    'source::tests::a_valid_source_is_canonical_and_its_bodies_are_numbered_by_lowest_node',
    'source::tests::every_invalid_input_is_refused_with_its_reason',
    'source::tests::the_degenerate_axis_decision_is_exact',
    'source::tests::the_source_encodings_do_not_depend_on_list_order',
    'verify::tests::an_e_hat_that_overflows_while_e_encodes_stops_as_e_does',
    'verify::tests::e_hat_couples_force_and_moment_through_the_body_extent_in_binary64',
    'verify::tests::phi_is_exact_where_the_product_is_normal_and_rounded_up_below_2_to_the_minus_584',
    'wide_sum::tests::a_refused_span_adds_nothing',
    'wide_sum::tests::committed_vectors_match_their_recorded_sha256',
    'wide_sum::tests::rv12_counterexample_is_rounded_once_and_a_fold_misrounds_it',
    'wide_sum::tests::targeted_sums_match_the_fraction_oracle',
    'wide_sum::tests::zero_negation_absolute_value_and_reuse',
    'wide_sum::tests::n5_stream_p128_l4',
    'wide_sum::tests::n5_stream_p192_l4',
    'wide_sum::tests::n5_stream_p320_l8',
    'wide_sum::tests::n5_stream_p576_l16',
    'wide_sum::tests::sum_differential_p1024',
    'wide_sum::tests::sum_differential_p128',
    'wide_sum::tests::sum_differential_p192',
    'wide_sum::tests::sum_differential_p256',
    'wide_sum::tests::sum_differential_p320',
    'wide_sum::tests::sum_differential_p512',
    'wide_sum::tests::sum_differential_p576',
]
SLOW = set([
    'wide_sum::tests::n5_stream_p128_l4',
    'wide_sum::tests::n5_stream_p192_l4',
    'wide_sum::tests::n5_stream_p320_l8',
    'wide_sum::tests::n5_stream_p576_l16',
    'wide_sum::tests::sum_differential_p1024',
    'wide_sum::tests::sum_differential_p128',
    'wide_sum::tests::sum_differential_p192',
    'wide_sum::tests::sum_differential_p256',
    'wide_sum::tests::sum_differential_p320',
    'wide_sum::tests::sum_differential_p512',
    'wide_sum::tests::sum_differential_p576',
])
MUTANTS = {}


def m(name, what, edits, tests, expect='kill'):
    MUTANTS[name] = dict(what=what, edits=edits, tests=tests, expect=expect)


m('NONE', 'the unmutated candidate', [], [])

# ---------------------------------------------------------------- test names
CTRL = 'adaptive::method_tests::every_control_follows_gens_schedule_and_r7s_expectations_honestly'
ECH = 'adaptive::method_tests::e_charge_and_e_estimate_equal_the_emulation_at_every_verification'
ECH100 = 'adaptive::method_tests::e_charge_on_rf_large_at_100_members_at_256'
EHR = 'adaptive::method_tests::e_headroom_every_state_pair_is_within_2_to_the_8_of_its_resolution'
G5 = 'adaptive::method_tests::sd_g5_decides_each_test_exactly_at_its_boundary_in_r7s_order'
G5C = 'adaptive::method_tests::sd_g5_charge_boundaries_at_p_256_and_p_512_and_the_translation_row'
G5GATE = 'adaptive::method_tests::sd_g5_the_gates_bounded_test_at_its_boundary_and_the_best_state'
G5G = 'adaptive::method_tests::sd_g5_the_g_check_passes_at_2_to_the_p_minus_16_and_fails_above'
LEV = 'adaptive::method_tests::sd_g5_lever2s_estimate_residual_is_nonzero_where_the_assembled_one_vanishes'
DS52 = 'adaptive::method_tests::directional_span_under_5a2_was_selected_at_256_and_published_within_its_claim'
EUNIT = 'adaptive::scale_tests::e_unit_g_the_bounded_operator_and_e_equal_the_generators_emulation_at_every_precision'
EUC = 'adaptive::scale_tests::e_uc_uc_s_and_b_equal_the_emulation_and_never_fall_below_the_norm_per_block'
EUC100 = 'adaptive::scale_tests::e_uc_on_rf_large_at_100_members_at_256'
LOOP = 'adaptive::scale_tests::factors_l_and_d_bits_are_pinned_and_the_shifted_loop_without_a_shift_reproduces_them'
BLOCKS = 'adaptive::scale_tests::blocks_are_the_free_free_components_and_data_follows_7a'
RFL10 = 'adaptive::references_tests::rf_large_at_10_members_is_selected_at_128_and_honest'
RFL_CHAIN = 'adaptive::references_tests::rf_large_chain_ax_at_100_members_is_selected_at_128_and_honest'
RFL_TREE = 'adaptive::references_tests::rf_large_tree_ax_at_100_members_is_selected_at_128_and_honest'
R1 = 'adaptive::references_tests::r1s_cases_through_the_adapter_pass_are_refused_or_are_not_covered_as_section_4_10_lists'
NP = 'adaptive::references_tests::n05_and_n06_agree_with_the_intended_basis_and_n05_is_not_np_as_represented_answer'
B1L = 'adaptive::references_tests::b1_l_is_exact_with_the_ledger'
STRESS = 'bound::tests::the_low_precision_stress_keeps_every_bound_above_the_exact_norm_and_m27s_does_not'
F2FAM = 'bound::tests::v4s_f2_family_bounds_equal_the_emulation_and_are_never_below_the_exact_norm'
SEARCHED = 'bound::tests::sd_g5s_searched_profiles_reach_each_boundary_of_7b_to_7d'
ITEM6A = 'adaptive::classification_tests::item_6a_floors_force_and_moment_by_phi_where_it_binds_and_nowhere_else'
CLASSBITS = 'adaptive::classification_tests::the_classification_matches_the_binary64_reimplementation_bit_for_bit'
CLASSBOUND = 'adaptive::classification_tests::the_classifications_boundaries_hold_exactly'
STRESSK = 'adaptive::classification_tests::stress_scales_and_intensified_factors_match_bit_for_bit'
S8W = 'adaptive::classification_tests::input_derived_rows_and_s8_ws_far_node_rows_are_classified_as_the_design_says'
GOLDEN = 'adaptive::tests::golden_work_counts'
CASELIM = 'adaptive::tests::a_case_limit_is_exhausted_exactly_where_its_work_ends'
CHARGED = 'adaptive::tests::failed_and_verification_attempts_are_charged'
SUMCHARGE = 'adaptive::tests::a_sums_rounding_charges_the_limbs_it_rounds'
O8 = 'adaptive::tests::retained_states_equal_the_generators_emulation_of_the_method_bit_for_bit'
SKEW6 = 'adaptive::tests::the_six_member_skew_run_is_rejected_at_128_and_its_verification_is_reused'
K28 = 'adaptive::tests::k_1e_minus_28_escalates_past_128_and_is_accepted_at_256_against_512'
ZT = 'adaptive::tests::a_structural_zero_kind_is_accepted_through_the_coupled_scale'
RONLY = 'adaptive::tests::the_reactions_only_control_is_rejected_at_128_by_a_reaction'
EVERYQ = 'adaptive::tests::every_published_quantity_takes_part_in_the_stop_rule'
PRED = 'adaptive::tests::the_predicate_is_decided_exactly_at_its_boundary'
DETERM = 'adaptive::tests::runs_and_list_permutations_are_deterministic'
N0506 = 'adaptive::tests::n05_and_n06_are_accepted_at_128_against_256'
CEILCASE = 'adaptive::tests::a_ceiling_case_is_unresolved_with_its_attempts'
FORBID = 'adaptive::tests::k4s_files_call_no_binary64_transcendental_or_fused_function'
PRESC = 'assemble::tests::a_prescribed_motion_control_matches_its_exact_reference'
RHS = 'assemble::tests::the_reduced_rhs_enters_prescribed_columns_exactly_and_rounds_once'
RIGID = 'assemble::tests::every_element_annihilates_the_rigid_motions_to_within_64_units_of_2_to_minus_p'
DUP = 'assemble::tests::the_duplicate_operand_entry_is_one_exact_sum_where_a_sequential_fold_loses_the_weak_member'
ASMBITS = 'assemble::tests::assembled_entries_equal_the_fraction_emulation_bit_for_bit'
COMB_ESC = 'combine::tests::a_combination_escalates_independently_of_its_operands'
COMB_RULE = 'combine::tests::a_combination_runs_its_own_stop_rule'
COMB_PRESC = 'combine::tests::a_combined_prescribed_value_is_published_from_its_exact_sum_rounded_once'
B1C = 'combine::tests::b1_c_the_cancelled_1e80_terms_leave_the_1e_minus_8_load_exactly'
B1E = 'combine::tests::b1_e_is_caught_the_combination_reproduces_the_truth_that_a_sum_of_states_loses'
F_ILL = 'factor::tests::a_constructed_ill_conditioned_matrix_whose_pivots_pass_escalates_on_rcond_at_128_only'
F_PIVOT = 'factor::tests::a_pivot_that_fails_at_128_escalates_and_passes_at_256'
F_P64 = 'factor::tests::the_p_plus_64_residual_takes_one_correction_on_two_span_and_records_its_basis'
F_RCM = 'factor::tests::the_rcm_port_orders_sparse_directs_small_graphs_as_sparse_direct_does'
F_SPAN = 'factor::tests::the_span_decision_is_exact'
F_MECH = 'factor::tests::witnessed_mechanisms_are_refused_before_any_attempt_and_the_rx_companion_solves'
L_NET = 'ledger::tests::the_accessor_nets_once_with_the_quantum_and_a_zero_is_plus_zero'
L_CANCEL = 'ledger::tests::cancelling_contributions_are_exact_per_dof'
L_TIE = 'ledger::tests::the_ledger_enters_a_prescribed_coupled_rhs_exactly_even_when_its_tail_decides_a_tie'
L_PROJ = 'ledger::tests::the_projection_matches_the_fraction_oracle_at_every_precision'
R_FOLD = 'recover::tests::a_recovered_action_is_one_exact_expansion_where_a_sequential_fold_loses_a_term'
R_ZERO = 'recover::tests::an_exact_zero_is_published_as_plus_zero'
R_EVERY = 'recover::tests::every_recovered_kind_matches_its_exact_reference'
R_N06 = 'recover::tests::n06s_torque_and_spring_action_are_nonzero_and_correct'
WS_SPAN = 'wide_sum::tests::a_refused_span_adds_nothing'
WS_RV12 = 'wide_sum::tests::rv12_counterexample_is_rounded_once_and_a_fold_misrounds_it'
WS_TGT = 'wide_sum::tests::targeted_sums_match_the_fraction_oracle'
WS_ZERO = 'wide_sum::tests::zero_negation_absolute_value_and_reuse'
V_HAT = 'verify::tests::e_hat_couples_force_and_moment_through_the_body_extent_in_binary64'
V_PHI = 'verify::tests::phi_is_exact_where_the_product_is_normal_and_rounded_up_below_2_to_the_minus_584'

AD = 'src/structural/retained/adaptive.rs'
VE = 'src/structural/retained/verify.rs'
AS = 'src/structural/retained/assemble.rs'
BO = 'src/structural/retained/bound.rs'
FA = 'src/structural/retained/factor.rs'
RE = 'src/structural/retained/recover.rs'
LE = 'src/structural/retained/ledger.rs'
WS = 'src/structural/retained/wide_sum.rs'
CO = 'src/structural/retained/combine.rs'
ES = 'src/exact_sum.rs'

# ---------------------------------------------------------------- R7 §7's killable list
m('R7-M1', 'drop V (force and moment rows: no 2^(8-2p)*e_hat term in (a))',
  [(AD, "                        difference.add_wide_scaled(&e, false, 1, 8 - big_p)?;\n",
    "                        let _ = e; // R7-M1\n", 1)], [CTRL, G5])
m('R7-M2', 'e_hat uncoupled (e_hat = E)',
  [(VE, "    if extent == 0.0 {\n        return e;\n    }\n    let [fo, mo] = e;",
    "    if extent == 0.0 || extent == extent {\n        return e;\n    }\n    let [fo, mo] = e;", 1)], [CTRL, V_HAT])
m('R7-M3', 'Phi at every p', [(AD, "if report.is_some() && verification_precision == 1024 {",
                               "if report.is_some() {", 1)], [CTRL])
m('R7-M4', 'no Phi', [(AD, "if report.is_some() && verification_precision == 1024 {",
                       "if report.is_some() && verification_precision == 0 {", 1)], [CTRL])
m('R7-M5', 'E without the prescribed |u|',
  [(VE, "        let w_abs: Vec<Wide<L>> = u.iter().map(|v| v.abs()).collect();",
    "        let w_abs: Vec<Wide<L>> = u\n            .iter()\n            .enumerate()\n            .map(|(g, v)| if prep.source.constraint(g).is_some() { Wide::<L>::ZERO } else { v.abs() })\n            .collect();", 1)],
  [CTRL, ECH])
m('R7-M6', 'E from sum of |ledger terms| (the source loads) instead of the exact net',
  [(VE, "        if let Some(net) = ledger.and_then(|l| l.net(g)) {\n            if !net.is_zero() {\n                sum.add_integer(false, &net.magnitude, net.exponent)?;\n            }\n        }",
    "        if ledger.is_some() {\n            for l in source.loads() {\n                if l.dof.global() == g {\n                    sum.add_binary64(l.value.abs(), false)?;\n                }\n            }\n        }", 1)],
  [CTRL, EUNIT])
m('R7-M7', 'entrywise operator |B|^T|D||B| in A-bar (and hence E and the gate)',
  [(AS, "    pub(crate) bend_y: Wide<L>,\n    pub(crate) g_exp: u32,\n}\n\nimpl<const L: usize> BoundedCoefficients<L>",
    "    pub(crate) bend_y: Wide<L>,\n    pub(crate) g_exp: u32,\n    pub(crate) bm: [[Wide<L>; 12]; 6],\n}\n\nimpl<const L: usize> BoundedCoefficients<L>", 1),
   (AS, "            bend_y: self.bend_y.widen::<M>(),\n            g_exp: self.g_exp,\n",
    "            bend_y: self.bend_y.widen::<M>(),\n            g_exp: self.g_exp,\n            bm: self.bm.map(|r| r.map(|v| v.widen::<M>())),\n", 1),
   (AS, "            bend_y: self.bend_y,\n            g_exp: self.g_exp,\n",
    "            bend_y: self.bend_y,\n            g_exp: self.g_exp,\n            bm: self.b,\n", 1),
   (AS, "    let (z, y) = (m.bend_z.abs(), m.bend_y.abs());\n    let d_rows",
    "    for r in 0..6 {\n        for c in 0..12 {\n            bb[r][c] = m.bm[r][c].abs();\n        }\n    }\n    let (z, y) = (m.bend_z.abs(), m.bend_y.abs());\n    let d_rows", 1)],
  [CTRL])
m('R7-M8', 'Phi = 2^-(2p-74)*e_hat (2^-950 at 1024)',
  [(VE, "pub(crate) const PHI_SCALE_BITS: u64 = 0x2490_0000_0000_0000;",
    "pub(crate) const PHI_SCALE_BITS: u64 = 0x0490_0000_0000_0000;", 1),
   (VE, "let back = f64::from_bits(0x5B50_0000_0000_0000); // 2^438",
    "let back = f64::from_bits(0x7B50_0000_0000_0000); // 2^950", 1)], [CTRL])
m('R7-M9', 'floor at 256 (and 512)', [(AD, "if report.is_some() && verification_precision == 1024 {",
                                       "if report.is_some() && verification_precision >= 512 {", 1)], [CTRL])
m('R7-M10', 'g = 1',
  [(AS, "            return u32::try_from(k).map_err(|_| AttemptStop::Exponent);",
    "            let _ = u32::try_from(k).map_err(|_| AttemptStop::Exponent);\n            return Ok(0);", 1)], [CTRL, G5G])
m('R7-M11', 'skip the estimate test (b)',
  [(AD, "                rejection = Some(Rejection::VerificationEstimate { index });\n                return Ok(false);",
    "                let _ = index; // R7-M11", 1)], [CTRL, G5])
m('R7-M12', 'coalesced gate only (K4 at A2): no bounded fallback',
  [(AD, "                match chosen {\n                    Some((k, ratio)) => {",
    "                let _ = chosen;\n                match None::<(usize, f64)> {\n                    Some((k, ratio)) => {", 1)], [CTRL])
m('R7-M13', 'the bounded test drives refinement (tested on every evaluated state first)',
  [(AD, "            if rows.iter().all(|row| row.0) {\n                residual_worst = tracker.finish(&mut ctx16)?.unwrap_or(0.0);\n                break;\n            }",
    "            if let Some((_, ratio)) = bounded_fallback(\n                &mut ctx_q,\n                &mut ctx64,\n                &mut ctx16,\n                &mut sum,\n                p,\n                shared,\n                prep,\n                group,\n                &u,\n                std::slice::from_ref(&u_free),\n                &guard,\n            )? {\n                residual_worst = ratio;\n                gate = GateTest::Bounded { state: 0, evaluated: 1 };\n                break;\n            }\n            if rows.iter().all(|row| row.0) {\n                residual_worst = tracker.finish(&mut ctx16)?.unwrap_or(0.0);\n                break;\n            }", 1)],
  [F_P64, CTRL, GOLDEN])
m('R7-M15', "W's residual on the state before the SEED hook (the gate's evaluation)",
  [(AD, "    pub(crate) fn apply<const L: usize>(",
    "    pub(crate) fn remove<const L: usize>(\n        ctx: &mut WideContext<L>,\n        u: &mut [Wide<L>],\n    ) -> Result<(), AttemptStop>\n    where\n        Wide<L>: SupportedWidth,\n    {\n        SEED.with(|s| {\n            for &(g, v) in s.borrow().iter() {\n                u[g] = ctx.sub(&u[g], &Wide::<L>::from_f64(v)?)?;\n            }\n            Ok(())\n        })\n    }\n\n    pub(crate) fn apply<const L: usize>(", 1),
   (VE, "        let u_free: Vec<Wide<L>> = free.iter().map(|&g| u[g]).collect();",
    "        let mut unseeded = u.to_vec();\n        super::adaptive::seed::remove(&mut ctx, &mut unseeded)?;\n        let u_free: Vec<Wide<L>> = free.iter().map(|&g| unseeded[g]).collect();", 1)],
  [CTRL])
m('R7-M18', 'W and the charge at q = 2p + 64',
  [(VE, "    (3 * (verification_precision / 2) + 64).min(1024)",
    "    (verification_precision + 64).min(1024)", 1)], [CTRL])
m('R7-M20', 'drop theta',
  [(AD, "            if theta.is_some_and(|t| t.cmp_value(&half) == CmpOrdering::Greater) {",
    "            if false && theta.is_some_and(|t| t.cmp_value(&half) == CmpOrdering::Greater) {", 1)], [CTRL, G5])
m('R7-M22', 'no W-plus on translation and rotation rows',
  [(AD, "                        if let Some(wp) = &r.w_plus[index] {\n                            difference.add_wide(wp, false)?;\n                        }",
    "                        let _ = &r.w_plus[index]; // R7-M22", 1)], [CTRL, G5C])
m('R7-M23', 'drop the g check', [(AD, "        if let Some(member) = r.g_violation {",
                                  "        if let Some(member) = None::<u32> {", 1)], [CTRL, G5])
m('R7-M25', 'theta and the g check unscoped (every block carries data; every member in scope)',
  [(VE, "        let data = data_blocks(\n            blocks,",
    "        let _scoped = data_blocks(\n            blocks,", 1),
   (VE, "        let mut start = Vec::new();\n        for b in 0..blocks.len() {",
    "        let data = vec![true; blocks.len()];\n        let mut start = Vec::new();\n        for b in 0..blocks.len() {", 1),
   (VE, "            let in_scope = dofs.iter().any(|&d| {",
    "            let in_scope = true || dofs.iter().any(|&d| {", 1)], [CTRL])
m('R7-M26', 'F*est (F = 2^16) in place of B',
  [(VE, "            let bound = if data[b] {\n                certified(&vs.uc[b].uc, &s)\n            } else {",
    "            let bound = if data[b] {\n                let _ = &s;\n                Some(shared.est_blocks[b].mul_pow2(16)?)\n            } else {", 1)], [CTRL])
m('R7-M28', 'no shift (Uc alone)',
  [(VE, "            if data[b] && shift_needed(&mut sum, &vs.uc[b].uc, &shared.est_blocks[b], n_c)? {",
    "            if false && data[b] && shift_needed(&mut sum, &vs.uc[b].uc, &shared.est_blocks[b], n_c)? {", 1)],
  [RFL_CHAIN, RFL_TREE, ECH100])
m('R7-M29', "R5's form: one global Uc, per-body scope, no shift",
  [(VE, "        let data = data_blocks(\n            blocks,",
    "        let block_data = data_blocks(\n            blocks,", 1),
   (VE, "        let mut start = Vec::new();\n        for b in 0..blocks.len() {",
    "        let block_body: Vec<u32> = blocks\n            .positions\n            .iter()\n            .map(|p| source.body_of_node((free[p[0]] / 6) as u32))\n            .collect();\n        let data: Vec<bool> = (0..blocks.len())\n            .map(|b| (0..blocks.len()).any(|c| block_body[c] == block_body[b] && block_data[c]))\n            .collect();\n        let global_uc: Option<Wide<L>> = vs\n            .uc\n            .iter()\n            .try_fold(zero, |acc, x| x.uc.map(|v| wmax(&acc, &v)));\n        let mut start = Vec::new();\n        for b in 0..blocks.len() {", 1),
   (VE, "            if data[b] && shift_needed(&mut sum, &vs.uc[b].uc, &shared.est_blocks[b], n_c)? {",
    "            if false && data[b] && shift_needed(&mut sum, &vs.uc[b].uc, &shared.est_blocks[b], n_c)? {", 1),
   (VE, "            let bound = if data[b] {\n                certified(&vs.uc[b].uc, &s)\n            } else {",
    "            let bound = if data[b] {\n                let _ = &s;\n                global_uc\n            } else {", 1)],
  [CTRL, RFL_CHAIN, RFL_TREE])
m('R7-M16', 'the bounded test on the last eligible state, not the best',
  [(AD, "                (None, None) | (Some(_), None) => false,\n                (None, Some(_)) => true,\n                (Some(w), Some(b)) => cmp_ratio(w, b)? == CmpOrdering::Less,",
    "                (None, None) | (Some(_), None) => true,\n                (None, Some(_)) => true,\n                (Some(w), Some(b)) => cmp_ratio(w, b)?.is_ne() || true,", 1)], [G5GATE, CTRL])
# Kept for the derivation (ROOT's ruling): run, their outcome recorded.
m('R7-M17', 'drop the charge (d) [guard]',
  [(AD, "                rejection = Some(Rejection::Charge { index });\n                return Ok(false);",
    "                let _ = index; // R7-M17", 1)], [CTRL, G5, G5C], expect='guard')
m('R7-M21', "prescribed values rounded (their sum at P) in W's residual [guard]",
  [(VE, "            let sign = sum.signum();\n            prescribed_nonzero[*g] = sign != 0;",
    "            let mut rounded = sum.clone();\n            let v = rounded.round(&mut ctx)?;\n            terms_w[*g] = if v.is_zero() { Vec::new() } else { vec![v.widen::<W>()] };\n            let sign = sum.signum();\n            prescribed_nonzero[*g] = sign != 0;", 1)],
  [CTRL, ECH], expect='guard')
m('R7-M24', 'est in place of B [guard]',
  [(VE, "            let bound = if data[b] {\n                certified(&vs.uc[b].uc, &s)\n            } else {",
    "            let bound = if data[b] {\n                let _ = &s;\n                Some(shared.est_blocks[b])\n            } else {", 1)], [CTRL, ECH], expect='guard')
m('R7-M27', "the shift bound without its backward-error and rounding terms (sigma' = sigma) [guard at design precision]",
  [(BO, "            let sp = sub_toward(ctx, sum, &s, &e, Toward::Down)?;",
    "            let _ = (sub_toward::<L>, &e);\n            let sp = s;", 1)], [CTRL, STRESS, ECH], expect='guard')

# ---------------------------------------------------------------- K4-M34 to M40
m('K4-M34', 'nearest instead of directed (upward) rounding in the Uc pass',
  [(BO, "            let t = mul_toward(ctx, sum, &l, &a[j], Toward::Up)?;\n            acc = add_toward(ctx, sum, &acc, &t, Toward::Up)?;",
    "            let t = ctx.mul(&l, &a[j])?;\n            acc = ctx.add(&acc, &t)?;", 1)], [EUC, F2FAM, SEARCHED, ECH])
m('K4-M35', 'a changed operation order in shifted_factor (reversed inner sums)',
  [(BO, "            for kk in first[i].max(first[j])..j {\n                let t = ctx.mul(&work[kk], &get(&rows, j, kk))?;",
    "            for kk in (first[i].max(first[j])..j).rev() {\n                let t = ctx.mul(&work[kk], &get(&rows, j, kk))?;", 1)], [LOOP])
m('K4-M36', 'data flags without the prescribed-coupling rule',
  [(BO, "            if ordering.position[c] == usize::MAX && prescribed_nonzero[c] {",
    "            if false && ordering.position[c] == usize::MAX && prescribed_nonzero[c] {", 1)], [BLOCKS, ECH])
m('K4-M37', 'Phi in the stop rule but not in the classification',
  [(AD, "        decision.floor.as_deref(),", "        None,", 1)], [CTRL])
m('K4-M38', 'E rounded to nearest, not upward, to binary64',
  [(VE, "        .map(|[fo, mo]| Ok([binary64_up(fo)?, binary64_up(mo)?]))",
    "        .map(|[fo, mo]| {\n            let near = |x: &Wide<L>| match x.to_binary64() {\n                super::wide::multi::Binary64Outcome::Normal(v) => v.abs(),\n                super::wide::multi::Binary64Outcome::Subnormal { value, .. } => value.abs(),\n                super::wide::multi::Binary64Outcome::Underflow { .. } => 0.0,\n                super::wide::multi::Binary64Outcome::Overflow { .. } => f64::INFINITY,\n            };\n            let _ = binary64_up::<L>;\n            Ok([near(fo), near(mo)])\n        })", 1)], [EUNIT, ECH])
m('K4-M39', "V formed from the row's E at P instead of the published e_hat (S4)",
  [(AD, "                        let e = Wide::<M>::from_f64(hat(meta.body, meta.kind))?;\n                        difference.add_wide_scaled(&e, false, 1, 8 - big_p)?;",
    "                        let e = r.e_rows[index].unwrap_or(Wide::<M>::ZERO);\n                        difference.add_wide_scaled(&e, false, 1, 8 - big_p)?;", 1)], [G5, CTRL])
m('K4-M40', "W's residual without the prescribed columns",
  [(VE, "                &u_free,\n                Some(&terms_w),\n", "                &u_free,\n                None,\n", 2)], [CTRL, ECH])
m('K4-M24', "a combination's acceptance tied to its operands (accepted at their precision) [CEIL5A3]",
  [(AD, "pub(crate) const METHOD_TOKEN: &str",
    "thread_local! {\n    pub(crate) static TIED: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };\n}\n\npub(crate) const METHOD_TOKEN: &str", 1),
   (AD, "            Ok(false) => {\n                attempts[candidate_index].outcome = AttemptOutcome::Rejected(",
    "            Ok(false) if !prep.factors.is_empty() && TIED.with(|t| t.get()) == p => {\n                attempts[candidate_index].outcome = AttemptOutcome::Accepted;\n                attempts[v_index].outcome = AttemptOutcome::Verified;\n                return finish_selected(\n                    prep,\n                    group,\n                    cache.clone(),\n                    states,\n                    &candidate,\n                    verification_p,\n                    attempts,\n                    &decision,\n                    &report,\n                    geometry,\n                );\n            }\n            Ok(false) => {\n                attempts[candidate_index].outcome = AttemptOutcome::Rejected(", 1),
   (CO, "        match run_schedule(prep, first.group.clone(), &mut cache, case_limit, meter) {",
    "        super::adaptive::TIED.with(|t| {\n            t.set(operands.iter().map(|o| o.1.selected_precision()).max().unwrap_or(0))\n        });\n        match run_schedule(prep, first.group.clone(), &mut cache, case_limit, meter) {", 1)],
  [CTRL, COMB_RULE, COMB_ESC])

# ---------------------------------------------------------------- K4's earlier mutants (brief and checkpoint 0)
m('K4-M1', 'the multi-term sum folds: its positive and negative parts each rounded to p, then added at p',
  [(WS, "        let (negative, magnitude, used) = self.net();\n        let anchor = i64::try_from(self.anchor).map_err(|_| SumRefusal::Exponent)?;\n        self.work.rounded_limbs += used as u64;",
    "        {\n            let anchor = i64::try_from(self.anchor).map_err(|_| SumRefusal::Exponent)?;\n            let used = self.used.max(1);\n            let pos = context.from_integer(false, &self.positive[..used], anchor)?;\n            let neg = context.from_integer(true, &self.negative[..used], anchor)?;\n            if used > 0 {\n                return Ok(context.add(&pos, &neg)?);\n            }\n        }\n        let (negative, magnitude, used) = self.net();\n        let anchor = i64::try_from(self.anchor).map_err(|_| SumRefusal::Exponent)?;\n        self.work.rounded_limbs += used as u64;", 1)],
  [WS_RV12, WS_TGT])
m('K4-M2', 'the sum drops its far tail (only the top L + 1 limbs reach the rounding)',
  [(WS, "        Ok(context.from_integer(negative, &magnitude[..used.max(1)], anchor)?)",
    "        let top = used.max(1);\n        let start = top.saturating_sub(L + 1);\n        Ok(context.from_integer(negative, &magnitude[start..top], anchor + 64 * start as i64)?)", 1)],
  [WS_TGT, WS_RV12])
m('K4-M3', 'the sum truncates at p instead of rounding to nearest',
  [(WS, "        Ok(context.from_integer(negative, &magnitude[..used.max(1)], anchor)?)",
    "        let top = used.max(1);\n        let mut window = magnitude;\n        let bits = top * 64 - window[top - 1].leading_zeros() as usize;\n        let cut = bits.saturating_sub(context.precision() as usize);\n        for i in 0..cut {\n            window[i / 64] &= !(1u64 << (i % 64));\n        }\n        Ok(context.from_integer(negative, &window[..top], anchor)?)", 1)],
  [WS_TGT, WS_RV12])
m('K4-M4', 'the sum returns -0 for an exact zero',
  [(WS, "        if self.empty {\n            return Ok(context.from_integer(false, &[0], 0)?);\n        }\n        let (negative, magnitude, used) = self.net();",
    "        if self.empty {\n            return Ok(Wide::<L>::ZERO.neg());\n        }\n        let (negative, magnitude, used) = self.net();\n        if magnitude[..used.max(1)].iter().all(|&l| l == 0) {\n            return Ok(Wide::<L>::ZERO.neg());\n        }", 1)],
  [WS_ZERO, R_ZERO])
m('K4-M5', 'the sum drops a term at its span limit instead of refusing',
  [(WS, "        if span > SPAN_LIMIT_BITS {\n            return Err(SumRefusal::Span);\n        }",
    "        if span > SPAN_LIMIT_BITS {\n            return Ok(());\n        }", 1)], [WS_SPAN, WS_TGT])
m('K4-M6', "the accessor returns the positive magnitude only (no netting)",
  [(ES, "    pub(crate) fn net_parts(&self) -> (bool, Magnitude, i64) {\n",
    "    pub(crate) fn net_parts(&self) -> (bool, Magnitude, i64) {\n        if self.positive != self.negative || self.positive == self.negative {\n            return (false, self.positive, QUANTUM_EXPONENT);\n        }\n", 1)], [L_NET, L_CANCEL])
m('K4-M7', "the accessor's quantum is 2^-2147",
  [(ES, "                QUANTUM_EXPONENT,\n            ),", "                QUANTUM_EXPONENT + 1,\n            ),", 2)], [L_NET, L_PROJ])
m('K4-M8', 'the ledger is cut to its top 128 bits before it joins a coupled rhs or a reaction',
  [(LE, "            Some(net) if !net.is_zero() => {\n                sum.add_integer(net.negative != negate, &net.magnitude, net.exponent)\n            }",
    "            Some(net) if !net.is_zero() => {\n                let mag = &net.magnitude;\n                let bits = mag.len() * 64 - mag[mag.len() - 1].leading_zeros() as usize;\n                let drop = bits.saturating_sub(128);\n                let mut top: u128 = 0;\n                for i in (drop..bits).rev() {\n                    top = (top << 1) | u128::from((mag[i / 64] >> (i % 64)) & 1);\n                }\n                sum.add_integer(\n                    net.negative != negate,\n                    &[top as u64, (top >> 64) as u64],\n                    net.exponent + drop as i64,\n                )\n            }", 1)],
  [L_TIE, B1L])
m('K4-M9', 'the pivot screen with u = 2^-53',
  [(FA, "    sum.add_wide_scaled(pivot, false, 1, i64::from(p))?;\n    sum.add_wide_scaled(pivot, true, m, 0)?;",
    "    sum.add_wide_scaled(pivot, false, 1, i64::from(p.min(53)))?;\n    sum.add_wide_scaled(pivot, true, m, 0)?;", 1)], [N0506, R_N06, CTRL])
m('K4-M10', 'the rcond escalation is skipped',
  [(FA, "            return Err(AttemptStop::Condition);", "            return Ok(Wide::<L>::ZERO);", 1)], [F_ILL, K28])
m('K4-M11', 'the residual is formed at p, not p + 64',
  [(AD, "        128 => run!(&mut cache.s128, 4, 4, 192, P128),", "        128 => run!(&mut cache.s128, 4, 4, 128, P128),", 1),
   (AD, "        256 => run!(&mut cache.s256, 4, 8, 320, P256),", "        256 => run!(&mut cache.s256, 4, 8, 256, P256),", 1),
   (AD, "        512 => run!(&mut cache.s512, 8, 16, 576, P512),", "        512 => run!(&mut cache.s512, 8, 16, 512, P512),", 1)],
  [F_P64, GOLDEN, CTRL])
m('K4-M12', 'the residual is folded at q, not one exact expansion',
  [(AD, "        let mut count = 0u64;\n        for index in structure.pattern.row_range(i) {\n            let j = structure.pattern.column(index);\n            let kij = &k_q[index];",
    "        let mut folded = {\n            let mut t = ExactWideSum::new();\n            ledger.add_to(i, &mut t, false)?;\n            t.round(ctx_q)?\n        };\n        let mut count = 0u64;\n        for index in structure.pattern.row_range(i) {\n            let j = structure.pattern.column(index);\n            let kij = &k_q[index];", 1),
   (AD, "            r.add_product(ctx_q, kij, &uj, true)?;\n            let negative",
    "            r.add_product(ctx_q, kij, &uj, true)?;\n            let prod = ctx_q.mul(kij, &uj)?;\n            folded = ctx_q.sub(&folded, &prod)?;\n            let negative", 1),
   (AD, "        let m = 2 * count + 2;\n        let mut absolute = r.clone();",
    "        r.clear();\n        r.add_wide(&folded, false)?;\n        let m = 2 * count + 2;\n        let mut absolute = r.clone();", 1)],
  [F_P64, O8, CTRL])
m('K4-M13', 'the 2p verification reuses the 128-bit formation',
  [(AD, "        let members = form_members(&mut ctx, &mut sum, &g, source)?;\n        let directional = form_directional(&mut ctx, &mut sum, source)?;",
    "        let mut c128 = WideContext::<L>::new(128).expect(\"supported precision\");\n        let members = form_members(&mut c128, &mut sum, &g, source)?;\n        let directional = form_directional(&mut c128, &mut sum, source)?;", 1)],
  [K28, CTRL, O8])
m('K4-M14a', 'the stop rule scales by |q_p| instead of |q_2p|',
  [(AD, "            let q2 = &verification[index];\n            let s_star = &scales[meta.body as usize][meta.kind.index()];",
    "            let q2c = candidate[index].widen::<M>();\n            let q2 = &q2c;\n            let s_star = &scales[meta.body as usize][meta.kind.index()];", 1)], [PRED, SKEW6, CTRL])
m('K4-M14b', 'the stop rule with 2^-53 instead of 2^-64',
  [(AD, "            sum.add_wide_scaled(&magnitude, false, 1, -64)?;", "            sum.add_wide_scaled(&magnitude, false, 1, -53)?;", 1)], [SKEW6, PRED, CTRL])
m('K4-M15', 'the uncoupled S(kind) instead of S*',
  [(AD, "        if extent == 0.0 {\n            out.push([tr, ro, fo, mo]);\n            continue;\n        }\n        let lb = Wide::<M>::from_f64(extent)?;",
    "        if extent == 0.0 || extent == extent {\n            out.push([tr, ro, fo, mo]);\n            continue;\n        }\n        let lb = Wide::<M>::from_f64(extent)?;", 1)], [ZT, CTRL])
m('K4-M16', 'reactions left out of the stop rule',
  [(AD, "        for (index, meta) in layout.iter().enumerate() {\n            let q2 = &verification[index];\n            let magnitude = magnitude_of(index, meta);",
    "        for (index, meta) in layout.iter().enumerate() {\n            if matches!(meta.id, QuantityId::Reaction(_)) {\n                continue;\n            }\n            let q2 = &verification[index];\n            let magnitude = magnitude_of(index, meta);", 1)], [RONLY, EVERYQ, CTRL])
m('K4-M17', "an attempt's context recorded twice",
  [(AD, "    work.record(&ctx64);\n    let sum_work = sum.work();\n    let total = work.limb_multiply_equivalents() + sum_work.limb_multiply_equivalents();\n    Spent {",
    "    work.record(&ctx64);\n    work.record(&ctx);\n    let sum_work = sum.work();\n    let total = work.limb_multiply_equivalents() + sum_work.limb_multiply_equivalents();\n    Spent {", 1)], [GOLDEN])
m('K4-M18', "from_integer's input length is not charged",
  [(WS, "        self.work.rounded_limbs += used as u64;\n", "", 1)], [SUMCHARGE, GOLDEN])
m('K4-M19', 'the verification pass is not charged to the budget or the meter',
  [(AD, "            record.verification_work = spent.total;\n            budget.used = budget.used.saturating_add(spent.total);\n            meter.charge(spent.total);",
    "            record.verification_work = spent.total;", 1)], [CASELIM, CHARGED])
m('K4-M20', 'budget exhaustion is ignored by the stage guard',
  [(AD, "        if used > self.case_room {", "        if false && used > self.case_room {", 1),
   (AD, "        } else if used > self.invocation_room {", "        } else if false && used > self.invocation_room {", 1)], [CASELIM])
m('K4-M21', 'a witnessed mechanism is escalated instead of refused',
  [(AD, "        }) => {\n            return Err((\n                Refusal::MechanismWitnessed {\n                    body,\n                    rigid_parameters,\n                },\n                Vec::new(),\n            ))\n        }",
    "        }) => {\n            let _ = (body, rigid_parameters);\n            Vec::new()\n        }", 1)], [F_MECH, R1, CTRL])
m('K4-M22', "Gram-Schmidt from FK-style binary64 axes",
  [(AS, "    let (ez, _) = normalize(ctx, sum, &zc)?;\n",
    "    let (ez, _) = normalize(ctx, sum, &zc)?;\n    let (ex, ey, ez) = {\n        let d = [xj[0] - xi[0], xj[1] - xi[1], xj[2] - xi[2]];\n        let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();\n        let fx = [d[0] / n, d[1] / n, d[2] / n];\n        let y = m.y_reference;\n        let pr = y[0] * fx[0] + y[1] * fx[1] + y[2] * fx[2];\n        let c = [y[0] - pr * fx[0], y[1] - pr * fx[1], y[2] - pr * fx[2]];\n        let cn = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();\n        let fy = [c[0] / cn, c[1] / cn, c[2] / cn];\n        let fz = [fx[1] * fy[2] - fx[2] * fy[1], fx[2] * fy[0] - fx[0] * fy[2], fx[0] * fy[1] - fx[1] * fy[0]];\n        let _ = (ex, ey, ez);\n        (\n            [lift::<L>(fx[0])?, lift::<L>(fx[1])?, lift::<L>(fx[2])?],\n            [lift::<L>(fy[0])?, lift::<L>(fy[1])?, lift::<L>(fy[2])?],\n            [lift::<L>(fz[0])?, lift::<L>(fz[1])?, lift::<L>(fz[2])?],\n        )\n    };\n", 1)],
  [RIGID, R1, CTRL])
m('K4-M23', 'a directional spring formed from its binary64-normalized direction',
  [(AS, "        let n = [\n            lift::<L>(s.direction[0])?,\n            lift::<L>(s.direction[1])?,\n            lift::<L>(s.direction[2])?,\n        ];",
    "        let dn = s.direction;\n        let nn = (dn[0] * dn[0] + dn[1] * dn[1] + dn[2] * dn[2]).sqrt();\n        let n = [\n            lift::<L>(dn[0] / nn)?,\n            lift::<L>(dn[1] / nn)?,\n            lift::<L>(dn[2] / nn)?,\n        ];", 1)], [R1, CTRL, ASMBITS])
m('K4-M25a', 'b rounded to nearest (not upward)',
  [(AD, "    if nearest * two64 < s_star {\n        next_up(nearest)", "    if false && nearest * two64 < s_star {\n        next_up(nearest)", 1)], [CLASSBITS, CLASSBOUND])
m('K4-M25b', 'the S* < 2^-988 rule dropped',
  [(AD, "    if s_star < small || value.abs() < threshold(s_star) {", "    let _ = small;\n    if value.abs() < threshold(s_star) {", 1)], [CLASSBITS, CLASSBOUND])
m('K4-M26', 'the directional span decided with a binary64 determinant',
  [(FA, "fn determinant_nonzero(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Option<bool> {\n",
    "fn determinant_nonzero(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Option<bool> {\n    if a[0] == a[0] {\n        let det = a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])\n            + a[2] * (b[0] * c[1] - b[1] * c[0]);\n        return Some(det != 0.0);\n    }\n", 1)], [F_SPAN])
m('K4-M27', 'station interpolation with 1 - t in place of t',
  [(RE, "        let t = lift::<L>(s.fraction)?;", "        let t = lift::<L>(1.0 - s.fraction)?;", 1)], [R_EVERY, CTRL])
m('K4-M28', 'RCM tie-break by descending index',
  [(FA, "        .min_by_key(|&node| (degrees[node], node))", "        .min_by_key(|&node| (degrees[node], std::cmp::Reverse(node)))", 1)], [F_RCM])
m('K4-M29', 'the classification with <= t instead of < t',
  [(AD, "    if s_star < small || value.abs() < threshold(s_star) {", "    if s_star < small || value.abs() <= threshold(s_star) {", 1)], [CLASSBOUND, CLASSBITS])
m('K4-M30i', "(K4-M30's inverse; O7's start rule was superseded) a combination starts at its operands' precision",
  [(AD, "pub(crate) const METHOD_TOKEN: &str",
    "thread_local! {\n    pub(crate) static TIED: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };\n}\n\npub(crate) const METHOD_TOKEN: &str", 1),
   (AD, "    let mut c = 0;\n    while c < 3 {",
    "    let mut c = if prep.factors.is_empty() {\n        0\n    } else {\n        PRECISIONS\n            .iter()\n            .position(|&q| q == TIED.with(|t| t.get()))\n            .unwrap_or(0)\n            .min(2)\n    };\n    while c < 3 {", 1),
   (CO, "        match run_schedule(prep, first.group.clone(), &mut cache, case_limit, meter) {",
    "        super::adaptive::TIED.with(|t| {\n            t.set(operands.iter().map(|o| o.1.selected_precision()).max().unwrap_or(0))\n        });\n        match run_schedule(prep, first.group.clone(), &mut cache, case_limit, meter) {", 1)],
  [COMB_ESC, COMB_RULE, CTRL])
m('K4-M31', 'a binary64 load fold in a K4 file (the ledger)',
  [(LE, "        let mut entries: Vec<(usize, ExactAccumulator, bool)> = Vec::new();\n        for load in source.loads() {\n            let dof = load.dof.global();",
    "        let mut entries: Vec<(usize, ExactAccumulator, bool)> = Vec::new();\n        let mut folded = 0.0f64;\n        for load in source.loads() {\n            folded += load.value;\n            let _ = folded;\n            let dof = load.dof.global();", 1)], [], )
MUTANTS['K4-M31']['itests'] = ['s11_site_table']
MUTANTS['K4-M31']['archive'] = 'projects/chirality-piping/core'
m('NONE-S11', 'the unmutated candidate, for the S11 site table (the whole core tree: the table reads other crates)', [], [])
MUTANTS['NONE-S11']['itests'] = ['s11_site_table']
MUTANTS['NONE-S11']['archive'] = 'projects/chirality-piping/core'
m('K4-M32', 'rcond from the unequilibrated K',
  [(FA, "                    sum.add_wide_scaled(&k[index].abs(), false, 1, self.scale[a] + self.scale[b])?;",
    "                    sum.add_wide_scaled(&k[index].abs(), false, 1, 0)?;", 1)], [K28, F_ILL, CTRL])
m('K4-M33', "a prescribed row published from its p-rounded state",
  [(AD, "            *value = exact_publication(&self.prescribed[k].1);", "            let _ = exact_publication(&self.prescribed[k].1);", 1)], [COMB_PRESC])
m('D1', "promote the binary64 assembled K to p (every entry rounded to binary64)",
  [(AS, "        values[index] = sum.round(ctx)?;\n        done[index] = true;",
    "        values[index] = Wide::<L>::from_f64(\n            super::recover::publish_value(&sum.round(ctx)?)\n                .value()\n                .unwrap_or(0.0),\n        )?;\n        done[index] = true;", 1)], [NP, CTRL])
m('D2', 'round u to binary64 before recovery',
  [(AD, "        seed::apply(&mut ctx, &mut u)?;\n",
    "        seed::apply(&mut ctx, &mut u)?;\n        for v in u.iter_mut() {\n            *v = Wide::<L>::from_f64(super::recover::publish_value(v).value().unwrap_or(0.0))?;\n        }\n", 1)], [R_N06, CTRL])
m('D3', 'drop K_fc*u_c from the reduced rhs',
  [(AS, "            if source.constraint(c).is_some() && !u[c].is_zero() {", "            if false && source.constraint(c).is_some() && !u[c].is_zero() {", 1)], [RHS, PRESC, CTRL])
m('D4', 'flip an axis sign in B',
  [(AS, "        b[0][6 + k] = ex[k];", "        b[0][6 + k] = ex[k].neg();", 1)], [RIGID])
m('D5', 'fix p at 128 (no escalation)', [(AD, "    while c < 3 {", "    while c < 1 {", 1)], [K28, CTRL])
m('D6', 'accept on the screens alone (the decision always accepts)',
  [(AD, "    let mut run = || -> Result<bool, AttemptStop> {\n        let spent",
    "    let mut run = || -> Result<bool, AttemptStop> {\n        if report.is_some() {\n            return Ok(true);\n        }\n        let spent", 1)], [SKEW6, CTRL])
m('D7', 'disable the geometry check',
  [(AD, "    let geometry = match geometry_first(source) {",
    "    let geometry = match Ok::<Vec<BodyGeometry>, GeometryRefusal>(geometry_first(source).unwrap_or_default()) {", 1)], [F_MECH, R1])
m('D10', 'order the free DOFs by index (list order) instead of the pattern (RCM)',
  [(FA, "    let order = reverse_cuthill_mckee(&adjacency);\n", "    let order: Vec<usize> = (0..adjacency.len()).collect();\n    let _ = reverse_cuthill_mckee;\n", 1)],
  [DETERM, O8, EUNIT])
m('D13', 'fold the loads at p instead of the ledger (reduced rhs)',
  [(AS, "        sum.clear();\n        ledger.add_to(i, sum, false)?;",
    "        sum.clear();\n        let _ = ledger;\n        let mut f = Wide::<L>::ZERO;\n        for l in source.loads() {\n            if l.dof.global() == i {\n                f = ctx.add(&f, &Wide::<L>::from_f64(l.value)?)?;\n            }\n        }\n        sum.add_wide(&f, false)?;", 1)], [B1L, R1])
m('D14', "combine term by term in binary64 (the combination's ledger folded)",
  [(LE, "                entries[index]\n                    .1\n                    .add_product(factor, load.value)\n                    .map_err(LedgerRefusal::Accumulator)?;",
    "                let folded = entries[index].1.round().unwrap_or(0.0) + factor * load.value;\n                let mut acc = ExactAccumulator::new();\n                acc.add(folded).map_err(LedgerRefusal::Accumulator)?;\n                entries[index].1 = acc;", 1)], [B1C, B1E])
m('D15', "combination outputs out of the stop rule (accepted at their first candidate)",
  [(AD, "            Ok(false) => {\n                attempts[candidate_index].outcome = AttemptOutcome::Rejected(",
    "            Ok(false) if !prep.factors.is_empty() => {\n                attempts[candidate_index].outcome = AttemptOutcome::Accepted;\n                attempts[v_index].outcome = AttemptOutcome::Verified;\n                return finish_selected(\n                    prep,\n                    group,\n                    cache.clone(),\n                    states,\n                    &candidate,\n                    verification_p,\n                    attempts,\n                    &decision,\n                    &report,\n                    geometry,\n                );\n            }\n            Ok(false) => {\n                attempts[candidate_index].outcome = AttemptOutcome::Rejected(", 1)],
  [COMB_RULE, B1E, CTRL])
m('D16a', 'sequential addition at p in assembly',
  [(AS, "        values[index] = sum.round(ctx)?;\n        done[index] = true;",
    "        let mut acc = Wide::<L>::ZERO;\n        for item in &structure.items[structure.starts[index]..structure.starts[index + 1]] {\n            let v = match *item {\n                Contribution::Member { member, a, b } => *members[member as usize].ke(a as usize, b as usize),\n                Contribution::Spring { spring } => Wide::<L>::from_f64(source.springs()[spring as usize].stiffness)?,\n                Contribution::Directional { spring, a, b } => directional[spring as usize].k[a as usize][b as usize],\n            };\n            acc = ctx.add(&acc, &v)?;\n        }\n        values[index] = acc;\n        done[index] = true;", 1)],
  [DUP, ASMBITS])
m('D16b', 'sequential addition at p in recovery (d = T u)',
  [(RE, "                sum.clear();\n                for c in 0..3 {\n                    sum.add_product(ctx, &m.axes[r][c], &u[dofs[3 * block + c]], false)?;\n                }\n                d[3 * block + r] = sum.round(ctx)?;",
    "                let mut acc = zero;\n                for c in 0..3 {\n                    let t = ctx.mul(&m.axes[r][c], &u[dofs[3 * block + c]])?;\n                    acc = ctx.add(&acc, &t)?;\n                }\n                d[3 * block + r] = acc;", 1)],
  [R_FOLD, R_EVERY])
m('D17', 'drop the verified-accuracy classification (every row relative)',
  [(AD, "pub(crate) fn classify(value: f64, s_star: f64) -> RowClass {\n",
    "pub(crate) fn classify(value: f64, s_star: f64) -> RowClass {\n    if value == value || value != value {\n        return RowClass::RelativeVerified;\n    }\n", 1)], [S8W, CLASSBITS])
m('D20', 'R = 10^9*2^-64 (rounded) in the threshold',
  [(AD, "    f64::from_bits(FLOOR_RATIO_BITS) * s_star\n", "    (1e9 / 18_446_744_073_709_551_616.0) * s_star\n", 1)], [CLASSBITS, CLASSBOUND])
m('D25', 'k = 1 for the intensified stress factor',
  [(AD, "    let k = f64::from_bits(K_SQRT2_BITS);\n", "    let k = 1.0f64;\n", 1)], [STRESSK])
m('D13b', 'fold the loads in binary64 in the ledger itself (every use of the ledger)',
  [(LE, "            entries[index]\n                .1\n                .add(load.value)\n                .map_err(LedgerRefusal::Accumulator)?;",
    "            let folded = entries[index].1.round().unwrap_or(0.0) + load.value;\n            let mut acc = ExactAccumulator::new();\n            acc.add(folded).map_err(LedgerRefusal::Accumulator)?;\n            entries[index].1 = acc;", 1)], [B1L, R1, CTRL])
