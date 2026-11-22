//! On-demand model checks: the
//! checks are the solve's own checks before stiffness formation. For every
//! model below, in both solver modes, the findings equal the leading
//! diagnostics of the solve's envelope (id, code, severity and affected refs;
//! the solve may append to a branch warning's message once it has published
//! rows). Where a finding blocks, the solve publishes exactly those findings.
//! A request the solve refuses outright is refused with the same text. All
//! models are invented.
use open_pipe_stress_product_physics::LinearStaticPreviewRequest;

fn preview_model_checks_value(request: Value) -> Result<Vec<open_pipe_stress_product_physics::Diagnostic>, String> {
    LinearStaticPreviewRequest::pre_formation_checks_value(request)
}
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, solve_input_missing_codes, PreviewSolverMode};
use serde_json::{json, Value};

// The frozen 0.8.0 requests keep their bytes; each parsed copy is re-authored (common/current_model.rs).
#[path = "common/current_model.rs"]
mod current_model;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const BUNDLED_DEMO: &str = include_str!("../../../fixtures/product_preview/invented_demo_model.json");
const REFUSED_DEMO: &str = include_str!("../../../fixtures/product_preview/invented_preview_model.json");
/// Codex review r1 of #1665 (invented): a schema-valid model with a fresh
/// generated self-weight load. `intermediate_stop` makes it invalid in code.
const INTERMEDIATE_STOP: &str = include_str!("fixtures/model_checks/intermediate_stop_witness.json");

/// The committed fixture made invalid twice: an unknown support family, which
/// validation refuses, and a section density other than the one the generated
/// self-weight load recorded (7850 kg/m^3), so the load is stale and the next
/// stage would refuse it. The solve stops after validation. The fixture stays a
/// valid current model document (tests/test_model_document_schema.py).
fn intermediate_stop() -> Value {
    let mut wire: Value = serde_json::from_str(INTERMEDIATE_STOP).unwrap();
    wire["model"]["supports"][0]["family"] = json!("review_unknown");
    wire["model"]["pipe_segments"][0]["section"]["material_density"]["value"] = json!(15700.0);
    wire
}

