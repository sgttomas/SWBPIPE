//! Current-format parser controls use the actual public/captured deserializer.
use super::*;
use serde_json::{json, Value};

fn request() -> Value {
    json!({"model": {
        "schema_version": "0.8.0",
        "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:b7b"},
        "analysis_status": {"mechanics": "MODEL_INCOMPLETE", "rule_check": "RULE_INPUTS_INCOMPLETE", "professional_acceptance": "HUMAN_REVIEW_REQUIRED"},
        "nodes": [], "pipe_segments": [], "supports": []
    }})
}

fn contract() -> Value {
    json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"})
}

fn assert_decode_refused(raw: Value) {
    assert!(serde_json::from_value::<LinearStaticPreviewRequest>(raw.clone()).is_err());
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        assert!(source_receipt::CapturedInvocation::parse(raw.clone(), mode).is_err());
        assert!(run_linear_static_preview_value_with_mode(raw.clone(), mode).is_err());
        let invocation = json!({"request": &raw, "solver_mode": mode.as_str()});
        let request_id = "request:b7b".to_string();
        let context = RetainedHeadlessContext::from_borrowed_roots(&raw, &invocation, &request_id);
        assert!(run_linear_static_preview_value_with_retained_headless(
            raw.clone(), mode, context, RetainedByteLimit::new(0),
        ).is_err());
    }
}

#[test]
fn b7b_contract_absence_and_current_object_are_distinct_positive_controls() {
    let raw = request();
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(raw.clone()).unwrap();
    assert!(model_format::is_pressure_free(&parsed.model));
    assert!(source_receipt::CapturedInvocation::parse(raw, PreviewSolverMode::SparseInteractive).is_ok());
    let mut exact = request();
    exact["model"]["pressure_contract"] = contract();
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(exact.clone()).unwrap();
    assert!(!model_format::is_pressure_free(&parsed.model));
    assert_eq!(exact_admission::ExactContract::of(&parsed.model), Some(exact_admission::ExactContract::PressureV3));
    assert!(source_receipt::CapturedInvocation::parse(exact, PreviewSolverMode::DenseScrutiny).is_ok());
}

