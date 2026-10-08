| # | File (PP) | Hunk | Context (git's function line) | +lines | −lines | SP commits |
|---|---|---|---|---|---|---|
| 1 | `lib.rs` | `-2994,7 +2994,8` | fn permitted_run( | 2 | 1 | 603e238517 |
| 2 | `lib.rs` | `-3039,19 +3040,13` | fn receipt_encoding_detail(check: retained_wire::ReceiptCheck) -> Opti | 5 | 11 | 56c5579f07 |
| 3 | `lib.rs` | `-3066,39 +3061,111` | impl ReservedNotice { | 91 | 19 | 56c5579f07, 7458527ff7 |
| 4 | `lib.rs` | `-3156,7 +3223,8` | fn dn_trigger_excluded(seed: &retained_product::OrdinarySeed) -> bool  | 2 | 1 | 56c5579f07 |
| 5 | `lib.rs` | `-3173,49 +3241,89` | fn retained_w1( | 60 | 20 | 56c5579f07, 7458527ff7 |
| 6 | `lib.rs` | `-3226,13 +3334,11` | fn retained_w1( | 3 | 5 | 7458527ff7 |
| 7 | `lib.rs` | `-3249,6 +3355,8` | pub(crate) mod retained_tests_hooks { | 2 | 0 | 56c5579f07 |
| 8 | `lib.rs` | `-3332,20 +3440,24` | pub(crate) mod retained_tests_hooks { | 9 | 5 | 7458527ff7, 89222942ee |
| 9 | `lib.rs` | `-3355,6 +3467,8` | pub(crate) mod retained_tests_hooks { | 2 | 0 | 7458527ff7 |
| 10 | `retained_product.rs` | `-64,7 +64,7` | pub(super) struct SolverObservations { | 1 | 1 | 7458527ff7 |
| 11 | `retained_product.rs` | `-128,7 +128,14` | pub(super) struct ProductCapture { | 8 | 1 | 7458527ff7 |
| 12 | `retained_product.rs` | `-147,8 +154,140` | pub(super) struct ProductCapture { | 133 | 1 | 56c5579f07, 7458527ff7 |
| 13 | `retained_product.rs` | `-687,7 +826,10` | impl ProductCapture { | 4 | 1 | 56c5579f07 |
| 14 | `retained_product.rs` | `-704,6 +846,21` | impl ProductCapture { | 15 | 0 | 56c5579f07 |
| 15 | `retained_product.rs` | `-716,7 +873,14` | impl ProductCapture { | 8 | 1 | 56c5579f07 |
| 16 | `retained_product.rs` | `-864,6 +1028,11` | impl ProductCapture { | 5 | 0 | 7458527ff7 |
| 17 | `retained_product.rs` | `-886,7 +1055,7` | impl ProductCapture { | 1 | 1 | 7458527ff7 |
| 18 | `retained_product.rs` | `-897,12 +1066,13` | impl ProductCapture { | 2 | 1 | 7458527ff7 |
| 19 | `retained_product.rs` | `-911,12 +1081,13` | impl ProductCapture { | 2 | 1 | 7458527ff7 |
| 20 | `retained_product.rs` | `-926,6 +1097,89` | impl ProductCapture { | 83 | 0 | 56c5579f07 |
| 21 | `retained_product.rs` | `-1293,7 +1547,8` | impl ProductCapture { | 2 | 1 | 56c5579f07 |
| 22 | `retained_product.rs` | `-1674,11 +1929,15` | impl ProductCapture { | 6 | 2 | 56c5579f07 |
| 23 | `retained_product.rs` | `-1713,7 +1972,8` | impl ProductCapture { | 2 | 1 | 7458527ff7 |
| 24 | `retained_product.rs` | `-1742,18 +2002,23` | impl ProductCapture { | 8 | 3 | 7458527ff7 |
| 25 | `retained_product.rs` | `-1767,7 +2032,7` | impl ProductCapture { | 1 | 1 | 7458527ff7 |
| 26 | `retained_product.rs` | `-1964,7 +2229,7` | impl ProductCapture { | 1 | 1 | 7458527ff7 |
| 27 | `retained_product.rs` | `-1984,7 +2249,7` | impl ProductCapture { | 1 | 1 | 7458527ff7 |
| 28 | `retained_product.rs` | `-2008,10 +2273,10` | impl ProductCapture { | 2 | 2 | 7458527ff7 |
| 29 | `retained_product.rs` | `-2217,6 +2482,7` | fn validate_final_metadata( | 1 | 0 | 7458527ff7 |
| 30 | `retained_product.rs` | `-2433,6 +2699,14` | fn validate_final_metadata( | 8 | 0 | 7458527ff7 |
| 31 | `retained_product.rs` | `-3194,10 +3468,13` | impl ProductCapture { | 6 | 3 | 56c5579f07 |
| 32 | `retained_product.rs` | `-3227,15 +3504,18` | impl ProductCapture { | 6 | 3 | 56c5579f07, 603e238517 |
| 33 | `retained_product.rs` | `-3247,11 +3527,10` | impl ProductCapture { | 4 | 5 | 56c5579f07, 7458527ff7, 59393e2d43, 0ec3651a36 |
| 34 | `retained_product.rs` | `-3307,6 +3586,360` | pub(super) struct PreparedCaseFailure { | 354 | 0 | 56c5579f07, 7458527ff7, 0ec3651a36, 8db7906ee7 |
| 35 | `retained_product.rs` | `-3316,131 +3949,136` | impl ProductCapture { | 126 | 121 | 56c5579f07, 7458527ff7, 0ec3651a36, 8db7906ee7 |
| 36 | `retained_product.rs` | `-3463,17 +4101,12` | impl PreparedCase { | 4 | 9 | 7458527ff7 |
| 37 | `retained_product.rs` | `-3490,16 +4123,35` | struct PreparedPayload { values:k::FrozenProductValues, maxima:Vec<Pre | 24 | 5 | 7458527ff7 |
| 38 | `retained_product.rs` | `-3514,11 +4166,11` | impl<'a> ProductCaseView<'a> { | 4 | 4 | 7458527ff7 |
| 39 | `retained_product.rs` | `-3549,7 +4201,7` | impl PreparedCandidateRefusal { | 1 | 1 | 7458527ff7 |
| 40 | `retained_product.rs` | `-3583,11 +4235,12` | impl ProductCapture { | 3 | 2 | 7458527ff7 |
| 41 | `retained_product.rs` | `-3638,15 +4291,15` | impl ProductCapture { | 4 | 4 | 7458527ff7 |
| 42 | `retained_product.rs` | `-3656,68 +4309,74` | impl PreparedCase { | 50 | 44 | 7458527ff7 |
| 43 | `retained_product.rs` | `-3725,62 +4384,80` | impl PreparedCase { | 48 | 30 | 7458527ff7 |
| 44 | `retained_product.rs` | `-3788,52 +4465,55` | fn apply_prepared_overlay(envelope:&mut MechanicsEnvelope,payload:&Pre | 31 | 28 | 7458527ff7 |
| 45 | `retained_receipt.rs` | `-123,7 +123,7` | fn summary_coverage<'a>(trace:&PreparedTrace,capture:&'a p::ProductCap | 1 | 1 | 7458527ff7 |
| 46 | `retained_receipt.rs` | `-174,7 +174,7` | pub(super) fn project<'a>(trace:&'a PreparedTrace,capture:&'a p::Produ | 1 | 1 | 7458527ff7 |
| 47 | `retained_receipt.rs` | `-191,7 +191,7` | pub(super) fn project<'a>(trace:&'a PreparedTrace,capture:&'a p::Produ | 1 | 1 | 7458527ff7 |
| 48 | `retained_wire.rs` | `-738,9 +738,18` | fn range_trigger(e: &Enc, trigger: &RangeTrigger) -> Value { | 10 | 1 | 7458527ff7 |
| 49 | `retained_wire.rs` | `-854,12 +863,16` | fn ordinary_members(e: &Enc, seed: &rp::OrdinarySeed) -> (Value, Value | 9 | 5 | 7458527ff7 |
| 50 | `retained_wire.rs` | `-886,7 +899,7` | fn material_basis(e: &Enc, capture: &rp::ProductCapture, raw: &Value,  | 1 | 1 | 7458527ff7 |
| 51 | `retained_wire.rs` | `-895,6 +908,8` | fn material_basis(e: &Enc, capture: &rp::ProductCapture, raw: &Value,  | 2 | 0 | 7458527ff7 |
| 52 | `retained_wire.rs` | `-964,7 +979,7` | fn case_source(e: &Enc, capture: &rp::ProductCapture, source: &k::Prim | 1 | 1 | 7458527ff7 |
| 53 | `retained_wire.rs` | `-1212,7 +1227,7` | fn public_error(e: &Enc, f: &rr::FailureRef<'_>, capture: &rp::Product | 1 | 1 | 7458527ff7 |
| 54 | `retained_wire.rs` | `-1261,7 +1276,8` | fn proof_trace(e: &Enc, view: &rr::PreparedAttemptView<'_>, capture: & | 2 | 1 | 7458527ff7 |
| 55 | `retained_wire.rs` | `-1292,7 +1308,7` | fn product_attempt(e: &Enc, view: &rr::PreparedAttemptView<'_>, case_i | 1 | 1 | 7458527ff7 |
| 56 | `retained_wire.rs` | `-1364,6 +1380,8` | fn kernel_outcome<'a>(e: &Enc, outcome: &'a k::ExecutionOutcome) -> (& | 2 | 0 | 7458527ff7 |
| 57 | `retained_wire.rs` | `-1379,10 +1397,21` | fn run_value(e: &Enc, run: &k::RunOrigins, records: &[k::AttemptRecord | 14 | 3 | 7458527ff7 |
| 58 | `retained_wire.rs` | `-1401,16 +1430,29` | fn invocation_arrays(e: &Enc, inv: &k::RecordedInvocation, run: &k::Ru | 14 | 1 | 7458527ff7 |
| 59 | `retained_wire.rs` | `-1421,28 +1463,44` | fn ordinary_value(e: &Enc, env: &Value, case_id: &str, case_index: usi | 26 | 10 | 7458527ff7 |
| 60 | `retained_wire.rs` | `-1453,10 +1511,11` | fn finish(e: Enc, mut env: Value, invocation: &source_receipt::Capture | 4 | 3 | 56c5579f07 |
| 61 | `retained_wire.rs` | `-1469,13 +1528,6` | pub(super) fn serialize_selected(candidate: &rp::PrivatePreparedCandid | 0 | 7 | 7458527ff7 |
| 62 | `retained_wire.rs` | `-1489,13 +1541,6` | impl SelectedCandidate for rp::PrivatePreparedCandidate { | 0 | 7 | 7458527ff7 |
| 63 | `retained_wire.rs` | `-1506,7 +1551,7` | fn serialize_selected_from(candidate: &impl SelectedCandidate, overlai | 1 | 1 | 7458527ff7 |
| 64 | `retained_wire.rs` | `-1539,8 +1584,8` | fn serialize_selected_from(candidate: &impl SelectedCandidate, overlai | 2 | 2 | 56c5579f07, 7458527ff7 |
| 65 | `retained_wire.rs` | `-1550,10 +1595,10` | fn serialize_selected_from(candidate: &impl SelectedCandidate, overlai | 4 | 4 | 7458527ff7 |
| 66 | `retained_wire.rs` | `-1575,9 +1620,13` | pub(super) const UNAVAILABLE_MESSAGE: &str = "Retained-precision recov | 6 | 2 | 7458527ff7 |
| 67 | `retained_wire.rs` | `-1608,7 +1657,7` | pub(super) fn serialize_unavailable(refused: Refused<'_>, invocation:  | 1 | 1 | 7458527ff7 |
| 68 | `retained_wire.rs` | `-1633,8 +1682,8` | pub(super) fn serialize_unavailable(refused: Refused<'_>, invocation:  | 2 | 2 | 7458527ff7 |
| 69 | `retained_wire.rs` | `-1648,7 +1697,7` | pub(super) fn serialize_unavailable(refused: Refused<'_>, invocation:  | 1 | 1 | 7458527ff7 |
| 70 | `retained_wire.rs` | `-1668,18 +1717,292` | pub(super) fn serialize_unavailable(refused: Refused<'_>, invocation:  | 278 | 4 | 56c5579f07, 7458527ff7 |
| 71 | `retained_wire.rs` | `-1979,7 +2302,8` | pub(super) fn test_after_conserved(before: u64, increment: u64, after: | 2 | 1 | 7458527ff7 |
| 72 | `retained_facade_tests.rs` | `-406,7 +406,9` | fn u3_n9_single_parse_custody() { | 3 | 1 | 7458527ff7 |
| 73 | `retained_facade_tests.rs` | `-482,6 +484,11` | fn u3_capture_permit_is_linear() { | 5 | 0 | 7458527ff7 |
| 74 | `retained_facade_tests.rs` | `-495,8 +502,10` | fn u3_r2_base_readers_accept_the_unavailable_notice() { | 3 | 1 | 7458527ff7 |
| 75 | `retained_facade_tests.rs` | `-506,7 +515,11` | fn u3_r2_base_readers_accept_the_unavailable_notice() { | 5 | 1 | 7458527ff7 |
| 76 | `retained_facade_tests.rs` | `-1325,3 +1338,718` | fn b1_t4_retained_w1_applies_decision_21_and_keeps_a_seedless_case() { | 715 | 0 | 56c5579f07, 7458527ff7, 89222942ee, 0ec3651a36, c17340d50b, 603e238517 |
| 77 | `retained_product_tests.rs` | `-134,7 +134,7` | fn i50_dump(e: &MechanicsEnvelope, o: &ProductCapture, mode: PreviewSo | 1 | 1 | 7458527ff7 |
| 78 | `retained_product_tests.rs` | `-194,7 +194,7` | fn i50_actual_named_case_both_modes_complete_private_verdict() { | 1 | 1 | 7458527ff7 |
| 79 | `retained_product_tests.rs` | `-443,7 +443,7` | fn i50_actual_support_bijections_and_accounting_prefixes() { | 1 | 1 | 7458527ff7 |
| 80 | `retained_product_tests.rs` | `-619,7 +619,7` | fn i50_support_coverage_native_non_aliasing_and_g5a() { | 1 | 1 | 7458527ff7 |
| 81 | `retained_product_tests.rs` | `-914,7 +914,7` | fn i50_observation_custody_presence_fields_and_failure_prefixes() { | 1 | 1 | 7458527ff7 |
| 82 | `retained_product_tests.rs` | `-1048,7 +1048,7` | fn actual_ordinary_zero_loaded_capture_and_final_verdict() { | 1 | 1 | 7458527ff7 |
| 83 | `retained_product_tests.rs` | `-1140,7 +1140,7` | fn actual_ordinary_zero_loaded_capture_and_final_verdict() { | 1 | 1 | 7458527ff7 |
| 84 | `retained_product_tests.rs` | `-1442,7 +1442,7` | fn adapter_byte_prefix_count_range_and_loss_are_typed() { | 1 | 1 | 7458527ff7 |
| 85 | `retained_product_tests.rs` | `-1470,7 +1470,7` | fn actual_sparse_zero_and_cancelled_summary_coverage_is_not_inferred_f | 1 | 1 | 7458527ff7 |
| 86 | `retained_product_tests.rs` | `-1486,7 +1486,7` | fn actual_sparse_zero_and_cancelled_summary_coverage_is_not_inferred_f | 1 | 1 | 7458527ff7 |
| 87 | `retained_product_tests.rs` | `-1603,7 +1603,7` | fn rv60_fixed_mode_sign_refuses_in_isolated_synthetic_zero_snapshot()  | 1 | 1 | 7458527ff7 |
| 88 | `retained_product_tests.rs` | `-1798,7 +1798,7` | fn i47_actual_selected_material_zero_loaded_capture_and_final_verdict( | 1 | 1 | 7458527ff7 |
| 89 | `retained_product_tests.rs` | `-2008,7 +2008,7` | fn i47_actual_selection_and_resolver_validity_controls() { | 1 | 1 | 7458527ff7 |
| 90 | `retained_product_tests.rs` | `-2025,7 +2025,7` | fn i47_modulus_record_closed_binding_and_independent_presence() { | 1 | 1 | 7458527ff7 |
| 91 | `retained_product_tests.rs` | `-2170,7 +2170,7` | fn i47_modulus_record_closed_binding_and_independent_presence() { | 1 | 1 | 7458527ff7 |
| 92 | `retained_product_tests.rs` | `-2348,7 +2348,7` | fn i47_successful_aggregate_capture_sticky_errors_and_work_prefixes()  | 1 | 1 | 7458527ff7 |
| 93 | `retained_product_tests.rs` | `-2393,7 +2393,7` | fn i51_first_prepared_native_both_modes() { | 1 | 1 | 7458527ff7 |
| 94 | `retained_product_tests.rs` | `-2405,7 +2405,7` | fn i51_first_prepared_native_both_modes() { | 1 | 1 | 7458527ff7 |
| 95 | `retained_product_tests.rs` | `-2463,7 +2463,7` | fn i51_c0_isolated_late_hook_custody_and_prefixes() { | 1 | 1 | 7458527ff7 |
| 96 | `retained_product_tests.rs` | `-2495,7 +2495,7` | fn i51_c0_isolated_late_hook_custody_and_prefixes() { | 1 | 1 | 7458527ff7 |
| 97 | `retained_product_tests.rs` | `-2503,9 +2503,11` | fn i51_c0_isolated_late_hook_custody_and_prefixes() { | 4 | 2 | 56c5579f07 |
| 98 | `retained_product_tests.rs` | `-2515,11 +2517,21` | fn i51_c0_isolated_late_hook_custody_and_prefixes() { | 11 | 1 | 56c5579f07, 7458527ff7 |
| 99 | `retained_product_tests.rs` | `-2531,7 +2543,7` | fn i51_c0_isolated_late_hook_custody_and_prefixes() { | 1 | 1 | 7458527ff7 |
| 100 | `retained_product_tests.rs` | `-2568,7 +2580,7` | fn i51_c0_isolated_guard_accounting_boundaries() { | 1 | 1 | 7458527ff7 |
| 101 | `retained_product_tests.rs` | `-2620,11 +2632,11` | fn i51_actual_exact_pressure_selection_suppresses_all_prepared_work()  | 2 | 2 | 7458527ff7 |
| 102 | `retained_product_tests.rs` | `-2642,7 +2654,7` | fn i51_ready_for_controls()->(MechanicsEnvelope,retained_product::Prep | 1 | 1 | 7458527ff7 |
| 103 | `retained_product_tests.rs` | `-2670,7 +2682,7` | fn i51_frozen_owner_values_and_numeric_refusal_controls() { | 1 | 1 | 7458527ff7 |
| 104 | `retained_product_tests.rs` | `-2919,7 +2931,7` | fn prepared_trace_prior_owned_cause_precedes_sticky_adapter_and_fresh_ | 1 | 1 | 7458527ff7 |
| 105 | `retained_product_tests.rs` | `-3055,7 +3067,7` | fn i61_fk_certificate_failure(variant:usize)->(retained_product::Prepa | 1 | 1 | 7458527ff7 |
| 106 | `retained_wire_tests.rs` | `-174,7 +174,7` | fn u2_foreign_owner_refused() { | 1 | 1 | 7458527ff7 |
| 107 | `retained_wire_tests.rs` | `-234,7 +234,7` | fn u1_stage_slots_require_an_exact_status() { | 1 | 1 | 7458527ff7 |
| 108 | `retained_wire_tests.rs` | `-518,7 +518,7` | fn u1g2_d38_failure_before_any_run() { | 1 | 1 | 7458527ff7 |
| 109 | `retained_wire_tests.rs` | `-574,7 +574,7` | fn u2_failure_path_foreign_owner_refused() { | 1 | 1 | 7458527ff7 |
| 110 | `retained_wire_tests.rs` | `-594,7 +594,7` | fn u1g2_failed_verification_keeps_its_reason() { | 1 | 1 | 7458527ff7 |
| 111 | `retained_wire_tests.rs` | `-705,7 +705,7` | fn u1g2_run_and_body_charge_conservation_negatives() { | 1 | 1 | 7458527ff7 |
| 112 | `retained_tests_hooks/grant2.rs` | `-14,9 +14,10` | use super::{arm, consume, Armed}; | 3 | 2 | 7458527ff7 |
| 113 | `retained_tests_hooks/grant2.rs` | `-27,11 +28,23` | pub(crate) struct Counts { pub(crate) runs: usize, pub(crate) complete | 13 | 1 | 7458527ff7 |
| 114 | `retained_tests_hooks/grant2.rs` | `-48,10 +61,12` | pub(super) fn merge(a: &mut Armed, faults: &Armed) { | 4 | 2 | 56c5579f07 |
| 115 | `retained_tests_hooks/grant2.rs` | `-64,10 +79,23` | pub(crate) fn fail_next_late_gate() { arm(\|a\| a.late_gate = true); } | 14 | 1 | 56c5579f07, 0ec3651a36 |

Hunks: 115; with no attributed commit: 0
