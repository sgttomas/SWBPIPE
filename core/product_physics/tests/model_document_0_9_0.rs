//! Model document 0.9.0, slice S1-core (DEL-02-01 Design `model-document-0.9.0.md`
//! §1.3, §1.5, §2.1, §2.2, §2.4 and §5's S1 row; DEL-05-02 Design
//! `absolute-sum-srss-combinations.md` §2): the members' document-level codes and the
//! interim solve behaviour, through the public value entry the desktop and the runner
//! use, in both solver modes.
//!
//! Every document is built at the public `MODEL_SCHEMA_VERSION`, so the version step
//! carries these tests unchanged.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const PROV: &str = "invented_s1_core_test_input";
const EXACT: &str = include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json");

fn force(id: &str, node: &str, direction: &str, value: f64) -> Value {
    json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": node}, "direction": direction,
           "magnitude": {"value": value, "unit": "N"}, "dimension": "force", "provenance": PROV})
}

/// A linear pressure-free cantilever: an anchor at N0, a single-DOF UX spring at the tip
/// N2 and a UX line stop at N1; two cases and one combination of each basis.
fn model() -> Value {
    let pipe = |id: &str, from: &str, to: &str| json!({"id": id, "from": from, "to": to, "material": "mat",
        "y_reference": {"x": 0, "y": 0, "z": 1}, "section": {"outside_diameter": {"value": 0.2, "unit": "m"},
        "wall_thickness": {"value": 0.01, "unit": "m"}}, "provenance": PROV});
    let node = |id: &str, x: f64| json!({"id": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV});
    json!({
        "schema_version": MODEL_SCHEMA_VERSION,
        "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": "invented:s1-core", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC", "stress": "Pa"}},
        "nodes": [node("N0", 0.0), node("N1", 2.0), node("N2", 4.0)],
        "pipe_segments": [pipe("P1", "N0", "N1"), pipe("P2", "N1", "N2")],
        "materials": [{"id": "mat", "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "shear_modulus": {"value": 8.0e10, "unit": "Pa"}, "provenance": PROV}],
        "supports": [
            {"id": "anchor", "node": "N0", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
            {"id": "spring", "node": "N2", "family": "spring", "restraints": ["UX"], "stiffness": {"dof": "UX", "value": {"value": 1.0e6, "unit": "N/m"}}, "provenance": PROV},
            {"id": "stop", "node": "N1", "family": "line_stop", "restraints": ["UX"], "provenance": PROV},
        ],
        "load_cases": [
            {"id": "L1", "primitive_loads": [force("L1:x", "N2", "global_x", 1000.0), force("L1:y", "N2", "global_y", 500.0), force("L1:z", "N1", "global_z", -250.0)], "provenance": PROV},
            {"id": "L2", "primitive_loads": [force("L2:y", "N2", "global_y", -300.0), force("L2:x", "N1", "global_x", 120.0)], "provenance": PROV},
        ],
        "combinations": [
            {"id": "C-SUM", "basis": "mechanics", "terms": [{"load_case": "L1", "factor": 1.0}, {"load_case": "L2", "factor": 0.5}], "provenance": PROV},
            {"id": "C-SUB", "basis": "result_state_subtraction", "minuend_id": "L1", "subtrahend_id": "L2", "provenance": PROV},
            {"id": "C-ENV", "basis": "range_envelope", "operand_ids": ["L1", "L2"], "mode": "max", "provenance": PROV},
        ],
    })
}

/// The exact-route witness request (pressure contract 3.0.0), at the current version.
fn exact_model() -> Value {
    let mut request: Value = serde_json::from_str(EXACT).unwrap();
    request["model"]["schema_version"] = json!(MODEL_SCHEMA_VERSION);
    request["model"].take()
}

fn exact_materials() -> Value {
    let request: Value = serde_json::from_str(EXACT).unwrap();
    request["materials"].clone()
}

fn solve_with(model: &Value, materials: Value, mode: PreviewSolverMode) -> Result<Value, String> {
    run_linear_static_preview_value_with_mode(json!({"model": model, "materials": materials}), mode)
        .map(|envelope| serde_json::to_value(envelope).unwrap())
}

fn solve(model: &Value, mode: PreviewSolverMode) -> Value {
    let materials = if model.get("pressure_contract").is_some() { exact_materials() } else { json!([]) };
    solve_with(model, materials, mode).unwrap_or_else(|refusal| panic!("refused at the request boundary: {refusal}"))
}

fn text(envelope: &Value) -> String {
    serde_json::to_string(envelope).unwrap()
}

/// Byte equality of two envelopes, reporting the first difference in context.
fn assert_same_bytes(left: &Value, right: &Value, what: &str) {
    let (a, b) = (text(left), text(right));
    if a != b {
        let at = a.bytes().zip(b.bytes()).position(|(x, y)| x != y).unwrap_or(a.len().min(b.len()));
        let from = at.saturating_sub(200);
        panic!("{what}: first difference at byte {at}:\n left: {}\nright: {}", &a[from..(at + 200).min(a.len())], &b[from..(at + 200).min(b.len())]);
    }
}

fn diagnostics(envelope: &Value) -> Vec<(String, String, String)> {
    envelope["diagnostics"].as_array().unwrap().iter()
        .map(|d| (d["code"].as_str().unwrap().into(), d["severity"].as_str().unwrap().into(), d["message"].as_str().unwrap().into()))
        .collect()
}

fn codes(envelope: &Value) -> Vec<String> {
    diagnostics(envelope).into_iter().map(|(code, _, _)| code).collect()
}

fn combination_rows<'a>(envelope: &'a Value, id: &str) -> Vec<&'a Value> {
    envelope["results"].as_array().unwrap().iter()
        .filter(|r| r["basis_ref"]["ref_type"] == "combination" && r["basis_ref"]["ref_id"] == id).collect()
}

/// The model solves on both modes, publishing rows for every case and combination.
#[test]
fn the_base_documents_solve() {
    for mode in MODES {
        let solved = solve(&model(), mode);
        assert_eq!(solved["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:?}", diagnostics(&solved));
        for combination in ["C-SUM", "C-SUB", "C-ENV"] {
            assert!(!combination_rows(&solved, combination).is_empty(), "{mode:?}: {combination} publishes");
        }
        let exact = solve(&exact_model(), mode);
        assert_eq!(exact["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:?}", diagnostics(&exact));
    }
}

fn with_unused_members(mut document: Value) -> Value {
    document["condition_sets"] = json!([
        {"id": "cs:line", "label": "Line", "provenance": PROV,
         "conditions": [{"key": "OPE1", "label": "Operating", "temperature": {"value": 150.0, "unit": "degC"}, "pressure": {"value": 1.5, "unit": "MPa"},
                         "contents_density": {"value": 998.0, "unit": "kg/m^3"}},
                        {"key": "HYDRO", "temperature": {"value": 20.0, "unit": "degC"}}],
         "installation_temperature": {"value": 20.0, "unit": "degC"},
         "design": {"temperature": {"value": 180.0, "unit": "degC"}, "pressure": {"value": 2.0, "unit": "MPa"}},
         "peak": {"temperature": {"value": 200.0, "unit": "degC"}},
         "added_weight": {"magnitude": {"value": -50.0, "unit": "N/m"}, "direction": "global_z"},
         "exposure": {"wind": true, "snow_ice": false, "wave": true}, "tags": ["line"], "notes": "invented"},
        {"id": "cs:unassigned", "provenance": PROV},
    ]);
    document["spectra"] = json!([
        {"id": "sp:g", "abscissa": "frequency", "ordinate": "g_multiple", "units": {"abscissa": "Hz"}, "g_value": {"value": 9.80665, "unit": "m/s^2"},
         "damping_ratio": 0.05, "interpolation": {"abscissa": "log", "ordinate": "log"}, "points": [[1.0, 0.2], [10.0, 0.5]]},
    ]);
    for pipe in document["pipe_segments"].as_array_mut().unwrap() {
        pipe["condition_set_ref"] = json!("cs:line");
    }
    document
}

/// E5 (§1.8): a document with condition sets, assignments and spectra but no
/// `condition_use` solves byte-identically to the same document without them, on both
/// solver modes; on the exact route every solved member is identical and only the
/// hashes bound to the authored request differ.
#[test]
fn e5_unused_sets_assignments_and_spectra_change_no_published_byte() {
    for (base, exact) in [(model(), false), (exact_model(), true)] {
        let annotated = with_unused_members(base.clone());
        for mode in MODES {
            let reference = solve(&base, mode);
            let solved = solve(&annotated, mode);
            assert_eq!(solved["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:?}", diagnostics(&solved));
            if exact {
                // The exact route's source-block custody hashes bind the authored request
                // (§1.8's hash-bound exception, as `model_hash` is); everything solved is equal.
                for member in ["status", "results", "diagnostics", "summary", "numerical_quality", "formulation_basis"] {
                    assert_same_bytes(&solved[member], &reference[member], &format!("exact {mode:?}: {member}"));
                }
            } else {
                assert_same_bytes(&solved, &reference, &format!("{mode:?}: unused 0.9.0 members change no published byte"));
            }
        }
    }
}

fn support_mut<'a>(document: &'a mut Value, id: &str) -> &'a mut Value {
    document["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == id).unwrap()
}

/// §5's S1 row: an exactly axis-aligned `direction` takes today's path, byte for byte.
/// d = −e_x and (2,0,0) on a UX record solve as the record without direction; (0,3,0) on
/// a UX spring or line stop solves as its UY twin.
#[test]
fn axis_aligned_directions_solve_as_their_twins_byte_for_byte() {
    let uy_twin = |id: &str| {
        let mut twin = model();
        let support = support_mut(&mut twin, id);
        support["restraints"] = json!(["UY"]);
        if support.get("stiffness").is_some() {
            support["stiffness"]["dof"] = json!("UY");
        }
        twin
    };
    for id in ["spring", "stop"] {
        for (direction, twin) in [
            (json!({"x": -1.0, "y": 0.0, "z": 0.0}), model()),
            (json!({"x": 2.0, "y": 0.0, "z": 0.0}), model()),
            (json!({"x": -0.0, "y": 3.0, "z": 0.0}), uy_twin(id)),
        ] {
            let mut directed = model();
            support_mut(&mut directed, id)["direction"] = direction.clone();
            for mode in MODES {
                let solved = solve(&directed, mode);
                let reference = solve(&twin, mode);
                assert_eq!(solved["status"]["mechanics"], "MECHANICS_SOLVED", "{id} {direction} {mode:?}: {:?}", diagnostics(&solved));
                assert_eq!(text(&solved), text(&reference), "{id} {direction} {mode:?}: solves as its twin");
            }
        }
    }
    // The UY twin is a different solve: the rewrite is not a no-op.
    assert_ne!(text(&solve(&uy_twin("spring"), MODES[0])), text(&solve(&model(), MODES[0])));
}

/// absolute-sum-srss §2 (r1 F3): an id that names both a load case and a combination
/// resolves to the load case, as every reference did before 0.9.0, with no diagnostic. The
/// subtraction over `L1` solves exactly as when the shadowing combination has another id.
#[test]
fn a_flat_document_whose_combination_shares_a_load_case_id_keeps_its_meaning() {
    let mut shared = model();
    shared["combinations"].as_array_mut().unwrap().insert(0,
        json!({"id": "L1", "basis": "mechanics", "terms": [{"load_case": "L2", "factor": 2.0}], "provenance": PROV}));
    let mut renamed = shared.clone();
    renamed["combinations"][0]["id"] = json!("C-SHADOW");
    for mode in MODES {
        let (a, b) = (solve(&shared, mode), solve(&renamed, mode));
        assert_eq!(a["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:?}", diagnostics(&a));
        assert_eq!(codes(&a), codes(&b), "{mode:?}: no diagnostic is added");
        for combination in ["C-SUM", "C-SUB", "C-ENV"] {
            let rows = combination_rows(&a, combination);
            assert!(!rows.is_empty());
            assert_eq!(serde_json::to_string(&rows).unwrap(), serde_json::to_string(&combination_rows(&b, combination)).unwrap(), "{mode:?}: {combination}");
        }
        assert_eq!(combination_rows(&a, "L1").len(), combination_rows(&b, "C-SHADOW").len(), "{mode:?}: the shadowing combination publishes");
    }
}

/// The blocking findings of a document on both modes; it publishes no result row.
fn refused(document: &Value) -> Vec<(String, String, String)> {
    let mut first = None;
    for mode in MODES {
        let solved = solve(document, mode);
        assert_eq!(solved["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}: {:?}", diagnostics(&solved));
        assert!(solved["results"].as_array().unwrap().is_empty(), "{mode:?}: no rows");
        let found = diagnostics(&solved);
        if let Some(first) = &first {
            assert_eq!(first, &found, "both modes refuse alike");
        }
        first = Some(found);
    }
    first.unwrap()
}

fn assert_refused_by(document: &Value, code: &str, message_part: &str) {
    let found = refused(document);
    assert!(found.iter().any(|(c, severity, message)| c == code && severity == "blocking" && message.contains(message_part)),
        "{code} ({message_part}) in {found:?}");
}

fn mutated(mutate: impl FnOnce(&mut Value)) -> Value {
    let mut document = with_unused_members(model());
    mutate(&mut document);
    document
}

#[test]
fn every_condition_set_document_code_refuses_at_the_solve() {
    assert_refused_by(&mutated(|d| d["condition_sets"][0]["conditions"][1]["key"] = json!("OPE1")),
        "OPERATING_CONDITION_KEY_DUPLICATE", "condition_sets[0].conditions[1].key `OPE1` repeats a key");
    assert_refused_by(&mutated(|d| d["condition_sets"][0]["conditions"][0]["temperature"]["unit"] = json!("MPa")),
        "OPERATING_CONDITION_QUANTITY_INVALID", "condition_sets[0].conditions[0].temperature is not an absolute temperature: its unit `MPa`");
    assert_refused_by(&mutated(|d| d["condition_sets"][0]["installation_temperature"] = json!({"value": -300.0, "unit": "degC"})),
        "OPERATING_CONDITION_QUANTITY_INVALID", "below zero kelvin");
    assert_refused_by(&mutated(|d| d["condition_sets"][0]["conditions"][0]["contents_density"]["value"] = json!(-1.0)),
        "OPERATING_CONDITION_QUANTITY_INVALID", "is negative");
    assert_refused_by(&mutated(|d| d["condition_sets"][0]["added_weight"]["magnitude"]["unit"] = json!("N")),
        "OPERATING_CONDITION_QUANTITY_INVALID", "condition_sets[0].added_weight.magnitude");
    assert_refused_by(&mutated(|d| d["condition_sets"][0]["peak"]["temperature"] = json!({"value": 1.5e-24, "unit": "K"})),
        "OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED", "condition_sets[0].peak.temperature is outside");
    assert_refused_by(&mutated(|d| d["condition_sets"][1]["provenance"] = json!("  ")),
        "OPERATING_CONDITION_PROVENANCE_MISSING", "condition_sets[1].provenance is blank");
    assert_refused_by(&mutated(|d| d["pipe_segments"][1]["condition_set_ref"] = json!("cs:none")),
        "OPERATING_CONDITION_SET_UNRESOLVED", "pipe_segments[1].condition_set_ref `cs:none` of pipe `P2` names no condition set");
    assert_refused_by(&mutated(|d| d["load_cases"][0]["condition_use"] = json!({})),
        "LOAD_CASE_CONDITION_USE_EMPTY", "is empty");
    assert_refused_by(&mutated(|d| d["load_cases"][0]["condition_use"] = json!({"temperature": "OPE1", "wind_exposure": false})),
        "LOAD_CASE_CONDITION_USE_EMPTY", "load_cases[0].condition_use.wind_exposure of load case `L1` is false");
    assert_refused_by(&mutated(|d| d["condition_sets"][1]["id"] = json!("cs:line")), "DUPLICATE_ID", "condition-set IDs must be unique");
    assert_refused_by(&mutated(|d| d["condition_sets"][1]["id"] = json!(" ")), "EMPTY_ID", "condition-set ID must be explicit");
    // A set is judged whether or not a case uses it: no case uses the unassigned set here.
    assert_refused_by(&mutated(|d| d["condition_sets"][1]["conditions"] = json!([{"key": "K", "pressure": {"value": 1.0, "unit": "degC"}}])),
        "OPERATING_CONDITION_QUANTITY_INVALID", "condition_sets[1].conditions[0].pressure");
}

#[test]
fn a_condition_use_is_refused_by_name_on_both_routes_and_a_present_null_is_not_absence() {
    assert_refused_by(&mutated(|d| d["load_cases"][1]["condition_use"] = json!({"temperature": "OPE1"})),
        "LOAD_CASE_CONDITION_USE_UNSUPPORTED", "load case `L2` uses operating conditions");
    let mut exact = with_unused_members(exact_model());
    exact["load_cases"][0]["condition_use"] = json!({"temperature": "OPE1"});
    let found = refused(&exact);
    assert!(found.iter().any(|(code, _, _)| code == "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED"), "{found:?}");
    assert!(!found.iter().any(|(code, _, _)| code == "LOAD_CASE_CONDITION_USE_UNSUPPORTED"), "{found:?}");
    // A present null is refused at the closed records, not read as absence.
    for null in [
        mutated(|d| d["load_cases"][0]["condition_use"] = Value::Null),
        mutated(|d| d["pipe_segments"][0]["condition_set_ref"] = Value::Null),
        mutated(|d| d["condition_sets"][0]["label"] = Value::Null),
        mutated(|d| d["condition_sets"] = Value::Null),
    ] {
        assert!(solve_with(&null, json!([]), MODES[0]).is_err());
    }
    // An undeclared key on a new record is refused by name before anything is solved.
    let refusal = solve_with(&mutated(|d| d["condition_sets"][0]["conditions"][0]["temprature"] = json!(1)), json!([]), MODES[0]).unwrap_err();
    assert!(refusal.starts_with("UNKNOWN_MODEL_FIELD") && refusal.contains("unknown field `temprature` in condition `OPE1` of condition set `cs:line`"), "{refusal}");
    let refusal = solve_with(&mutated(|d| d["supports"][1]["direction"] = json!({"x": 1, "y": 0, "z": 0, "w": 0})), json!([]), MODES[0]).unwrap_err();
    assert!(refusal.contains("unknown field `w` in the direction of support `spring`"), "{refusal}");
}

#[test]
fn a_test_condition_is_refused_by_name() {
    assert_refused_by(&mutated(|d| d["load_cases"][0]["test_condition"] = json!({"kind": "hydrostatic", "held_supports": "none"})),
        "LOAD_CASE_TEST_CONDITION_UNSUPPORTED", "load case `L1` is a hydrostatic test condition");
}

#[test]
fn every_direction_code_refuses_at_the_solve() {
    let direct = |id: &str, direction: Value| mutated(|d| support_mut(d, id)["direction"] = direction);
    assert_refused_by(&direct("spring", Value::Null), "SUPPORT_DIRECTION_INVALID", "supports[1].direction of support `spring` is not three finite numbers");
    assert_refused_by(&direct("spring", json!({"x": 1, "y": 0})), "SUPPORT_DIRECTION_INVALID", "supports[1].direction");
    assert_refused_by(&direct("spring", json!({"x": "1", "y": 0, "z": 0})), "SUPPORT_DIRECTION_INVALID", "supports[1].direction");
    assert_refused_by(&direct("spring", json!({"x": 0.0, "y": -0.0, "z": 0})), "SUPPORT_DIRECTION_ZERO", "is zero");
    assert_refused_by(&direct("anchor", json!({"x": 1, "y": 0, "z": 0})), "SUPPORT_DIRECTION_RECORD_UNSUPPORTED", "explicit line_stop, spring or snubber family");
    assert_refused_by(&mutated(|d| { let s = support_mut(d, "stop"); s["direction"] = json!({"x": 1, "y": 0, "z": 0}); s["family"] = Value::Null; }),
        "SUPPORT_DIRECTION_RECORD_UNSUPPORTED", "explicit line_stop, spring or snubber family");
    assert_refused_by(&mutated(|d| { let s = support_mut(d, "stop"); s["direction"] = json!({"x": 1, "y": 0, "z": 0}); s["restraints"] = json!(["UX", "UY"]); }),
        "SUPPORT_DIRECTION_RECORD_UNSUPPORTED", "exactly one DOF");
    assert_refused_by(&mutated(|d| { let s = support_mut(d, "stop"); s["direction"] = json!({"x": 1, "y": 0, "z": 0}); s["restraints"] = json!(["RX"]); }),
        "SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED", "rotational DOF");
    assert_refused_by(&mutated(|d| { let s = support_mut(d, "stop"); s["direction"] = json!({"x": 1, "y": 0, "z": 0});
            s["nonlinear"] = json!({"behavior": "one_way", "dof": "UX", "initial_state": "active", "active_when": "negative_reaction"}); }),
        "SUPPORT_DIRECTION_RECORD_UNSUPPORTED", "nonlinear member");
    // Interim until the skew kernel: two or more nonzero components.
    assert_refused_by(&direct("spring", json!({"x": 0.0, "y": 3.0, "z": 4.0})), "SUPPORT_DIRECTION_SKEW_UNSUPPORTED", "two or more nonzero components");
}

fn snubber(extra: Value) -> Value {
    let mut record = json!({"id": "snub", "node": "N2", "family": "snubber", "restraints": ["UY"], "provenance": PROV});
    for (key, value) in extra.as_object().unwrap() {
        record[key] = value.clone();
    }
    record
}

/// §2.4 (r2 F5): a valid stiff snubber reaches `SNUBBER_DYNAMIC_ONLY_UNSUPPORTED` and no
/// other code, on every static solve on both routes.
#[test]
fn a_valid_stiff_snubber_reaches_only_its_dynamic_only_refusal_on_both_routes() {
    let stiff = snubber(json!({"stiffness": {"dof": "UY", "value": {"value": 1.0e7, "unit": "N/m"}}}));
    for (mut document, node) in [(model(), "N2"), (exact_model(), "")] {
        let mut record = stiff.clone();
        if node.is_empty() {
            record["node"] = document["nodes"][1]["id"].clone();
        }
        document["supports"].as_array_mut().unwrap().push(record);
        let found = refused(&document);
        assert_eq!(found, vec![(
            "SNUBBER_DYNAMIC_ONLY_UNSUPPORTED".to_string(),
            "blocking".to_string(),
            "support `snub`: snubbers are dynamic-only restraints; the response-spectrum analysis that uses them is not built yet".to_string(),
        )]);
    }
    // Rigid (no stiffness), and skewed with one restraint: still only the snubber refusal.
    for record in [snubber(json!({})), snubber(json!({"direction": {"x": 0.0, "y": 3.0, "z": 4.0}}))] {
        let mut document = model();
        document["supports"].as_array_mut().unwrap().push(record);
        assert_eq!(refused(&document).iter().map(|(code, _, _)| code.as_str()).collect::<Vec<_>>(), ["SNUBBER_DYNAMIC_ONLY_UNSUPPORTED"]);
    }
}

#[test]
fn every_snubber_code_refuses_at_the_solve() {
    let with = |record: Value| { let mut d = model(); d["supports"].as_array_mut().unwrap().push(record); d };
    assert_refused_by(&with(snubber(json!({"restraints": ["UY", "RZ"]}))), "SNUBBER_ROTATIONAL_RESTRAINT_UNSUPPORTED", "supports[3].restraints[1] of snubber `snub`");
    assert_refused_by(&with(snubber(json!({"stiffness": {"dof": "UX", "value": {"value": 1.0e7, "unit": "N/m"}}}))), "SNUBBER_STIFFNESS_INVALID", "supports[3].stiffness");
    assert_refused_by(&with(snubber(json!({"restraints": ["UX", "UY"], "stiffness": {"dof": "UY", "value": {"value": 1.0e7, "unit": "N/m"}}}))), "SNUBBER_STIFFNESS_INVALID", "supports[3].stiffness");
    assert_refused_by(&with(snubber(json!({"stiffness": {"dof": "UY", "value": {"value": 0.0, "unit": "N/m"}}}))), "SNUBBER_STIFFNESS_INVALID", "positive finite");
    assert_refused_by(&with(snubber(json!({"stiffness": {"dof": "UY", "value": {"value": 1.0, "unit": "N*m/rad"}}}))), "SNUBBER_STIFFNESS_INVALID", "linear-stiffness unit");
    assert_refused_by(&with(snubber(json!({"hanger": {"hanger_type": "spring_hanger"}}))), "SNUBBER_MEMBER_CONFLICT", "hanger or nonlinear member");
    assert_refused_by(&with(snubber(json!({"nonlinear": {"behavior": "one_way", "dof": "UY", "initial_state": "active", "active_when": "negative_reaction"}}))),
        "SNUBBER_MEMBER_CONFLICT", "hanger or nonlinear member");
    assert_refused_by(&with(snubber(json!({"restraints": ["UX", "UY"], "direction": {"x": 0.0, "y": 3.0, "z": 4.0}}))),
        "SUPPORT_DIRECTION_RECORD_UNSUPPORTED", "on a snubber only with exactly one translational restraint");
}

fn with_combination(combination: Value) -> Value {
    let mut document = model();
    document["combinations"].as_array_mut().unwrap().push(combination);
    document
}

#[test]
fn every_combination_document_code_refuses_at_the_solve() {
    let sub = |extra: Value| { let mut c = json!({"id": "C-X", "basis": "result_state_subtraction", "minuend_id": "L1", "subtrahend_id": "L2", "provenance": PROV});
        for (k, v) in extra.as_object().unwrap() { c[k] = v.clone(); } c };
    assert_refused_by(&with_combination(json!({"id": "C-X", "basis": "mechanics", "terms": [{"load_case": "L1", "factor": 1.0}], "difference_basis": "operating_minus_sustained", "provenance": PROV})),
        "LOAD_COMBINATION_DIFFERENCE_BASIS_MISPLACED", "combinations[3].difference_basis of combination `C-X` is admitted only with basis `result_state_subtraction`");
    // A present null reaches its named code, not a decode error.
    assert_refused_by(&with_combination(sub(json!({"difference_basis": null}))), "LOAD_COMBINATION_DIFFERENCE_BASIS_UNKNOWN", "is not `operating_minus_sustained`");
    assert_refused_by(&with_combination(sub(json!({"difference_basis": "sustained"}))), "LOAD_COMBINATION_DIFFERENCE_BASIS_UNKNOWN", "is not `operating_minus_sustained`");
    assert_refused_by(&with_combination(sub(json!({"subtrahend_support_state": "own"}))), "LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_MISPLACED", "admitted only with difference_basis");
    assert_refused_by(&with_combination(sub(json!({"difference_basis": "operating_minus_sustained", "subtrahend_support_state": null}))),
        "LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNKNOWN", "is not `own` or `minuend_final`");
    assert_refused_by(&with_combination(sub(json!({"subtrahend_id": "nowhere"}))), "LOAD_COMBINATION_OPERAND_UNKNOWN", "combinations[3].subtrahend_id `nowhere` of combination `C-X` names no load case and no combination");
    assert_refused_by(&with_combination(json!({"id": "C-X", "basis": "range_envelope", "operand_ids": ["L1", "nowhere"], "mode": "max", "provenance": PROV})),
        "LOAD_COMBINATION_OPERAND_UNKNOWN", "combinations[3].operand_ids[1]");
    assert_refused_by(&with_combination(json!({"id": "C-X", "basis": "srss", "terms": [{"combination": "nowhere", "factor": 1.0}], "provenance": PROV})),
        "LOAD_COMBINATION_OPERAND_UNKNOWN", "combinations[3].terms[0].combination");
    // A term's load_case keeps its code.
    assert_refused_by(&with_combination(json!({"id": "C-X", "basis": "mechanics", "terms": [{"load_case": "nowhere", "factor": 1.0}], "provenance": PROV})),
        "LOAD_COMBINATION_LOAD_CASE_UNKNOWN", "not present");
    assert_refused_by(&with_combination(sub(json!({"difference_basis": "operating_minus_sustained", "minuend_id": "C-SUM"}))),
        "LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE", "combinations[3].minuend_id `C-SUM` of combination `C-X` is a combination");
    let mut cycle = model();
    cycle["combinations"].as_array_mut().unwrap().extend([
        json!({"id": "C-A", "basis": "mechanics", "terms": [{"combination": "C-B", "factor": 1.0}], "provenance": PROV}),
        json!({"id": "C-B", "basis": "range_envelope", "operand_ids": ["L1", "C-A"], "mode": "max", "provenance": PROV}),
        json!({"id": "C-SELF", "basis": "result_state_subtraction", "minuend_id": "C-SELF", "subtrahend_id": "L1", "provenance": PROV}),
    ]);
    let found = refused(&cycle);
    let cycles: Vec<_> = found.iter().filter(|(code, _, _)| code == "LOAD_COMBINATION_OPERAND_CYCLE").map(|(_, _, m)| m.as_str()).collect();
    assert_eq!(cycles, ["combinations `C-A`, `C-B` name each other in a cycle; a combination's operands must not lead back to it.",
                        "combinations `C-SELF` name each other in a cycle; a combination's operands must not lead back to it."]);
    // A term carries exactly one of load_case and combination.
    for term in [json!({"load_case": "L1", "combination": "C-SUM", "factor": 1.0}), json!({"factor": 1.0})] {
        assert_refused_by(&with_combination(json!({"id": "C-X", "basis": "absolute_sum", "terms": [term], "provenance": PROV})),
            "LOAD_COMBINATION_SHAPE_INVALID", "exactly one of load_case and combination");
    }
    // The unsigned bases take the mechanics shape.
    assert_refused_by(&with_combination(json!({"id": "C-X", "basis": "srss", "terms": [], "provenance": PROV})), "LOAD_COMBINATION_TERMS_EMPTY", "at least one");
    assert_refused_by(&with_combination(json!({"id": "C-X", "basis": "absolute_sum", "terms": [{"load_case": "L1", "factor": 1.0}], "mode": "max", "provenance": PROV})),
        "LOAD_COMBINATION_SHAPE_INVALID", "explicit terms only");
}

/// The interim warnings withhold the combination and each combination that names it;
/// the cases and the other combinations publish exactly as without them.
#[test]
fn the_interim_combination_members_withhold_only_their_combinations() {
    let withheld = [
        (json!({"id": "C-DB", "basis": "result_state_subtraction", "minuend_id": "L1", "subtrahend_id": "L2", "difference_basis": "operating_minus_sustained", "provenance": PROV}),
         vec!["LOAD_COMBINATION_DIFFERENCE_BASIS_UNSUPPORTED"]),
        (json!({"id": "C-DB", "basis": "result_state_subtraction", "minuend_id": "L1", "subtrahend_id": "L2", "difference_basis": "operating_minus_sustained",
                "subtrahend_support_state": "minuend_final", "provenance": PROV}),
         vec!["LOAD_COMBINATION_DIFFERENCE_BASIS_UNSUPPORTED", "LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED"]),
        (json!({"id": "C-DB", "basis": "absolute_sum", "terms": [{"load_case": "L1", "factor": 1.0}], "provenance": PROV}), vec!["LOAD_COMBINATION_UNSIGNED_BASIS_UNSUPPORTED"]),
        (json!({"id": "C-DB", "basis": "srss", "terms": [{"load_case": "L1", "factor": 1.0}, {"load_case": "L2", "factor": 1.0}], "provenance": PROV}), vec!["LOAD_COMBINATION_UNSIGNED_BASIS_UNSUPPORTED"]),
        (json!({"id": "C-DB", "basis": "mechanics", "terms": [{"combination": "C-SUM", "factor": 1.0}], "provenance": PROV}), vec!["LOAD_COMBINATION_NESTED_UNSUPPORTED"]),
        (json!({"id": "C-DB", "basis": "range_envelope", "operand_ids": ["L1", "C-SUB"], "mode": "min", "provenance": PROV}), vec!["LOAD_COMBINATION_NESTED_UNSUPPORTED"]),
    ];
    for (combination, expected) in withheld {
        let mut document = with_combination(combination.clone());
        // A dependant: it names the withheld combination, so it is withheld too.
        document["combinations"].as_array_mut().unwrap().push(
            json!({"id": "C-DEP", "basis": "result_state_subtraction", "minuend_id": "C-DB", "subtrahend_id": "L2", "provenance": PROV}));
        for mode in MODES {
            let solved = solve(&document, mode);
            let reference = solve(&model(), mode);
            assert_eq!(solved["status"]["mechanics"], "MECHANICS_SOLVED", "{combination} {mode:?}: {:?}", diagnostics(&solved));
            let warnings: Vec<_> = diagnostics(&solved).into_iter().filter(|(_, severity, _)| severity == "warning")
                .filter(|(code, _, _)| code.starts_with("LOAD_COMBINATION_")).collect();
            let mut want: Vec<String> = expected.iter().map(|code| code.to_string()).collect();
            want.push("LOAD_COMBINATION_NESTED_UNSUPPORTED".into());
            assert_eq!(warnings.iter().map(|(code, _, _)| code.clone()).collect::<Vec<_>>(), want, "{combination} {mode:?}");
            assert!(warnings.last().unwrap().2.contains("combination `C-DEP` takes a combination as an operand"));
            assert!(combination_rows(&solved, "C-DB").is_empty() && combination_rows(&solved, "C-DEP").is_empty(), "{mode:?}: withheld");
            let mut published = solved.clone();
            published["diagnostics"] = reference["diagnostics"].clone();
            assert_eq!(text(&published), text(&reference), "{combination} {mode:?}: the rest publishes byte for byte");
        }
    }
}

#[test]
fn a_difference_basis_is_refused_by_name_on_the_exact_route() {
    let mut exact = exact_model();
    let case = exact["load_cases"][0]["id"].clone();
    let mut second = exact["load_cases"][0].clone();
    second["id"] = json!("case:second");
    exact["load_cases"].as_array_mut().unwrap().push(second);
    exact["combinations"] = json!([{"id": "C-DB", "basis": "result_state_subtraction", "minuend_id": case, "subtrahend_id": "case:second",
        "difference_basis": "operating_minus_sustained", "provenance": PROV}]);
    let found = refused(&exact);
    assert!(found.iter().any(|(code, severity, _)| code == "LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED" && severity == "blocking"), "{found:?}");
}

#[test]
fn every_spectrum_code_refuses_at_the_solve() {
    assert_refused_by(&mutated(|d| d["spectra"][0]["damping_ratio"] = json!(1.0)), "SPECTRUM_RECORD_INVALID", "spectrum `sp:g`: spectra[0].damping_ratio 1 is not a finite ratio in [0, 1)");
    assert_refused_by(&mutated(|d| { d["spectra"][0]["units"]["ordinate"] = json!("m/s^2"); }), "SPECTRUM_RECORD_INVALID", "spectra[0].units.ordinate is present only when ordinate is acceleration");
    assert_refused_by(&mutated(|d| { let s = d["spectra"][0].clone(); d["spectra"].as_array_mut().unwrap().push(s); }), "SPECTRUM_ID_DUPLICATE", "spectra[1].id `sp:g` repeats");
    assert_refused_by(&mutated(|d| d["spectra"][0]["id"] = json!("")), "EMPTY_ID", "spectrum ID must be explicit and non-empty");
    assert!(solve_with(&mutated(|d| d["spectra"][0]["label"] = Value::Null), json!([]), MODES[0]).is_err(), "null is refused at the closed record");
}