#[test]
fn b7b_authored_contract_null_array_scalar_and_malformed_fail_closed() {
    for value in [Value::Null, json!([]), json!(["3.0.0", "exact_pressure_v3", "euler_bernoulli"]),
        json!(false), json!(1), json!("exact_pressure_v3"),
        json!({"version": 3, "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}),
        json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli", "extra": 1}),
    ] {
        let mut raw = request();
        raw["model"]["pressure_contract"] = value;
        assert_decode_refused(raw);
    }
}

#[test]
fn b7b_contract_duplicate_keys_are_refused_on_the_actual_text_decoder() {
    let mut raw = request();
    raw["model"]["pressure_contract"] = contract();
    let encoded = serde_json::to_string(&raw).unwrap();
    for (key, value) in [("version", "3.0.0"), ("mode", "exact_pressure_v3"), ("shear_model", "euler_bernoulli")] {
        let old = format!("\"{key}\":\"{value}\"");
        let duplicate = encoded.replacen(&old, &format!("{old},{old}"), 1);
        let error = serde_json::from_str::<LinearStaticPreviewRequest>(&duplicate).unwrap_err().to_string();
        assert!(error.contains("duplicate field"), "{error}");
    }
}

#[test]
fn b7b_top_level_shear_is_rejection_only_even_when_null() {
    for value in [Value::Null, json!("euler_bernoulli"), json!(false), json!({})] {
        let mut raw = request();
        raw["model"]["shear_model"] = value;
        let error = serde_json::from_value::<LinearStaticPreviewRequest>(raw.clone()).unwrap_err().to_string();
        assert!(error.contains("SHEAR_MODEL_WITHOUT_PRESSURE_CONTRACT"), "{error}");
        assert_decode_refused(raw.clone());
        raw["model"]["pressure_contract"] = contract();
        let error = serde_json::from_value::<LinearStaticPreviewRequest>(raw.clone()).unwrap_err().to_string();
        assert!(error.contains("unknown field"), "{error}");
        assert_decode_refused(raw);
    }
}

#[test]
fn b7b_corrosion_allowance_is_an_authored_object_without_a_zero_default() {
    let section = json!({"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}});
    let absent: PipeSectionInput = serde_json::from_value(section.clone()).unwrap();
    assert!(absent.corrosion_allowance.is_none());
    let mut zero = section.clone();
    zero["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
    let explicit: PipeSectionInput = serde_json::from_value(zero).unwrap();
    assert_eq!(explicit.corrosion_allowance.unwrap().value.to_bits(), 0.0f64.to_bits());
    for value in [Value::Null, json!([]), json!([0.0, "m"]), json!(0), json!({"value": 0.0}), json!({"value": "0", "unit": "m"})] {
        let mut invalid = section.clone();
        invalid["corrosion_allowance"] = value;
        assert!(serde_json::from_value::<PipeSectionInput>(invalid.clone()).is_err());
        let mut raw = request();
        raw["model"]["pipe_segments"] = json!([{"id": "pipe:b7b", "from": "node:a", "to": "node:b", "material": "material:b7b", "section": invalid}]);
        assert_decode_refused(raw);
    }
}

#[test]
fn b7b_valid_retired_versions_keep_the_named_refusal() {
    for version in RETIRED_MODEL_SCHEMA_VERSIONS {
        let mut raw = request();
        raw["model"]["schema_version"] = json!(version);
        raw["model"]["pressure_contract"] = contract();
        let parsed: LinearStaticPreviewRequest = serde_json::from_value(raw).unwrap();
        let mut diagnostics = Vec::new();
        pressure_runtime::validate_profile(&parsed.model, &mut diagnostics);
        assert_eq!(diagnostics[0].code, MODEL_SCHEMA_VERSION_RETIRED);
    }
}

fn current_joint_request() -> Value {
    // Re-expression of invented authored data only; no user-model default.
    let mut raw: Value = serde_json::from_str(include_str!(
        "../../../validation/references/t4_u5a/requests/J3-0.request.json"
    )).unwrap();
    raw["model"]["schema_version"] = json!("0.8.0");
    raw["model"]["pressure_contract"] = contract();
    for pipe in raw["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
    }
    raw
}

fn public_codes(raw: Value) -> Vec<String> {
    run_linear_static_preview_value_with_mode(raw, PreviewSolverMode::SparseInteractive)
        .unwrap().diagnostics.into_iter().map(|diagnostic| diagnostic.code).collect()
}

/// T4-U6 (DEL-05-03 Design §5): `EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED`
/// is removed, so the former nonzero-allowance refusal is now an admission with
/// the D-5 geometry; every other allowance refusal, and its priority, stays.
#[test]
fn b7b_explicit_ca_refusals_preserve_unit_and_section_priority() {
    let base = current_joint_request();
    let variants = [
        (None, "EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED"),
        (Some(json!({"value": 0.0, "unit": "Pa"})), "UNIT_CONVERSION_UNAVAILABLE"),
        (Some(json!({"value": -0.001, "unit": "m"})), "PIPE_DIMENSION_INVALID"),
        (Some(json!({"value": 1.0, "unit": "m"})), "PIPE_DIMENSION_INVALID"),
    ];
    for (quantity, expected) in variants {
        let mut raw = base.clone();
        let section = raw["model"]["pipe_segments"][0]["section"].as_object_mut().unwrap();
        match quantity {
            Some(value) => { section.insert("corrosion_allowance".into(), value); }
            None => { section.remove("corrosion_allowance"); }
        }
        let codes = public_codes(raw);
        assert!(codes.iter().any(|code| code == expected), "{expected}: {codes:?}");
        assert!(!codes.iter().any(|code| code == "EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED"), "{codes:?}");
    }
    // A nonzero allowance is admitted: the case solves, no allowance refusal is
    // published, and the member's evidence carries the D-5 sections. Every
    // section takes it, so the region's pressure bores stay equal.
    let mut raw = base.clone();
    for pipe in raw["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["corrosion_allowance"] = json!({"value": 0.001, "unit": "m"});
    }
    let envelope = run_linear_static_preview_value_with_mode(raw.clone(), PreviewSolverMode::SparseInteractive).unwrap();
    assert!(!envelope.diagnostics.iter().any(|d| d.code == "EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED" || d.severity == "blocking"),
        "{:?}", envelope.diagnostics);
    assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED");
    let pipe = &raw["model"]["pipe_segments"][0];
    let section = &envelope.contract_evidence.as_ref().unwrap()["exact_cases"][0]["pipe_sections"]
        .as_array().unwrap().iter().find(|s| s["pipe_id"] == pipe["id"]).unwrap().clone();
    let wall = pipe["section"]["wall_thickness"]["value"].as_f64().unwrap();
    let mill = pipe["section"]["mill_tolerance"]["value"].as_f64().unwrap_or(0.0);
    assert_eq!(section["corrosion_allowance_m"].as_f64().unwrap().to_bits(), 0.001f64.to_bits());
    assert_eq!(section["nominal_section"]["wall_m"].as_f64().unwrap().to_bits(), wall.to_bits());
    assert_eq!(section["reduced_section"]["wall_m"].as_f64().unwrap().to_bits(), ((wall - mill) - 0.001).to_bits());
    let mut raw = base;
    raw["model"].as_object_mut().unwrap().remove("pressure_contract");
    let codes = public_codes(raw);
    assert!(codes.iter().any(|code| code == "CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT"), "{codes:?}");
}

#[test]
fn b7b_shear_required_and_unknown_have_named_public_refusals() {
    let mut raw = current_joint_request();
    raw["model"]["pressure_contract"].as_object_mut().unwrap().remove("shear_model");
    assert!(public_codes(raw).iter().any(|code| code == "EXACT_PRESSURE_SHEAR_MODEL_REQUIRED"));
    // T4-U8 (DEL-05-03 Design §5): EXACT_PRESSURE_SHEAR_MODEL_UNSUPPORTED is
    // removed; timoshenko_annulus_energy_v1 is admitted, no shear refusal.
    let mut raw = current_joint_request();
    raw["model"]["pressure_contract"]["shear_model"] = json!("timoshenko_annulus_energy_v1");
    assert!(!public_codes(raw).iter().any(|code| code.starts_with("EXACT_PRESSURE_SHEAR_MODEL")));
    let mut raw = current_joint_request();
    raw["model"]["pressure_contract"]["shear_model"] = Value::Null;
    assert!(public_codes(raw).iter().any(|code| code == "EXACT_PRESSURE_SHEAR_MODEL_REQUIRED"));
    let mut raw = current_joint_request();
    raw["model"]["pressure_contract"]["shear_model"] = json!("unknown");
    assert!(public_codes(raw).iter().any(|code| code == "PRESSURE_CONTRACT_UNSUPPORTED"));
}

#[test]
fn b7b_absent_contract_objective_connector_keeps_its_named_refusal() {
    let mut raw = current_joint_request();
    raw["model"].as_object_mut().unwrap().remove("pressure_contract");
    for pipe in raw["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"].as_object_mut().unwrap().remove("corrosion_allowance");
    }
    assert!(public_codes(raw).iter().any(|code| code == "PREVIEW_CONTRACT_VERSION_MISMATCH"));
}

/// B7b D3 (3ce082a9bb; last re-registered at e70c115764) registered the current source, so the
/// former interim fact (the changed source was unregistered and issued no permit) is replaced by
/// the current one. In the registered build: it is the registered profile (M = 12 GiB), its
/// binding is exact (a changed reviewed input or another build identity is Stale), the public
/// Direct route reports Registered and issues the capture permit for an admitted exact request,
/// and Headless stays refused at D1.0 (D-2). Any other build identity (another toolchain, target,
/// profile or RUSTFLAGS, as on a hosted runner) is unregistered, and by `retained_facade_tests`'
/// convention the same test is its control: the profile is Stale, the pure binding and identity
/// checks still hold, the Direct route reports Stale with the D1.1 build refusal and publishes the
/// plain ordinary bytes with no W1 run, and Headless stays refused at D1.0.
#[test]
fn b7b_current_source_is_registered_and_issues_a_permit() {
    use retained_memory::{bindings_hold, identity_match, LAYOUT_WITNESSES, READER_LAYOUTS};
    if !crate::retained_facade_tests::registered() {
        use crate::retained_facade_tests::{REGISTERED_IDENTITY, RETAINED_LIMIT};
        use retained_memory::D1Clause;
        assert_eq!(retained_memory::retained_profile(), Err(ProfileStatus::Stale), "an unregistered build is Stale");
        let compiled = option_env!("OPS_RETAINED_REVIEWED_INPUTS").expect("build.rs records the reviewed inputs");
        assert!(bindings_hold(LAYOUT_WITNESSES, Some(compiled), compiled, &READER_LAYOUTS, &READER_LAYOUTS));
        let last = compiled.chars().last().unwrap();
        let changed = format!("{}{}", &compiled[..compiled.len() - 1], if last == '0' { '1' } else { '0' });
        assert!(!bindings_hold(LAYOUT_WITNESSES, Some(&changed), compiled, &READER_LAYOUTS, &READER_LAYOUTS));
        assert_eq!(identity_match(Some(REGISTERED_IDENTITY), [REGISTERED_IDENTITY].into_iter()), Ok(0));
        assert_eq!(identity_match(Some("v1;another-build"), [REGISTERED_IDENTITY].into_iter()), Err(ProfileStatus::Stale));
        assert_eq!(identity_match(option_env!("OPS_RETAINED_BUILD_IDENTITY"), [REGISTERED_IDENTITY].into_iter()),
            Err(ProfileStatus::Stale), "this build's identity is not the registered one");
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let raw = crate::retained_facade_tests::m3x();
            let plain = serde_json::to_vec(&run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let ordinary = |publication: RetainedPublication| match publication {
                RetainedPublication::Ordinary(envelope) => serde_json::to_vec(&envelope).unwrap(),
                other => panic!("{mode:?}: {other:?}"),
            };
            let output = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode, RETAINED_LIMIT).unwrap();
            let admission = output.admission().unwrap();
            assert_eq!(admission.profile, ProfileStatus::Stale, "{mode:?}");
            assert_eq!(admission.law().refusal.and_then(|refusal| refusal.clause()), Some(D1Clause::Build), "{mode:?}: no permit");
            assert!(output.retained().is_none(), "{mode:?}: no W1 run");
            assert_eq!(ordinary(output.into_publication()), plain, "{mode:?}: the unchanged ordinary route");
            let invocation = json!({"request": &raw, "solver_mode": mode.as_str()});
            let request_id = "request:b7b".to_string();
            let context = RetainedHeadlessContext::from_borrowed_roots(&raw, &invocation, &request_id);
            let output = run_linear_static_preview_value_with_retained_headless(
                raw.clone(), mode, context, RETAINED_LIMIT,
            ).unwrap();
            assert_eq!(output.admission().unwrap().profile, ProfileStatus::Stale, "{mode:?}");
            assert_eq!(output.admission().unwrap().law().refusal.and_then(|refusal| refusal.clause()), Some(D1Clause::Caller),
                "{mode:?}: Headless is refused at D1.0");
            assert!(output.retained().is_none());
            assert_eq!(ordinary(output.into_publication()), plain, "{mode:?}: Headless publishes the ordinary route");
        }
        return;
    }
    let facts = retained_memory::retained_profile().expect("the current source is the registered profile");
    assert_eq!(facts.qualified_limit(), RetainedByteLimit::new(12_884_901_888));
    let compiled = option_env!("OPS_RETAINED_REVIEWED_INPUTS").expect("build.rs records the reviewed inputs");
    assert!(bindings_hold(LAYOUT_WITNESSES, Some(compiled), compiled, &READER_LAYOUTS, &READER_LAYOUTS));
    // Any one changed reviewed-input digest breaks the binding: the build would be Stale.
    let last = compiled.chars().last().unwrap();
    let changed = format!("{}{}", &compiled[..compiled.len() - 1], if last == '0' { '1' } else { '0' });
    assert!(!bindings_hold(LAYOUT_WITNESSES, Some(&changed), compiled, &READER_LAYOUTS, &READER_LAYOUTS));
    assert_eq!(identity_match(Some(facts.identity()), [facts.identity()].into_iter()), Ok(0));
    assert_eq!(identity_match(Some("v1;another-build"), [facts.identity()].into_iter()), Err(ProfileStatus::Stale));
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let raw = crate::retained_facade_tests::m3x();
        let output = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode, facts.qualified_limit()).unwrap();
        let admission = output.admission().unwrap();
        assert_eq!(admission.profile, ProfileStatus::Registered, "{mode:?}");
        assert_eq!(admission.law().refusal, None, "{mode:?}: the permit is issued");
        assert!(output.retained().is_some(), "{mode:?}: W1 ran under the permit");
        let invocation = json!({"request": &raw, "solver_mode": mode.as_str()});
        let request_id = "request:b7b".to_string();
        let context = RetainedHeadlessContext::from_borrowed_roots(&raw, &invocation, &request_id);
        let output = run_linear_static_preview_value_with_retained_headless(
            raw.clone(), mode, context, facts.qualified_limit(),
        ).unwrap();
        assert_eq!(output.admission().unwrap().profile, ProfileStatus::Registered, "{mode:?}");
        assert!(output.admission().unwrap().law().refusal.is_some(), "{mode:?}: Headless is refused at D1.0");
        assert!(output.retained().is_none());
        assert!(matches!(output.into_publication(), RetainedPublication::Ordinary(_)));
    }
}

#[test]
fn b7b_ca_zero_preserves_every_derived_section_bit_and_invalid_sum_is_refused() {
    let section = json!({"outside_diameter": {"value": 0.2, "unit": "m"},
        "wall_thickness": {"value": 0.01, "unit": "m"}, "mill_tolerance": {"value": 0.002, "unit": "m"}});
    let absent: PipeSectionInput = serde_json::from_value(section).unwrap();
    let mut explicit = absent.clone();
    explicit.corrosion_allowance = Some(Quantity { value: 0.0, unit: "m".into() });
    let before = derive_pipe_section(&absent, "pipe:b7b", &mut Vec::new()).unwrap();
    let after = derive_pipe_section(&explicit, "pipe:b7b", &mut Vec::new()).unwrap();
    for (a, b) in [(before.area, after.area), (before.internal_area, after.internal_area),
        (before.second_moment, after.second_moment), (before.torsion_constant, after.torsion_constant),
        (before.section_modulus, after.section_modulus), (before.torsion_radius, after.torsion_radius),
        (before.wall_thickness, after.wall_thickness)] {
        assert_eq!(a.to_bits(), b.to_bits());
    }
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.009] {
        explicit.corrosion_allowance.as_mut().unwrap().value = value;
        let mut diagnostics = Vec::new();
        assert!(derive_pipe_section(&explicit, "pipe:b7b", &mut diagnostics).is_none());
        assert_eq!(diagnostics[0].code, "PIPE_DIMENSION_INVALID");
    }
}

