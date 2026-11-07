//! RT-A6 H1 (DEL-05-01 Work `hydrostatic-test.md` §4, §4.1, §6 item 3-4): the
//! explicit contents selection of a named source case, consumed once by the
//! maintained self-weight generator into a different new case, which then
//! states the selection as its own.
//!
//! Independent references. The header data of §6 (invented, not product
//! defaults): r_o = 0.1 m, t = 0.01 m, steel 7,850 kg/m^3, test contents
//! 1,000 kg/m^3 and the explicitly entered gravity 9.80665 m/s^2 (signed
//! -9.80665 along global Y here). On the pressure-free route the metal and
//! contents share the wall t: metal area pi(r_o^2 - r_i^2) = pi * 0.0019 m^2,
//! bore pi r_i^2 = pi * 0.0081 m^2, so w = (902800199/4000000) pi N/m. On the
//! exact route (DEL-05-03 D-5) the metal is on the nominal wall and the
//! contents on the pressure bore r_o - ((t - c_m) - c_a); with c_m = 0 and
//! c_a = 0.001 m the bore radius is 0.091 m, the mass pi (14.915 + 8.281) and
//! w = 23.196 * 9.80665 pi = (5686876335/25000000) pi N/m. The rationals are
//! evaluated here once in binary64; the comparison bound below is this test's
//! own statement of binary64 agreement between two evaluation orders, not a
//! product tolerance. A 6 m cantilever's root reaction is w L.
//!
//! Twins: a document whose section states 1,000 kg/m^3 and no use, and one
//! whose source case selects 1,000 kg/m^3 through a condition set, generate
//! bit-equal loads and solve to identical results, with intentionally
//! different provenance.
use open_pipe_stress_product_physics::operating_conditions::contents::{
    CASE_CONTENTS_CONTEXT_CONFLICT, CASE_CONTENTS_CONTEXT_MISSING, CONTENTS_SOURCE_METHOD,
};
use open_pipe_stress_product_physics::operating_conditions::preview_operating_conditions;
use open_pipe_stress_product_physics::self_weight::{
    generate_contents_aware_self_weight_operations, generate_self_weight_operations, inspect_applied_self_weight,
    AppliedSelfWeightState, ContentsAwareSelfWeightRequest, SelfWeightOperationPlan, SelfWeightRequestRefusal,
    SELF_WEIGHT_CONTENTS_NOT_CONSUMED, SELF_WEIGHT_CONTENTS_SOURCE_INVALID,
};
use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, MechanicsEnvelope, PreviewModel, PreviewSolverMode,
};
use serde_json::{json, Value};
use std::f64::consts::PI;

const PROV: &str = "invented_rt_a6_h1_contents_test_not_library_data";
const SOURCE: &str = "case:source";
const TARGET: &str = "case:weight";
const PIPE: &str = "pipe:a";
const LENGTH: f64 = 6.0;
const GRAVITY: f64 = -9.80665;
const HASH: &str = "sha256:0000000000000000000000000000000000000000000000000000000000000000";
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

/// The independent PF and EX weights (module docs).
fn reference_weight(exact: bool) -> f64 {
    if exact {
        5686876335.0 / 25000000.0 * PI
    } else {
        902800199.0 / 4000000.0 * PI
    }
}

fn assert_agrees(label: &str, actual: f64, reference: f64) {
    assert!(
        actual.is_finite() && (actual - reference).abs() <= 1e-14 * reference.abs(),
        "{label}: {actual:.17e} vs independent {reference:.17e}"
    );
}

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

