//! DEL-04-04 REQ-012 and AC-009: a friction support whose normal is the
//! reaction of a one-way support, solved through PP's public entry in both
//! solver modes against the frozen independent reference
//! `validation/references/friction_one_way_normal/` (its README is the
//! binding's account of the model, the cases and the classification).
//!
//! Each case is authored as a plain `3.0.0/exact_pressure_v3` document from the
//! reference's own inputs: nodes A, B and S; the members `riser` A→B and `run`
//! B→S with the reference's section (zero mill tolerance and corrosion
//! allowance) and `euler_bernoulli`; the anchor at A; a one-way support at S on
//! UZ bearing with a positive reaction; and a friction support at S on UX with
//! μ = 3/8 whose `normal_reaction_source` names that one-way support on its own
//! DOF. No pressure region; every load is a nodal force. The authored initial
//! states (one-way active, friction sticking) are seeds only.
//!
//! **States** equal the reference's exactly. **Values** (every displacement and
//! rotation, every reaction, every member end action and axial force, and the
//! published derived normal) agree at |observed − expected| ≤
//! 1e-9·max(|expected|, family floor), the family floor being the largest
//! magnitude of that family in the case. The reference chooses no number
//! (its `tolerance_class`), and DEL-09-03 REQ-007 leaves the agreement of
//! values undecided outside the cases its limits were measured on; this is the
//! rule the maintained nonlinear binding (`rt_a3_nonlinear_references.rs`)
//! already applies, chosen here for the same reason and not loosened. A
//! mismatch is the product's to report, never the reference's or the
//! tolerance's to adjust.
//!
//! **Standing.** The confirmation admits only a base restraint as a derived
//! normal's source, so no case here is confirmed: each is published
//! `NUMERICAL_INTEGRITY_SENSITIVE` with its named reason. The change makes
//! this configuration solve; it does not confirm it.

use open_pipe_stress_product_physics::result_export::{reader_verdict, semantic_contract};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const REFERENCES: &str = include_str!("../../../validation/references/friction_one_way_normal/references.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const PROVENANCE: &str = "invented_friction_one_way_normal_reference";
const MATERIAL: &str = "material:fown";
const CASE: &str = "case:main";
const ONE_WAY: &str = "support:one-way-S";
const FRICTION: &str = "support:friction-S";
const ANCHOR: &str = "support:anchor-A";
const SENSITIVE: &str = "NUMERICAL_INTEGRITY_SENSITIVE";
const NOT_BASE: &str = "support support:friction-S: friction normal source is not a base restraint";
const DOFS: [(&str, &str, f64); 6] = [
    ("UX", "global_nodal_displacement_x", 1000.0),
    ("UY", "global_nodal_displacement_y", 1000.0),
    ("UZ", "global_nodal_displacement_z", 1000.0),
    ("RX", "global_nodal_rotation_x", 1.0),
    ("RY", "global_nodal_rotation_y", 1.0),
    ("RZ", "global_nodal_rotation_z", 1.0),
];
const ACTIONS: [(&str, &str, &str); 6] = [
    ("FX", "Fx", "element_global_end_force_v1"),
    ("FY", "Fy", "element_global_end_force_v1"),
    ("FZ", "Fz", "element_global_end_force_v1"),
    ("MX", "Mx", "element_global_end_moment_v1"),
    ("MY", "My", "element_global_end_moment_v1"),
    ("MZ", "Mz", "element_global_end_moment_v1"),
];
const MEMBERS: [(&str, &str, &str); 2] = [("riser", "A", "B"), ("run", "B", "S")];

fn references() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}

fn cases(references: &Value) -> &Vec<Value> {
    references["cases"].as_array().unwrap()
}

/// An exact reference input ("a" or "a/b"); expected values are read from `value`.
fn ratio(text: &Value) -> f64 {
    let text = text.as_str().unwrap_or_else(|| panic!("not a ratio string: {text}"));
    match text.split_once('/') {
        Some((a, b)) => a.parse::<f64>().unwrap() / b.parse::<f64>().unwrap(),
        None => text.parse::<f64>().unwrap(),
    }
}

fn value(v: &Value) -> f64 {
    v["value"].as_f64().unwrap_or_else(|| panic!("no value in {v}"))
}

fn quantity(v: f64, unit: &str) -> Value {
    json!({"value": v, "unit": unit})
}