#[test]
fn b7b_load_reference_group_is_optional_as_a_whole_and_keys_on_presence() {
    let mut absent = request();
    absent["model"]["pressure_contract"] = contract();
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(absent.clone()).unwrap();
    assert!(!case_state::is_load_state(&parsed.model));
    let mut diagnostics = Vec::new();
    case_state::resolve::validate_document(&parsed.model, false, &mut diagnostics);
    assert!(diagnostics.is_empty());
    for value in [Value::Null, json!([])] {
        let mut partial = absent.clone();
        partial["model"]["reference_configurations"] = value;
        let parsed: LinearStaticPreviewRequest = serde_json::from_value(partial.clone()).unwrap();
        assert!(case_state::is_load_state(&parsed.model));
        assert!(carries_load_reference_state(&partial["model"]));
        let mut diagnostics = Vec::new();
        case_state::resolve::validate_document(&parsed.model, false, &mut diagnostics);
        assert!(!diagnostics.is_empty(), "a partial group cannot be ignored");
        partial["model"].as_object_mut().unwrap().remove("pressure_contract");
        let parsed: LinearStaticPreviewRequest = serde_json::from_value(partial).unwrap();
        let mut diagnostics = Vec::new();
        case_state::resolve::validate_document(&parsed.model, false, &mut diagnostics);
        assert_eq!(diagnostics[0].code, "LOAD_STATE_CONTRACT_VERSION_MISMATCH");
    }
}

