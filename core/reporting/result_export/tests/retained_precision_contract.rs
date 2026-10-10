//! Shared statement controls: synthetic bases, plus the two listed producer-solved L = 0
//! bases (07l), which the Direct entry published in the registered dev/test build (their own
//! `provenance` and `qualification` say so), and B1's reader-local n-case receipts derived
//! from the synthetic bases. Reading them establishes no execution; no native Current evidence.
use open_pipe_stress_result_export::retained_precision as rp;
use serde_json::Value;
fn corpus() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/results/retained_precision_cases.json"
    ))
    .unwrap()
}
fn decode(v: &Value) -> f64 {
    f64::from_bits(u64::from_str_radix(v.as_str().unwrap(), 16).unwrap())
}
#[test]
fn fixed_small_bound_vectors() {
    for row in corpus()["arithmetic"]["small_bounds"].as_array().unwrap() {
        assert_eq!(
            rp::absolute_bound(decode(&row["value"]), decode(&row["scale"]))
                .unwrap()
                .to_bits(),
            decode(&row["expected"]).to_bits(),
            "{}",
            row["id"]
        );
    }
}
#[test]
fn finite_product_vectors_and_ranges() {
    for row in corpus()["arithmetic"]["products"].as_array().unwrap() {
        let result = rp::upward_product(decode(&row["a"]), decode(&row["b"]));
        if row["expected"].is_null() {
            assert!(result.is_err());
        } else {
            assert_eq!(
                result.unwrap().to_bits(),
                decode(&row["expected"]).to_bits()
            );
        }
    }
    assert_eq!(
        rp::upward_small_sum(1.0, f64::from_bits(1))
            .unwrap()
            .to_bits(),
        0x3ff0000000000001
    );
    assert_eq!(rp::upward_product(-0.0, 1.0).unwrap().to_bits(), 0);
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
        assert!(rp::upward_product(value, 1.0).is_err());
    }
}

/// D32 (RV79-N-e) and the 07e format rule (RV78-N1): a rehash or edit-path
/// index is a strict integral value: a JSON number, never a boolean, finite,
/// integral, >= 0 and not -0.
fn index(v: &Value) -> Option<usize> {
    let n = v.as_f64()?;
    (n.is_finite() && n >= 0.0 && n.fract() == 0.0 && !(n == 0.0 && n.is_sign_negative()))
        .then_some(n as usize)
}
/// The 07e format rule (SHARED_SNAPSHOT_07E `format_rule`) with 07o's steps
/// (B2-C REVISION_01 §5.2, S-6), in dependency order: 1 preparation hashes, for
/// each `sources[*].preparation.attempt_ref` that resolves and whose members are
/// all prepared; 2 operand-preparation hashes, for each
/// `sources[*].preparation.operand_preparation_ref` that resolves to a prepared
/// record whose members are all prepared (DEF-O's H); 3 source identities, for
/// each selected case whose `source_ref` resolves; 4 each CombinationSource
/// operand's identity, for each operand `source_ref` that resolves; 5 each
/// `retained_selected` combination's identity, when its `source_ref` resolves;
/// 6 publication hash; 7 receipt hash. A reference that is not an index, or
/// does not resolve, is skipped.
fn rehash(source: &mut Value) {
    // S-1 (REVISION_01 §2): every preparation hash carries the route's
    // definition H: DEF-E's on the exact identity, DEF-O's otherwise.
    let h = if source["producer"]["semantic_contract_id"] == rp::EXACT_CONTRACT_ID {
        rp::EXACT_DEFINITION_HASH
    } else {
        rp::DEFINITION_HASH
    };
    rehash_with(source, h);
}
/// The 07e format rule with an explicit preparation-payload definition H.
fn rehash_with(source: &mut Value, definition_hash: &str) {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    // Snapshot 07 format: an entry that removes retained_precision or its body
    // (a G0 pin) has nothing to rehash.
    if !source
        .get("retained_precision")
        .and_then(|r| r.get("body"))
        .is_some_and(Value::is_object)
    {
        return;
    }
    let b = &mut source["retained_precision"]["body"];
    let attempts = b["product_attempts"].clone();
    for s in b["sources"].as_array_mut().unwrap() {
        if let Some(ai) = index(&s["preparation"]["attempt_ref"]) {
            let a = &attempts[ai];
            // As G1: only an addressable attempt has a preparation digest.
            if !a.is_object() {
                continue;
            }
            if a["preparation"]["members"]
                .as_array()
                .unwrap()
                .iter()
                .all(|m| m["result"]["kind"] == "prepared")
            {
                let members: Vec<_> = a["preparation"]["members"].as_array().unwrap().iter().map(|m| serde_json::json!({"member":m["member"],"old_source":m["old_source"],"old_facts":m["old_facts"],"section":m["result"]["section"]})).collect();
                let payload = serde_json::json!({"definition_id":a["definition_id"],"definition_sha256":definition_hash,"owner_ref":a["owner_ref"],"ordinary_attempt_ref":a["ordinary_attempt_ref"],"material_basis_ref":a["material_basis_ref"],"members":members});
                s["preparation"]["sha256"] =
                    domain_hash("retained_precision_preparation_v1", &payload)
                        .unwrap()
                        .into();
            }
        }
    }
    // S-6 step 2: operand-preparation hashes (C3a-5), with DEF-O's H.
    let records = b["operand_preparations"].clone();
    for s in b["sources"].as_array_mut().unwrap() {
        if let Some(pi) = index(&s["preparation"]["operand_preparation_ref"]) {
            let p = &records[pi];
            if !p.is_object() || p["result"]["kind"] != "prepared" {
                continue;
            }
            if p["preparation"]["members"]
                .as_array()
                .unwrap()
                .iter()
                .all(|m| m["result"]["kind"] == "prepared")
            {
                let members: Vec<_> = p["preparation"]["members"].as_array().unwrap().iter().map(|m| serde_json::json!({"member":m["member"],"old_source":m["old_source"],"old_facts":m["old_facts"],"section":m["result"]["section"]})).collect();
                let payload = serde_json::json!({"definition_id":p["definition_id"],"definition_sha256":rp::DEFINITION_HASH,"owner_ref":p["owner_ref"],"ordinary_attempt_ref":p["ordinary_attempt_ref"],"material_basis_ref":p["material_basis_ref"],"purpose":p["purpose"],"members":members});
                s["preparation"]["sha256"] =
                    domain_hash("retained_precision_operand_preparation_v1", &payload)
                        .unwrap()
                        .into();
            }
        }
    }
    let sources = b["sources"].clone();
    for c in b["cases"].as_array_mut().unwrap() {
        if c["status"] == "selected" {
            // Only a resolving source has an identity digest to recompute.
            let Some(mut s) = index(&c["source_ref"])
                .and_then(|i| sources.get(i))
                .filter(|s| s.is_object())
                .cloned()
            else {
                continue;
            };
            s.as_object_mut().unwrap().remove("index");
            c["source_identity_sha256"] = domain_hash("retained_precision_source_mp_v2", &s)
                .unwrap()
                .into();
        }
    }
    // S-6 step 4: each CombinationSource operand's identity of the CaseSource it names.
    let identity = |s: &Value| {
        let mut s = s.clone();
        s.as_object_mut().unwrap().remove("index");
        Value::from(domain_hash("retained_precision_source_mp_v2", &s).unwrap())
    };
    for s in b["sources"].as_array_mut().unwrap() {
        if s["owner"]["kind"] != "combination" {
            continue;
        }
        for o in s["operands"].as_array_mut().into_iter().flatten() {
            if let Some(named) = index(&o["source_ref"]).and_then(|i| sources.get(i)).filter(|x| x.is_object()) {
                o["source_identity_sha256"] = identity(named);
            }
        }
    }
    // S-6 step 5: each retained_selected combination's identity of its CombinationSource.
    let sources = b["sources"].clone();
    for c in b["combinations"].as_array_mut().into_iter().flatten() {
        if c["disposition"] != "retained_selected" {
            continue;
        }
        if let Some(s) = index(&c["source_ref"]).and_then(|i| sources.get(i)).filter(|x| x.is_object()) {
            c["source_identity_sha256"] = identity(s);
        }
    }
    let mut public = source.clone();
    public.as_object_mut().unwrap().remove("retained_precision");
    source["retained_precision"]["body"]["publication_sha256"] =
        domain_hash("retained_precision_publication_mp_v2", &public)
            .unwrap()
            .into();
    source["retained_precision"]["receipt_sha256"] = domain_hash(
        "retained_precision_receipt_mp_v2",
        &source["retained_precision"]["body"],
    )
    .unwrap()
    .into();
}
fn edit(source: &mut Value, e: &Value) {
    let path = e["path"].as_array().unwrap();
    let mut parent = source;
    for p in &path[..path.len() - 1] {
        parent = if let Some(i) = index(p) {
            &mut parent[i]
        } else {
            &mut parent[p.as_str().unwrap()]
        };
    }
    let last = path.last().unwrap();
    if e["op"] == "remove" {
        if let Some(i) = index(last) {
            parent.as_array_mut().unwrap().remove(i);
        } else {
            parent
                .as_object_mut()
                .unwrap()
                .remove(last.as_str().unwrap());
        }
    } else if let Some(i) = index(last) {
        parent[i] = e["value"].clone();
    } else {
        parent[last.as_str().unwrap()] = e["value"].clone();
    }
}
/// The reader's own expectation: `expected_by_reader.rust` when present (the
/// per-language G7 base code), else the shared `expected` (snapshot 06b format).
fn expected_for(m: &Value) -> &Value {
    m.get("expected_by_reader")
        .and_then(|e| e.get("rust"))
        .unwrap_or(&m["expected"])
}
/// Apply a shared mutation or must-pass entry exactly as SHARED_SNAPSHOT_06C
/// `format_change` specifies: edit a copy of the base source; edit a copy of
/// the base invocation; when invocation edits exist, bind the receipt's
/// invocation digest to the edited invocation; then rehash per `rehash`.
/// Returns the edited source and the invocation to validate against.
fn apply_entry(shared: &Value, entry: &Value) -> (Value, Value) {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    let case = shared["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == entry["base"])
        .unwrap();
    let mut source = case["source"].clone();
    for e in entry["edits"].as_array().unwrap() {
        edit(&mut source, e);
    }
    let mut invocation = case["invocation"].clone();
    let invocation_edits = entry["invocation_edits"].as_array().cloned().unwrap_or_default();
    for e in &invocation_edits {
        edit(&mut invocation, e);
    }
    if !invocation_edits.is_empty() {
        source["retained_precision"]["body"]["invocation"]["value"] =
            domain_hash("source_blocks_invocation_v1", &invocation)
                .unwrap()
                .into();
    }
    // The shared format admits only rehash:"all" (D11).
    assert_eq!(entry["rehash"], "all", "{}", entry["id"]);
    rehash(&mut source);
    // D24: the optional `after_rehash` edit list is applied after the rehash.
    for e in entry["after_rehash"].as_array().cloned().unwrap_or_default() {
        edit(&mut source, &e);
    }
    (source, invocation)
}
#[test]
fn complete_synthetic_controls_carry_their_shared_eligibility() {
    let shared = corpus();
    for case in shared["cases"].as_array().unwrap() {
        // PR-B2 ruling 5: a base the corpus states refused bound (`expected` a gate
        // and code) is refused there, and passes unbound and on transport.
        if case["expected"].get("gate").is_some() {
            assert_eq!(observe_validation(rp::validate(&case["source"], Some(&case["invocation"]))), case["expected"], "{}", case["id"]);
            assert!(matches!(rp::validate(&case["source"], None), Ok(v) if !v.numerical_eligible), "{}", case["id"]);
            assert!(matches!(rp::validate_transport_metadata(&case["source"]), Ok(v) if !v.numerical_eligible), "{}", case["id"]);
            continue;
        }
        let got = rp::validate(&case["source"], Some(&case["invocation"]))
            .unwrap_or_else(|e| panic!("{}: {e:?}", case["id"]));
        assert!(got.invocation_bound);
        // U7 (07i, D-U7-2): 15 bases are eligible (07l: 13 synthetic plus the two
        // producer-solved L = 0 bases); the two with an unavailable case are not.
        assert_eq!(
            got.numerical_eligible,
            case["expected"]["numerical_eligible"].as_bool().unwrap(),
            "{}: the shared eligibility",
            case["id"]
        );
        assert_eq!(
            got.publication_sha256,
            case["source"]["retained_precision"]["body"]["publication_sha256"]
        );
        let expected = case["expected_classifications"].as_array().unwrap();
        assert_eq!(got.classifications.len(), expected.len());
        for (got, want) in got.classifications.iter().zip(expected) {
            assert_eq!(got.result_id, want["result_id"]);
            assert_eq!(got.basis_ref, want["basis_ref"]);
            assert_eq!(
                format!("{:016x}", got.normalized_bits),
                want["normalized_bits"]
            );
            assert_eq!(
                got.scale_bits.map(|b| format!("{b:016x}")),
                want["scale_bits"].as_str().map(str::to_owned)
            );
            let name = match got.class {
                rp::AccuracyClass::RelativeVerified => "relative_verified",
                rp::AccuracyClass::AbsoluteVerified { bound_bits } => {
                    assert_eq!(format!("{bound_bits:016x}"), want["bound_bits"]);
                    "absolute_verified"
                }
                rp::AccuracyClass::InputDerived => "input_derived",
                rp::AccuracyClass::NonQuantity => "non_quantity",
                rp::AccuracyClass::NotCovered => "not_covered",
            };
            assert_eq!(name, want["class"]);
        }
        let unbound = rp::validate(&case["source"], None).unwrap();
        assert!(!unbound.invocation_bound && !unbound.numerical_eligible);
        let transport = rp::validate_transport_metadata(&case["source"]).unwrap();
        assert!(
            !transport.invocation_bound
                && !transport.numerical_eligible
                && transport.classifications.is_empty()
        );
    }
}
#[test]
fn shared_rehashed_first_failure_mutations() {
    let shared = corpus();
    let mut failures = Vec::new();
    for mutation in shared["mutations"].as_array().unwrap() {
        let (source, invocation) = apply_entry(&shared, mutation);
        let expected = expected_for(mutation);
        match rp::validate(&source, Some(&invocation)) {
            Err(e) if e.gate == expected["gate"] && e.code == expected["code"] => {}
            Err(got) => failures.push(format!(
                "{} expected {} got {got:?}",
                mutation["id"], expected
            )),
            Ok(got) => failures.push(format!(
                "{} expected {} got admitted statement (eligible={})",
                mutation["id"], expected, got.numerical_eligible
            )),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Every shared mutation's id, in corpus order (07m's 294, 07n's 240, then 07o's 69), pinned here
/// so that `slice_outcomes` checks each slice's entries by id and position (I83 §7
/// item 8; B1 SC item 12): a reorder of same-code entries inside a slice, which the
/// code tally cannot see, fails. Generated from 07n (sha256 `ea113e7b…e283`).
const MUTATION_IDS: [&str; 603] = [
    "method_null",
    "method_number",
    "method_alternate_metadata",
    "method_alternate_container",
    "method_empty",
    "method_wrong",
    "method_missing",
    "nonfinite_then_method",
    "shape_then_encoding",
    "negative_zero_counter",
    "coverage_then_method",
    "member_prefix_coverage",
    "diagnostic_then_method",
    "native_work",
    "product_new_not_ready",
    "body_scale",
    "section_echo",
    "absolute_bound",
    "input_dof",
    "old_inputs_must_bind_old_J",
    "new_stiffness_must_bind_C2",
    "bad_invocation_digest",
    "v1_relabel",
    "native_source_digest_rehashed",
    "native_stiffness_digest_rehashed",
    "physical_residual_basis",
    "physical_stop_stage_partition",
    "negative_zero_scale_with_wrong_method",
    "shared_stage_projection_preserves_total",
    "verification_own_stage_projection_preserves_total",
    "coverage_member_missing",
    "coverage_object_not_array",
    "coverage_entry_null",
    "coverage_stop_three",
    "coverage_stop_five",
    "coverage_stop_numeric_flag",
    "coverage_stop_null_flag",
    "coverage_has_data_numeric",
    "coverage_has_data_null",
    "coverage_has_data_missing",
    "coverage_unknown_field",
    "coverage_body_string",
    "coverage_shape_then_duplicate",
    "coverage_defect_with_null_method",
    "coverage_body_negative_zero",
    "coverage_body_fraction",
    "coverage_body_negative",
    "coverage_encoding_then_duplicate",
    "coverage_empty_array",
    "coverage_duplicate_body",
    "coverage_foreign_body",
    "coverage_extra_body",
    "coverage_duplicate_then_flag_and_method",
    "attempt_owner_only",
    "coverage_null_on_ready",
    "coverage_null_then_method",
    "coverage_has_data_false",
    "certified_bound_unbound_drop_existing_g5",
    "coverage_drop_certified_bound",
    "coverage_no_data_claim_with_free_loads",
    "coverage_stop_forbidden_entry",
    "coverage_stop_uncoupled_consistent_roster",
    "stop_rule_drop_required_zero",
    "stop_rule_duplicate_zero",
    "estimate_drop_required_zero",
    "estimate_forbidden_translation_zero",
    "charge_drop_required_zero",
    "charge_follows_stop_below_p512",
    "certified_bound_duplicate",
    "coverage_flag_defect_then_method",
    "nodata_forbidden_stop_zero",
    "nodata_forbidden_estimate_zero",
    "nodata_forbidden_charge_zero",
    "nodata_forbidden_certified_bound",
    "nodata_has_data_without_bound",
    "nodata_estimate_coupling_L_nonzero",
    "coverage_flag_defect_then_body_scale",
    "layout_force_row_input_derived",
    "layout_constrained_displacement_not_input_derived",
    "layout_nonzero_prescription",
    "coverage_null_and_product_work",
    "product_work_only",
    "native_work_then_coverage_null",
    "layout_row_relabelled",
    "layout_row_dropped",
    "layout_rows_reordered",
    "layout_row_foreign_body",
    "nodata_empty_coverage",
    "verification_bound_duplicate_entry",
    "verification_bound_missing_entry",
    "nodata_verification_bound_nonnull",
    "schedule_fresh_first_p256",
    "cancelled_no_data_claim",
    "cancelled_drop_B_only",
    "unavailable_certificate_passed_coverage_null",
    "unavailable_empty_coverage",
    "unavailable_no_data_claim_with_free_loads",
    "unavailable_stop_uncoupled",
    "unavailable_layout_relabelled",
    "unavailable_verification_bound_duplicate",
    "cert_failed_before_summary_g5a_passed",
    "lane_k_failed_with_coverage",
    "maxima_abandoned_with_coverage",
    "prefix_captured_with_members",
    "two_body_swap_has_data",
    "two_body_swap_has_data_with_rosters",
    "two_body_swap_stop_only",
    "two_body_body_order_swapped",
    "two_body_missing_body",
    "two_body_resolution_order_swapped",
    "p512_floor_not_phi",
    "p512_positive_floor_forces_stop",
    "p512_charge_follows_estimate",
    "p512_floor_null",
    "p512_zero_floor_body_given_positive_floor",
    "copy_flags_into_loaded",
    "rebind_source_run_only",
    "cross_case_gate_order_selected",
    "cross_case_gate_order_unavailable",
    "unavailable_record_resolution_duplicate",
    "unavailable_record_theta_duplicate",
    "skip_after_failed_candidate_reused",
    "skip_after_failed_candidate_wrong_slot",
    "failed_build_reason_mismatch",
    "failed_slot_not_cached",
    "failed_verification_reused_as_candidate",
    "failed_verification_one_slot",
    "cached_failed_slot_rebuilt",
    "corrections_above_three",
    "group_sources_out_of_order",
    "w2_published_without_initial_failure",
    "legacy_source_dangling_diagnostic",
    "formation_d5_dangling_diagnostic",
    "native_stage_disagrees_with_run",
    "conversion_subnormal_kind_normal_bits",
    "conversion_normal_kind_subnormal_bits",
    "conversion_ready_underflow_nonzero_row",
    "conversion_ready_overflow",
    "conversion_subnormal_metadata_negative_zero",
    "g7_maximum_off_enclosure",
    "row_index_foreign_support_norm",
    "row_index_missing_row",
    "row_index_unsorted",
    "row_index_ready_subset",
    "maps_extra_support_id",
    "maps_member_ends_swapped",
    "maxima_abandoned_separate_failure",
    "certificate_check_wrong_wrapper",
    "certificate_stage_check_disagree",
    "stage_entered_after_failure",
    "ceiling_before_last_slot",
    "escalating_end_not_a_terminal_translation",
    "terminal_stop_wrong_translation",
    "terminal_refusal_wrong_kind",
    "idle_budget_below_invocation_limit",
    "idle_ready_group_not_ledger_refusal",
    "work_accounting_prior_not_on_wire",
    "refused_member_conversion_kind_bits",
    "prefix_attached_old_input_unbound",
    "report_reference_unrelated_diagnostic",
    "distinct_stiffness_merged_group",
    "named_point_value_mismatch",
    "interpolation_target_mismatch",
    "work_accounting_after_escalating_stop",
    "idle_work_accounting_run",
    "work_accounting_at_last_slot",
    "mm_unnormalized_coordinate",
    "interpolation_missing_alpha",
    "interpolation_duplicate_temperature",
    "interpolation_target_below_range",
    "interpolation_target_at_lower_point",
    "interpolation_target_at_upper_point",
    "interpolation_target_above_range",
    "adapter_fault_present",
    "accounting_cause_without_fault",
    "scalar_trace_lost_unavailable",
    "work_accounting_cause_exact_status",
    "idle_budget_not_exhausted_no_group",
    "execution_order_swapped",
    "run_id_not_execution_position",
    "old_members_reordered",
    "complete_old_short_of_source",
    "unsourced_old_member_noncontiguous",
    "complete_old_longer_than_source",
    "unavailable_source_backref_foreign",
    "attempt_basis_not_ordinary",
    "native_error_with_selected_run",
    "group_call_out_of_range",
    "escalating_failed_verification_pass_entered",
    "stop_rule_quantity_other_body",
    "candidate_record_with_verification",
    "ordinary_diagnostic_ref_duplicate",
    "ordinary_diagnostic_ref_dangling",
    "ordinary_dangling_plus_adapter_fault",
    "selected_case_ordinary_checks_passed",
    "section_accounting_exact_status",
    "nested_stop_work_accounting_exact_status",
    "view_work_fault_exact_status",
    "old_operational_accounting_not_lost",
    "interpolation_target_at_lower_point_equal_e",
    "interpolation_target_below_range_equal_e",
    "interpolation_target_at_upper_point_equal_e",
    "interpolation_target_above_range_equal_e",
    "source_preparation_null",
    "run_ref_null_with_case_run",
    "preparation_error_with_selected_run",
    "prepared_failure_cause_without_own_attempt",
    "prepared_failure_cause_foreign_attempt",
    "reason_code_phase_mismatch_on_facade",
    "reason_code_phase_mismatch_on_preparation",
    "rejected_verification_failed_with_completed_verification",
    "rejected_attempt_with_verified_record",
    "ordinary_attempt_order_swapped",
    "record_index_not_contiguous",
    "selected_verification_record_not_verified",
    "projection_conversions_count_mismatch",
    "preparation_conversion_count_mismatch",
    "retained_diagnostic_names_unrequested_case",
    "g0_case_limit_threshold",
    "g0_invocation_limit_threshold",
    "g0_receipt_version_not_1",
    "g0_canonicalization_profile",
    "g0_retained_precision_absent",
    "g0_receipt_body_absent",
    "refusal_work_accounting_variant",
    "refusal_count_range_variant",
    "unavailable_source_ref_absent",
    "source_decline_with_source_ref",
    "rcond_label_plus_adapter_fault",
    "native_attempt_defect_after_native_work_defect",
    "dangling_candidate_record",
    "dangling_build_ref",
    "dangling_ordinary_attempt_ref",
    "g5b_zero_section_area",
    "ordinary_dangling_ref_plus_source_preparation_null",
    "g5b_zero_section_length",
    "unavailable_attempt_under_facade_failure_cause",
    "unavailable_attempt_under_source_error_cause",
    "preparation_error_selected_run_under_receipt_cause",
    "selected_case_without_c3_attempt",
    "vbuild_on_escalating_failed_verification",
    "dangling_attempt_source_ref",
    "source_member_map_kernel_id_noncanonical",
    "forged_publication_hash",
    "forged_preparation_hash",
    "source_identity_stale_receipt_rehashed",
    "forged_receipt_hash",
    "g5a_sanity_margin_between_2m40_and_2m39",
    "idle_run_exhausted_meter_chain_broken",
    "verification_estimate_quantity_not_in_layout",
    "charge_quantity_not_in_layout",
    "publication_enclosure_quantity_not_in_layout",
    "empty_body_inventory",
    "verification_summary_on_escalating_failed_verification",
    "model_schema_version_0_4_0_rejected",
    "forged_source_identity_float_source_ref",
    "verification_estimate_names_translation_row",
    "ready_attempt_under_facade_failure_cause",
    "ready_attempt_under_prepared_product_failure",
    "g0_receipt_version_non_integral",
    "g0_case_limit_non_integral",
    "g0_invocation_limit_non_integral",
    "verification_estimate_names_rotation_row",
    "g5a_error_with_g5a_not_entered",
    "observable_error_with_observables_not_entered",
    "numeric_error_with_checks_not_entered",
    "proof_error_with_certificate_passed",
    "values_error_with_values_completed",
    "f5_ordinary_refs_omit_naming_diagnostic",
    "f5_ordinary_refs_out_of_envelope_order",
    "f5_ordinary_refs_list_retained_precision_m09",
    "f5_ordinary_refs_list_invocation_level_m10",
    "f5_ordinary_refs_list_other_case",
    "f5_ordinary_refs_relaxed_d6a_form",
    "f5_affected_refs_string_names_no_case",
    "f5_ordinary_refs_second_case_relaxed_d6a_form",
    "f5_ordinary_refs_strict_prefix",
    "g7_not_required_quality_enum_invalid",
    "isolated_rotation_stop_sparse_interactive",
    "isolated_has_data_sparse_interactive",
    "isolated_estimate_coupled_sparse_interactive",
    "isolated_rotation_stop_dense_scrutiny",
    "isolated_has_data_dense_scrutiny",
    "isolated_estimate_coupled_dense_scrutiny",
    "isolated_translation_rotation_stop_sparse_interactive",
    "isolated_translation_rotation_stop_dense_scrutiny",
    "g7_selected_quality_enum_invalid",
    "g7_unavailable_quality_enum_invalid",
    "g7_quality_case_evidence_ref_empty",
    "g7_quality_case_extra_member",
    "g7_quality_status_invalid",
    "g7_formulation_limitations_empty",
    "g7_contract_evidence_null",
    "g7_source_block_recovery_present",
    "d38_m1_error_kind_native",
    "d38_m2_native_completed",
    "d38_m3_run_ref_without_run",
    "d38_m4_execution_order_lists_case",
    "d38_m5_proof_start_completed",
    "d38_m6_attempt_source_null",
    "d38_m7_call_lists_source",
    "d38_m8_case_source_other",
    "d38_m9_case_c_builds_kept",
    "f1_p4_parity_on_w2_published_case_a_dense",
    "f1_p2_parity_duplicated_l0_dense",
    "f1_p3_parity_in_sparse_l0",
    "f1_p1_mode_code_3_sparse_l0",
    "f1_requested_mode_flipped_w_c2_case_b",
    "nr_w_c2_case_b_verdict_sensitive",
    "nr_w_c2_case_b_initial_not_attempted",
    "nr_w_c2_case_b_product_attempt_ref_own_attempt",
    "n2_w_c2_case_b_solve_quality_missing",
    "a4_reason_as_4b",
    "f_mb_index_1",
    "f_mb_index_swapped",
    "f_src_index_1",
    "f_src_index_swapped",
    "f_mb_case_indices_duplicate",
    "f_mb_case_indices_out_of_range",
    "f_src_owner_case_id_other",
    "f_src_owner_case_id_unknown",
    "f_src_owner_case_index_out_of_range",
    "f_src_owner_other_case_consistent",
    "f_ordinary_basis_ref_dangling",
    "f_mb_case_indices_empty",
    "f_b_basis_ref_7",
    "f_c_basis_omits_case_1",
    "f_c_basis_out_of_order",
    "f_src_owner_kind_combination",
    "g_combinations_null",
    "g_combinations_empty_object",
    "g_combinations_object",
    "g_combinations_string",
    "g_combinations_zero",
    "g_combinations_false",
    "g_combinations_nonempty",
    "g_components_null",
    "g_components_string",
    "g_components_empty_object",
    "g_components_nonempty",
    "g_reference_configurations_null",
    "g_reference_configurations_empty",
    "g_reference_configurations_object",
    "g_pressure_contract_empty_object",
    "g_pressure_contract_false",
    "g_pressure_contract_zero",
    "g_pressure_contract_empty_string",
    "g_pressure_contract_empty_array",
    "g_order_before_preparation",
    "g_order_control_preparation",
    "ca_receipt_phase_routing",
    "ca_receipt_phase_preparation",
    "ca_receipt_phase_kernel",
    "ca_receipt_phase_facade",
    "ca_receipt_code_source_unavailable",
    "ca_receipt_code_facade_certificate",
    "ca_receipt_code_kernel_selected",
    "ca_receipt_code_caller_not_qualified",
    "ca_facade_ok",
    "ca_facade_phase_routing",
    "ca_facade_phase_preparation",
    "ca_facade_phase_kernel",
    "ca_facade_phase_receipt",
    "ca_facade_code_receipt_encoding",
    "ca_facade_owner_other_case",
    "ca_facade_owner_combination",
    "ca_precondition_beside_run",
    "ca_kernel_beside_selected_run",
    "cb_source_error_no_decline",
    "cb_source_error_decline_error_differs",
    "cb_source_error_phase_routing",
    "cb_source_error_code_caller",
    "cb_receipt_phase_preparation",
    "cb_facade_no_run",
    "cb_kernel_no_run",
    "cb_control_ppf_reason_missing_attempt",
    "cb_pre_caller_caller_not_qualified_kernel",
    "cb_pre_caller_resource_admission_not_available_routing",
    "cb_pre_caller_source_unavailable_routing",
    "cb_pre_caller_upstream_no_wrap_not_established_routing",
    "cb_pre_resource_admission_resource_admission_not_available_kernel",
    "cb_pre_resource_admission_caller_not_qualified_routing",
    "cb_pre_resource_admission_source_unavailable_routing",
    "cb_pre_resource_admission_upstream_no_wrap_not_established_routing",
    "cb_pre_upstream_no_wrap_upstream_no_wrap_not_established_kernel",
    "cb_pre_upstream_no_wrap_caller_not_qualified_routing",
    "cb_pre_upstream_no_wrap_resource_admission_not_available_routing",
    "cb_pre_upstream_no_wrap_source_unavailable_routing",
    "cb_pre_capture_source_unavailable_kernel",
    "cb_pre_capture_caller_not_qualified_routing",
    "cb_pre_capture_resource_admission_not_available_routing",
    "cb_pre_capture_upstream_no_wrap_not_established_routing",
    "cb_pre_source_family_source_unavailable_kernel",
    "cb_pre_source_family_caller_not_qualified_routing",
    "cb_pre_source_family_resource_admission_not_available_routing",
    "cb_pre_source_family_upstream_no_wrap_not_established_routing",
    "cc_pre_caller_keyed",
    "cc_pre_resource_admission_keyed",
    "cc_pre_upstream_no_wrap_keyed",
    "cc_pre_capture_keyed",
    "cc_pre_source_family_keyed",
    "cc_pre_capture_cross",
    "cd_kernel_ok",
    "cd_kernel_code_unresolved",
    "cd_kernel_phase_facade",
    "cd_kernel_cause_other",
    "cd_ppf_control",
    "c2_receipt_phase_kernel",
    "c2_receipt_code_facade",
    "c2_receipt_phase_preparation",
    "c2_facade_ok",
    "c2_facade_phase_kernel",
    "h_recovery_present",
    "h_evidence_null",
    "h_evidence_array",
    "h_recovery_and_evidence_null",
    "h_carrier_present",
    "h_carrier_and_quality_defect",
    "h_carrier_and_recovery",
    "h_quality_extra_member",
    "h_quality_representation",
    "h_quality_quantization",
    "h_quality_policy",
    "h_quality_status_bogus",
    "h_quality_cases_object",
    "h_case_extra_member",
    "h_case_basis_ref_extra",
    "h_case_basis_ref_type_empty",
    "h_case_solve_quality_bogus",
    "h_case_structural_bogus",
    "h_case_fidelity_bogus",
    "h_case_accuracy_bogus",
    "h_case_evidence_refs_empty_string",
    "h_case_evidence_refs_not_list",
    "h_formulation_extra_member",
    "h_formulation_limitations_empty",
    "h_formulation_limitations_empty_string",
    "h_formulation_limitations_other",
    "h_quality_and_formulation",
    "h_case_and_formulation",
    "h_schema_version_010",
    "n6_structural_status_list",
    "n6_structural_status_dict",
    "n6_model_matrix_fidelity_list",
    "n6_model_matrix_fidelity_dict",
    "n6_accuracy_evidence_list",
    "n6_accuracy_evidence_dict",
    "n6_status_list",
    "n6_status_dict",
    "n6_carrier_evidence_with_case_defect",
    "n6_contract_evidence_null_and_source_block_recovery",
    "n4_null_first",
    "n4_null_appended",
    "n4_number_first",
    "n4_string_first",
    "n4_array_first",
    "r_b_basis_ref_7",
    "r_b_basis_ref_1_second_basis",
    "r_c_basis_omits_case_1",
    "r_c_cases_out_of_order",
    "r_c_extra_empty_basis",
    "r_d1_extra_member",
    "r_d2_mode_unknown",
    "r_d2_mode_list",
    "r_d2_mode_removed",
    "r_d2_mode_null",
    "r_e_elastic_modulus_wrong",
    "r_e_selection_base",
    "r_e_material_id_other",
    "r_e_no_material",
    "r_c_missing_sourceless_basis",
    "g8_unavailable_mode_code_2_in_sparse",
    "g8_unavailable_mode_code_3",
    "g8_unavailable_mode_row_duplicated",
    "g8_unavailable_mode_row_removed",
    "g8_unavailable_parity_in_sparse_P3",
    "g8_unavailable_parity_with_method_G6_first",
    "g8_unavailable_requested_mode_flipped",
    "g8_selected_mode_code_2_in_sparse",
    "dz_parity_twice_case1_P2",
    "dz_parity_case1_w2_published_P4",
    "dz_parity_case0_w2_published_P4",
    "fz_unavailable_two_parity_P2",
    "fz_unavailable_parity_w2_published_P4",
    "limit_l0_dense_parity_deleted_indices_unshifted",
    "limit_l0_sparse_mode_row_deleted_P1",
    "nr_report_outcome_sensitive",
    "nr_initial_not_attempted",
    "nr_verdict_sensitive",
    "nr_w2_published_verdict_sensitive",
    "nr_verdict_not_assessed",
    "nr_report_with_w2_published",
    "nr_product_attempt_ref_with_attempt",
    "nr_product_attempt_ref_dangling",
    "nz_w2_published_parity_P4",
    "orphan_source_beside_t7",
    "t_evidence_extra_member",
    "t_preview_cases_not_list",
    "t_case_extra_member",
    "t_case_id_empty",
    "t_case_id_duplicate",
    "t_coverage_complete_false",
    "t_coverage_overlap",
    "t_coverage_duplicate_ids",
    "t_attributed_and_withheld",
    "t_withheld_reason_unknown",
    "t_withheld_ok",
    "t_attribution_sets_differ",
    "t_withheld_duplicate_multiset",
    "t_extrema_extra_member",
    "t_extrema_approximation_other",
    "t_extrema_station_fraction_1_5",
    "t_extrema_local_fraction_negative",
    "t_extrema_span_index_negative",
    "t_extrema_span_index_fraction",
    "t_extrema_subdivisions_over",
    "t_extrema_bounds_inverted",
    "t_extrema_lower_negative",
    "t_extrema_upper_string",
    "t_extrema_global_upper_string",
    "t_extrema_certified_gap_null",
    "t_extrema_pipe_in_unavailable",
    "t_extrema_pipe_duplicate",
    "t_measure_ok",
    "t_measure_sif_zero",
    "t_measure_location_mid",
    "t_measure_moment_string",
    "t_measure_duplicate_result",
    "t_gate_withheld_reason_null",
    "t_gate_released_with_reason",
    "t_gate_duplicate",
    "t_gate_withheld_string",
    "t_measure_extra_member",
    "t_gate_extra_member",
    // 07o (B2-C REVISION_01 §5.1-§5.3; lane C's corpus): 69 more.
    "b2o_m01_combination_attempt_definition_unknown",
    "b2o_m02_operand_preparation_definition_unknown",
    "b2o_m03_projection_policy_changed",
    "b2o_m04_work_policy_changed",
    "b2o_m05_canonicalization_changed",
    "b2o_m06_work_case_limit_changed",
    "b2o_m07_work_invocation_limit_changed",
    "b2o_m08_case_attempt_definition_def_c",
    "b2o_m09_combination_attempt_definition_def_o",
    "b2o_m10_combination_identity_changed",
    "b2o_m11_operand_identity_changed",
    "b2o_m12_operand_prepared_hash_case_domain",
    "b2o_m13_operand_preparations_empty",
    "b2o_m14_ordinary_entry_run_null",
    "b2o_m15_withheld_result_ids_gain_row",
    "b2o_m16_term_factor_uppercase",
    "b2o_m17_requested_operand_factor_nan",
    "b2o_m18_entries_swapped",
    "b2o_m19_entry_removed",
    "b2o_m20_combination_renamed_to_case_id",
    "b2o_m21_result_ids_lose_row",
    "b2o_m22_result_ids_gain_case_row",
    "b2o_m23_result_ids_swapped",
    "b2o_m24_execution_order_loses_combination",
    "b2o_m25_execution_order_combination_index",
    "b2o_m26_combination_attempt_first",
    "b2o_m27_operand_preparation_owner_selected",
    "b2o_m28_operand_preparation_for_ordinary",
    "b2o_m29_operand_preparation_removed",
    "b2o_m30_operand_preparation_duplicated",
    "b2o_m31_combination_selected_diagnostic_removed",
    "b2o_m32_unavailable_diagnostic_ref_other",
    "b2o_m33_selected_diagnostic_on_ordinary",
    "b2o_m34_combination_diagnostic_two_refs",
    "b2o_m35_ordinary_term_selected_case",
    "b2o_m36_subtraction_recast_unavailable",
    "b2o_m37_withheld_reason_other_code",
    "b2o_m38_requested_operands_swapped",
    "b2o_m39_representative_operand_1",
    "b2o_m40_operand_case_index_other",
    "b2o_m41_import_from_operand_1",
    "b2o_m42_combination_group_imports_removed",
    "b2o_m43_combination_call_before_off",
    "b2o_m44_charged_batch_after",
    "b2o_m45_import_from_prepared_operand",
    "b2o_m46_pre_source_run_refs_gain_run",
    "b2o_m47_pre_source_after_ne_before",
    "b2o_m48_combination_reason_without_call",
    "b2o_m49_operand_index_selected_term",
    "b2o_m50_refused_record_source_ref",
    "b2o_m51_prepared_record_stage_failed",
    "b2o_m52_unresolved_attempt_native_completed",
    "b2o_m53_reason_code_kernel_unresolved",
    "b2o_m54_combination_translation_scale_ulp",
    "b2o_m55_combination_row_not_covered",
    "b2o_m56_method_on_unavailable_row",
    "b2o_m57_method_removed_from_selected_row",
    "b2o_m58_method_on_ordinary_row",
    "b2o_m59_range_mode_changed",
    "b2o_m60_subtraction_operands_swapped",
    "b2o_m61_range_operand_ids_reversed",
    "b2o_m62_kernel_source_replaced",
    "b2o_m63_operand_effective_wall_changed",
    "b2o_m64_operand_old_facts_d_changed",
    "b2o_m65_operand_preparation_definition_def_c",
    "b2o_m66_refused_record_origin_capture",
    "b2o_m67_combination_attempt_origin_capture",
    "b2o_m68_operand_prepared_source_after_combination",
    "b2o_m69_combination_magnitude_off",
];
/// Observe one slice of the shared mutations against this reader's own
/// expectation, print one outcome per mutation (visible with --nocapture) and
/// check the slice's ids (`MUTATION_IDS`) and its tally. 07h held 277 mutations;
/// RV94 N-3's G7 probe makes 278; 07l (U8-2) appends the 8 L = 0 mutations, making
/// 286; 07m (B6) appends 8 G7 mutations, making 294; 07n (B1 SC) appends 240,
/// making 534 (`snapshot_07n_counts_and_format` pins the counts), and no slice moves.
fn slice_outcomes(tag: &str, range: std::ops::Range<usize>, want: &[(&str, usize)]) {
    use std::collections::BTreeMap;
    let shared = corpus();
    let mutations = shared["mutations"].as_array().unwrap();
    let ids: Vec<&str> = mutations[range.clone()]
        .iter()
        .map(|m| m["id"].as_str().unwrap())
        .collect();
    assert_eq!(
        ids,
        MUTATION_IDS[range.clone()],
        "{tag}: the slice's ids, in order"
    );
    let mut tally = BTreeMap::new();
    let mut matched = 0;
    for mutation in &mutations[range.clone()] {
        let observed = observe(&shared, mutation);
        let expected = expected_for(mutation);
        let ok = observed == *expected;
        matched += usize::from(ok);
        *tally
            .entry(format!(
                "{} {}",
                expected["gate"].as_str().unwrap(),
                expected["code"].as_str().unwrap()
            ))
            .or_insert(0) += 1;
        println!(
            "{tag} {}",
            serde_json::json!({"id":mutation["id"],"base":mutation["base"],"expected":expected,"observed":observed,"match":ok})
        );
    }
    let want: BTreeMap<String, usize> = want.iter().map(|(k, n)| (k.to_string(), *n)).collect();
    assert_eq!(tally, want, "{tag}");
    assert_eq!(matched, range.len(), "{tag}");
}

/// Snapshot-03 controls (the first 30 shared mutations), so the outcome
/// listing covers all 178 (D15, RV78-N9).
#[test]
fn snapshot_03_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_03",
        0..30,
        &[
            ("G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", 1),
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 6),
            ("G2 RETAINED_PRECISION_ENCODING_MISMATCH", 3),
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 2),
            ("G4 RETAINED_PRECISION_DIAGNOSTIC_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 4),
            ("G5b RETAINED_PRECISION_SCALE_MISMATCH", 1),
            ("G5b RETAINED_PRECISION_SECTION_MISMATCH", 1),
            ("G5c RETAINED_PRECISION_CLASSIFICATION_MISMATCH", 1),
            ("G5c RETAINED_PRECISION_INPUT_DOF_MISMATCH", 1),
            ("G6 RETAINED_PRECISION_ROW_METHOD_MISMATCH", 3),
            ("G8 RETAINED_PRECISION_INVOCATION_MISMATCH", 1),
            ("G8 RETAINED_PRECISION_PREPARATION_MISMATCH", 4),
        ],
    );
}

/// Snapshot-04 summary-coverage controls (I57 s4/s5), mutations 30..77.
#[test]
fn snapshot_04_coverage_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME",
        30..77,
        &[
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 14),
            ("G2 RETAINED_PRECISION_ENCODING_MISMATCH", 4),
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 6),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 2),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 1),
            ("G5a RETAINED_PRECISION_SCALE_MISMATCH", 20),
        ],
    );
}

/// Snapshot-05a controls (I62 C1a), mutations 77..104.
#[test]
fn snapshot_05a_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_05A",
        77..104,
        &[
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 3),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 5),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 2),
            ("G5a RETAINED_PRECISION_SCALE_MISMATCH", 16),
        ],
    );
}

/// Snapshot-05b controls (I62 C1b, unchanged in 05c), mutations 104..121.
#[test]
fn snapshot_05b_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_05B",
        104..121,
        &[
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 2),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 1),
            ("G5a RETAINED_PRECISION_SCALE_MISMATCH", 13),
            ("G5b RETAINED_PRECISION_SCALE_MISMATCH", 1),
        ],
    );
}

/// Snapshot-06a checklist controls (I62 C2-1), mutations 121..150; 06b moved
/// `prefix_old_inputs_unbound` to the must-pass entries (P7 settlement), and
/// the G7 entry uses this reader's own base code (`expected_by_reader`).
#[test]
fn snapshot_06a_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_06A",
        121..150,
        &[
            ("G2 RETAINED_PRECISION_ENCODING_MISMATCH", 1),
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 3),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 10),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 10),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 2),
            ("G7 SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS", 1),
            ("G8 RETAINED_PRECISION_PREPARATION_MISMATCH", 2),
        ],
    );
}

/// Snapshot-06b settlements (I62 C2-2), mutations 150..163, with
/// `ceiling_before_last_slot` under its 06c name.
#[test]
fn snapshot_06b_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_06B",
        150..163,
        &[
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 8),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 1),
            ("G8 RETAINED_PRECISION_PREPARATION_MISMATCH", 3),
        ],
    );
}

/// Snapshot-06c (I62 C2-3), mutations 163..173: WorkAccounting terminals
/// rejected, and the invocation-level G8 refusals through `invocation_edits`.
#[test]
fn snapshot_06c_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_06C",
        163..173,
        &[
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 3),
            ("G8 RETAINED_PRECISION_PREPARATION_MISMATCH", 7),
        ],
    );
}


/// Snapshot-06d (I62 C2-5), mutations 173..178: R1-R3 at G5 WORK, and the
/// tightened idle sibling at the exhaustion rule.
#[test]
fn snapshot_06d_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_06D",
        173..178,
        &[
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 4),
        ],
    );
}

/// Snapshot-07/07a review-repair pins (I62 B2; D18 in 07a), mutations 178..236.
#[test]
fn snapshot_07_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_07",
        178..236,
        &[
            ("G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", 6),
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 4),
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 7),
            ("G4 RETAINED_PRECISION_DIAGNOSTIC_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 16),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 11),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 7),
            ("G5b RETAINED_PRECISION_SECTION_MISMATCH", 2),
            ("G8 RETAINED_PRECISION_PREPARATION_MISMATCH", 4),
        ],
    );
}

/// Snapshot-07b/07c confirmation-repair pins (I62; D19-D30, and 07c's D21
/// verification-summary pin), mutations 236..254.
#[test]
fn snapshot_07b_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_07B",
        236..254,
        &[
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 4),
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 2),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 5),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 5),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 1),
            ("G5a RETAINED_PRECISION_SCALE_MISMATCH", 1),
        ],
    );
}

/// Snapshot-07d repair pins (I62; D31, D32, D33 and RV78-N1's two D19 Ready
/// negatives), mutations 254..259.
#[test]
fn snapshot_07d_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_07D",
        254..259,
        &[
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 2),
            ("G8 RETAINED_PRECISION_INVOCATION_MISMATCH", 1),
        ],
    );
}

/// Snapshot-07e pins (I62; RV79-S1's three non-integral G0 values and
/// RV81-N1's D33 rotation row), mutations 259..263.
#[test]
fn snapshot_07e_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_07E",
        259..263,
        &[
            ("G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", 3),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 1),
        ],
    );
}

/// Snapshot-07f pins (I62; D37, RV79's five X1 probes on F', which include
/// RV78's Y1, Y2 and Y4), mutations 263..268.
#[test]
fn snapshot_07f_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_07F",
        263..268,
        &[("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 5)],
    );
}

/// Snapshot-07g pins (I61 U6e; F5, D-U6-7): A2's exact ordinary list, the
/// omission, order, RETAINED_PRECISION_* (U1 M09), invocation-level (U1 M10),
/// another case and 07f's relaxed form, mutations 268..274.
#[test]
fn snapshot_07g_mutation_outcomes() {
    slice_outcomes(
        "I61_OUTCOME_07G",
        268..274,
        &[("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 6)],
    );
}

/// Snapshot-07h pins (I61 U6e repair; RV90 S1, N1, N2): a non-array
/// `affected_refs` names no case, F5 on the second case, and a strict prefix
/// of A2's list, mutations 274..277.
#[test]
fn snapshot_07h_mutation_outcomes() {
    slice_outcomes(
        "I61_OUTCOME_07H",
        274..277,
        &[("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 3)],
    );
}

/// Snapshot-07l pins (U8-2; I69) on the two producer-solved L = 0 bases:
/// SNAPSHOT_05_PLAN §1.2's isolated rotation stop, has_data and coupled estimate
/// (sparse, then dense), then I69's translation-plus-rotation stop (sparse, then
/// dense), mutations 278..286.
#[test]
fn snapshot_07l_mutation_outcomes() {
    slice_outcomes(
        "I70_OUTCOME_07L",
        278..286,
        &[("G5a RETAINED_PRECISION_SCALE_MISMATCH", 8)],
    );
}

/// Mutation 277, RV94 N-3's G7 probe (07k), as its own one-entry slice (B6;
/// I70's item 2): an invalid enum in a not_required case's quality, at G7
/// with the base code, which TS now shares (PLAN decision 11).
#[test]
fn snapshot_07k_mutation_outcomes() {
    // The slice's entry, by id, as Python's and TS's slices check (a reorder
    // of same-code entries leaves the tally unchanged).
    assert_eq!(corpus()["mutations"][277]["id"], "g7_not_required_quality_enum_invalid");
    slice_outcomes(
        "I83_OUTCOME_07K",
        277..278,
        &[("G7 SOURCE_NUMERICAL_CASE_INVALID", 1)],
    );
}

/// Snapshot-07m pins (B6): the N-3 class at its full width (a selected and an
/// unavailable case's quality enum, an empty evidence ref, an extra case
/// member), then the sibling base header classes (quality status, empty
/// limitations, null contract_evidence, a source_block_recovery member),
/// mutations 286..294, each at G7 with this reader's base header code.
#[test]
fn snapshot_07m_mutation_outcomes() {
    let shared = corpus();
    let ids: Vec<&str> = shared["mutations"].as_array().unwrap()[286..294]
        .iter()
        .map(|m| m["id"].as_str().unwrap())
        .collect();
    assert_eq!(
        ids,
        [
            "g7_selected_quality_enum_invalid",
            "g7_unavailable_quality_enum_invalid",
            "g7_quality_case_evidence_ref_empty",
            "g7_quality_case_extra_member",
            "g7_quality_status_invalid",
            "g7_formulation_limitations_empty",
            "g7_contract_evidence_null",
            "g7_source_block_recovery_present",
        ]
    );
    slice_outcomes(
        "I83_OUTCOME_07M",
        286..294,
        &[
            ("G7 SOURCE_NUMERICAL_CASE_INVALID", 4),
            ("G7 SOURCE_NUMERICAL_QUALITY_INVALID", 1),
            ("G7 SOURCE_FORMULATION_BASIS_UNSUPPORTED", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED", 1),
            ("G7 SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN", 1),
        ],
    );
}

/// F5 (D-U6-7; A2) on the real milestone receipts: U1's producer mutants M09
/// (RETAINED_PRECISION_* listed), M10 (diagnostics that do not name the case
/// listed) and M20 (another row method token) are refused by this reader, each
/// resealed as a corpus entry would be.
#[test]
fn f5_kills_u1_m09_m10_m20_on_the_real_milestone_receipts() {
    use serde_json::json;
    for (mode, text) in [
        ("sparse_interactive", include_str!("../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json")),
        ("dense_scrutiny", include_str!("../../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json")),
    ] {
        let doc: Value = serde_json::from_str(text).unwrap();
        let (source, invocation) = (&doc["source"], &doc["invocation"]);
        let case = source["retained_precision"]["body"]["cases"][0]["basis_ref"]["ref_id"].clone();
        let names = |d: &Value| d["affected_refs"].as_array().is_some_and(|a| a.contains(&case));
        let retained = |d: &Value| d["code"].as_str().unwrap().starts_with("RETAINED_PRECISION_");
        let ds = source["diagnostics"].as_array().unwrap();
        let exact: Vec<Value> = ds.iter().filter(|d| names(d) && !retained(d)).map(|d| d["id"].clone()).collect();
        assert_eq!(source["retained_precision"]["body"]["ordinary_attempts"][0]["diagnostic_refs"], json!(exact), "{mode}");
        let m09: Vec<Value> = ds.iter().filter(|d| names(d)).map(|d| d["id"].clone()).collect();
        let m10: Vec<Value> = ds.iter().filter(|d| !retained(d)).map(|d| d["id"].clone()).collect();
        assert!(m09 != exact && m10 != exact, "{mode}: the mutants differ from the exact list");
        let refs = json!(["retained_precision", "body", "ordinary_attempts", 0, "diagnostic_refs"]);
        for (edit_, want) in [
            (set(refs.clone(), json!(m09)), ("G5", ATTEMPT)),
            (set(refs.clone(), json!(m10)), ("G5", ATTEMPT)),
            (set(json!(["results", 0, "recovery_method"]), json!("other")), ("G6", "RETAINED_PRECISION_ROW_METHOD_MISMATCH")),
        ] {
            let mut edited = source.clone();
            edit(&mut edited, &edit_);
            rehash(&mut edited);
            let got = rp::validate(&edited, Some(invocation)).err().map(|e| (e.gate, e.code));
            assert_eq!(got.as_ref().map(|(g, c)| (*g, c.as_str())), Some(want), "{mode} {edit_}");
        }
        let mut resealed = source.clone();
        rehash(&mut resealed);
        assert_eq!(resealed, *source, "{mode}: the receipt reseals to itself");
    }
}

fn observe(shared: &Value, mutation: &Value) -> Value {
    let (source, invocation) = apply_entry(shared, mutation);
    match rp::validate(&source, Some(&invocation)) {
        Err(e) => serde_json::json!({"gate":e.gate,"code":e.code}),
        Ok(_) => serde_json::json!(null),
    }
}




fn attempt_mismatch(got: Result<(), rp::ValidationError>) -> bool {
    matches!(got, Err(e) if e.gate == "G5" && e.code == "RETAINED_PRECISION_ATTEMPT_MISMATCH")
}

/// Rust reader-logic controls mirroring I62's Python-only tests for checklist
/// N5, N6, N8 and N10, which have no native-faithful shared base yet (the
/// Ceiling, an idle/pre-schedule Run, a verification-pass terminal). They run
/// the same schedule replay `validate` uses, on one Run.
#[test]
fn schedule_replay_terminal_branches_reader_logic() {
    use serde_json::json;
    let shared = corpus();
    let schedule = rp::reader_logic::schedule;
    let run_of = |id: &str| {
        shared["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == id)
            .unwrap()["source"]["retained_precision"]["body"]["cases"][0]["run"]
            .clone()
    };
    let selected = run_of("ordinary_prepared_synthetic");
    assert!(schedule(&selected).is_ok());
    let ceiling = json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"ceiling"}});
    // N10: an idle (pre-schedule) Run has no records, no charge and a reasoned
    // non-selected terminal.
    let mut idle = selected.clone();
    idle["records"] = json!([]);
    idle["attempts"] = json!([]);
    idle["case_charge"] = json!(0);
    idle["invocation_increment"] = json!(0);
    idle["kernel_terminal"] =
        json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"budget","scope":"invocation"}});
    assert!(schedule(&idle).is_ok());
    let mut bad = idle.clone();
    bad["kernel_terminal"] = json!({"kind":"selected","reason":null});
    assert!(attempt_mismatch(schedule(&bad)), "idle Run cannot select");
    let mut bad = idle.clone();
    bad["case_charge"] = json!(1);
    assert!(attempt_mismatch(schedule(&bad)), "idle Run carries no charge");
    // 06b/06c ruling: no WorkAccounting terminal is emitted, idle or not.
    let mut bad = idle.clone();
    bad["kernel_terminal"] = json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"work_accounting","fault":"overflow"}});
    assert!(attempt_mismatch(schedule(&bad)), "idle WorkAccounting is not emitted");
    // N6: a rejected p128 candidate must hand its verification to a reused p256.
    let reason = json!({"space":"attempt","tag":"stop_rule","quantity":{"tag":"displacement","dof":{"node":1,"component":"UX"}},"body":0,"kind":"translation"});
    let mut rejected = selected.clone();
    rejected["attempts"][0]["outcome"] = json!({"kind":"rejected","reason":reason});
    rejected["records"][0]["outcome"] = json!({"kind":"rejected","reason":reason});
    rejected["records"][1]["outcome"] = json!({"kind":"solved"});
    rejected["kernel_terminal"] = ceiling.clone();
    assert!(attempt_mismatch(schedule(&rejected)), "rejected must continue");
    // N5: a non-escalating verification-pass failure is terminal.
    let stop = json!({"space":"attempt","tag":"stop","stop":{"space":"stop","tag":"structure"}});
    let mut vfail = selected.clone();
    let verification_failed = json!({"kind":"rejected","reason":{"space":"attempt","tag":"verification_failed"}});
    vfail["attempts"][0]["outcome"] = verification_failed.clone();
    vfail["records"][0]["outcome"] = verification_failed;
    vfail["attempts"][0]["verification"] =
        json!({"record":1,"precision":256,"phase":"failed","reason":stop});
    vfail["records"][1]["outcome"] = json!({"kind":"failed","reason":stop});
    vfail["kernel_terminal"] = json!({"kind":"refused","reason":{"space":"refusal","tag":"structure"}});
    assert!(schedule(&vfail).is_ok());
    let mut bad = vfail.clone();
    bad["kernel_terminal"] = json!({"kind":"selected","reason":null});
    assert!(attempt_mismatch(schedule(&bad)), "verification-pass failure cannot select");
    // N8: the Ceiling, when the reused p512 candidate is rejected and its p1024
    // verification only solved.
    let mut ladder = run_of("p512_ladder_synthetic");
    assert!(schedule(&ladder).is_ok());
    ladder["attempts"][2]["outcome"] = json!({"kind":"rejected","reason":reason});
    ladder["records"][2]["outcome"] = json!({"kind":"rejected","reason":reason});
    ladder["records"][3]["outcome"] = json!({"kind":"solved"});
    ladder["kernel_terminal"] = ceiling;
    assert!(schedule(&ladder).is_ok());
    let mut bad = ladder.clone();
    bad["kernel_terminal"] = json!({"kind":"refused","reason":{"space":"refusal","tag":"structure"}});
    assert!(attempt_mismatch(schedule(&bad)), "exhausted ladder is the Ceiling");
}

/// 06c WorkAccounting rejections fail in the schedule replay itself (the
/// emitted-terminal rule), not only through a later check on the same code:
/// the replay alone, without the statement, rejects each WorkAccounting Run.
#[test]
fn work_accounting_mutations_fail_at_the_terminal_rule() {
    let shared = corpus();
    for id in [
        "work_accounting_after_escalating_stop",
        "idle_work_accounting_run",
        "work_accounting_at_last_slot",
    ] {
        let mutation = shared["mutations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|m| m["id"] == id)
            .unwrap();
        let (source, _) = apply_entry(&shared, mutation);
        let runs: Vec<&Value> = source["retained_precision"]["body"]["cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| &c["run"])
            .filter(|r| r["kernel_terminal"]["reason"]["tag"] == "work_accounting")
            .collect();
        assert_eq!(runs.len(), 1, "{id}");
        assert!(attempt_mismatch(rp::reader_logic::schedule(runs[0])), "{id}");
    }
}

/// Rust reader-logic control mirroring I62's Python-only test for checklist O5
/// (no native-faithful source_decline base yet).
#[test]
fn source_decline_relation_reader_logic() {
    use serde_json::json;
    let shared = corpus();
    let fixture = shared["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == "two_case_preparation_failure_synthetic")
        .unwrap();
    let mut source = fixture["source"].clone();
    let decline = json!({
        "input_owner": {"case_index": 1, "case_id": "case:unavailable-row", "material_basis_ref": 0},
        "constructor_counts": {"nodes": 2, "members": 1, "springs": 0, "constraints": 6, "nodal_terms": 6, "stations": 3, "supports": 1, "id_utf8_bytes": 0, "directional_springs": 0},
        "error": {"tag": "no_nodes"}
    });
    source["retained_precision"]["body"]["cases"][1]["source_decline"] = decline.clone();
    assert!(rp::reader_logic::ordinary(&source).is_ok());
    let mut wrong = decline;
    wrong["input_owner"]["case_index"] = json!(0);
    source["retained_precision"]["body"]["cases"][1]["source_decline"] = wrong;
    assert!(attempt_mismatch(rp::reader_logic::ordinary(&source)));
}

/// Reader-local controls (not shared corpus) for checklist checks added in the
/// I63 audit that no shared mutation decides first: P2 (observables and G5a
/// enter together), P6 (a failed maxima merges its work) and P9 (an
/// unavailable error matches the first failed stage), each on a shared
/// must-pass base; and the G7 bare code with the Rust base code as detail.
#[test]
fn g5_audit_local_controls() {
    use serde_json::json;
    let shared = corpus();
    let entry = |id: &str| {
        shared["must_pass"]
            .as_array()
            .unwrap()
            .iter()
            .find(|m| m["id"] == id)
            .unwrap()
            .clone()
    };
    let attempt = json!(["retained_precision", "body", "product_attempts", 1]);
    let at = |tail: Value| {
        let mut p = attempt.as_array().unwrap().clone();
        p.extend(tail.as_array().unwrap().iter().cloned());
        Value::Array(p)
    };
    for (name, base, extra) in [
        (
            "P9: maxima failure reported as a proof error",
            "maxima_abandoned",
            vec![(
                at(json!(["result"])),
                json!({"kind":"unavailable","error":{"kind":"proof","cause":{"kind":"work_accounting","fault":"overflow"}}}),
            )],
        ),
        (
            "P6: failed maxima without merged completion",
            "maxima_abandoned",
            vec![(at(json!(["proof", "completion"])), json!({"kind":"not_entered"}))],
        ),
        (
            "P2: observables entered without G5a",
            "cert_failed_before_summary",
            vec![
                (at(json!(["stages", "observables"])), json!("failed")),
                (
                    at(json!(["proof", "checks", "observables"])),
                    json!({"kind":"failed","error":{"kind":"observable","cause":{"kind":"accounting","event":"map_write"}}}),
                ),
            ],
        ),
    ] {
        let mut mutation = entry(base);
        for (path, value) in extra {
            mutation["edits"]
                .as_array_mut()
                .unwrap()
                .push(json!({"path":path,"op":"set","value":value}));
        }
        mutation["expected"] = json!(null);
        assert_eq!(
            observe(&shared, &mutation),
            json!({"gate":"G5","code":"RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"}),
            "{name}"
        );
    }
    // G7: the bare base code is the error code.
    let g7 = shared["mutations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["id"] == "g7_maximum_off_enclosure")
        .unwrap();
    let case = shared["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == g7["base"])
        .unwrap();
    let mut source = case["source"].clone();
    for e in g7["edits"].as_array().unwrap() {
        edit(&mut source, e);
    }
    rehash(&mut source);
    let got = rp::validate(&source, Some(&case["invocation"])).unwrap_err();
    // G7 settlement (06b): this reader's own bare base code, detail separate.
    assert_eq!(
        (got.gate, got.code.as_str()),
        ("G7", "SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS")
    );
}

/// Snapshot-05a shared must-pass entries: each rehashed rewrite keeps every
/// public relation, so the reader admits it with the base case's
/// classifications (or, 07n, the entry's own `expected_classifications` when it
/// states them) and the eligibility the entry states (07i, U7).
#[test]
fn shared_must_pass_entries_validate() {
    let shared = corpus();
    let entries = shared["must_pass"].as_array().unwrap();
    // Snapshot 07: 06d's 18 plus the equal-E bracket control; 07h adds F5's
    // reordered-envelope exact-list control (RV90 N2); 07j adds C04's not_required case;
    // 07l (U8-2) appends the 4 L = 0 entries on the producer-solved bases, 24 to 28;
    // 07n (B1 SC) appends 50, 28 to 78; 07o (B2-C REVISION_01 §5.1) appends 19, to 97.
    assert_eq!(entries.len(), 97);
    let mut failures = Vec::new();
    for entry in entries {
        assert_eq!(entry["rehash"], "all");
        let case = shared["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == entry["base"])
            .unwrap();
        let (source, invocation) = apply_entry(&shared, entry);
        // PR-B2 ruling 5: an entry on a base refused bound states that refusal as its
        // `expected` (its unbound and transport reads: `snapshot_07o_unbound_and_transport_reads`).
        if entry["expected"] != "pass" {
            assert_eq!(entry["expected"], case["expected"], "{}", entry["id"]);
            let got = observe_validation(rp::validate(&source, Some(&invocation)));
            println!("I63_MUST_PASS {} stated {got}", entry["id"]);
            if got != entry["expected"] {
                failures.push(format!("{} got {got}", entry["id"]));
            }
            continue;
        }
        let got = match rp::validate(&source, Some(&invocation)) {
            Ok(got) => got,
            Err(e) => {
                failures.push(format!("{} rejected {e:?}", entry["id"]));
                continue;
            }
        };
        // 07n (B1 SC): an admitted rewrite whose classes differ from its base's
        // (a case no longer selected, a row added or removed) states its own.
        let expected = entry
            .get("expected_classifications")
            .unwrap_or(&case["expected_classifications"])
            .as_array()
            .unwrap();
        // U7 (07i): each must-pass entry states its eligibility.
        let same = got.numerical_eligible
            == entry["expected_eligibility"]["numerical_eligible"].as_bool().unwrap()
            && got.invocation_bound
            && got.classifications.len() == expected.len()
            && got.classifications.iter().zip(expected).all(|(g, w)| {
                let class = match g.class {
                    rp::AccuracyClass::RelativeVerified => "relative_verified",
                    rp::AccuracyClass::AbsoluteVerified { bound_bits } => {
                        if format!("{bound_bits:016x}") != w["bound_bits"] {
                            return false;
                        }
                        "absolute_verified"
                    }
                    rp::AccuracyClass::InputDerived => "input_derived",
                    rp::AccuracyClass::NonQuantity => "non_quantity",
                    rp::AccuracyClass::NotCovered => "not_covered",
                };
                g.result_id == w["result_id"]
                    && g.basis_ref == w["basis_ref"]
                    && format!("{:016x}", g.normalized_bits) == w["normalized_bits"]
                    && g.scale_bits.map(|b| format!("{b:016x}"))
                        == w["scale_bits"].as_str().map(str::to_owned)
                    && class == w["class"]
            });
        println!("I63_MUST_PASS {} {}", entry["id"], same);
        if !same {
            failures.push(format!("{} classifications differ", entry["id"]));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// C1 G5b "same E/ê/Φ at p512" (verify.rs `e_hat` through `phi_512`). No p512 corpus base
/// exists until C1b, so the rounding is pinned directly. Expected bits were
/// derived independently as the least binary64 >= e·2^-438 with an exact
/// rational oracle (hand-checked for the subnormal ties).
#[test]
fn p512_floor_phi_follows_native_rounding() {
    for (input, expected) in [
        (0x0000000000000000u64, 0x0000000000000000u64),
        (0x3ff0000000000000, 0x2490000000000000), // 1 -> 2^-438 exactly
        (0x1a70000000000001, 0x0000001000000001), // 2^-600(1+2^-52): nearest is below, next up
        (0x7fefffffffffffff, 0x648fffffffffffff), // MAX scales exactly
        (0x0000000000000001, 0x0000000000000001), // underflows to 0, next up is 2^-1074
        (0x1838000000000000, 0x0000000000000002), // 1.5 ulp ties to even 2: not below
        (0x1844000000000000, 0x0000000000000003), // 2.5 ulp ties to even 2: below, next up
    ] {
        assert_eq!(
            rp::phi_512(f64::from_bits(input)).to_bits(),
            expected,
            "{input:016x}"
        );
    }
    assert_eq!(rp::e_hat([3.0, 0.0], 0.0), [3.0, 0.0]);
    assert_eq!(rp::e_hat([3.0, 0.0], 2.0), [3.0, 6.0]);
    assert_eq!(rp::e_hat([0.0, 8.0], 2.0), [4.0, 8.0]);
}

/// Reader-local synthetic controls (not shared corpus entries), mirroring the
/// three Python-only I62 checkpoint-B layout controls: G5a rederives the
/// canonical layout from the bound source maps, so a purported input-derived
/// force row, an unmarked constrained displacement or a nonzero prescription
/// (outside this C3 D=false scope) fails at G5a before the later G8 binding.
/// The fourth control is Rust-specific: the full canonical rebuild also rejects
/// a non-input kind relabel at G5a (the Python draft reaches G8 for it).
#[test]
fn coverage_layout_controls_fail_at_g5a() {
    use serde_json::json;
    let shared = corpus();
    let c = &shared["cases"][0];
    for (name, tail, value) in [
        (
            "reaction force row flagged input-derived",
            json!(["layout", 44, "input_derived"]),
            json!(true),
        ),
        (
            "constrained displacement not flagged input-derived",
            json!(["layout", 0, "input_derived"]),
            json!(false),
        ),
        (
            "nonzero prescription",
            json!(["constraints", 0, "value"]),
            json!("3ff0000000000000"),
        ),
        (
            "rust-specific: end-action force row relabelled translation",
            json!(["layout", 14, "kind"]),
            json!("translation"),
        ),
    ] {
        let mut path = json!(["retained_precision", "body", "sources", 0]);
        path.as_array_mut()
            .unwrap()
            .extend(tail.as_array().unwrap().iter().cloned());
        let mut source = c["source"].clone();
        edit(&mut source, &json!({"path":path,"op":"set","value":value}));
        rehash(&mut source);
        let got = rp::validate(&source, Some(&c["invocation"])).unwrap_err();
        assert_eq!(
            (got.gate, got.code.as_str()),
            ("G5a", "RETAINED_PRECISION_SCALE_MISMATCH"),
            "{name}"
        );
    }
}

/// I57 s4/s5 over-rejection guard, mirroring I62's Python-only controls: these
/// rewrites keep every public relation (feasibility, rederived estimate and
/// charge, exact rosters, record binding, direct data facts), so the reader must
/// admit them with the base classifications; only producer custody or replay
/// can catch such attested private flags. Since U7 they keep the base's eligibility.
#[test]
fn publicly_consistent_coverage_attestations_are_not_rejected() {
    use serde_json::json;
    let shared = corpus();
    let zero = "0000000000000000";
    let coverage = json!(["retained_precision", "body", "product_attempts", 0, "proof", "summary_coverage", 0]);
    let selection = json!(["retained_precision", "body", "cases", 0, "selection"]);
    let verification = json!(["retained_precision", "body", "cases", 0, "run", "records", 1, "verification"]);
    let at = |base: &Value, tail: Value| {
        let mut p = base.as_array().unwrap().clone();
        p.extend(tail.as_array().unwrap().iter().cloned());
        Value::Array(p)
    };
    let four = json!(["translation", "rotation", "force", "moment"])
        .as_array()
        .unwrap()
        .iter()
        .map(|k| json!({"body":0,"kind":k,"value":zero}))
        .collect::<Vec<_>>();
    let variants = [
        (
            "loaded_stop_without_force_moment",
            "ordinary_prepared_synthetic",
            vec![
                (at(&coverage, json!(["stop"])), json!([true, true, false, false])),
                (at(&selection, json!(["stop_rule"])), json!(four[..2])),
            ],
        ),
        (
            "loaded_all_stop_false",
            "ordinary_prepared_synthetic",
            vec![
                (at(&coverage, json!(["stop"])), json!([false, false, false, false])),
                (at(&selection, json!(["stop_rule"])), json!([])),
            ],
        ),
        (
            "no_data_all_stop_true",
            "ordinary_prepared_no_data_synthetic",
            vec![
                (at(&coverage, json!(["stop"])), json!([true, true, true, true])),
                (at(&selection, json!(["stop_rule"])), json!(four)),
            ],
        ),
        (
            "no_data_attested_data_block",
            "ordinary_prepared_no_data_synthetic",
            vec![
                (at(&coverage, json!(["has_data"])), json!(true)),
                (
                    at(&selection, json!(["certified_bound"])),
                    json!([{"body":0,"value":"3ff0000000000000"}]),
                ),
                (
                    at(&verification, json!(["bound"])),
                    json!([{"body":0,"value":"3ff0000000000000"}]),
                ),
                (at(&verification, json!(["data_blocks"])), json!(1)),
            ],
        ),
    ];
    for (name, base, edits) in variants {
        let case = shared["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == base)
            .unwrap();
        let mut source = case["source"].clone();
        for (path, value) in edits {
            edit(&mut source, &json!({"path":path,"op":"set","value":value}));
        }
        rehash(&mut source);
        let got = rp::validate(&source, Some(&case["invocation"]))
            .unwrap_or_else(|e| panic!("{name}: {e:?}"));
        assert!(case["expected"]["numerical_eligible"].as_bool().unwrap(), "{name}: an eligible base");
        assert!(got.numerical_eligible, "{name}");
        let expected = case["expected_classifications"].as_array().unwrap();
        assert_eq!(got.classifications.len(), expected.len(), "{name}");
        for (got, want) in got.classifications.iter().zip(expected) {
            assert_eq!(got.result_id, want["result_id"], "{name}");
            assert_eq!(
                format!("{:016x}", got.normalized_bits),
                want["normalized_bits"],
                "{name}"
            );
            assert_eq!(
                got.scale_bits.map(|b| format!("{b:016x}")),
                want["scale_bits"].as_str().map(str::to_owned),
                "{name}"
            );
        }
    }
}

#[test]
fn canonical_zero_and_invalid_helper_operands() {
    for zero in [0.0, -0.0] {
        for value in [0.0, -0.0, 1.0, -1.0, f64::MAX, -f64::MAX] {
            assert_eq!(rp::absolute_bound(value, zero).unwrap().to_bits(), 0);
        }
        assert_eq!(rp::upward_product(zero, 1.0).unwrap().to_bits(), 0);
        assert_eq!(rp::upward_product(1.0, zero).unwrap().to_bits(), 0);
        assert_eq!(rp::upward_small_sum(zero, zero).unwrap().to_bits(), 1);
    }
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
        assert!(rp::absolute_bound(1.0, bad).is_err());
        assert!(rp::upward_small_sum(bad, 0.0).is_err());
        assert!(rp::upward_small_sum(0.0, bad).is_err());
        assert!(rp::upward_product(1.0, bad).is_err());
    }
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(rp::absolute_bound(bad, 1.0).is_err());
    }
    assert!(rp::upward_small_sum(f64::MAX, 0.0).is_err());
    assert!(rp::upward_small_sum(f64::MAX, f64::MAX).is_err());
}

// Expected values independently computed with Fraction and least-upper binary search.
// Every expected result and its immediate predecessor were checked in the rational oracle.
#[test]
fn independent_rational_product_and_single_round_sum_controls() {
    let cases: &[(u64, u64, Option<u64>, Option<u64>)] = &[
        (
            0x0000000000000000,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x0000000000000001),
        ),
        (
            0x0000000000000000,
            0x0000000000000001,
            Some(0x0000000000000000),
            Some(0x0000000000000002),
        ),
        (
            0x0000000000000000,
            0x0000000000000002,
            Some(0x0000000000000000),
            Some(0x0000000000000003),
        ),
        (
            0x0000000000000000,
            0x000fffffffffffff,
            Some(0x0000000000000000),
            Some(0x0010000000000000),
        ),
        (
            0x0000000000000000,
            0x0010000000000000,
            Some(0x0000000000000000),
            Some(0x0010000000000001),
        ),
        (
            0x0000000000000000,
            0x3ff0000000000000,
            Some(0x0000000000000000),
            Some(0x3ff0000000000001),
        ),
        (
            0x0000000000000000,
            0x3ff0000000000001,
            Some(0x0000000000000000),
            Some(0x3ff0000000000002),
        ),
        (
            0x0000000000000000,
            0x7fefffffffffffff,
            Some(0x0000000000000000),
            None,
        ),
        (
            0x0000000000000001,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x0000000000000002),
        ),
        (
            0x0000000000000001,
            0x0000000000000001,
            Some(0x0000000000000001),
            Some(0x0000000000000003),
        ),
        (
            0x0000000000000001,
            0x0000000000000002,
            Some(0x0000000000000001),
            Some(0x0000000000000004),
        ),
        (
            0x0000000000000001,
            0x000fffffffffffff,
            Some(0x0000000000000001),
            Some(0x0010000000000001),
        ),
        (
            0x0000000000000001,
            0x0010000000000000,
            Some(0x0000000000000001),
            Some(0x0010000000000002),
        ),
        (
            0x0000000000000001,
            0x3ff0000000000000,
            Some(0x0000000000000001),
            Some(0x3ff0000000000001),
        ),
        (
            0x0000000000000001,
            0x3ff0000000000001,
            Some(0x0000000000000002),
            Some(0x3ff0000000000002),
        ),
        (
            0x0000000000000001,
            0x7fefffffffffffff,
            Some(0x3ccfffffffffffff),
            None,
        ),
        (
            0x0000000000000002,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x0000000000000003),
        ),
        (
            0x0000000000000002,
            0x0000000000000001,
            Some(0x0000000000000001),
            Some(0x0000000000000004),
        ),
        (
            0x0000000000000002,
            0x0000000000000002,
            Some(0x0000000000000001),
            Some(0x0000000000000005),
        ),
        (
            0x0000000000000002,
            0x000fffffffffffff,
            Some(0x0000000000000001),
            Some(0x0010000000000002),
        ),
        (
            0x0000000000000002,
            0x0010000000000000,
            Some(0x0000000000000001),
            Some(0x0010000000000003),
        ),
        (
            0x0000000000000002,
            0x3ff0000000000000,
            Some(0x0000000000000002),
            Some(0x3ff0000000000001),
        ),
        (
            0x0000000000000002,
            0x3ff0000000000001,
            Some(0x0000000000000003),
            Some(0x3ff0000000000002),
        ),
        (
            0x0000000000000002,
            0x7fefffffffffffff,
            Some(0x3cdfffffffffffff),
            None,
        ),
        (
            0x000fffffffffffff,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x0010000000000000),
        ),
        (
            0x000fffffffffffff,
            0x0000000000000001,
            Some(0x0000000000000001),
            Some(0x0010000000000001),
        ),
        (
            0x000fffffffffffff,
            0x0000000000000002,
            Some(0x0000000000000001),
            Some(0x0010000000000002),
        ),
        (
            0x000fffffffffffff,
            0x000fffffffffffff,
            Some(0x0000000000000001),
            Some(0x001fffffffffffff),
        ),
        (
            0x000fffffffffffff,
            0x0010000000000000,
            Some(0x0000000000000001),
            Some(0x0020000000000000),
        ),
        (
            0x000fffffffffffff,
            0x3ff0000000000000,
            Some(0x000fffffffffffff),
            Some(0x3ff0000000000001),
        ),
        (
            0x000fffffffffffff,
            0x3ff0000000000001,
            Some(0x0010000000000000),
            Some(0x3ff0000000000002),
        ),
        (
            0x000fffffffffffff,
            0x7fefffffffffffff,
            Some(0x400ffffffffffffe),
            None,
        ),
        (
            0x0010000000000000,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x0010000000000001),
        ),
        (
            0x0010000000000000,
            0x0000000000000001,
            Some(0x0000000000000001),
            Some(0x0010000000000002),
        ),
        (
            0x0010000000000000,
            0x0000000000000002,
            Some(0x0000000000000001),
            Some(0x0010000000000003),
        ),
        (
            0x0010000000000000,
            0x000fffffffffffff,
            Some(0x0000000000000001),
            Some(0x0020000000000000),
        ),
        (
            0x0010000000000000,
            0x0010000000000000,
            Some(0x0000000000000001),
            Some(0x0020000000000001),
        ),
        (
            0x0010000000000000,
            0x3ff0000000000000,
            Some(0x0010000000000000),
            Some(0x3ff0000000000001),
        ),
        (
            0x0010000000000000,
            0x3ff0000000000001,
            Some(0x0010000000000001),
            Some(0x3ff0000000000002),
        ),
        (
            0x0010000000000000,
            0x7fefffffffffffff,
            Some(0x400fffffffffffff),
            None,
        ),
        (
            0x3ff0000000000000,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x3ff0000000000001),
        ),
        (
            0x3ff0000000000000,
            0x0000000000000001,
            Some(0x0000000000000001),
            Some(0x3ff0000000000001),
        ),
        (
            0x3ff0000000000000,
            0x0000000000000002,
            Some(0x0000000000000002),
            Some(0x3ff0000000000001),
        ),
        (
            0x3ff0000000000000,
            0x000fffffffffffff,
            Some(0x000fffffffffffff),
            Some(0x3ff0000000000001),
        ),
        (
            0x3ff0000000000000,
            0x0010000000000000,
            Some(0x0010000000000000),
            Some(0x3ff0000000000001),
        ),
        (
            0x3ff0000000000000,
            0x3ff0000000000000,
            Some(0x3ff0000000000000),
            Some(0x4000000000000001),
        ),
        (
            0x3ff0000000000000,
            0x3ff0000000000001,
            Some(0x3ff0000000000001),
            Some(0x4000000000000001),
        ),
        (
            0x3ff0000000000000,
            0x7fefffffffffffff,
            Some(0x7fefffffffffffff),
            None,
        ),
        (
            0x3ff0000000000001,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x3ff0000000000002),
        ),
        (
            0x3ff0000000000001,
            0x0000000000000001,
            Some(0x0000000000000002),
            Some(0x3ff0000000000002),
        ),
        (
            0x3ff0000000000001,
            0x0000000000000002,
            Some(0x0000000000000003),
            Some(0x3ff0000000000002),
        ),
        (
            0x3ff0000000000001,
            0x000fffffffffffff,
            Some(0x0010000000000000),
            Some(0x3ff0000000000002),
        ),
        (
            0x3ff0000000000001,
            0x0010000000000000,
            Some(0x0010000000000001),
            Some(0x3ff0000000000002),
        ),
        (
            0x3ff0000000000001,
            0x3ff0000000000000,
            Some(0x3ff0000000000001),
            Some(0x4000000000000001),
        ),
        (
            0x3ff0000000000001,
            0x3ff0000000000001,
            Some(0x3ff0000000000003),
            Some(0x4000000000000002),
        ),
        (0x3ff0000000000001, 0x7fefffffffffffff, None, None),
        (
            0x7fefffffffffffff,
            0x0000000000000000,
            Some(0x0000000000000000),
            None,
        ),
        (
            0x7fefffffffffffff,
            0x0000000000000001,
            Some(0x3ccfffffffffffff),
            None,
        ),
        (
            0x7fefffffffffffff,
            0x0000000000000002,
            Some(0x3cdfffffffffffff),
            None,
        ),
        (
            0x7fefffffffffffff,
            0x000fffffffffffff,
            Some(0x400ffffffffffffe),
            None,
        ),
        (
            0x7fefffffffffffff,
            0x0010000000000000,
            Some(0x400fffffffffffff),
            None,
        ),
        (
            0x7fefffffffffffff,
            0x3ff0000000000000,
            Some(0x7fefffffffffffff),
            None,
        ),
        (0x7fefffffffffffff, 0x3ff0000000000001, None, None),
        (0x7fefffffffffffff, 0x7fefffffffffffff, None, None),
        (
            0x1e14c1df32b7eb67,
            0x58f5a9f6bd147bdf,
            Some(0x371c1af4d22d76fc),
            Some(0x58f5a9f6bd147be0),
        ),
        (
            0x45a1bd06d6f37315,
            0x69dc42e529de3bce,
            Some(0x6f8f54f5c43d5caf),
            Some(0x69dc42e529de3bcf),
        ),
        (
            0x32ffc4f8841a1663,
            0x00758dfe049896e7,
            Some(0x0000000000000001),
            Some(0x32ffc4f8841a1664),
        ),
        (
            0x0f44b3af157d881f,
            0x69e910987501504d,
            Some(0x3940371d418648e8),
            Some(0x69e910987501504e),
        ),
        (
            0x59dafd6f66bb2720,
            0x7bdaf27a12034453,
            None,
            Some(0x7bdaf27a12034454),
        ),
        (
            0x3c085d7ec865e9bb,
            0x5d502e64cc7f5fdd,
            Some(0x5968a42514a87bce),
            Some(0x5d502e64cc7f5fde),
        ),
        (
            0x7379354794a89760,
            0x0890bd8e67b2714e,
            Some(0x3c1a5fed4e700641),
            Some(0x7379354794a89761),
        ),
        (
            0x790e76d10ab72b48,
            0x1a967111d5ad54f5,
            Some(0x53b55d544416b4ef),
            Some(0x790e76d10ab72b49),
        ),
        (
            0x47af280bbabd5668,
            0x1df0c52f6a03ee99,
            Some(0x25b054028f9dfe01),
            Some(0x47af280bbabd5669),
        ),
        (
            0x3555abfd56407372,
            0x0b1c4f0d8c08c319,
            Some(0x00832c07719aa2a5),
            Some(0x3555abfd56407373),
        ),
        (
            0x50b0a66a03cac061,
            0x5912f88f59c516a1,
            Some(0x69d3bddfdc3d2732),
            Some(0x5912f88f59c516a2),
        ),
        (
            0x54f8810202bf0b6b,
            0x40f703cfeaebab2f,
            Some(0x56019fa4c0926913),
            Some(0x54f8810202bf0b6c),
        ),
        (
            0x007756f928519d84,
            0x72cc29c001c3ee89,
            Some(0x33548a8d7da3a412),
            Some(0x72cc29c001c3ee8a),
        ),
        (
            0x0e72468450ddfafb,
            0x7a802b184971bb78,
            Some(0x490277bd92216cb4),
            Some(0x7a802b184971bb79),
        ),
        (
            0x130f0192f1925d73,
            0x2fee14a73fa050a2,
            Some(0x030d257ccc2d7f16),
            Some(0x2fee14a73fa050a3),
        ),
        (
            0x1cd97741ba5d4455,
            0x6ffd93d326ff751e,
            Some(0x4ce789b77036f0cb),
            Some(0x6ffd93d326ff751f),
        ),
        (
            0x2ae0cdbd939da8a2,
            0x14a94c27b30e8d35,
            Some(0x00001a91732cae9e),
            Some(0x2ae0cdbd939da8a3),
        ),
        (
            0x0ea67c3523d9183d,
            0x761964445d6b5c9a,
            Some(0x44d1d77db3a6f85c),
            Some(0x761964445d6b5c9b),
        ),
        (
            0x5c5d0bd8dad67520,
            0x481cee444497234f,
            Some(0x648a42a3d3c66f50),
            Some(0x5c5d0bd8dad67521),
        ),
        (
            0x7c00a178e98d4d09,
            0x59d31f8813d0a5a2,
            None,
            Some(0x7c00a178e98d4d0a),
        ),
        (
            0x263bbd3068423790,
            0x6bb0413a9a5064ad,
            Some(0x51fc2e46964190ea),
            Some(0x6bb0413a9a5064ae),
        ),
        (
            0x19b7b224d143a01a,
            0x0165f75fe299c99c,
            Some(0x0000000000000001),
            Some(0x19b7b224d143a01b),
        ),
        (
            0x58c28aaeb07713b7,
            0x34c5ff5d30c5acca,
            Some(0x4d997df386553534),
            Some(0x58c28aaeb07713b8),
        ),
        (
            0x2d3b3176f09ec037,
            0x1becffd5ce90505f,
            Some(0x0938a4afef211b6e),
            Some(0x2d3b3176f09ec038),
        ),
        (
            0x6549797e3d6a6a81,
            0x7b0537e3bbd836f7,
            None,
            Some(0x7b0537e3bbd836f8),
        ),
        (
            0x3b8bbfe507c4f700,
            0x76869f95022c94a1,
            Some(0x72239e5026bb17b7),
            Some(0x76869f95022c94a2),
        ),
        (
            0x36b01599fe419fd4,
            0x120289ac726e2bbc,
            Some(0x08c2a2b38fdafca6),
            Some(0x36b01599fe419fd5),
        ),
        (
            0x0800247f1281a7fa,
            0x44ed643569f39d0b,
            Some(0x0cfda7404f88e2c2),
            Some(0x44ed643569f39d0c),
        ),
        (
            0x7804a4d869e9f8dd,
            0x3b3934efaed29390,
            Some(0x735042ef92636f6b),
            Some(0x7804a4d869e9f8de),
        ),
        (
            0x5463e746c0715d9b,
            0x2dd5839525b71047,
            Some(0x424ac33bf889d840),
            Some(0x5463e746c0715d9c),
        ),
        (
            0x491b06f77cf06eca,
            0x0f8df37aec7fcf2e,
            Some(0x18b94bf502e58661),
            Some(0x491b06f77cf06ecb),
        ),
        (
            0x7169074ddfe7a1bb,
            0x136ce4aa9178cf47,
            Some(0x44e6993d975b4bc4),
            Some(0x7169074ddfe7a1bc),
        ),
        (
            0x675b4f86f0c4ded9,
            0x3d809336a53a79d3,
            Some(0x64ec4acedef84bff),
            Some(0x675b4f86f0c4deda),
        ),
        (
            0x7f373395edbad3ca,
            0x60934def1d9f089f,
            None,
            Some(0x7f373395edbad3cb),
        ),
        (
            0x0a4b341bdadc9113,
            0x4a070eed184bbb09,
            Some(0x14639a2460140bbd),
            Some(0x4a070eed184bbb0a),
        ),
        (
            0x1ba58a2050d87045,
            0x132176b02c8f040b,
            Some(0x0000000000000001),
            Some(0x1ba58a2050d87046),
        ),
        (
            0x044696ceeca9e87c,
            0x23a82c9f981c63e9,
            Some(0x0000000000000001),
            Some(0x23a82c9f981c63ea),
        ),
        (
            0x24e2f0b4be902c75,
            0x6cc7800493d19287,
            Some(0x51bbd18ee30c8334),
            Some(0x6cc7800493d19288),
        ),
        (
            0x31be23921ba29e40,
            0x4b473dd6511f23eb,
            Some(0x3d15e3ceacaa06ab),
            Some(0x4b473dd6511f23ec),
        ),
        (
            0x4ff3f108b47aac5d,
            0x6aa0d78ae01a34ee,
            Some(0x7aa4fdacafae14f1),
            Some(0x6aa0d78ae01a34ef),
        ),
        (
            0x1c6c501b45b372b2,
            0x01a48b91afde62af,
            Some(0x0000000000000001),
            Some(0x1c6c501b45b372b3),
        ),
        (
            0x22a05475fb83ade1,
            0x595ac6798ebcbe19,
            Some(0x3c0b53d0fd28efb1),
            Some(0x595ac6798ebcbe1a),
        ),
        (
            0x28439aa40bcb1e76,
            0x1307fe164a6e6ff1,
            Some(0x0000000000000001),
            Some(0x28439aa40bcb1e77),
        ),
        (
            0x34f06c9efed7f618,
            0x6b73f608142a485f,
            Some(0x60747d8b26315a3b),
            Some(0x6b73f608142a4860),
        ),
        (
            0x489f43e2559e4d15,
            0x520c1729a6c579ee,
            Some(0x5abb720787832214),
            Some(0x520c1729a6c579ef),
        ),
        (
            0x6c5d233d42db4c15,
            0x313d400eb7f55ba7,
            Some(0x5daaa24366083063),
            Some(0x6c5d233d42db4c16),
        ),
        (
            0x7a4b5d6de1e2e0fa,
            0x48b69fcdce093260,
            None,
            Some(0x7a4b5d6de1e2e0fb),
        ),
        (
            0x072bdf0fd674a571,
            0x4eee53d76e643698,
            Some(0x162a6a250cfa1197),
            Some(0x4eee53d76e643699),
        ),
        (
            0x415c1c6ddd1638c3,
            0x6f93e0107648a5e7,
            Some(0x710175b6b27062a1),
            Some(0x6f93e0107648a5e8),
        ),
        (
            0x4d674000a80223da,
            0x76d35940f3e72527,
            None,
            Some(0x76d35940f3e72528),
        ),
        (
            0x0815afd3c1eb0758,
            0x27c52fcb3aadec7f,
            Some(0x0000000000000001),
            Some(0x27c52fcb3aadec80),
        ),
        (
            0x08342bb3953f38c0,
            0x54e5da9bfdbca2ec,
            Some(0x1d2b8cf3c9f10440),
            Some(0x54e5da9bfdbca2ed),
        ),
        (
            0x14776c2e4838c43a,
            0x4c6897d24047e28c,
            Some(0x20f20043158c1368),
            Some(0x4c6897d24047e28d),
        ),
        (
            0x19f43e99520159ff,
            0x0c50b78289be9a30,
            Some(0x0000000000000001),
            Some(0x19f43e9952015a00),
        ),
        (
            0x20b3b14903307b17,
            0x17e03a1ca5cb7113,
            Some(0x0000000000000001),
            Some(0x20b3b14903307b18),
        ),
        (
            0x08ef2e045830e0e9,
            0x41a7871907eeed9a,
            Some(0x0aa6ecb5a5657379),
            Some(0x41a7871907eeed9b),
        ),
        (
            0x43c2720f29b1d4ad,
            0x2f13781dcab28a9e,
            Some(0x32e671ebcba53200),
            Some(0x43c2720f29b1d4ae),
        ),
        (
            0x1f14d4c069b24d7d,
            0x042a240fea98575a,
            Some(0x0000000000000001),
            Some(0x1f14d4c069b24d7e),
        ),
        (
            0x26bdd27ef6f335f1,
            0x494242d6993ea82b,
            Some(0x301104b18f480b0e),
            Some(0x494242d6993ea82c),
        ),
        (
            0x1679aa7860758b66,
            0x7e78facff51ee5f0,
            Some(0x55040904e01e60b6),
            Some(0x7e78facff51ee5f1),
        ),
        (
            0x2f27f35569148f18,
            0x7626f920f17d9aad,
            Some(0x656131c0d0ebd0f9),
            Some(0x7626f920f17d9aae),
        ),
        (
            0x02b8fef331f932e9,
            0x161744a438bbb1b5,
            Some(0x0000000000000001),
            Some(0x161744a438bbb1b6),
        ),
        (
            0x3f264fe9b297eb7a,
            0x13b03343d03c421c,
            Some(0x12e69766fc86b184),
            Some(0x3f264fe9b297eb7b),
        ),
        (
            0x7d42a9e9a20deee9,
            0x7f80203059681946,
            None,
            Some(0x7f802030596943e5),
        ),
        (
            0x3cff89f8637b6b63,
            0x3f7019b9dae50d21,
            Some(0x3c7fbcae525a335e),
            Some(0x3f7019b9dae52cab),
        ),
        (
            0x798d4764731a53aa,
            0x70d8ab12a99eb0cb,
            None,
            Some(0x798d4764731a53ab),
        ),
        (
            0x254b680bc66a8821,
            0x32272dbc557fe470,
            Some(0x1783d9f413aaad35),
            Some(0x32272dbc557fe471),
        ),
        (
            0x458f4e911a472b06,
            0x26b8a2c4a611ca00,
            Some(0x2c581a2af5fc2198),
            Some(0x458f4e911a472b07),
        ),
        (
            0x2091ddf2e1b3504d,
            0x3492cc77dc34ea7d,
            Some(0x1534fe04ef4cba0b),
            Some(0x3492cc77dc34ea7e),
        ),
        (
            0x2a99110bb4e7347d,
            0x063d43f35af9d728,
            Some(0x0000000000000001),
            Some(0x2a99110bb4e7347e),
        ),
        (
            0x4195dd912bb6f27d,
            0x559cef52c6638fb3,
            Some(0x5743c56669d18c7e),
            Some(0x559cef52c6638fb4),
        ),
        (
            0x62d788b7c1c6f2c1,
            0x3fac2238d89b3ab6,
            Some(0x6294b0cbdb5a8cab),
            Some(0x62d788b7c1c6f2c2),
        ),
        (
            0x530c07ab017c2eed,
            0x2edd8bc158e575aa,
            Some(0x41f9e15da444731d),
            Some(0x530c07ab017c2eee),
        ),
        (
            0x2502a8d101bf661b,
            0x6f2b2cd270774238,
            Some(0x543fb1265c97cd6f),
            Some(0x6f2b2cd270774239),
        ),
        (
            0x35d44331b54ed27d,
            0x23393fe1bca3aeda,
            Some(0x191ff9e41ed4e6f1),
            Some(0x35d44331b54ed27e),
        ),
        (
            0x69be8e05ce0e47fb,
            0x06e0cb2a4a82de6a,
            Some(0x30b009003d674fcc),
            Some(0x69be8e05ce0e47fc),
        ),
        (
            0x33c906276ad8821f,
            0x7e74cee6da78d8e9,
            Some(0x725045a4c79ae049),
            Some(0x7e74cee6da78d8ea),
        ),
        (
            0x3f04ac5c61624c5a,
            0x28427ee165a76fed,
            Some(0x2757e5d88038f43e),
            Some(0x3f04ac5c61624c5b),
        ),
        (
            0x2f29e8cd37c88e0d,
            0x5b9c064005dc6080,
            Some(0x4ad6b0c30da3302d),
            Some(0x5b9c064005dc6081),
        ),
        (
            0x2ca3f454236f95df,
            0x29fdfe6a8fc801d6,
            Some(0x16b2b4120ef5ab1b),
            Some(0x2ca3f454236f999f),
        ),
        (
            0x1a5a58292a12e8b4,
            0x049cdf322628936d,
            Some(0x0000000000000001),
            Some(0x1a5a58292a12e8b5),
        ),
        (
            0x022f314ff57d5c37,
            0x7de9e797fbca04f6,
            Some(0x40294046971f16b1),
            Some(0x7de9e797fbca04f7),
        ),
        (
            0x4bcbf7a7483b2a60,
            0x49ce869a0f2a1f3f,
            Some(0x55aaadd0854be1dd),
            Some(0x4bcbf7a74859b0fb),
        ),
        (
            0x37be91e7024a654f,
            0x1fa2e2231fc59290,
            Some(0x17720a19eeb8c6ed),
            Some(0x37be91e7024a6550),
        ),
        (
            0x3ff9aad67a26435c,
            0x164e08ae165ad7ce,
            Some(0x1658171f6afef1f0),
            Some(0x3ff9aad67a26435d),
        ),
        (
            0x1a9ca6b3b2722423,
            0x4559d50f457b1698,
            Some(0x200720ffabded818),
            Some(0x4559d50f457b1699),
        ),
        (
            0x71fe9de609f1343a,
            0x618ced671552302d,
            None,
            Some(0x71fe9de609f1343b),
        ),
        (
            0x0bea4d22585cc8b1,
            0x736061ccf40390f8,
            Some(0x3f5aede6e1288a59),
            Some(0x736061ccf40390f9),
        ),
        (
            0x712b888fd8ecb9c3,
            0x0954ae5d09cb0d4a,
            Some(0x3a91cb60830ea7f1),
            Some(0x712b888fd8ecb9c4),
        ),
        (
            0x018ac9dcff27c830,
            0x5521dc1d8f6be511,
            Some(0x16bde703ebd002d0),
            Some(0x5521dc1d8f6be512),
        ),
        (
            0x4e0655348b22bed2,
            0x217433eac7a3de61,
            Some(0x2f8c32f8fa2fd813),
            Some(0x4e0655348b22bed3),
        ),
        (
            0x730004d9a1f9a596,
            0x4ee90d54da62dab9,
            None,
            Some(0x730004d9a1f9a597),
        ),
        (
            0x13b641e174d4bdaa,
            0x3041de7d346986c2,
            Some(0x0408db7fcff15a94),
            Some(0x3041de7d346986c3),
        ),
        (
            0x09986c4f138daa1c,
            0x30146570995132d1,
            Some(0x0000000000000001),
            Some(0x30146570995132d2),
        ),
        (
            0x0e4763221e174bac,
            0x0fce64ce62412cf7,
            Some(0x0000000000000001),
            Some(0x0fce64ce79a44f16),
        ),
        (
            0x58c821e97a783236,
            0x62b626ea5c5d45c7,
            Some(0x7b90b4a98717373e),
            Some(0x62b626ea5c5d45c8),
        ),
    ];
    for &(a, b, product, total) in cases {
        assert_eq!(
            rp::upward_product(f64::from_bits(a), f64::from_bits(b))
                .ok()
                .map(f64::to_bits),
            product,
            "product {a:016x} {b:016x}"
        );
        assert_eq!(
            rp::upward_small_sum(f64::from_bits(a), f64::from_bits(b))
                .ok()
                .map(f64::to_bits),
            total,
            "single-round sum {a:016x} {b:016x}"
        );
    }
}

#[test]
fn audit_mutations_retain_native_and_product_first_failures() {
    use serde_json::json;
    let shared = corpus();
    let c = &shared["cases"][0];
    let mutations = [
        (
            "residual basis",
            json!([
                "retained_precision",
                "body",
                "cases",
                0,
                "run",
                "records",
                0,
                "residual_basis"
            ]),
            json!(128),
            "RETAINED_PRECISION_ATTEMPT_MISMATCH",
        ),
        (
            "refinement count",
            json!([
                "retained_precision",
                "body",
                "cases",
                0,
                "run",
                "records",
                0,
                "corrections"
            ]),
            json!(4),
            "RETAINED_PRECISION_ATTEMPT_MISMATCH",
        ),
        (
            "stop stage",
            json!([
                "retained_precision",
                "body",
                "cases",
                0,
                "run",
                "records",
                0,
                "work",
                "stop_rule_lme"
            ]),
            json!(0),
            "RETAINED_PRECISION_WORK_MISMATCH",
        ),
        (
            "missing entered lane",
            json!([
                "retained_precision",
                "body",
                "product_attempts",
                0,
                "proof",
                "lanes"
            ]),
            json!([]),
            "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH",
        ),
        (
            "missing completed projection",
            json!([
                "retained_precision",
                "body",
                "product_attempts",
                0,
                "proof",
                "projection_outcomes"
            ]),
            json!([]),
            "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH",
        ),
        (
            "unmerged completed values",
            json!([
                "retained_precision",
                "body",
                "product_attempts",
                0,
                "proof",
                "completion"
            ]),
            json!({"kind":"not_entered"}),
            "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH",
        ),
    ];
    for (name, path, value, code) in mutations {
        let mut source = c["source"].clone();
        edit(&mut source, &json!({"path":path,"op":"set","value":value}));
        rehash(&mut source);
        let got = rp::validate(&source, Some(&c["invocation"])).unwrap_err();
        assert_eq!((got.gate, got.code.as_str()), ("G5", code), "{name}");
    }
    let mut source = c["source"].clone();
    source["retained_precision"]["body"]["product_attempts"][0]["preparation"]["members"][0]
        ["work"]["conversions"]["value"] = json!(8);
    source["retained_precision"]["body"]["product_attempts"][0]["proof"]["completion"] =
        json!({"kind":"not_entered"});
    rehash(&mut source);
    let got = rp::validate(&source, Some(&c["invocation"])).unwrap_err();
    assert_eq!(
        (got.gate, got.code.as_str()),
        ("G5", "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"),
        "C3 association must precede earlier member work defect"
    );
}

#[test]
fn negative_zero_wire_scale_remains_g2() {
    let shared = corpus();
    let c = &shared["cases"][0];
    let mut source = c["source"].clone();
    source["retained_precision"]["body"]["cases"][0]["selection"]["body_scales"][0]
        ["translation"] = "8000000000000000".into();
    source["results"][0]["recovery_method"] = "wrong".into();
    rehash(&mut source);
    let got = rp::validate(&source, Some(&c["invocation"])).unwrap_err();
    assert_eq!(
        (got.gate, got.code.as_str()),
        ("G2", "RETAINED_PRECISION_ENCODING_MISMATCH")
    );
}

// ---------------------------------------------------------------------------
// Review repair wave 07, phase 1 (ROOT ruling "Reader review RV78-RV81",
// D1-D15): one reader-local test per relation, built from RV78's PROBES.json
// edits where one exists. These are not shared corpus entries.

fn rb(tail: Value) -> Value {
    let mut p = vec![Value::from("retained_precision"), Value::from("body")];
    p.extend(tail.as_array().unwrap().iter().cloned());
    Value::Array(p)
}
fn set(path: Value, value: Value) -> Value {
    serde_json::json!({"path": path, "op": "set", "value": value})
}
fn remove(path: Value) -> Value {
    serde_json::json!({"path": path, "op": "remove"})
}
fn base_source(shared: &Value, id: &str) -> Value {
    shared["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == id)
        .unwrap()["source"]
        .clone()
}
/// Observe a reader-local probe: `edits` on `base`, fully rehashed.
fn probe(shared: &Value, base: &str, edits: Vec<Value>) -> Value {
    observe(
        shared,
        &serde_json::json!({"id": "i63_probe", "base": base, "edits": edits, "rehash": "all"}),
    )
}
fn gate(g: &str, code: &str) -> Value {
    serde_json::json!({"gate": g, "code": code})
}
const COVERAGE: &str = "RETAINED_PRECISION_COVERAGE_MISMATCH";
const ATTEMPT: &str = "RETAINED_PRECISION_ATTEMPT_MISMATCH";
const PRODUCT: &str = "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH";
const UNSUPPORTED: &str = "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED";
const P_BASE: &str = "two_case_preparation_failure_synthetic";
const F_BASE: &str = "two_case_facade_after_certificate_synthetic";
const ORD: &str = "ordinary_prepared_synthetic";

/// D1: G3 member ids, complete old inventories, captured_prefix split, run ids
/// and execution order; Run origin owner in G5 class 1.
#[test]
fn d1_g3_coverage_relations() {
    use serde_json::json;
    let shared = corpus();
    let p = base_source(&shared, P_BASE);
    let old0 = p["retained_precision"]["body"]["product_attempts"][1]["operational"]["old"][0].clone();
    let mut old1 = old0.clone();
    old1["member"] = json!(1);
    let cases = [
        // RV78 R2b / RV80-S2: an unsourced old id outside 0..len (was G8).
        ("unsourced old id 1", P_BASE, vec![set(rb(json!(["product_attempts", 1, "operational", "old", 0, "member"])), json!(1))], gate("G3", COVERAGE)),
        // A prepared member id outside 0..len.
        ("prepared id 1", ORD, vec![set(rb(json!(["product_attempts", 0, "preparation", "members", 0, "member"])), json!(1))], gate("G3", COVERAGE)),
        // Unsourced complete old: empty, or a count other than the CaseSource's.
        ("unsourced complete old longer than the model", P_BASE, vec![set(rb(json!(["product_attempts", 1, "operational", "old"])), json!([old0, old1]))], gate("G3", COVERAGE)),
        // RV78 R3/R3b: sourced complete old against the source member map.
        ("sourced old short of source", "two_body_synthetic", vec![remove(rb(json!(["product_attempts", 0, "operational", "old", 1])))], gate("G3", COVERAGE)),
        // RV78 R1a/R1b: run id = execution-order position.
        ("execution order swapped", "two_case_synthetic", vec![set(rb(json!(["work", "execution_order"])), json!([{"kind":"case","index":1},{"kind":"case","index":0}]))], gate("G3", COVERAGE)),
        ("run id not its position", "two_case_synthetic", vec![set(rb(json!(["cases", 1, "run", "id"])), json!(5)), set(rb(json!(["calls", 0, "run_refs"])), json!([0, 5]))], gate("G3", COVERAGE)),
        // D1: Run origin owner is G5 class 1, not G3.
        ("run origin owner moved (G5 class 1)", "two_case_synthetic", vec![
            set(rb(json!(["cases", 1, "run", "origin", "owner_ref"])), json!({"kind":"case","index":0})),
            set(rb(json!(["calls", 0, "owner_refs", 1])), json!({"kind":"case","index":0})),
        ], gate("G5", ATTEMPT)),
    ];
    let mut misses = Vec::new();
    for (name, base, edits, want) in cases {
        let got = probe(&shared, base, edits);
        if got != want {
            misses.push(format!("{name}: got {got} want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
    // D1 (checkpoint A): an empty unsourced complete old list is not a G3
    // failure by itself; here the receipt has a one-member CaseSource, so the
    // count comparison rejects it at G3.
    assert_eq!(
        probe(&shared, P_BASE, vec![set(rb(json!(["product_attempts", 1, "operational", "old"])), json!([]))]),
        gate("G3", COVERAGE),
        "empty list against a one-member CaseSource"
    );
    // D1: captured_prefix references are G5 PRODUCT_ATTEMPT, not G3.
    let mut captured = shared["must_pass"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["id"] == "prefix_captured")
        .unwrap()
        .clone();
    captured["edits"].as_array_mut().unwrap().push(set(rb(json!(["product_attempts", 1, "run_ref"])), json!(0)));
    assert_eq!(observe(&shared, &captured), gate("G5", PRODUCT), "captured prefix with a run_ref");
}

/// D2: the G0 union; absent or wrong-typed G0 fields fail G0, other shape
/// defects wait for G1.
#[test]
fn d2_g0_union() {
    use serde_json::json;
    let shared = corpus();
    let mut misses = Vec::new();
    let g0 = gate("G0", UNSUPPORTED);
    for (name, edits) in [
        ("canonicalization absent", vec![remove(rb(json!(["canonicalization"])))]),
        ("policy absent", vec![remove(rb(json!(["policy"])))]),
        ("receipt_version 2", vec![set(rb(json!(["receipt_version"])), json!(2))]),
        ("receipt_version as text", vec![set(rb(json!(["receipt_version"])), json!("1"))]),
        ("case_limit absent", vec![remove(rb(json!(["work", "case_limit"])))]),
        ("invocation_limit changed", vec![set(rb(json!(["work", "invocation_limit"])), json!(60000000001u64))]),
        ("component_version", vec![set(json!(["producer", "component_version"]), json!("0.3.0"))]),
        ("schema_version", vec![set(json!(["schema_version"]), json!("0.3.0"))]),
        ("definition_id absent", vec![remove(rb(json!(["product_attempts", 0, "definition_id"])))]),
    ] {
        let got = probe(&shared, ORD, edits);
        if got != g0 {
            misses.push(format!("{name}: got {got}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
    // A shape defect outside the G0 union is G1.
    assert_eq!(
        probe(&shared, ORD, vec![set(rb(json!(["unexpected_member"])), json!(0))]),
        gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")
    );
}

/// D3: inside native class 1 an ATTEMPT defect wins over an earlier WORK one.
#[test]
fn d3_native_attempt_before_work() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, ORD, vec![
            set(rb(json!(["calls", 0, "invocation_before"])), json!(1)),
            set(rb(json!(["cases", 0, "run", "records", 0, "corrections"])), json!(4)),
            set(rb(json!(["cases", 0, "selection", "corrections"])), json!(4)),
        ]),
        gate("G5", ATTEMPT)
    );
    // D16: a dangling native build reference is WORK (C1 build provenance),
    // deferred, so a class-1 ATTEMPT defect still wins.
    let dangling = set(rb(json!(["cases", 0, "run", "records", 0, "shared_build_ref"])), json!(99));
    assert_eq!(probe(&shared, ORD, vec![dangling.clone()]), gate("G5", "RETAINED_PRECISION_WORK_MISMATCH"));
    assert_eq!(
        probe(&shared, ORD, vec![
            dangling,
            set(rb(json!(["cases", 0, "run", "records", 0, "corrections"])), json!(4)),
            set(rb(json!(["cases", 0, "selection", "corrections"])), json!(4)),
        ]),
        gate("G5", ATTEMPT)
    );
    // A dangling class-1 reference of an ATTEMPT check (a group's call) is ATTEMPT.
    assert_eq!(
        probe(&shared, "two_case_synthetic", vec![
            set(rb(json!(["calls", 0, "invocation_before"])), json!(1)),
            set(rb(json!(["groups", 0, "call"])), json!(3)),
        ]),
        gate("G5", ATTEMPT)
    );
    // The WORK defect alone still reports WORK at the end of class 1.
    assert_eq!(
        probe(&shared, ORD, vec![set(rb(json!(["calls", 0, "invocation_before"])), json!(1))]),
        gate("G5", "RETAINED_PRECISION_WORK_MISMATCH")
    );
}

/// D4 a-e: association relations (RV78 R4, R5, R6a; RV80 PR5).
#[test]
fn d4_association_relations() {
    use serde_json::json;
    let shared = corpus();
    let mut misses = Vec::new();
    let f = base_source(&shared, F_BASE);
    let attempt0 = f["retained_precision"]["body"]["product_attempts"][0].clone();
    for (name, base, edits) in [
        ("D4a source back-reference foreign (R4)", F_BASE, vec![set(rb(json!(["sources", 1, "preparation", "attempt_ref"])), json!(0))]),
        ("D4a source preparation null", F_BASE, vec![set(rb(json!(["sources", 1, "preparation"])), json!(null))]),
        ("D4b basis not the ordinary's (R5)", "two_case_two_groups_synthetic", vec![
            set(rb(json!(["product_attempts", 1, "material_basis_ref"])), json!(0)),
            set(rb(json!(["sources", 1, "material_basis_ref"])), json!(0)),
        ]),
        ("D4c cause without its own attempt (RV81-B2)", F_BASE, vec![
            set(rb(json!(["product_attempts"])), json!([attempt0])),
            set(rb(json!(["cases", 1, "product_attempt_ref"])), json!(null)),
        ]),
        ("D4d native with a selected Run (R6a)", F_BASE, vec![set(rb(json!(["product_attempts", 1, "result", "error"])), json!({"kind":"native","run_ref":1}))]),
        ("D4d native with a foreign run_ref (PR5)", F_BASE, vec![set(rb(json!(["product_attempts", 1, "result", "error"])), json!({"kind":"native","run_ref":0}))]),
        ("D4e run_ref without a native call", P_BASE, vec![set(rb(json!(["product_attempts", 1, "run_ref"])), json!(0))]),
    ] {
        let got = probe(&shared, base, edits);
        if got != gate("G5", PRODUCT) {
            misses.push(format!("{name}: got {got}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// D5 a-e: native record relations (RV78 T1, T2, T3, R8).
#[test]
fn d5_native_record_relations() {
    use serde_json::json;
    let shared = corpus();
    let mut misses = Vec::new();
    let verification_failed = json!({"kind":"rejected","reason":{"space":"attempt","tag":"verification_failed"}});
    for (name, base, edits) in [
        ("D5a candidate with a verification (T3)", ORD, vec![set(rb(json!(["cases", 0, "run", "records", 0, "verification"])), shared["cases"][0]["source"]["retained_precision"]["body"]["cases"][0]["run"]["records"][1]["verification"].clone())]),
        ("D5a candidate with verification-pass work", ORD, vec![set(rb(json!(["cases", 0, "run", "records", 0, "work", "verification_lme"])), json!(1))]),
        ("D5b escalating failed verification shows a pass (T1)", "verification_failure_skip_synthetic", vec![set(rb(json!(["cases", 0, "run", "records", 1, "work", "verification_lme"])), json!(1))]),
        ("D5c verification_failed with a completed phase", ORD, vec![
            set(rb(json!(["cases", 0, "run", "attempts", 0, "outcome"])), verification_failed.clone()),
            set(rb(json!(["cases", 0, "run", "records", 0, "outcome"])), verification_failed.clone()),
        ]),
        ("D5d stop-rule quantity of another body (T2)", "p512_ladder_synthetic", vec![
            set(rb(json!(["cases", 0, "run", "records", 0, "outcome", "reason", "quantity"])), json!({"tag":"displacement","dof":{"node":3,"component":"UX"}})),
            set(rb(json!(["cases", 0, "run", "attempts", 0, "outcome", "reason", "quantity"])), json!({"tag":"displacement","dof":{"node":3,"component":"UX"}})),
        ]),
        ("D5e group call out of range (R8)", "two_case_synthetic", vec![set(rb(json!(["groups", 0, "call"])), json!(3))]),
    ] {
        let got = probe(&shared, base, edits);
        if got != gate("G5", ATTEMPT) {
            misses.push(format!("{name}: got {got}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// D6b (checks_passed part), D6c, D6d and D7.
#[test]
fn d6_d7_ordinary_and_diagnostic_relations() {
    use serde_json::json;
    let shared = corpus();
    // D6b (RV78 T4e): a selected case with checks_passed ordinary quality.
    assert_eq!(
        probe(&shared, ORD, vec![
            set(json!(["numerical_quality", "cases", 0, "solve_quality"]), json!("checks_passed")),
            set(rb(json!(["ordinary_attempts", 0, "initial", "outcome"])), json!("checks_passed")),
        ]),
        gate("G5", ATTEMPT)
    );
    // D6b (checkpoint A): not_assessed does not route to retained precision.
    assert_eq!(
        probe(&shared, ORD, vec![set(json!(["numerical_quality", "cases", 0, "solve_quality"]), json!("not_assessed"))]),
        gate("G5", ATTEMPT)
    );
    // D6a as amended by F5 (D-U6-7; A2): the untyped list is exactly the
    // diagnostics naming the case, in envelope order, excluding
    // RETAINED_PRECISION_*. Checkpoint A's relaxed form (a listed diagnostic of
    // another scope is admitted) is refused since snapshot 07g.
    let other = base_source(&shared, ORD)["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| {
            !d["affected_refs"]
                .as_array()
                .is_some_and(|a| a.contains(&json!("case:six-component-load")))
        })
        .map(|d| d["id"].clone());
    {
        let other = other.expect("a diagnostic of another scope in the base");
        // RV90 N3 (07h): the base's exact list plus the other-scope element, so
        // that element is the only defect (`[integrity, other]` was also 07f's
        // relaxed form, refused under F5 without `other`).
        let mut refs = base_source(&shared, ORD)["retained_precision"]["body"]["ordinary_attempts"][0]["diagnostic_refs"]
            .as_array()
            .unwrap()
            .clone();
        assert!(!refs.contains(&other));
        refs.push(other);
        assert_eq!(
            probe(&shared, ORD, vec![set(rb(json!(["ordinary_attempts", 0, "diagnostic_refs"])), Value::Array(refs))]),
            gate("G5", ATTEMPT),
            "F5: a listed diagnostic of another scope is refused"
        );
        assert_eq!(probe(&shared, ORD, vec![]), Value::Null, "F5: the repaired base's exact list is admitted");
    }
    for (name, refs) in [
        ("duplicate (RV78 T4a)", json!(["diagnostic:numerical-integrity:case:six-component-load", "diagnostic:numerical-integrity:case:six-component-load"])),
        ("dangling (RV78 T4b)", json!(["diagnostic:numerical-integrity:case:six-component-load", "diagnostic:rv78:absent"])),
    ] {
        assert_eq!(
            probe(&shared, ORD, vec![set(rb(json!(["ordinary_attempts", 0, "diagnostic_refs"])), refs)]),
            gate("G5", ATTEMPT),
            "{name}"
        );
    }
    // D6c: a published W2 preserving a Formation initial failure is admitted
    // with a nonzero exponent b and rejected with b = 0 (C2:158).
    let trigger_error = json!({"tag":"numerical_range","name":"synthetic"});
    let w2 = |b: i64| {
        vec![
            set(rb(json!(["ordinary_attempts", 0, "initial"])), json!({"kind":"formation_failure","error":trigger_error,"basis_index":0})),
            set(rb(json!(["ordinary_attempts", 0, "w2"])), json!({"kind":"published","trigger":{"tag":"formation","error":trigger_error},"force_scale_exponent":b,"report_diagnostic_ref":"diagnostic:numerical-integrity:case:six-component-load"})),
        ]
    };
    assert_eq!(probe(&shared, ORD, w2(1)), Value::Null, "W2 published, b = 1");
    assert_eq!(probe(&shared, ORD, w2(0)), gate("G5", ATTEMPT), "W2 published, b = 0");
    // RV78 T-4e/T4d: ordinary (class 2) precedes the deferred C3 work list.
    assert_eq!(
        probe(&shared, F_BASE, vec![
            set(rb(json!(["ordinary_attempts", 1, "diagnostic_refs"])), json!(["diagnostic:numerical-integrity:case:unavailable-row", "diagnostic:rv78:absent"])),
            set(rb(json!(["product_attempts", 1, "adapter", "fault"])), json!({"kind":"overflow","event":"allocation_request"})),
        ]),
        gate("G5", ATTEMPT)
    );
    // D6d: a legacy work_ref that does not resolve is a reference check (ATTEMPT).
    assert_eq!(
        probe(&shared, ORD, vec![set(rb(json!(["ordinary_attempts", 0, "legacy_source", "work_ref"])), json!(0))]),
        gate("G5", ATTEMPT)
    );
    // D7: a RETAINED_PRECISION_SELECTED diagnostic naming no requested case.
    let diagnostics = base_source(&shared, ORD)["diagnostics"].clone();
    let i = diagnostics
        .as_array()
        .unwrap()
        .iter()
        .position(|d| d["code"] == "RETAINED_PRECISION_SELECTED")
        .unwrap();
    assert_eq!(
        probe(&shared, ORD, vec![set(json!(["diagnostics", i, "affected_refs"]), json!(["case:absent"]))]),
        gate("G4", "RETAINED_PRECISION_DIAGNOSTIC_MISMATCH")
    );
}

/// D13 reader-local pins, and kills for RV80's surviving mutants where the
/// rule is implemented (M07 by d1 above, M12, M13-M15 by d4 above, M08, M16,
/// M18).
#[test]
fn d13_reader_local_pins_and_mutant_kills() {
    use serde_json::json;
    let shared = corpus();
    // theta = +0 on a no-data body (selection and record theta both moved).
    assert_eq!(
        probe(&shared, "ordinary_prepared_no_data_synthetic", vec![
            set(rb(json!(["cases", 0, "selection", "theta", 0, "value"])), json!("3fd0000000000000")),
            set(rb(json!(["cases", 0, "run", "records", 1, "verification", "theta", 0, "value"])), json!("3fd0000000000000")),
        ]),
        gate("G5a", "RETAINED_PRECISION_SCALE_MISMATCH")
    );
    // M12 (RV80 PR3): an unavailable attempt's record bound must follow has_data.
    assert_eq!(
        probe(&shared, F_BASE, vec![set(rb(json!(["cases", 1, "run", "records", 1, "verification", "bound", 0, "value"])), json!(null))]),
        gate("G5a", "RETAINED_PRECISION_SCALE_MISMATCH")
    );
    // The Ceiling after a p128 verification-solve failure: v256 fails with an
    // escalating stop (two slots), the fresh p512 candidate is rejected and its
    // v1024 only solved.
    let schedule = rp::reader_logic::schedule;
    let mut run = base_source(&shared, "verification_failure_skip_synthetic")["retained_precision"]["body"]["cases"][0]["run"].clone();
    assert!(schedule(&run).is_ok());
    let stop_rule = json!({"kind":"rejected","reason":{"space":"attempt","tag":"stop_rule","quantity":{"tag":"displacement","dof":{"node":1,"component":"UX"}},"body":0,"kind":"translation"}});
    run["attempts"][1]["outcome"] = stop_rule.clone();
    run["records"][2]["outcome"] = stop_rule;
    run["records"][3]["outcome"] = json!({"kind":"solved"});
    run["kernel_terminal"] = json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"ceiling"}});
    assert!(schedule(&run).is_ok(), "Ceiling after a p128 verification-solve failure");
    run["kernel_terminal"] = json!({"kind":"refused","reason":{"space":"refusal","tag":"structure"}});
    assert!(attempt_mismatch(schedule(&run)));
    // M16: a terminal Budget stop ends on its exact translation, scope included.
    let mut budget = base_source(&shared, ORD)["retained_precision"]["body"]["cases"][0]["run"].clone();
    let stop = json!({"space":"attempt","tag":"stop","stop":{"space":"stop","tag":"budget","scope":"case"}});
    budget["records"] = json!([budget["records"][0].clone()]);
    budget["records"][0]["outcome"] = json!({"kind":"failed","reason":stop});
    budget["attempts"][0]["outcome"] = json!({"kind":"failed","reason":stop});
    budget["attempts"][0]["verification"] = json!(null);
    budget["kernel_terminal"] = json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"budget","scope":"case"}});
    assert!(schedule(&budget).is_ok());
    budget["kernel_terminal"]["reason"]["scope"] = json!("invocation");
    assert!(attempt_mismatch(schedule(&budget)), "Budget scope is part of the translation");
    // M08: an idle group-null Run is Budget(invocation) exactly at exhaustion.
    let body = json!({"work":{"invocation_limit":100},"groups":[]});
    let idle = |before: u64| {
        json!({"records":[],"attempts":[],"case_charge":0,"invocation_increment":0,
               "invocation_before":before,"origin":{"group":null},
               "kernel_terminal":{"kind":"unresolved","reason":{"space":"unresolved","tag":"budget","scope":"invocation"}}})
    };
    assert!(rp::reader_logic::schedule_in(&idle(100), &body).is_ok());
    assert!(attempt_mismatch(rp::reader_logic::schedule_in(&idle(99), &body)));
    // R3' with `both`: both faults must be emitted by the cause's owner (here
    // the ProofTrace, for a proof cause).
    let attempt = |proof: Value| {
        json!({"adapter":{"fault":null},
               "result":{"kind":"unavailable","error":{"kind":"proof","cause":{"kind":"work_accounting","fault":"both"}}},
               "proof":proof})
    };
    assert_eq!(
        rp::reader_logic::accounting(&attempt(json!({"numeric":{"kind":"unavailable","fault":"overflow"}}))),
        [true, true, false, true]
    );
    assert_eq!(
        rp::reader_logic::accounting(&attempt(json!({"numeric":{"kind":"unavailable","fault":"overflow"},"sticky_status":"inconsistent"}))),
        [true, true, true, true]
    );
    // R3' binds to the owner: a fault emitted elsewhere in the attempt does not count.
    let mut elsewhere = attempt(json!({"numeric":{"kind":"exact","value":0}}));
    elsewhere["overlay_work"] = json!({"sticky_status":"both"});
    assert_eq!(rp::reader_logic::accounting(&elsewhere), [true, true, false, true]);
    // R1' and R2' (lost; OperationalError accounting on an old operational entry).
    assert_eq!(
        rp::reader_logic::accounting(&json!({"adapter":{"fault":null},"x":{"kind":"accounting","event":"map_write"},"y":{"lost":true}})),
        [false, false, true, true]
    );
    assert_eq!(
        rp::reader_logic::accounting(&json!({"adapter":{"fault":null},"operational":{"old":[{"result":{"kind":"refused","error":{"kind":"accounting"}}}],"new":[]}})),
        [true, false, true, true]
    );
    // R4: a SectionError accounting needs a non-exact status in its member's work.
    let member = |work: Value| {
        json!({"adapter":{"fault":null},"preparation":{"members":[{"result":{"kind":"refused","error":{"kind":"accounting"}},"work":work}]}})
    };
    assert_eq!(rp::reader_logic::accounting(&member(json!({"sticky_status":"exact"}))), [true, true, true, false]);
    assert_eq!(rp::reader_logic::accounting(&member(json!({"sticky_status":"overflow"}))), [true, true, true, true]);
    // M18: G7 keeps the bare base code and carries any further text as detail.
    let e = rp::reader_logic::g7_error("SOURCE_PREVIEW_PHYSICS_ROW_SIGNATURE: bad row");
    assert_eq!((e.gate, e.code.as_str()), ("G7", "SOURCE_PREVIEW_PHYSICS_ROW_SIGNATURE"));
    assert_eq!(e.detail.as_deref(), Some("SOURCE_PREVIEW_PHYSICS_ROW_SIGNATURE: bad row"));
    let e = rp::reader_logic::g7_error("SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS");
    assert_eq!((e.code.as_str(), e.detail), ("SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS", None));
}

/// D13: the 2^-988 switch in the absolute bound, on both sides. Expected bits
/// from an exact rational oracle (least binary64 at or above the exact value).
#[test]
fn d13_absolute_bound_small_scale_switch() {
    for (scale, value, expected) in [
        (0x0230000000000000u64, 0x0000000000000000u64, 0x0000000000400000u64),
        (0x0230000000000000, 0x3ff0000000000000, 0x0000000000400000),
        (0x0230000000000000, 0x4330000000000000, 0x0000000000400000),
        (0x022fffffffffffff, 0x0000000000000000, 0x0000000000400001),
        (0x022fffffffffffff, 0x3ff0000000000000, 0x3ca0000000000001),
        (0x022fffffffffffff, 0x4330000000000000, 0x3fe0000000000001),
    ] {
        assert_eq!(
            rp::absolute_bound(f64::from_bits(value), f64::from_bits(scale))
                .unwrap()
                .to_bits(),
            expected,
            "{scale:016x} {value:016x}"
        );
    }
}

/// D8 kernel scope (checkpoint A; C1:66-68): a work_accounting stop anywhere in
/// a Run, a build or a group preparation is a class-1 ATTEMPT defect, ahead of
/// the deferred native WORK checks (a failed build reason here would be WORK).
#[test]
fn d8_kernel_scope_work_accounting_anywhere() {
    use serde_json::json;
    let shared = corpus();
    let wa = json!({"space":"stop","tag":"work_accounting","fault":"overflow"});
    for (name, edits) in [
        ("build reason", vec![set(rb(json!(["builds", 0, "reason"])), wa.clone())]),
        ("record outcome", vec![
            set(rb(json!(["cases", 0, "run", "records", 0, "outcome"])), json!({"kind":"failed","reason":{"space":"attempt","tag":"stop","stop":wa}})),
        ]),
    ] {
        assert_eq!(probe(&shared, ORD, edits), gate("G5", ATTEMPT), "{name}");
    }
}

/// D8 R2' and R4 on shared bases (also pinned by 07's shared mutations).
#[test]
fn d8_accounting_rules_on_shared_bases() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, P_BASE, vec![set(rb(json!(["product_attempts", 1, "operational", "old", 0, "result"])), json!({"kind":"refused","error":{"kind":"accounting"}}))]),
        gate("G5", "RETAINED_PRECISION_WORK_MISMATCH"),
        "R2' old operational accounting without a lost trace"
    );
}

// ---------------------------------------------------------------------------
// Confirmation repair round (ROOT rulings D19-D30): reader-local tests.

/// D19: the converse of D4c. An unavailable C3 attempt needs a
/// prepared_product_failure cause naming it (RV79-C1); a Ready attempt's case is
/// selected or unavailable with receipt_failure. B1's alignment set (item 3) puts
/// C2's cause table first, in the ordinary class, so each D19 probe states the
/// cause with its C2 phase and code; a cause its case cannot carry (a precondition
/// beside a Run) is refused by that table at G5 ATTEMPT.
#[test]
fn d19_converse_cause_binding() {
    use serde_json::json;
    let shared = corpus();
    let preparation_error = base_source(&shared, P_BASE)["retained_precision"]["body"]
        ["product_attempts"][1]["result"]["error"]
        .clone();
    let receipt_failure = json!({"kind":"receipt_failure","check":"association","field_path":"retained_precision.body"});
    let precondition =
        json!({"kind":"unavailable_precondition","precondition":"capture","affected_refs":[]});
    let receipt = json!({"code":"receipt_encoding","phase":"receipt","cause":receipt_failure});
    for (name, base, edits, want) in [
        (
            "unavailable attempt under receipt_failure",
            F_BASE,
            vec![set(rb(json!(["cases", 1, "reason"])), receipt.clone())],
            gate("G5", PRODUCT),
        ),
        (
            "a precondition beside the case's Run (C2's table)",
            F_BASE,
            vec![set(
                rb(json!(["cases", 1, "reason", "cause"])),
                precondition.clone(),
            )],
            gate("G5", ATTEMPT),
        ),
        (
            "preparation error with a selected Run under receipt_failure (RV79-B1c)",
            F_BASE,
            vec![
                set(rb(json!(["cases", 1, "reason"])), receipt.clone()),
                set(
                    rb(json!(["product_attempts", 1, "result", "error"])),
                    preparation_error.clone(),
                ),
            ],
            gate("G5", PRODUCT),
        ),
        (
            "P' preparation failure under unavailable_precondition",
            P_BASE,
            vec![set(
                rb(json!(["cases", 1, "reason", "cause"])),
                precondition.clone(),
            )],
            gate("G5", PRODUCT),
        ),
    ] {
        assert_eq!(probe(&shared, base, edits), want, "{name}");
    }
    // A Ready attempt in an unavailable case: receipt_failure only.
    let unavailable_case = |reason: Value| {
        vec![
            remove(rb(json!(["cases", 1, "method"]))),
            remove(rb(json!(["cases", 1, "selection"]))),
            remove(rb(json!(["cases", 1, "source_identity_sha256"]))),
            set(rb(json!(["cases", 1, "status"])), json!("unavailable")),
            set(rb(json!(["cases", 1, "reason"])), reason),
            set(
                rb(json!(["cases", 1, "diagnostic_ref"])),
                json!("diagnostic:retained:synthetic-zero-load"),
            ),
            set(
                json!([
                    "diagnostics",
                    base_source(&shared, "two_case_synthetic")["diagnostics"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .position(|d| d["id"] == "diagnostic:retained:synthetic-zero-load")
                        .unwrap(),
                    "code"
                ]),
                json!("RETAINED_PRECISION_UNAVAILABLE"),
            ),
        ]
    };
    let facade = json!({"kind":"facade_failure","owner_ref":{"kind":"case","index":1},"row_id":null,"recipe":"identity","operand_index":null,"check":"identity","predicate":null});
    assert_eq!(
        probe(
            &shared,
            "two_case_synthetic",
            unavailable_case(json!({"code":"facade_certificate","phase":"facade","cause":facade}))
        ),
        gate("G5", PRODUCT),
        "Ready attempt under a C2 cause"
    );
    // With receipt_failure the D19 relation holds; the next defect is the
    // unavailable case's rows still carrying a recovery method (G6).
    assert_eq!(
        probe(&shared, "two_case_synthetic", unavailable_case(receipt)),
        gate("G6", "RETAINED_PRECISION_ROW_METHOD_MISMATCH"),
        "Ready attempt under receipt_failure"
    );
}

/// D20: a selected case without a C3 attempt is a C3 association defect
/// (PRODUCT_ATTEMPT), split out of the ordinary check.
#[test]
fn d20_selected_without_c3_attempt() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, ORD, vec![
            set(rb(json!(["product_attempts"])), json!([])),
            set(rb(json!(["cases", 0, "product_attempt_ref"])), json!(null)),
        ]),
        gate("G5", PRODUCT)
    );
}

/// D21: a verification shared build on an escalating failed verification is
/// evidence that the verification pass ran (RV79-C2).
#[test]
fn d21_verification_shared_build_is_pass_evidence() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, "verification_failure_skip_synthetic", vec![set(rb(json!(["cases", 0, "run", "records", 1, "verification_shared_build_ref"])), json!(0))]),
        gate("G5", ATTEMPT)
    );
}

/// D22: a dangling attempt source_ref is G5 PRODUCT_ATTEMPT; the dependent G3
/// checks are skipped.
#[test]
fn d22_dangling_attempt_source_ref() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, F_BASE, vec![set(rb(json!(["product_attempts", 1, "source_ref"])), json!(9))]),
        gate("G5", PRODUCT)
    );
}

/// D24: the harness applies `after_rehash` edits after the rehash; forged
/// receipt and publication hashes give G1.
#[test]
fn d24_after_rehash_edits() {
    use serde_json::json;
    let shared = corpus();
    for (name, path) in [
        ("receipt hash", json!(["retained_precision", "receipt_sha256"])),
        ("publication hash", rb(json!(["publication_sha256"]))),
    ] {
        let entry = json!({"id":"i63_after_rehash","base":ORD,"edits":[],"rehash":"all",
            "after_rehash":[{"path":path,"op":"set","value":"0".repeat(64)}]});
        assert_eq!(observe(&shared, &entry), gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH"), "{name}");
    }
    let entry = json!({"id":"i63_after_rehash_none","base":ORD,"edits":[],"rehash":"all","after_rehash":[]});
    assert_eq!(observe(&shared, &entry), Value::Null);
}

/// D25: readers validate parsed values; an integral float counter is the same
/// number as the integer (I-JSON/JCS), so the receipt still validates.
#[test]
fn d25_integral_float_is_the_same_number() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, ORD, vec![
            set(rb(json!(["cases", 0, "run", "case_charge"])), json!(17.0)),
            set(rb(json!(["work", "charged"])), json!(17.0)),
        ]),
        Value::Null
    );
}

/// D27: the idle (group-null) Run rule reads the recorded invocation_before;
/// a broken meter chain is native WORK (RV80 PR14).
#[test]
fn d27_idle_rule_reads_recorded_invocation_before() {
    use serde_json::json;
    let shared = corpus();
    let run = |k: &str| rb(json!(["cases", 1, "run", k]));
    assert_eq!(
        probe(&shared, F_BASE, vec![
            set(run("records"), json!([])),
            set(run("attempts"), json!([])),
            set(run("case_charge"), json!(0)),
            set(run("invocation_increment"), json!(0)),
            set(run("invocation_before"), json!(60000000000u64)),
            set(run("invocation_after"), json!(60000000000u64)),
            set(run("cache_before"), json!([])),
            set(run("cache_after"), json!([])),
            set(rb(json!(["cases", 1, "run", "origin", "group"])), json!(null)),
            set(rb(json!(["cases", 1, "run", "kernel_terminal"])), json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"budget","scope":"invocation"}})),
            set(rb(json!(["groups", 0, "source_refs"])), json!([0])),
        ]),
        gate("G5", "RETAINED_PRECISION_WORK_MISMATCH")
    );
}

/// D28: every quantity-bearing attempt reason resolves to a layout row with the
/// same body and kind (RV80 PR12).
#[test]
fn d28_quantity_reasons_resolve_to_layout() {
    use serde_json::json;
    let shared = corpus();
    let mut misses = Vec::new();
    for (tag, extra) in [
        ("verification_estimate", json!({})),
        ("charge", json!({})),
        ("publication_enclosure", json!({"predicate":"absolute_bound"})),
    ] {
        let mut reason = json!({"space":"attempt","tag":tag,"quantity":{"tag":"displacement","dof":{"node":99,"component":"UX"}},"body":0,"kind":"translation"});
        for (k, v) in extra.as_object().unwrap() {
            reason[k] = v.clone();
        }
        let got = probe(&shared, "p512_ladder_synthetic", vec![
            set(rb(json!(["cases", 0, "run", "records", 0, "outcome", "reason"])), reason.clone()),
            set(rb(json!(["cases", 0, "run", "attempts", 0, "outcome", "reason"])), reason),
        ]);
        if got != gate("G5", ATTEMPT) {
            misses.push(format!("{tag}: got {got}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// D29: an empty body inventory under a coverage roster fails G3 (RV80 PR11).
#[test]
fn d29_empty_body_inventory() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, ORD, vec![
            set(rb(json!(["sources", 0, "body_membership"])), json!([])),
            set(rb(json!(["product_attempts", 0, "proof", "summary_coverage"])), json!([])),
        ]),
        gate("G3", COVERAGE)
    );
}

/// D29: any CaseSource with an empty body inventory fails G3, with or without
/// a coverage roster (here on cert_failed_before_summary, coverage null).
#[test]
fn d29_empty_body_inventory_without_roster() {
    use serde_json::json;
    let shared = corpus();
    let mut entry = shared["must_pass"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["id"] == "cert_failed_before_summary")
        .unwrap()
        .clone();
    assert_eq!(observe(&shared, &entry), Value::Null);
    entry["edits"]
        .as_array_mut()
        .unwrap()
        .push(set(rb(json!(["sources", 1, "body_membership"])), json!([])));
    assert_eq!(observe(&shared, &entry), gate("G3", COVERAGE));
}

/// D30: the native run_ref on a nonselected Run (kills M13; no shared base has
/// a nonselected native Run).
#[test]
fn d30_native_run_ref_on_nonselected_run() {
    use serde_json::json;
    let case = |run_id: u64| {
        json!({"status":"unavailable","reason":{"code":"kernel_unresolved","phase":"kernel","cause":{"kind":"prepared_product_failure","product_attempt_ref":0}},
               "run":{"id":run_id,"kernel_terminal":{"kind":"unresolved","reason":{"space":"unresolved","tag":"ceiling"}}}})
    };
    let attempt = json!({"result":{"kind":"unavailable","error":{"kind":"native","run_ref":3}},"stages":{"native":"failed"},"proof":null});
    assert!(rp::reader_logic::reason_table(&case(3), &attempt).is_ok());
    let got = rp::reader_logic::reason_table(&case(2), &attempt).unwrap_err();
    assert_eq!((got.gate, got.code.as_str()), ("G5", PRODUCT));
}

/// D31: G8 admits model schema_version 0.1.0 or 0.2.0 without a pressure
/// contract; 0.4.0 stays excluded. B3a (B3-D §6.3, decision B3D-10): 0.3.0
/// without a contract is outside D1.3 (the producer cannot emit it,
/// `PRESSURE_CONTRACT_REQUIRED`), so it is G8 INVOCATION_MISMATCH; 0.3.0 with
/// the legacy contract is `b3a_legacy_pressure_contract_namespace_at_g8`'s.
/// The invocation edit rebinds the receipt's invocation digest.
#[test]
fn d31_model_schema_versions_at_g8() {
    use serde_json::json;
    let shared = corpus();
    let entry = |version: &str| {
        json!({"id":"i63_d31","base":ORD,"edits":[],"rehash":"all",
            "invocation_edits":[{"path":["request","model","schema_version"],"op":"set","value":version}]})
    };
    let mut misses = Vec::new();
    for (version, want) in [
        ("0.1.0", Value::Null),
        ("0.2.0", Value::Null),
        ("0.3.0", gate("G8", "RETAINED_PRECISION_INVOCATION_MISMATCH")),
        ("0.4.0", gate("G8", "RETAINED_PRECISION_INVOCATION_MISMATCH")),
        ("0.0.9", gate("G8", "RETAINED_PRECISION_INVOCATION_MISMATCH")),
    ] {
        let got = observe(&shared, &entry(version));
        if got != want {
            misses.push(format!("{version}: got {got}, want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// Every integer in `v` rewritten as the integral float of the same value.
fn as_integral_floats(v: &mut Value) {
    match v {
        Value::Number(n) if !n.is_f64() => *v = Value::from(n.as_f64().unwrap()),
        Value::Array(a) => a.iter_mut().for_each(as_integral_floats),
        Value::Object(o) => o.values_mut().for_each(as_integral_floats),
        _ => {}
    }
}

/// D32: integers by value everywhere. G0's receipt_version and work limits,
/// references and every other receipt integer written as integral floats
/// validate; a non-integral, -0 or wrong value still fails where it did.
#[test]
fn d32_integers_by_value() {
    use serde_json::json;
    let shared = corpus();
    let g0 = gate("G0", UNSUPPORTED);
    let mut misses = Vec::new();
    for (name, edits, want) in [
        ("receipt_version 1.0", vec![set(rb(json!(["receipt_version"])), json!(1.0))], Value::Null),
        ("float limits", vec![
            set(rb(json!(["work", "case_limit"])), json!(20000000000.0)),
            set(rb(json!(["work", "invocation_limit"])), json!(60000000000.0)),
        ], Value::Null),
        ("float source_ref", vec![set(rb(json!(["cases", 0, "source_ref"])), json!(0.0))], Value::Null),
        ("float attempt refs", vec![
            set(rb(json!(["cases", 0, "product_attempt_ref"])), json!(0.0)),
            set(rb(json!(["sources", 0, "preparation", "attempt_ref"])), json!(0.0)),
        ], Value::Null),
        ("float quality binding", vec![set(rb(json!(["cases", 0, "ordinary", "quality_binding", "index"])), json!(0.0))], Value::Null),
        ("receipt_version 1.5", vec![set(rb(json!(["receipt_version"])), json!(1.5))], g0.clone()),
        ("receipt_version -0", vec![set(rb(json!(["receipt_version"])), json!(-0.0))], g0.clone()),
        ("receipt_version 2.0", vec![set(rb(json!(["receipt_version"])), json!(2.0))], g0.clone()),
        ("case_limit 20000000000.5", vec![set(rb(json!(["work", "case_limit"])), json!(20000000000.5))], g0.clone()),
        ("invocation_limit 6e10+1", vec![set(rb(json!(["work", "invocation_limit"])), json!(60000000001.0))], g0.clone()),
        ("source_ref -0", vec![set(rb(json!(["cases", 0, "source_ref"])), json!(-0.0))], gate("G2", "RETAINED_PRECISION_ENCODING_MISMATCH")),
        ("source_ref 0.5", vec![set(rb(json!(["cases", 0, "source_ref"])), json!(0.5))], gate("G2", "RETAINED_PRECISION_ENCODING_MISMATCH")),
    ] {
        let got = probe(&shared, ORD, edits);
        if got != want {
            misses.push(format!("{name}: got {got}, want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
    // Every receipt integer at once, on every complete base.
    for case in shared["cases"].as_array().unwrap() {
        let mut source = case["source"].clone();
        as_integral_floats(&mut source["retained_precision"]["body"]);
        rehash(&mut source);
        // A base stated refused bound (PR-B2 ruling 5) keeps its stated refusal.
        if case["expected"].get("gate").is_some() {
            assert_eq!(observe_validation(rp::validate(&source, Some(&case["invocation"]))), case["expected"], "{}", case["id"]);
            continue;
        }
        assert!(
            rp::validate(&source, Some(&case["invocation"])).is_ok(),
            "{}: {:?}",
            case["id"],
            rp::validate(&source, Some(&case["invocation"])).err()
        );
    }
}

/// D32 shared pin shape: a forged source identity under `source_ref: 0.0`
/// fails G1, exactly as under `source_ref: 0`.
#[test]
fn d32_forged_source_identity_under_float_ref() {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    use serde_json::json;
    let shared = corpus();
    for (name, forge) in [("control", false), ("forged", true)] {
        for source_ref in [json!(0), json!(0.0)] {
            let entry = json!({"id":"i63_d32_identity","base":ORD,"rehash":"all",
                "edits":[{"path":["retained_precision","body","cases",0,"source_ref"],"op":"set","value":source_ref}]});
            let (mut source, invocation) = apply_entry(&shared, &entry);
            if forge {
                // The digest of a different source statement, receipt rehashed.
                let mut other = source["retained_precision"]["body"]["sources"][0].clone();
                other.as_object_mut().unwrap().remove("index");
                other["stiffness_sha256"] = json!("0".repeat(64));
                source["retained_precision"]["body"]["cases"][0]["source_identity_sha256"] =
                    domain_hash("retained_precision_source_mp_v2", &other).unwrap().into();
                source["retained_precision"]["receipt_sha256"] = domain_hash(
                    "retained_precision_receipt_mp_v2",
                    &source["retained_precision"]["body"],
                )
                .unwrap()
                .into();
            }
            let got = match rp::validate(&source, Some(&invocation)) {
                Err(e) => json!({"gate":e.gate,"code":e.code}),
                Ok(_) => Value::Null,
            };
            let want = if forge { gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH") } else { Value::Null };
            assert_eq!(got, want, "{name} source_ref {source_ref}");
        }
    }
}

/// D33 (RV80-N1, PR16): a verification_estimate reason names a Force or Moment
/// layout row; on an existing translation or rotation row it is G5 ATTEMPT.
/// `charge` is unrestricted.
#[test]
fn d33_verification_estimate_names_force_or_moment() {
    use serde_json::json;
    let shared = corpus();
    let mut misses = Vec::new();
    for (tag, quantity, kind, want) in [
        ("verification_estimate", json!({"tag":"displacement","dof":{"node":1,"component":"UX"}}), "translation", gate("G5", ATTEMPT)),
        ("verification_estimate", json!({"tag":"displacement","dof":{"node":1,"component":"RX"}}), "rotation", gate("G5", ATTEMPT)),
        ("verification_estimate", json!({"tag":"end_action","member":0,"end":"i","component":"UX"}), "force", Value::Null),
        ("verification_estimate", json!({"tag":"end_action","member":0,"end":"i","component":"RX"}), "moment", Value::Null),
        ("charge", json!({"tag":"displacement","dof":{"node":1,"component":"UX"}}), "translation", Value::Null),
    ] {
        let reason = json!({"space":"attempt","tag":tag,"quantity":quantity,"body":0,"kind":kind});
        let got = probe(&shared, "p512_ladder_synthetic", vec![
            set(rb(json!(["cases", 0, "run", "records", 0, "outcome", "reason"])), reason.clone()),
            set(rb(json!(["cases", 0, "run", "attempts", 0, "outcome", "reason"])), reason),
        ]);
        if got != want {
            misses.push(format!("{tag} on {kind}: got {got}, want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// RV80-N2 (kills M39): D21's last-slot case. A Ceiling-shaped Run whose last
/// attempt is a p256 candidate whose escalating v512 verification solve fails:
/// no fresh attempt follows, so the replay's own shared-build check never
/// runs and only the D21 record check catches a verification shared build.
/// The control's class-1 checks (ATTEMPT and WORK) all pass; it fails later,
/// because no Ceiling base with a native-faithful case exists (deferred).
#[test]
fn d21_last_slot_verification_shared_build() {
    use serde_json::json;
    let shared = corpus();
    let run = |tail: Value| {
        let mut p = json!(["cases", 0, "run"]);
        p.as_array_mut().unwrap().extend(tail.as_array().unwrap().iter().cloned());
        rb(p)
    };
    let stop = json!({"space":"attempt","tag":"stop","stop":{"space":"stop","tag":"condition"}});
    let rejected = json!({"kind":"rejected","reason":{"space":"attempt","tag":"verification_failed"}});
    let zero = |w: &mut Value| {
        for (_, v) in w.as_object_mut().unwrap() {
            *v = json!(0);
        }
    };
    let base = base_source(&shared, "p512_ladder_synthetic");
    let r = &base["retained_precision"]["body"]["cases"][0]["run"];
    let mut rec2 = r["records"][2].clone();
    rec2["role"] = json!("verification");
    rec2["outcome"] = json!({"kind":"failed","reason":stop});
    rec2["verification"] = Value::Null;
    rec2["verification_shared_build_ref"] = Value::Null;
    let w = &mut rec2["work"];
    zero(&mut w["own_stages"]);
    w["own_stages"]["solve"] = json!(3);
    w["wide_lme"] = json!(3);
    w["own_lme"] = json!(3);
    w["stop_rule_lme"] = json!(0);
    w["verification_lme"] = json!(0);
    w["verification_shared_lme"] = json!(0);
    w["verification_shared_built_here"] = json!(false);
    zero(&mut w["shared_stages"]);
    w["shared_stages"]["formation"] = json!(6);
    let mut att1 = r["attempts"][1].clone();
    att1["verification"] = json!({"record":2,"precision":512,"phase":"failed","reason":stop});
    att1["outcome"] = rejected.clone();
    att1["case_charge"] = json!(10);
    att1["invocation_increment"] = json!(10);
    let mut edits = vec![
        remove(run(json!(["records", 3]))),
        remove(run(json!(["attempts", 2]))),
        set(run(json!(["records", 2])), rec2),
        set(run(json!(["records", 1, "outcome"])), rejected),
        set(run(json!(["attempts", 1])), att1),
        set(run(json!(["kernel_terminal"])), json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"ceiling"}})),
        set(run(json!(["cache_after"])), json!([{"slot":"s128","build":0},{"slot":"s256","build":1},{"slot":"s512","build":3},{"slot":"v256","build":2}])),
        set(run(json!(["case_charge"])), json!(27)),
        set(run(json!(["invocation_increment"])), json!(27)),
        set(run(json!(["invocation_after"])), json!(27)),
        set(rb(json!(["calls", 0, "invocation_after"])), json!(27)),
        set(rb(json!(["work", "charged"])), json!(27)),
        remove(rb(json!(["builds", 6]))),
        remove(rb(json!(["builds", 5]))),
        remove(rb(json!(["builds", 4]))),
    ];
    let control = probe(&shared, "p512_ladder_synthetic", edits.clone());
    assert!(
        control != gate("G5", ATTEMPT) && control != gate("G5", "RETAINED_PRECISION_WORK_MISMATCH"),
        "control must clear G5 class 1: {control}"
    );
    // The schedule replay alone accepts the Ceiling run with or without the build.
    edits.push(set(run(json!(["records", 2, "verification_shared_build_ref"])), json!(2)));
    let entry = json!({"id":"i63_d21_last_slot","base":"p512_ladder_synthetic","edits":edits,"rehash":"all"});
    let (mutated, _) = apply_entry(&shared, &entry);
    let replay = &mutated["retained_precision"]["body"]["cases"][0]["run"];
    assert!(rp::reader_logic::schedule(replay).is_ok());
    assert_eq!(observe(&shared, &entry), gate("G5", ATTEMPT));
}

/// RV80-N1 (M56): the same entry on the metadata-only transport path.
fn transport(shared: &Value, entry: &Value) -> Value {
    let (source, _) = apply_entry(shared, entry);
    match rp::validate_transport_metadata(&source) {
        Err(e) => serde_json::json!({"gate":e.gate,"code":e.code}),
        Ok(_) => serde_json::json!(null),
    }
}
fn transport_probe(shared: &Value, base: &str, edits: Vec<Value>) -> Value {
    transport(
        shared,
        &serde_json::json!({"id": "i63_transport_probe", "base": base, "edits": edits, "rehash": "all"}),
    )
}

/// D34 (RV79-N1): a JSON number equal to -0 anywhere in the receipt fails G2
/// ENCODING, including the integer fields the schema writes as enum or const
/// values, which no base carries: `G5aError.quantity_kind` (on F_BASE's
/// unavailable attempt) and `source_decline.constructor_counts.directional_springs`
/// (on the shared `unavailable_attempt_under_source_error_cause` entry). Each
/// control with +0 passes G1 and G2 and fails where it did.
#[test]
fn d34_negative_zero_anywhere_in_receipt_fails_g2() {
    use serde_json::json;
    let shared = corpus();
    let g2 = gate("G2", "RETAINED_PRECISION_ENCODING_MISMATCH");
    let mut misses = Vec::new();
    let mut check = |name: &str, got: Value, want: &Value| {
        if got != *want {
            misses.push(format!("{name}: got {got}, want {want}"));
        }
    };
    let error = rb(json!(["product_attempts", 1, "result", "error"]));
    for kind in ["sanity", "lower"] {
        let cause = |q: Value| {
            let mut c = json!({"kind":kind,"quantity_kind":q});
            c[if kind == "sanity" { "body" } else { "member" }] = json!(0);
            json!({"kind":"g5a","cause":c})
        };
        let control = probe(&shared, F_BASE, vec![set(error.clone(), cause(json!(0)))]);
        assert!(
            !matches!(control["gate"].as_str(), Some("G0" | "G1" | "G2")) && !control.is_null(),
            "{kind} control: {control}"
        );
        check(&format!("{kind} quantity_kind -0"), probe(&shared, F_BASE, vec![set(error.clone(), cause(json!(-0.0)))]), &g2);
        // RV80-N1 (M56): the transport path applies the same G2.
        check(&format!("{kind} quantity_kind -0 transport"), transport_probe(&shared, F_BASE, vec![set(error.clone(), cause(json!(-0.0)))]), &g2);
        let t = transport_probe(&shared, F_BASE, vec![set(error.clone(), cause(json!(0)))]);
        assert!(t != g2 && !matches!(t["gate"].as_str(), Some("G0" | "G1")), "{kind} transport control: {t}");
        check(&format!("{kind} quantity_kind 1"), probe(&shared, F_BASE, vec![set(error.clone(), cause(json!(1)))]), &control);
        // The JSON text "-0" parses as -0 and is caught the same way.
        let entry = json!({"id":"i63_d34_text","base":F_BASE,"rehash":"all","edits":[set(error.clone(), cause(json!(-0.0)))]});
        let (source, invocation) = apply_entry(&shared, &entry);
        let text = serde_json::to_string(&source).unwrap().replace("\"quantity_kind\":-0.0", "\"quantity_kind\":-0");
        assert!(text.contains("\"quantity_kind\":-0}") || text.contains("\"quantity_kind\":-0,"));
        let parsed: Value = serde_json::from_str(&text).unwrap();
        let got = rp::validate(&parsed, Some(&invocation)).unwrap_err();
        check(&format!("{kind} text -0"), json!({"gate":got.gate,"code":got.code}), &g2);
        let got = rp::validate(&parsed, None).unwrap_err();
        check(&format!("{kind} text -0 without invocation"), json!({"gate":got.gate,"code":got.code}), &g2);
        let got = rp::validate_transport_metadata(&parsed).unwrap_err();
        check(&format!("{kind} text -0 transport"), json!({"gate":got.gate,"code":got.code}), &g2);
    }
    let mut entry = shared["mutations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["id"] == "unavailable_attempt_under_source_error_cause")
        .unwrap()
        .clone();
    check("source_decline control", observe(&shared, &entry), expected_for(&entry));
    entry["edits"][1]["value"]["constructor_counts"]["directional_springs"] = json!(-0.0);
    check("directional_springs -0", observe(&shared, &entry), &g2);
    check("directional_springs -0 transport", transport(&shared, &entry), &g2);
    entry["edits"][1]["value"]["constructor_counts"]["directional_springs"] = json!(1);
    check("directional_springs 1", observe(&shared, &entry), &gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH"));
    // An encoded U field written as -0 stays G2.
    check(
        "case_charge -0",
        probe(&shared, ORD, vec![set(rb(json!(["cases", 0, "run", "case_charge"])), json!(-0.0))]),
        &g2,
    );
    check(
        "case_charge -0 transport",
        transport_probe(&shared, ORD, vec![set(rb(json!(["cases", 0, "run", "case_charge"])), json!(-0.0))]),
        &g2,
    );
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// RV78-N1 (07e format rule): the harness rehash indexes only a strict integral
/// value: a JSON number, never a boolean, finite, integral, >= 0 and not -0
/// (0.0 is index 0; 0.5, true and -0.0 are not). A reference that is not an
/// index, or does not resolve, is skipped and left for the reader to report.
#[test]
fn rehash_index_rule_07e() {
    use serde_json::json;
    for (v, want) in [(json!(0), 0), (json!(1), 1), (json!(1.0), 1), (json!(0.0), 0), (json!(7), 7)] {
        assert_eq!(index(&v), Some(want), "{v}");
    }
    for v in [json!(true), json!(false), json!(0.5), json!(-0.0), json!(-1), json!(-1.0), json!(null), json!("0"), json!([0]), json!({"index":0})] {
        assert_eq!(index(&v), None, "{v}");
    }
    // Unresolvable references are skipped: the rehash leaves the stated digest
    // for the reader, which reports the dangling or malformed reference.
    let shared = corpus();
    let mut source = base_source(&shared, ORD);
    let before = source["retained_precision"]["body"]["cases"][0]["source_identity_sha256"].clone();
    for r in [json!(true), json!(0.5), json!(-0.0), json!(9)] {
        source["retained_precision"]["body"]["cases"][0]["source_ref"] = r.clone();
        source["retained_precision"]["body"]["sources"][0]["preparation"]["attempt_ref"] = r.clone();
        rehash(&mut source);
        assert_eq!(source["retained_precision"]["body"]["cases"][0]["source_identity_sha256"], before, "{r}");
    }
    // 07o (S-6 steps 2, 4 and 5): the same rule for an operand-prepared source's
    // `operand_preparation_ref`, a CombinationSource operand's `source_ref` and a
    // combination entry's `source_ref`, each on W-CB3.
    let (fresh, _) = b2_w_cb3("sparse_interactive");
    for r in [json!(true), json!(0.5), json!(-0.0), json!(9)] {
        for (reference, digest) in [
            (json!(["sources", 1, "preparation", "operand_preparation_ref"]), json!(["sources", 1, "preparation", "sha256"])),
            (json!(["sources", 2, "operands", 1, "source_ref"]), json!(["sources", 2, "operands", 1, "source_identity_sha256"])),
            (json!(["combinations", 0, "source_ref"]), json!(["combinations", 0, "source_identity_sha256"])),
        ] {
            let mut source = fresh.clone();
            let pointer = |v: &Value| format!("/retained_precision/body/{}", v.as_array().unwrap().iter().map(|x| x.to_string().trim_matches('"').to_owned()).collect::<Vec<_>>().join("/"));
            let before = source.pointer(&pointer(&digest)).unwrap().clone();
            edit(&mut source, &set(rb(reference.clone()), r.clone()));
            rehash(&mut source);
            assert_eq!(source.pointer(&pointer(&digest)).unwrap(), &before, "{reference} {r}");
        }
    }
}

/// D37 (D35 widened; RV78-S1/S2, RV79-X1): every product-attempt error kind
/// agrees with the stage record in both directions, at G5 PRODUCT_ATTEMPT.
/// Probes on F_BASE's unavailable attempt 1 (every pipeline stage completed,
/// certificate passed, observables and G5a not entered, a capture error), on
/// P_BASE's preparation failure, and on the must-pass shapes for the other
/// kinds. Each consistent shape stays clear of PRODUCT_ATTEMPT.
#[test]
fn d37_error_kind_agrees_with_stage_record() {
    use serde_json::json;
    let shared = corpus();
    let product = gate("G5", PRODUCT);
    let a1 = |k: &str| rb(json!(["product_attempts", 1, k]));
    let error = rb(json!(["product_attempts", 1, "result", "error"]));
    let storage = json!({"kind":"storage","detail":"adapter vector"});
    let proof_error = |k: &str| json!({"kind":k,"cause":{"kind":"storage"}});
    let g5a_error = json!({"kind":"g5a","cause":{"kind":"zero","row":0}});
    let observable_error = json!({"kind":"observable","cause":storage});
    let mut stages = base_source(&shared, F_BASE)["retained_precision"]["body"]["product_attempts"][1]["stages"].clone();
    let checks = |obs: Value, g: Value| json!({"certificate":{"kind":"passed"},"observables":obs,"g5a":g});
    let mut misses = Vec::new();
    let mut run = |name: &str, base: &str, edits: Vec<Value>, rejected: bool| {
        let got = probe(&shared, base, edits);
        if (got == product) != rejected {
            misses.push(format!("{name}: got {got}, rejected wanted {rejected}"));
        }
    };
    // The base shape (capture after a completed certificate) is consistent.
    run("capture after certificate (base)", F_BASE, vec![], false);
    // X1 / Y-direction: the kind presupposes stages the record does not show.
    run("g5a with G5a not entered", F_BASE, vec![set(error.clone(), g5a_error.clone())], true);
    run("observable with observables not entered", F_BASE, vec![set(error.clone(), observable_error.clone())], true);
    run("proof with certificate passed", F_BASE, vec![set(error.clone(), proof_error("proof"))], true);
    run("values with values completed", F_BASE, vec![set(error.clone(), json!({"kind":"values","cause":{"kind":"storage"},"proof":{"kind":"storage"}}))], true);
    run("numeric with checks not entered", F_BASE, vec![set(error.clone(), json!({"kind":"numeric","cause":null}))], true);
    run("abandoned with certificate completed", F_BASE, vec![set(error.clone(), json!({"kind":"abandoned","cause":storage,"proof":{"kind":"storage"}}))], true);
    // After a completed certificate both checks are entered.
    stages["observables"] = json!("completed");
    stages["g5a"] = json!("completed");
    let all_passed = vec![set(a1("stages"), stages.clone()), set(rb(json!(["product_attempts", 1, "proof", "checks"])), checks(json!({"kind":"passed"}), json!({"kind":"passed"})))];
    let with = |mut v: Vec<Value>, e: Value| {
        v.push(set(error.clone(), e));
        v
    };
    run("numeric with both checks passed", F_BASE, with(all_passed.clone(), json!({"kind":"numeric","cause":null})), false);
    run("capture at the commit (every stage completed)", F_BASE, with(all_passed.clone(), json!({"kind":"capture","cause":storage})), false);
    run("g5a with G5a passed", F_BASE, with(all_passed.clone(), g5a_error.clone()), true);
    run("observable with observables passed", F_BASE, with(all_passed.clone(), observable_error.clone()), true);
    let mut g5a_failed = stages.clone();
    g5a_failed["g5a"] = json!("failed");
    let g5a_shape = vec![set(a1("stages"), g5a_failed.clone()), set(rb(json!(["product_attempts", 1, "proof", "checks"])), checks(json!({"kind":"passed"}), json!({"kind":"failed","error":g5a_error})))];
    run("g5a with observables passed and G5a failed", F_BASE, with(g5a_shape.clone(), g5a_error.clone()), false);
    run("numeric with G5a failed", F_BASE, with(g5a_shape.clone(), json!({"kind":"numeric","cause":null})), true);
    run("capture with G5a failed", F_BASE, with(g5a_shape, json!({"kind":"capture","cause":storage})), true);
    let mut observable_failed = stages.clone();
    observable_failed["observables"] = json!("failed");
    let observable_shape = vec![set(a1("stages"), observable_failed.clone()), set(rb(json!(["product_attempts", 1, "proof", "checks"])), checks(json!({"kind":"failed","error":observable_error}), json!({"kind":"passed"})))];
    run("observable with observables failed", F_BASE, with(observable_shape.clone(), observable_error.clone()), false);
    run("g5a with observables failed", F_BASE, with(observable_shape.clone(), g5a_error.clone()), true);
    run("numeric with observables failed", F_BASE, with(observable_shape, json!({"kind":"numeric","cause":null})), true);
    // The other direction on a failed stage (P9): P_BASE's failed preparation.
    let p_error = rb(json!(["product_attempts", 1, "result", "error"]));
    run("preparation failure (base)", P_BASE, vec![], false);
    for (name, e) in [
        ("proof after failed preparation", proof_error("proof")),
        ("numeric after failed preparation", json!({"kind":"numeric","cause":null})),
        ("g5a after failed preparation", g5a_error.clone()),
        ("capture after failed preparation", json!({"kind":"capture","cause":{"kind":"storage","detail":"prepared vector"}})),
    ] {
        run(name, P_BASE, vec![set(p_error.clone(), e)], true);
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// RV94 S-1 on 07j's two-case statement whose second case is not_required: with
/// the receipt's case order the standing is eligible and the summary Current;
/// with the not_required case omitted, or the two reordered, the standing is
/// `needs_recompute` and the summary not-Current ([73, 0]).
#[test]
fn u7_07j_not_required_case_omitted_or_reordered_is_not_current() {
    use open_pipe_stress_result_export::semantic_contract as s;
    let shared = corpus();
    let entry = shared["must_pass"].as_array().unwrap().iter().find(|e| e["id"] == "not_required_second_case_checks_passed").unwrap();
    let (source, invocation) = apply_entry(&shared, entry);
    let cases = source["retained_precision"]["body"]["cases"].as_array().unwrap();
    assert_eq!(cases.iter().map(|c| c["status"].as_str().unwrap()).collect::<Vec<_>>(), ["selected", "not_required"]);
    let order: Vec<Value> = cases.iter().map(|c| c["basis_ref"].clone()).collect();
    assert_eq!(s::numerical_use_standing_with_context(&source, &order, Some(&invocation)), "numerically_eligible");
    let withheld = |refs: &[Value]| -> Vec<u64> {
        s::classification_summary(&source, Some(&invocation), refs).iter().map(|c| c["withheld"].as_u64().unwrap()).collect()
    };
    let current = withheld(&order);
    assert_eq!(withheld(&[]), [73, 0]);
    assert_ne!(current, [73, 0]);
    let reversed: Vec<Value> = order.iter().rev().cloned().collect();
    for refs in [order[..1].to_vec(), reversed] {
        assert_eq!(s::numerical_use_standing_with_context(&source, &refs, Some(&invocation)), "needs_recompute", "{refs:?}");
        assert_eq!(withheld(&refs), [73, 0], "{refs:?}");
    }
}

// ---------------------------------------------------------------------------
// B1 SR-RS (I90; PLAN_v2 §2.4; DESIGN_v2 §2 and §3.2-§3.3): reader-local tests on
// synthetic n-case receipts, derived from the shared two-case bases and must-pass entries.
// They are not shared corpus entries: SC's 07n pins the shared ones (W-C2,
// `d38_beside_selected` with m1-m8, F-1's five, and `not_required`'s three).

/// An entry on `base`: a must-pass entry's edits (when named), then `edits`; the
/// invocation edits rebind the receipt's invocation digest (`apply_entry`).
fn b1_entry(shared: &Value, base: &str, must: Option<&str>, edits: Vec<Value>, invocation: Vec<Value>) -> Value {
    let mut all = Vec::new();
    if let Some(id) = must {
        let entry = shared["must_pass"].as_array().unwrap().iter().find(|e| e["id"] == id).unwrap();
        assert_eq!(entry["base"], base, "{id}");
        all.extend(entry["edits"].as_array().unwrap().iter().cloned());
    }
    all.extend(edits);
    serde_json::json!({"id": "b1_probe", "base": base, "edits": all, "invocation_edits": invocation, "rehash": "all"})
}
/// The reader's verdict: `{"admitted": eligible}`, or the first failure.
fn b1_verdict(shared: &Value, entry: &Value) -> Value {
    let (source, invocation) = apply_entry(shared, entry);
    match rp::validate(&source, Some(&invocation)) {
        Ok(v) => serde_json::json!({"admitted": v.numerical_eligible}),
        Err(e) => gate(e.gate, &e.code),
    }
}
fn admitted(eligible: bool) -> Value {
    serde_json::json!({"admitted": eligible})
}
/// The index of a base's row of `kind` for load case `case`.
fn b1_row(source: &Value, kind: &str, case: &str) -> usize {
    source["results"].as_array().unwrap().iter().position(|r| r["kind"] == kind && r["basis_ref"]["ref_id"] == case).unwrap()
}
/// A base's rows with `extra` appended, as one `results` edit.
fn b1_rows(source: &Value, extra: Vec<Value>) -> Value {
    let mut rows = source["results"].as_array().unwrap().clone();
    rows.extend(extra);
    set(serde_json::json!(["results"]), Value::Array(rows))
}
/// The dense base's parity row, moved to `case` with a fresh id. A non-selected
/// case's row carries no recovery method (G6).
fn b1_parity_row(shared: &Value, case: &str, id: &str) -> Value {
    let mut row = base_source(shared, "ordinary_prepared_dense_synthetic")["results"][1].clone();
    assert_eq!(row["kind"], "sparse_live_path_dense_parity_relative_delta");
    row["id"] = serde_json::json!(id);
    row["basis_ref"]["ref_id"] = serde_json::json!(case);
    row.as_object_mut().unwrap().remove("recovery_method");
    row
}
const PREP: &str = "RETAINED_PRECISION_PREPARATION_MISMATCH";
const NOT_REQUIRED: &str = "not_required_second_case_checks_passed";
const UNAVAILABLE_ROW: &str = "case:unavailable-row";

/// R-D38 (4b), DESIGN_v2 §2, on F_BASE: case 1 (unavailable, with its own CaseSource and a
/// prepared attempt) is rewritten so that its native stage failed before any Run, beside
/// selected case 0, as SC's `d38_beside_selected` rewrites W-C2's case C: no Run, no
/// `execution_order`, Call or Group entry, the call's after-value and `charged` recomputed
/// (case 1's Run built nothing: it reused case 0's builds), a typed capture cause, and
/// every hash resealed.
fn d38_edits(shared: &Value) -> Vec<Value> {
    use serde_json::json;
    let body = &base_source(shared, F_BASE)["retained_precision"]["body"];
    assert!(body["builds"].as_array().unwrap().iter().all(|b| b["origin"]["run"] == 0), "case 1's Run built nothing");
    let after = body["cases"][0]["run"]["invocation_after"].clone();
    let mut stages = serde_json::Map::new();
    for k in ["preparation", "native", "proof_start", "projection", "maxima", "values", "aliases", "certificate", "observables", "g5a"] {
        stages.insert(k.into(), json!("not_entered"));
    }
    stages.insert("preparation".into(), json!("completed"));
    stages.insert("native".into(), json!("failed"));
    vec![
        set(rb(json!(["cases", 1, "run"])), Value::Null),
        set(rb(json!(["cases", 1, "reason"])), json!({"code": "source_unavailable", "phase": "preparation", "cause": {"kind": "prepared_product_failure", "product_attempt_ref": 1}})),
        set(rb(json!(["product_attempts", 1, "run_ref"])), Value::Null),
        set(rb(json!(["product_attempts", 1, "proof"])), Value::Null),
        set(rb(json!(["product_attempts", 1, "stages"])), Value::Object(stages)),
        set(rb(json!(["product_attempts", 1, "result"])), json!({"kind": "unavailable", "error": {"kind": "capture", "cause": {"kind": "origin", "cause": {"kind": "capacity"}}}})),
        set(rb(json!(["calls", 0, "owner_refs"])), json!([{"kind": "case", "index": 0}])),
        set(rb(json!(["calls", 0, "source_refs"])), json!([0])),
        set(rb(json!(["calls", 0, "run_refs"])), json!([0])),
        set(rb(json!(["calls", 0, "invocation_after"])), after.clone()),
        set(rb(json!(["groups", 0, "source_refs"])), json!([0])),
        set(rb(json!(["work", "charged"])), after),
        set(rb(json!(["work", "execution_order"])), json!([{"kind": "case", "index": 0}])),
    ]
}

/// R-D38 (4b) admitted beside a selected case: G0-G8 pass and the standing is
/// needs_recompute. Before B1 the reader refused it (an entered native stage needed a Run).
/// m1-m8 (DESIGN_v2 §2) and the other (4b) conjuncts are refused, each at this reader's
/// first failure.
#[test]
fn b1_d38_capture_before_any_run_beside_a_selected_case() {
    use open_pipe_stress_result_export::semantic_contract as s;
    use serde_json::json;
    let shared = corpus();
    let d38 = d38_edits(&shared);
    let entry = b1_entry(&shared, F_BASE, None, d38.clone(), vec![]);
    assert_eq!(b1_verdict(&shared, &entry), admitted(false), "(4b) beside a selected case");
    let (source, invocation) = apply_entry(&shared, &entry);
    let order: Vec<Value> = source["retained_precision"]["body"]["cases"].as_array().unwrap().iter().map(|c| c["basis_ref"].clone()).collect();
    assert_eq!(s::numerical_use_standing_with_context(&source, &order, Some(&invocation)), "needs_recompute");
    let with = |extra: Vec<Value>| -> Value {
        let mut edits = d38.clone();
        edits.extend(extra);
        b1_entry(&shared, F_BASE, None, edits, vec![])
    };
    let attempt = |tail: Value| {
        rb(json!(["product_attempts", 1])
            .as_array()
            .unwrap()
            .iter()
            .cloned()
            .chain(tail.as_array().unwrap().iter().cloned())
            .collect())
    };
    let cases = [
        (
            "m1 error kind native",
            vec![set(
                attempt(json!(["result", "error"])),
                json!({"kind": "native", "run_ref": 1}),
            )],
            gate("G5", PRODUCT),
        ),
        (
            "m2 native completed",
            vec![set(
                attempt(json!(["stages", "native"])),
                json!("completed"),
            )],
            gate("G5", PRODUCT),
        ),
        (
            "m3 run_ref while the case has no Run",
            vec![set(attempt(json!(["run_ref"])), json!(1))],
            gate("G5", PRODUCT),
        ),
        (
            "m4 execution_order still lists the case",
            vec![set(
                rb(json!(["work", "execution_order"])),
                json!([{"kind": "case", "index": 0}, {"kind": "case", "index": 1}]),
            )],
            gate("G3", COVERAGE),
        ),
        (
            "m5 proof_start completed",
            vec![set(
                attempt(json!(["stages", "proof_start"])),
                json!("completed"),
            )],
            gate("G5", PRODUCT),
        ),
        (
            "m6 source_ref null with preparation completed",
            vec![set(attempt(json!(["source_ref"])), Value::Null)],
            gate("G5", PRODUCT),
        ),
        (
            "m7 the case's source in the call's source_refs",
            vec![set(rb(json!(["calls", 0, "source_refs"])), json!([0, 1]))],
            gate("G5", ATTEMPT),
        ),
        (
            "m7 the case's source in the group's source_refs",
            vec![set(rb(json!(["groups", 0, "source_refs"])), json!([0, 1]))],
            gate("G5", ATTEMPT),
        ),
        (
            "m8 case source_ref differs from the attempt's",
            vec![set(rb(json!(["cases", 1, "source_ref"])), json!(0))],
            gate("G5", PRODUCT),
        ),
        (
            "result ready",
            vec![set(attempt(json!(["result"])), json!({"kind": "ready"}))],
            gate("G5", PRODUCT),
        ),
        (
            "preparation failed",
            vec![set(
                attempt(json!(["stages", "preparation"])),
                json!("failed"),
            )],
            gate("G5", PRODUCT),
        ),
        (
            "observables and G5a entered",
            vec![
                set(attempt(json!(["stages", "observables"])), json!("failed")),
                set(attempt(json!(["stages", "g5a"])), json!("failed")),
            ],
            gate("G5", PRODUCT),
        ),
        (
            "reason code kernel_unresolved",
            vec![set(
                rb(json!(["cases", 1, "reason", "code"])),
                json!("kernel_unresolved"),
            )],
            gate("G5", PRODUCT),
        ),
        (
            "reason phase kernel",
            vec![set(
                rb(json!(["cases", 1, "reason", "phase"])),
                json!("kernel"),
            )],
            gate("G5", PRODUCT),
        ),
        (
            "cause names the other attempt",
            vec![set(
                rb(json!([
                    "cases",
                    1,
                    "reason",
                    "cause",
                    "product_attempt_ref"
                ])),
                json!(0),
            )],
            gate("G5", PRODUCT),
        ),
        // C2's cause table (B1's alignment set, item 3) refuses it first, in the ordinary class.
        (
            "cause not a prepared product failure",
            vec![set(
                rb(json!(["cases", 1, "reason", "cause"])),
                json!({"kind": "receipt_failure", "check": "association", "field_path": "b1"}),
            )],
            gate("G5", ATTEMPT),
        ),
    ];
    let mut misses = Vec::new();
    for (name, edits, want) in cases {
        let got = b1_verdict(&shared, &with(edits));
        if got != want {
            misses.push(format!("{name}: got {got} want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// R-D38 (4a), which B1 leaves unchanged (RV113 N-1): a native failure with the case's own
/// non-selected Run is admitted at `validate` and never enters (4b)'s branch. No shared base
/// has a non-selected native Run (D30's note), so F_BASE's case 1 Run is made idle in its
/// ready group (refused `ledger_unavailable`, as a CasePrep failure leaves it; no records, no
/// charge), the case `kernel_refused`, and its attempt native-failed (preparation completed,
/// no proof) with a native error naming that Run, or with a capture error. (4b)'s reason on
/// this Run-bearing shape is refused.
#[test]
fn b1_d38_4a_native_failure_with_a_run_is_admitted() {
    use serde_json::json;
    let shared = corpus();
    let body = &base_source(&shared, F_BASE)["retained_precision"]["body"];
    let mut run = body["cases"][1]["run"].clone();
    let before = run["invocation_before"].clone();
    run["records"] = json!([]);
    run["attempts"] = json!([]);
    run["case_charge"] = json!(0);
    run["invocation_increment"] = json!(0);
    run["invocation_after"] = before.clone();
    run["cache_after"] = run["cache_before"].clone();
    run["kernel_terminal"] = json!({"kind": "refused", "reason": {"space": "refusal", "tag": "ledger_unavailable", "error": {"tag": "accumulator", "error": {"tag": "non_finite"}}}});
    let mut attempt = body["product_attempts"][1].clone();
    assert_eq!(attempt["run_ref"], json!(1), "the attempt keeps its Run");
    attempt["proof"] = Value::Null;
    for k in ["preparation", "native", "proof_start", "projection", "maxima", "values", "aliases", "certificate", "observables", "g5a"] {
        attempt["stages"][k] = json!("not_entered");
    }
    attempt["stages"]["preparation"] = json!("completed");
    attempt["stages"]["native"] = json!("failed");
    attempt["result"] = json!({"kind": "unavailable", "error": {"kind": "native", "run_ref": 1}});
    let a4 = vec![
        set(rb(json!(["cases", 1, "run"])), run),
        set(rb(json!(["cases", 1, "reason"])), json!({"code": "kernel_refused", "phase": "kernel", "cause": {"kind": "prepared_product_failure", "product_attempt_ref": 1}})),
        set(rb(json!(["product_attempts", 1])), attempt),
        set(rb(json!(["calls", 0, "invocation_after"])), before.clone()),
        set(rb(json!(["work", "charged"])), before),
    ];
    let with = |extra: Vec<Value>| -> Value { b1_entry(&shared, F_BASE, None, a4.iter().cloned().chain(extra).collect(), vec![]) };
    let cases = [
        ("(4a): a native error naming the case's refused Run", with(vec![]), admitted(false)),
        ("(4a): a capture error beside the case's refused Run", with(vec![set(rb(json!(["product_attempts", 1, "result", "error"])), json!({"kind": "capture", "cause": {"kind": "origin", "cause": {"kind": "capacity"}}}))]), admitted(false)),
        ("(4b)'s reason on the Run-bearing failure", with(vec![set(rb(json!(["cases", 1, "reason", "code"])), json!("source_unavailable")), set(rb(json!(["cases", 1, "reason", "phase"])), json!("preparation"))]), gate("G5", PRODUCT)),
    ];
    let mut misses = Vec::new();
    for (name, entry, want) in cases {
        let got = b1_verdict(&shared, &entry);
        if got != want {
            misses.push(format!("{name}: got {got} want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// G5's `not_required` rule (DESIGN_v2 §3.3, decision 9), on 07j's two-case statement
/// (selected, then not_required): a W2-published case with the verdict checks_passed (T-4's
/// case B, by an evaluation or a formation trigger) is admitted and the statement is
/// eligible. A non-null product attempt (another case's, at G3; the case's own, by the rule
/// itself), `initial` not_attempted and another verdict stay refused, and a Passed-verdict
/// report keeps its outcome equality.
#[test]
fn b1_g5_not_required_admits_a_w2_published_case() {
    use open_pipe_stress_result_export::semantic_contract as s;
    use serde_json::json;
    let shared = corpus();
    let report = base_source(&shared, P_BASE)["retained_precision"]["body"]["ordinary_attempts"][1]["initial"]["report_diagnostic_ref"].clone();
    let ordinary = |tail: &str| rb(json!(["ordinary_attempts", 1, tail]));
    let evaluation = vec![
        set(ordinary("initial"), json!({"kind": "structural_failure", "error": {"tag": "range", "detail": "b1"}, "diagnostic_ref": null})),
        set(ordinary("w2"), json!({"kind": "published", "trigger": {"tag": "evaluation", "error": {"tag": "range", "detail": "b1"}}, "force_scale_exponent": 3, "report_diagnostic_ref": report})),
    ];
    let formation = vec![
        set(ordinary("initial"), json!({"kind": "formation_failure", "error": {"tag": "numerical_range", "name": "b1"}, "basis_index": 0})),
        set(ordinary("w2"), json!({"kind": "published", "trigger": {"tag": "formation", "error": {"tag": "numerical_range", "name": "b1"}}, "force_scale_exponent": -2, "report_diagnostic_ref": report})),
    ];
    for (name, edits) in [("evaluation", &evaluation), ("formation", &formation)] {
        let entry = b1_entry(&shared, P_BASE, Some(NOT_REQUIRED), edits.clone(), vec![]);
        assert_eq!(b1_verdict(&shared, &entry), admitted(true), "{name}");
        let (source, invocation) = apply_entry(&shared, &entry);
        assert_eq!(rp::reader_logic::ordinary(&source), Ok(()), "{name}: G5's ordinary pass");
        let order: Vec<Value> = source["retained_precision"]["body"]["cases"].as_array().unwrap().iter().map(|c| c["basis_ref"].clone()).collect();
        assert_eq!(s::numerical_use_standing_with_context(&source, &order, Some(&invocation)), "numerically_eligible", "{name}");
    }
    let mut sensitive = evaluation.clone();
    sensitive.push(set(json!(["numerical_quality", "cases", 1, "solve_quality"]), json!("sensitive")));
    let cases = [
        ("W2-published, verdict sensitive", sensitive, gate("G5", ATTEMPT)),
        ("initial not_attempted", vec![set(ordinary("initial"), json!({"kind": "not_attempted", "cause": "ineligible"}))], gate("G5", ATTEMPT)),
        ("report outcome differs from the verdict", vec![set(rb(json!(["ordinary_attempts", 1, "initial", "outcome"])), json!("sensitive"))], gate("G5", ATTEMPT)),
        ("product_attempt_ref non-null (G3 first)", vec![set(rb(json!(["cases", 1, "product_attempt_ref"])), json!(0))], gate("G3", COVERAGE)),
    ];
    let mut misses = Vec::new();
    for (name, edits, want) in cases {
        let got = b1_verdict(&shared, &b1_entry(&shared, P_BASE, Some(NOT_REQUIRED), edits, vec![]));
        if got != want {
            misses.push(format!("{name}: got {got} want {want}"));
        }
    }
    // RV113 S-1: the rule's `product_attempt_ref` null conjunct. 07j's entry without its
    // removal of `product_attempts/1` keeps case 1's own attempt, and the not_required case
    // names it: G3 passes (the attempt exists and is the case's), so the rule itself refuses
    // it, at G5 ATTEMPT (G5's ordinary class, before D19's PRODUCT_ATTEMPT).
    let nr = shared["must_pass"].as_array().unwrap().iter().find(|e| e["id"] == NOT_REQUIRED).unwrap();
    let removal = remove(rb(json!(["product_attempts", 1])));
    let mut own: Vec<Value> = nr["edits"].as_array().unwrap().iter().filter(|e| **e != removal).cloned().collect();
    assert_eq!(own.len() + 1, nr["edits"].as_array().unwrap().len(), "07j removes case 1's attempt once");
    own.push(set(rb(json!(["cases", 1, "product_attempt_ref"])), json!(1)));
    let got = b1_verdict(&shared, &b1_entry(&shared, P_BASE, None, own, vec![]));
    if got != gate("G5", ATTEMPT) {
        misses.push(format!("product_attempt_ref naming the case's own attempt: got {got}"));
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// F-1 text B's P1 and the requested mode (DESIGN_v2 §3.2-§3.3), in G8's per-case loop:
/// every case is checked, here the second one, unavailable (F_BASE) or not_required (07j).
#[test]
fn b1_g8_mode_row_and_requested_mode_for_every_case() {
    use serde_json::json;
    let shared = corpus();
    let f = base_source(&shared, F_BASE);
    let p = base_source(&shared, P_BASE);
    let fm = b1_row(&f, "linear_solver_mode_basis", UNAVAILABLE_ROW);
    let pm = b1_row(&p, "linear_solver_mode_basis", UNAVAILABLE_ROW);
    let mut duplicate = f["results"][fm].clone();
    duplicate["id"] = json!("result:b1:duplicate-mode");
    let cases = [
        ("unavailable case: dense code in sparse", b1_entry(&shared, F_BASE, None, vec![set(json!(["results", fm, "value"]), json!(2.0))], vec![])),
        ("unavailable case: mode code 3", b1_entry(&shared, F_BASE, None, vec![set(json!(["results", fm, "value"]), json!(3.0))], vec![])),
        ("unavailable case: two mode rows", b1_entry(&shared, F_BASE, None, vec![b1_rows(&f, vec![duplicate])], vec![])),
        ("unavailable case: requested mode flipped", b1_entry(&shared, F_BASE, None, vec![set(rb(json!(["ordinary_attempts", 1, "requested_mode"])), json!("dense_scrutiny"))], vec![])),
        ("not_required case: dense code in sparse", b1_entry(&shared, P_BASE, Some(NOT_REQUIRED), vec![set(json!(["results", pm, "value"]), json!(2.0))], vec![])),
    ];
    let mut misses = Vec::new();
    for (name, entry) in cases {
        let got = b1_verdict(&shared, &entry);
        if got != gate("G8", PREP) {
            misses.push(format!("{name}: got {got}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// F-1 text B's P2-P4 (DESIGN_v2 §3.2) for every case. 07j's two-case statement is made
/// dense (the invocation's mode, both requested modes and both mode rows; it has no parity
/// row): the selected dense case at b = 0 without a parity row is admitted (before B1 it
/// needed exactly one), and so is a not_required case's single parity row at b = 0. Two
/// parity rows (P2), a parity row in sparse_interactive (P3) and a parity row on a
/// W2-published case (P4) are refused at G8; a parity row beside a W2 that failed (so
/// published nothing) is admitted, since P4 reads published, not triggered.
#[test]
fn b1_g8_parity_rows_p2_to_p4_for_every_case() {
    use serde_json::json;
    let shared = corpus();
    let p = base_source(&shared, P_BASE);
    let report = p["retained_precision"]["body"]["ordinary_attempts"][1]["initial"]["report_diagnostic_ref"].clone();
    let dense = vec![
        set(rb(json!(["ordinary_attempts", 0, "requested_mode"])), json!("dense_scrutiny")),
        set(rb(json!(["ordinary_attempts", 1, "requested_mode"])), json!("dense_scrutiny")),
    ];
    let mut dense_rows = p.clone();
    for case in ["case:six-component-load", UNAVAILABLE_ROW] {
        dense_rows["results"][b1_row(&p, "linear_solver_mode_basis", case)]["value"] = json!(2.0);
    }
    let to_dense = vec![set(json!(["solver_mode"]), json!("dense_scrutiny"))];
    let w2 = vec![
        set(rb(json!(["ordinary_attempts", 1, "initial"])), json!({"kind": "structural_failure", "error": {"tag": "range", "detail": "b1"}, "diagnostic_ref": null})),
        set(rb(json!(["ordinary_attempts", 1, "w2"])), json!({"kind": "published", "trigger": {"tag": "evaluation", "error": {"tag": "range", "detail": "b1"}}, "force_scale_exponent": 3, "report_diagnostic_ref": report})),
    ];
    // A W2 that was triggered and failed published nothing (b = 0).
    let w2_failed = vec![
        w2[0].clone(),
        set(rb(json!(["ordinary_attempts", 1, "w2"])), json!({"kind": "failed", "trigger": {"tag": "evaluation", "error": {"tag": "range", "detail": "b1"}}, "failure": {"tag": "not_engaged"}, "diagnostic_ref": report})),
    ];
    let one = b1_parity_row(&shared, UNAVAILABLE_ROW, "result:b1:parity-1");
    let two = b1_parity_row(&shared, UNAVAILABLE_ROW, "result:b1:parity-2");
    // Both mode rows carry the dense code; `rows` are appended to them in the one results edit.
    let on_dense = |rows: Vec<Value>, extra: Vec<Value>| -> Value {
        let mut edits = dense.clone();
        edits.push(b1_rows(&dense_rows, rows));
        edits.extend(extra);
        b1_entry(&shared, P_BASE, Some(NOT_REQUIRED), edits, to_dense.clone())
    };
    // Dense, ordinary only: the selected source, then the dense base's selected case with W2.
    let d = base_source(&shared, "ordinary_prepared_dense_synthetic");
    let mut parity_twice = d["results"][1].clone();
    parity_twice["id"] = json!("result:b1:parity-twice");
    let dense_w2 = vec![
        set(rb(json!(["ordinary_attempts", 0, "initial"])), json!({"kind": "structural_failure", "error": {"tag": "range", "detail": "b1"}, "diagnostic_ref": null})),
        set(rb(json!(["ordinary_attempts", 0, "w2"])), json!({"kind": "published", "trigger": {"tag": "evaluation", "error": {"tag": "range", "detail": "b1"}}, "force_scale_exponent": 3, "report_diagnostic_ref": d["retained_precision"]["body"]["ordinary_attempts"][0]["initial"]["report_diagnostic_ref"]})),
    ];
    let f = base_source(&shared, F_BASE);
    let cases = [
        ("dense b = 0, no parity row on either case", on_dense(vec![], vec![]), admitted(true)),
        ("dense b = 0, one parity row on the not_required case", on_dense(vec![one.clone()], vec![]), admitted(true)),
        ("dense, W2-published not_required case without a parity row", on_dense(vec![], w2.clone()), admitted(true)),
        // RV113 N-3: P4 reads W2 published, not W2 triggered.
        ("dense b = 0, one parity row on a not_required case whose W2 failed", on_dense(vec![one.clone()], w2_failed), admitted(true)),
        ("P2: two parity rows on the not_required case", on_dense(vec![one.clone(), two], vec![]), gate("G8", PREP)),
        ("P4: a parity row on the W2-published not_required case", on_dense(vec![one], w2), gate("G8", PREP)),
        ("P2: two parity rows on the dense selected case", b1_entry(&shared, "ordinary_prepared_dense_synthetic", None, vec![b1_rows(&d, vec![parity_twice])], vec![]), gate("G8", PREP)),
        ("P4: a parity row on a W2-published selected case", b1_entry(&shared, "ordinary_prepared_dense_synthetic", None, dense_w2, vec![]), gate("G8", PREP)),
        ("P3: a parity row on the sparse unavailable case", b1_entry(&shared, F_BASE, None, vec![b1_rows(&f, vec![b1_parity_row(&shared, UNAVAILABLE_ROW, "result:b1:sparse-parity")])], vec![]), gate("G8", PREP)),
    ];
    let mut misses = Vec::new();
    for (name, entry, want) in cases {
        let got = b1_verdict(&shared, &entry);
        if got != want {
            misses.push(format!("{name}: got {got} want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

// ---------------------------------------------------------------------------
// B1 SR-RS repair 02 (I90): the three-reader alignment set (RR "RV113's three returns
// verified; I3 made at `2ba2f81863`; the three-reader alignment set ruled", items 1-4).
// Reader-local tests; 07n pins the shared entries.

/// The reader's unbound verdict (no invocation): `{"admitted": eligible}`, or the first failure.
fn b1_unbound(shared: &Value, entry: &Value) -> Value {
    let (source, _) = apply_entry(shared, entry);
    match rp::validate(&source, None) {
        Ok(v) => serde_json::json!({"admitted": v.numerical_eligible}),
        Err(e) => gate(e.gate, &e.code),
    }
}
/// The transport verdict: `{"admitted": false}`, or the first failure.
fn b1_transport(shared: &Value, entry: &Value) -> Value {
    let (source, _) = apply_entry(shared, entry);
    match rp::validate_transport_metadata(&source) {
        Ok(v) => serde_json::json!({"admitted": v.numerical_eligible}),
        Err(e) => gate(e.gate, &e.code),
    }
}
fn b1_table(rows: Vec<(&str, Value, Value)>, verdict: impl Fn(&Value) -> Value) {
    let mut misses = Vec::new();
    for (name, entry, want) in rows {
        let got = verdict(&entry);
        if got != want {
            misses.push(format!("{name}: got {got} want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// Item 1, (f) and its family, at G3 COVERAGE bound and unbound: a material basis's and a
/// source's index is its position; `case_indices` is unique and in range; a source's owner
/// is its own case (the case at `owner.case_index` carries `owner.case_id`).
#[test]
fn b1_r2_f_family_at_g3_bound_and_unbound() {
    use serde_json::json;
    let shared = corpus();
    let rows = vec![
        (
            "a basis index not its position",
            b1_entry(
                &shared,
                ORD,
                None,
                vec![set(rb(json!(["material_bases", 0, "index"])), json!(1))],
                vec![],
            ),
        ),
        (
            "two basis indices swapped",
            b1_entry(
                &shared,
                "two_case_two_groups_synthetic",
                None,
                vec![
                    set(rb(json!(["material_bases", 0, "index"])), json!(1)),
                    set(rb(json!(["material_bases", 1, "index"])), json!(0)),
                ],
                vec![],
            ),
        ),
        (
            "a source index not its position",
            b1_entry(
                &shared,
                ORD,
                None,
                vec![set(rb(json!(["sources", 0, "index"])), json!(1))],
                vec![],
            ),
        ),
        (
            "two source indices swapped",
            b1_entry(
                &shared,
                "two_case_synthetic",
                None,
                vec![
                    set(rb(json!(["sources", 0, "index"])), json!(1)),
                    set(rb(json!(["sources", 1, "index"])), json!(0)),
                ],
                vec![],
            ),
        ),
        (
            "case_indices with a duplicate",
            b1_entry(
                &shared,
                "two_case_synthetic",
                None,
                vec![set(
                    rb(json!(["material_bases", 0, "case_indices"])),
                    json!([0, 1, 1]),
                )],
                vec![],
            ),
        ),
        (
            "case_indices out of range",
            b1_entry(
                &shared,
                "two_case_synthetic",
                None,
                vec![set(
                    rb(json!(["material_bases", 0, "case_indices"])),
                    json!([0, 1, 2]),
                )],
                vec![],
            ),
        ),
        (
            "a source owner naming another case's id",
            b1_entry(
                &shared,
                "two_case_synthetic",
                None,
                vec![set(
                    rb(json!(["sources", 1, "owner", "case_id"])),
                    json!("case:six-component-load"),
                )],
                vec![],
            ),
        ),
    ];
    let want = gate("G3", COVERAGE);
    let bound: Vec<_> = rows
        .iter()
        .map(|(n, e)| (*n, e.clone(), want.clone()))
        .collect();
    b1_table(bound, |e| b1_verdict(&shared, e));
    b1_table(
        rows.into_iter()
            .map(|(n, e)| (n, e, want.clone()))
            .collect(),
        |e| b1_unbound(&shared, e),
    );
}

/// Item 1: the ordinary attempt's basis reference at G5 ATTEMPT bound and unbound (it
/// resolves to a basis that lists its case). G8 still binds the basis to the invocation:
/// a basis listing its cases out of the invocation's order is G8 PREPARATION bound and
/// admitted unbound.
#[test]
fn b1_r2_ordinary_basis_reference_at_g5_attempt() {
    use serde_json::json;
    let shared = corpus();
    let missing = b1_entry(
        &shared,
        P_BASE,
        Some(NOT_REQUIRED),
        vec![set(
            rb(json!(["ordinary_attempts", 1, "material_basis_ref"])),
            json!(7),
        )],
        vec![],
    );
    let omits = b1_entry(
        &shared,
        P_BASE,
        Some(NOT_REQUIRED),
        vec![set(
            rb(json!(["material_bases", 0, "case_indices"])),
            json!([0]),
        )],
        vec![],
    );
    let reordered = b1_entry(
        &shared,
        P_BASE,
        Some(NOT_REQUIRED),
        vec![set(
            rb(json!(["material_bases", 0, "case_indices"])),
            json!([1, 0]),
        )],
        vec![],
    );
    b1_table(
        vec![
            ("the basis missing", missing.clone(), gate("G5", ATTEMPT)),
            (
                "the basis not listing the case",
                omits.clone(),
                gate("G5", ATTEMPT),
            ),
            (
                "the cases out of the invocation's order",
                reordered.clone(),
                gate("G8", PREP),
            ),
        ],
        |e| b1_verdict(&shared, e),
    );
    b1_table(
        vec![
            ("the basis missing", missing, gate("G5", ATTEMPT)),
            ("the basis not listing the case", omits, gate("G5", ATTEMPT)),
            (
                "the cases out of the invocation's order",
                reordered,
                admitted(false),
            ),
        ],
        |e| b1_unbound(&shared, e),
    );
}

/// Item 2, (g), at G8 INVOCATION, PP's acceptance: no `reference_configurations` member
/// (null included); `pressure_contract` absent or null; `components` absent or [];
/// `combinations` absent or an array whose entries the receipt's equal (B2-C §10.1 G8).
/// Unbound reads do not see the invocation.
#[test]
fn b1_r2_g_model_scope_at_g8_invocation() {
    use serde_json::json;
    let shared = corpus();
    let model = |k: &str| json!(["request", "model", k]);
    let invocation = |edits: Vec<Value>| b1_entry(&shared, ORD, None, vec![], edits);
    let refused = gate("G8", "RETAINED_PRECISION_INVOCATION_MISMATCH");
    b1_table(
        vec![
            (
                "combinations null",
                invocation(vec![set(model("combinations"), Value::Null)]),
                refused.clone(),
            ),
            (
                "combinations an object",
                invocation(vec![set(model("combinations"), json!({"x": 1}))]),
                refused.clone(),
            ),
            (
                "combinations a string",
                invocation(vec![set(model("combinations"), json!("x"))]),
                refused.clone(),
            ),
            (
                "components null",
                invocation(vec![set(model("components"), Value::Null)]),
                refused.clone(),
            ),
            (
                "components a string",
                invocation(vec![set(model("components"), json!("x"))]),
                refused.clone(),
            ),
            (
                "components an object",
                invocation(vec![set(model("components"), json!({}))]),
                refused.clone(),
            ),
            (
                "reference_configurations null",
                invocation(vec![set(model("reference_configurations"), Value::Null)]),
                refused.clone(),
            ),
            (
                "pressure_contract false",
                invocation(vec![set(model("pressure_contract"), json!(false))]),
                refused.clone(),
            ),
            // U3: the retired legacy pressure label is refused here, at any schema.
            (
                "pressure_contract 1.0.0/legacy_pressure_v1 (retired)",
                invocation(vec![set(
                    model("pressure_contract"),
                    json!({"version": "1.0.0", "mode": "legacy_pressure_v1"}),
                )]),
                refused.clone(),
            ),
            (
                "schema 0.3.0 with pressure_contract 1.0.0/legacy_pressure_v1 (retired)",
                invocation(vec![
                    set(model("schema_version"), json!("0.3.0")),
                    set(
                        model("pressure_contract"),
                        json!({"version": "1.0.0", "mode": "legacy_pressure_v1"}),
                    ),
                ]),
                refused.clone(),
            ),
            (
                "combinations and components []",
                invocation(vec![
                    set(model("combinations"), json!([])),
                    set(model("components"), json!([])),
                ]),
                admitted(true),
            ),
            (
                "combinations and components absent",
                invocation(vec![
                    remove(model("combinations")),
                    remove(model("components")),
                ]),
                admitted(true),
            ),
            (
                "pressure_contract null",
                invocation(vec![set(model("pressure_contract"), Value::Null)]),
                admitted(true),
            ),
        ],
        |e| b1_verdict(&shared, e),
    );
    assert_eq!(
        b1_unbound(
            &shared,
            &invocation(vec![set(model("combinations"), Value::Null)])
        ),
        admitted(false)
    );
}

/// Item 3: C2's cause table at G5 ATTEMPT, in the ordinary class, for an unavailable case
/// whose cause is not a prepared product failure. On 07j's two-case base, case 1 with a
/// Ready attempt and its selected Run (rows without a method), by `validate`:
/// `receipt_failure` and `facade_failure`, each satisfied and broken (a satisfied facade
/// failure beside a Ready attempt then fails D19, G5 PRODUCT_ATTEMPT).
#[test]
fn b1_r2_c2_cause_table_receipt_and_facade() {
    use serde_json::json;
    let shared = corpus();
    let base = base_source(&shared, "two_case_synthetic");
    let mut rows = base["results"].as_array().unwrap().clone();
    for r in rows
        .iter_mut()
        .filter(|r| r["basis_ref"]["ref_id"] == "case:zero-load")
    {
        r.as_object_mut().unwrap().remove("recovery_method");
    }
    let diagnostic = base["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .position(|d| d["id"] == "diagnostic:retained:synthetic-zero-load")
        .unwrap();
    let case = |reason: Value| {
        b1_entry(
            &shared,
            "two_case_synthetic",
            None,
            vec![
                remove(rb(json!(["cases", 1, "method"]))),
                remove(rb(json!(["cases", 1, "selection"]))),
                remove(rb(json!(["cases", 1, "source_identity_sha256"]))),
                set(rb(json!(["cases", 1, "status"])), json!("unavailable")),
                set(rb(json!(["cases", 1, "reason"])), reason),
                set(
                    rb(json!(["cases", 1, "diagnostic_ref"])),
                    json!("diagnostic:retained:synthetic-zero-load"),
                ),
                set(
                    json!(["diagnostics", diagnostic, "code"]),
                    json!("RETAINED_PRECISION_UNAVAILABLE"),
                ),
                set(json!(["results"]), Value::Array(rows.clone())),
            ],
            vec![],
        )
    };
    let receipt = json!({"kind": "receipt_failure", "check": "encoding", "field_path": "retained_precision.body"});
    let facade = |index: u64| json!({"kind": "facade_failure", "owner_ref": {"kind": "case", "index": index}, "row_id": null, "recipe": "identity", "operand_index": null, "check": "identity", "predicate": null});
    let reason = |code: &str, phase: &str, cause: &Value| json!({"code": code, "phase": phase, "cause": cause});
    b1_table(
        vec![
            (
                "receipt_failure: receipt_encoding, receipt",
                case(reason("receipt_encoding", "receipt", &receipt)),
                admitted(false),
            ),
            (
                "receipt_failure: publication_hash_range, receipt",
                case(reason("publication_hash_range", "receipt", &receipt)),
                admitted(false),
            ),
            (
                "receipt_failure: invocation_not_representable, receipt",
                case(reason("invocation_not_representable", "receipt", &receipt)),
                admitted(false),
            ),
            (
                "receipt_failure: phase kernel",
                case(reason("kernel_unresolved", "kernel", &receipt)),
                gate("G5", ATTEMPT),
            ),
            (
                "receipt_failure: code facade_certificate",
                case(reason("facade_certificate", "receipt", &receipt)),
                gate("G5", ATTEMPT),
            ),
            (
                "receipt_failure: phase preparation",
                case(reason("source_unavailable", "preparation", &receipt)),
                gate("G5", ATTEMPT),
            ),
            (
                "facade_failure: satisfied (D19 next)",
                case(reason("facade_certificate", "facade", &facade(1))),
                gate("G5", PRODUCT),
            ),
            (
                "facade_failure: phase kernel",
                case(reason("facade_certificate", "kernel", &facade(1))),
                gate("G5", ATTEMPT),
            ),
            (
                "facade_failure: code receipt_encoding",
                case(reason("receipt_encoding", "facade", &facade(1))),
                gate("G5", ATTEMPT),
            ),
            (
                "facade_failure: another case's owner",
                case(reason("facade_certificate", "facade", &facade(0))),
                gate("G5", ATTEMPT),
            ),
        ],
        |e| b1_verdict(&shared, e),
    );
}

/// Item 3 on G5's ordinary class (`reader_logic::ordinary`, the pass `validate` runs):
/// `source_error`, `unavailable_precondition` keyed by its precondition, and a kernel
/// reason, each satisfied and broken. 07j's base case 1 (unavailable, no source or Run)
/// carries the first two; the kernel reason uses F_BASE's case 1 with its own refused Run
/// (the (4a) witness's idle Run).
#[test]
fn b1_r2_c2_cause_table_source_precondition_kernel() {
    use serde_json::json;
    let shared = corpus();
    let ordinary = |source: &Value| match rp::reader_logic::ordinary(source) {
        Ok(()) => admitted(false),
        Err(e) => gate(e.gate, &e.code),
    };
    let p = base_source(&shared, P_BASE);
    let with_reason = |reason: Value, decline: Option<Value>| {
        let mut s = p.clone();
        s["retained_precision"]["body"]["cases"][1]["reason"] = reason;
        if let Some(d) = decline {
            s["retained_precision"]["body"]["cases"][1]["source_decline"] = d;
        }
        s
    };
    let error = json!({"tag": "no_nodes"});
    let decline = |e: &Value| {
        json!({
            "input_owner": {"case_index": 1, "case_id": "case:unavailable-row", "material_basis_ref": 0},
            "constructor_counts": {"nodes": 2, "members": 1, "springs": 0, "constraints": 6, "nodal_terms": 6, "stations": 3, "supports": 1, "id_utf8_bytes": 0, "directional_springs": 0},
            "error": e
        })
    };
    let source_error = json!({"kind": "source_error", "error": error});
    let precondition = |which: &str| json!({"kind": "unavailable_precondition", "precondition": which, "affected_refs": []});
    let reason = |code: &str, phase: &str, cause: Value| json!({"code": code, "phase": phase, "cause": cause});
    let mut rows = vec![
        (
            "source_error: satisfied",
            with_reason(
                reason("source_unavailable", "preparation", source_error.clone()),
                Some(decline(&error)),
            ),
            admitted(false),
        ),
        (
            "source_error: no source_decline",
            with_reason(
                reason("source_unavailable", "preparation", source_error.clone()),
                None,
            ),
            gate("G5", ATTEMPT),
        ),
        (
            "source_error: the decline's error differs",
            with_reason(
                reason("source_unavailable", "preparation", source_error.clone()),
                Some(decline(&json!({"tag": "no_members"}))),
            ),
            gate("G5", ATTEMPT),
        ),
        (
            "source_error: phase routing",
            with_reason(
                reason("source_unavailable", "routing", source_error.clone()),
                Some(decline(&error)),
            ),
            gate("G5", ATTEMPT),
        ),
        (
            "source_error: code caller_not_qualified",
            with_reason(
                reason("caller_not_qualified", "preparation", source_error),
                Some(decline(&error)),
            ),
            gate("G5", ATTEMPT),
        ),
        (
            "unavailable_precondition: phase kernel",
            with_reason(
                reason("source_unavailable", "kernel", precondition("capture")),
                None,
            ),
            gate("G5", ATTEMPT),
        ),
    ];
    let keyed = [
        ("caller", "caller_not_qualified"),
        ("resource_admission", "resource_admission_not_available"),
        ("upstream_no_wrap", "upstream_no_wrap_not_established"),
        ("capture", "source_unavailable"),
        ("source_family", "source_unavailable"),
    ];
    for (which, code) in keyed {
        for phase in ["routing", "preparation"] {
            rows.push((
                "unavailable_precondition: its keyed code",
                with_reason(reason(code, phase, precondition(which)), None),
                admitted(false),
            ));
        }
        for (_, other) in keyed.iter().filter(|(_, c)| *c != code) {
            rows.push((
                "unavailable_precondition: another precondition's code",
                with_reason(reason(other, "preparation", precondition(which)), None),
                gate("G5", ATTEMPT),
            ));
        }
    }
    // A kernel reason: F_BASE's case 1 with its own refused (idle) Run.
    let body = &base_source(&shared, F_BASE)["retained_precision"]["body"];
    let mut run = body["cases"][1]["run"].clone();
    let before = run["invocation_before"].clone();
    for (k, v) in [
        ("records", json!([])),
        ("attempts", json!([])),
        ("case_charge", json!(0)),
        ("invocation_increment", json!(0)),
        ("invocation_after", before.clone()),
    ] {
        run[k] = v;
    }
    run["cache_after"] = run["cache_before"].clone();
    let terminal = json!({"space": "refusal", "tag": "ledger_unavailable", "error": {"tag": "accumulator", "error": {"tag": "non_finite"}}});
    run["kernel_terminal"] = json!({"kind": "refused", "reason": terminal});
    let kernel = |code: &str, phase: &str, cause: Value| {
        let (source, _) = apply_entry(
            &shared,
            &b1_entry(
                &shared,
                F_BASE,
                None,
                vec![
                    set(rb(json!(["cases", 1, "run"])), run.clone()),
                    set(
                        rb(json!(["cases", 1, "reason"])),
                        reason(code, phase, cause),
                    ),
                    set(rb(json!(["calls", 0, "invocation_after"])), before.clone()),
                    set(rb(json!(["work", "charged"])), before.clone()),
                ],
                vec![],
            ),
        );
        source
    };
    rows.extend([
        (
            "kernel reason: satisfied",
            kernel("kernel_refused", "kernel", terminal.clone()),
            admitted(false),
        ),
        (
            "kernel reason: code kernel_unresolved",
            kernel("kernel_unresolved", "kernel", terminal.clone()),
            gate("G5", ATTEMPT),
        ),
        (
            "kernel reason: phase facade",
            kernel("kernel_refused", "facade", terminal.clone()),
            gate("G5", ATTEMPT),
        ),
        (
            "kernel reason: a cause other than the terminal's",
            kernel(
                "kernel_refused",
                "kernel",
                json!({"space": "refusal", "tag": "structure"}),
            ),
            gate("G5", ATTEMPT),
        ),
    ]);
    let mut misses = Vec::new();
    for (name, source, want) in rows {
        let got = ordinary(&source);
        if got != want {
            misses.push(format!(
                "{name}: got {got} want {want} ({})",
                source["retained_precision"]["body"]["cases"][1]["reason"]
            ));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// Item 4: transport keeps the header check at G2 and adds the preview-physics metadata
/// check at G7 (`SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`), as PY and TS run it. Each edit
/// below passes the header and is refused by the metadata check; the header's own
/// defects stay at G2.
#[test]
fn b1_r2_transport_metadata_at_g7() {
    use serde_json::json;
    let shared = corpus();
    let ev = |tail: Value| {
        let mut p = vec![json!("contract_evidence")];
        p.extend(tail.as_array().unwrap().iter().cloned());
        Value::Array(p)
    };
    let x = |k: &str| ev(json!(["preview_cases", 0, "pipe_stress_extrema", 0, k]));
    let on = |base: &str, edits: Vec<Value>| b1_entry(&shared, base, None, edits, vec![]);
    let g7 = gate("G7", "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID");
    let limitations = base_source(&shared, ORD)["formulation_basis"]["limitations"].clone();
    let mut shorter = limitations.as_array().unwrap().clone();
    shorter.pop();
    b1_table(
        vec![
            ("the base", on(ORD, vec![]), admitted(false)),
            (
                "a valid combination gate",
                on(
                    ORD,
                    vec![set(
                        ev(json!(["combination_gates"])),
                        json!([{"combination_id": "c", "withheld": false, "reason": null}]),
                    )],
                ),
                admitted(false),
            ),
            (
                "carrier_evidence present",
                on(ORD, vec![set(json!(["carrier_evidence"]), json!({}))]),
                g7.clone(),
            ),
            (
                "limitations other than the table's",
                on(
                    ORD,
                    vec![set(
                        json!(["formulation_basis", "limitations"]),
                        Value::Array(shorter),
                    )],
                ),
                g7.clone(),
            ),
            (
                "contract_evidence with another member",
                on(ORD, vec![set(ev(json!(["extra"])), json!([]))]),
                g7.clone(),
            ),
            (
                "preview_cases not a list",
                on(ORD, vec![set(ev(json!(["preview_cases"])), json!({}))]),
                g7.clone(),
            ),
            (
                "a preview case missing a member",
                on(
                    ORD,
                    vec![remove(ev(json!([
                        "preview_cases",
                        0,
                        "intensified_measures"
                    ])))],
                ),
                g7.clone(),
            ),
            (
                "two preview cases with one id",
                on(
                    "two_case_synthetic",
                    vec![set(
                        ev(json!(["preview_cases", 1, "load_case_id"])),
                        json!("case:six-component-load"),
                    )],
                ),
                g7.clone(),
            ),
            (
                "coverage complete with an unavailable pipe",
                on(
                    ORD,
                    vec![set(
                        ev(json!([
                            "preview_cases",
                            0,
                            "stress_maximum_coverage",
                            "unavailable_pipe_ids"
                        ])),
                        json!(["pipe:x"]),
                    )],
                ),
                g7.clone(),
            ),
            (
                "an extrema constant changed",
                on(ORD, vec![set(x("approximation"), json!("other"))]),
                g7.clone(),
            ),
            (
                "an extrema fraction above 1",
                on(ORD, vec![set(x("station_fraction"), json!(2.0))]),
                g7.clone(),
            ),
            (
                "an extrema span index negative",
                on(ORD, vec![set(x("span_index"), json!(-1))]),
                g7.clone(),
            ),
            (
                "subdivisions above the maximum",
                on(ORD, vec![set(x("subdivisions"), json!(131073))]),
                g7.clone(),
            ),
            (
                "extrema bounds inverted",
                on(ORD, vec![set(x("value_lower_pa"), json!(41354910.5))]),
                g7.clone(),
            ),
            (
                "a withheld record with an unknown reason",
                on(
                    ORD,
                    vec![set(
                        ev(json!([
                            "preview_cases",
                            0,
                            "support_attribution",
                            "withheld"
                        ])),
                        json!([{"support_id": "support:other", "reason": "OTHER"}]),
                    )],
                ),
                g7.clone(),
            ),
            (
                "a support attributed and withheld",
                on(
                    ORD,
                    vec![set(
                        ev(json!([
                            "preview_cases",
                            0,
                            "support_attribution",
                            "withheld"
                        ])),
                        json!([{"support_id": "support:fixture-root", "reason": "CONSTANT_EFFORT_NOT_CONSUMED"}]),
                    )],
                ),
                g7.clone(),
            ),
            (
                "attribution sets differ between cases",
                on(
                    "two_case_synthetic",
                    vec![set(
                        ev(json!([
                            "preview_cases",
                            1,
                            "support_attribution",
                            "attributed_support_ids"
                        ])),
                        json!([]),
                    )],
                ),
                g7.clone(),
            ),
            (
                "an intensified measure in another place",
                on(
                    ORD,
                    vec![set(
                        ev(json!(["preview_cases", 0, "intensified_measures"])),
                        json!([{"result_id": "r", "component_id": "k", "pipe_id": "p", "location": "midspan", "factor_role": "bend", "sif": 1.0, "sif_source_reference": "s", "section_modulus_m3": 1.0, "bending_moment_y_n_m": 0.0, "bending_moment_z_n_m": 0.0}]),
                    )],
                ),
                g7.clone(),
            ),
            (
                "a withheld gate without a code",
                on(
                    ORD,
                    vec![set(
                        ev(json!(["combination_gates"])),
                        json!([{"combination_id": "c", "withheld": true, "reason": null}]),
                    )],
                ),
                g7.clone(),
            ),
            (
                "two gates with one id",
                on(
                    ORD,
                    vec![set(
                        ev(json!(["combination_gates"])),
                        json!([{"combination_id": "c", "withheld": false, "reason": null}, {"combination_id": "c", "withheld": false, "reason": null}]),
                    )],
                ),
                g7.clone(),
            ),
            (
                "the header's own defect stays at G2",
                on(
                    ORD,
                    vec![set(json!(["numerical_quality", "status"]), json!("other"))],
                ),
                gate("G2", "SOURCE_NUMERICAL_QUALITY_INVALID"),
            ),
        ],
        |e| b1_transport(&shared, e),
    );
}

// ---------------------------------------------------------------------------
// B1's reader follow-up toward I4' (I101; RR "I4 made at `30f3d1b24a`; …", rulings 2
// and 4): reader-local rows on synthetic receipts. SC's 07n pins the shared ones.

/// The transport verdict with the refusal's detail: `{"admitted": false}`, or the first
/// failure's gate, code and detail.
fn b1_transport_detail(shared: &Value, entry: &Value) -> Value {
    let (source, _) = apply_entry(shared, entry);
    match rp::validate_transport_metadata(&source) {
        Ok(v) => serde_json::json!({"admitted": v.numerical_eligible}),
        Err(e) => serde_json::json!({"gate": e.gate, "code": e.code, "detail": e.detail}),
    }
}
/// The metadata check's refusal of one demand, as `b1_transport_detail` reports it.
fn b1_metadata_refusal(demand: &str) -> Value {
    serde_json::json!({
        "gate": "G7",
        "code": "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID",
        "detail": format!("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: {demand}"),
    })
}

/// Ruling 2, RS's side: PY's extrema-number demand in the transport metadata check. An
/// extremum whose `global_upper_bound_pa` or `certified_gap_pa` is not a JSON number
/// (null included) is refused at G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, "extrema
/// numbers", at PY's place (before the fractions); a JSON integer is a number. The raw
/// path is unchanged: RS's base reader refuses RV113's two shapes at G7 with its own raw
/// code, `SOURCE_PREVIEW_PHYSICS_NUMBER_INVALID` (a declared per-reader raw code).
#[test]
fn b1_i4p_transport_extrema_numbers_at_g7() {
    use serde_json::json;
    let shared = corpus();
    let x = |k: &str| {
        json!([
            "contract_evidence",
            "preview_cases",
            0,
            "pipe_stress_extrema",
            0,
            k
        ])
    };
    let on = |edits: Vec<Value>| b1_entry(&shared, ORD, None, edits, vec![]);
    let shapes = [
        (
            "global_upper_bound_pa a string",
            "global_upper_bound_pa",
            json!("x"),
        ),
        (
            "global_upper_bound_pa null",
            "global_upper_bound_pa",
            Value::Null,
        ),
        (
            "global_upper_bound_pa a boolean",
            "global_upper_bound_pa",
            json!(true),
        ),
        ("certified_gap_pa null", "certified_gap_pa", Value::Null),
        ("certified_gap_pa a string", "certified_gap_pa", json!("0")),
        ("certified_gap_pa a list", "certified_gap_pa", json!([0.0])),
    ];
    let mut rows: Vec<(&str, Value, Value)> = shapes
        .iter()
        .map(|(name, k, v)| {
            (
                *name,
                on(vec![set(x(k), v.clone())]),
                b1_metadata_refusal("extrema numbers"),
            )
        })
        .collect();
    rows.extend([
        ("the base", on(vec![]), admitted(false)),
        (
            "both members JSON integers",
            on(vec![
                set(x("global_upper_bound_pa"), json!(41354909)),
                set(x("certified_gap_pa"), json!(0)),
            ]),
            admitted(false),
        ),
        (
            "a string bound beside a fraction above 1 (the demand comes first)",
            on(vec![
                set(x("global_upper_bound_pa"), json!("x")),
                set(x("station_fraction"), json!(2.0)),
            ]),
            b1_metadata_refusal("extrema numbers"),
        ),
        (
            "a fraction above 1 alone",
            on(vec![set(x("station_fraction"), json!(2.0))]),
            b1_metadata_refusal("extrema fractions"),
        ),
    ]);
    b1_table(rows, |e| b1_transport_detail(&shared, e));
    let raw = gate("G7", "SOURCE_PREVIEW_PHYSICS_NUMBER_INVALID");
    let rv113 = [
        (
            "t_extrema_global_upper_string",
            on(vec![set(x("global_upper_bound_pa"), json!("x"))]),
        ),
        (
            "t_extrema_certified_gap_null",
            on(vec![set(x("certified_gap_pa"), Value::Null)]),
        ),
    ];
    b1_table(
        rv113
            .iter()
            .map(|(n, e)| (*n, e.clone(), raw.clone()))
            .collect(),
        |e| b1_verdict(&shared, e),
    );
    b1_table(
        rv113
            .iter()
            .map(|(n, e)| (*n, e.clone(), raw.clone()))
            .collect(),
        |e| b1_unbound(&shared, e),
    );
}

/// Ruling 4 (RV113's SR-RS addendum 02, S-1), C2's three conjuncts that no RS test broke
/// alone, by `validate`, bound and unbound, each at G5 ATTEMPT:
/// - `unavailable_precondition` with its keyed code in an admitted phase, beside the case's
///   selected Run (the no-Run conjunct; RV113's `ca_precondition_beside_run`);
/// - `receipt_failure` with a receipt code, `receipt_encoding`, in phase kernel (the phase
///   conjunct; RV113's `ca_receipt_phase_kernel`);
/// - a `facade_failure` with its phase, code and owner, on a case with no Run (the
///   selected-Run conjunct; RV113's `cb_facade_no_run`).
#[test]
fn b1_i4p_c2_conjuncts_alone() {
    use serde_json::json;
    let shared = corpus();
    // C-a: 07j's two-case base, case 1 unavailable beside its Ready attempt and selected
    // Run (rows without a method), as `b1_r2_c2_cause_table_receipt_and_facade` builds it.
    let base = base_source(&shared, "two_case_synthetic");
    let mut rows = base["results"].as_array().unwrap().clone();
    for r in rows
        .iter_mut()
        .filter(|r| r["basis_ref"]["ref_id"] == "case:zero-load")
    {
        r.as_object_mut().unwrap().remove("recovery_method");
    }
    let diagnostic = base["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .position(|d| d["id"] == "diagnostic:retained:synthetic-zero-load")
        .unwrap();
    let beside_run = |reason: Value| {
        b1_entry(
            &shared,
            "two_case_synthetic",
            None,
            vec![
                remove(rb(json!(["cases", 1, "method"]))),
                remove(rb(json!(["cases", 1, "selection"]))),
                remove(rb(json!(["cases", 1, "source_identity_sha256"]))),
                set(rb(json!(["cases", 1, "status"])), json!("unavailable")),
                set(rb(json!(["cases", 1, "reason"])), reason),
                set(
                    rb(json!(["cases", 1, "diagnostic_ref"])),
                    json!("diagnostic:retained:synthetic-zero-load"),
                ),
                set(
                    json!(["diagnostics", diagnostic, "code"]),
                    json!("RETAINED_PRECISION_UNAVAILABLE"),
                ),
                set(json!(["results"]), Value::Array(rows.clone())),
            ],
            vec![],
        )
    };
    // C-b: the preparation-failure base, case 1 unavailable with no product attempt,
    // source or Run.
    let p1 = base_source(&shared, P_BASE)["retained_precision"]["body"]["cases"][1].clone();
    let no_run = |reason: Value| {
        b1_entry(
            &shared,
            P_BASE,
            None,
            vec![
                set(
                    rb(json!(["cases", 1])),
                    json!({
                        "basis_ref": p1["basis_ref"], "ordinary": p1["ordinary"],
                        "product_attempt_ref": null, "status": "unavailable", "reason": reason,
                        "diagnostic_ref": p1["diagnostic_ref"], "run": null, "source_ref": null
                    }),
                ),
                remove(rb(json!(["product_attempts", 1]))),
            ],
            vec![],
        )
    };
    let reason = |code: &str, phase: &str, cause: Value| json!({"code": code, "phase": phase, "cause": cause});
    let receipt = json!({"kind": "receipt_failure", "check": "encoding", "field_path": "retained_precision.body"});
    let facade = json!({"kind": "facade_failure", "owner_ref": {"kind": "case", "index": 1}, "row_id": null, "recipe": "identity", "operand_index": null, "check": "identity", "predicate": null});
    let capture =
        json!({"kind": "unavailable_precondition", "precondition": "capture", "affected_refs": []});
    let entries = vec![
        (
            "unavailable_precondition, keyed, phase preparation, beside the selected Run",
            beside_run(reason("source_unavailable", "preparation", capture)),
        ),
        (
            "receipt_failure, receipt_encoding, phase kernel",
            beside_run(reason("receipt_encoding", "kernel", receipt)),
        ),
        (
            "facade_failure, facade_certificate, phase facade, naming its case, with no Run",
            no_run(reason("facade_certificate", "facade", facade)),
        ),
    ];
    let want = gate("G5", ATTEMPT);
    b1_table(
        entries
            .iter()
            .map(|(n, e)| (*n, e.clone(), want.clone()))
            .collect(),
        |e| b1_verdict(&shared, e),
    );
    b1_table(
        entries
            .into_iter()
            .map(|(n, e)| (n, e, want.clone()))
            .collect(),
        |e| b1_unbound(&shared, e),
    );
}

/// Ruling 4 (RV113's SR-RS addendum 02, S-1), the transport metadata check's nine
/// demands that no RS test broke alone (RV113's N45, N47, N48, N56, N58, N60 and
/// N63-N65). Each row breaks one demand and is refused by it (gate, code and detail);
/// the two controls carry a valid measure and a valid withheld gate.
#[test]
fn b1_i4p_transport_metadata_demands_alone() {
    use serde_json::json;
    let shared = corpus();
    let pc = |tail: Value| {
        let mut p = vec![json!("contract_evidence"), json!("preview_cases"), json!(0)];
        p.extend(tail.as_array().unwrap().iter().cloned());
        Value::Array(p)
    };
    let on = |edits: Vec<Value>| b1_entry(&shared, ORD, None, edits, vec![]);
    let measure = json!({
        "result_id": "result:i101:measure", "component_id": "component:i101", "pipe_id": "pipe:fixture-span",
        "location": "end_i", "factor_role": "bend", "sif": 1.5, "sif_source_reference": "i101",
        "section_modulus_m3": 1e-4, "bending_moment_y_n_m": 1.0, "bending_moment_z_n_m": 2.0
    });
    let with = |v: &Value, k: &str, x: Value| {
        let mut v = v.clone();
        v[k] = x;
        v
    };
    let measures = |list: Value| on(vec![set(pc(json!(["intensified_measures"])), list)]);
    let coverage = |unavailable: Value, outside: Value| {
        on(vec![set(
            pc(json!(["stress_maximum_coverage"])),
            json!({"complete": false, "unavailable_pipe_ids": unavailable, "outside_domain_pipe_ids": outside}),
        )])
    };
    let gate_record = json!({"combination_id": "combination:i101", "withheld": true, "reason": "NONLINEAR_COMBINATION_REQUIRES_SOLVE"});
    let gates = |list: Value| {
        on(vec![set(
            json!(["contract_evidence", "combination_gates"]),
            list,
        )])
    };
    let m = b1_metadata_refusal;
    b1_table(
        vec![
            (
                "a valid measure",
                measures(json!([measure.clone()])),
                admitted(false),
            ),
            (
                "a valid withheld gate",
                gates(json!([gate_record.clone()])),
                admitted(false),
            ),
            (
                "N45: a preview case with another member",
                on(vec![set(pc(json!(["extra"])), json!(1))]),
                m("preview case shape"),
            ),
            (
                "N47: an unavailable pipe listed twice",
                coverage(json!(["p", "p"]), json!([])),
                m("maximum coverage values"),
            ),
            (
                "N48: a pipe both unavailable and outside the domain",
                coverage(json!(["p"]), json!(["p"])),
                m("maximum coverage overlap"),
            ),
            (
                "N56: an extremum's pipe listed unavailable",
                coverage(json!(["pipe:fixture-span"]), json!([])),
                m("extrema member partition"),
            ),
            (
                "N58: a measure's moment a string",
                measures(json!([with(&measure, "bending_moment_y_n_m", json!("1"))])),
                m("intensified measure inputs"),
            ),
            (
                "N60: two measures with one result id",
                measures(json!([
                    measure.clone(),
                    with(&measure, "component_id", json!("component:i101b"))
                ])),
                m("duplicate evidence result binding"),
            ),
            (
                "N63: an extremum with another member",
                on(vec![set(
                    pc(json!(["pipe_stress_extrema", 0, "extra"])),
                    json!(1),
                )]),
                m("extrema shape"),
            ),
            (
                "N64: a measure with another member",
                measures(json!([with(&measure, "extra", json!(1))])),
                m("intensified measure shape"),
            ),
            (
                "N65: a gate with another member",
                gates(json!([with(&gate_record, "extra", json!(1))])),
                m("combination gate shape"),
            ),
        ],
        |e| b1_transport_detail(&shared, e),
    );
}

/// B3a dropped (RR "Owner decisions: the legacy pressure contract is retired
/// product-wide; ..."; formerly B3-D §6.3 and §6.4; REVISION_01 §3, N-4): G8's
/// namespace predicate is type-strict. Only branch L (0.1.0 or 0.2.0,
/// `pressure_contract` absent or JSON null) is admitted; any other value is G8
/// INVOCATION_MISMATCH. "L3" in the entry names is B3a's retired contract
/// `{"version":"1.0.0","mode":"legacy_pressure_v1"}` on 0.3.0, now refused on
/// every base, and before any load is read (the former N-11 entries). The
/// table is the one shape RS, TS and PY pin alike, entry for entry.
/// Reader-local synthetic receipts: invocation edits on the shared bases,
/// rehashed.
#[test]
fn b3a_legacy_pressure_contract_namespace_at_g8() {
    use serde_json::json;
    let shared = corpus();
    let l3 = json!({"version": "1.0.0", "mode": "legacy_pressure_v1"});
    let schema = |v: &str| set(json!(["request", "model", "schema_version"]), json!(v));
    let contract = |v: Value| set(json!(["request", "model", "pressure_contract"]), v);
    let no_contract = || remove(json!(["request", "model", "pressure_contract"]));
    let inv = gate("G8", "RETAINED_PRECISION_INVOCATION_MISMATCH");
    let base_invocation = |id: &str| {
        shared["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == id)
            .unwrap()["invocation"]
            .clone()
    };
    // N-11: a zero-magnitude element pressure load appended to case 0.
    let zero_pressure = |id: &str| {
        let mut loads = base_invocation(id)["request"]["model"]["load_cases"][0]["primitive_loads"].clone();
        loads.as_array_mut().unwrap().push(json!({
            "id": "load:b3a-zero-pressure", "category": "pressure",
            "target": {"type": "element", "pipe": "pipe:fixture-span"},
            "magnitude": {"value": 0, "unit": "Pa"}, "dimension": "pressure",
            "provenance": "synthetic_integration_input_not_library_data"}));
        set(json!(["request", "model", "load_cases", 0, "primitive_loads"]), loads)
    };
    let with = |key: &str, value: Value| {
        let mut c = l3.clone();
        c[key] = value;
        c
    };
    let mut entries: Vec<(String, &str, Vec<Value>, Value)> = Vec::new();
    for base in [
        ORD,
        "ordinary_prepared_dense_synthetic",
        "two_case_synthetic",
        "u8_l0_isolated_node_sparse_interactive",
        "u8_l0_isolated_node_dense_scrutiny",
    ] {
        entries.push((format!("L3 on {base}"), base, vec![schema("0.3.0"), contract(l3.clone())], inv.clone()));
    }
    for (name, edits, want) in [
        ("L: 0.2.0, contract null", vec![contract(Value::Null)], Value::Null),
        ("L: 0.1.0, contract absent", vec![schema("0.1.0"), no_contract()], Value::Null),
        ("L3: mode exact_straight_pressure_v2, version 1.0.0", vec![schema("0.3.0"), contract(with("mode", json!("exact_straight_pressure_v2")))], inv.clone()),
        ("L3: version 1.0.1", vec![schema("0.3.0"), contract(with("version", json!("1.0.1")))], inv.clone()),
        ("L3: the exact contract 2.0.0", vec![schema("0.3.0"), contract(json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"}))], inv.clone()),
        ("0.3.0, contract null", vec![schema("0.3.0"), contract(Value::Null)], inv.clone()),
        ("0.3.0, contract absent", vec![schema("0.3.0"), no_contract()], inv.clone()),
        ("L3: an extra key", vec![schema("0.3.0"), contract(with("extra", json!("x")))], inv.clone()),
        ("L3: an extra key valued null", vec![schema("0.3.0"), contract(with("extra", Value::Null))], inv.clone()),
        ("L3: version only", vec![schema("0.3.0"), contract(json!({"version": "1.0.0"}))], inv.clone()),
        ("L3: mode only", vec![schema("0.3.0"), contract(json!({"mode": "legacy_pressure_v1"}))], inv.clone()),
        ("L3: version a number", vec![schema("0.3.0"), contract(with("version", json!(1.0)))], inv.clone()),
        ("L3: mode null", vec![schema("0.3.0"), contract(with("mode", Value::Null))], inv.clone()),
        ("0.3.0, contract {}", vec![schema("0.3.0"), contract(json!({}))], inv.clone()),
        ("0.2.0 keeping the L3 contract", vec![contract(l3.clone())], inv.clone()),
        ("0.1.0 keeping the L3 contract", vec![schema("0.1.0"), contract(l3.clone())], inv.clone()),
        ("0.4.0 with the L3 contract", vec![schema("0.4.0"), contract(l3.clone())], inv.clone()),
        ("N-4: 0.2.0, contract {}", vec![contract(json!({}))], inv.clone()),
        ("N-4: 0.2.0, contract false", vec![contract(json!(false))], inv.clone()),
        ("N-4: 0.2.0, contract []", vec![contract(json!([]))], inv.clone()),
        ("N-4: 0.2.0, contract \"\"", vec![contract(json!(""))], inv.clone()),
        ("N-4: 0.2.0, contract 0", vec![contract(json!(0))], inv.clone()),
        ("N-11: L3 with a zero-magnitude element pressure load", vec![schema("0.3.0"), contract(l3.clone()), zero_pressure(ORD)], inv.clone()),
    ] {
        entries.push((name.to_string(), ORD, edits, want));
    }
    entries.push((
        "N-11: L3 dense with a zero-magnitude element pressure load".into(),
        "ordinary_prepared_dense_synthetic",
        vec![schema("0.3.0"), contract(l3.clone()), zero_pressure("ordinary_prepared_dense_synthetic")],
        inv.clone(),
    ));
    let mut misses = Vec::new();
    for (name, base, edits, want) in &entries {
        let got = observe(
            &shared,
            &json!({"id": "i101_b3a", "base": base, "edits": [], "rehash": "all", "invocation_edits": edits}),
        );
        if got != *want {
            misses.push(format!("{name}: got {got}, want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
    assert_eq!(entries.len(), 29);
}

// ---- B3b (I101): the exact successor `<physics-retained>` on reader-local synthetic receipts ----
/// RN64(2e11 / (2 RN64(1 + nu))) is exactly the bases' G = 7.7e10 Pa, so the
/// synthetic exact successor keeps every receipt number of its preview base.
const EXACT_NU: f64 = 0.2987012987012987;
/// The exact producer's seven `exact_straight_pressure_formulation_basis`
/// strings, as physics-source-1's committed n05 envelope publishes them.
fn exact_limitations() -> Value {
    let n05: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/physics_source/n05-sparse_interactive.raw.json"
    ))
    .unwrap();
    n05["formulation_basis"]["limitations"].clone()
}
fn decode_bits(v: &Value) -> f64 {
    f64::from_bits(u64::from_str_radix(v.as_str().unwrap(), 16).unwrap())
}
/// A reader-local synthetic exact successor (not producer output; lane P's
/// `m3x` successors replace it as the witness): a shared preview base whose
/// cases are all selected, re-stated on the exact route.
/// - Invocation: model 0.3.0 with `{2.0.0, exact_straight_pressure_v2}`, every
///   case's `pressure_regions` explicitly [], each material with `poisson_ratio`
///   EXACT_NU and `homogeneous_isotropic_E_nu_v1`.
/// - Envelope: the exact identity and profile with the exact producer's
///   limitations; physics-1's `contract_evidence` built from the receipt's own
///   sections (OD, wall, ro, A, I, J, Z), the base's extrema and coverage, the
///   material's E, nu and G-hat, and a zero pressure assembly.
/// - Receipt: `derived_e_nu` shear origins, `geometry.route` exact, the exact
///   definition id; the invocation digest bound; rehashed with DEF-E's H.
fn exact_successor(shared: &Value, base: &str) -> (Value, Value) {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    use serde_json::json;
    let case = shared["cases"].as_array().unwrap().iter().find(|c| c["id"] == base).unwrap();
    let mut source = case["source"].clone();
    let mut invocation = case["invocation"].clone();
    {
        let model = &mut invocation["request"]["model"];
        model["schema_version"] = json!("0.3.0");
        model["pressure_contract"] = json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"});
        for c in model["load_cases"].as_array_mut().unwrap() {
            c["pressure_regions"] = json!([]);
        }
        for m in model["materials"].as_array_mut().unwrap() {
            m["poisson_ratio"] = json!({"value": EXACT_NU, "unit": "1"});
            m["constitutive_basis"] = json!("homogeneous_isotropic_E_nu_v1");
        }
    }
    let model = invocation["request"]["model"].clone();
    let body = source["retained_precision"]["body"].clone();
    let nodes: Vec<Value> = model["nodes"].as_array().unwrap().iter().map(|n| n["id"].clone()).collect();
    let mut exact_cases = Vec::new();
    for (ci, rc) in body["cases"].as_array().unwrap().iter().enumerate() {
        assert_eq!(rc["status"], "selected", "{base}: every case selected");
        let id = rc["basis_ref"]["ref_id"].clone();
        let s = &body["sources"][rc["source_ref"].as_u64().unwrap() as usize];
        let mb = &body["material_bases"][body["product_attempts"][rc["product_attempt_ref"].as_u64().unwrap() as usize]["material_basis_ref"].as_u64().unwrap() as usize];
        let mut sections = Vec::new();
        let mut materials = Vec::new();
        for (i, p) in model["pipe_segments"].as_array().unwrap().iter().enumerate() {
            let st = &s["section_terms"][i];
            assert_eq!(s["id_maps"]["members"][i]["id"], p["id"]);
            let g = &st["geometry"];
            let (od, wall) = (decode_bits(&g["normalized_od"]), decode_bits(&g["effective_wall"]));
            let ri = od * 0.5 - wall;
            sections.push(json!({"pipe_id": p["id"], "geometry_basis": "authored_normalized_od_wall_v1",
                "outside_diameter_m": od, "effective_wall_thickness_m": wall, "ri_m": ri,
                "ro_m": decode_bits(&g["actual_radius"]), "Ai_m2": std::f64::consts::PI * ri * ri,
                "As_m2": decode_bits(&st["area"]), "I_m4": decode_bits(&g["actual_second_moment"]),
                "J_m4": decode_bits(&g["actual_polar_moment"]), "Z_m3": decode_bits(&st["section_modulus"])}));
            let m = mb["materials"].as_array().unwrap().iter().find(|m| m["id"] == p["material"]).unwrap();
            let e = decode_bits(&m["elastic_modulus"]);
            let g_hat = e / (2.0 * (1.0 + EXACT_NU));
            assert_eq!(g_hat.to_bits(), decode_bits(&m["shear_modulus"]).to_bits(), "G-hat keeps the receipt's G");
            let authored = model["materials"].as_array().unwrap().iter().find(|a| a["id"] == p["material"]).unwrap();
            materials.push(json!({"pipe_id": p["id"], "material_id": p["material"], "E_pa": e, "nu": EXACT_NU,
                "G_pa": g_hat, "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "thermal_consumed": false,
                "alpha_per_kelvin": null, "provenance": authored["provenance"]}));
        }
        let preview = source["contract_evidence"]["preview_cases"].as_array().unwrap().iter()
            .find(|c| c["load_case_id"] == id).unwrap();
        let zeros = json!(vec![0.0; nodes.len() * 6]);
        exact_cases.push(json!({"load_case_id": id, "profile_mode": "exact_straight_pressure_v2",
            "material_basis": "base_material_common_E_nu", "pipe_materials": materials, "pipe_sections": sections,
            "pipe_stress_extrema": preview["pipe_stress_extrema"],
            "stress_maximum_coverage": {"complete": preview["stress_maximum_coverage"]["complete"],
                "unavailable_pipe_ids": preview["stress_maximum_coverage"]["unavailable_pipe_ids"]},
            "pressure_rhs_assembly": {"method": "source_factor_grouped_pressure_rhs_v1", "load_case_id": id,
                "node_order": nodes, "dof_order": ["Fx", "Fy", "Fz", "Mx", "My", "Mz"],
                "dof_units": ["N", "N", "N", "N*m", "N*m", "N*m"], "assembled_pressure_rhs_global": zeros,
                "groups": [], "rounded_cap_rhs_global": zeros, "rounded_poisson_rhs_global": zeros,
                "rounded_cap_and_eigen_ledgers_are_observational": true, "cancellation_screen": 0.0,
                "screen_limit": 1e-9, "screen_roundoff_multiplier": 32,
                "screen_is_not_numerical_qualification": true}}));
        let _ = ci;
    }
    source["producer"]["semantic_contract_id"] = json!(rp::EXACT_CONTRACT_ID);
    source["formulation_basis"] = json!({"profile_id": rp::EXACT_PROFILE, "limitations": exact_limitations()});
    source["contract_evidence"] = json!({"pressure": [], "connector": [], "exact_cases": exact_cases});
    let b = &mut source["retained_precision"]["body"];
    for mb in b["material_bases"].as_array_mut().unwrap() {
        for m in mb["materials"].as_array_mut().unwrap() {
            m["shear_origin"] = json!({"kind": "derived_e_nu", "poisson_ratio": format!("{:016x}", EXACT_NU.to_bits()),
                "constitutive_basis": "homogeneous_isotropic_E_nu_v1"});
        }
    }
    for s in b["sources"].as_array_mut().unwrap() {
        for st in s["section_terms"].as_array_mut().unwrap() {
            st["geometry"]["route"] = json!("exact");
        }
    }
    for a in b["product_attempts"].as_array_mut().unwrap() {
        a["definition_id"] = json!(rp::EXACT_DEFINITION_ID);
    }
    b["invocation"]["value"] = domain_hash("source_blocks_invocation_v1", &invocation).unwrap().into();
    rehash(&mut source);
    (source, invocation)
}
/// The three readings of a statement: bound, unbound and transport (gate and
/// code, or "ok" with the eligibility).
fn readings(source: &Value, invocation: &Value) -> Value {
    use serde_json::json;
    let one = |r: Result<rp::Validation, rp::ValidationError>| match r {
        Ok(v) => json!({"ok": {"eligible": v.numerical_eligible}}),
        Err(e) => json!({"gate": e.gate, "code": e.code}),
    };
    json!({"bound": one(rp::validate(source, Some(invocation))), "unbound": one(rp::validate(source, None)),
        "transport": one(rp::validate_transport_metadata(source))})
}
/// One B3b shape: `edits` on the synthetic exact successor's source and
/// `invocation_edits` on its invocation (rebinding the digest), rehashed with the
/// route's H, then `after` (edits after the rehash).
struct ExactShape {
    name: &'static str,
    base: &'static str,
    edits: Vec<Value>,
    invocation_edits: Vec<Value>,
    after: Vec<Value>,
}
/// The optional file base: `B3B_EXACT_BASE` names a `{source, invocation}` file (for
/// example lane P's `m3x` exact successor); every shape is then also read on it.
const FILE_BASE: &str = "<file>";
/// Lane P's m3x exact successors (B3b-P; PP retained_facade_tests.rs `EXACT_PINNED`): base
/// name, its S-1 shape's base, the document text, its sha256 and the receipt sha256. Every
/// shape is also read on each.
const M3X: [(&str, &str, &str, &str, &str); 2] = [
    (
        "m3x_sparse_interactive",
        "<s1:m3x_sparse_interactive>",
        include_str!("../../../../fixtures/results/retained_precision_exact_successor_sparse_interactive.json"),
        "02465c6c92ac2e4360a77910cb54803590b5a11042dfddb223bf78f9e856e5d6",
        "b1b4a6682260ca6bc499950b30f0f7179a77c038e266cc4b42045ed86ed3896f",
    ),
    (
        "m3x_dense_scrutiny",
        "<s1:m3x_dense_scrutiny>",
        include_str!("../../../../fixtures/results/retained_precision_exact_successor_dense_scrutiny.json"),
        "31f10f04f6f335dfb1a7e5f904198972903bfc9208660031bbfaa5c547d347cc",
        "eabd2fc57b42158ad415ae664e7c712c1c4db21258b667251758172f3a5b776d",
    ),
];
fn exact_base(shared: &Value, base: &str) -> (Value, Value) {
    use sha2::{Digest, Sha256};
    if let Some((name, _, text, document_sha, receipt_sha)) = M3X.iter().find(|m| m.0 == base) {
        assert_eq!(format!("{:x}", Sha256::digest(text.as_bytes())), *document_sha, "{name}: lane P's pinned document");
        let doc: Value = serde_json::from_str(text).unwrap();
        assert_eq!(doc["source"]["retained_precision"]["receipt_sha256"], *receipt_sha, "{name}");
        return (doc["source"].clone(), doc["invocation"].clone());
    }
    if base == FILE_BASE {
        let path = std::env::var("B3B_EXACT_BASE").unwrap();
        let doc: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        return (doc["source"].clone(), doc["invocation"].clone());
    }
    exact_successor(shared, base)
}
fn exact_shape_input(shared: &Value, shape: &ExactShape) -> (Value, Value) {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    let (mut source, mut invocation) = exact_base(shared, shape.base);
    for e in &shape.edits {
        edit(&mut source, e);
    }
    for e in &shape.invocation_edits {
        edit(&mut invocation, e);
    }
    if !shape.invocation_edits.is_empty() {
        source["retained_precision"]["body"]["invocation"]["value"] =
            domain_hash("source_blocks_invocation_v1", &invocation).unwrap().into();
    }
    rehash(&mut source);
    for e in &shape.after {
        edit(&mut source, e);
    }
    (source, invocation)
}
fn ulps(v: f64, n: i64) -> f64 {
    f64::from_bits((v.to_bits() as i64 + n) as u64)
}
const EXACT_BASES: [&str; 3] = [ORD, "ordinary_prepared_dense_synthetic", "two_case_synthetic"];
const INVOCATION: &str = "RETAINED_PRECISION_INVOCATION_MISMATCH";
const PREPARATION: &str = "RETAINED_PRECISION_PREPARATION_MISMATCH";
const SECTION: &str = "RETAINED_PRECISION_SECTION_MISMATCH";
/// REVISION_01 §4.3's B3b list on the synthetic exact successor (entries 1-32;
/// entry 9 relabels the shared preview base), plus this reader's added shapes; then
/// every row, the base and S-1's shape on each of lane P's two m3x successors.
/// The expected first failures are bound; `B3B_SHAPES_OUT` writes every shape's
/// bound, unbound and transport readings (JSON lines), and `B3B_INPUTS_OUT` the
/// materialized inputs, for the three-reader comparison.
fn b3b_shapes(shared: &Value) -> Vec<(ExactShape, Value)> {
    let mut out = Vec::new();
    for b in EXACT_BASES {
        out.push((ExactShape { name: "base", base: b, edits: vec![], invocation_edits: vec![], after: vec![] }, Value::Null));
    }
    out.extend(b3b_rows(shared, ORD));
    // 09: the shared preview base relabelled physics-retained-1 (identity and profile).
    out.push((ExactShape { name: "09 the preview successor relabelled physics-retained-1", base: "<preview:ordinary_prepared_synthetic>", edits: vec![], invocation_edits: vec![], after: vec![] }, gate("G0", UNSUPPORTED)));
    // 11 (S-1): the preparation hash over a payload with DEF-O's H; source
    // identity, publication and receipt hashes recomputed.
    out.push((ExactShape { name: "11 S-1: the preparation hashed with DEF-O's H", base: "<s1:ordinary_prepared_synthetic>", edits: vec![], invocation_edits: vec![], after: vec![] }, gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")));
    for (name, s1, ..) in M3X {
        out.push((ExactShape { name: "base", base: name, edits: vec![], invocation_edits: vec![], after: vec![] }, Value::Null));
        out.extend(b3b_rows(shared, name));
        out.push((ExactShape { name: "11 S-1: the preparation hashed with DEF-O's H", base: s1, edits: vec![], invocation_edits: vec![], after: vec![] }, gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")));
    }
    // RV120's F2 forgeries (B28, B29): only G8 step 4's E and G-hat bits refuse them.
    for (name, base, edits) in rv120_forgeries() {
        out.push((ExactShape { name, base, edits, invocation_edits: vec![], after: vec![] }, gate("G8", PREPARATION)));
    }
    out.extend(rv120_probes(shared));
    out.extend(repair03_probes());
    if std::env::var("B3B_EXACT_BASE").is_ok() {
        out.push((ExactShape { name: "base", base: FILE_BASE, edits: vec![], invocation_edits: vec![], after: vec![] }, Value::Null));
        out.extend(b3b_rows(shared, FILE_BASE));
        out.push((ExactShape { name: "11 S-1: the preparation hashed with DEF-O's H", base: "<s1:file>", edits: vec![], invocation_edits: vec![], after: vec![] }, gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")));
    }
    out
}
/// RV120's F2 forgeries (REVIEW_RV120 b3_readers_01: its `forge_eg.py` and
/// `forge_eg_inputs.jsonl`, here as edits on the synthetic exact base and lane P's sparse
/// m3x successor; the 07e rehash recomputes the rest). E or G-hat moves one ulp in all five
/// receipt copies (the material basis, the id-map member, the prepared `old_source`, the old
/// and new operational inputs), the operational stiffness the readers derive moves with it
/// (the attempt results and both section-term copies), the evidence `G_pa` moves for G-hat
/// (N-6), and the sources' native hashes and their copies are resealed. The shipped readers
/// refuse each at G8 step 4 (`PREPARATION_MISMATCH`); without step 4's E bits (mutant B28)
/// or G-hat bits (B29) the matching forgeries read bound and eligible.
fn rv120_forgeries() -> Vec<(&'static str, &'static str, Vec<Value>)> {
    use serde_json::json;
    vec![
        ("RV120 F2: E +1 ulp in every receipt copy, stiffness and native hashes resealed", "ordinary_prepared_synthetic", vec![
            set(json!(["retained_precision", "body", "cases", 0, "selection", "section_terms", 0, "axial_stiffness"]), json!("41c4990f17e516ae")),
            set(json!(["retained_precision", "body", "groups", 0, "stiffness_sha256"]), json!("1fbb32395a897d401b673f4527d6858876a6c6668ff3779fa9484855ad2285b2")),
            set(json!(["retained_precision", "body", "material_bases", 0, "materials", 0, "elastic_modulus"]), json!("42474876e8000001")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "new", 0, "inputs", 6]), json!("42474876e8000001")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "new", 0, "result", "axial_stiffness"]), json!("41c4990f17e516ae")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "old", 0, "inputs", 6]), json!("42474876e8000001")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "old", 0, "result", "axial_stiffness"]), json!("41c4990f17e516ae")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "preparation", "members", 0, "old_source", 0]), json!("42474876e8000001")),
            set(json!(["retained_precision", "body", "sources", 0, "id_maps", "members", 0, "E"]), json!("42474876e8000001")),
            set(json!(["retained_precision", "body", "sources", 0, "kernel_source_sha256"]), json!("57dc66560ce90fb976a633b07f55c736b11f2b78265d791a3a4123c0751be478")),
            set(json!(["retained_precision", "body", "sources", 0, "section_terms", 0, "axial_stiffness"]), json!("41c4990f17e516ae")),
            set(json!(["retained_precision", "body", "sources", 0, "stiffness_sha256"]), json!("1fbb32395a897d401b673f4527d6858876a6c6668ff3779fa9484855ad2285b2")),
        ]),
        ("RV120 F2: G-hat +1 ulp in every receipt copy, stiffness and native hashes resealed", "ordinary_prepared_synthetic", vec![
            set(json!(["contract_evidence", "exact_cases", 0, "pipe_materials", 0, "G_pa"]), json!(f64::from_bits(0x4231ed8ec2000001))),
            set(json!(["retained_precision", "body", "cases", 0, "selection", "section_terms", 0, "torsional_stiffness"]), json!("4128c47ead23fa81")),
            set(json!(["retained_precision", "body", "groups", 0, "stiffness_sha256"]), json!("77a160acc4ffcf3ed919fe1e59ab3b670cc3ca4dcacbe192d9f932042c9e0980")),
            set(json!(["retained_precision", "body", "material_bases", 0, "materials", 0, "shear_modulus"]), json!("4231ed8ec2000001")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "new", 0, "inputs", 7]), json!("4231ed8ec2000001")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "new", 0, "result", "torsional_stiffness"]), json!("4128c47ead23fa81")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "old", 0, "inputs", 7]), json!("4231ed8ec2000001")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "old", 0, "result", "torsional_stiffness"]), json!("4128c47ead23fa82")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "preparation", "members", 0, "old_source", 1]), json!("4231ed8ec2000001")),
            set(json!(["retained_precision", "body", "sources", 0, "id_maps", "members", 0, "G"]), json!("4231ed8ec2000001")),
            set(json!(["retained_precision", "body", "sources", 0, "kernel_source_sha256"]), json!("d098e88140c2f23621802f449f296928412138208b2adf72553b0a4790fe8f58")),
            set(json!(["retained_precision", "body", "sources", 0, "section_terms", 0, "torsional_stiffness"]), json!("4128c47ead23fa81")),
            set(json!(["retained_precision", "body", "sources", 0, "stiffness_sha256"]), json!("77a160acc4ffcf3ed919fe1e59ab3b670cc3ca4dcacbe192d9f932042c9e0980")),
        ]),
        ("RV120 F2: E +1 ulp in every receipt copy, stiffness and native hashes resealed", "m3x_sparse_interactive", vec![
            set(json!(["retained_precision", "body", "cases", 0, "selection", "section_terms", 0, "axial_stiffness"]), json!("41b7b801dd7467b1")),
            set(json!(["retained_precision", "body", "groups", 0, "stiffness_sha256"]), json!("1fbbf4ed8e9842ff8f6dcbe94517a8527eead7b775277e327e62b6e4538d712e")),
            set(json!(["retained_precision", "body", "material_bases", 0, "materials", 0, "elastic_modulus"]), json!("42474876e8000001")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "new", 0, "inputs", 6]), json!("42474876e8000001")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "new", 0, "result", "axial_stiffness"]), json!("41b7b801dd7467b1")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "old", 0, "inputs", 6]), json!("42474876e8000001")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "old", 0, "result", "axial_stiffness"]), json!("41b7b801dd7467b1")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "preparation", "members", 0, "old_source", 0]), json!("42474876e8000001")),
            set(json!(["retained_precision", "body", "sources", 0, "id_maps", "members", 0, "E"]), json!("42474876e8000001")),
            set(json!(["retained_precision", "body", "sources", 0, "kernel_source_sha256"]), json!("ab4596d0b1a50b195b0368d573dbf26ed4c2cb6c1e76da6490052217b12b8bb2")),
            set(json!(["retained_precision", "body", "sources", 0, "section_terms", 0, "axial_stiffness"]), json!("41b7b801dd7467b1")),
            set(json!(["retained_precision", "body", "sources", 0, "stiffness_sha256"]), json!("1fbbf4ed8e9842ff8f6dcbe94517a8527eead7b775277e327e62b6e4538d712e")),
        ]),
        ("RV120 F2: G-hat +1 ulp in every receipt copy, stiffness and native hashes resealed", "m3x_sparse_interactive", vec![
            set(json!(["contract_evidence", "exact_cases", 0, "pipe_materials", 0, "G_pa"]), json!(f64::from_bits(0x4232a05f20000001))),
            set(json!(["retained_precision", "body", "cases", 0, "selection", "section_terms", 0, "torsional_stiffness"]), json!("4135fb0cf390a830")),
            set(json!(["retained_precision", "body", "groups", 0, "stiffness_sha256"]), json!("d7aa429e5be25210f15a34f7138f7178efbbc443d6a016cc35f857268d9ec194")),
            set(json!(["retained_precision", "body", "material_bases", 0, "materials", 0, "shear_modulus"]), json!("4232a05f20000001")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "new", 0, "inputs", 7]), json!("4232a05f20000001")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "new", 0, "result", "torsional_stiffness"]), json!("4135fb0cf390a830")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "old", 0, "inputs", 7]), json!("4232a05f20000001")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "operational", "old", 0, "result", "torsional_stiffness"]), json!("4135fb0cf390a831")),
            set(json!(["retained_precision", "body", "product_attempts", 0, "preparation", "members", 0, "old_source", 1]), json!("4232a05f20000001")),
            set(json!(["retained_precision", "body", "sources", 0, "id_maps", "members", 0, "G"]), json!("4232a05f20000001")),
            set(json!(["retained_precision", "body", "sources", 0, "kernel_source_sha256"]), json!("7f965da3aaa6d9720c549022c8947fa602a2065c26415d5663f5dc398e8f7b84")),
            set(json!(["retained_precision", "body", "sources", 0, "section_terms", 0, "torsional_stiffness"]), json!("4135fb0cf390a830")),
            set(json!(["retained_precision", "body", "sources", 0, "stiffness_sha256"]), json!("d7aa429e5be25210f15a34f7138f7178efbbc443d6a016cc35f857268d9ec194")),
        ]),
    ]
}
/// RV120's probes for repair 02 (REVIEW_RV120 b3_readers_01, `gen_b3_probes.py`): N2, an
/// `exact_cases` entry for a case not in the invocation (entry 0 copied, renamed), on the
/// synthetic exact base and lane P's sparse m3x successor; and F1's G5b order probes on the
/// exact `two_case_synthetic` (case 0's evidence `As_m2` one ulp, with or without case 1's
/// body-scale force or section-term area changed). Bound: G7 (physics-1's base code) for N2;
/// the shared G5b's SCALE or SECTION code, then the exact evidence, for F1.
fn rv120_probes(shared: &Value) -> Vec<(ExactShape, Value)> {
    use serde_json::json;
    let mut out = Vec::new();
    for base in [ORD, "m3x_sparse_interactive"] {
        let (source, _) = exact_base(shared, base);
        let entry = source["contract_evidence"]["exact_cases"][0].clone();
        let mut other = entry.clone();
        other["load_case_id"] = json!("case:rv120-other");
        out.push((
            ExactShape { name: "RV120 N2: an exact_cases entry for a case not in the invocation", base, edits: vec![set(json!(["contract_evidence", "exact_cases"]), json!([entry, other]))], invocation_edits: vec![], after: vec![] },
            gate("G7", "SOURCE_PHYSICS_NUMERICAL_CASE_COVERAGE"),
        ));
    }
    let base = "two_case_synthetic";
    let (source, _) = exact_base(shared, base);
    let as_m2 = json!(ulps(source["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]["As_m2"].as_f64().unwrap(), 1));
    let evidence = set(json!(["contract_evidence", "exact_cases", 0, "pipe_sections", 0, "As_m2"]), as_m2);
    let force = set(rb(json!(["cases", 1, "selection", "body_scales", 0, "force"])), json!("0000000000000001"));
    let area = set(rb(json!(["cases", 1, "selection", "section_terms", 0, "area"])), json!("3f00000000000000"));
    for (name, edits, want) in [
        ("RV120 F1 control: case 0's evidence As_m2 one ulp", vec![evidence.clone()], gate("G5b", SECTION)),
        ("RV120 F1 control: case 1's body_scales force one ulp", vec![force.clone()], gate("G5b", "RETAINED_PRECISION_SCALE_MISMATCH")),
        ("RV120 F1 order: case 0's evidence As_m2 and case 1's body_scales force", vec![evidence.clone(), force], gate("G5b", "RETAINED_PRECISION_SCALE_MISMATCH")),
        ("RV120 F1 order: case 0's evidence As_m2 and case 1's section term area", vec![evidence, area], gate("G5b", SECTION)),
    ] {
        out.push((ExactShape { name, base, edits, invocation_edits: vec![], after: vec![] }, want));
    }
    out
}
/// RV120 N2: RS's physics-1 transport check reads RV120's probe with one code, run after
/// run (it iterates its cases in array order, as TS and PY do): entry 0 passes, and the
/// copy's maximum result ids repeat entry 0's. Bound and unbound stay at G7's case coverage.
#[test]
fn b3b_rv120_n2_transport_code_is_array_ordered_and_stable() {
    use serde_json::json;
    let shared = corpus();
    let probes: Vec<_> = rv120_probes(&shared).into_iter().filter(|(s, _)| s.name.starts_with("RV120 N2: ")).collect();
    assert_eq!(probes.len(), 2);
    let mut digests = Vec::new();
    for (shape, _) in &probes {
        let (source, invocation) = b3b_input(&shared, shape);
        digests.push(sha256_canonical(&json!([source, invocation])));
        let want = json!({"bound": {"gate": "G7", "code": "SOURCE_PHYSICS_NUMERICAL_CASE_COVERAGE"},
            "unbound": {"gate": "G7", "code": "SOURCE_PHYSICS_NUMERICAL_CASE_COVERAGE"},
            "transport": {"gate": "G7", "code": "SOURCE_PHYSICS_TRANSPORT_MAXIMUM_RESULT"}});
        // Each call builds fresh hash maps (a fresh random order), so a hash-ordered loop
        // would show both of the probe's first failures well within these runs.
        for run in 0..64 {
            assert_eq!(readings(&source, &invocation), want, "{} [{}], run {run}", shape.name, shape.base);
        }
    }
    assert_eq!(digests, RV120_N2_INPUTS, "RV120's N2 inputs");
}
/// Repair 03 (N2b, WORKING_ITEMS' ruling on RV120's N2): two faults in the two
/// `exact_cases` entries of the exact `two_case_synthetic` (one entry's `profile_mode`
/// "x", the other's `material_basis` not a string), and the same faults swapped. physics-1's
/// base validator reads its cases in array order on the bound and unbound path, as TS and
/// PY do, so entry 0's fault is G7's code: CASE_PROFILE, or STRING_INVALID when swapped.
fn repair03_probes() -> Vec<(ExactShape, Value)> {
    use serde_json::json;
    let profile = |i: usize| set(json!(["contract_evidence", "exact_cases", i, "profile_mode"]), json!("x"));
    let basis = |i: usize| set(json!(["contract_evidence", "exact_cases", i, "material_basis"]), json!(0));
    let base = "two_case_synthetic";
    vec![
        (ExactShape { name: "N2b: entry 0's profile_mode and entry 1's material_basis", base, edits: vec![profile(0), basis(1)], invocation_edits: vec![], after: vec![] }, gate("G7", "SOURCE_PHYSICS_CASE_PROFILE")),
        (ExactShape { name: "N2b: entry 0's material_basis and entry 1's profile_mode", base, edits: vec![basis(0), profile(1)], invocation_edits: vec![], after: vec![] }, gate("G7", "SOURCE_PHYSICS_STRING_INVALID")),
    ]
}
/// Repair 03 (N2b): RS's physics-1 base validator reads the two-fault probes with one code
/// on the bound and unbound path, run after run: entry 0's fault, in array order. Transport
/// refuses both by its closed shape.
#[test]
fn b3b_repair03_n2b_physics_code_is_array_ordered_and_stable() {
    use serde_json::json;
    let shared = corpus();
    let mut digests = Vec::new();
    for (shape, want) in repair03_probes() {
        let (source, invocation) = b3b_input(&shared, &shape);
        digests.push(sha256_canonical(&json!([source, invocation])));
        let want = json!({"bound": want, "unbound": want,
            "transport": {"gate": "G7", "code": "SOURCE_PHYSICS_TRANSPORT_SHAPE"}});
        // Each call builds fresh hash maps (a fresh random order), so a hash-ordered loop
        // would show both entries' faults well within these runs.
        for run in 0..64 {
            assert_eq!(readings(&source, &invocation), want, "{} [{}], run {run}", shape.name, shape.base);
        }
    }
    assert_eq!(digests, REPAIR03_N2B_INPUTS, "the N2b probe inputs");
}
/// The N2b probe inputs (sha256 of the canonical `[source, invocation]`).
const REPAIR03_N2B_INPUTS: [&str; 2] = ["ab2807e39c79015c5929dac4c4a4a86f82e9e6187c66511686c93f57de60759d", "9451b8272ac84a10984b6848fe44d5fcd3585903ce7ab1f3d8fd59f4a3675da6"];
/// RV120's N2 probe inputs (sha256 of the canonical `[source, invocation]`), as in its index.
const RV120_N2_INPUTS: [&str; 2] = ["b73004449c1894a0b562032d7352d18cae0ad351f7e58d91a1572e76963530a0", "a6bb4fdf601064c2946e440b15a6bb7cb320839c9c3097fff11b2f632c6a09fe"];
fn sha256_canonical(v: &Value) -> String {
    use open_pipe_stress_canonical_json::canonical_json;
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(canonical_json(v).as_bytes()))
}
/// The forgeries' materialized inputs: sha256 of the canonical JSON of `[source, invocation]`
/// (`canonical_json`), the same bytes as RV120's input lines (checked against its index).
const RV120_FORGERY_INPUTS: [(&str, &str, &str); 4] = [
    ("RV120 F2: E +1 ulp", "ordinary_prepared_synthetic", "97566866cef65597109de17d34c69508ef3889a1d526321c8feedf5dc88b1f4a"),
    ("RV120 F2: G-hat +1 ulp", "ordinary_prepared_synthetic", "6e67243c49548db2cbcd10be6156c30182c1e884addbf95a264e33965431ef13"),
    ("RV120 F2: E +1 ulp", "m3x_sparse_interactive", "3828c07c73d41e33324ec6be349ab4d265e57b439222962d1df1a91c974eb1ff"),
    ("RV120 F2: G-hat +1 ulp", "m3x_sparse_interactive", "c17c8283efeb034afe82c4e9fdb4f46aad54ffef676c181733fbc6eec43226e6"),
];
/// RV120 F2: the four forgeries are RV120's inputs (by digest), and all three readings
/// hold: bound G8 PREPARATION_MISMATCH (step 4), unbound and on transport never eligible.
#[test]
fn b3b_rv120_f2_forgeries_are_refused_at_g8_step_4() {
    use open_pipe_stress_canonical_json::canonical_json;
    use serde_json::json;
    use sha2::{Digest, Sha256};
    let shared = corpus();
    let shapes = b3b_shapes(&shared);
    let forged: Vec<_> = shapes.iter().filter(|(s, _)| s.name.starts_with("RV120 F2: ")).collect();
    assert_eq!(forged.len(), 4);
    let mut digests = Vec::new();
    for ((shape, want), (prefix, base, _)) in forged.into_iter().zip(RV120_FORGERY_INPUTS) {
        assert!(shape.name.starts_with(prefix) && shape.base == base, "{}", shape.name);
        assert_eq!(*want, json!({"gate": "G8", "code": PREPARATION}));
        let (source, invocation) = b3b_input(&shared, shape);
        digests.push(format!("{:x}", Sha256::digest(canonical_json(&json!([source, invocation])).as_bytes())));
        assert_eq!(
            readings(&source, &invocation),
            json!({"bound": {"gate": "G8", "code": PREPARATION}, "unbound": {"ok": {"eligible": false}}, "transport": {"ok": {"eligible": false}}}),
            "{} [{base}]",
            shape.name
        );
    }
    assert_eq!(digests, RV120_FORGERY_INPUTS.map(|f| f.2), "RV120's inputs");
}
/// REVISION_01 §4.3's entries and the added shapes on one exact base, with every
/// edited value taken from that base (its owner entry, material, nu, case and pipe).
fn b3b_rows(shared: &Value, base_name: &'static str) -> Vec<(ExactShape, Value)> {
    use serde_json::json;
    let ce = |tail: Value| {
        let mut p = vec![json!("contract_evidence"), json!("exact_cases"), json!(0)];
        p.extend(tail.as_array().unwrap().iter().cloned());
        Value::Array(p)
    };
    let (base, base_invocation) = exact_base(shared, base_name);
    let entry = &base["contract_evidence"]["exact_cases"][0];
    let section = |k: &str| entry["pipe_sections"][0][k].as_f64().unwrap();
    let material = |k: &str| entry["pipe_materials"][0][k].as_f64().unwrap();
    let receipt_g = decode_bits(&base["retained_precision"]["body"]["material_bases"][0]["materials"][0]["shear_modulus"]);
    let receipt_e = decode_bits(&base["retained_precision"]["body"]["material_bases"][0]["materials"][0]["elastic_modulus"]);
    let base_model = &base_invocation["request"]["model"];
    let authored_nu = base_model["materials"][0]["poisson_ratio"]["value"].as_f64().unwrap();
    let (case_id, pipe_id) = (base_model["load_cases"][0]["id"].clone(), base_model["pipe_segments"][0]["id"].clone());
    let model = |tail: Value| {
        let mut p = vec![json!("request"), json!("model")];
        p.extend(tail.as_array().unwrap().iter().cloned());
        Value::Array(p)
    };
    let shape = |name, edits, invocation_edits, after| ExactShape { name, base: base_name, edits, invocation_edits, after };
    let g0 = gate("G0", UNSUPPORTED);
    let inv = gate("G8", INVOCATION);
    let prep = gate("G8", PREPARATION);
    let sec = gate("G5b", SECTION);
    let mut out = Vec::new();
    let rows = vec![
        ("01 identity -> the preview id", vec![set(json!(["producer", "semantic_contract_id"]), json!(rp::CONTRACT_ID))], vec![], vec![], g0.clone()),
        ("02 profile -> the preview profile", vec![set(json!(["formulation_basis", "profile_id"]), json!(rp::PROFILE))], vec![], vec![], g0.clone()),
        ("02b producer component_version 0.2.1", vec![set(json!(["producer", "component_version"]), json!("0.2.1"))], vec![], vec![], g0.clone()),
        ("03 an attempt's definition_id -> the ordinary id", vec![set(rb(json!(["product_attempts", 0, "definition_id"])), json!(rp::DEFINITION_ID))], vec![], vec![], g0.clone()),
        ("04 projection_policy changed", vec![set(rb(json!(["projection_policy"])), json!("RP-LOGICAL-ATTEMPTS-v2"))], vec![], vec![], g0.clone()),
        ("04b policy changed", vec![set(rb(json!(["policy"])), json!("M03-INTEGRITY-MP-v3"))], vec![], vec![], g0.clone()),
        ("04c facade_policy changed", vec![set(rb(json!(["facade_policy"])), json!("RP-FACADE-SI-v3"))], vec![], vec![], g0.clone()),
        ("05 work_policy changed", vec![set(rb(json!(["work_policy"])), json!("W1-LME-20B-60B-v2"))], vec![], vec![], g0.clone()),
        ("06 canonicalization changed", vec![set(rb(json!(["canonicalization"])), json!("openpipestress_jcs_ijson_v2"))], vec![], vec![], g0.clone()),
        ("07 work.case_limit changed", vec![set(rb(json!(["work", "case_limit"])), json!(19_999_999_999u64))], vec![], vec![], g0.clone()),
        ("08 work.invocation_limit changed", vec![set(rb(json!(["work", "invocation_limit"])), json!(60_000_000_001u64))], vec![], vec![], g0.clone()),
        ("08b receipt_version 2", vec![set(rb(json!(["receipt_version"])), json!(2))], vec![], vec![], g0.clone()),
        ("10 the exact successor relabelled preview (identity and profile)", vec![set(json!(["producer", "semantic_contract_id"]), json!(rp::CONTRACT_ID)), set(json!(["formulation_basis", "profile_id"]), json!(rp::PROFILE))], vec![], vec![], g0.clone()),
        ("12 owner entry's As_m2 one ulp", vec![set(ce(json!(["pipe_sections", 0, "As_m2"])), json!(ulps(section("As_m2"), 1)))], vec![], vec![], sec.clone()),
        ("13 owner entry's Z_m3 one ulp", vec![set(ce(json!(["pipe_sections", 0, "Z_m3"])), json!(ulps(section("Z_m3"), 1)))], vec![], vec![], sec.clone()),
        ("14 owner entry's I_m4 one ulp", vec![set(ce(json!(["pipe_sections", 0, "I_m4"])), json!(ulps(section("I_m4"), 1)))], vec![], vec![], sec.clone()),
        ("15 owner entry's ro_m one ulp", vec![set(ce(json!(["pipe_sections", 0, "ro_m"])), json!(ulps(section("ro_m"), 1)))], vec![], vec![], sec.clone()),
        ("15b owner entry's J_m4 one ulp", vec![set(ce(json!(["pipe_sections", 0, "J_m4"])), json!(ulps(section("J_m4"), 1)))], vec![], vec![], sec.clone()),
        ("15c owner entry's outside_diameter_m one ulp", vec![set(ce(json!(["pipe_sections", 0, "outside_diameter_m"])), json!(ulps(section("outside_diameter_m"), 1)))], vec![], vec![], sec.clone()),
        ("15d owner entry's effective_wall_thickness_m one ulp", vec![set(ce(json!(["pipe_sections", 0, "effective_wall_thickness_m"])), json!(ulps(section("effective_wall_thickness_m"), 1)))], vec![], vec![], sec.clone()),
        ("15e the owner entry's load_case_id renamed (no entry for the selected case)", vec![set(ce(json!(["load_case_id"])), json!("case:other"))], vec![], vec![], sec.clone()),
        ("15f the owner entry's pipe section listed twice", vec![set(ce(json!(["pipe_sections"])), json!([entry["pipe_sections"][0], entry["pipe_sections"][0]]))], vec![], vec![], sec.clone()),
        ("15g the owner entry's As_m2 a string", vec![set(ce(json!(["pipe_sections", 0, "As_m2"])), json!(format!("{}", section("As_m2"))))], vec![], vec![], sec.clone()),
        ("15h the owner entry listed twice", vec![set(json!(["contract_evidence", "exact_cases"]), json!([entry, entry]))], vec![], vec![], sec.clone()),
        ("16b contract_evidence removed", vec![remove(json!(["contract_evidence"]))], vec![], vec![], sec.clone()),
        ("16 connector non-empty", vec![set(json!(["contract_evidence", "connector"]), json!([{"id": "connector:x"}]))], vec![], vec![], gate("G7", "SOURCE_PHYSICS_CONNECTOR_UNSUPPORTED")),
        ("17 an entry's G_pa three ulps", vec![set(ce(json!(["pipe_materials", 0, "G_pa"])), json!(ulps(material("G_pa"), 3)))], vec![], vec![], gate("G7", "SOURCE_PHYSICS_MATERIAL_G_BINDING")),
        ("18 recovery_method added to an exact_cases entry", vec![set(ce(json!(["recovery_method"])), json!("retained_source_blocks_exact_v1"))], vec![], vec![], gate("G7", "SOURCE_PHYSICS_CASE_SHAPE")),
        ("19 invocation contract -> legacy 1.0.0", vec![], vec![set(model(json!(["pressure_contract"])), json!({"version": "1.0.0", "mode": "legacy_pressure_v1"}))], vec![], inv.clone()),
        ("20 invocation schema -> 0.4.0", vec![], vec![set(model(json!(["schema_version"])), json!("0.4.0"))], vec![], inv.clone()),
        ("21 invocation pressure_contract false", vec![], vec![set(model(json!(["pressure_contract"])), json!(false))], vec![], inv.clone()),
        ("21b invocation contract with an extra key", vec![], vec![set(model(json!(["pressure_contract"])), json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2", "extra": null}))], vec![], inv.clone()),
        ("21c invocation contract version 2.0.1", vec![], vec![set(model(json!(["pressure_contract"])), json!({"version": "2.0.1", "mode": "exact_straight_pressure_v2"}))], vec![], inv.clone()),
        ("21d invocation contract removed (0.3.0, absent)", vec![], vec![remove(model(json!(["pressure_contract"])))], vec![], inv.clone()),
        // Entry 22: B2-C admits model combinations; G8's entry and expression equality refuses this one (no receipt entry names it).
        ("22 a combination added to the invocation", vec![], vec![set(model(json!(["combinations"])), json!([{"id": "combination:x", "kind": "algebraic", "terms": [{"load_case": case_id, "factor": 1.0}]}]))], vec![], inv.clone()),
        ("22b a component added to the invocation", vec![], vec![set(model(json!(["components"])), json!([{"id": "component:x"}]))], vec![], inv.clone()),
        ("23 a case naming modulus_basis_ref", vec![], vec![set(model(json!(["load_cases", 0, "modulus_basis_ref"])), json!("point:x"))], vec![], prep.clone()),
        // D1.5: only the base common E/nu, even a named point equal to the base, with
        // the receipt's selector naming it alike (step 3 alone refuses it).
        ("23b a case naming a point equal to the base, the receipt's selector alike", vec![set(rb(json!(["material_bases", 0, "selector"])), json!({"kind": "named", "id": "point:base"}))], vec![
            set(model(json!(["materials", 0, "temperature_points"])), json!([{"id": "point:base", "elastic_modulus": base_model["materials"][0]["elastic_modulus"], "poisson_ratio": base_model["materials"][0]["poisson_ratio"]}])),
            set(model(json!(["load_cases", 0, "modulus_basis_ref"])), json!("point:base")),
        ], vec![], prep.clone()),
        ("24 a material's shear_origin -> explicit_g", vec![set(rb(json!(["material_bases", 0, "materials", 0, "shear_origin"])), json!({"kind": "explicit_g"}))], vec![], vec![], prep.clone()),
        ("24b a material's selection -> named_point", vec![set(rb(json!(["material_bases", 0, "materials", 0, "selection"])), json!({"kind": "named_point", "point_id": "point:x"}))], vec![], vec![], prep.clone()),
        ("25b a material's elastic_modulus one ulp", vec![set(rb(json!(["material_bases", 0, "materials", 0, "elastic_modulus"])), json!(format!("{:016x}", ulps(receipt_e, 1).to_bits())))], vec![], vec![], prep.clone()),
        ("25 a material's shear_modulus one ulp", vec![set(rb(json!(["material_bases", 0, "materials", 0, "shear_modulus"])), json!(format!("{:016x}", ulps(receipt_g, 1).to_bits())))], vec![], vec![], prep.clone()),
        ("26 shear_origin.poisson_ratio bits changed", vec![set(rb(json!(["material_bases", 0, "materials", 0, "shear_origin", "poisson_ratio"])), json!(format!("{:016x}", ulps(material("nu"), 1).to_bits())))], vec![], vec![], prep.clone()),
        ("27 authored nu changed in the invocation", vec![], vec![set(model(json!(["materials", 0, "poisson_ratio", "value"])), json!(ulps(authored_nu, 1)))], vec![], prep.clone()),
        ("28 S-C only: an entry's pipe_materials nu one ulp", vec![set(ce(json!(["pipe_materials", 0, "nu"])), json!(ulps(material("nu"), 1)))], vec![], vec![], prep.clone()),
        ("29 N-6: an entry's G_pa one ulp", vec![set(ce(json!(["pipe_materials", 0, "G_pa"])), json!(ulps(material("G_pa"), 1)))], vec![], vec![], prep.clone()),
        ("30 a case's pressure_regions -> null", vec![], vec![set(model(json!(["load_cases", 0, "pressure_regions"])), Value::Null)], vec![], prep.clone()),
        ("30b a case's pressure_regions absent", vec![], vec![remove(model(json!(["load_cases", 0, "pressure_regions"])))], vec![], prep.clone()),
        ("31 a case's pressure_regions -> one region", vec![], vec![set(model(json!(["load_cases", 0, "pressure_regions"])), json!([{"id": "region:x", "member_pipe_ids": [pipe_id], "pressure": {"value": 0, "unit": "Pa"}}]))], vec![], prep.clone()),
        ("32 a member's geometry.route -> preview", vec![set(rb(json!(["sources", 0, "section_terms", 0, "geometry", "route"])), json!("preview"))], vec![], vec![], prep.clone()),
        ("32b the authored poisson_ratio unit -> \"\"", vec![], vec![set(model(json!(["materials", 0, "poisson_ratio", "unit"])), json!(""))], vec![], prep.clone()),
        ("32c the authored constitutive_basis removed (S-C)", vec![], vec![remove(model(json!(["materials", 0, "constitutive_basis"])))], vec![], prep.clone()),
    ];
    for (name, edits, invocation_edits, after, want) in rows {
        out.push((shape(name, edits, invocation_edits, after), want));
    }
    out
}
fn b3b_input(shared: &Value, shape: &ExactShape) -> (Value, Value) {
    use serde_json::json;
    match shape.base {
        "<preview:ordinary_prepared_synthetic>" => {
            let case = shared["cases"].as_array().unwrap().iter().find(|c| c["id"] == ORD).unwrap();
            let mut source = case["source"].clone();
            source["producer"]["semantic_contract_id"] = json!(rp::EXACT_CONTRACT_ID);
            source["formulation_basis"]["profile_id"] = json!(rp::EXACT_PROFILE);
            rehash(&mut source);
            (source, case["invocation"].clone())
        }
        s1 if s1.starts_with("<s1:") => {
            let name = &s1[4..s1.len() - 1];
            let (mut source, invocation) = exact_base(shared, if name == "file" { FILE_BASE } else { name });
            rehash_with(&mut source, rp::DEFINITION_HASH);
            (source, invocation)
        }
        _ => exact_shape_input(shared, shape),
    }
}
#[test]
fn b3b_exact_successor_shapes_first_failures() {
    use std::io::Write;
    let shared = corpus();
    let shapes = b3b_shapes(&shared);
    let mut misses = Vec::new();
    let mut lines = Vec::new();
    for (shape, want) in &shapes {
        let (source, invocation) = b3b_input(&shared, shape);
        let got = observe_validation(rp::validate(&source, Some(&invocation)));
        if got != *want {
            misses.push(format!("{} [{}]: got {got}, want {want}", shape.name, shape.base));
        }
        let mut line = readings(&source, &invocation);
        line["name"] = shape.name.into();
        line["base"] = shape.base.into();
        lines.push(line);
    }
    if let Ok(path) = std::env::var("B3B_SHAPES_OUT") {
        let mut f = std::fs::File::create(path).unwrap();
        for l in &lines {
            writeln!(f, "{}", serde_json::to_string(l).unwrap()).unwrap();
        }
    }
    // `B3B_INPUTS_OUT`: every shape's materialized statement and invocation (JSON lines),
    // for the other readers' comparison on identical bytes.
    if let Ok(path) = std::env::var("B3B_INPUTS_OUT") {
        let mut f = std::fs::File::create(path).unwrap();
        for (shape, want) in &shapes {
            let (source, invocation) = b3b_input(&shared, shape);
            let line = serde_json::json!({"name": shape.name, "base": shape.base, "expected_bound": want, "source": source, "invocation": invocation});
            writeln!(f, "{}", serde_json::to_string(&line).unwrap()).unwrap();
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
    assert_eq!(shapes.iter().filter(|(s, _)| s.base != FILE_BASE && s.base != "<s1:file>").count(), 3 + 52 + 2 + 2 * (1 + 52 + 1) + 4 + 2 + 4 + 2);
}
fn observe_validation(r: Result<rp::Validation, rp::ValidationError>) -> Value {
    match r {
        Err(e) => serde_json::json!({"gate": e.gate, "code": e.code}),
        Ok(_) => Value::Null,
    }
}
/// B3b: the synthetic exact successors and lane P's m3x successors validate bound
/// (eligible), unbound and on transport (never eligible); the standing is the receipt's; the base
/// dispatch returns the exact table; S-C's code is the G8 failure's detail.
#[test]
fn b3b_exact_successor_readings_standing_and_dispatch() {
    use open_pipe_stress_result_export::semantic_contract as sc;
    use serde_json::json;
    let shared = corpus();
    for base in EXACT_BASES.into_iter().chain(M3X.map(|m| m.0)) {
        let (source, invocation) = exact_base(&shared, base);
        assert_eq!(
            readings(&source, &invocation),
            json!({"bound": {"ok": {"eligible": true}}, "unbound": {"ok": {"eligible": false}},
                "transport": {"ok": {"eligible": false}}}),
            "{base}"
        );
        let refs: Vec<Value> = source["retained_precision"]["body"]["cases"].as_array().unwrap().iter().map(|c| c["basis_ref"].clone()).collect();
        assert_eq!(sc::numerical_use_standing_with_context(&source, &refs, Some(&invocation)), "numerically_eligible", "{base}");
        assert_eq!(sc::numerical_use_standing_with_context(&source, &refs, None), "needs_recompute", "{base}");
        let (table, version) = sc::for_source(&source).unwrap();
        assert_eq!((table["semantic_contract_id"].as_str(), version), (Some(rp::EXACT_CONTRACT_ID), "0.3.0"));
        assert!(std::ptr::eq(table, sc::physics_retained_contract()));
        let (table, _) = sc::for_source_metadata(&source).unwrap();
        assert!(std::ptr::eq(table, sc::physics_retained_contract()));
        let classes = sc::retained_row_classes(&source).unwrap().unwrap();
        assert_eq!(classes.len(), rp::validate(&source, None).unwrap().classifications.len());
        // F-5: the receipt member and the method token stay the successors' own.
        let mut downgraded = source.clone();
        downgraded["producer"]["semantic_contract_id"] = json!(sc::PHYSICS_ID);
        downgraded["formulation_basis"]["profile_id"] = json!("exact_straight_pressure_v2");
        assert_eq!(sc::for_source(&downgraded).unwrap_err(), sc::RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
    }
    // Unbound and transport readings of three shapes: a G8 shape reads unbound and on
    // transport; removing the evidence is G5b bound and unbound, and on transport
    // physics-1's own transport-shape code at G7 (no header refusal: physics-1's
    // header does not require the evidence, as preview-physics-1's does).
    let shapes = b3b_shapes(&shared);
    for (prefix, want) in [
        ("29 ", json!({"bound": {"gate": "G8", "code": PREPARATION}, "unbound": {"ok": {"eligible": false}}, "transport": {"ok": {"eligible": false}}})),
        ("16b ", json!({"bound": {"gate": "G5b", "code": SECTION}, "unbound": {"gate": "G5b", "code": SECTION}, "transport": {"gate": "G7", "code": "SOURCE_PHYSICS_TRANSPORT_SHAPE"}})),
        ("01 ", json!({"bound": {"gate": "G0", "code": UNSUPPORTED}, "unbound": {"gate": "G0", "code": UNSUPPORTED}, "transport": {"gate": "G0", "code": UNSUPPORTED}})),
    ] {
        let (shape, _) = shapes.iter().find(|(s, _)| s.name.starts_with(prefix)).unwrap();
        let (source, invocation) = b3b_input(&shared, shape);
        assert_eq!(readings(&source, &invocation), want, "{}", shape.name);
    }
    // S-C's own code is the detail of G8's PREPARATION_MISMATCH (B3D-13).
    let (shape, _) = shapes.iter().find(|(s, _)| s.name.starts_with("28 ")).unwrap();
    let (source, invocation) = b3b_input(&shared, shape);
    let e = rp::validate(&source, Some(&invocation)).unwrap_err();
    assert_eq!((e.gate, e.code.as_str()), ("G8", PREPARATION));
    assert!(e.detail.as_deref().is_some_and(|d| d.contains("ACTUAL_SELECTED_MATERIAL")), "{e:?}");
    // The table pins: identity, profile and bytes.
    assert_eq!(sc::verify_physics_retained_table(b"{}").unwrap_err(), "SOURCE_PHYSICS_RETAINED_TABLE_HASH");
    assert_eq!(sc::physics_retained_contract()["formulation_profile_id"], rp::EXACT_PROFILE);
}

/// B3b (D2 §4.9.7; B3-D §7): derivative.rs carries the exact successor's
/// `contract_evidence` and its receipt whole, and validates the document against
/// the source (the synthetic exact successors and lane P's m3x successors). The base and origin are a minimal 0.2.0 scaffold of the desktop's
/// (T6S-2's `desktop_base` and `desktop_origin`, retained_precision_derivative_golden.rs);
/// the exact successor's golden, on lane P's `m3x` successors, is lane T's.
#[test]
fn b3b_exact_successor_derivative_carries_the_receipt_and_evidence() {
    use open_pipe_stress_result_export::{derivative as d, semantic_contract as sc};
    use serde_json::json;
    let shared = corpus();
    for base in EXACT_BASES.into_iter().chain(M3X.map(|m| m.0)) {
        let (source, invocation) = exact_base(&shared, base);
        let model = &invocation["request"]["model"];
        let run = source["run_id"].as_str().unwrap();
        let project = model["project"]["id"].as_str().unwrap();
        let provenance = json!({"source_name":"b3b exact successor derivative control","source_location":"core/reporting/result_export/tests/retained_precision_contract.rs","source_license":"project-local","contributor":"test","contributor_certification":"test","redistribution_status":"private_only","review_status":"pending"});
        let diagnostics: Vec<Value> = source["diagnostics"].as_array().unwrap().iter().map(|x| json!({
            "code": x["code"], "class": "ASSUMPTION_WARNING", "severity": x["severity"],
            "source": {"ref_type":"source","ref_id":x["source"]}, "affected_object": {"ref_type":"preview_entity","ref_id":x["affected_refs"][0]},
            "message": x["message"], "remediation": "Review source model and preview limitations.", "provenance": provenance})).collect();
        let mut statuses = vec!["HUMAN_REVIEW_REQUIRED".to_string(), source["status"]["mechanics"].as_str().unwrap().into(), source["status"]["rule_check"].as_str().unwrap().into()];
        statuses.sort();
        let doc_base = json!({
            "schema_version": "0.2.0", "deliverable_id": "DEL-08-04", "package_id": "PKG-08", "scope_item": "SOW-046", "objectives": ["OBJ-007", "OBJ-009"],
            "export_format_status": {"baseline_format":"schema_first_json_result_envelope","additional_formats":"TBD","public_transport_protocol":"TBD","local_fea_package_format":"TBD","external_adapter_formats":"TBD"},
            "result_envelope": {
                "schema_version": "0.2.0", "envelope_id": format!("result-envelope:{run}"), "model_ref": d::reference("model_payload", project),
                "run_ref": d::reference("analysis_run", run),
                "solver_version": {"solver_name": source["producer"]["component_name"], "solver_version": source["producer"]["component_version"], "solver_build_ref": "test:b3b-exact-derivative-control"},
                "unit_system_ref": d::reference("unit_system", &format!("{project}:units")),
                "load_basis_refs": model["load_cases"].as_array().unwrap().iter().map(|c| d::reference("LoadCase", c["id"].as_str().unwrap())).collect::<Vec<_>>(),
                "result_sets": [{"set_id":format!("result-set:{run}:mechanics"),"set_type":"mechanics","basis_ref":d::reference("analysis_run", run),"values":[]}],
                "diagnostics": diagnostics, "provenance": provenance,
                "reproducibility": {"model_hash":null,"run_hashes":[],"audit_manifest_ref":d::reference("audit_manifest", "test:b3b-manifest"),"deterministic_ordering":true},
                "analysis_status": statuses,
                "professional_boundary": {"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},
                "downstream_use": {"review":true,"regression_comparison":true,"report_consumption":true,"headless_automation":true,"governed_downstream_tooling":true,"additional_export_formats":"TBD"},
            }
        });
        let origin = json!({
            "origin_id": "source-origin:current-received", "origin_class": "received_current_dimension_absent",
            "qualification_ref": d::reference("current_manifest", "test:b3b-manifest"), "authentic_producer_available": false,
            "received_carrier_checksum": d::checksum(&source, "received_current_dimension_absent_carrier", d::reference("received_current_carrier", run)).unwrap(),
            "original_producer_checksum": null,
            "origin_limit": "Test-built origin: a reader-local synthetic exact successor, not a qualified Current received carrier",
            "actual_model_ref": d::reference("model_payload", project), "mechanics_run_ref": d::reference("mechanics_run", run),
            "request_model_ref": null, "request_run_ref": null, "request_alias_disclosure": null,
        });
        let doc = d::derive_document(doc_base, model, &source, origin, None).unwrap_or_else(|e| panic!("{base}: {e}"));
        let e = &doc["result_envelope"];
        assert_eq!(e["retained_precision"], source["retained_precision"], "{base}: the receipt travels whole");
        assert_eq!(e["contract_evidence"], source["contract_evidence"], "{base}: physics-1's evidence as received");
        assert_eq!(e["semantic_contract_ref"], d::reference("semantic_contract", sc::PHYSICS_RETAINED_ID));
        d::validate_document(&doc, &source).unwrap();
        // A document without the receipt is refused against the exact successor.
        let mut stripped = doc.clone();
        stripped["result_envelope"].as_object_mut().unwrap().remove("retained_precision");
        assert_eq!(d::validate_document(&stripped, &source).unwrap_err(), d::RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH);
    }
}

/// ROOT's ruling on I100's B3 addendum 01: G8's sourced-case check, on the preview and
/// exact routes. A sourced case passes only with `pressure_regions` absent, null or []
/// (type-strict; [] only on the exact route), `equivalent_static` absent or null, and no
/// `analysis_state` member (null included); a case-level `pressure`, a key PP's typed case
/// lacks, is not read. I100's 22 values on PP's milestone successors, both modes (an
/// invocation edit, resealed with DEF-O's H), and its x08 (`analysis_state`) and p02 (a
/// case-level `pressure`) on the synthetic and m3x exact successors (DEF-E's H): bound G8
/// or eligible; unbound and on transport never refused and never eligible (G8 does not run).
#[test]
fn b3_add1_g8_sourced_case_on_the_preview_and_exact_routes() {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    use serde_json::json;
    const MILESTONES: [(&str, &str); 2] = [
        ("sparse_interactive", include_str!("../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json")),
        ("dense_scrutiny", include_str!("../../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json")),
    ];
    let preview: Vec<(&str, Value, bool)> = vec![
        ("analysis_state", json!({"kind": "load_reference_state"}), false),
        ("analysis_state", Value::Null, false),
        ("analysis_state", json!({}), false),
        ("pressure", json!({"value": 1000.0, "unit": "Pa"}), true),
        ("pressure", Value::Null, true),
        ("pressure", json!(0), true),
        ("pressure_regions", json!("x"), false),
        ("pressure_regions", json!({}), false),
        ("pressure_regions", json!({"id": "region:x"}), false),
        ("pressure_regions", json!(0), false),
        ("pressure_regions", json!(1), false),
        ("pressure_regions", json!(true), false),
        ("pressure_regions", json!(false), false),
        ("pressure_regions", json!(""), false),
        ("pressure_regions", json!([]), true),
        ("pressure_regions", Value::Null, true),
        ("pressure_regions", json!([{"id": "region:x", "member_pipe_ids": ["M1"]}]), false),
        ("equivalent_static", json!({}), false),
        ("equivalent_static", json!(false), false),
        ("equivalent_static", json!(0), false),
        ("equivalent_static", Value::Null, true),
        ("notes", json!("free text"), true),
    ];
    let exact: Vec<(&str, Value, bool)> = vec![
        ("analysis_state", json!({"kind": "load_reference"}), false),
        ("pressure", json!({"value": 1000.0, "unit": "Pa"}), true),
    ];
    let want = |passes: bool| {
        let bound = if passes { json!({"ok": {"eligible": true}}) } else { json!({"gate": "G8", "code": PREPARATION}) };
        json!({"bound": bound, "unbound": {"ok": {"eligible": false}}, "transport": {"ok": {"eligible": false}}})
    };
    let mut misses = Vec::new();
    let mut checked = 0;
    let mut read = |label: String, source: &Value, invocation: &Value, rows: &[(&str, Value, bool)]| {
        assert_eq!(readings(source, invocation), want(true), "{label}: the base");
        for (key, value, passes) in rows {
            let mut invocation = invocation.clone();
            invocation["request"]["model"]["load_cases"][0][*key] = value.clone();
            let mut source = source.clone();
            source["retained_precision"]["body"]["invocation"]["value"] =
                domain_hash("source_blocks_invocation_v1", &invocation).unwrap().into();
            rehash(&mut source);
            let got = readings(&source, &invocation);
            if got != want(*passes) {
                misses.push(format!("{label}: {key} = {value}: got {got}, want {}", want(*passes)));
            }
            checked += 1;
        }
    };
    for (mode, text) in MILESTONES {
        let doc: Value = serde_json::from_str(text).unwrap();
        read(format!("milestone {mode}"), &doc["source"], &doc["invocation"], &preview);
    }
    let shared = corpus();
    for base in EXACT_BASES.into_iter().chain(M3X.map(|m| m.0)) {
        let (source, invocation) = exact_base(&shared, base);
        read(base.to_string(), &source, &invocation, &exact);
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
    assert_eq!(checked, 2 * 22 + 5 * 2);
}

// ---------------------------------------------------------------------------
// Snapshot 07n (B1 SC, I100; PLAN_v2 §2.5), appended to 07m: W-C2's two
// producer-solved bases, `d38_beside_selected`, the out-of-order-authored and
// SF-2 successors, and 290 entries. Every 07n entry states its bound expectation
// (per reader only in the declared class: Rust's own raw G7 code where Python and
// TS share one), its unbound read (`expected_unbound`, or
// `expected_unbound_by_reader` beside `expected_by_reader`) and its transport
// read (`expected_transport`), each "pass" (admitted, not eligible) or a gate and
// code; an admitted rewrite whose classes differ from its base's states
// `expected_classifications` (`shared_must_pass_entries_validate` reads it). Detail
// texts are not pinned (A-N1). I101 pins this reader's harness to it.

const N07_BASES: [&str; 9] = [
    "w_c2_sparse_interactive",
    "w_c2_dense_scrutiny",
    "d38_beside_selected",
    "cause_milestone_reversed_sparse_interactive",
    "cause_milestone_reversed_dense_scrutiny",
    "sf2_c_b_a_sparse_interactive",
    "sf2_c_b_a_dense_scrutiny",
    "sf2_a_a2_sparse_interactive",
    "sf2_a_a2_dense_scrutiny",
];
/// 07n's entries: the mutations after 07m's 294 and the must-pass entries after 07m's 28.
fn n07_entries(shared: &Value) -> Vec<Value> {
    let mut entries = shared["mutations"].as_array().unwrap()[294..534].to_vec();
    entries.extend(
        shared["must_pass"].as_array().unwrap()[28..78]
            .iter()
            .cloned(),
    );
    entries
}

/// 07n's counts and format: 26 cases (+9), 534 mutations (+240) and 78 must-pass
/// entries (+50), appended; the new keys on new entries only; the 45 per-reader
/// entries all in the declared class (Python and TS share the G7 expectation, Rust
/// gives its own raw G7 code, the unbound read the same per reader); 16 entries with
/// their own classes; 19 bases and 46 must-pass entries eligible.
#[test]
fn snapshot_07n_counts_and_format() {
    let shared = corpus();
    let ids = |key: &str| -> Vec<String> {
        shared[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| e["id"].as_str().unwrap().to_owned())
            .collect()
    };
    // 07o appends after 07n (`snapshot_07o_counts_and_format`); 07n is the prefix.
    let (cases, mutations, must_pass) = (ids("cases"), ids("mutations"), ids("must_pass"));
    let (cases, mutations, must_pass) = (&cases[..26], &mutations[..534], &must_pass[..78]);
    assert_eq!(cases[17..], N07_BASES);
    assert_eq!(mutations, &MUTATION_IDS[..534]);
    let all: std::collections::BTreeSet<&String> = mutations.iter().chain(must_pass).collect();
    assert_eq!(all.len(), 612, "entry ids are unique");
    let new_keys = [
        "expected_unbound",
        "expected_unbound_by_reader",
        "expected_transport",
        "expected_classifications",
    ];
    let old: Vec<&Value> = shared["mutations"].as_array().unwrap()[..294]
        .iter()
        .chain(&shared["must_pass"].as_array().unwrap()[..28])
        .collect();
    assert!(
        old.iter()
            .all(|e| new_keys.iter().all(|k| e.get(*k).is_none())),
        "07m's entries carry no 07n key"
    );
    let new = n07_entries(&shared);
    let allowed = [
        "id",
        "base",
        "edits",
        "invocation_edits",
        "rehash",
        "expected",
        "expected_by_reader",
        "expected_eligibility",
        "expected_classifications",
        "expected_unbound",
        "expected_unbound_by_reader",
        "expected_transport",
    ];
    for e in &new {
        let o = e.as_object().unwrap();
        assert!(
            o.keys().all(|k| allowed.contains(&k.as_str())),
            "{}",
            e["id"]
        );
        assert_eq!(e["rehash"], "all");
        assert!(e.get("expected_transport").is_some(), "{}", e["id"]);
        assert!(
            e.get("expected_unbound").is_some() != e.get("expected_unbound_by_reader").is_some(),
            "{}",
            e["id"]
        );
    }
    let per: Vec<&Value> = new
        .iter()
        .filter(|e| e.get("expected_by_reader").is_some())
        .collect();
    assert_eq!(per.len(), 45);
    for e in &per {
        let r = &e["expected_by_reader"];
        assert!(
            r["python"] == r["typescript"]
                && r["python"] == e["expected"]
                && r["rust"] != e["expected"]
                && e["expected"]["gate"] == "G7"
                && r["rust"]["gate"] == "G7"
                && e["expected_unbound_by_reader"] == *r,
            "{}: the declared class",
            e["id"]
        );
    }
    let classes = shared["must_pass"].as_array().unwrap()[28..78]
        .iter()
        .filter(|e| e.get("expected_classifications").is_some())
        .count();
    assert_eq!(classes, 16);
    let eligible = |key: &str, field: &str, n: usize| {
        shared[key].as_array().unwrap()[..n]
            .iter()
            .filter(|e| e[field]["numerical_eligible"] == true)
            .count()
    };
    assert_eq!(
        (
            eligible("cases", "expected", 26),
            eligible("must_pass", "expected_eligibility", 78)
        ),
        (19, 46)
    );
}

/// 07n's 240 mutations as one slice, observed by this reader against its own
/// expectation and tallied against a literal, as the earlier snapshots' slices are.
#[test]
fn snapshot_07n_mutation_outcomes() {
    slice_outcomes(
        "I101_OUTCOME_07N",
        294..534,
        &[
            ("G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", 1),
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 6),
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 15),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 73),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 17),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 1),
            ("G6 RETAINED_PRECISION_ROW_METHOD_MISMATCH", 1),
            ("G7 SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN", 4),
            ("G7 SOURCE_FORMULATION_BASIS_UNSUPPORTED", 3),
            ("G7 SOURCE_NUMERICAL_CASE_INVALID", 14),
            ("G7 SOURCE_NUMERICAL_QUALITY_INVALID", 9),
            ("G7 SOURCE_PREVIEW_PHYSICS_ARRAY_INVALID", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_CASE_SHAPE", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_DUPLICATE_CASE", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_DUPLICATE_ID", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED", 2),
            ("G7 SOURCE_PREVIEW_PHYSICS_EVIDENCE_SHAPE", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_EXTREMA_BASIS", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS", 2),
            ("G7 SOURCE_PREVIEW_PHYSICS_EXTREMA_SHAPE", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_EXTREMA_STATION", 2),
            ("G7 SOURCE_PREVIEW_PHYSICS_EXTREMA_SUBDIVISIONS", 3),
            ("G7 SOURCE_PREVIEW_PHYSICS_FOREIGN_METHOD_EVIDENCE", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_GATE_DUPLICATE", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_GATE_REASON", 2),
            ("G7 SOURCE_PREVIEW_PHYSICS_GATE_SHAPE", 2),
            ("G7 SOURCE_PREVIEW_PHYSICS_INTENSIFIED_RESULT_MISSING", 5),
            ("G7 SOURCE_PREVIEW_PHYSICS_INTENSIFIED_SHAPE", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_NUMBER_INVALID", 3),
            ("G7 SOURCE_PREVIEW_PHYSICS_STRESS_COVERAGE", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_STRESS_COVERAGE_PARTITION", 3),
            ("G7 SOURCE_PREVIEW_PHYSICS_STRING_INVALID", 1),
            (
                "G7 SOURCE_PREVIEW_PHYSICS_SUPPORT_ATTRIBUTION_CASE_MISMATCH",
                1,
            ),
            ("G7 SOURCE_PREVIEW_PHYSICS_SUPPORT_LISTED_TWICE", 2),
            ("G7 SOURCE_PREVIEW_PHYSICS_SUPPORT_WITHHELD_REASON", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_SUPPORT_WITHHELD_RECORD", 1),
            ("G8 RETAINED_PRECISION_INVOCATION_MISMATCH", 25),
            ("G8 RETAINED_PRECISION_PREPARATION_MISMATCH", 29),
        ],
    );
}

/// 07n: each entry's unbound read (no invocation) and transport read, as the corpus
/// states them for this reader: "pass" (admitted, not eligible) or a gate and code.
#[test]
fn snapshot_07n_unbound_and_transport_reads() {
    let shared = corpus();
    let read = |got: Result<rp::Validation, rp::ValidationError>| match got {
        Ok(v) => {
            assert!(!v.invocation_bound && !v.numerical_eligible);
            Value::from("pass")
        }
        Err(e) => serde_json::json!({"gate": e.gate, "code": e.code}),
    };
    let mut misses = Vec::new();
    let new = n07_entries(&shared);
    assert_eq!(new.len(), 290);
    for entry in &new {
        let (source, _) = apply_entry(&shared, entry);
        let unbound = entry
            .get("expected_unbound")
            .unwrap_or_else(|| &entry["expected_unbound_by_reader"]["rust"]);
        let got = read(rp::validate(&source, None));
        if got != *unbound {
            misses.push(format!("{} unbound: got {got} want {unbound}", entry["id"]));
        }
        let got = read(rp::validate_transport_metadata(&source));
        if got != entry["expected_transport"] {
            misses.push(format!(
                "{} transport: got {got} want {}",
                entry["id"], entry["expected_transport"]
            ));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

// ---- B2 (B2-C CONTRACT with REVISION_01 and REVISION_02): combinations ----

/// The committed W-CB3 combination successor documents (B2-P's live successors):
/// `(source, invocation)` for a mode.
fn b2_w_cb3(mode: &str) -> (Value, Value) {
    let text = match mode {
        "sparse_interactive" => include_str!("../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json"),
        _ => include_str!("../../../../fixtures/results/retained_precision_combination_successor_dense_scrutiny.json"),
    };
    let doc: Value = serde_json::from_str(text).unwrap();
    (doc["source"].clone(), doc["invocation"].clone())
}
const B2_MODES: [&str; 2] = ["sparse_interactive", "dense_scrutiny"];
/// S-6 from step `from` (4, 5 or 6) on: the hashes that depend on an edited
/// inner hash, leaving that inner hash as written (REVISION_01 §5.2's inner-hash
/// tests: each inner conjunct is then the only one a test can reach).
fn b2_reseal(source: &mut Value, from: u8) {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    let identity = |s: &Value| {
        let mut s = s.clone();
        s.as_object_mut().unwrap().remove("index");
        Value::from(domain_hash("retained_precision_source_mp_v2", &s).unwrap())
    };
    let body = &mut source["retained_precision"]["body"];
    if from <= 4 {
        let sources = body["sources"].clone();
        for s in body["sources"].as_array_mut().unwrap() {
            if s["owner"]["kind"] != "combination" {
                continue;
            }
            for o in s["operands"].as_array_mut().unwrap() {
                o["source_identity_sha256"] = identity(&sources[o["source_ref"].as_u64().unwrap() as usize]);
            }
        }
    }
    if from <= 5 {
        let sources = body["sources"].clone();
        for c in body["combinations"].as_array_mut().unwrap() {
            if c["disposition"] == "retained_selected" {
                c["source_identity_sha256"] = identity(&sources[c["source_ref"].as_u64().unwrap() as usize]);
            }
        }
    }
    let mut public = source.clone();
    public.as_object_mut().unwrap().remove("retained_precision");
    source["retained_precision"]["body"]["publication_sha256"] =
        domain_hash("retained_precision_publication_mp_v2", &public).unwrap().into();
    source["retained_precision"]["receipt_sha256"] =
        domain_hash("retained_precision_receipt_mp_v2", &source["retained_precision"]["body"]).unwrap().into();
}
/// One W-CB3 shape: `edits` on the source, `invocation_edits` on the invocation
/// (its digest rebound), the full rehash, then `inner` edits resealed by the S-6
/// steps after the edited hash's own (`b2_reseal`); the bound first failure, or
/// null when the statement validates.
fn b2_shape(mode: &str, edits: &[Value], invocation_edits: &[Value], inner: &[Value]) -> Value {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    let (mut source, mut invocation) = b2_w_cb3(mode);
    for e in edits {
        edit(&mut source, e);
    }
    for e in invocation_edits {
        edit(&mut invocation, e);
    }
    if !invocation_edits.is_empty() {
        source["retained_precision"]["body"]["invocation"]["value"] =
            domain_hash("source_blocks_invocation_v1", &invocation).unwrap().into();
    }
    rehash(&mut source);
    if !inner.is_empty() {
        for e in inner {
            edit(&mut source, e);
        }
        // The edited hash's S-6 step: 2 (a preparation hash), 4 (an operand
        // identity) or 5 (the combination identity); reseal the steps after it.
        let path = inner[0]["path"].to_string();
        let from = if path.contains("preparation") { 4 } else if path.contains("operands") { 5 } else { 6 };
        b2_reseal(&mut source, from);
    }
    observe_validation(rp::validate(&source, Some(&invocation)))
}

/// B2-P's committed W-CB3 successors (`retained_selected` with one operand
/// preparation), both modes, are read whole: bound, eligible, with the selected
/// case's classes then the combination's own G5c classes; unbound and on
/// transport never eligible; and the 07o rehash (S-6, steps 1-7) is a fixed
/// point of each.
#[test]
fn b2_w_cb3_successors_validate() {
    for mode in B2_MODES {
        let (source, invocation) = b2_w_cb3(mode);
        let got = rp::validate(&source, Some(&invocation)).unwrap_or_else(|e| panic!("{mode}: {e:?}"));
        assert!(got.invocation_bound && got.numerical_eligible, "{mode}");
        let rows = source["results"].as_array().unwrap();
        let expected: Vec<&Value> = rows
            .iter()
            .filter(|r| r["basis_ref"]["ref_id"] == "case:a")
            .chain(rows.iter().filter(|r| r["basis_ref"]["ref_type"] == "combination"))
            .map(|r| &r["id"])
            .collect();
        assert_eq!(got.classifications.iter().map(|c| Value::from(c.result_id.clone())).collect::<Vec<_>>().iter().collect::<Vec<_>>(), expected, "{mode}");
        assert!(got.classifications.iter().any(|c| c.basis_ref["ref_type"] == "combination" && c.scale_bits.is_some()), "{mode}: G5c classes the combination's rows");
        assert!(matches!(rp::validate(&source, None), Ok(v) if !v.numerical_eligible), "{mode}");
        assert!(matches!(rp::validate_transport_metadata(&source), Ok(v) if !v.numerical_eligible), "{mode}");
        let mut resealed = source.clone();
        rehash(&mut resealed);
        assert_eq!(resealed, source, "{mode}: the 07o rehash reseals the producer's receipt to itself");
    }
}

/// B2-C §10.1, every new check by gate, on W-CB3 (both modes): each edit reaches
/// its gate and code. Rows name CONTRACT §10.3's mutation where one is the
/// same edit. The `inner` rows are REVISION_01 §5.2's three inner-hash tests:
/// each inner hash edited and only the publication and receipt hashes resealed.
#[test]
fn b2_w_cb3_new_checks_first_failures() {
    let mut misses = Vec::new();
    for mode in B2_MODES {
        b2_w_cb3_rows(mode, &mut misses);
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}
fn b2_w_cb3_rows(mode: &str, misses: &mut Vec<String>) {
    use serde_json::json;
    let (source, _) = b2_w_cb3(mode);
    let body = &source["retained_precision"]["body"];
    let result_ids = body["combinations"][0]["result_ids"].as_array().unwrap().clone();
    let diagnostics = source["diagnostics"].as_array().unwrap();
    let selected_diagnostic = diagnostics.iter().position(|d| d["id"] == "diagnostic:retained-precision:combination:ab:selected").unwrap();
    let combination_row = source["results"].as_array().unwrap().iter().position(|r| r["basis_ref"]["ref_type"] == "combination").unwrap();
    // The combination's last row (a support moment magnitude, not hull-projected).
    let last_row = source["results"].as_array().unwrap().iter().rposition(|r| r["basis_ref"]["ref_type"] == "combination").unwrap();
    assert_eq!(source["results"][last_row]["kind"], "support_reaction_moment_magnitude_v2");
    let mut swapped_attempts = body["product_attempts"].clone();
    swapped_attempts.as_array_mut().unwrap().swap(0, 1);
    swapped_attempts[0]["id"] = json!(0);
    swapped_attempts[1]["id"] = json!(1);
    let mut duplicated = body["operand_preparations"].as_array().unwrap().clone();
    let mut copy = duplicated[0].clone();
    copy["id"] = json!(1);
    duplicated.push(copy);
    // m68: the operand-prepared CaseSource moved after the CombinationSource.
    let mut moved = body["sources"].as_array().unwrap().clone();
    let combination_source = moved.remove(2);
    moved.insert(1, combination_source);
    moved[1]["index"] = json!(1);
    moved[2]["index"] = json!(2);
    moved[1]["operands"][1]["source_ref"] = json!(2);
    // m20: the combination renamed to case A's id throughout (entry, evidence, rows,
    // diagnostics, its CombinationSource's owner).
    let mut renamed = vec![
        set(rb(json!(["combinations", 0, "basis_ref", "ref_id"])), json!("case:a")),
        set(json!(["contract_evidence", "combination_gates", 0, "combination_id"]), json!("case:a")),
        set(rb(json!(["sources", 2, "owner", "combination_id"])), json!("case:a")),
    ];
    for (i, r) in source["results"].as_array().unwrap().iter().enumerate() {
        if r["basis_ref"]["ref_type"] == "combination" {
            renamed.push(set(json!(["results", i, "basis_ref", "ref_id"]), json!("case:a")));
        }
    }
    for (i, d) in diagnostics.iter().enumerate() {
        if d["affected_refs"] == json!(["combination:ab"]) {
            renamed.push(set(json!(["diagnostics", i, "affected_refs"]), json!(["case:a"])));
        }
    }
    let swapped_operands = json!([body["calls"][1]["requested_operands"][1], body["calls"][1]["requested_operands"][0]]);
    let factor = |v: &str| json!(v);
    let wf = gate("G5", "RETAINED_PRECISION_WORK_MISMATCH");
    let rows: Vec<(&str, Vec<Value>, Vec<Value>, Vec<Value>, Value)> = vec![
        ("control", vec![], vec![], vec![], Value::Null),
        // G1: the three inner hashes (S-6 steps 2, 4, 5).
        ("inner: the operand-prepared CaseSource's preparation hash", vec![], vec![], vec![set(rb(json!(["sources", 1, "preparation", "sha256"])), json!("0".repeat(64)))], gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")),
        ("inner: a CombinationSource operand's identity", vec![], vec![], vec![set(rb(json!(["sources", 2, "operands", 1, "source_identity_sha256"])), json!("0".repeat(64)))], gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")),
        ("inner: the combination's identity", vec![], vec![], vec![set(rb(json!(["combinations", 0, "source_identity_sha256"])), json!("0".repeat(64)))], gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")),
        ("m8 a case attempt carries DEF-C's id", vec![set(rb(json!(["product_attempts", 0, "definition_id"])), json!(rp::COMBINATION_DEFINITION_ID))], vec![], vec![], gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")),
        ("m9 the combination attempt carries DEF-O's id", vec![set(rb(json!(["product_attempts", 1, "definition_id"])), json!(rp::DEFINITION_ID))], vec![], vec![], gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")),
        ("m65 the operand preparation carries DEF-C's id", vec![set(rb(json!(["operand_preparations", 0, "definition_id"])), json!(rp::COMBINATION_DEFINITION_ID))], vec![], vec![], gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")),
        ("m1 the combination attempt's id unknown", vec![set(rb(json!(["product_attempts", 1, "definition_id"])), json!("RP-UNKNOWN"))], vec![], vec![], gate("G0", UNSUPPORTED)),
        ("m2 the operand preparation's id unknown", vec![set(rb(json!(["operand_preparations", 0, "definition_id"])), json!("RP-UNKNOWN"))], vec![], vec![], gate("G0", UNSUPPORTED)),
        // G2.
        ("m16 a term's factor in uppercase hex", vec![set(rb(json!(["combinations", 0, "expression", "terms", 0, "factor"])), factor("3FF0000000000000"))], vec![], vec![], gate("G2", "RETAINED_PRECISION_ENCODING_MISMATCH")),
        ("m17 a requested operand's factor NaN", vec![set(rb(json!(["calls", 1, "requested_operands", 0, "factor"])), factor("7ff8000000000000"))], vec![], vec![], gate("G2", "RETAINED_PRECISION_ENCODING_MISMATCH")),
        // G3.
        ("(a) the gate entry names another combination", vec![set(json!(["contract_evidence", "combination_gates", 0, "combination_id"]), json!("combination:other"))], vec![], vec![], gate("G3", COVERAGE)),
        ("(b) m20 the combination renamed to a case id throughout", renamed, vec![], vec![], gate("G3", COVERAGE)),
        ("(c) m21 result_ids loses a row", vec![set(rb(json!(["combinations", 0, "result_ids"])), json!(result_ids[..result_ids.len() - 1]))], vec![], vec![], gate("G3", COVERAGE)),
        ("(c) m22 result_ids gains a case row", vec![set(rb(json!(["combinations", 0, "result_ids"])), json!([result_ids.clone(), vec![source["results"][0]["id"].clone()]].concat()))], vec![], vec![], gate("G3", COVERAGE)),
        ("(c) m23 two result_ids swapped", vec![set(rb(json!(["combinations", 0, "result_ids", 0])), result_ids[1].clone()), set(rb(json!(["combinations", 0, "result_ids", 1])), result_ids[0].clone())], vec![], vec![], gate("G3", COVERAGE)),
        ("(c) a combination row naming no entry (and in no result_ids)", vec![
            set(json!(["results", last_row, "basis_ref", "ref_id"]), json!("combination:other")),
            set(rb(json!(["combinations", 0, "result_ids"])), json!(result_ids[..result_ids.len() - 1])),
        ], vec![], vec![], gate("G3", COVERAGE)),
        ("(e) m26 the combination attempt before the case attempt", vec![set(rb(json!(["product_attempts"])), swapped_attempts), set(rb(json!(["cases", 0, "product_attempt_ref"])), json!(1)), set(rb(json!(["combinations", 0, "product_attempt_ref"])), json!(0)), set(rb(json!(["sources", 0, "preparation", "attempt_ref"])), json!(1))], vec![], vec![], gate("G3", COVERAGE)),
        ("(f) m24 execution_order loses the combination Run", vec![set(rb(json!(["work", "execution_order"])), json!([{"kind": "case", "index": 0}]))], vec![], vec![], gate("G3", COVERAGE)),
        ("(f) m25 the combination's execution index changed", vec![set(rb(json!(["work", "execution_order", 1, "index"])), json!(1))], vec![], vec![], gate("G3", COVERAGE)),
        ("(d) m27 the operand preparation's owner is the selected case", vec![set(rb(json!(["operand_preparations", 0, "owner_ref", "index"])), json!(0))], vec![], vec![], gate("G3", COVERAGE)),
        ("(d) requested_by names no combination", vec![set(rb(json!(["operand_preparations", 0, "requested_by"])), json!([1]))], vec![], vec![], gate("G3", COVERAGE)),
        ("(d, h) m30 the operand preparation duplicated", vec![set(rb(json!(["operand_preparations"])), json!(duplicated))], vec![], vec![], gate("G3", COVERAGE)),
        ("(d) m29 the operand preparation removed (its source too)", vec![
            remove(rb(json!(["operand_preparations"]))),
            remove(rb(json!(["sources", 1]))),
            set(rb(json!(["sources", 1, "index"])), json!(1)),
            set(rb(json!(["sources", 1, "operands", 1, "source_ref"])), json!(0)),
            set(rb(json!(["combinations", 0, "source_ref"])), json!(1)),
            set(rb(json!(["combinations", 0, "run", "origin", "source_ref"])), json!(1)),
            set(rb(json!(["product_attempts", 1, "source_ref"])), json!(1)),
            set(rb(json!(["calls", 1, "source_refs"])), json!([1])),
            set(rb(json!(["calls", 1, "requested_operands", 1, "source_ref"])), json!(0)),
            set(rb(json!(["groups", 1, "first_source_ref"])), json!(1)),
            set(rb(json!(["groups", 1, "source_refs"])), json!([1])),
        ], vec![], vec![], gate("G3", COVERAGE)),
        ("(g) the combination names the operand-prepared CaseSource", vec![set(rb(json!(["combinations", 0, "source_ref"])), json!(1))], vec![], vec![], gate("G3", COVERAGE)),
        ("(i) m68 the operand-prepared CaseSource after the CombinationSource", vec![
            set(rb(json!(["sources"])), json!(moved)),
            set(rb(json!(["operand_preparations", 0, "source_ref"])), json!(2)),
            set(rb(json!(["combinations", 0, "source_ref"])), json!(1)),
            set(rb(json!(["combinations", 0, "run", "origin", "source_ref"])), json!(1)),
            set(rb(json!(["product_attempts", 1, "source_ref"])), json!(1)),
            set(rb(json!(["calls", 1, "source_refs"])), json!([1])),
            set(rb(json!(["calls", 1, "requested_operands", 1, "source_ref"])), json!(2)),
            set(rb(json!(["groups", 1, "first_source_ref"])), json!(1)),
            set(rb(json!(["groups", 1, "source_refs"])), json!([1])),
        ], vec![], vec![], gate("G3", COVERAGE)),
        // G4.
        ("m31 the combination's selected diagnostic removed", vec![remove(json!(["diagnostics", selected_diagnostic]))], vec![], vec![], gate("G4", "RETAINED_PRECISION_DIAGNOSTIC_MISMATCH")),
        ("m34 its affected_refs name the combination and a case", vec![set(json!(["diagnostics", selected_diagnostic, "affected_refs"]), json!(["combination:ab", "case:a"]))], vec![], vec![], gate("G4", "RETAINED_PRECISION_DIAGNOSTIC_MISMATCH")),
        ("an unavailable diagnostic for the selected combination", vec![set(json!(["diagnostics", selected_diagnostic, "code"]), json!("RETAINED_PRECISION_UNAVAILABLE"))], vec![], vec![], gate("G4", "RETAINED_PRECISION_DIAGNOSTIC_MISMATCH")),
        // G5, ordinary class.
        ("disposition: the gate entry withheld", vec![set(json!(["contract_evidence", "combination_gates", 0, "withheld"]), json!(true)), set(json!(["contract_evidence", "combination_gates", 0, "reason"]), json!("NONLINEAR_COMBINATION_REQUIRES_SOLVE"))], vec![], vec![], gate("G5", ATTEMPT)),
        ("D6a: the combination's diagnostic_refs emptied", vec![set(rb(json!(["combinations", 0, "diagnostic_refs"])), json!([]))], vec![], vec![], gate("G5", ATTEMPT)),
        // G5, native class.
        ("m38 requested_operands swapped", vec![set(rb(json!(["calls", 1, "requested_operands"])), swapped_operands.clone())], vec![], vec![], gate("G5", ATTEMPT)),
        ("the terms' order: requested and CombinationSource operands both swapped", vec![
            set(rb(json!(["calls", 1, "requested_operands"])), swapped_operands.clone()),
            set(rb(json!(["sources", 2, "operands"])), json!([body["sources"][2]["operands"][1], body["sources"][2]["operands"][0]])),
            set(rb(json!(["sources", 2, "representative_source_ref"])), json!(1)),
        ], vec![], vec![], gate("G5", ATTEMPT)),
        ("the combination Run's cache_before drops an import", vec![set(rb(json!(["combinations", 0, "run", "cache_before"])), json!(body["combinations"][0]["run"]["cache_before"].as_array().unwrap()[1..]))], vec![], vec![], gate("G5", ATTEMPT)),
        ("m39 representative_source_ref -> operand 1's", vec![set(rb(json!(["sources", 2, "representative_source_ref"])), json!(1))], vec![], vec![], gate("G5", ATTEMPT)),
        ("m40 operands[0].case_index -> the other case", vec![set(rb(json!(["sources", 2, "operands", 0, "case_index"])), json!(1))], vec![], vec![], gate("G5", ATTEMPT)),
        ("m45 an import from the prepared operand", vec![set(rb(json!(["groups", 1, "imports", 0, "operand_index"])), json!(1))], vec![], vec![], gate("G5", ATTEMPT)),
        ("m42 the combination group's imports removed", vec![remove(rb(json!(["groups", 1, "imports"])))], vec![], vec![], gate("G5", ATTEMPT)),
        ("the mechanics Call names another combination", vec![set(rb(json!(["calls", 1, "owner_refs", 0, "index"])), json!(1))], vec![], vec![], gate("G5", ATTEMPT)),
        ("the ledger hash differs from the Selection's", vec![set(rb(json!(["sources", 2, "ledger_sha256"])), json!("0".repeat(64)))], vec![], vec![], gate("G5", ATTEMPT)),
        ("m43 the combination Call's before is not the batch's after", vec![set(rb(json!(["calls", 1, "invocation_before"])), json!(body["calls"][1]["invocation_before"].as_u64().unwrap() + 1))], vec![], vec![], wf.clone()),
        ("m44 charged is the batch Call's after", vec![set(rb(json!(["work", "charged"])), body["calls"][0]["invocation_after"].clone())], vec![], vec![], wf.clone()),
        // G5, products.
        ("the combination attempt's native failed beside a selected Run", vec![set(rb(json!(["product_attempts", 1, "stages", "native"])), json!("failed"))], vec![], vec![], gate("G5", PRODUCT)),
        ("m51 the operand preparation failed while prepared", vec![set(rb(json!(["operand_preparations", 0, "stage"])), json!("failed"))], vec![], vec![], gate("G5", PRODUCT)),
        // G5a-G5c on the combination owner.
        ("G5a: the combination's floor ratio", vec![set(rb(json!(["combinations", 0, "selection", "floor_ratio"])), json!("3dd0000000000001"))], vec![], vec![], gate("G5a", "RETAINED_PRECISION_SCALE_MISMATCH")),
        ("m55 a combination row listed not_covered", vec![set(rb(json!(["combinations", 0, "selection", "not_covered"])), json!([result_ids[1].clone()]))], vec![], vec![], gate("G5c", "RETAINED_PRECISION_CLASSIFICATION_MISMATCH")),
        // G6.
        ("m57 recovery_method removed from a combination row", vec![remove(json!(["results", combination_row, "recovery_method"]))], vec![], vec![], gate("G6", "RETAINED_PRECISION_ROW_METHOD_MISMATCH")),
        // G8, invocation.
        ("a model combination's factor 2", vec![], vec![set(json!(["request", "model", "combinations", 0, "terms", 1, "factor"]), json!(2.0))], vec![], gate("G8", INVOCATION)),
        ("the model's combination renamed", vec![], vec![set(json!(["request", "model", "combinations", 0, "id"]), json!("combination:other"))], vec![], gate("G8", INVOCATION)),
        ("the model's combinations removed", vec![], vec![remove(json!(["request", "model", "combinations"]))], vec![], gate("G8", INVOCATION)),
        ("the model's combination a subtraction", vec![], vec![set(json!(["request", "model", "combinations", 0]), json!({"id": "combination:ab", "basis": "result_state_subtraction", "minuend_id": "case:a", "subtrahend_id": "case:b"}))], vec![], gate("G8", INVOCATION)),
        // G8, preparation.
        ("m62 kernel_source_sha256 replaced", vec![set(rb(json!(["sources", 2, "kernel_source_sha256"])), json!("0".repeat(64)))], vec![], vec![], gate("G8", PREP)),
        ("m63 the operand CaseSource's effective wall", vec![set(rb(json!(["sources", 1, "section_terms", 0, "geometry", "effective_wall"])), json!("3f847ae147ae147c"))], vec![], vec![], gate("G8", PREP)),
        ("m64 the operand preparation's old D", vec![set(rb(json!(["operand_preparations", 0, "preparation", "members", 0, "old_facts", 0])), json!("3fc999999999999b"))], vec![], vec![], gate("G8", PREP)),
        ("the combination attempt's material basis", vec![set(rb(json!(["product_attempts", 1, "material_basis_ref"])), json!(1))], vec![], vec![], gate("G8", PREP)),
    ];
    for (name, edits, invocation_edits, inner, want) in &rows {
        let got = b2_shape(mode, edits, invocation_edits, inner);
        println!("B2_RS_SHAPE {mode} {name}: {got}");
        if got != *want {
            misses.push(format!("{mode} {name}: got {got}, want {want}"));
        }
    }
}

/// B2-C §4's reason table for a `retained_unavailable` combination with an
/// attempt, by the attempt's own error (reader logic).
#[test]
fn b2_combination_reason_table_rows() {
    use serde_json::json;
    let entry = |code: &str, phase: &str, terminal: &str| json!({"reason": {"code": code, "phase": phase}, "run": {"id": 1, "kernel_terminal": {"kind": terminal}}});
    let attempt = |error: Value| json!({"result": {"kind": "unavailable", "error": error}, "proof": {"checks": {"observables": {"kind": "failed", "error": {"kind": "observable", "cause": {"kind": "x"}}}}}});
    let ok = |e: Value, a: Value| rp::reader_logic::combination_reason_table(&e, &a).is_ok();
    assert!(ok(entry("combination_unresolved", "kernel", "unresolved"), attempt(json!({"kind": "native", "run_ref": 1}))));
    assert!(!ok(entry("combination_unresolved", "kernel", "unresolved"), attempt(json!({"kind": "native", "run_ref": 0}))), "native names its Run");
    assert!(!ok(entry("combination_unresolved", "kernel", "selected"), attempt(json!({"kind": "native", "run_ref": 1}))), "native with a selected Run");
    assert!(!ok(entry("kernel_unresolved", "kernel", "unresolved"), attempt(json!({"kind": "native", "run_ref": 1}))), "m53: a case code");
    assert!(ok(entry("combination_unresolved", "kernel", "refused"), attempt(json!({"kind": "capture", "cause": {"kind": "x"}}))));
    assert!(ok(entry("facade_certificate", "facade", "selected"), attempt(json!({"kind": "capture", "cause": {"kind": "x"}}))));
    assert!(ok(entry("facade_certificate", "facade", "selected"), attempt(json!({"kind": "observable", "cause": {"kind": "x"}}))));
    assert!(!ok(entry("facade_certificate", "facade", "selected"), attempt(json!({"kind": "observable", "cause": {"kind": "y"}}))), "the observables check's error");
    assert!(!ok(entry("facade_certificate", "facade", "unresolved"), attempt(json!({"kind": "proof", "cause": {}}))), "a facade error needs a selected Run");
    assert!(!ok(entry("facade_certificate", "kernel", "selected"), attempt(json!({"kind": "proof", "cause": {}}))));
    assert!(!ok(entry("combination_unresolved", "preparation", "unresolved"), attempt(json!({"kind": "preparation"}))), "a preparation error is refused");
}

/// B2-C §10.1 G5 products: each `retained_unavailable` entry's cause (reader
/// logic, on a two-case body: A selected, B as the row states).
#[test]
fn b2_combination_cause_rows() {
    use serde_json::json;
    let body = |b: Value, cause: Value, refs: Value, ops: Value, call: Value| {
        let mut body = json!({
            "cases": [{"basis_ref": {"ref_id": "case:a"}, "status": "selected", "source_ref": 0},
                      {"basis_ref": {"ref_id": "case:b"}, "status": b["status"], "source_ref": b["source_ref"], "run": b["run"]}],
            "combinations": [{"disposition": "retained_unavailable", "expression": {"kind": "mechanics", "terms": [{"case_id": "case:a"}, {"case_id": "case:b"}]},
                "reason": {"code": "combination_unresolved", "phase": "preparation", "cause": cause}}],
            "calls": [{}, call]});
        for (k, v) in refs.as_object().unwrap() {
            body["combinations"][0][k] = v.clone();
        }
        if !ops.is_null() {
            body["operand_preparations"] = ops;
        }
        body
    };
    let none = json!({"call_ref": null, "run": null, "source_ref": null, "product_attempt_ref": null});
    let unsourced = json!({"status": "unavailable", "source_ref": null, "run": null});
    let ledger = json!({"status": "unavailable", "source_ref": 1, "run": {"kernel_terminal": {"kind": "refused", "reason": {"tag": "ledger_unavailable"}}}});
    let rebuilt = json!({"status": "unavailable", "source_ref": 1, "run": {"kernel_terminal": {"kind": "unresolved", "reason": {"tag": "ceiling"}}}});
    let not_required = json!({"status": "not_required", "source_ref": null, "run": null});
    let refused = json!([{"owner_ref": {"index": 1}, "result": {"kind": "refused"}}]);
    let prepared = json!([{"owner_ref": {"index": 1}, "result": {"kind": "prepared"}}]);
    let osu = |i: u64| json!({"kind": "operand_source_unavailable", "operand_index": i});
    let opf = json!({"kind": "operand_preparation_failure", "operand_preparation_ref": 0});
    let reason = json!({"space": "combination", "tag": "operands_differ"});
    let pre = json!({"result": {"kind": "pre_source_refusal", "reason": reason}});
    let with_call = json!({"call_ref": 1, "run": null, "source_ref": null, "product_attempt_ref": null});
    let rows: Vec<(&str, Value, bool)> = vec![
        ("OSU at the unsourced term", body(unsourced.clone(), osu(1), none.clone(), Value::Null, json!({})), true),
        ("OSU at the ledger-refused term", body(ledger.clone(), osu(1), none.clone(), Value::Null, json!({})), true),
        ("m49 OSU naming the selected term", body(unsourced.clone(), osu(0), none.clone(), Value::Null, json!({})), false),
        ("OSU at a rebuilt (unavailable, sourced) term", body(rebuilt, osu(1), none.clone(), Value::Null, json!({})), false),
        ("OSU with a Call", body(unsourced.clone(), osu(1), with_call.clone(), Value::Null, json!({})), false),
        ("OPF at the refused record", body(not_required.clone(), opf.clone(), none.clone(), refused.clone(), json!({})), true),
        ("OPF at a prepared record", body(not_required.clone(), opf.clone(), none.clone(), prepared, json!({})), false),
        ("OPF where a term is unsourced", body(unsourced.clone(), opf.clone(), none.clone(), refused.clone(), json!({})), false),
        ("m48 a CombinationReason with no Call", body(not_required.clone(), reason.clone(), none.clone(), refused.clone(), json!({})), false),
        ("a CombinationReason with its pre-source Call", body(not_required.clone(), reason.clone(), with_call.clone(), Value::Null, pre.clone()), true),
        ("a CombinationReason not the Call's", body(not_required.clone(), json!({"space": "combination", "tag": "no_operands"}), with_call.clone(), Value::Null, pre.clone()), false),
        ("a no-Call cause in phase kernel", {
            let mut b = body(unsourced.clone(), osu(1), none.clone(), Value::Null, json!({}));
            b["combinations"][0]["reason"]["phase"] = json!("kernel");
            b
        }, false),
    ];
    for (name, b, want) in rows {
        assert_eq!(rp::reader_logic::combination_causes(&b).is_ok(), want, "{name}");
    }
}

/// R-COMB-1 (R) (B2-C §5; REVISION_01 §2, S-3; REVISION_02 §4.1): after the G5c
/// classes, every row of a `retained_unavailable` combination, and of an
/// `ordinary` one naming a case that is not `not_required`, in authored and then
/// publication order: quantity rows `not_covered`, record rows `non_quantity`,
/// `normalized_bits` from the value, `scale_bits` null. An `ordinary` combination
/// over `not_required` cases only, and a `retained_selected` or `base_withheld`
/// one, append nothing (reader logic).
#[test]
fn b2_r_comb_1_appended_classes() {
    use serde_json::json;
    let row = |id: &str, combination: &str, kind: &str, unit: &str, value: f64| json!({"id": id, "basis_ref": {"ref_type": "combination", "ref_id": combination}, "kind": kind, "unit": unit, "value": value});
    let source = json!({
        "results": [
            row("r:sub:1", "combination:sub", "global_nodal_displacement_x", "mm", 2.0),
            row("r:2b:1", "combination:2b", "global_nodal_displacement_x", "mm", 3.0),
            row("r:unavailable:1", "combination:unavailable", "element_local_axial_force", "kN", -1.5),
            row("r:sub:record", "combination:sub", "combination_modulus_basis_record", "1", 1.0),
            row("r:selected:1", "combination:selected", "global_nodal_displacement_x", "mm", 1.0),
        ],
        "retained_precision": {"body": {
            "cases": [{"basis_ref": {"ref_id": "case:a"}, "status": "selected"}, {"basis_ref": {"ref_id": "case:b"}, "status": "not_required"}],
            "combinations": [
                {"basis_ref": {"ref_type": "combination", "ref_id": "combination:sub"}, "disposition": "ordinary", "expression": {"kind": "result_state_subtraction", "minuend_id": "case:a", "subtrahend_id": "case:b"}},
                {"basis_ref": {"ref_type": "combination", "ref_id": "combination:2b"}, "disposition": "ordinary", "expression": {"kind": "mechanics", "terms": [{"case_id": "case:b"}]}},
                {"basis_ref": {"ref_type": "combination", "ref_id": "combination:unavailable"}, "disposition": "retained_unavailable", "expression": {"kind": "mechanics", "terms": [{"case_id": "case:a"}, {"case_id": "case:b"}]}},
                {"basis_ref": {"ref_type": "combination", "ref_id": "combination:selected"}, "disposition": "retained_selected", "expression": {"kind": "mechanics", "terms": [{"case_id": "case:a"}]}},
            ]}}});
    let got: Vec<(String, u64, Option<u64>, rp::AccuracyClass)> = rp::reader_logic::r_comb_1(&source)
        .into_iter()
        .map(|c| (c.result_id, c.normalized_bits, c.scale_bits, c.class))
        .collect();
    assert_eq!(got, vec![
        ("r:sub:1".to_owned(), 0.002f64.to_bits(), None, rp::AccuracyClass::NotCovered),
        ("r:sub:record".to_owned(), 1.0f64.to_bits(), None, rp::AccuracyClass::NonQuantity),
        ("r:unavailable:1".to_owned(), (-1500.0f64).to_bits(), None, rp::AccuracyClass::NotCovered),
    ]);
}

/// PR-B2 N-1 (the shared negative-value refusal): an extremum's
/// `global_upper_bound_pa` or `certified_gap_pa` below zero fails exactly where
/// a non-number fails today: on transport at G7 SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID
/// ("extrema numbers"), and bound or unbound in the base reader at G7
/// SOURCE_PREVIEW_PHYSICS_NUMBER_INVALID. +0 and -0 are accepted; no zero refusal.
#[test]
fn n1_negative_extrema_numbers_refused_zero_accepted() {
    use serde_json::json;
    let shared = corpus();
    let path = |k: &str| json!(["contract_evidence", "preview_cases", 0, "pipe_stress_extrema", 0, k]);
    let case = shared["cases"].as_array().unwrap().iter().find(|c| c["id"] == ORD).unwrap();
    for k in ["global_upper_bound_pa", "certified_gap_pa"] {
        for (value, want) in [
            (json!(-1.0), Some(("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID", "SOURCE_PREVIEW_PHYSICS_NUMBER_INVALID"))),
            (json!(-5e-324), Some(("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID", "SOURCE_PREVIEW_PHYSICS_NUMBER_INVALID"))),
            (json!("1"), Some(("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID", "SOURCE_PREVIEW_PHYSICS_NUMBER_INVALID"))),
            (json!(0.0), None),
            (json!(-0.0), None),
        ] {
            let mut source = case["source"].clone();
            edit(&mut source, &set(path(k), value.clone()));
            rehash(&mut source);
            let transport = rp::validate_transport_metadata(&source);
            let bound = rp::validate(&source, Some(&case["invocation"]));
            let unbound = rp::validate(&source, None);
            match want {
                Some((metadata, base)) => {
                    let t = transport.unwrap_err();
                    assert_eq!((t.gate, t.code.as_str(), t.detail.as_deref()), ("G7", metadata, Some(format!("{metadata}: extrema numbers").as_str())), "{k} {value}");
                    for e in [bound.unwrap_err(), unbound.unwrap_err()] {
                        assert_eq!((e.gate, e.code.as_str()), ("G7", base), "{k} {value}");
                    }
                }
                None => {
                    assert!(transport.is_ok() && bound.is_ok() && unbound.is_ok(), "{k} {value}: zero is accepted");
                }
            }
        }
    }
}

/// 07o (B2-C CONTRACT §10.2-§10.3 with REVISION_01 §5 and REVISION_02 §4; lane C's
/// corpus, appended after 07n): 19 bases (45), 69 mutations (603) and 19 must-pass
/// entries (97), ids unique; the format rule states S-6's seven steps; each 07o
/// must-pass entry's standing (`expected_eligibility.standing`) is this reader's.
#[test]
fn snapshot_07o_counts_and_format() {
    use open_pipe_stress_result_export::semantic_contract as sc;
    let shared = corpus();
    let ids = |key: &str| -> Vec<String> {
        shared[key].as_array().unwrap().iter().map(|e| e["id"].as_str().unwrap().to_owned()).collect()
    };
    let (cases, mutations, must_pass) = (ids("cases"), ids("mutations"), ids("must_pass"));
    assert_eq!((cases.len(), mutations.len(), must_pass.len()), (45, 603, 97));
    assert_eq!(mutations, MUTATION_IDS);
    let all: std::collections::BTreeSet<&String> = mutations.iter().chain(&must_pass).collect();
    assert_eq!(all.len(), 700, "entry ids are unique");
    let rule = shared["format_rule"]["order"].as_str().unwrap();
    for step in ["operand_preparation_ref", "retained_precision_operand_preparation_v1", "operands[*].source_identity_sha256", "retained_selected combination"] {
        assert!(rule.contains(step), "{step}");
    }
    for entry in &shared["must_pass"].as_array().unwrap()[78..] {
        let (source, invocation) = apply_entry(&shared, entry);
        let order: Vec<Value> = source["retained_precision"]["body"]["cases"].as_array().unwrap().iter().map(|c| c["basis_ref"].clone()).collect();
        let standing = sc::numerical_use_standing_with_context(&source, &order, Some(&invocation));
        println!("B2_RS_07O_STANDING {} {standing}", entry["id"]);
        // An entry stated refused bound (PR-B2 ruling 5) has no numerical-use standing.
        if entry["expected"] != "pass" {
            assert_eq!(standing, "unsupported", "{}", entry["id"]);
            continue;
        }
        // The corpus's `eligible` is this reader's `numerically_eligible`.
        let want = match entry["expected_eligibility"]["standing"].as_str().unwrap() {
            "eligible" => "numerically_eligible",
            other => other,
        };
        assert_eq!(standing, want, "{}", entry["id"]);
    }
}

/// 07o's 69 mutations as one slice, observed by this reader against its own
/// expectation (the designed first failure; m69 at Rust's own G7 base code) and
/// tallied against a literal.
#[test]
fn snapshot_07o_mutation_outcomes() {
    slice_outcomes(
        "B2_RS_OUTCOME_07O",
        534..603,
        &[
            ("G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", 7),
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 9),
            ("G2 RETAINED_PRECISION_ENCODING_MISMATCH", 2),
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 14),
            ("G4 RETAINED_PRECISION_DIAGNOSTIC_MISMATCH", 4),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 10),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 8),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 3),
            ("G5b RETAINED_PRECISION_SCALE_MISMATCH", 1),
            ("G5c RETAINED_PRECISION_CLASSIFICATION_MISMATCH", 1),
            ("G6 RETAINED_PRECISION_ROW_METHOD_MISMATCH", 3),
            ("G7 SOURCE_PREVIEW_PHYSICS_COMBINATION_MAGNITUDE", 1),
            ("G8 RETAINED_PRECISION_INVOCATION_MISMATCH", 3),
            ("G8 RETAINED_PRECISION_PREPARATION_MISMATCH", 3),
        ],
    );
}

/// PR-B2 ruling 5: 07o states the two hook-produced bases refused bound at G8
/// PREPARATION_MISMATCH (each hook records the refused member's old facts with
/// D = +0 against the invocation's OD, which B1's and C3a's old-fact binding
/// refuses), and passing unbound and on transport. Exactly those two bases and
/// their must-pass entries carry a refused statement; every 07o entry that states
/// an unbound or transport read is read so by this reader.
#[test]
fn snapshot_07o_unbound_and_transport_reads() {
    let shared = corpus();
    let refused: Vec<&Value> = shared["cases"].as_array().unwrap()[26..]
        .iter()
        .filter(|c| c["expected"].get("gate").is_some())
        .map(|c| &c["id"])
        .collect();
    assert_eq!(refused, ["b2_operand_preparation_failure", "b2_operand_source_unavailable"]);
    let stated: Vec<&Value> = shared["must_pass"].as_array().unwrap()[78..]
        .iter()
        .filter(|e| e["expected"] != "pass")
        .map(|e| &e["base"])
        .collect();
    assert_eq!(stated, refused);
    let read = |got: Result<rp::Validation, rp::ValidationError>| match got {
        Ok(v) => {
            assert!(!v.invocation_bound && !v.numerical_eligible);
            Value::from("pass")
        }
        Err(e) => serde_json::json!({"gate": e.gate, "code": e.code}),
    };
    let mut seen = 0;
    for entry in shared["mutations"].as_array().unwrap()[534..].iter().chain(&shared["must_pass"].as_array().unwrap()[78..]) {
        if entry.get("expected_unbound").is_none() && entry.get("expected_transport").is_none() {
            continue;
        }
        seen += 1;
        let (source, _) = apply_entry(&shared, entry);
        assert_eq!(read(rp::validate(&source, None)), entry["expected_unbound"], "{}", entry["id"]);
        assert_eq!(read(rp::validate_transport_metadata(&source)), entry["expected_transport"], "{}", entry["id"]);
    }
    assert_eq!(seen, 2);
}

/// PR-B2 lane X: the shared B2 parity probes, pinned alike in the RS, PY and TS
/// readers. Each shape is a forgery on a corpus base in the corpus entry grammar
/// (S-6 rehash), with the bound and unbound reading every reader gives: a gate
/// and code, or "pass" (rulings 1-3, a prepared operand preparation's
/// completeness, a refused Call's members, and UTF-8 range order).
#[test]
fn b2_parity_probes_shared() {
    let shared = corpus();
    let probes: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/retained_precision_b2_parity_probes.json"
    ))
    .unwrap();
    let read = |got: Result<rp::Validation, rp::ValidationError>| match got {
        Ok(_) => Value::from("pass"),
        Err(e) => serde_json::json!({"gate": e.gate, "code": e.code}),
    };
    let shapes = probes["shapes"].as_array().unwrap();
    assert_eq!(shapes.len(), 16);
    let mut misses = Vec::new();
    for shape in shapes {
        let (source, invocation) = apply_entry(&shared, shape);
        let bound = read(rp::validate(&source, Some(&invocation)));
        let unbound = read(rp::validate(&source, None));
        if bound != shape["expected"] || unbound != shape["expected_unbound"] {
            misses.push(format!("{}: bound {bound}, unbound {unbound}", shape["name"]));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}