fn node_id(name: &str) -> String {
    format!("node:{name}")
}

/// The case's plain v3 document (module docs). `source` overrides the friction
/// support's normal source for the refusal tests.
fn document_with(references: &Value, case: &Value, one_way: Value, source: Value) -> Value {
    let model = &references["model"];
    let section = &model["section"];
    let nodes = model["nodes"].as_object().unwrap().iter().map(|(name, xyz)| {
        let xyz = xyz.as_array().unwrap();
        json!({"id": node_id(name), "position": {"x": xyz[0].as_f64().unwrap(), "y": xyz[1].as_f64().unwrap(), "z": xyz[2].as_f64().unwrap()}, "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let pipes = MEMBERS.iter().map(|(id, from, to)| json!({"id": id, "from": node_id(from), "to": node_id(to),
        "section": {"outside_diameter": quantity(ratio(&section["D"]["exact"]), "m"), "wall_thickness": quantity(ratio(&section["t"]["exact"]), "m"),
            "mill_tolerance": quantity(0.0, "m"), "corrosion_allowance": quantity(0.0, "m")},
        "material": MATERIAL, "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROVENANCE})).collect::<Vec<_>>();
    let supports = vec![
        json!({"id": ANCHOR, "node": node_id("A"), "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE}),
        json!({"id": ONE_WAY, "node": node_id("S"), "family": "nonlinear", "restraints": [], "nonlinear": one_way, "provenance": PROVENANCE}),
        json!({"id": FRICTION, "node": node_id("S"), "family": "nonlinear", "restraints": [],
            "nonlinear": {"behavior": "friction", "dof": "UX", "initial_state": "sticking",
                "friction_coefficient": quantity(ratio(&model["friction_coefficient"]["exact"]), "1"),
                "normal_reaction_source": source}, "provenance": PROVENANCE}),
    ];
    let loads = case["primitive_loads"].as_array().unwrap().iter().enumerate().map(|(k, load)| {
        let direction = match load["component"].as_str().unwrap() { "FX" => "global_x", "FZ" => "global_z", o => panic!("{o}") };
        json!({"id": format!("load:{k}"), "category": "concentrated_force", "target": {"type": "node", "node": node_id(load["node"].as_str().unwrap())},
            "direction": direction, "magnitude": quantity(ratio(&load["value"]["exact"]), "N"), "dimension": "force", "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    json!({"model": {"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
        "project": {"id": "project:friction-one-way-normal", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes,
        "materials": [{"id": MATERIAL, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": quantity(ratio(&model["material"]["E"]["exact"]), "Pa"),
            "poisson_ratio": quantity(ratio(&model["material"]["nu"]["exact"]), "1"), "provenance": PROVENANCE}],
        "supports": supports, "components": [],
        "load_cases": [{"id": CASE, "primitive_loads": loads, "pressure_regions": [], "provenance": PROVENANCE}],
        "combinations": []},
        "materials": []})
}

fn one_way_record() -> Value {
    json!({"behavior": "one_way", "dof": "UZ", "initial_state": "active", "active_when": "positive_reaction"})
}

fn document(references: &Value, case: &Value) -> Value {
    document_with(references, case, one_way_record(), json!({"support_ref": ONE_WAY, "dof": "UZ"}))
}

fn solve(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).expect("the entry accepts the document")).unwrap()
}

fn diagnostics(envelope: &Value) -> &Vec<Value> {
    envelope["diagnostics"].as_array().unwrap()
}

fn blocking(envelope: &Value) -> Vec<&Value> {
    diagnostics(envelope).iter().filter(|d| d["severity"] == "blocking").collect()
}

/// The case's one integrity record: (code, message).
fn standing(envelope: &Value) -> (String, String) {
    let found = diagnostics(envelope).iter()
        .filter(|d| d["affected_refs"] == json!([CASE]) && d["code"].as_str().unwrap_or("").starts_with("NUMERICAL_INTEGRITY_"))
        .collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{found:#?}");
    (found[0]["code"].as_str().unwrap().to_string(), found[0]["message"].as_str().unwrap().to_string())
}

fn reader(envelope: &Value, document: Option<&Value>) -> Option<String> {
    let found = reader_verdict::validate("pressure", envelope, document.map(|d| &d["model"]), None);
    (found["ok"] != true).then(|| found["error"].to_string())
}

fn rows<'a>(envelope: &'a Value, kind: &str, entity: &str, location: Option<&str>, component: Option<&str>) -> Vec<&'a Value> {
    envelope["results"].as_array().unwrap().iter().filter(|r| {
        r["kind"] == kind && r["entity_ref"] == entity
            && !r["metadata"]["basis"].as_str().unwrap_or("").contains("; section_basis=reduced_section")
            && location.is_none_or(|l| r["metadata"]["location"] == l)
            && component.is_none_or(|c| r["metadata"]["component"] == c)
            && r["basis_ref"]["ref_type"] == "load_case" && r["basis_ref"]["ref_id"] == CASE
    }).collect()
}

fn row(envelope: &Value, kind: &str, entity: &str, location: Option<&str>, component: Option<&str>) -> f64 {
    let found = rows(envelope, kind, entity, location, component);
    assert_eq!(found.len(), 1, "{kind} {entity} {location:?} {component:?}: {} rows", found.len());
    found[0]["value"].as_f64().unwrap()
}

/// The product's published state token of a nonlinear support.
fn state_token(envelope: &Value, support: &str) -> String {
    let found = rows(envelope, "nonlinear_support_active_set_state_code", support, None, None);
    assert_eq!(found.len(), 1, "{support}: state rows");
    let basis = found[0]["metadata"]["basis"].as_str().unwrap();
    basis.split("final_state=").nth(1).unwrap().split(';').next().unwrap().trim().to_string()
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Family {
    Translation,
    Rotation,
    Force,
    Moment,
}

/// Every expected value of a case, keyed, with its family. The out-of-plane
/// quantities the reference does not list are exactly 0 (its `plane`).
fn expected(case: &Value) -> BTreeMap<String, (f64, Family)> {
    let mut out = BTreeMap::new();
    let family = |key: &str| match &key[..1] {
        "U" => Family::Translation,
        "R" => Family::Rotation,
        "F" => Family::Force,
        _ => Family::Moment,
    };
    let get = |block: &Value, key: &str| block.get(key).map(value).unwrap_or(0.0);
    for node in ["B", "S"] {
        for (dof, _, _) in DOFS {
            out.insert(format!("displacement/{node}/{dof}"), (get(&case["displacements"][node], dof), family(dof)));
        }
    }
    for key in ["FX", "FY", "FZ", "MX", "MY", "MZ"] {
        out.insert(format!("reaction/anchor/{key}"), (get(&case["reactions"]["anchor A"], key), family(key)));
    }
    out.insert("reaction/one-way/FZ".into(), (value(&case["reactions"]["one-way S"]["FZ"]), Family::Force));
    out.insert("reaction/friction/FX".into(), (value(&case["reactions"]["friction S"]["FX"]), Family::Force));
    for (pipe, from, to) in MEMBERS {
        for node in [from, to] {
            for (key, _, _) in ACTIONS {
                out.insert(format!("end_action/{pipe}/{node}/{key}"), (get(&case["member_end_actions"][pipe][node], key), family(key)));
            }
        }
        out.insert(format!("axial/{pipe}"), (value(&case["member_axial_force_tension_positive"][pipe]), Family::Force));
    }
    out.insert("normal".into(), (value(&case["deciding_quantities"]["normal N (one-way reaction)"]), Family::Force));
    out
}

fn observed(envelope: &Value) -> BTreeMap<String, f64> {
    let mut out = BTreeMap::new();
    for node in ["B", "S"] {
        for (dof, kind, factor) in DOFS {
            out.insert(format!("displacement/{node}/{dof}"), row(envelope, kind, &node_id(node), None, None) / factor);
        }
    }
    let component = |key: &str| format!("{}{}", &key[..1], key[1..].to_lowercase());
    for key in ["FX", "FY", "FZ", "MX", "MY", "MZ"] {
        out.insert(format!("reaction/anchor/{key}"), row(envelope, "support_reaction_component_v2", ANCHOR, None, Some(&component(key))));
    }
    out.insert("reaction/one-way/FZ".into(), row(envelope, "support_reaction_component_v2", ONE_WAY, None, Some("Fz")));
    out.insert("reaction/friction/FX".into(), row(envelope, "support_reaction_component_v2", FRICTION, None, Some("Fx")));
    for (pipe, from, to) in MEMBERS {
        for (node, end) in [(from, "end_i"), (to, "end_j")] {
            for (key, component, kind) in ACTIONS {
                out.insert(format!("end_action/{pipe}/{node}/{key}"), row(envelope, kind, pipe, Some(end), Some(component)));
            }
        }
        // Tension positive: the element's local axial force at end_j.
        out.insert(format!("axial/{pipe}"), row(envelope, "element_local_axial_force", pipe, Some("end_j"), None));
    }
    out.insert("normal".into(), row(envelope, "nonlinear_support_friction_normal_reaction_derived", FRICTION, None, None));
    out
}

/// Every value outside the binding's tolerance (module docs).
fn value_mismatches(case: &Value, envelope: &Value) -> Vec<String> {
    let want = expected(case);
    let got = observed(envelope);
    let floor = |family: Family| want.values().filter(|(_, f)| *f == family).map(|(v, _)| v.abs()).fold(0.0, f64::max);
    let mut found = Vec::new();
    for (key, (expected, family)) in &want {
        let observed = got[key];
        let tolerance = 1e-9 * expected.abs().max(floor(*family));
        if (observed - expected).abs() > tolerance {
            found.push(format!("{key}: observed {observed:e}, expected {expected:e} (tolerance {tolerance:e})"));
        }
    }
    found
}

/// The product tokens of the reference's states: one-way active/inactive;
/// friction stick = sticking, slide± = sliding with that slip sign.
fn state_mismatches(case: &Value, envelope: &Value) -> Vec<String> {
    let mut found = Vec::new();
    let one_way = case["states"]["one-way S"].as_str().unwrap();
    let friction = case["states"]["friction S"].as_str().unwrap();
    let token = state_token(envelope, ONE_WAY);
    if token != one_way {
        found.push(format!("one-way: {token}, reference {one_way}"));
    }
    let (want, sense) = match friction {
        "stick" => ("sticking", 0.0),
        "slide+" => ("sliding", 1.0),
        "slide-" => ("sliding", -1.0),
        "inactive" => ("inactive", 0.0),
        other => panic!("{other}"),
    };
    let token = state_token(envelope, FRICTION);
    let slip = row(envelope, "nonlinear_support_final_displacement", FRICTION, None, None);
    if token != want || (sense != 0.0 && slip * sense <= 0.0) {
        found.push(format!("friction: {token} with slip {slip:e}, reference {friction}"));
    }
    found
}

/// The cases whose state the reference settles and strictly confirms.
fn settled(case: &Value) -> bool {
    case["strict confirmation"]["establishes"] == true
}

#[test]
fn every_settled_case_solves_to_the_reference_in_both_modes() {
    let references = references();
    let mut failures = Vec::new();
    for case in cases(&references).iter().filter(|c| settled(c)) {
        let id = case["id"].as_str().unwrap();
        let document = document(&references, case);
        for mode in MODES {
            let label = format!("{id} [{}]", mode.as_str());
            let envelope = solve(&document, mode);
            if envelope["status"]["mechanics"] != "MECHANICS_SOLVED" || !blocking(&envelope).is_empty() {
                failures.push(format!("{label}: not solved: {:#?}", blocking(&envelope)));
                continue;
            }
            failures.extend(state_mismatches(case, &envelope).into_iter().map(|m| format!("{label}: {m}")));
            failures.extend(value_mismatches(case, &envelope).into_iter().map(|m| format!("{label}: {m}")));
            failures.extend(normal_rule_violation(&envelope).into_iter().map(|m| format!("{label}: {m}")));
            let (code, message) = standing(&envelope);
            if code != SENSITIVE || !message.contains(&format!("not established at 128-bit precision ({NOT_BASE})")) {
                failures.push(format!("{label}: standing {code}: {message}"));
            }
            if let Some(error) = reader(&envelope, Some(&document)) {
                failures.push(format!("{label}: the reader with the model refuses: {error}"));
            }
            if let Some(error) = reader(&envelope, None) {
                failures.push(format!("{label}: the reader without the model refuses: {error}"));
            }
            let numerical_use = numerical_use(&envelope, &document);
            if numerical_use != "needs_recompute" {
                failures.push(format!("{label}: readable and sensitive, yet its numerical use is {numerical_use}"));
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// Readable is not eligible: a sensitive result's numerical-use standing, with
/// every load case requested, is `needs_recompute`. The desktop's rule check
/// binds solver results only at `numerically_eligible`
/// (`qualify_rule_mechanics_with_context`; DEL-06-03 AC-004), so these results
/// authorize no rule outcome.
fn numerical_use(envelope: &Value, document: &Value) -> &'static str {
    let bases = document["model"]["load_cases"].as_array().unwrap().iter()
        .map(|case| json!({"ref_type": "load_case", "ref_id": case["id"]}))
        .collect::<Vec<_>>();
    semantic_contract::numerical_use_standing_with_context(envelope, &bases, None)
}


/// The published derived normal follows DEL-04-04 REQ-012's rule bit for bit:
/// +0 while the one-way is inactive (whatever its published reaction, there the
/// loop's residual), and its published bearing reaction while it is active.
fn normal_rule_violation(envelope: &Value) -> Option<String> {
    let normal = row(envelope, "nonlinear_support_friction_normal_reaction_derived", FRICTION, None, None);
    let source = row(envelope, "support_reaction_component_v2", ONE_WAY, None, Some("Fz"));
    let expected = if state_token(envelope, ONE_WAY) == "active" && source > 0.0 { source } else { 0.0 };
    (normal.to_bits() != expected.to_bits()).then(|| format!("normal {normal:e} is not the rule's {expected:e} (one-way Fz {source:e})"))
}

fn case<'a>(references: &'a Value, id: &str) -> &'a Value {
    cases(references).iter().find(|c| c["id"] == id).unwrap_or_else(|| panic!("no case {id}"))
}

/// A lifted case's one-way publishes a nonzero residual (non-Passed, kd5 §1.2:
/// inspection evidence, not zero), and the derived normal is still exactly +0:
/// the old |R| rule would have published that residual as a bearing normal.
/// (The inactive friction support's own published reaction is likewise its
/// free row's residual in a non-Passed case, not exact zero.)
#[test]
fn a_lifted_shoe_publishes_a_zero_normal_beside_a_nonzero_residual() {
    let references = references();
    let envelope = solve(&document(&references, case(&references, "LIFT-0")), PreviewSolverMode::SparseInteractive);
    assert_eq!(state_token(&envelope, ONE_WAY), "inactive");
    assert_eq!(state_token(&envelope, FRICTION), "inactive");
    assert_ne!(row(&envelope, "support_reaction_component_v2", ONE_WAY, None, Some("Fz")), 0.0, "the residual this pin needs");
    assert_eq!(row(&envelope, "nonlinear_support_friction_normal_reaction_derived", FRICTION, None, None).to_bits(), 0.0_f64.to_bits());
}

/// The two boundary witnesses (README "Boundary witnesses"): the reference
/// confirms neither, and the product confirms neither. STICK-BOUNDARY
/// classifies as stick (|T| ≤ μN with equality). LIFT-BOUNDARY's classification
/// is not settled by the scope (open reading 1): the product either publishes a
/// classification-consistent pair at the reference's values, or does not settle
/// within its iteration cap and refuses the case by its named non-convergence,
/// never a third outcome.
#[test]
fn the_boundary_witnesses_are_not_confirmed() {
    let references = references();
    for mode in MODES {
        let stick = solve(&document(&references, case(&references, "STICK-BOUNDARY")), mode);
        assert!(blocking(&stick).is_empty(), "{:#?}", blocking(&stick));
        assert_eq!((state_token(&stick, ONE_WAY), state_token(&stick, FRICTION)), ("active".to_string(), "sticking".to_string()));
        assert_eq!(value_mismatches(case(&references, "STICK-BOUNDARY"), &stick), Vec::<String>::new());
        assert_eq!(standing(&stick).0, SENSITIVE);
        assert!(standing(&stick).1.contains(NOT_BASE));

        let witness = case(&references, "LIFT-BOUNDARY");
        let lift = solve(&document(&references, witness), mode);
        if lift["status"]["mechanics"] == "MECHANICS_SOLVED" {
            let pair = (state_token(&lift, ONE_WAY), state_token(&lift, FRICTION));
            assert!(
                pair == ("inactive".into(), "inactive".into()) || pair == ("active".into(), "sliding".into()),
                "{mode:?}: {pair:?}"
            );
            assert_eq!(value_mismatches(witness, &lift), Vec::<String>::new());
            assert_eq!(standing(&lift).0, SENSITIVE);
            assert_eq!(normal_rule_violation(&lift), None);
        } else {
            let codes = blocking(&lift).iter().map(|d| d["code"].as_str().unwrap().to_string()).collect::<Vec<_>>();
            assert_eq!(codes, ["NONLINEAR_SUPPORT_NONCONVERGENCE", "SOLVER_SYSTEM_BLOCKED"], "{mode:?}");
            assert!(lift["results"].as_array().unwrap().is_empty(), "{mode:?}");
        }
    }
}

/// The sources that stay refused, each by name, with no case published.
#[test]
fn the_remaining_normal_sources_are_refused_by_name() {
    let references = references();
    let stick = case(&references, "STICK");
    let source = |dof: &str| json!({"support_ref": ONE_WAY, "dof": dof});
    let refusals = [
        ("a gap source", json!({"behavior": "gap", "dof": "UZ", "initial_state": "inactive", "closes_when": "negative_displacement", "gap": quantity(0.001, "m")}), source("UZ"), "NONLINEAR_FRICTION_NORMAL_SOURCE_INVALID"),
        ("a friction source", json!({"behavior": "friction", "dof": "UZ", "initial_state": "sticking", "friction_coefficient": quantity(0.375, "1"), "normal_reaction": quantity(1.0, "N")}), source("UZ"), "NONLINEAR_FRICTION_NORMAL_SOURCE_INVALID"),
        ("a rotational DOF", json!({"behavior": "one_way", "dof": "RY", "initial_state": "active", "active_when": "positive_reaction"}), source("RY"), "NONLINEAR_FRICTION_NORMAL_SOURCE_DOF_INVALID"),
        ("a DOF other than the one-way's own", one_way_record(), source("UX"), "NONLINEAR_FRICTION_NORMAL_SOURCE_DOF_UNRESTRAINED"),
        ("an unknown support", one_way_record(), json!({"support_ref": "support:none", "dof": "UZ"}), "NONLINEAR_FRICTION_NORMAL_SOURCE_UNKNOWN"),
    ];
    for (label, one_way, source, code) in refusals {
        for mode in MODES {
            let envelope = solve(&document_with(&references, stick, one_way.clone(), source.clone()), mode);
            let codes = blocking(&envelope).iter().map(|d| d["code"].as_str().unwrap()).collect::<Vec<_>>();
            assert!(codes.contains(&code), "{label} [{mode:?}]: {codes:?}");
            assert!(envelope["results"].as_array().unwrap().is_empty(), "{label} [{mode:?}]");
        }
    }
    // Explicit and derived together.
    let mut both = document(&references, stick);
    both["model"]["supports"][2]["nonlinear"]["normal_reaction"] = quantity(1.0, "N");
    let envelope = solve(&both, PreviewSolverMode::SparseInteractive);
    let codes = blocking(&envelope).iter().map(|d| d["code"].as_str().unwrap()).collect::<Vec<_>>();
    assert!(codes.contains(&"NONLINEAR_FRICTION_NORMAL_REACTION_AMBIGUOUS"), "{codes:?}");
}

fn set_rows(envelope: &mut Value, kind: &str, entity: &str, edit: &dyn Fn(&mut Value)) {
    let mut hit = 0;
    for row in envelope["results"].as_array_mut().unwrap() {
        if row["kind"] == kind && row["entity_ref"] == entity {
            edit(row);
            hit += 1;
        }
    }
    assert!(hit > 0, "{kind} {entity}");
}

/// The reader's rule (`result_export::nonlinear_support_evidence`) from the
/// refusing side: a published normal that is not the rule's value; a nonzero
/// normal beside an inactive source; and a pulling reaction passed as a normal,
/// which only the model's bearing sense shows (without the model the reader
/// admits it: an active source's normal there is |R|, as for a linear source).
#[test]
fn the_reader_refuses_a_normal_the_rule_does_not_give() {
    let references = references();
    for (id, forge, without, with) in [
        ("SLIDE-POS-B", "a doubled normal", Some("NORMAL"), Some("NORMAL")),
        ("LIFT-0", "the inactive source's residual as the normal", Some("NORMAL"), Some("NORMAL")),
        ("SLIDE-POS-B", "a pulling reaction under the model's opposite sense", None, Some("MODEL_NORMAL")),
    ] {
        let document = document(&references, case(&references, id));
        // Sparse: there LIFT-0's inactive one-way publishes a nonzero residual.
        let mut envelope = solve(&document, PreviewSolverMode::SparseInteractive);
        let mut model_document = document.clone();
        match forge {
            "a doubled normal" => set_rows(&mut envelope, "nonlinear_support_friction_normal_reaction_derived", FRICTION, &|r| r["value"] = json!(2.0 * r["value"].as_f64().unwrap())),
            "the inactive source's residual as the normal" => {
                let residual = row(&envelope, "support_reaction_component_v2", ONE_WAY, None, Some("Fz")).abs();
                assert!(residual > 0.0);
                set_rows(&mut envelope, "nonlinear_support_friction_normal_reaction_derived", FRICTION, &|r| r["value"] = json!(residual));
            }
            _ => model_document["model"]["supports"][1]["nonlinear"]["active_when"] = json!("negative_reaction"),
        }
        let code = |found: Option<String>| found.map(|e| e.split("SOURCE_PHYSICS_NONLINEAR_").nth(1).map(|t| t.split('"').next().unwrap().to_string()).unwrap_or(e));
        assert_eq!(code(reader(&envelope, None)).as_deref(), without, "{id}: {forge}, without the model");
        assert_eq!(code(reader(&envelope, Some(&model_document))).as_deref(), with, "{id}: {forge}, with the model");
    }
}

/// LIM-019's product witness: the NI cycle's off-axis chord
/// (`oneway_source_tests.rs`, `a_lift_off_friction_cycle_ends_in_named_non_convergence`)
/// as a document, with this file's section, material, one-way source and friction
/// support (μ = 3/10). One member A(0,0,0)→S(3,−4,0); S's UZ and rotations are
/// held; −100 N along global X and +3 N along global Y at S. The iteration cycles
/// although an admissible state exists, and both modes refuse the mechanics
/// result by name: non-convergence, the blocked solve, and no result row. This
/// asserts the refusal only, not the iteration's path or its last iterate.
#[test]
fn a_lift_off_friction_cycle_is_refused_by_name_with_no_result() {
    let references = references();
    let mut document = document(&references, case(&references, "STICK"));
    let model = &mut document["model"];
    model["nodes"] = json!([
        {"id": node_id("A"), "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROVENANCE},
        {"id": node_id("S"), "position": {"x": 3.0, "y": -4.0, "z": 0.0}, "provenance": PROVENANCE},
    ]);
    let pipe = model["pipe_segments"][0].clone();
    model["pipe_segments"] = json!([pipe]);
    model["pipe_segments"][0]["to"] = json!(node_id("S"));
    model["supports"][1]["nonlinear"]["dof"] = json!("UY");
    model["supports"][2]["nonlinear"]["normal_reaction_source"]["dof"] = json!("UY");
    model["supports"][2]["nonlinear"]["friction_coefficient"] = quantity(0.3, "1");
    model["supports"].as_array_mut().unwrap().push(json!({"id": "support:out-of-plane-S", "node": node_id("S"), "family": "anchor",
        "restraints": ["UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE}));
    model["load_cases"][0]["primitive_loads"] = json!([
        {"id": "load:x", "category": "concentrated_force", "target": {"type": "node", "node": node_id("S")}, "direction": "global_x",
            "magnitude": quantity(-100.0, "N"), "dimension": "force", "provenance": PROVENANCE},
        {"id": "load:y", "category": "concentrated_force", "target": {"type": "node", "node": node_id("S")}, "direction": "global_y",
            "magnitude": quantity(3.0, "N"), "dimension": "force", "provenance": PROVENANCE},
    ]);
    for mode in MODES {
        let envelope = solve(&document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        let mut codes = blocking(&envelope).iter().map(|d| d["code"].as_str().unwrap()).collect::<Vec<_>>();
        codes.sort_unstable();
        assert_eq!(codes, ["NONLINEAR_SUPPORT_NONCONVERGENCE", "SOLVER_SYSTEM_BLOCKED"], "{mode:?}");
        assert!(envelope["results"].as_array().unwrap().is_empty(), "{mode:?}");
    }
}