#[test]
fn b7b_early_raw_group_selector_keeps_request_material_authored_presence() {
    let baseline = current_joint_request();
    for value in [Value::Null, json!([]), json!({})] {
        let mut request = baseline.clone();
        let mut material = request["model"]["materials"][0].clone();
        material["expansion_laws"] = value;
        request["materials"] = json!([material]);
        let typed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
        assert!(case_state::carries_fields(&typed.model));
        let mut diagnostics = Vec::new();
        case_state::resolve::validate_document(&typed.model, true, &mut diagnostics);
        assert!(diagnostics.iter().any(|finding| finding.code == "LOAD_STATE_REQUEST_MATERIALS_UNSUPPORTED"));
        request["model"]["load_cases"][0]["primitive_loads"] = json!([{
            "id": "load:b7b:support-motion", "category": "imposed_displacement", "direction": "UX",
            "dimension": "length", "magnitude": {"value": 0.0, "unit": "m"},
            "target": {"type": "support", "support": "support:b7b"}
        }]);
        // The closed target DTO refuses this malformed support primitive (it has no
        // `dof`); group selection never manufactures an accepted target variant.
        assert_decode_refused(request);
    }
}

#[test]
fn b7b_member_connector_ids_absence_and_empty_array_are_the_only_empty_forms() {
    let mut raw = request();
    raw["model"]["pressure_contract"] = contract();
    raw["model"]["load_cases"] = json!([{"id": "case:b7b", "primitive_loads": [], "pressure_regions": [{}]}]);
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(raw.clone()).unwrap();
    assert!(parsed.model.load_cases[0].pressure_regions.as_ref().unwrap()[0].member_connector_ids.is_none());
    raw["model"]["load_cases"][0]["pressure_regions"][0]["member_connector_ids"] = json!([]);
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(parsed.model.load_cases[0].pressure_regions.as_ref().unwrap()[0].member_connector_ids.as_deref(), Some(&[][..]));
    for value in [Value::Null, json!(false), json!(1), json!("connector:x"), json!({}), json!([null]), json!([1])] {
        raw["model"]["load_cases"][0]["pressure_regions"][0]["member_connector_ids"] = value;
        assert_decode_refused(raw.clone());
    }
}

