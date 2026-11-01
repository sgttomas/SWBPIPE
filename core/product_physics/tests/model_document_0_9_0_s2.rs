//! Model document 0.9.0, slice S2 (DEL-02-01 Work `model-document-0.9.0.md` §1.3,
//! §1.5, §1.7, §1.8 and §5's S2 row): the pressure-free resolution of a case's
//! `condition_use` through the public value entry the desktop and the runner use,
//! and the solve's public pre-formation checks, in both solver modes.
//!
//! Expectations are independent of the product's lowering: the twin T(D) comes
//! from `common/condition_twin.rs` (written from §1.5's text, its temperature
//! differences read from the Python reference table), and the mechanics values
//! from the closed forms in `validation/references/rt_g5_s2_conditions/`.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, LinearStaticPreviewRequest, PreviewSolverMode, MODEL_SCHEMA_VERSION,
};
use serde_json::{json, Value};

#[path = "common/condition_twin.rs"]
mod twin;
use twin::PROV;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const EXACT: &str = include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json");

fn document() -> Value {
    twin::document(MODEL_SCHEMA_VERSION)
}

fn solve(model: &Value, mode: PreviewSolverMode) -> Value {
    let materials = if model.get("pressure_contract").is_some() {
        serde_json::from_str::<Value>(EXACT).unwrap()["materials"].clone()
    } else {
        json!([])
    };
    let envelope = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": materials}), mode)
        .unwrap_or_else(|refusal| panic!("refused at the request boundary: {refusal}"));
    serde_json::to_value(envelope).unwrap()
}

fn pre_formation(model: &Value) -> Vec<Value> {
    LinearStaticPreviewRequest::pre_formation_checks_value(json!({"model": model, "materials": []}))
        .unwrap()
        .into_iter()
        .map(|d| serde_json::to_value(d).unwrap())
        .collect()
}

fn text(value: &Value) -> String {
    serde_json::to_string(value).unwrap()
}

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

fn case_rows<'a>(envelope: &'a Value, case: &str) -> Vec<&'a Value> {
    envelope["results"].as_array().unwrap().iter()
        .filter(|r| r["basis_ref"]["ref_type"] == "load_case" && r["basis_ref"]["ref_id"] == case).collect()
}