fn base(nodes: Value, pipes: Value, supports: Value, components: Value) -> Value {
    json!({"model":{"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:checks","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":nodes,"pipe_segments":pipes,"supports":supports,"components":components,
        "materials":[{"id":"material:m","elastic_modulus":{"value":200e9,"unit":"Pa"},"shear_modulus":{"value":76.923076923e9,"unit":"Pa"},
            "thermal_expansion_coefficient":{"value":1.2e-5,"unit":"1/degC"},"provenance":"checks_invented_test_values"}],
        "load_cases":[{"id":"case:f","primitive_loads":[
            {"id":"load:f","category":"concentrated_force","target":{"type":"node","node":"node:b"},"direction":"global_y","magnitude":{"value":1000.0,"unit":"N"},"dimension":"force","provenance":"checks"}],
            "provenance":"checks"}],
        "combinations":[]},"materials":[]})
}
fn node(id: &str, x: f64, y: f64, z: f64) -> Value {
    json!({"id":id,"position":{"x":x,"y":y,"z":z},"provenance":"checks"})
}
fn pipe(id: &str, from: &str, to: &str) -> Value {
    json!({"id":id,"from":from,"to":to,"section":{"outside_diameter":{"value":0.12,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"material":"material:m","y_reference":{"x":0.0,"y":1.0,"z":0.0},"provenance":"checks"})
}
fn anchor(id: &str, node: &str) -> Value {
    json!({"id":id,"node":node,"family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"checks"})
}
fn restraint(id: &str, node: &str, restraints: &[&str]) -> Value {
    json!({"id":id,"node":node,"family":"guide","restraints":restraints,"provenance":"checks"})
}
fn tee(id: &str, node: &str, header: &str, branch: &str) -> Value {
    json!({"id":id,"kind":"tee","node":node,"geometry":{"branch_header_pipe_ref":header,"branch_branch_pipe_ref":branch},"provenance":"checks"})
}
fn cantilever() -> Value {
    base(json!([node("node:a", 0.0, 0.0, 0.0), node("node:b", 1.0, 0.0, 0.0)]), json!([pipe("pipe:a-b", "node:a", "node:b")]), json!([anchor("support:a", "node:a")]), json!([]))
}
fn two_parts(second_anchored: bool) -> Value {
    let mut wire = cantilever();
    let model = &mut wire["model"];
    model["nodes"].as_array_mut().unwrap().extend([node("node:c", 5.0, 0.0, 0.0), node("node:d", 5.0, 0.0, 1.0)]);
    model["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:c-d", "node:c", "node:d"));
    if second_anchored {
        model["supports"].as_array_mut().unwrap().push(anchor("support:c", "node:c"));
    }
    wire
}
fn document(text: &str) -> Value {
    json!({"model": serde_json::from_str::<Value>(text).unwrap(), "materials": []})
}

/// The models of the corpus, by name. Each is solved and checked.
fn corpus() -> Vec<(&'static str, Value)> {
    let mut cases = vec![
        ("connected cantilever", cantilever()),
        ("two anchored parts", two_parts(true)),
        ("an unsupported part", two_parts(false)),
        ("bundled demo", document(BUNDLED_DEMO)),
        ("refused demo with a tee on two pipes", document(REFUSED_DEMO)),
    ];
    let mut near = cantilever();
    near["model"]["nodes"].as_array_mut().unwrap().extend([node("node:n1", 3.0, 0.0, 0.0), node("node:n2", 3.004, 0.0, 0.0), node("node:n3", 3.004, 0.0, 0.0)]);
    cases.push(("coincident and near-duplicate nodes", near));
    let mut tee_two = base(
        json!([node("node:a", 0.0, 0.0, 0.0), node("node:b", 1.0, 0.0, 0.0), node("node:c", 2.0, 0.0, 0.0)]),
        json!([pipe("pipe:a-b", "node:a", "node:b"), pipe("pipe:b-c", "node:b", "node:c")]),
        json!([anchor("support:a", "node:a"), anchor("support:c", "node:c")]),
        json!([tee("component:tee", "node:b", "pipe:a-b", "pipe:b-c")]),
    );
    cases.push(("a tee on two pipes", tee_two.clone()));
    tee_two["model"]["components"] = json!([tee("component:tee", "node:b", "pipe:a-b", "pipe:a-b")]);
    cases.push(("a tee whose header is its branch", tee_two));
    let mut no_support = cantilever();
    no_support["model"]["supports"] = json!([]);
    cases.push(("no support", no_support));
    let mut under = cantilever();
    under["model"]["supports"] = json!([restraint("support:a", "node:a", &["UX", "UY"])]);
    cases.push(("fewer than six ground constraints", under));
    let mut family = cantilever();
    family["model"]["supports"][0]["family"] = json!("invented_family");
    cases.push(("an unknown support family", family));
    let mut unknown_node = cantilever();
    unknown_node["model"]["pipe_segments"][0]["to"] = json!("node:missing");
    cases.push(("a pipe end naming an unknown node", unknown_node));
    let mut tolerance = cantilever();
    tolerance["model"]["model_settings"] = json!({"node_proximity_tolerance":{"value":-1.0,"unit":"mm"}});
    cases.push(("a negative node-proximity tolerance", tolerance));
    let mut zero_length = cantilever();
    zero_length["model"]["nodes"][1]["position"] = json!({"x":0.0,"y":0.0,"z":0.0});
    cases.push(("a zero-length pipe", zero_length));
    cases.push(("an unknown support family with a stale generated self-weight load", intermediate_stop()));
    cases.extend(solve_input_cases());
    cases
}

fn solved(wire: &Value, mode: PreviewSolverMode) -> Vec<Value> {
    let envelope = run_linear_static_preview_value_with_mode(wire.clone(), mode).expect("the solve reads the request");
    serde_json::to_value(envelope).unwrap()["diagnostics"].as_array().cloned().unwrap_or_default()
}
fn checked(wire: &Value) -> Vec<Value> {
    let findings = preview_model_checks_value(wire.clone()).expect("the checks read the request");
    findings.into_iter().map(|d| serde_json::to_value(d).unwrap()).collect()
}

#[test]
fn the_checks_are_the_solves_leading_diagnostics() {
    let mut codes_seen = std::collections::BTreeSet::new();
    for (name, wire) in corpus() {
        let findings = checked(&wire);
        let blocks = findings.iter().any(|d| d["severity"] == "blocking");
        for mode in MODES {
            let published = solved(&wire, mode);
            assert!(published.len() >= findings.len(), "{name} ({mode:?}): the solve published fewer diagnostics\nchecks {findings:#?}\nsolve {published:#?}");
            for (finding, diagnostic) in findings.iter().zip(&published) {
                for key in ["id", "code", "severity", "affected_refs", "source"] {
                    assert_eq!(finding.get(key), diagnostic.get(key), "{name} ({mode:?}): {key} differs\ncheck {finding:#}\nsolve {diagnostic:#}");
                }
                let (check_text, solve_text) = (finding["message"].as_str().unwrap(), diagnostic["message"].as_str().unwrap());
                assert!(solve_text.starts_with(check_text), "{name} ({mode:?}): message differs\ncheck {check_text}\nsolve {solve_text}");
            }
            if blocks {
                assert_eq!(published.len(), findings.len(), "{name} ({mode:?}): a blocked solve publishes exactly the checks' findings\nchecks {findings:#?}\nsolve {published:#?}");
            }
        }
        codes_seen.extend(findings.iter().filter_map(|d| d["code"].as_str().map(str::to_string)));
    }
    // The corpus reaches each family the checks report.
    for code in [
        "MODEL_TOPOLOGY_DISCONNECTED", "NODE_COINCIDENT", "NODE_NEAR_DUPLICATE", "BRANCH_TOPOLOGY_INVALID",
        "SUPPORT_INPUT_MISSING", "SOLVER_SYSTEM_BLOCKED", "PIPE_ENDPOINT_UNKNOWN", "PIPE_ELEMENT_INPUT_INVALID",
        "SUPPORT_FAMILY_INVALID", "MODEL-SETTING-TOLERANCE-NOT-POSITIVE",
    ] {
        assert!(codes_seen.contains(code), "the corpus reaches {code}; seen {codes_seen:?}");
    }
}

#[test]
fn the_checks_write_nothing_and_repeat_exactly() {
    for (name, wire) in corpus() {
        let before = wire.to_string();
        let first = checked(&wire);
        assert_eq!(wire.to_string(), before, "{name}: the request is unchanged");
        assert_eq!(checked(&wire), first, "{name}: a second run gives the same findings");
    }
}

#[test]
fn a_request_the_solve_refuses_is_refused_with_the_same_text() {
    let mut undeclared = cantilever();
    undeclared["model"]["nodes"][0]["undeclared_member"] = json!(1);
    let solve = run_linear_static_preview_value_with_mode(undeclared.clone(), MODES[0]).err().expect("the solve refuses an undeclared key");
    assert_eq!(preview_model_checks_value(undeclared).err(), Some(solve));
    let unreadable = json!({"model": {"nodes": "not a list"}, "materials": []});
    let solve = run_linear_static_preview_value_with_mode(unreadable.clone(), MODES[0]).err().expect("the solve refuses an unreadable request");
    assert_eq!(preview_model_checks_value(unreadable).err(), Some(solve));
}

#[test]
fn the_checks_stop_where_the_solve_stops() {
    // Validation blocks on the family, so the stale self weight is never reached.
    let wire = intermediate_stop();
    let codes = |findings: &[Value]| findings.iter().map(|d| d["code"].as_str().unwrap().to_string()).collect::<Vec<_>>();
    let found = codes(&checked(&wire));
    assert!(found.contains(&"SUPPORT_FAMILY_INVALID".to_string()), "{found:?}");
    assert!(!found.contains(&"SELF_WEIGHT_INPUTS_STALE".to_string()), "{found:?}");
    for mode in MODES {
        assert_eq!(codes(&solved(&wire, mode)), found, "{mode:?}");
    }
    // With a valid family the same load is refused as stale: a check that ran past the first stop would list it.
    let mut valid = wire.clone();
    valid["model"]["supports"][0]["family"] = json!("anchor");
    let found = codes(&checked(&valid));
    assert!(found.contains(&"SELF_WEIGHT_INPUTS_STALE".to_string()), "{found:?}");
}

// ---------------------------------------------------------------------------
// Issue 530 and the missing-data use case (DEL-04-06 REQ-013, AC-008; DEL-07-06
// AC-007): the cases on which the checks once differed from the solve, and the
// load generation's findings before any solve. All models are invented.
// ---------------------------------------------------------------------------

const STRAIGHT_TIMOSHENKO: &str = include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json");
const SKEW_TIMOSHENKO: &str = include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SKEW-SPRING-TIMOSHENKO.request.json");
const SKEW_EULER_BERNOULLI: &str = include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SKEW-SPRING-EULER-BERNOULLI.request.json");
const SHARED_TERMINAL_JUNCTION: &str = include_str!("fixtures/shared_terminal_junction.request.json");
const MISSING_DATA: &str = include_str!("../../../fixtures/embedded_agent/use_cases/missing-data/start_model.json");
const PERSON_ENTERED: &str = include_str!("../../../fixtures/embedded_agent/use_cases/missing-data/person_entered.json");
const CODE_TABLE: &str = include_str!("../../../fixtures/diagnostics/exact_route_codes.json");

fn current(text: &str) -> Value {
    current_model::reauthor_request(serde_json::from_str(text).unwrap())
}

/// Issue 530's model: one member A-B along +X and a line stop at B along (3, 4, 0).
fn skew_model(supports: Value) -> Value {
    let q = |value: f64, unit: &str| json!({"value": value, "unit": unit});
    json!({"model": {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:skew", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [node("A", 0.0, 0.0, 0.0), node("B", 4.0, 0.0, 0.0)],
        "pipe_segments": [{"id": "pipe:AB", "from": "A", "to": "B", "section": {"outside_diameter": q(0.206, "m"), "wall_thickness": q(0.011, "m")},
            "material": "material:skew", "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": "checks"}],
        "supports": supports, "components": [],
        "materials": [{"id": "material:skew", "elastic_modulus": q(171_798_691_840.0, "Pa"), "shear_modulus": q(68_719_476_736.0, "Pa"), "provenance": "checks"}],
        "load_cases": [{"id": "case:skew", "primitive_loads": [{"id": "load:global_y", "category": "concentrated_force", "target": {"type": "node", "node": "B"},
            "direction": "global_y", "magnitude": q(1000.0, "N"), "dimension": "force", "provenance": "checks"}], "provenance": "checks"}],
        "combinations": []}, "materials": []})
}
fn skewed_stop() -> Value {
    json!({"id": "stop_B", "node": "B", "family": "line_stop", "restraints": ["UY"], "direction": {"x": 3.0, "y": 4.0, "z": 0.0}, "provenance": "checks"})
}

/// The straight Timoshenko request with a wind case whose exposed span is part
/// of its member (`partial`) or the whole member, and a seismic case carrying an
/// entered weight load. Its pipe states no density, so the seismic generation
/// refuses once nothing before formation does.
fn timoshenko_wind_and_seismic(partial: bool) -> Value {
    let mut request = current(STRAIGHT_TIMOSHENKO);
    let pipe = request["model"]["pipe_segments"][0]["id"].clone();
    let wind = if partial {
        json!({"pressure": {"value": 480.0, "unit": "Pa"}, "shape_factor": {"value": 0.7, "unit": "1"}, "direction": "global_y", "exposed_pipe_refs": [],
            "exposed_spans": [{"pipe_ref": pipe, "start_fraction": {"value": 0.25, "unit": "1"}, "end_fraction": {"value": 0.75, "unit": "1"}}]})
    } else {
        json!({"pressure": {"value": 480.0, "unit": "Pa"}, "shape_factor": {"value": 0.7, "unit": "1"}, "direction": "global_y", "exposed_pipe_refs": [pipe]})
    };
    let cases = request["model"]["load_cases"].as_array_mut().unwrap();
    cases[0]["equivalent_static"] = json!({"wind": wind, "provenance": "checks"});
    cases.push(json!({"id": "case:seismic", "kind": "primitive_user_load", "primitive_loads": [
            {"id": "load:weight", "category": "weight", "target": {"type": "element", "pipe": pipe}, "direction": "global_y",
                "magnitude": {"value": -100.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": "checks"}],
        "equivalent_static": {"seismic": {"gravity_acceleration": {"value": 9.81, "unit": "m/s^2"}, "g_factor_y": {"value": 0.4, "unit": "none"}},
            "provenance": "checks"},
        "provenance": "checks"}));
    request
}

/// An eccentric offset on a component that is not a reducer, which the solve
/// refuses on the normalized geometry.
fn eccentric_offset_on_a_valve() -> Value {
    let mut wire = cantilever();
    wire["model"]["components"] = json!([{"id": "component:v", "kind": "valve", "node": "node:a",
        "geometry": {"rigid_pipe_ref": "pipe:a-b", "eccentric_offset": {"x": 0.0, "y": 0.0, "z": 1.0, "unit": "mm"}},
        "modifiers": {"stiffness_scaling_user_value": {"value": 2.0, "unit": "1"}, "source_reference": "invented"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": "checks"}]);
    wire
}

/// The missing-data use case's start model with the person's first `entered` values.
fn missing_data(entered: usize) -> Value {
    let mut model: Value = serde_json::from_str(MISSING_DATA).unwrap();
    let entries: Value = serde_json::from_str(PERSON_ENTERED).unwrap();
    for entry in entries["entries"].as_array().unwrap().iter().take(entered) {
        assert_eq!(entry["field_path"], "section.material_density");
        let pipe = model["pipe_segments"].as_array_mut().unwrap().iter_mut().find(|pipe| pipe["id"] == entry["entity_ref"]).unwrap();
        pipe["section"]["material_density"] = json!({"value": entry["value"], "unit": entry["unit"]});
    }
    json!({"model": model, "materials": []})
}

fn solve_input_cases() -> Vec<(&'static str, Value)> {
    let anchor_five = json!({"id": "anchor_A", "node": "A", "family": "anchor", "restraints": ["UY", "UZ", "RX", "RY", "RZ"], "provenance": "checks"});
    let anchor_six = json!({"id": "anchor_A", "node": "A", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": "checks"});
    let junction = || {
        let mut request: Value = serde_json::from_str(SHARED_TERMINAL_JUNCTION).unwrap();
        if request.get("model").is_none() {
            request = json!({"model": request, "materials": []});
        }
        request
    };
    let mut differ = junction();
    differ["model"]["load_cases"][0]["pressure_regions"][2]["pressure"] = json!({"value": 5.0, "unit": "MPa"});
    let mut dry = junction();
    dry["model"]["load_cases"][0]["pressure_regions"].as_array_mut().unwrap().remove(2);
    vec![
        ("issue 530: five anchor restraints and one skewed stop", skew_model(json!([anchor_five, skewed_stop()]))),
        ("issue 530: a full anchor and one skewed stop", skew_model(json!([anchor_six, skewed_stop()]))),
        ("issue 530: only a skewed stop", skew_model(json!([skewed_stop()]))),
        ("issue 530: PFT-SKEW-SPRING-TIMOSHENKO", current(SKEW_TIMOSHENKO)),
        ("issue 530: PFT-SKEW-SPRING-EULER-BERNOULLI", current(SKEW_EULER_BERNOULLI)),
        ("a Timoshenko partial-span wind entry with a seismic weight case", timoshenko_wind_and_seismic(true)),
        ("a Timoshenko whole-span wind entry with a seismic weight case", timoshenko_wind_and_seismic(false)),
        ("an eccentric offset on a valve", eccentric_offset_on_a_valve()),
        ("a shared terminal junction", junction()),
        ("a shared terminal junction whose pressures differ", differ),
        ("a shared terminal junction with a pipe outside its regions", dry),
        ("missing data: the start model", missing_data(0)),
        ("missing data: the first density entered", missing_data(1)),
        ("missing data: both densities entered", missing_data(2)),
    ]
}

fn solve_inputs(wire: &Value) -> (Vec<Value>, Vec<Value>) {
    let checks = LinearStaticPreviewRequest::solve_input_checks_value(wire.clone()).expect("the checks read the request");
    let values = |list: Vec<open_pipe_stress_product_physics::Diagnostic>| list.into_iter().map(|d| serde_json::to_value(d).unwrap()).collect::<Vec<_>>();
    (values(checks.before_formation), values(checks.load_generation))
}
fn envelope(wire: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).expect("the solve reads the request")).unwrap()
}
fn is_blocking(diagnostic: &Value) -> bool {
    matches!(diagnostic["severity"].as_str(), Some("blocking" | "error"))
}

/// For every model of the corpus, in both modes: the findings before formation
/// are the pre-formation checks; a model the solve solves has no blocking
/// finding from the checks, either part; a model whose checks before formation
/// do not block is not refused by the solve before formation; and every load
/// generation finding is blocking (the generation's disclosures are the
/// solve's to publish).
#[test]
fn the_solve_input_checks_agree_with_the_solve() {
    for (name, wire) in corpus() {
        let (before, generation) = solve_inputs(&wire);
        assert_eq!(before, checked(&wire), "{name}: the findings before formation are the pre-formation checks");
        if before.iter().any(is_blocking) {
            assert!(generation.is_empty(), "{name}: no generation runs past a blocking finding before formation");
        }
        assert!(generation.iter().all(is_blocking), "{name}: {generation:#?}");
        for mode in MODES {
            let solve = envelope(&wire, mode);
            let published = solve["diagnostics"].as_array().cloned().unwrap_or_default();
            let solved = solve["status"]["mechanics"] == "MECHANICS_SOLVED";
            if solved {
                let blocking: Vec<_> = before.iter().chain(&generation).filter(|d| is_blocking(d)).collect();
                assert!(blocking.is_empty(), "{name} ({mode:?}): the solve solves, yet the checks block\n{blocking:#?}");
            }
            if !before.iter().any(is_blocking) {
                let first_blocking = published.iter().position(is_blocking);
                assert!(first_blocking.is_none_or(|at| at >= before.len()), "{name} ({mode:?}): the checks pass before formation, yet the solve refuses there\n{published:#?}");
            }
        }
    }
}

/// Issue 530: the solve solves the model whose skewed stop supplies the sixth
/// ground constraint, and the checks no longer block it; with only the skewed
/// stop both refuse with the same finding.
#[test]
fn a_skewed_restraint_counts_in_the_checks_as_in_the_solve() {
    let anchor_five = json!({"id": "anchor_A", "node": "A", "family": "anchor", "restraints": ["UY", "UZ", "RX", "RY", "RZ"], "provenance": "checks"});
    let wire = skew_model(json!([anchor_five, skewed_stop()]));
    assert!(!checked(&wire).iter().any(is_blocking), "{:#?}", checked(&wire));
    for mode in MODES {
        assert_eq!(envelope(&wire, mode)["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
    }
    let only = skew_model(json!([skewed_stop()]));
    let found = checked(&only);
    assert_eq!(found.iter().filter(|d| is_blocking(d)).map(|d| d["code"].as_str().unwrap()).collect::<Vec<_>>(), ["SOLVER_SYSTEM_BLOCKED"]);
    assert!(found.iter().any(|d| d["message"].as_str().unwrap().ends_with("; skewed constraints: 1")), "{found:#?}");
}

/// The three steps the checks once restated or omitted: each refusal or
/// warning the solve raises before formation is a finding of the checks.
#[test]
fn partial_spans_eccentric_offsets_and_junction_closures_are_found_before_formation() {
    let codes = |wire: &Value| checked(wire).iter().map(|d| d["code"].as_str().unwrap().to_string()).collect::<Vec<_>>();
    assert!(codes(&timoshenko_wind_and_seismic(true)).contains(&"PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED".to_string()));
    assert!(codes(&eccentric_offset_on_a_valve()).contains(&"RIGID_COMPONENT_ECCENTRIC_OFFSET_INVALID".to_string()));
    let cases = solve_input_cases();
    let case = |name: &str| &cases.iter().find(|(n, _)| *n == name).unwrap().1;
    assert!(codes(case("a shared terminal junction whose pressures differ")).contains(&"EXACT_PRESSURE_JUNCTION_PRESSURES_DIFFER".to_string()));
    assert!(codes(case("a shared terminal junction with a pipe outside its regions")).contains(&"EXACT_PRESSURE_JUNCTION_PIPE_OUTSIDE_REGIONS".to_string()));
    // With the whole span the seismic case's missing density is the generation's finding.
    let (before, generation) = solve_inputs(&timoshenko_wind_and_seismic(false));
    assert!(!before.iter().any(is_blocking), "{before:#?}");
    assert!(generation.iter().any(|d| d["code"] == "EQUIVALENT_STATIC_INPUT_MISSING"), "{generation:#?}");
}

/// DEL-07-06 AC-007, Missing data: each missing density is reported on its pipe
/// before any solve, as the solve states it; entering one leaves exactly the
/// other; entering both leaves none, and the solve then runs.
#[test]
fn every_missing_density_is_reported_before_any_solve() {
    let missing = solve_input_missing_codes();
    let blocking = |wire: &Value| {
        let (before, generation) = solve_inputs(wire);
        assert!(!before.iter().any(is_blocking), "{before:#?}");
        generation.into_iter().filter(is_blocking).collect::<Vec<_>>()
    };
    let refs = |found: &[Value]| found.iter().map(|d| d["affected_refs"].clone()).collect::<Vec<_>>();
    let start = blocking(&missing_data(0));
    assert_eq!(refs(&start), [json!(["pipe:md-inboard", "material_density", "case:md-seismic"]), json!(["pipe:md-outboard", "material_density", "case:md-seismic"])]);
    let one = blocking(&missing_data(1));
    assert_eq!(refs(&one), [json!(["pipe:md-outboard", "material_density", "case:md-seismic"])]);
    for (wire, found) in [(missing_data(0), &start), (missing_data(1), &one)] {
        for finding in found {
            assert_eq!(finding["code"], "EQUIVALENT_STATIC_INPUT_MISSING");
            assert!(missing.iter().any(|code| finding["code"] == code.as_str()), "{finding:#}");
        }
        for mode in MODES {
            let solve = envelope(&wire, mode);
            let refused: Vec<Value> = solve["diagnostics"].as_array().unwrap().iter().filter(|d| is_blocking(d)).cloned().collect();
            assert_eq!(&refused, found, "{mode:?}: the solve refuses with exactly the checks' findings");
        }
    }
    let both = missing_data(2);
    assert!(blocking(&both).is_empty());
    for mode in MODES {
        assert_eq!(envelope(&both, mode)["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
    }
}

/// The exported list of the codes that mean a solve input is missing: every
/// member is a code the solve crates construct, classified `input_error`, and
/// every `input_error` code named `…_MISSING` is a member.
#[test]
fn the_missing_input_codes_are_the_tables_missing_input_errors() {
    let table: Value = serde_json::from_str(CODE_TABLE).unwrap();
    let classified = |code: &str| table["codes"].as_array().unwrap().iter().find(|entry| entry["code"] == code).map(|entry| entry["classification"].clone());
    let listed = solve_input_missing_codes();
    assert!(!listed.is_empty());
    let mut sorted = listed.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted, listed, "the list is sorted and has no duplicate");
    for code in &listed {
        assert_eq!(classified(code), Some(json!("input_error")), "{code}");
    }
    for entry in table["codes"].as_array().unwrap() {
        let code = entry["code"].as_str().unwrap();
        if code.ends_with("_MISSING") && entry["classification"] == "input_error" {
            assert!(listed.iter().any(|listed| listed == code), "{code} is a missing input and is not listed");
        }
    }
}

// ---------------------------------------------------------------------------
// The blocked envelope's findings (PR 579 review): where a finding blocks, the
// checks carry exactly what the blocked solve publishes, field by field, and
// the load generation's refusals are listed for every case in case order.
// ---------------------------------------------------------------------------

const LOAD_STATE_BASE: &str = include_str!("fixtures/model_checks/load_state_fan_out_base.json");
const FIELDS: [&str; 6] = ["id", "code", "severity", "message", "affected_refs", "source"];

fn same_fields(context: &str, checks: &[Value], solve: &[Value]) {
    assert_eq!(checks.len(), solve.len(), "{context}: counts differ\nchecks {checks:#?}\nsolve {solve:#?}");
    for (finding, diagnostic) in checks.iter().zip(solve) {
        for key in FIELDS {
            assert_eq!(finding.get(key), diagnostic.get(key), "{context}: {key} differs\ncheck {finding:#}\nsolve {diagnostic:#}");
        }
    }
}

/// On the preview route the blocked solve strips a `result:` ref it can no
/// longer resolve; so do both checks, and the finding is otherwise the solve's.
#[test]
fn a_blocking_finding_naming_a_result_ref_is_published_as_the_blocked_solve_publishes_it() {
    let mut wire = missing_data(0);
    assert!(wire["model"].get("pressure_contract").is_none(), "the start model is pressure-free");
    wire["model"]["pipe_segments"][0]["from"] = json!("result:missing-node");
    let pre = checked(&wire);
    let (before, generation) = solve_inputs(&wire);
    assert!(generation.is_empty(), "{generation:#?}");
    let endpoint: Vec<&Value> = pre.iter().filter(|d| d["code"] == "PIPE_ENDPOINT_UNKNOWN").collect();
    assert_eq!(endpoint.len(), 1, "{pre:#?}");
    assert!(is_blocking(endpoint[0]), "{:#}", endpoint[0]);
    for finding in pre.iter().chain(&before) {
        assert!(!finding["affected_refs"].as_array().unwrap().iter().any(|r| r.as_str().unwrap().starts_with("result:")), "{finding:#}");
    }
    same_fields("pre-formation and solve-input checks", &pre, &before);
    for mode in MODES {
        same_fields(&format!("checks and the blocked solve ({mode:?})"), &pre, &solved(&wire, mode));
    }
}

/// Two cases refuse in the load generation, each on its own; the checks list
/// every refusal, in case order, and the solve, stopping at the first refusing
/// case after solving the case before it, publishes that case's refusals.
#[test]
fn every_refusing_case_is_reported_in_case_order() {
    let mut wire = missing_data(0);
    let seismic = wire["model"]["load_cases"][0].clone();
    let mut second = seismic.clone();
    second["id"] = json!("case:md-seismic-x");
    second["equivalent_static"]["seismic"]["g_factor_x"]["value"] = json!(0.3);
    second["equivalent_static"]["seismic"]["g_factor_y"]["value"] = json!(0);
    let mut force = seismic.clone();
    force["id"] = json!("case:md-force");
    force.as_object_mut().unwrap().remove("equivalent_static");
    force["primitive_loads"] = json!([{"id": "load:md-tip", "category": "concentrated_force", "target": {"type": "node", "node": "node:md-3"},
        "direction": "global_y", "magnitude": {"value": -500.0, "unit": "N"}, "dimension": "force", "provenance": "checks"}]);
    wire["model"]["load_cases"] = json!([force, seismic, second]);
    let (before, generation) = solve_inputs(&wire);
    assert!(!before.iter().any(is_blocking), "{before:#?}");
    let refs = generation.iter().map(|d| d["affected_refs"].clone()).collect::<Vec<_>>();
    assert_eq!(refs, [
        json!(["pipe:md-inboard", "material_density", "case:md-seismic"]),
        json!(["pipe:md-outboard", "material_density", "case:md-seismic"]),
        json!(["pipe:md-inboard", "material_density", "case:md-seismic-x"]),
        json!(["pipe:md-outboard", "material_density", "case:md-seismic-x"]),
    ]);
    assert!(generation.iter().all(|d| d["code"] == "EQUIVALENT_STATIC_INPUT_MISSING"), "{generation:#?}");
    let first_case: Vec<Value> = generation.iter().filter(|d| d["affected_refs"][2] == "case:md-seismic").cloned().collect();
    for mode in MODES {
        let solve = envelope(&wire, mode);
        assert_eq!(solve["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        let refused: Vec<Value> = solve["diagnostics"].as_array().unwrap().iter().filter(|d| is_blocking(d)).cloned().collect();
        same_fields(&format!("the first refusing case ({mode:?})"), &first_case, &refused);
    }
}

/// A load-state case's loads are its effective case's: a stored load listed in
/// the case's sources, aimed at a node the model does not have, is refused by
/// the generation on the resolved case, as the solve refuses it; unlisted, it is
/// not in the effective case, nothing is generated for it and the case solves.
#[test]
fn a_load_state_case_is_generated_on_its_effective_case() {
    let mut model: Value = serde_json::from_str(LOAD_STATE_BASE).unwrap();
    let case = &mut model["load_cases"][0];
    case["primitive_loads"] = json!([{"id": "load:stray", "category": "concentrated_force", "target": {"type": "node", "node": "node:missing"},
        "direction": "global_x", "magnitude": {"value": 10.0, "unit": "N"}, "dimension": "force", "provenance": "checks"}]);
    case["analysis_state"]["load_sources"] = json!([{"source_ref": "load:stray", "factor": 2.0}]);
    let wire = json!({"model": model, "materials": []});
    let (before, generation) = solve_inputs(&wire);
    assert!(!before.iter().any(is_blocking), "{before:#?}");
    assert_eq!(generation.iter().map(|d| d["code"].as_str().unwrap()).collect::<Vec<_>>(), ["LOAD_NODE_UNKNOWN"], "{generation:#?}");
    assert_eq!(generation[0]["affected_refs"], json!(["load:stray", "node:missing", "case:kelvin-interval"]));
    for mode in MODES {
        let solve = envelope(&wire, mode);
        assert_eq!(solve["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        let refused: Vec<Value> = solve["diagnostics"].as_array().unwrap().iter().filter(|d| is_blocking(d)).cloned().collect();
        same_fields(&format!("the load-state solve ({mode:?})"), &generation, &refused);
    }
    let mut unlisted = wire.clone();
    unlisted["model"]["load_cases"][0]["analysis_state"]["load_sources"] = json!([]);
    let (before, generation) = solve_inputs(&unlisted);
    assert!(!before.iter().any(is_blocking), "{before:#?}");
    assert!(generation.is_empty(), "{generation:#?}");
    for mode in MODES {
        assert_eq!(envelope(&unlisted, mode)["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
    }
}


/// A preview-route cantilever whose concentrated force targets its pipe passes
/// the checks before formation and the load generation, and its solve blocks
/// after formation, where the load application refuses the force. Beside it,
/// an empty case named `case_id` draws the non-blocking warning that it
/// applies no action.
fn force_on_the_pipe_with_an_empty_case(case_id: &str) -> Value {
    let mut wire = cantilever();
    wire["model"]["load_cases"][0]["primitive_loads"][0]["target"] = json!({"type": "element", "pipe": "pipe:a-b"});
    wire["model"]["load_cases"].as_array_mut().unwrap().push(json!({"id": case_id, "primitive_loads": [], "provenance": "checks"}));
    wire
}

/// REQ-013's exception, in both modes: blocked after its checks and load
/// generation, the preview-route solve keeps a non-blocking finding that names
/// no result field for field, and omits one whose affected references name a
/// result, since it publishes no result rows.
#[test]
fn a_solve_blocked_after_formation_omits_only_a_warning_that_names_a_result() {
    for (case_id, kept) in [("case:empty", true), ("result:empty", false)] {
        let wire = force_on_the_pipe_with_an_empty_case(case_id);
        let (before, generation) = solve_inputs(&wire);
        assert!(generation.is_empty(), "{case_id}: {generation:#?}");
        assert!(!before.iter().any(is_blocking), "{case_id}: {before:#?}");
        let empty: Vec<Value> = before.iter().filter(|d| d["code"] == "LOAD_CASE_APPLIES_NO_ACTION").cloned().collect();
        assert_eq!(empty.len(), 1, "{case_id}: {before:#?}");
        assert_eq!(empty[0]["severity"], "warning");
        assert_eq!(empty[0]["affected_refs"], json!([case_id]));
        for mode in MODES {
            let solve = envelope(&wire, mode);
            assert_eq!(solve["status"]["mechanics"], "MODEL_INCOMPLETE", "{case_id} ({mode:?})");
            let published = solve["diagnostics"].as_array().cloned().unwrap_or_default();
            let codes: Vec<&str> = published.iter().map(|d| d["code"].as_str().unwrap()).collect();
            if kept {
                let found: Vec<Value> = published.iter().filter(|d| d["code"] == "LOAD_CASE_APPLIES_NO_ACTION").cloned().collect();
                same_fields(&format!("{case_id} ({mode:?})"), &empty, &found);
                assert_eq!(codes, ["LOAD_CASE_APPLIES_NO_ACTION", "LOAD_CATEGORY_PREVIEW_MAPPED", "LOAD_INPUT_INVALID"], "{case_id} ({mode:?})\n{published:#?}");
            } else {
                assert_eq!(codes, ["LOAD_CATEGORY_PREVIEW_MAPPED", "LOAD_INPUT_INVALID"], "{case_id} ({mode:?})\n{published:#?}");
            }
            let invalid = published.iter().find(|d| d["code"] == "LOAD_INPUT_INVALID").unwrap();
            assert!(is_blocking(invalid), "{invalid:#}");
            assert_eq!(invalid["affected_refs"], json!(["load:f", "case:f"]), "{case_id} ({mode:?})");
        }
    }
}

/// REQ-013's exception where the solve blocks before its load generation, in
/// both modes: the cantilever's force case names a modulus basis the model does
/// not define, which the checks before formation do not refuse and the
/// preview-route solve does, at basis resolution. Beside it, an empty case named
/// `case_id` draws the non-blocking warning that it applies no action; the
/// blocked solve keeps that warning when it names no result and omits it when it
/// names one.
#[test]
fn a_solve_blocked_at_basis_resolution_omits_only_a_warning_that_names_a_result() {
    for (case_id, kept) in [("case:empty", true), ("result:empty", false)] {
        let mut wire = cantilever();
        wire["model"]["load_cases"][0]["modulus_basis_ref"] = json!("basis:undefined");
        wire["model"]["load_cases"].as_array_mut().unwrap().push(json!({"id": case_id, "primitive_loads": [], "provenance": "checks"}));
        let (before, generation) = solve_inputs(&wire);
        assert!(generation.is_empty(), "{case_id}: {generation:#?}");
        assert!(!before.iter().any(is_blocking), "{case_id}: {before:#?}");
        let empty: Vec<Value> = before.iter().filter(|d| d["code"] == "LOAD_CASE_APPLIES_NO_ACTION").cloned().collect();
        assert_eq!(empty.len(), 1, "{case_id}: {before:#?}");
        assert_eq!(empty[0]["severity"], "warning");
        assert_eq!(empty[0]["affected_refs"], json!([case_id]));
        for mode in MODES {
            let solve = envelope(&wire, mode);
            assert_eq!(solve["status"]["mechanics"], "MODEL_INCOMPLETE", "{case_id} ({mode:?})");
            let published = solve["diagnostics"].as_array().cloned().unwrap_or_default();
            let codes: Vec<&str> = published.iter().map(|d| d["code"].as_str().unwrap()).collect();
            if kept {
                let found: Vec<Value> = published.iter().filter(|d| d["code"] == "LOAD_CASE_APPLIES_NO_ACTION").cloned().collect();
                same_fields(&format!("{case_id} ({mode:?})"), &empty, &found);
                assert_eq!(codes, ["LOAD_CASE_APPLIES_NO_ACTION", "MODULUS_BASIS_UNRESOLVED"], "{case_id} ({mode:?})\n{published:#?}");
            } else {
                assert_eq!(codes, ["MODULUS_BASIS_UNRESOLVED"], "{case_id} ({mode:?})\n{published:#?}");
            }
            let unresolved = published.iter().find(|d| d["code"] == "MODULUS_BASIS_UNRESOLVED").unwrap();
            assert!(is_blocking(unresolved), "{unresolved:#}");
        }
    }
}
