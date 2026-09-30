| Mutant | Edit | Result | Killed by | Controls that move (* false claim) |
|---|---|---|---|---|
| NONE | the unmutated candidate | control: passes | — |  |
| R7-M1 | drop V (force and moment rows: no 2^(8-2p)*e_hat term in (a)) | killed | CTRL, G5 | ASSEMBLY-SAT→128*, DEMOTION2→512, EXACT-RIGID→128, F-2→128*, F-2-CEIL→128*, F-2-SPOS→128, GS-TRANS-y345→512, M10-G→128, M7-GS1→512, MIXED-2^-200→512, PRESCRIBED-TAIL→128, PRESCRIBED-TAIL-FREE→128, PT-A→128, PTF-A→128, RIGID-UNLOADED→512 |
| R7-M2 | e_hat uncoupled (e_hat = E) | killed | CTRL, V_HAT | EHAT-OVERFLOW→128, F-3-FREE→-, HH-FOOL-m100-LOADED→-, HH-FOOL-m40-LOADED→-, LOADONLY-y001→512, OBLIQUE-K1E-4→256, PROBE-y001-m1-axial→512, PROBE-y001-m1-inplane→512, PROBE-y001-m3-axial→-, PROBE-y001-m3-general→512, PROBE-y001-m3-inplane→512, PROBE-y345-m3-general→512, R115-SEED3→-, RF-LARGE-CHAIN-n00010-ROT→512, RF-LARGE-TREE-n00010-AX→-, RF-LARGE-TREE-n00010-ROT→512, S8-W-1e-10→-, SKEW-K1E-28-AXIAL→- |
| R7-M3 | Phi at every p | killed | CTRL | ASSEMBLY-SAT→512, DEMOTION2→512, EXACT-RIGID→256, F-2-CEIL→256, GS-TRANS-y345→512, M7-GS1→512, PRESCRIBED-TAIL→256, PRESCRIBED-TAIL-FREE→256, PT-A→256, PTF-A→256, RIGID-UNLOADED→512 |
| R7-M4 | no Phi | killed | CTRL | (operand PT-A unresolved; the list stops) |
| R7-M5 | E without the prescribed /u/ | killed | CTRL, ECH | ASSEMBLY-SAT→128*, BLOCK-PRESC→512, DEMOTION2→512, EXACT-RIGID→128, M10-G→128, M7-GS1→-, PRESCRIBED-TAIL→128, PT-A→128, RIGID-UNLOADED→- |
| R7-M6 | E from sum of /ledger terms/ (the source loads) instead of the exact net | killed | CTRL, EUNIT | LEDGER-AT-RESTRAINT→256 |
| R7-M7 | entrywise operator /B/^T/D//B/ in A-bar (and hence E and the gate) | killed | CTRL | DEMOTION2→512, GS-ROT-y345-LOADED→-, GS-TRANS-y345→-, LOADONLY-y345→-, M10-ANISO→512, PROBE-y345-m1-axial→-, PROBE-y345-m1-inplane→-, PROBE-y345-m1-outofplane→-, PROBE-y345-m1-torque→-, PROBE-y345-m3-axial→-, PROBE-y345-m3-inplane→-, PROBE-y345-m3-outofplane→-, PROBE-y345-m3-torque→-, THETA-STUB-COUPLED→128 |
| R7-M8 | Phi = 2^-(2p-74)*e_hat (2^-950 at 1024) | killed | CTRL | (operand PTF-A unresolved; the list stops) |
| R7-M9 | floor at 256 (and 512) | killed | CTRL | ASSEMBLY-SAT→512, DEMOTION2→512, EXACT-RIGID→256, F-2-CEIL→256, GS-TRANS-y345→512, M7-GS1→512, PRESCRIBED-TAIL→256, PRESCRIBED-TAIL-FREE→256, PT-A→256, PTF-A→256, RIGID-UNLOADED→512 |
| R7-M10 | g = 1 | killed | CTRL, G5G | G-PRESC-MEMBER→128, M10-G→128 |
| R7-M11 | skip the estimate test (b) | killed | CTRL, G5 | CEIL5A3→-, CEIL5A3-A→512, CEIL5A3-B→512, DIRECTIONAL-SPAN→256, LEVER2-k100-s40→-, LEVER2-k110-s20→-, LEVER2-k90-s40→-, SEEDED-COMMON→128 |
| R7-M12 | coalesced gate only (K4 at A2): no bounded fallback | killed | CTRL | DIRECTIONAL-WELL→-, GS-ROT-y345-LOADED→-, GS-TRANS-y345→-, LOADONLY-y345→-, M10-ANISO→512, PROBE-y345-m1-axial→-, PROBE-y345-m1-inplane→-, PROBE-y345-m1-outofplane→-, PROBE-y345-m1-torque→-, PROBE-y345-m3-axial→-, PROBE-y345-m3-inplane→-, PROBE-y345-m3-outofplane→-, PROBE-y345-m3-torque→- |
| R7-M13 | the bounded test drives refinement (tested on every evaluated state first) | killed | F_P64, CTRL, GOLDEN | none |
| R7-M15 | W's residual on the state before the SEED hook (the gate's evaluation) | killed | CTRL | SEEDED-COMMON→128, SEEDED-SOFT→128* |
| R7-M18 | W and the charge at q = 2p + 64 | killed | CTRL | CEIL5A3→-, CEIL5A3-A→512, CEIL5A3-B→512, CHARGE-SLENDER→512, HH-FOOL-m100-LOADED→512, HH-SLENDER-m40→512, LEVER2-k100-s40→-, LEVER2-k110-s20→-, LEVER2-k90-s40→-, N06→512, PIVOT→512, SKEW-K1E-12→512, SKEW-K1E-28→512, SKEW-K1E-28-AXIAL→512, SKEW6-K1E-12→512, THETA-STUB→512, THETA-STUB-COUPLED→512 |
| R7-M20 | drop theta | killed | CTRL, G5 | THETA-STUB-COUPLED→128 |
| R7-M22 | no W-plus on translation and rotation rows | killed | CTRL, G5C | DEMOTION2→512, SEEDED-SOFT→128* |
| R7-M23 | drop the g check | killed | CTRL, G5 | G-PRESC-MEMBER→128 |
| R7-M25 | theta and the g check unscoped (every block carries data; every member in scope) | killed | CTRL | G-FIXED-MEMBER→256, THETA-STUB→256, THETA-ZERO-BODY→256 |
| R7-M26 | F*est (F = 2^16) in place of B | killed | CTRL | CHARGE-SLENDER→256, HH-SLENDER-m40→256 |
| R7-M28 | no shift (Uc alone) | killed | RFL_CHAIN, RFL_TREE, ECH100 | none |
| R7-M29 | R5's form: one global Uc, per-body scope, no shift | killed | CTRL, RFL_CHAIN, RFL_TREE | HH-SLENDER-m40→256, THETA-STUB→256 |
| R7-M16 | the bounded test on the last eligible state, not the best | killed | G5GATE | none |
| R7-M17 | drop the charge (d) [guard] | guard: no control moves | G5, G5C | none |
| R7-M21 | prescribed values rounded (their sum at P) in W's residual [guard] | guard: no control moves | GOLDEN | none |
| R7-M24 | est in place of B [guard] | guard: no control moves | ECH | none |
| R7-M27 | the shift bound without its backward-error and rounding terms (sigma' = sigma) [guard at design precision] | guard: no control moves | STRESS, ECH | none |
| K4-M34 | nearest instead of directed (upward) rounding in the Uc pass | killed | EUC, F2FAM, ECH |  |
| K4-M35 | a changed operation order in shifted_factor (reversed inner sums) | killed | LOOP |  |
| K4-M36 | data flags without the prescribed-coupling rule | killed | BLOCKS, ECH |  |
| K4-M37 | Phi in the stop rule but not in the classification | killed | CTRL | F-2-CEIL→512*, F-3-FREE→512*, F-3-ROT→512*, GS-ROT-y345→512*, GS-TRANS-y345→512*, M10-ANISO→512*, M7-GS1→512*, RIGID-UNLOADED→512* |
| K4-M38 | E rounded to nearest, not upward, to binary64 | killed | EUNIT |  |
| K4-M39 | V formed from the row's E at P instead of the published e_hat (S4) | killed | G5 | none |
| K4-M40 | W's residual without the prescribed columns | killed | CTRL, ECH | (operand PTF-A unresolved; the list stops) |
| K4-M24 | a combination's acceptance tied to its operands (accepted at their precision) [CEIL5A3] | killed | CTRL | CEIL5A3→512 |
| K4-M1 | the multi-term sum folds: its positive and negative parts each rounded to p, then added at p | killed | WS_RV12, WS_TGT |  |
| K4-M2 | the sum drops its far tail (only the top L + 1 limbs reach the rounding) | killed | WS_TGT |  |
| K4-M3 | the sum truncates at p instead of rounding to nearest | killed | WS_TGT, WS_RV12 |  |
| K4-M4 | the sum returns -0 for an exact zero | killed | WS_ZERO |  |
| K4-M5 | the sum drops a term at its span limit instead of refusing | killed | WS_SPAN, WS_TGT |  |
| K4-M6 | the accessor returns the positive magnitude only (no netting) | killed | L_NET |  |
| K4-M7 | the accessor's quantum is 2^-2147 | killed | L_NET, L_PROJ |  |
| K4-M8 | the ledger is cut to its top 128 bits before it joins a coupled rhs or a reaction | killed | L_TIE |  |
| K4-M9 | the pivot screen with u = 2^-53 | killed | N0506, R_N06, CTRL |  |
| K4-M10 | the rcond escalation is skipped | killed | F_ILL, K28 |  |
| K4-M11 | the residual is formed at p, not p + 64 | killed | F_P64, GOLDEN, CTRL |  |
| K4-M12 | the residual is folded at q, not one exact expansion | killed | ECH |  |
| K4-M13 | the 2p verification reuses the 128-bit formation | killed | K28, CTRL |  |
| K4-M14a | the stop rule scales by /q_p/ instead of /q_2p/ | killed | PRED |  |
| K4-M14b | the stop rule with 2^-53 instead of 2^-64 | killed | SKEW6, PRED, CTRL |  |
| K4-M15 | the uncoupled S(kind) instead of S* | killed | ZT, CTRL |  |
| K4-M16 | reactions left out of the stop rule | killed | RONLY, EVERYQ, CTRL |  |
| K4-M17 | an attempt's context recorded twice | killed | GOLDEN |  |
| K4-M18 | from_integer's input length is not charged | killed | SUMCHARGE, GOLDEN |  |
| K4-M19 | the verification pass is not charged to the budget or the meter | killed | CASELIM |  |
| K4-M20 | budget exhaustion is ignored by the stage guard | killed | CASELIM |  |
| K4-M21 | a witnessed mechanism is escalated instead of refused | killed | F_MECH, R1 |  |
| K4-M22 | Gram-Schmidt from FK-style binary64 axes | killed | RIGID, R1, CTRL |  |
| K4-M23 | a directional spring formed from its binary64-normalized direction | killed | R1, ASMBITS |  |
| K4-M25a | b rounded to nearest (not upward) | killed | CLASSBITS, CLASSBOUND |  |
| K4-M25b | the S* < 2^-988 rule dropped | killed | CLASSBITS, CLASSBOUND |  |
| K4-M26 | the directional span decided with a binary64 determinant | killed | F_SPAN |  |
| K4-M27 | station interpolation with 1 - t in place of t | killed | R_EVERY, CTRL |  |
| K4-M28 | RCM tie-break by descending index | killed | F_RCM |  |
| K4-M29 | the classification with <= t instead of < t | killed | CLASSBOUND, CLASSBITS |  |
| K4-M30i | (K4-M30's inverse; O7's start rule was superseded) a combination starts at its operands' precision | killed | COMB_ESC, COMB_RULE, CTRL |  |
| K4-M31 | a binary64 load fold in a K4 file (the ledger) | killed | S11 |  |
| NONE-S11 | the unmutated candidate, for the S11 site table (the whole core tree: the table reads other crates) | control: passes | — |  |
| K4-M32 | rcond from the unequilibrated K | killed | CTRL |  |
| K4-M33 | a prescribed row published from its p-rounded state | killed | COMB_PRESC |  |
| D1 | promote the binary64 assembled K to p (every entry rounded to binary64) | killed | NP, CTRL |  |
| D2 | round u to binary64 before recovery | killed | R_N06, CTRL |  |
| D3 | drop K_fc*u_c from the reduced rhs | killed | RHS |  |
| D4 | flip an axis sign in B | killed | RIGID |  |
| D5 | fix p at 128 (no escalation) | killed | K28, CTRL |  |
| D6 | accept on the screens alone (the decision always accepts) | killed | SKEW6, CTRL |  |
| D7 | disable the geometry check | killed | F_MECH, R1 |  |
| D10 | order the free DOFs by index (list order) instead of the pattern (RCM) | killed | O8, EUNIT |  |
| D13 | fold the loads at p instead of the ledger (reduced rhs) | killed | ECH | CEILING-S→- |
| D14 | combine term by term in binary64 (the combination's ledger folded) | killed | B1C, B1E |  |
| D15 | combination outputs out of the stop rule (accepted at their first candidate) | killed | COMB_RULE, CTRL |  |
| D16a | sequential addition at p in assembly | killed | DUP, ASMBITS |  |
| D16b | sequential addition at p in recovery (d = T u) | killed | R_FOLD |  |
| D17 | drop the verified-accuracy classification (every row relative) | killed | S8W, CLASSBITS |  |
| D20 | R = 10^9*2^-64 (rounded) in the threshold | killed | CLASSBITS, CLASSBOUND |  |
| D25 | k = 1 for the intensified stress factor | killed | STRESSK |  |
| D13b | fold the loads in binary64 in the ledger itself (every use of the ledger) | killed | B1L, R1, CTRL |  |

Killing tests: ASMBITS = assembled_entries_equal_the_fraction_emulation_bit_for_bit; B1C = b1_c_the_cancelled_1e80_terms_leave_the_1e_minus_8_load_exactly; B1E = b1_e_is_caught_the_combination_reproduces_the_truth_that_a_sum_of_states_loses; B1L = b1_l_is_exact_with_the_ledger; BLOCKS = blocks_are_the_free_free_components_and_data_follows_7a; CASELIM = a_case_limit_is_exhausted_exactly_where_its_work_ends; CLASSBITS = the_classification_matches_the_binary64_reimplementation_bit_for_bit; CLASSBOUND = the_classifications_boundaries_hold_exactly; COMB_ESC = a_combination_escalates_independently_of_its_operands; COMB_PRESC = a_combined_prescribed_value_is_published_from_its_exact_sum_rounded_once; COMB_RULE = a_combination_runs_its_own_stop_rule; CTRL = every_control_follows_gens_schedule_and_r7s_expectations_honestly; DUP = the_duplicate_operand_entry_is_one_exact_sum_where_a_sequential_fold_loses_the_weak_member; ECH = e_charge_and_e_estimate_equal_the_emulation_at_every_verification; ECH100 = e_charge_on_rf_large_at_100_members_at_256; EUC = e_uc_uc_s_and_b_equal_the_emulation_and_never_fall_below_the_norm_per_block; EUNIT = e_unit_g_the_bounded_operator_and_e_equal_the_generators_emulation_at_every_precision; EVERYQ = every_published_quantity_takes_part_in_the_stop_rule; F2FAM = v4s_f2_family_bounds_equal_the_emulation_and_are_never_below_the_exact_norm; F_ILL = a_constructed_ill_conditioned_matrix_whose_pivots_pass_escalates_on_rcond_at_128_only; F_MECH = witnessed_mechanisms_are_refused_before_any_attempt_and_the_rx_companion_solves; F_P64 = the_p_plus_64_residual_takes_one_correction_on_two_span_and_records_its_basis; F_RCM = the_rcm_port_orders_sparse_directs_small_graphs_as_sparse_direct_does; F_SPAN = the_span_decision_is_exact; G5 = sd_g5_decides_each_test_exactly_at_its_boundary_in_r7s_order; G5C = sd_g5_charge_boundaries_at_p_256_and_p_512_and_the_translation_row; G5G = sd_g5_the_g_check_passes_at_2_to_the_p_minus_16_and_fails_above; G5GATE = sd_g5_the_gates_bounded_test_at_its_boundary_and_the_best_state; GOLDEN = golden_work_counts; K28 = k_1e_minus_28_escalates_past_128_and_is_accepted_at_256_against_512; LOOP = factors_l_and_d_bits_are_pinned_and_the_shifted_loop_without_a_shift_reproduces_them; L_NET = the_accessor_nets_once_with_the_quantum_and_a_zero_is_plus_zero; L_PROJ = the_projection_matches_the_fraction_oracle_at_every_precision; L_TIE = the_ledger_enters_a_prescribed_coupled_rhs_exactly_even_when_its_tail_decides_a_tie; N0506 = n05_and_n06_are_accepted_at_128_against_256; NP = n05_and_n06_agree_with_the_intended_basis_and_n05_is_not_np_as_represented_answer; O8 = retained_states_equal_the_generators_emulation_of_the_method_bit_for_bit; PRED = the_predicate_is_decided_exactly_at_its_boundary; R1 = r1s_cases_through_the_adapter_pass_are_refused_or_are_not_covered_as_section_4_10_lists; RFL_CHAIN = rf_large_chain_ax_at_100_members_is_selected_at_128_and_honest; RFL_TREE = rf_large_tree_ax_at_100_members_is_selected_at_128_and_honest; RHS = the_reduced_rhs_enters_prescribed_columns_exactly_and_rounds_once; RIGID = every_element_annihilates_the_rigid_motions_to_within_64_units_of_2_to_minus_p; RONLY = the_reactions_only_control_is_rejected_at_128_by_a_reaction; R_EVERY = every_recovered_kind_matches_its_exact_reference; R_FOLD = a_recovered_action_is_one_exact_expansion_where_a_sequential_fold_loses_a_term; R_N06 = n06s_torque_and_spring_action_are_nonzero_and_correct; S11 = s11_site_table; S8W = input_derived_rows_and_s8_ws_far_node_rows_are_classified_as_the_design_says; SKEW6 = the_six_member_skew_run_is_rejected_at_128_and_its_verification_is_reused; STRESS = the_low_precision_stress_keeps_every_bound_above_the_exact_norm_and_m27s_does_not; STRESSK = stress_scales_and_intensified_factors_match_bit_for_bit; SUMCHARGE = a_sums_rounding_charges_the_limbs_it_rounds; V_HAT = e_hat_couples_force_and_moment_through_the_body_extent_in_binary64; WS_RV12 = rv12_counterexample_is_rounded_once_and_a_fold_misrounds_it; WS_SPAN = a_refused_span_adds_nothing; WS_TGT = targeted_sums_match_the_fraction_oracle; WS_ZERO = zero_negation_absolute_value_and_reuse; ZT = a_structural_zero_kind_is_accepted_through_the_coupled_scale