/// E2: D and its independent twin T(D) solve to byte-identical envelopes on both
/// modes. On the pressure-free route no envelope field binds the authored
/// document's hash (as E5 found at S1), so nothing is excepted. The public
/// pre-formation checks agree too.
#[test]
fn e2_the_authored_conditions_solve_as_their_twin_byte_for_byte() {
    let d = document();
    let t = twin::twin(&d);
    assert!(t.get("condition_sets").is_none() && t["load_cases"][0].get("condition_use").is_none());
    for mode in MODES {
        let (solved, reference) = (solve(&d, mode), solve(&t, mode));
        assert_eq!(solved["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:?}", diagnostics(&solved));
        for case in ["OPE", "SUS", "WIND", "PLAIN"] {
            assert!(!case_rows(&solved, case).is_empty(), "{mode:?}: {case} publishes");
        }
        // The difference-basis combination keeps its interim withholding (1b is not built).
        assert!(diagnostics(&solved).iter().any(|(code, severity, _)| code == "LOAD_COMBINATION_DIFFERENCE_BASIS_UNSUPPORTED" && severity == "warning"));
        assert_same_bytes(&solved, &reference, &format!("{mode:?}: D against T(D)"));
    }
    assert_eq!(text(&json!(pre_formation(&d))), text(&json!(pre_formation(&t))), "pre-formation checks");
}

/// E2 on variants: no set exposed (the existing finding names the empty
/// exposure), and sets without added weight (they contribute none).
#[test]
fn e2_variants_solve_or_refuse_as_their_twins() {
    let mut none_exposed = document();
    for set in none_exposed["condition_sets"].as_array_mut().unwrap() {
        set.as_object_mut().unwrap().remove("exposure");
    }
    let mut no_weight = document();
    for set in no_weight["condition_sets"].as_array_mut().unwrap() {
        set.as_object_mut().unwrap().remove("added_weight");
    }
    for (name, d) in [("none_exposed", none_exposed), ("no_weight", no_weight)] {
        for mode in MODES {
            assert_same_bytes(&solve(&d, mode), &solve(&twin::twin(&d), mode), &format!("{name} {mode:?}"));
        }
        assert_eq!(text(&json!(pre_formation(&d))), text(&json!(pre_formation(&twin::twin(&d)))), "{name}");
    }
    let mut none_exposed = document();
    for set in none_exposed["condition_sets"].as_array_mut().unwrap() {
        set.as_object_mut().unwrap().remove("exposure");
    }
    let found = diagnostics(&solve(&none_exposed, MODES[0]));
    assert!(found.iter().any(|(code, severity, message)| code == "EQUIVALENT_STATIC_INPUT_MISSING" && severity == "blocking"
        && message.contains("exposed_pipe_refs or exposed_spans")), "{found:?}");
}

/// E4: order is part of the definition (§1.5 "Order"). OPE authors weights 1e8,
/// -1e8 and -0.3 N/m on P2 and its set adds -0.3 N/m. As built, the pressure-free
/// route sums a span's line-load intensities in one exact accumulator rounded once
/// (S11 §4.4 E7), so the published envelope does not depend on record order: the
/// mutant twin with generated records first solves to the same bytes. The order is
/// therefore checked where it is observable, in the lowered typed model (E1, which
/// refuses the prepended and set-order mutants), and here D solves as the governing
/// twin.
#[test]
fn e4_generated_records_follow_the_authored_ones() {
    let d = document();
    let prepended = twin::twin_with(&d, twin::Order::Prepended);
    assert_ne!(text(&prepended), text(&twin::twin(&d)), "the mutant twin is a different document");
    for mode in MODES {
        let solved = solve(&d, mode);
        assert_eq!(solved["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:?}", diagnostics(&solved));
        assert_same_bytes(&solved, &solve(&twin::twin(&d), mode), &format!("{mode:?}: D against the governing twin"));
        assert_same_bytes(&solved, &solve(&prepended, mode), &format!("{mode:?}: exact intensity sums make record order unobservable here"));
    }
}

/// E5 for S2's document: with every `condition_use` removed it solves exactly as
/// the same document without sets and references; and a set no pipe carries
/// changes nothing in a solved document that uses conditions.
#[test]
fn e5_unused_sets_and_the_no_condition_baseline() {
    let mut unused = document();
    for case in unused["load_cases"].as_array_mut().unwrap() {
        case.as_object_mut().unwrap().remove("condition_use");
    }
    let mut bare = unused.clone();
    bare.as_object_mut().unwrap().remove("condition_sets");
    for pipe in bare["pipe_segments"].as_array_mut().unwrap() {
        pipe.as_object_mut().unwrap().remove("condition_set_ref");
    }
    // WIND without its use has no marks: both refuse it alike. Keep the rest solving.
    unused["load_cases"].as_array_mut().unwrap().remove(2);
    bare["load_cases"].as_array_mut().unwrap().remove(2);
    let mut extra = document();
    extra["condition_sets"].as_array_mut().unwrap().push(json!({"id": "cs:spare", "provenance": PROV,
        "conditions": [{"key": "OPE1", "temperature": {"value": 999.0, "unit": "degC"}}], "added_weight": {"magnitude": {"value": -9.0, "unit": "N/m"}, "direction": "global_x"}}));
    for mode in MODES {
        let solved = solve(&unused, mode);
        assert_eq!(solved["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:?}", diagnostics(&solved));
        assert_same_bytes(&solved, &solve(&bare, mode), &format!("{mode:?}: no condition use"));
        assert_same_bytes(&solve(&extra, mode), &solve(&document(), mode), &format!("{mode:?}: an unassigned set"));
    }
}

/// The blocking findings of a document on both modes and the pre-formation path;
/// it publishes no result row.
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
    let found = first.unwrap();
    let checks: Vec<(String, String, String)> = pre_formation(document).iter()
        .map(|d| (d["code"].as_str().unwrap().into(), d["severity"].as_str().unwrap().into(), d["message"].as_str().unwrap().into())).collect();
    for item in found.iter().filter(|(_, severity, _)| severity == "blocking") {
        assert!(checks.contains(item), "the pre-formation path reports {item:?}: {checks:?}");
    }
    found
}

fn assert_refused_by(document: &Value, code: &str, parts: &[&str]) {
    let found = refused(document);
    assert!(found.iter().any(|(c, severity, message)| c == code && severity == "blocking" && parts.iter().all(|p| message.contains(p))),
        "{code} {parts:?} in {found:?}");
}

fn mutated(mutate: impl FnOnce(&mut Value)) -> Value {
    let mut d = document();
    mutate(&mut d);
    d
}

fn set_mut<'a>(d: &'a mut Value, id: &str) -> &'a mut Value {
    d["condition_sets"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == id).unwrap()
}