/// B7b's one re-authoring of a copied lib-test model (manager decision; the
/// integration twin is `tests/common/current_model.rs`). A 0.5.0, 0.6.0 or 0.7.0 copy changes its
/// `schema_version` to 0.8.0 and nothing else (DEL-04-07 Design
/// `u4b-u5b-implementation.md` §1.4; DEL-02-01 Design `model-document-0.7.0.md` §2 and `model-document-0.8.0.md` §5). An older copy: `schema_version` becomes
/// 0.8.0; a declared v2 or v3 exact contract becomes the current
/// `3.0.0/exact_pressure_v3` object with `shear_model` `euler_bernoulli`; and, on
/// a contract-present document, every pipe section lacking one gains an explicit
/// zero `corrosion_allowance`. Nothing else changes: no physical or numerical
/// input, and no other contract (a retired-contract refusal control keeps its form).
pub(crate) fn reauthor_current_model(model: &mut Value) {
    if matches!(model["schema_version"].as_str(), Some("0.5.0" | "0.6.0" | "0.7.0")) {
        model["schema_version"] = json!(crate::MODEL_SCHEMA_VERSION);
        return;
    }
    if crate::is_retired_model_schema_version(model["schema_version"].as_str().unwrap_or("")) {
        model["schema_version"] = json!(crate::MODEL_SCHEMA_VERSION);
    }
    let declared = model.get("pressure_contract").and_then(|contract| {
        Some((contract.get("version")?.as_str()?, contract.get("mode")?.as_str()?))
    });
    if matches!(declared, Some(("2.0.0", "exact_straight_pressure_v2") | ("3.0.0", "exact_pressure_v3"))) {
        model["pressure_contract"] = contract();
    }
    if model.get("pressure_contract") == Some(&contract()) {
        for pipe in model["pipe_segments"].as_array_mut().into_iter().flatten() {
            let section = pipe["section"].as_object_mut().expect("pipe section object");
            section.entry("corrosion_allowance").or_insert(json!({"value": 0.0, "unit": "m"}));
        }
    }
}

