//! preview-physics-1 reader (T0R S1 sections 9 and 10). Actual producer bytes are
//! the positive controls; every tamper is an explicitly corrupted in-memory copy.
//! Fixture bytes are never rewritten. This proves consumer admission only.
use open_pipe_stress_result_export::{derivative as d, semantic_contract as s};
use serde_json::{json, Value};

fn fixture(name: &str) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/results")
        .join(name);
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}
fn invented() -> Value {
    fixture("preview_physics_invented_sparse.json")
}
fn bases(raw: &Value) -> Vec<Value> {
    raw["numerical_quality"]["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["basis_ref"].clone())
        .collect()
}
fn rows(raw: &mut Value) -> &mut Vec<Value> {
    raw["results"].as_array_mut().unwrap()
}
fn row<'a>(raw: &'a mut Value, id: &str) -> &'a mut Value {
    rows(raw)
        .iter_mut()
        .find(|r| r["id"] == id)
        .unwrap_or_else(|| panic!("{id}"))
}
fn case<'a>(raw: &'a mut Value, id: &str) -> &'a mut Value {
    raw["contract_evidence"]["preview_cases"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|c| c["load_case_id"] == id)
        .unwrap()
}
fn refused(raw: &Value, code: &str) {
    let error = s::for_source(raw).expect_err(code);
    assert!(error.contains(code), "expected {code}, got {error}");
    assert_eq!(s::numerical_use_standing(raw, &bases(raw)), "unsupported");
}
fn derive(raw: &Value) -> Result<Value, String> {
    let base: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"
    ))
    .unwrap();
    let model = raw["model_ref"].as_str().unwrap();
    let carrier = d::checksum(
        raw,
        "attested_headless_producer_carrier",
        d::reference("test_carrier", "preview-physics"),
    )?;
    let origin = json!({"origin_id":"preview-physics-consumer-test","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Retained producer bytes; consumer contract test only","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap()),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}

const MAX_A: &str = "result:elastic-maximum:10:load:L-100:10:pipe:P-120";
const MAX_B: &str = "result:elastic-maximum:10:load:L-200:10:pipe:P-120";
const CE: &str = "support:CE-120";
const COMB: &str = "combination:C-OPER-ALT";

/// Admits the gated combination and adds a consistent mechanics state for it:
/// the reader checks structure and representation only, not the sums.
fn with_admitted_combination() -> Value {
    let mut raw = invented();
    raw["contract_evidence"]["combination_gates"] =
        json!([{"combination_id":COMB,"withheld":false,"reason":null}]);
    raw["diagnostics"]
        .as_array_mut()
        .unwrap()
        .retain(|d| d["code"] != "NONLINEAR_COMBINATION_REQUIRES_SOLVE");
    let basis = json!({"ref_type":"combination","ref_id":COMB});
    let md = |component: &str, coordinate: &str| json!({"component":component,"coordinate_system":coordinate,"location":"node","basis":"explicit_user_linear_combination","sign_convention":"positive value follows explicit user linear combination of matching source result sign conventions"});
    let (ux, uy, uz) = (1.25_f64, 0.0_f64, -5.75_f64);
    let prefix = "result:combination:combination-C-OPER-ALT";
    let mut added = vec![
        json!({"id":format!("{prefix}:disp:node-N-140:ux"),"kind":"global_nodal_displacement_x","value":ux,"unit":"mm","entity_ref":"node:N-140","basis_ref":basis,"source_result_refs":["result:disp:node-N-140:ux","result:loadcase:load-L-200:disp:node-N-140:ux"],"metadata":md("nodal_displacement_x","global")}),
        json!({"id":format!("{prefix}:disp:node-N-140:uy"),"kind":"global_nodal_displacement_y","value":uy,"unit":"mm","entity_ref":"node:N-140","basis_ref":basis,"source_result_refs":["result:disp:node-N-140:uy","result:loadcase:load-L-200:disp:node-N-140:uy"],"metadata":md("nodal_displacement_y","global")}),
        json!({"id":format!("{prefix}:disp:node-N-140:uz"),"kind":"global_nodal_displacement_z","value":uz,"unit":"mm","entity_ref":"node:N-140","basis_ref":basis,"source_result_refs":["result:disp:node-N-140:uz","result:loadcase:load-L-200:disp:node-N-140:uz"],"metadata":md("nodal_displacement_z","global")}),
        json!({"id":format!("{prefix}:disp:node-N-140"),"kind":"displacement_magnitude","value":ux.hypot(uy).hypot(uz),"unit":"mm","entity_ref":"node:N-140","basis_ref":basis,"source_result_refs":[format!("{prefix}:disp:node-N-140:ux"),format!("{prefix}:disp:node-N-140:uy"),format!("{prefix}:disp:node-N-140:uz")]}),
    ];
    let values = [3.0, 4.0, 0.0, 0.0, 0.0, -100.0];
    for (c, v) in ["Fx", "Fy", "Fz", "Mx", "My", "Mz"].iter().zip(values) {
        added.push(json!({"id":format!("{prefix}:support-action:13:support:S-100:{c}"),"kind":"support_reaction_component_v2","value":v,"unit":if c.starts_with('F'){"N"}else{"N*m"},"entity_ref":"support:S-100","basis_ref":basis,"source_result_refs":[format!("result:support-action:10:load:L-100:13:support:S-100:{c}"),format!("result:support-action:10:load:L-200:13:support:S-100:{c}")],"metadata":md(c,"global")}));
    }
    added.push(json!({"id":format!("{prefix}:support-action:13:support:S-100:force_magnitude"),"kind":"support_reaction_force_magnitude_v2","value":5.0,"unit":"N","entity_ref":"support:S-100","basis_ref":basis,"metadata":md("force_magnitude","global")}));
    added.push(json!({"id":format!("{prefix}:support-action:13:support:S-100:moment_magnitude"),"kind":"support_reaction_moment_magnitude_v2","value":100.0,"unit":"N*m","entity_ref":"support:S-100","basis_ref":basis,"metadata":md("moment_magnitude","global")}));
    rows(&mut raw).extend(added);
    raw
}

#[test]
fn actual_producer_envelopes_admit_bind_and_derive() {
    for name in [
        "preview_physics_connected_sparse.json",
        "preview_physics_connected_dense.json",
        "preview_physics_invented_sparse.json",
        "preview_physics_invented_dense.json",
    ] {
        let raw = fixture(name);
        let (table, version) = s::for_source(&raw).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(
            table["semantic_contract_id"],
            s::PREVIEW_PHYSICS_ID,
            "{name}"
        );
        assert_eq!(version, "0.3.0");
        assert!(s::is_fresh_identity(
            raw["producer"]["semantic_contract_id"].as_str().unwrap()
        ));
        assert_eq!(s::standing_reason(&raw), None);
        assert_eq!(
            s::numerical_use_standing(&raw, &bases(&raw)),
            "numerically_eligible",
            "{name}"
        );
        let doc = derive(&raw).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(
            doc["result_envelope"]["contract_evidence"],
            raw["contract_evidence"]
        );
        let reloaded: Value = serde_json::from_slice(&serde_json::to_vec(&doc).unwrap()).unwrap();
        d::validate_document(&reloaded, &raw).unwrap();
        let mut forged = reloaded.clone();
        forged["result_envelope"]["contract_evidence"] =
            json!({"preview_cases":[],"combination_gates":[]});
        assert!(d::validate_document(&forged, &raw).is_err(), "{name}");
    }
}

#[test]
fn f1_positive_control_accepts_non_result_references() {
    // Hanger, DEC-046 and user SIF source-reference strings stay unresolved and accepted.
    let raw = invented();
    let refs: Vec<&str> = raw["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|d| d["affected_refs"].as_array().into_iter().flatten())
        .filter_map(Value::as_str)
        .collect();
    for expected in [
        "hanger",
        "DEC-046",
        "invented_user_entered_preview_no_code_table",
    ] {
        assert!(refs.contains(&expected), "{expected}");
    }
    assert!(s::for_source(&raw).is_ok());
    let mut dangling = raw.clone();
    dangling["diagnostics"][0]["affected_refs"]
        .as_array_mut()
        .unwrap()
        .push(json!("result:never-emitted"));
    refused(&dangling, "DANGLING_RESULT_REF");
    let mut headline = raw.clone();
    headline["summary"]["max_displacement"]["result_ref"] = json!("result:never-emitted");
    refused(&headline, "DANGLING_RESULT_REF");
}

#[test]
fn retired_kinds_and_codes_are_refused() {
    for kind in [
        "reaction_resultant",
        "open_formula_stress_summary",
        "component_user_stress_multiplier_review",
    ] {
        let mut raw = invented();
        let mut extra = raw["results"][0].clone();
        extra["id"] = json!("result:retired");
        extra["kind"] = json!(kind);
        rows(&mut raw).push(extra);
        refused(&raw, "RETIRED_KIND");
    }
    for code in [
        "COMPONENT_STRESS_MULTIPLIER_APPLIED",
        "COMBINATION_STRESS_SUMMARY_SKIPPED",
    ] {
        let mut raw = invented();
        let mut extra = raw["diagnostics"][0].clone();
        extra["id"] = json!("diagnostic:retired");
        extra["code"] = json!(code);
        raw["diagnostics"].as_array_mut().unwrap().push(extra);
        refused(&raw, "RETIRED_CODE");
    }
    let mut unknown = invented();
    rows(&mut unknown)[0]["kind"] = json!("unknown_future_quantity");
    refused(&unknown, "ROW_SIGNATURE");
}

#[test]
fn headline_must_govern_with_identity_ties_and_complete_coverage() {
    // Not governing: a smaller maximum, bound consistently otherwise.
    let mut raw = invented();
    let smaller = "result:elastic-maximum:10:load:L-100:10:pipe:P-100";
    let value = row(&mut raw, smaller)["value"].clone();
    raw["summary"]["max_open_formula_stress"] =
        json!({"value":value,"unit":"Pa","location_ref":"pipe:P-100","result_ref":smaller});
    refused(&raw, "HEADLINE_BINDING");

    // Tie: equal values in both cases; the smaller load-case id wins.
    let mut tie = invented();
    let a = row(&mut tie, MAX_A).clone();
    row(&mut tie, MAX_B)["value"] = a["value"].clone();
    let extrema_a = case(&mut tie, "load:L-100")["pipe_stress_extrema"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["result_id"] == MAX_A)
        .unwrap()
        .clone();
    for e in case(&mut tie, "load:L-200")["pipe_stress_extrema"]
        .as_array_mut()
        .unwrap()
    {
        if e["result_id"] == MAX_B {
            for k in [
                "value_lower_pa",
                "value_upper_pa",
                "global_upper_bound_pa",
                "certified_gap_pa",
            ] {
                e[k] = extrema_a[k].clone();
            }
        }
    }
    assert_eq!(
        tie["summary"]["max_open_formula_stress"]["result_ref"],
        MAX_A
    );
    s::for_source(&tie).unwrap();
    let mut wrong_tie = tie.clone();
    wrong_tie["summary"]["max_open_formula_stress"]["result_ref"] = json!(MAX_B);
    refused(&wrong_tie, "HEADLINE_BINDING");

    // Incomplete coverage: the headline must be withheld.
    let mut incomplete = invented();
    let pipe_max = "result:elastic-maximum:10:load:L-200:10:pipe:P-110";
    rows(&mut incomplete).retain(|r| r["id"] != pipe_max);
    let c = case(&mut incomplete, "load:L-200");
    c["pipe_stress_extrema"]
        .as_array_mut()
        .unwrap()
        .retain(|e| e["result_id"] != pipe_max);
    c["stress_maximum_coverage"] = json!({"complete":false,"unavailable_pipe_ids":["pipe:P-110"],"outside_domain_pipe_ids":[]});
    refused(&incomplete, "HEADLINE_PRESENCE");
    incomplete["summary"]["max_open_formula_stress"] = Value::Null;
    s::for_source(&incomplete).unwrap();
    // A member missing from the partition is refused even without a headline.
    let mut unpartitioned = incomplete.clone();
    case(&mut unpartitioned, "load:L-200")["stress_maximum_coverage"] =
        json!({"complete":true,"unavailable_pipe_ids":[],"outside_domain_pipe_ids":[]});
    refused(&unpartitioned, "STRESS_COVERAGE_PARTITION");
}

#[test]
fn maximum_outside_its_bounds_or_unbound_is_refused() {
    let mut raw = invented();
    let v = row(&mut raw, MAX_B)["value"].as_f64().unwrap();
    row(&mut raw, MAX_B)["value"] = json!(v * 1.5);
    refused(&raw, "EXTREMA_BOUNDS");
    let mut unbound = invented();
    case(&mut unbound, "load:L-200")["pipe_stress_extrema"]
        .as_array_mut()
        .unwrap()
        .retain(|e| e["result_id"] != MAX_B);
    refused(&unbound, "STRESS_COVERAGE_PARTITION");
}

#[test]
fn support_actions_withheld_records_and_magnitudes() {
    let mut raw = invented();
    row(
        &mut raw,
        "result:support-action:10:load:L-100:13:support:S-100:force_magnitude",
    )["value"] = json!(1.0e9);
    refused(&raw, "SUPPORT_MAGNITUDE");

    // Missing withheld record for the non-consuming constant effort.
    let mut missing = invented();
    case(&mut missing, "load:L-100")["support_attribution"]["withheld"] = json!([]);
    refused(&missing, "SUPPORT_WITHHELD_RECORD");

    // Zero-filled withheld support.
    let mut zero = invented();
    for c in [
        "Fx",
        "Fy",
        "Fz",
        "Mx",
        "My",
        "Mz",
        "force_magnitude",
        "moment_magnitude",
    ] {
        let mut filled = row(
            &mut zero,
            &format!("result:support-action:10:load:L-100:13:support:S-100:{c}"),
        )
        .clone();
        filled["id"] = json!(format!("result:support-action:10:load:L-100:14:{CE}:{c}"));
        filled["entity_ref"] = json!(CE);
        filled["value"] = json!(0.0);
        rows(&mut zero).push(filled);
    }
    refused(&zero, "WITHHELD_SUPPORT_ROW");

    // An attributed support with a missing component row.
    let mut partial = invented();
    rows(&mut partial)
        .retain(|r| r["id"] != "result:support-action:10:load:L-100:13:support:S-100:Mz");
    refused(&partial, "SUPPORT_COMPONENT_COVERAGE");

    // An ambiguous-attribution record requires its diagnostic, and hides the rows.
    let mut ambiguous = invented();
    let nl = "support:NL-140";
    for id in ["load:L-100", "load:L-200"] {
        let c = case(&mut ambiguous, id);
        c["support_attribution"]["attributed_support_ids"]
            .as_array_mut()
            .unwrap()
            .retain(|s| s != nl);
        c["support_attribution"]["withheld"]
            .as_array_mut()
            .unwrap()
            .push(json!({"support_id":nl,"reason":"SUPPORT_ACTION_ATTRIBUTION_WITHHELD"}));
    }
    rows(&mut ambiguous).retain(|r| {
        !(r["entity_ref"] == nl
            && (r["kind"].as_str().unwrap().starts_with("support_reaction_")
                || r["kind"] == "nonlinear_support_final_reaction"))
    });
    refused(&ambiguous, "SUPPORT_WITHHELD_RECORD");
    ambiguous["diagnostics"].as_array_mut().unwrap().push(json!({"id":"diagnostic:preview-physics:attribution:14:support:NL-140","code":"SUPPORT_ACTION_ATTRIBUTION_WITHHELD","severity":"warning","message":"hand-built","source":"core/product_physics","affected_refs":[nl,"node:N-140"]}));
    s::for_source(&ambiguous).unwrap();
    let mut leaked = ambiguous.clone();
    let reaction = invented()["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["entity_ref"] == nl && r["kind"] == "nonlinear_support_final_reaction")
        .unwrap()
        .clone();
    rows(&mut leaked).push(reaction);
    refused(&leaked, "WITHHELD_SUPPORT_ROW");
}

#[test]
fn intensified_rows_bind_their_measures() {
    let id = "result:intensified-bending:component-C-110:pipe-P-100:end-j";
    let mut raw = invented();
    let v = row(&mut raw, id)["value"].as_f64().unwrap();
    row(&mut raw, id)["value"] = json!(v * 1.0001);
    refused(&raw, "INTENSIFIED_VALUE");
    let mut unbound = invented();
    case(&mut unbound, "load:L-100")["intensified_measures"]
        .as_array_mut()
        .unwrap()
        .retain(|m| m["result_id"] != id);
    refused(&unbound, "INTENSIFIED_ROW_UNBOUND");
    let mut foreign = invented();
    row(&mut foreign, id)["source_result_refs"][0] =
        json!("result:loadcase:load-L-200:stress:pipe-P-100:end-j:bending-normal-y");
    refused(&foreign, "INTENSIFIED_SOURCE_BINDING");
}

#[test]
fn combinations_admitted_gated_and_magnitudes() {
    let raw = with_admitted_combination();
    s::for_source(&raw).unwrap();

    let mut gated = raw.clone();
    gated["contract_evidence"]["combination_gates"] = json!([{"combination_id":COMB,"withheld":true,"reason":"NONLINEAR_COMBINATION_REQUIRES_SOLVE"}]);
    refused(&gated, "GATED_COMBINATION_ROW");
    let mut unknown = raw.clone();
    unknown["contract_evidence"]["combination_gates"] = json!([]);
    refused(&unknown, "GATED_COMBINATION_ROW");

    let mut displacement = raw.clone();
    row(
        &mut displacement,
        "result:combination:combination-C-OPER-ALT:disp:node-N-140",
    )["value"] = json!(7.0);
    refused(&displacement, "COMBINATION_MAGNITUDE");
    let mut support = raw.clone();
    row(
        &mut support,
        "result:combination:combination-C-OPER-ALT:support-action:13:support:S-100:moment_magnitude",
    )["value"] = json!(100.5);
    refused(&support, "COMBINATION_MAGNITUDE");
    // A range envelope mode-selects and is exempt from recomputation.
    let mut range = support.clone();
    for r in rows(&mut range) {
        if r["basis_ref"]["ref_type"] == "combination" && r["metadata"].is_object() {
            r["metadata"]["basis"] = json!("explicit_user_range_envelope");
        }
    }
    s::for_source(&range).unwrap();

    for kind in [
        "pipe_elastic_normal_stress_maximum_v2",
        "component_equal_factor_intensified_bending_stress_v1",
    ] {
        let mut combined = raw.clone();
        let source = combined["results"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["kind"] == kind)
            .unwrap()
            .clone();
        let mut extra = source.clone();
        extra["id"] = json!(format!("result:combination:combination-C-OPER-ALT:{kind}"));
        extra["basis_ref"] = json!({"ref_type":"combination","ref_id":COMB});
        rows(&mut combined).push(extra);
        assert!(s::for_source(&combined).is_err(), "{kind} in a combination");
    }
    let mut dangling = raw.clone();
    row(
        &mut dangling,
        "result:combination:combination-C-OPER-ALT:disp:node-N-140:ux",
    )["source_result_refs"][0] = json!("result:never-emitted");
    refused(&dangling, "DANGLING_RESULT_REF");
}

#[test]
fn header_and_blocked_envelope_shape() {
    let mut no_evidence = invented();
    no_evidence
        .as_object_mut()
        .unwrap()
        .remove("contract_evidence");
    assert!(s::for_source_metadata(&no_evidence).is_err());
    let mut extra_key = invented();
    extra_key["contract_evidence"]["exact_cases"] = json!([]);
    refused(&extra_key, "EVIDENCE_SHAPE");
    let mut limitations = invented();
    limitations["formulation_basis"]["limitations"][0] = json!("changed");
    refused(&limitations, "FORMULATION_BASIS");
    let mut foreign = invented();
    foreign["source_block_recovery"] = json!({});
    assert!(s::for_source(&foreign).is_err());
    let mut case_missing = invented();
    case_missing["contract_evidence"]["preview_cases"]
        .as_array_mut()
        .unwrap()
        .pop();
    assert!(s::for_source(&case_missing).is_err());

    // A1 e: a sanitized blocked envelope (see blocked()).
    let mut blocked = blocked();
    blocked["numerical_quality"]["status"] = json!("not_assessed");
    blocked["numerical_quality"]["cases"] = json!([]);
    s::for_source(&blocked).unwrap();
    let requested = bases(&invented());
    assert_eq!(
        s::numerical_use_standing(&blocked, &requested),
        "needs_recompute"
    );
    let mut blocked_with_rows = blocked.clone();
    blocked_with_rows["results"] = invented()["results"].clone();
    blocked_with_rows["summary"]["component_stress_modifier_count"] =
        invented()["summary"]["component_stress_modifier_count"].clone();
    refused(&blocked_with_rows, "BLOCKED_ENVELOPE");
}

#[test]
fn precision_1_is_readable_but_never_fresh() {
    for name in [
        "precision_connected_ui_mechanics_sparse.json",
        "precision_connected_ui_mechanics_dense.json",
    ] {
        let raw = fixture(name);
        assert_eq!(
            s::for_source(&raw).unwrap().0["semantic_contract_id"],
            s::PRECISION_ID
        );
        assert!(!s::is_fresh_identity(s::PRECISION_ID));
        assert_eq!(
            s::standing_reason(&raw),
            Some(s::PRECISION_1_HISTORICAL_SEMANTICS)
        );
        assert_eq!(
            s::numerical_use_standing(&raw, &bases(&raw)),
            "needs_recompute",
            "{name}"
        );
    }
    for id in [s::PREVIEW_PHYSICS_ID, s::PHYSICS_ID, s::PHYSICS_SOURCE_ID] {
        assert!(s::is_fresh_identity(id));
    }
    assert_eq!(
        s::FRESH_IDENTITIES.len(),
        4,
        "T1 extends the set only when it activates its identities"
    );
    assert!(s::is_fresh_identity(
        "openpipestress.result_semantics/0.3.0/source-blocks-1"
    ));
    // Relabelling a fresh envelope as precision-1 does not make it Current.
    // A2 item 10: a relabelled envelope fails validation first, so it is unsupported.
    let mut relabelled = invented();
    relabelled["producer"]["semantic_contract_id"] = json!(s::PRECISION_ID);
    assert_eq!(
        s::numerical_use_standing(&relabelled, &bases(&relabelled)),
        "unsupported"
    );
}

#[test]
fn numerical_standing_progression_on_a_fresh_identity() {
    // The general-path checks formerly pinned on precision-1 (precision_contract.rs).
    let raw = invented();
    let b = bases(&raw);
    assert_eq!(s::numerical_use_standing(&raw, &b), "numerically_eligible");
    assert_eq!(s::numerical_use_standing(&raw, &[]), "needs_recompute");
    assert_eq!(s::numerical_use_standing(&raw, &b[..1]), "needs_recompute");
    assert_eq!(
        s::numerical_use_standing(&raw, &[b[0].clone(), b[0].clone()]),
        "needs_recompute"
    );
    let mut sensitive = raw.clone();
    sensitive["numerical_quality"]["status"] = json!("sensitive");
    sensitive["numerical_quality"]["cases"][0]["solve_quality"] = json!("sensitive");
    assert!(s::for_source(&sensitive).is_ok());
    assert_eq!(s::numerical_use_standing(&sensitive, &b), "needs_recompute");
    let mut unresolved = raw.clone();
    unresolved["numerical_quality"]["cases"][0]["evidence_refs"] = json!(["missing"]);
    assert_eq!(
        s::numerical_use_standing(&unresolved, &b),
        "needs_recompute"
    );
}

#[test]
fn rule_binding_refusal_is_limited_to_the_source_blocks_summary() {
    let raw = invented();
    for r in raw["results"].as_array().unwrap() {
        assert_eq!(s::rule_binding_refusal(&raw, r), None);
    }
}

#[test]
fn a1_withheld_notices_and_per_support_rows() {
    // A1 b: notice ids follow S1 section 8.
    let mut id = invented();
    for d in id["diagnostics"].as_array_mut().unwrap() {
        if d["code"] == "CONSTANT_EFFORT_NOT_CONSUMED" {
            d["id"] =
                json!("diagnostic:preview-physics:constant-effort-not-consumed:support:CE-120");
        }
    }
    refused(&id, "WITHHELD_NOTICE_ID");
    // A notice without a record in some case is refused (the other direction is
    // support_actions_withheld_records_and_magnitudes).
    let mut one_case = invented();
    case(&mut one_case, "load:L-200")["support_attribution"]["withheld"] = json!([]);
    refused(&one_case, "SUPPORT_WITHHELD_RECORD");
    // A per-support nonlinear row must name a support listed in that case.
    for kind in [
        "nonlinear_support_final_reaction",
        "nonlinear_support_final_displacement",
        "nonlinear_support_active_set_state_code",
    ] {
        let mut unlisted = invented();
        let mut extra = unlisted["results"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["kind"] == kind && r["basis_ref"]["ref_id"] == "load:L-100")
            .unwrap()
            .clone();
        extra["id"] = json!(format!("result:t0r-unlisted:{kind}"));
        extra["entity_ref"] = json!("support:never-listed");
        rows(&mut unlisted).push(extra);
        refused(&unlisted, "SUPPORT_ATTRIBUTION_MISSING");
    }
}

fn blocked() -> Value {
    let mut blocked = invented();
    blocked["status"]["mechanics"] = json!("MODEL_INCOMPLETE");
    blocked["results"] = json!([]);
    blocked["summary"]["max_open_formula_stress"] = Value::Null;
    blocked["summary"]["max_displacement"] = Value::Null;
    blocked["summary"]["component_stress_modifier_count"] = json!(0);
    blocked["contract_evidence"] = json!({"preview_cases":[],"combination_gates":[]});
    blocked["diagnostics"] = json!([{"id":"diagnostic:preview-physics:joint-equilibrium:13:component:C-1","code":"JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED","severity":"blocking","message":"hand-built blocked control","source":"core/product_physics","affected_refs":["component:C-1","pipe:P-1"]}]);
    blocked
}

#[test]
fn a1_blocked_envelope_keeps_items_2_and_3() {
    s::for_source(&blocked()).unwrap();
    let mut retired = blocked();
    retired["diagnostics"][0]["code"] = json!("COMPONENT_STRESS_MULTIPLIER_APPLIED");
    refused(&retired, "RETIRED_CODE");
    let mut result_ref = blocked();
    result_ref["diagnostics"][0]["affected_refs"]
        .as_array_mut()
        .unwrap()
        .push(json!("result:stress:pipe-P-1"));
    refused(&result_ref, "DANGLING_RESULT_REF");
    let mut headline_ref = blocked();
    headline_ref["summary"]["max_displacement"] = json!({"value":1.0,"unit":"mm","location_ref":"node:N-1","result_ref":"result:disp:node-N-1"});
    refused(&headline_ref, "DANGLING_RESULT_REF");
    let mut evidence = blocked();
    evidence["contract_evidence"]["combination_gates"] = json!([{"combination_id":"combination:C","withheld":true,"reason":"NONLINEAR_COMBINATION_REQUIRES_SOLVE"}]);
    refused(&evidence, "BLOCKED_ENVELOPE");
}

#[test]
fn a1_modifier_count_equals_intensified_rows() {
    let raw = invented();
    let rows_count = raw["results"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["kind"] == "component_equal_factor_intensified_bending_stress_v1")
        .count();
    assert!(rows_count > 0);
    assert_eq!(
        raw["summary"]["component_stress_modifier_count"],
        json!(rows_count)
    );
    let mut wrong = raw.clone();
    wrong["summary"]["component_stress_modifier_count"] = json!(rows_count + 1);
    refused(&wrong, "MODIFIER_COUNT");
    let mut absent = raw.clone();
    absent["summary"]
        .as_object_mut()
        .unwrap()
        .remove("component_stress_modifier_count");
    s::for_source(&absent).unwrap();
    let mut blocked_count = blocked();
    blocked_count["summary"]["component_stress_modifier_count"] = json!(1);
    refused(&blocked_count, "MODIFIER_COUNT");
}

#[test]
fn a1_diagnostic_without_affected_refs_is_allowed() {
    let mut raw = invented();
    let d = raw["diagnostics"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|d| d["code"] == "SPRING_HANGER_USER_DATA_REVIEWED")
        .unwrap();
    d.as_object_mut().unwrap().remove("affected_refs");
    s::for_source(&raw).unwrap();
    let mut null_refs = invented();
    null_refs["diagnostics"][0]["affected_refs"] = Value::Null;
    // A2 item 5: null is refused; so is an empty string entry.
    refused(&null_refs, "ARRAY_INVALID");
    let mut empty_entry = invented();
    empty_entry["diagnostics"][0]["affected_refs"] = json!([""]);
    refused(&empty_entry, "STRING_INVALID");
    let mut not_array = invented();
    not_array["diagnostics"][0]["affected_refs"] = json!("result:never-emitted");
    assert!(s::for_source(&not_array).is_err());
}

/// Shared id vector (T0R S6): actual producer output with non-ASCII and
/// supplementary-plane ids, read identically by the Rust, Python and TS readers.
/// Row ids use UTF-8 byte lengths with ':' between segments; diagnostic ids use
/// the concatenated ID() form. A character-count length is refused.
#[test]
fn shared_unicode_id_vector_is_admitted_and_byte_lengths_are_required() {
    let raw = fixture("preview_physics_unicode_ids_sparse.json");
    s::for_source(&raw).expect("shared unicode id vector is admitted");
    let ids: Vec<&str> = raw["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap())
        .collect();
    for expected in [
        "result:support-action:7:load:é:11:support:锚:Fx",
        "result:elastic-maximum:7:load:é:10:pipe:α-β",
        "result:elastic-maximum:9:load:𝔫:10:pipe:β-γ",
        "result:intensified-bending:component-ç:pipe-α-β:end-j",
        "result:loadcase:load-𝔫:intensified-bending:component-ç:pipe-β-γ:end-i",
    ] {
        assert!(ids.contains(&expected), "{expected}");
    }
    let diagnostics: Vec<&str> = raw["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["id"].as_str().unwrap())
        .collect();
    assert!(diagnostics
        .contains(&"diagnostic:preview-physics:constant-effort-not-consumed:13:support:ü-ce"));
    // Character-count lengths (é is 2 bytes, 𝔫 is 4) must be refused.
    let text = serde_json::to_string(&raw).unwrap();
    for (bytes, chars) in [
        (
            "result:elastic-maximum:7:load:é:",
            "result:elastic-maximum:6:load:é:",
        ),
        (
            "constant-effort-not-consumed:13:support:ü-ce",
            "constant-effort-not-consumed:12:support:ü-ce",
        ),
    ] {
        let tampered: Value = serde_json::from_str(&text.replace(bytes, chars)).unwrap();
        assert!(
            s::for_source(&tampered).is_err(),
            "character-count id admitted: {chars}"
        );
    }
}

/// A2 shared tamper vector: every reader applies the same RFC 6901 ops to the
/// same actual producer bases and gives the listed outcome.
#[test]
fn a2_shared_tamper_vector() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let read = |path: &str| -> Value {
        serde_json::from_str(&std::fs::read_to_string(root.join(path)).unwrap()).unwrap()
    };
    let vector = read("fixtures/results/preview_physics_tamper_vector.json");
    let variants = vector["variants"].as_array().unwrap();
    assert_eq!(variants.len(), 13);
    for variant in variants {
        let id = variant["id"].as_str().unwrap();
        let base = vector["bases"][variant["base"].as_str().unwrap()]
            .as_str()
            .unwrap();
        let mut source = read(base);
        for op in variant["ops"].as_array().unwrap() {
            let path = op["path"].as_str().unwrap();
            match op["op"].as_str().unwrap() {
                "replace" => {
                    *source
                        .pointer_mut(path)
                        .unwrap_or_else(|| panic!("{id}: {path}")) = op["value"].clone();
                }
                "remove" => {
                    let (parent, key) = path.rsplit_once('/').unwrap();
                    let key = key.replace("~1", "/").replace("~0", "~");
                    match source.pointer_mut(parent).unwrap() {
                        Value::Object(map) => {
                            assert!(map.remove(&key).is_some(), "{id}: {path}");
                        }
                        Value::Array(items) => {
                            items.remove(key.parse::<usize>().unwrap());
                        }
                        _ => panic!("{id}: {path}"),
                    }
                }
                "add" => {
                    let (parent, key) = path.rsplit_once('/').unwrap();
                    let key = key.replace("~1", "/").replace("~0", "~");
                    match source.pointer_mut(parent).unwrap() {
                        Value::Object(map) => {
                            map.insert(key, op["value"].clone());
                        }
                        Value::Array(items) if key == "-" => items.push(op["value"].clone()),
                        Value::Array(items) => {
                            items.insert(key.parse::<usize>().unwrap(), op["value"].clone())
                        }
                        _ => panic!("{id}: {path}"),
                    }
                }
                "reverse" => source
                    .pointer_mut(path)
                    .and_then(Value::as_array_mut)
                    .unwrap_or_else(|| panic!("{id}: {path}"))
                    .reverse(),
                other => panic!("{id}: unknown op {other}"),
            }
        }
        match variant["expect"].as_str().unwrap() {
            "accepted" => {
                s::for_source(&source).unwrap_or_else(|e| panic!("{id}: {e}"));
            }
            "refused" => assert!(s::for_source(&source).is_err(), "{id} was accepted"),
            other => panic!("{id}: unknown expectation {other}"),
        }
    }
}

#[test]
fn a2_attribution_identical_in_every_case_and_standing_order() {
    // A2 item 3: dropping a support consistently from one case alone is refused.
    let mut raw = invented();
    let support = "support:S-100";
    rows(&mut raw).retain(|r| {
        !(r["entity_ref"] == support
            && r["basis_ref"]["ref_id"] == "load:L-200"
            && r["kind"].as_str().unwrap().starts_with("support_reaction_"))
    });
    case(&mut raw, "load:L-200")["support_attribution"]["attributed_support_ids"]
        .as_array_mut()
        .unwrap()
        .retain(|s| s != support);
    refused(&raw, "SUPPORT_ATTRIBUTION_CASE_MISMATCH");
    // A2 item 10: a tampered historical source is unsupported, a valid one needs_recompute.
    let precision = fixture("precision_connected_ui_mechanics_sparse.json");
    assert_eq!(
        s::numerical_use_standing(&precision, &bases(&precision)),
        "needs_recompute"
    );
    let mut tampered = precision.clone();
    tampered["numerical_quality"]["value_representation"] = json!("decimal");
    assert_eq!(
        s::numerical_use_standing(&tampered, &bases(&precision)),
        "unsupported"
    );
}