/// E6: every case-level code of §1.3 at the solve and the pre-formation path,
/// naming the case, pipe, set, key and quantity.
#[test]
fn e6_every_case_level_code_refuses_by_name() {
    // A pipe without a set, for each use the case states.
    let missing = mutated(|d| { d["pipe_segments"][1].as_object_mut().unwrap().remove("condition_set_ref"); });
    let found = refused(&missing);
    let set_missing: Vec<&str> = found.iter().filter(|(c, _, _)| c == "OPERATING_CONDITION_SET_MISSING").map(|(_, _, m)| m.as_str()).collect();
    assert_eq!(set_missing.len(), 4, "OPE temperature and added weight, SUS added weight, WIND exposure: {set_missing:?}");
    assert!(set_missing.iter().all(|m| m.contains("pipe `P2` carries no condition set")));
    assert!(set_missing[0].contains("load case `OPE` uses temperature") && set_missing[1].contains("load case `OPE` uses added_weight")
        && set_missing[3].contains("load case `WIND` uses wind_exposure"), "{set_missing:?}");
    assert_refused_by(&mutated(|d| d["load_cases"][0]["condition_use"]["temperature"] = json!("HYDRO")),
        "OPERATING_CONDITION_KEY_MISSING", &["load case `OPE`", "condition set `cs:warm` of pipe `P1` has no condition `HYDRO`"]);
    assert_refused_by(&mutated(|d| { set_mut(d, "cs:hot")["conditions"][1].as_object_mut().unwrap().remove("temperature"); }),
        "OPERATING_CONDITION_VALUE_MISSING", &["load case `OPE`", "condition `OPE1` of set `cs:hot` (pipe `P2`) states no temperature"]);
    assert_refused_by(&mutated(|d| { set_mut(d, "cs:cold").as_object_mut().unwrap().remove("installation_temperature"); }),
        "OPERATING_CONDITION_INSTALLATION_TEMPERATURE_MISSING", &["load case `OPE`", "condition set `cs:cold` of pipe `P3` states no installation_temperature"]);
    assert_refused_by(&mutated(|d| d["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(json!({"id": "OPE:dt", "category": "thermal",
            "target": {"type": "element", "pipe": "P1"}, "direction": "global_x", "magnitude": {"value": 5.0, "unit": "K"}, "dimension": "temperature_change", "provenance": PROV}))),
        "LOAD_CASE_CONDITION_TEMPERATURE_CONFLICT", &["load case `OPE`", "thermal primitive `OPE:dt`"]);
    assert_refused_by(&mutated(|d| d["load_cases"][2]["equivalent_static"]["wind"]["exposed_pipe_refs"] = json!(["P1"])),
        "LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT", &["load case `WIND`", "equivalent_static.wind.exposed_pipe_refs"]);
    assert_refused_by(&mutated(|d| d["load_cases"][2]["equivalent_static"]["wind"]["exposed_spans"] = json!([{"pipe_ref": "P1",
            "start_fraction": {"value": 0.0, "unit": "1"}, "end_fraction": {"value": 0.5, "unit": "1"}}])),
        "LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT", &["load case `WIND`", "equivalent_static.wind.exposed_spans"]);
    assert_refused_by(&mutated(|d| { d["load_cases"][2].as_object_mut().unwrap().remove("equivalent_static"); }),
        "LOAD_CASE_CONDITION_WIND_INPUT_MISSING", &["load case `WIND`", "no equivalent_static.wind input"]);
    assert_refused_by(&mutated(|d| d["load_cases"][2]["equivalent_static"] = json!({"seismic": {"gravity_acceleration": {"value": 9.81, "unit": "m/s^2"}}, "provenance": PROV})),
        "LOAD_CASE_CONDITION_WIND_INPUT_MISSING", &["load case `WIND`"]);
    assert_refused_by(&mutated(|d| d["load_cases"][1]["condition_use"]["pressure"] = json!("OPE1")),
        "LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT", &["load case `SUS` uses condition `OPE1` for pressure"]);
    assert_refused_by(&mutated(|d| d["load_cases"][1]["condition_use"]["contents_density"] = json!("OPE1")),
        "LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED", &["load case `SUS` uses condition `OPE1` for contents density"]);
    // A generated id meets the existing duplicate-id check.
    assert_refused_by(&mutated(|d| d["load_cases"][3]["primitive_loads"][0]["id"] = json!("load:condition-temperature:3:OPE:2:P1")),
        "DUPLICATE_ID", &["primitive-load IDs must be unique"]);
    // Other members keep their refusals beside a resolvable use.
    assert_refused_by(&mutated(|d| d["load_cases"][0]["test_condition"] = json!({"kind": "hydrostatic", "held_supports": "none"})),
        "LOAD_CASE_TEST_CONDITION_UNSUPPORTED", &["load case `OPE`"]);
    assert_refused_by(&mutated(|d| d["supports"][2]["direction"] = json!({"x": 0.0, "y": 3.0, "z": 4.0})),
        "SUPPORT_DIRECTION_SKEW_UNSUPPORTED", &["two or more nonzero components"]);
    // Document-level codes keep their names whether or not a case uses the set.
    assert_refused_by(&mutated(|d| set_mut(d, "cs:warm")["conditions"][0]["temperature"] = json!({"value": 1.5e-24, "unit": "K"})),
        "OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED", &["condition_sets[2].conditions[0].temperature is outside"]);
    assert_refused_by(&mutated(|d| set_mut(d, "cs:warm")["installation_temperature"] = json!({"value": -500.0, "unit": "degF"})),
        "OPERATING_CONDITION_QUANTITY_INVALID", &["below zero kelvin"]);
}