/// [`reauthor_current_model`] applied to a copied request's `model`.
pub(crate) fn reauthor_current_request(mut request: Value) -> Value {
    reauthor_current_model(&mut request["model"]);
    request
}

#[test]
fn b7b_reauthoring_helper_changes_only_version_contract_and_allowance() {
    let mut raw = request();
    raw["model"]["schema_version"] = json!("0.3.0");
    raw["model"]["pressure_contract"] = json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"});
    raw["model"]["pipe_segments"] = json!([
        {"id": "pipe:a", "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}}},
        {"id": "pipe:b", "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"},
            "corrosion_allowance": {"value": 0.0, "unit": "mm"}}}]);
    let current = reauthor_current_request(raw.clone());
    let mut expected = raw.clone();
    expected["model"]["schema_version"] = json!("0.8.0");
    expected["model"]["pressure_contract"] = contract();
    expected["model"]["pipe_segments"][0]["section"]["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
    assert_eq!(current, expected);
    // Pressure-free documents gain only the version; a retired contract keeps its form.
    let mut free = request();
    free["model"]["schema_version"] = json!("0.2.0");
    free["model"]["pipe_segments"] = raw["model"]["pipe_segments"].clone();
    let mut expected = free.clone();
    expected["model"]["schema_version"] = json!("0.8.0");
    assert_eq!(reauthor_current_request(free), expected);
    let mut legacy = raw;
    legacy["model"]["pressure_contract"] = json!({"version": "1.0.0", "mode": "legacy_pressure_v1"});
    let current = reauthor_current_request(legacy.clone());
    assert_eq!(current["model"]["pressure_contract"], legacy["model"]["pressure_contract"]);
    assert!(current["model"]["pipe_segments"][0]["section"].get("corrosion_allowance").is_none());
    // A 0.5.0, 0.6.0 or 0.7.0 copy is re-authored by its version alone: nothing else moves,
    // even an incomplete exact document (its refusal stays the producer's to give).
    for version in ["0.5.0", "0.6.0", "0.7.0"] {
        let mut previous = legacy.clone();
        previous["model"]["schema_version"] = json!(version);
        previous["model"]["pressure_contract"] = contract();
        let mut expected = previous.clone();
        expected["model"]["schema_version"] = json!(crate::MODEL_SCHEMA_VERSION);
        assert_eq!(reauthor_current_request(previous), expected);
    }
}