/// A 6 m cantilever along X anchored at its root. `section_contents` is the
/// section's own contents density; `selected` adds the condition sets and the
/// source case selecting `HYDRO`.
fn document(exact: bool, section_contents: Option<f64>, selected: bool) -> Value {
    let mut section = json!({"outside_diameter": q(0.2, "m"), "wall_thickness": q(0.01, "m"),
        "material_density": q(7850.0, "kg/m^3")});
    if let Some(density) = section_contents {
        section["contents_density"] = q(density, "kg/m^3");
    }
    if exact {
        section["corrosion_allowance"] = q(0.001, "m");
    }
    let mut model = json!({
        "schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "invented:h1", "units": {"angle": "rad", "force": "N", "length": "m", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "root", "position": {"x": 0, "y": 0, "z": 0}, "provenance": PROV},
                  {"id": "tip", "position": {"x": LENGTH, "y": 0, "z": 0}, "provenance": PROV}],
        "materials": [{"id": "steel", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": q(2.0e11, "Pa"), "poisson_ratio": q(0.25, "1"), "shear_modulus": q(8.0e10, "Pa"), "provenance": PROV}],
        "pipe_segments": [{"id": PIPE, "from": "root", "to": "tip", "material": "steel", "y_reference": {"x": 0, "y": 1, "z": 0},
            "provenance": PROV, "section": section}],
        "supports": [{"id": "anchor", "family": "anchor", "node": "root", "provenance": PROV, "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]}],
        "load_cases": [], "combinations": []
    });
    if exact {
        model["pressure_contract"] = json!({"mode": "exact_pressure_v3", "version": "3.0.0", "shear_model": "euler_bernoulli"});
    }
    if selected {
        model["condition_sets"] = json!([
            {"id": "cs:hydro", "provenance": PROV, "conditions": [
                {"key": "HYDRO", "contents_density": q(1000.0, "kg/m^3")},
                {"key": "ALT", "contents_density": q(1000.0, "kg/m^3")},
                {"key": "DRY", "contents_density": q(0.0, "kg/m^3")}]},
            {"id": "cs:alt", "provenance": PROV, "conditions": [{"key": "HYDRO", "contents_density": q(1000.0, "kg/m^3")}]}
        ]);
        model["pipe_segments"][0]["condition_set_ref"] = json!("cs:hydro");
        model["load_cases"] = json!([case_shell(exact, SOURCE, Some("HYDRO"))]);
    }
    model
}

fn case_shell(exact: bool, id: &str, contents: Option<&str>) -> Value {
    let mut case = json!({"id": id, "label": id, "kind": "primitive_user_load", "status": "draft", "provenance": PROV, "primitive_loads": []});
    if let Some(key) = contents {
        case["condition_use"] = json!({"contents_density": key});
    }
    if exact {
        case["pressure_regions"] = json!([]);
    }
    case
}

fn typed(model: &Value) -> PreviewModel {
    serde_json::from_value(model.clone()).unwrap_or_else(|error| panic!("{error}: {model}"))
}

fn request(source: Option<&str>) -> Value {
    let mut request = json!({"case_id": TARGET, "label": "Invented hydrotest weight", "pipe_refs": [PIPE],
        "gravity": {"value": GRAVITY, "unit": "m/s^2", "axis": "global_y"}, "provenance": PROV, "source_model_hash": HASH});
    if let Some(source) = source {
        request["contents_source_case_ref"] = json!(source);
    }
    request
}

fn plan(model: &Value, source: Option<&str>) -> Result<SelfWeightOperationPlan, Vec<String>> {
    let parsed = ContentsAwareSelfWeightRequest::from_value(&request(source)).unwrap();
    generate_contents_aware_self_weight_operations(&typed(model), &parsed)
        .map_err(|found| found.into_iter().map(|d| d.code).collect())
}

/// The plan applied as the operation route applies it: the case shell (with
/// the exact route's explicit `pressure_regions: []`), then its primitives.
fn adopt(mut model: Value, plan: &SelfWeightOperationPlan) -> Value {
    let mut case: Value = serde_json::from_str(&plan.changes[0].after).unwrap();
    case["primitive_loads"] = Value::Array(plan.changes[1..].iter().map(|c| serde_json::from_str(&c.after).unwrap()).collect());
    if model.get("pressure_contract").is_some() {
        case["pressure_regions"] = json!([]);
    }
    model["load_cases"].as_array_mut().unwrap().push(case);
    model
}

fn record(model: &Value, case: &str) -> Value {
    let load = &model["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == case).unwrap()["primitive_loads"][0];
    serde_json::from_str(load["provenance"].as_str().unwrap()).unwrap()
}

fn intensity(model: &Value, case: &str) -> f64 {
    model["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == case).unwrap()["primitive_loads"][0]["magnitude"]["value"]
        .as_f64()
        .unwrap()
}

fn states(model: &Value) -> Vec<(String, AppliedSelfWeightState, Vec<String>)> {
    inspect_applied_self_weight(&typed(model), None)
        .into_iter()
        .map(|s| (s.case_id, s.state, s.diagnostics.into_iter().map(|d| d.code).collect()))
        .collect()
}

fn target_state(model: &Value) -> AppliedSelfWeightState {
    states(model).into_iter().find(|(case, ..)| case == TARGET).unwrap().1
}

fn solve(model: &Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let wire: LinearStaticPreviewRequest = serde_json::from_value(json!({"model": model, "materials": []})).unwrap();
    run_linear_static_preview_with_mode(wire, mode)
}

fn codes(envelope: &MechanicsEnvelope) -> Vec<String> {
    envelope.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect()
}

fn root_reaction_fy(envelope: &MechanicsEnvelope, case: &str) -> f64 {
    envelope
        .results
        .iter()
        .find(|r| {
            r.kind == "support_reaction_component_v2"
                && r.entity_ref == "anchor"
                && r.basis_ref.as_ref().is_some_and(|b| b.ref_id == case)
                && r.metadata.as_ref().is_some_and(|m| m.component == "Fy")
        })
        .unwrap_or_else(|| panic!("no FY reaction row for {case}: {:?}", envelope.results.iter().map(|r| (&r.kind, &r.entity_ref, &r.basis_ref, r.metadata.as_ref().map(|m| &m.component))).filter(|r| r.0.contains("reaction")).collect::<Vec<_>>()))
        .value
}

/// The contents document after generation, with the target adopted and the
/// unconsumed origin removed (the origin is provenance, not a dependency).
fn adopted(exact: bool) -> Value {
    let model = document(exact, None, true);
    let plan = plan(&model, Some(SOURCE)).unwrap();
    adopt(model, &plan)
}

fn without_origin(mut model: Value) -> Value {
    model["load_cases"].as_array_mut().unwrap().retain(|c| c["id"] != SOURCE);
    model
}

#[test]
fn contents_selected_weight_matches_the_independent_closed_form() {
    for exact in [false, true] {
        let model = adopted(exact);
        let value = intensity(&model, TARGET);
        assert_agrees(&format!("exact {exact}"), -value, reference_weight(exact));
        let evidence = record(&model, TARGET);
        let source = &evidence["contents_source"];
        assert_eq!(source["method"], CONTENTS_SOURCE_METHOD);
        assert_eq!(source["selected_by_case"], SOURCE);
        assert_eq!(source["origin_case"], SOURCE);
        assert_eq!(source["condition_set"], "cs:hydro");
        assert_eq!(source["condition_key"], "HYDRO");
        assert_eq!(source["normalized_contents_density_kg_per_m3"], json!(1000.0));
        assert_eq!(source["section_contents_density_superseded"], Value::Null);
        assert_eq!(evidence["mass_inputs"]["contents_density"], q(1000.0, "kg/m^3"));
        assert_eq!(evidence["contents_absent"], false);
        assert_eq!(evidence["normalized_dependencies"]["contents_source"], json!({"condition_set": "cs:hydro", "condition_key": "HYDRO"}));
        // The created target states the selection as its own; nothing else of
        // the source case is copied.
        let target = model["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == TARGET).unwrap();
        assert_eq!(target["condition_use"], json!({"contents_density": "HYDRO"}));
        assert!(target.get("test_condition").is_none() && target.get("pressure_regions").map_or(true, |r| r == &json!([])));
        assert_eq!(target_state(&model), AppliedSelfWeightState::Fresh, "exact {exact}");
        // The mechanics: one physical mass, the root reaction w L, in both solver modes.
        let solved = without_origin(model);
        for mode in MODES {
            let envelope = solve(&solved, mode);
            assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED", "exact {exact} {mode:?}: {:?}", envelope.diagnostics);
            // Statics of the solved system (equilibrium of the factored
            // stiffness solve), a looser bound than the generation's.
            let reaction = root_reaction_fy(&envelope, TARGET).abs();
            let expected = reference_weight(exact) * LENGTH;
            assert!((reaction - expected).abs() <= 1e-9 * expected, "reaction exact {exact} {mode:?}: {reaction} vs {expected}");
        }
    }
}

#[test]
fn contents_twins_equal_section_density_twins_with_distinct_provenance() {
    for exact in [false, true] {
        // The section-density twin: no condition use, the section states 1,000 kg/m^3.
        let section_twin = {
            let model = document(exact, Some(1000.0), false);
            let plan = plan(&model, None).unwrap();
            adopt(model, &plan)
        };
        // The contents twin, also with a different section density, which the
        // explicit selection supersedes (recorded, never consumed).
        let contents_twin = {
            let mut model = document(exact, Some(400.0), true);
            let plan = plan(&model, Some(SOURCE)).unwrap();
            model = adopt(model, &plan);
            without_origin(model)
        };
        assert_eq!(intensity(&section_twin, TARGET).to_bits(), intensity(&contents_twin, TARGET).to_bits(), "exact {exact}");
        let (a, b) = (record(&section_twin, TARGET), record(&contents_twin, TARGET));
        assert_eq!(a["mass_kg_per_m"], b["mass_kg_per_m"]);
        assert!(a.get("contents_source").is_none());
        assert_eq!(b["contents_source"]["section_contents_density_superseded"], q(400.0, "kg/m^3"));
        assert_ne!(a["normalized_dependencies"], b["normalized_dependencies"], "the sources are distinct");
        for mode in MODES {
            let (x, y) = (solve(&section_twin, mode), solve(&contents_twin, mode));
            assert_eq!(x.status.mechanics, "MECHANICS_SOLVED", "{:?}", x.diagnostics);
            assert_eq!(y.status.mechanics, "MECHANICS_SOLVED", "{:?}", y.diagnostics);
            let rows = |e: &MechanicsEnvelope| serde_json::to_value(&e.results).unwrap();
            assert_eq!(rows(&x), rows(&y), "exact {exact} {mode:?}: the twins' mechanics agree");
            assert_eq!(serde_json::to_value(&x.numerical_quality).unwrap()["status"], serde_json::to_value(&y.numerical_quality).unwrap()["status"]);
        }
    }
}

#[test]
fn explicit_zero_is_an_authored_density_and_absence_keeps_the_legacy_branch() {
    // Selected zero: contents present at 0, the metal mass only.
    let mut model = document(false, None, true);
    model["load_cases"][0]["condition_use"]["contents_density"] = json!("DRY");
    let zero = adopt(model.clone(), &plan(&model, Some(SOURCE)).unwrap());
    let evidence = record(&zero, TARGET);
    assert_eq!(evidence["contents_absent"], false);
    assert_eq!(evidence["mass_inputs"]["contents_density"], q(0.0, "kg/m^3"));
    let metal_only = (PI * 0.01) * (0.2 - 0.01) * 7850.0;
    assert_agrees("zero contents", evidence["mass_kg_per_m"].as_f64().unwrap(), metal_only);
    // Absence: the legacy plan, byte for byte the closed request's.
    let legacy = document(false, None, false);
    let closed: open_pipe_stress_product_physics::self_weight::SelfWeightRequest = serde_json::from_value(request(None)).unwrap();
    let direct = serde_json::to_value(generate_self_weight_operations(&typed(&legacy), &closed).unwrap()).unwrap();
    let wrapped = serde_json::to_value(plan(&legacy, None).unwrap()).unwrap();
    assert_eq!(direct, wrapped);
    let legacy_evidence = &wrapped["source_evidence"][0];
    assert!(legacy_evidence.get("contents_source").is_none());
    assert!(legacy_evidence["normalized_dependencies"].get("contents_source").is_none());
    assert_eq!(legacy_evidence["contents_absent"], true);
    assert!(serde_json::from_str::<Value>(wrapped["changes"][0]["after"].as_str().unwrap()).unwrap().get("condition_use").is_none());
    assert_eq!(legacy_evidence["mass_kg_per_m"].as_f64().unwrap().to_bits(), evidence["mass_kg_per_m"].as_f64().unwrap().to_bits());
}

#[test]
fn selected_dependencies_stale_and_unrelated_or_origin_edits_do_not() {
    for exact in [false, true] {
        let base = adopted(exact);
        // Save and reopen: the JSON text round trip keeps the target fresh.
        let reopened: Value = serde_json::from_str(&serde_json::to_string(&base).unwrap()).unwrap();
        assert_eq!(target_state(&reopened), AppliedSelfWeightState::Fresh);
        let edit = |change: &dyn Fn(&mut Value)| {
            let mut model = reopened.clone();
            change(&mut model);
            target_state(&model)
        };
        let target = |m: &mut Value| -> usize { m["load_cases"].as_array().unwrap().iter().position(|c| c["id"] == TARGET).unwrap() };
        // The target's own selection and its source: stale, equal densities included.
        assert_eq!(edit(&|m| { let i = target(m); m["load_cases"][i]["condition_use"]["contents_density"] = json!("ALT"); }), AppliedSelfWeightState::Stale, "same density, another key");
        assert_eq!(edit(&|m| m["pipe_segments"][0]["condition_set_ref"] = json!("cs:alt")), AppliedSelfWeightState::Stale, "same density, another set");
        assert_eq!(edit(&|m| m["condition_sets"][0]["conditions"][0]["contents_density"] = q(1100.0, "kg/m^3")), AppliedSelfWeightState::Stale);
        assert_eq!(edit(&|m| { let i = target(m); m["load_cases"][i].as_object_mut().unwrap().remove("condition_use"); }), AppliedSelfWeightState::Stale, "use removed");
        // The same authored value written differently is the same density.
        assert_eq!(edit(&|m| m["condition_sets"][0]["conditions"][0]["contents_density"] = json!({"unit": "kg/m^3", "value": 1e3})), AppliedSelfWeightState::Fresh);
        // The origin and unrelated sets, keys and section contents: no physical dependency.
        assert_eq!(edit(&|m| m["load_cases"][0]["condition_use"]["contents_density"] = json!("DRY")), AppliedSelfWeightState::Fresh, "origin edited");
        assert_eq!(edit(&|m| { m["load_cases"].as_array_mut().unwrap().remove(0); }), AppliedSelfWeightState::Fresh, "origin removed");
        assert_eq!(edit(&|m| m["condition_sets"][1]["conditions"][0]["contents_density"] = q(2000.0, "kg/m^3")), AppliedSelfWeightState::Fresh, "unrelated set");
        assert_eq!(edit(&|m| m["condition_sets"][0]["conditions"][1]["contents_density"] = q(2000.0, "kg/m^3")), AppliedSelfWeightState::Fresh, "unrelated key");
        assert_eq!(edit(&|m| m["pipe_segments"][0]["section"]["contents_density"] = q(300.0, "kg/m^3")), AppliedSelfWeightState::Fresh, "superseded section density");
        // A missing selected value is invalid by name, never a section-density fallback.
        let mut missing = reopened.clone();
        missing["condition_sets"][0]["conditions"][0].as_object_mut().unwrap().remove("contents_density");
        let (_, state, found) = states(&missing).into_iter().find(|(c, ..)| c == TARGET).unwrap();
        assert_eq!(state, AppliedSelfWeightState::Invalid);
        assert!(found.contains(&"OPERATING_CONDITION_VALUE_MISSING".to_string()), "{found:?}");
        // The refresh keeps the identity and the origin, and resolves the target's own selection.
        let mut drifted = reopened.clone();
        drifted["condition_sets"][0]["conditions"][0]["contents_density"] = q(1100.0, "kg/m^3");
        let status = inspect_applied_self_weight(&typed(&drifted), Some(&format!("sha256:{}", "1".repeat(64))))
            .into_iter()
            .find(|s| s.case_id == TARGET)
            .unwrap();
        let replacement = status.replacement.unwrap();
        let evidence: Value = serde_json::from_str(replacement["provenance"].as_str().unwrap()).unwrap();
        assert_eq!(evidence["contents_source"]["origin_case"], SOURCE);
        assert_eq!(evidence["contents_source"]["selected_by_case"], TARGET);
        assert_eq!(evidence["mass_inputs"]["contents_density"], q(1100.0, "kg/m^3"));
        let i = drifted["load_cases"].as_array().unwrap().iter().position(|c| c["id"] == TARGET).unwrap();
        drifted["load_cases"][i]["primitive_loads"][0] = replacement;
        assert_eq!(target_state(&drifted), AppliedSelfWeightState::Fresh);
    }
}

#[test]
fn a_legacy_record_in_a_case_that_now_selects_contents_is_stale() {
    let legacy = {
        let model = document(false, Some(1000.0), false);
        adopt(model.clone(), &plan(&model, None).unwrap())
    };
    let mut model = legacy.clone();
    model["condition_sets"] = document(false, None, true)["condition_sets"].clone();
    model["pipe_segments"][0]["condition_set_ref"] = json!("cs:hydro");
    assert_eq!(target_state(&model), AppliedSelfWeightState::Fresh, "sets alone change nothing");
    model["load_cases"][0]["condition_use"] = json!({"contents_density": "HYDRO"});
    assert_eq!(target_state(&model), AppliedSelfWeightState::Stale, "same density, now from a selection");
}

#[test]
fn request_and_context_refusals_are_named_and_never_legacy() {
    for (value, what) in [(Value::Null, "null"), (json!(7), "not a string"), (json!(""), "empty"), (json!("  \t"), "whitespace"), (json!(["case:source"]), "array")] {
        let mut wire = request(None);
        wire["contents_source_case_ref"] = value;
        match ContentsAwareSelfWeightRequest::from_value(&wire) {
            Err(SelfWeightRequestRefusal::ContentsSourceInvalid(message)) => assert!(message.contains("contents_source_case_ref"), "{what}"),
            other => panic!("{what}: {other:?}"),
        }
    }
    let mut unknown = request(Some(SOURCE));
    unknown["contents_source"] = json!("x");
    assert_eq!(ContentsAwareSelfWeightRequest::from_value(&unknown).unwrap_err(), SelfWeightRequestRefusal::Shape);
    assert_eq!(ContentsAwareSelfWeightRequest::from_value(&json!([])).unwrap_err(), SelfWeightRequestRefusal::Shape);
    let _ = SELF_WEIGHT_CONTENTS_SOURCE_INVALID;

    let model = document(false, None, true);
    let refused = |model: &Value, source: &str| plan(model, Some(source)).unwrap_err();
    assert_eq!(refused(&model, "case:none"), [CASE_CONTENTS_CONTEXT_MISSING]);
    assert_eq!(refused(&model, " case:source"), [CASE_CONTENTS_CONTEXT_MISSING], "an id is never trimmed");
    assert_eq!(refused(&model, "modal:weight-basis"), [CASE_CONTENTS_CONTEXT_MISSING], "a scratch id is no source");
    assert_eq!(refused(&model, TARGET), [CASE_CONTENTS_CONTEXT_CONFLICT], "the new target is not its own source");
    let mut duplicate = model.clone();
    duplicate["load_cases"].as_array_mut().unwrap().push(case_shell(false, SOURCE, Some("HYDRO")));
    assert_eq!(refused(&duplicate, SOURCE), [CASE_CONTENTS_CONTEXT_CONFLICT]);
    let mut no_use = model.clone();
    no_use["load_cases"][0].as_object_mut().unwrap().remove("condition_use");
    assert_eq!(refused(&no_use, SOURCE), [CASE_CONTENTS_CONTEXT_MISSING]);
    let mut other_use = model.clone();
    other_use["load_cases"][0]["condition_use"] = json!({"added_weight": true});
    assert_eq!(refused(&other_use, SOURCE), [CASE_CONTENTS_CONTEXT_MISSING]);
    // The target is a new case distinct from the source.
    let mut same = request(Some(SOURCE));
    same["case_id"] = json!(SOURCE);
    let parsed = ContentsAwareSelfWeightRequest::from_value(&same).unwrap();
    let found: Vec<String> = generate_contents_aware_self_weight_operations(&typed(&model), &parsed).unwrap_err().into_iter().map(|d| d.code).collect();
    assert_eq!(found, [CASE_CONTENTS_CONTEXT_CONFLICT]);
    // Per-pipe resolution: set, key, value, units and range, by name.
    let cases: [(&str, fn(&mut Value)); 7] = [
        ("OPERATING_CONDITION_SET_MISSING", |m| { m["pipe_segments"][0].as_object_mut().unwrap().remove("condition_set_ref"); }),
        ("OPERATING_CONDITION_SET_UNRESOLVED", |m| m["pipe_segments"][0]["condition_set_ref"] = json!("cs:none")),
        ("OPERATING_CONDITION_KEY_MISSING", |m| m["load_cases"][0]["condition_use"]["contents_density"] = json!("NONE")),
        ("OPERATING_CONDITION_VALUE_MISSING", |m| { m["condition_sets"][0]["conditions"][0].as_object_mut().unwrap().remove("contents_density"); }),
        ("OPERATING_CONDITION_QUANTITY_INVALID", |m| m["condition_sets"][0]["conditions"][0]["contents_density"] = q(1000.0, "kg/m")),
        ("OPERATING_CONDITION_QUANTITY_INVALID", |m| m["condition_sets"][0]["conditions"][0]["contents_density"] = q(-1.0, "kg/m^3")),
        ("OPERATING_CONDITION_KEY_DUPLICATE", |m| { m["condition_sets"][0]["conditions"].as_array_mut().unwrap().push(json!({"key": "HYDRO", "contents_density": q(1000.0, "kg/m^3")})); }),
    ];
    for (code, change) in cases {
        let mut bad = model.clone();
        change(&mut bad);
        assert_eq!(refused(&bad, SOURCE), [code], "{code}");
    }
    // NaN has no JSON form: set it on the typed model.
    let mut nan = typed(&model);
    nan.condition_sets[0].conditions.as_mut().unwrap()[0].contents_density.as_mut().unwrap().value = f64::NAN;
    let parsed = ContentsAwareSelfWeightRequest::from_value(&request(Some(SOURCE))).unwrap();
    let found: Vec<String> = generate_contents_aware_self_weight_operations(&nan, &parsed).unwrap_err().into_iter().map(|d| d.code).collect();
    assert_eq!(found, ["OPERATING_CONDITION_QUANTITY_INVALID"]);
    // All or none: a second pipe that cannot resolve leaves no partial plan.
    let mut two = model.clone();
    let mut second = two["pipe_segments"][0].clone();
    second["id"] = json!("pipe:b");
    second["from"] = json!("tip");
    second["to"] = json!("far");
    second.as_object_mut().unwrap().remove("condition_set_ref");
    two["pipe_segments"].as_array_mut().unwrap().push(second);
    two["nodes"].as_array_mut().unwrap().push(json!({"id": "far", "position": {"x": 12, "y": 0, "z": 0}, "provenance": PROV}));
    let mut wire = request(Some(SOURCE));
    wire["pipe_refs"] = json!([PIPE, "pipe:b"]);
    let parsed = ContentsAwareSelfWeightRequest::from_value(&wire).unwrap();
    let found: Vec<String> = generate_contents_aware_self_weight_operations(&typed(&two), &parsed).unwrap_err().into_iter().map(|d| d.code).collect();
    assert_eq!(found, ["OPERATING_CONDITION_SET_MISSING"]);
}

#[test]
fn a_component_weight_member_cannot_consume_selected_contents() {
    let mut model = document(false, None, true);
    model["components"] = json!([{"id": "component:v", "kind": "valve", "node": "tip",
        "geometry": {"rigid_pipe_ref": PIPE, "weight": q(500.0, "N"), "center_of_gravity": {"x": -0.5, "y": 0.0, "z": 0.0, "unit": "m"}},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROV}]);
    assert_eq!(plan(&model, Some(SOURCE)).unwrap_err(), [SELF_WEIGHT_CONTENTS_NOT_CONSUMED]);
    // Without a selection the component replacement is unchanged.
    model["load_cases"] = json!([]);
    assert!(plan(&model, None).is_ok());
}

#[test]
fn the_solve_admits_consumed_contents_only_and_refuses_the_rest_by_name() {
    // An unconsumed use (the origin, before the target exists) refuses by name on both routes.
    for exact in [false, true] {
        let envelope = solve(&document(exact, None, true), PreviewSolverMode::SparseInteractive);
        assert!(codes(&envelope).contains(&"LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED".to_string()), "exact {exact}: {:?}", envelope.diagnostics);
        // After generation the origin is still unconsumed: refused until its use is cleared or it is removed.
        let envelope = solve(&adopted(exact), PreviewSolverMode::SparseInteractive);
        let found = codes(&envelope);
        assert_eq!(found, ["LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED"], "exact {exact}");
        assert!(envelope.diagnostics.iter().any(|d| d.affected_refs.contains(&SOURCE.to_string())));
    }
    // A stale target blocks the solve after the lowering (the retained selection is validated).
    let mut stale = without_origin(adopted(true));
    stale["condition_sets"][0]["conditions"][0]["contents_density"] = q(1100.0, "kg/m^3");
    assert!(codes(&solve(&stale, PreviewSolverMode::SparseInteractive)).contains(&"SELF_WEIGHT_INPUTS_STALE".to_string()));
    let mut keyed = without_origin(adopted(false));
    keyed["load_cases"][0]["condition_use"]["contents_density"] = json!("ALT");
    assert!(codes(&solve(&keyed, PreviewSolverMode::SparseInteractive)).contains(&"SELF_WEIGHT_INPUTS_STALE".to_string()));
    // On the exact route every other use stays refused, with or without contents.
    let mut mixed = without_origin(adopted(true));
    mixed["load_cases"][0]["condition_use"]["added_weight"] = json!(true);
    assert!(codes(&solve(&mixed, PreviewSolverMode::SparseInteractive)).contains(&"LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED".to_string()));
    // A case whose seismic generation would not consume the selection is refused.
    let mut seismic = without_origin(adopted(false));
    seismic["load_cases"][0]["equivalent_static"] = json!({"seismic": {"g_factor_y": q(0.1, "1"), "gravity_acceleration": q(9.80665, "m/s^2")}});
    let found = codes(&solve(&seismic, PreviewSolverMode::SparseInteractive));
    assert!(found.contains(&"LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED".to_string()), "{found:?}");
    // The hydrostatic test case stays refused.
    let mut test = without_origin(adopted(true));
    test["load_cases"][0]["test_condition"] = json!({"kind": "hydrostatic", "held_supports": "none"});
    assert!(codes(&solve(&test, PreviewSolverMode::SparseInteractive)).contains(&"LOAD_CASE_TEST_CONDITION_UNSUPPORTED".to_string()));
    // The read-only preview resolves a consumed document and lists no generated record for contents.
    let preview = preview_operating_conditions(&without_origin(adopted(false)));
    assert_eq!(preview["resolved"], true, "{preview}");
    assert_eq!(preview["cases"][0]["generated_primitive_loads"], json!([]));
}