/// S1's blanket code is gone from the solve, and the exact route still refuses
/// every use by name until S4.
#[test]
fn e6_the_exact_route_refuses_every_use_and_the_blanket_code_is_gone() {
    let found = diagnostics(&solve(&document(), MODES[0]));
    assert!(!found.iter().any(|(code, _, _)| code == "LOAD_CASE_CONDITION_USE_UNSUPPORTED"), "{found:?}");
    let mut exact: Value = serde_json::from_str::<Value>(EXACT).unwrap()["model"].take();
    exact["schema_version"] = json!(MODEL_SCHEMA_VERSION);
    exact["condition_sets"] = json!([{"id": "cs", "provenance": PROV, "installation_temperature": {"value": 20.0, "unit": "degC"},
        "conditions": [{"key": "OPE1", "temperature": {"value": 150.0, "unit": "degC"}}], "added_weight": {"magnitude": {"value": -1.0, "unit": "N/m"}, "direction": "global_z"},
        "exposure": {"wind": true}}]);
    for pipe in exact["pipe_segments"].as_array_mut().unwrap() {
        pipe["condition_set_ref"] = json!("cs");
    }
    for using in [json!({"temperature": "OPE1"}), json!({"added_weight": true}), json!({"wind_exposure": true}), json!({"pressure": "OPE1"})] {
        let mut d = exact.clone();
        d["load_cases"][0]["condition_use"] = using.clone();
        let found = refused(&d);
        assert!(found.iter().any(|(code, _, _)| code == "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED"), "{using}: {found:?}");
        assert!(!found.iter().any(|(code, _, _)| code.starts_with("OPERATING_CONDITION_") || code.starts_with("LOAD_CASE_CONDITION_PRESSURE")), "{using}: {found:?}");
    }
}

/// Set order in `condition_sets` does not change the solve: the lowering walks
/// pipes, not sets.
#[test]
fn set_order_does_not_change_the_solve() {
    let mut reversed = document();
    reversed["condition_sets"].as_array_mut().unwrap().reverse();
    for mode in MODES {
        assert_same_bytes(&solve(&reversed, mode), &solve(&document(), mode), &format!("{mode:?}"));
    }
}

/// The typed ΔT route is unchanged beside conditions: PLAIN's typed 7 degF
/// primitives publish the same rows whether the other cases resolve conditions or
/// carry no use at all (the same case count, so row ids are comparable).
#[test]
fn the_typed_temperature_route_is_unchanged_beside_conditions() {
    let mut using = document();
    using["load_cases"].as_array_mut().unwrap().remove(2);
    let mut unused = using.clone();
    for case in unused["load_cases"].as_array_mut().unwrap() {
        case.as_object_mut().unwrap().remove("condition_use");
    }
    for mode in MODES {
        let (a, b) = (solve(&using, mode), solve(&unused, mode));
        assert_eq!(a["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:?}", diagnostics(&a));
        assert!(!case_rows(&a, "PLAIN").is_empty());
        assert_eq!(text(&json!(case_rows(&a, "PLAIN"))), text(&json!(case_rows(&b, "PLAIN"))), "{mode:?}");
        assert_ne!(text(&json!(case_rows(&a, "OPE"))), text(&json!(case_rows(&b, "OPE"))), "{mode:?}: the conditions act on OPE");
    }
}

// ------------------------------------------------------------------ mechanics references

fn mechanics() -> Value {
    twin::references()["mechanics"].clone()
}

fn num(text: &Value) -> f64 {
    text.as_str().unwrap().parse().unwrap()
}

/// A straight member along x with nodes at 0, L/2 and L, from the reference
/// inputs; `supports` are (node, family, restraints).
fn member(case: &str, set: Value, using: Value, supports: &[(&str, &[&str])], equivalent_static: Option<Value>) -> Value {
    let m = mechanics();
    let inputs = &m["inputs"];
    let length = num(&inputs["length_m"]);
    let node = |id: &str, x: f64| json!({"id": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV});
    let pipe = |id: &str, from: &str, to: &str| json!({"id": id, "from": from, "to": to, "material": "mat", "condition_set_ref": "cs",
        "y_reference": {"x": 0, "y": 0, "z": 1}, "provenance": PROV,
        "section": {"outside_diameter": {"value": num(&inputs["outside_diameter_m"]), "unit": "m"}, "wall_thickness": {"value": num(&inputs["wall_thickness_m"]), "unit": "m"}}});
    let mut load_case = json!({"id": case, "condition_use": using, "primitive_loads": [], "provenance": PROV});
    if let Some(generation) = equivalent_static {
        load_case["equivalent_static"] = generation;
    }
    json!({
        "schema_version": MODEL_SCHEMA_VERSION,
        "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": "invented:rt-g5-s2-mechanics", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC", "stress": "Pa"}},
        "nodes": [node("A", 0.0), node("B", length / 2.0), node("C", length)],
        "pipe_segments": [pipe("AB", "A", "B"), pipe("BC", "B", "C")],
        "materials": [{"id": "mat", "elastic_modulus": {"value": num(&inputs["elastic_modulus_pa"]), "unit": "Pa"},
            "shear_modulus": {"value": num(&inputs["shear_modulus_pa"]), "unit": "Pa"},
            "thermal_expansion_coefficient": {"value": num(&inputs["thermal_expansion_per_degc"]), "unit": "1/degC"}, "provenance": PROV}],
        "supports": supports.iter().map(|(node, restraints)| json!({"id": format!("S{node}"), "node": node, "family": "anchor", "restraints": restraints, "provenance": PROV})).collect::<Vec<_>>(),
        "condition_sets": [set],
        "load_cases": [load_case],
        "combinations": [],
    })
}

const ANCHOR: &[&str] = &["UX", "UY", "UZ", "RX", "RY", "RZ"];

fn si(row: &Value) -> f64 {
    let value = row["value"].as_f64().unwrap();
    match row["unit"].as_str().unwrap() {
        "mm" => value / 1000.0,
        "N" | "m" | "rad" | "Pa" => value,
        "MPa" => value * 1.0e6,
        "kN" => value * 1000.0,
        other => panic!("unit {other}"),
    }
}

fn find<'a>(envelope: &'a Value, kind: &str, entity: &str, component: Option<&str>) -> Vec<&'a Value> {
    envelope["results"].as_array().unwrap().iter()
        .filter(|r| r["kind"] == kind && r["entity_ref"] == entity && component.is_none_or(|c| r["metadata"]["component"] == c))
        .collect()
}

fn one(envelope: &Value, kind: &str, entity: &str, component: Option<&str>) -> f64 {
    let rows = find(envelope, kind, entity, component);
    assert_eq!(rows.len(), 1, "{kind} {entity} {component:?}");
    si(rows[0])
}

fn close(got: f64, want: f64, floor: f64, what: &str) {
    let tolerance = mechanics()["relative_tolerance"].as_f64().unwrap();
    assert!((got - want).abs() <= tolerance * want.abs() + floor, "{what}: {got:e} against {want:e}");
}

fn solved_both_modes(document: &Value) -> Vec<Value> {
    MODES.iter().map(|&mode| {
        let solved = solve(document, mode);
        assert_eq!(solved["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:?}", diagnostics(&solved));
        assert_same_bytes(&solved, &solve(&twin::twin(document), mode), &format!("{mode:?}: the mechanics document solves as its twin"));
        solved
    }).collect()
}

fn reference_case(id: &str) -> Value {
    mechanics()["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap().clone()
}

/// Restrained thermal expansion: the anchors carry ±E·A·α·ΔT along the pipe, the
/// midspan does not move, and the member force is E·A·α·ΔT in compression.
#[test]
fn mechanics_restrained_thermal_from_conditions() {
    let case = reference_case("restrained_thermal");
    let (expected, conditions) = (&case["expected"], &case["conditions"]);
    let set = json!({"id": "cs", "provenance": PROV, "installation_temperature": conditions["installation"], "conditions": [{"key": "OPE", "temperature": conditions["temperature"]}]});
    let document = member("L", set, json!({"temperature": "OPE"}), &[("A", ANCHOR), ("C", ANCHOR)], None);
    let floors = &mechanics()["absolute_floor"];
    for solved in solved_both_modes(&document) {
        close(one(&solved, "support_reaction_component_v2", "SA", Some("Fx")), expected["reaction_x_at_start_n"].as_f64().unwrap(), floors["force_n"].as_f64().unwrap(), "R_x at A");
        close(one(&solved, "support_reaction_component_v2", "SC", Some("Fx")), expected["reaction_x_at_end_n"].as_f64().unwrap(), floors["force_n"].as_f64().unwrap(), "R_x at C");
        close(one(&solved, "global_nodal_displacement_x", "B", None), 0.0, floors["displacement_m"].as_f64().unwrap(), "u_x at B");
        let axial: Vec<f64> = find(&solved, "element_local_axial_force", "AB", None).into_iter().chain(find(&solved, "element_local_axial_force", "BC", None)).map(si).collect();
        assert!(!axial.is_empty());
        for value in axial {
            close(value.abs(), expected["axial_force_n_magnitude"].as_f64().unwrap(), floors["force_n"].as_f64().unwrap(), "|N|");
        }
    }
}

/// Free expansion of a cantilever: the tip moves α·ΔT·L with ΔT from degF
/// conditions, and the anchor carries no axial force.
#[test]
fn mechanics_free_expansion_from_fahrenheit_conditions() {
    let case = reference_case("free_expansion");
    let (expected, conditions) = (&case["expected"], &case["conditions"]);
    let set = json!({"id": "cs", "provenance": PROV, "installation_temperature": conditions["installation"], "conditions": [{"key": "OPE", "temperature": conditions["temperature"]}]});
    let document = member("L", set, json!({"temperature": "OPE"}), &[("A", ANCHOR)], None);
    let floors = &mechanics()["absolute_floor"];
    for solved in solved_both_modes(&document) {
        close(one(&solved, "global_nodal_displacement_x", "C", None), expected["tip_displacement_x_m"].as_f64().unwrap(), floors["displacement_m"].as_f64().unwrap(), "u_x at C");
        close(one(&solved, "support_reaction_component_v2", "SA", Some("Fx")), 0.0, floors["force_n"].as_f64().unwrap(), "R_x at A");
    }
}

/// A cantilever under its set's added weight: the Euler-Bernoulli tip deflection
/// and rotation, and the anchor's vertical reaction.
#[test]
fn mechanics_added_weight_cantilever_from_a_set() {
    let case = reference_case("added_weight_cantilever");
    let expected = &case["expected"];
    let set = json!({"id": "cs", "provenance": PROV, "added_weight": case["conditions"]["added_weight"]});
    let document = member("L", set, json!({"added_weight": true}), &[("A", ANCHOR)], None);
    let floors = &mechanics()["absolute_floor"];
    for solved in solved_both_modes(&document) {
        close(one(&solved, "global_nodal_displacement_z", "C", None), expected["tip_displacement_z_m"].as_f64().unwrap(), floors["displacement_m"].as_f64().unwrap(), "u_z at C");
        close(one(&solved, "global_nodal_rotation_y", "C", None), expected["tip_rotation_y_rad"].as_f64().unwrap(), floors["displacement_m"].as_f64().unwrap(), "theta_y at C");
        close(one(&solved, "support_reaction_component_v2", "SA", Some("Fz")), expected["reaction_z_at_start_n"].as_f64().unwrap(), floors["force_n"].as_f64().unwrap(), "R_z at A");
    }
}

/// A cantilever whose set is wind-exposed: the user's pressure, shape factor and
/// direction give q = p·C·OD; tip deflection and reaction follow.
#[test]
fn mechanics_wind_cantilever_from_a_set() {
    let case = reference_case("wind_cantilever");
    let (expected, wind) = (&case["expected"], &case["conditions"]["wind"]);
    let set = json!({"id": "cs", "provenance": PROV, "exposure": {"wind": true}});
    let generation = json!({"wind": {"pressure": {"value": wind["pressure_pa"], "unit": "Pa"}, "shape_factor": {"value": wind["shape_factor"], "unit": "1"}, "direction": wind["direction"]}, "provenance": PROV});
    let document = member("L", set, json!({"wind_exposure": true}), &[("A", ANCHOR)], Some(generation));
    let floors = &mechanics()["absolute_floor"];
    for solved in solved_both_modes(&document) {
        close(one(&solved, "global_nodal_displacement_y", "C", None), expected["tip_displacement_y_m"].as_f64().unwrap(), floors["displacement_m"].as_f64().unwrap(), "u_y at C");
        close(one(&solved, "support_reaction_component_v2", "SA", Some("Fy")), expected["reaction_y_at_start_n"].as_f64().unwrap(), floors["force_n"].as_f64().unwrap(), "R_y at A");
    }
}

// ------------------------------------------------------------------ wind presence

/// An authored empty `exposed_pipe_refs` or `exposed_spans` states the case's own
/// exposure, so a wind-exposure use conflicts with it; without a condition use an
/// authored empty list solves exactly as an absent one (ordinary wind unchanged).
#[test]
fn an_authored_empty_exposure_list_is_a_conflict_and_ordinary_wind_is_unchanged() {
    for member in ["exposed_pipe_refs", "exposed_spans"] {
        assert_refused_by(&mutated(|d| d["load_cases"][2]["equivalent_static"]["wind"][member] = json!([])),
            "LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT", &["load case `WIND`", &format!("equivalent_static.wind.{member}")]);
        let plain = |authored: bool| {
            let mut d = twin::twin(&document());
            let wind = &mut d["load_cases"][2]["equivalent_static"]["wind"];
            wind.as_object_mut().unwrap().remove("exposed_pipe_refs");
            if authored {
                wind[member] = json!([]);
            }
            d
        };
        for mode in MODES {
            let (absent, empty) = (solve(&plain(false), mode), solve(&plain(true), mode));
            assert!(diagnostics(&absent).iter().any(|(code, _, m)| code == "EQUIVALENT_STATIC_INPUT_MISSING" && m.contains("exposed_pipe_refs or exposed_spans")));
            assert_same_bytes(&empty, &absent, &format!("{member} {mode:?}: an authored [] without a condition use"));
        }
    }
}

// ------------------------------------------------------------------ read-only preview

use open_pipe_stress_product_physics::operating_conditions::{preview_operating_conditions, OPERATING_CONDITION_PREVIEW_DOCUMENT_KIND};

/// The preview's generated records and wind ids are the independent twin's, in
/// order; it is deterministic and leaves its input unchanged.
#[test]
fn the_preview_lists_the_twins_generated_records() {
    let d = document();
    let before = d.clone();
    let preview = preview_operating_conditions(&d);
    assert_eq!(d, before, "the input is not changed");
    assert_eq!(text(&preview), text(&preview_operating_conditions(&d)), "deterministic");
    assert_eq!(preview["document_kind"], OPERATING_CONDITION_PREVIEW_DOCUMENT_KIND);
    assert_eq!(preview["resolved"], true);
    assert_eq!(preview["diagnostics"], json!([]));
    let t = twin::twin(&d);
    let cases = preview["cases"].as_array().unwrap();
    assert_eq!(cases.iter().map(|c| c["case_id"].as_str().unwrap()).collect::<Vec<_>>(), ["OPE", "SUS", "WIND"]);
    for case in cases {
        let id = case["case_id"].as_str().unwrap();
        let (authored, lowered) = (d["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap(),
                                   t["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap());
        let generated = &lowered["primitive_loads"].as_array().unwrap()[authored["primitive_loads"].as_array().unwrap().len()..];
        assert_eq!(text(&case["generated_primitive_loads"]), text(&json!(generated)), "{id}");
        let wind = if authored["condition_use"].get("wind_exposure").is_some() { lowered["equivalent_static"]["wind"]["exposed_pipe_refs"].clone() } else { Value::Null };
        assert_eq!(case["wind_exposed_pipe_refs"], wind, "{id}");
    }
}

/// Refused, the preview lists nothing and names exactly the lowering's findings,
/// which open the solve's own pre-formation findings; a request the solve's parse
/// refuses gives the same refusal text.
#[test]
fn the_preview_refuses_as_the_solve_does() {
    let mut exact: Value = serde_json::from_str::<Value>(EXACT).unwrap()["model"].take();
    exact["schema_version"] = json!(MODEL_SCHEMA_VERSION);
    exact["load_cases"][0]["condition_use"] = json!({"added_weight": true});
    for d in [
        mutated(|d| { d["pipe_segments"][1].as_object_mut().unwrap().remove("condition_set_ref"); }),
        mutated(|d| d["load_cases"][1]["condition_use"]["contents_density"] = json!("OPE1")),
        mutated(|d| d["load_cases"][2]["equivalent_static"]["wind"]["exposed_spans"] = json!([])),
        mutated(|d| d["load_cases"][0]["test_condition"] = json!({"kind": "hydrostatic", "held_supports": "none"})),
        mutated(|d| set_mut(d, "cs:warm")["installation_temperature"] = json!({"value": -500.0, "unit": "degF"})),
        exact,
    ] {
        let preview = preview_operating_conditions(&d);
        assert_eq!(preview["resolved"], false, "{preview}");
        assert_eq!(preview["cases"], json!([]));
        let listed = preview["diagnostics"].as_array().unwrap();
        assert!(!listed.is_empty());
        let checks = pre_formation(&d);
        assert_eq!(text(&json!(listed)), text(&json!(checks[..listed.len()])), "the lowering's findings open the pre-formation findings");
    }
    let unknown = mutated(|d| d["condition_sets"][0]["conditions"][0]["temprature"] = json!(1));
    let refusal = LinearStaticPreviewRequest::pre_formation_checks_value(json!({"model": unknown, "materials": []})).unwrap_err();
    let preview = preview_operating_conditions(&unknown);
    assert_eq!(preview["refusal"], json!(refusal));
    assert_eq!((preview["resolved"].clone(), preview["cases"].clone()), (json!(false), json!([])));
}

const PARITY_PATH: &str = "../../fixtures/operating_conditions/preview_parity.json";
const PARITY: &str = include_str!("../../../fixtures/operating_conditions/preview_parity.json");

fn parity_inputs() -> Vec<(&'static str, Value)> {
    let mut exact: Value = serde_json::from_str::<Value>(EXACT).unwrap()["model"].take();
    exact["schema_version"] = json!(MODEL_SCHEMA_VERSION);
    exact["load_cases"][0]["condition_use"] = json!({"temperature": "OPE1"});
    vec![
        ("resolved", document()),
        ("set_missing", mutated(|d| { d["pipe_segments"][1].as_object_mut().unwrap().remove("condition_set_ref"); })),
        ("authored_empty_spans", mutated(|d| d["load_cases"][2]["equivalent_static"]["wind"]["exposed_spans"] = json!([]))),
        ("unknown_field", mutated(|d| d["condition_sets"][0]["conditions"][0]["temprature"] = json!(1))),
        ("exact_route", exact),
    ]
}

/// The native preview of each parity input equals the committed parity fixture,
/// which the wasm engine's `preview_operating_conditions_json` must reproduce
/// (`tests/test_operating_condition_preview_wasm.py`). The fixture is written by
/// this test with `OPENPIPESTRESS_WRITE_PREVIEW_PARITY=1`, never by hand; its
/// numerical values are checked independently above (the twin, the Python table).
#[test]
fn the_native_preview_matches_the_parity_fixture() {
    let rows: Vec<Value> = parity_inputs().into_iter()
        .map(|(name, model)| json!({"name": name, "model": model.clone(), "preview": preview_operating_conditions(&model)})).collect();
    let fixture = json!({
        "description": "Native operating-condition previews (core/product_physics operating_conditions::preview_operating_conditions) of invented documents, for the wasm engine's parity check. Written by core/product_physics/tests/model_document_0_9_0_s2.rs with OPENPIPESTRESS_WRITE_PREVIEW_PARITY=1; never edited by hand.",
        "rows": rows,
    });
    let bytes = serde_json::to_string_pretty(&fixture).unwrap() + "\n";
    if std::env::var_os("OPENPIPESTRESS_WRITE_PREVIEW_PARITY").is_some() {
        std::fs::write(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(PARITY_PATH), &bytes).unwrap();
        return;
    }
    assert_eq!(PARITY, bytes, "the parity fixture is current (rewrite it with OPENPIPESTRESS_WRITE_PREVIEW_PARITY=1, then rerun)");
}
