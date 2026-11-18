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
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const BUNDLED_DEMO: &str = include_str!("../../../fixtures/product_preview/invented_demo_model.json");
const REFUSED_DEMO: &str = include_str!("../../../fixtures/product_preview/invented_preview_model.json");
/// Codex review r1 of #1665 (invented): a schema-valid model with a fresh
/// generated self-weight load. `intermediate_stop` makes it invalid in code.
const INTERMEDIATE_STOP: &str = include_str!("fixtures/model_checks/intermediate_stop_witness.json");
/// Issue 380: the Missing data use case's authored start model (neither pipe
/// states its density; one seismic static-equivalent case), its expectations,
/// and the values the person enters. The expected findings are read from the
/// scenario, never from product output.
const MISSING_DATA_START: &str = include_str!("../../../fixtures/embedded_agent/use_cases/missing-data/start_model.json");
const MISSING_DATA_SCENARIO: &str = include_str!("../../../fixtures/embedded_agent/use_cases/missing-data/scenario.json");
const MISSING_DATA_ENTRIES: &str = include_str!("../../../fixtures/embedded_agent/use_cases/missing-data/person_entered.json");

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
    cases.extend(generation_corpus());
    cases
}

// ---- Issue 380: equivalent-static generation's input checks -------------------

fn missing_data_start() -> Value {
    document(MISSING_DATA_START)
}
/// The start with the person's entries written at their fields.
fn missing_data_entered() -> Value {
    let mut wire = missing_data_start();
    let entries: Value = serde_json::from_str(MISSING_DATA_ENTRIES).unwrap();
    for entry in entries["entries"].as_array().unwrap() {
        let pipe = wire["model"]["pipe_segments"].as_array_mut().unwrap().iter_mut()
            .find(|pipe| pipe["id"] == entry["entity_ref"]).unwrap();
        let leaf = entry["field_path"].as_str().unwrap().strip_prefix("section.").unwrap().to_string();
        pipe["section"][leaf] = json!({"value": entry["value"], "unit": entry["unit"]});
    }
    wire
}
/// The scenario's missing values as (entity, field leaf).
fn scenario_missing() -> Vec<(String, String)> {
    let scenario: Value = serde_json::from_str(MISSING_DATA_SCENARIO).unwrap();
    scenario["expectations"]["missing_values"].as_array().unwrap().iter()
        .map(|item| (item["entity_ref"].as_str().unwrap().to_string(),
            item["field_path"].as_str().unwrap().rsplit('.').next().unwrap().to_string()))
        .collect()
}
fn seismic_case(wire: &mut Value) -> &mut Value {
    &mut wire["model"]["load_cases"][0]["equivalent_static"]["seismic"]
}
/// The entered model on the exact route: E and nu for the material, an explicit
/// corrosion allowance on each pipe, and no pressure region. Invented inputs.
fn on_exact_route(mut wire: Value) -> Value {
    let model = &mut wire["model"];
    model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
    let material = model["materials"][0].as_object_mut().unwrap();
    material.remove("shear_modulus");
    material.insert("constitutive_basis".into(), json!("homogeneous_isotropic_E_nu_v1"));
    material.insert("poisson_ratio".into(), json!({"value": 0.3, "unit": "1"}));
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
    }
    for case in model["load_cases"].as_array_mut().unwrap() {
        case["pressure_regions"] = json!([]);
    }
    wire
}
fn without_densities(mut wire: Value) -> Value {
    for pipe in wire["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"].as_object_mut().unwrap().remove("material_density");
    }
    wire
}
/// The entered model as a load/reference-state document on the exact route
/// (the only route that admits one), its generating case given a state.
fn as_load_state(wire: Value) -> Value {
    let mut wire = on_exact_route(wire);
    let model = &mut wire["model"];
    let pipes = model["pipe_segments"].as_array().unwrap().iter().map(|p| p["id"].clone()).collect::<Vec<_>>();
    let material = model["materials"][0]["id"].clone();
    model["reference_configurations"] = json!([{"id": "reference:installed", "label": "Installed reference",
        "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": pipes.iter().map(|p| json!({"pipe_ref": p, "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": "checks"})).collect::<Vec<_>>(),
        "provenance": "checks"}]);
    model["load_cases"][0]["analysis_state"] = json!({"contract": "openpipestress.load_reference_state/1.0.0", "reference_configuration_ref": "reference:installed",
        "element_states": pipes.iter().map(|p| json!({"pipe_ref": p, "material_selection": {"kind": "explicit_base_properties", "material_ref": material,
            "applicability_reference": "invented analytical basis declared applicable to this member"},
            "thermal_state": {"kind": "unchanged_reference", "provenance": "checks"}})).collect::<Vec<_>>(),
        "support_states": [{"support_ref": "support:md-anchor", "participation": {"kind": "active_model_device"}}],
        "load_sources": [], "history": {"kind": "independent_equilibrium"}, "provenance": "checks"});
    wire
}

fn generation_corpus() -> Vec<(&'static str, Value)> {
    let mut cases = vec![
        ("missing data: densities absent", missing_data_start()),
        ("missing data: densities entered", missing_data_entered()),
    ];
    let mut gravity = missing_data_entered();
    seismic_case(&mut gravity).as_object_mut().unwrap().remove("gravity_acceleration");
    cases.push(("seismic generation without gravity", gravity));
    let mut no_axis = missing_data_entered();
    for axis in ["g_factor_x", "g_factor_y", "g_factor_z"] {
        seismic_case(&mut no_axis).as_object_mut().unwrap().remove(axis);
    }
    cases.push(("seismic generation with no axis factor entered", no_axis));
    let mut one_axis = missing_data_entered();
    for axis in ["g_factor_x", "g_factor_z"] {
        seismic_case(&mut one_axis).as_object_mut().unwrap().remove(axis);
    }
    cases.push(("seismic generation with one axis factor entered", one_axis));
    let mut wind = missing_data_entered();
    wind["model"]["load_cases"][0]["equivalent_static"] = json!({"wind": {"shape_factor": {"value": 0.7, "unit": "1"},
        "direction": "global_z", "exposed_pipe_refs": ["pipe:md-inboard"]}, "provenance": "checks"});
    cases.push(("wind generation without pressure", wind.clone()));
    wind["model"]["load_cases"][0]["equivalent_static"]["wind"]["pressure"] = json!({"value": 500.0, "unit": "Pa"});
    cases.push(("wind generation with its inputs entered", wind.clone()));
    let mut wind_absent_pipe = wind;
    wind_absent_pipe["model"]["load_cases"][0]["equivalent_static"]["wind"]["exposed_pipe_refs"] = json!(["pipe:md-absent"]);
    cases.push(("wind generation marking an absent pipe", wind_absent_pipe));
    let mut two = missing_data_start();
    let mut second = two["model"]["load_cases"][0].clone();
    second["id"] = json!("case:md-seismic-2");
    two["model"]["load_cases"].as_array_mut().unwrap().push(second);
    cases.push(("two generating cases, densities absent", two));
    let mut authored_beside = missing_data_start();
    authored_beside["model"]["load_cases"].as_array_mut().unwrap().insert(0, json!({"id": "case:authored", "primitive_loads": [
        {"id": "load:authored", "category": "concentrated_force", "target": {"type": "node", "node": "node:md-3"}, "direction": "global_y",
         "magnitude": {"value": 1000.0, "unit": "N"}, "dimension": "force", "provenance": "checks"}], "provenance": "checks"}));
    cases.push(("an authored case before a generating case, densities absent", authored_beside));
    let mut collision = missing_data_entered();
    collision["model"]["load_cases"][0]["primitive_loads"] = json!([{"id": "case:md-seismic:generated:seismic:global-y:element-0",
        "category": "concentrated_force", "target": {"type": "node", "node": "node:md-3"}, "direction": "global_y",
        "magnitude": {"value": 1.0, "unit": "N"}, "dimension": "force", "provenance": "checks"}]);
    cases.push(("an authored load named like a generated one", collision));
    let mut earlier_stop = missing_data_start();
    earlier_stop["model"]["supports"][0]["family"] = json!("invented_family");
    cases.push(("an unknown support family before absent densities", earlier_stop));
    cases.push(("exact route: densities absent", without_densities(on_exact_route(missing_data_entered()))));
    cases.push(("exact route: densities entered", on_exact_route(missing_data_entered())));
    cases.push(("load-state document with generation, densities absent", without_densities(as_load_state(missing_data_entered()))));
    cases.push(("load-state document with generation, densities entered", as_load_state(missing_data_entered())));
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
        "SUPPORT_FAMILY_INVALID", "MODEL-SETTING-TOLERANCE-NOT-POSITIVE", "EQUIVALENT_STATIC_INPUT_MISSING",
        "EQUIVALENT_STATIC_INPUT_INVALID",
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

fn blocking(findings: &[Value]) -> Vec<&Value> {
    findings.iter().filter(|d| d["severity"] == "blocking").collect()
}
fn mechanics(wire: &Value, mode: PreviewSolverMode) -> String {
    let envelope = serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap()).unwrap();
    envelope["status"]["mechanics"].as_str().unwrap().to_string()
}

/// Issue 380 (DEL-04-06 REQ-013, DEL-07-07 REQ-008): before any solve, the checks
/// name each value the scenario says is missing, on its entity and field and
/// for its case, and nothing else blocks; with the person's values entered,
/// nothing blocks and the model solves in both modes, on both routes.
#[test]
fn generation_inputs_missing_from_the_scenario_are_found_before_any_solve() {
    let expected = scenario_missing();
    let case = "case:md-seismic";
    for (what, wire) in [("PF", missing_data_start()), ("EX", without_densities(on_exact_route(missing_data_entered())))] {
        let findings = checked(&wire);
        let named = blocking(&findings).iter().map(|d| {
            assert_eq!(d["code"], "EQUIVALENT_STATIC_INPUT_MISSING", "{what}: {d:#}");
            let refs = d["affected_refs"].as_array().unwrap();
            assert!(refs.iter().any(|r| r == case), "{what}: the finding names its case: {d:#}");
            (refs[0].as_str().unwrap().to_string(), refs[1].as_str().unwrap().to_string())
        }).collect::<Vec<_>>();
        assert_eq!(named, expected, "{what}: the scenario's missing values, each once");
        for mode in MODES {
            assert_eq!(mechanics(&wire, mode), "MODEL_INCOMPLETE", "{what} ({mode:?})");
        }
    }
    for (what, wire) in [("PF", missing_data_entered()), ("EX", on_exact_route(missing_data_entered()))] {
        assert!(blocking(&checked(&wire)).is_empty(), "{what}: nothing blocks once the person's values are entered");
        for mode in MODES {
            assert_eq!(mechanics(&wire, mode), "MECHANICS_SOLVED", "{what} ({mode:?})");
        }
    }
}

/// Each generation input the user owns is found by name before any solve:
/// gravity, the axis factors (an unentered axis is unselected; none entered is
/// missing), the wind inputs and its marked pipes, and every generating case.
/// A model the checks pass solves.
#[test]
fn every_generation_input_is_found_by_name_and_a_complete_model_solves() {
    let corpus = generation_corpus();
    let model = |name: &str| corpus.iter().find(|(n, _)| *n == name).unwrap().1.clone();
    let ids = |name: &str| blocking(&checked(&model(name))).iter().map(|d| d["id"].as_str().unwrap().to_string()).collect::<Vec<_>>();
    assert_eq!(ids("seismic generation without gravity"), ["diagnostic:equivalent-static:case-md-seismic:seismic:gravity"]);
    assert_eq!(ids("seismic generation with no axis factor entered"), ["diagnostic:equivalent-static:case-md-seismic:seismic:g-factors"]);
    assert_eq!(ids("wind generation without pressure"), ["diagnostic:equivalent-static:case-md-seismic:wind:inputs"]);
    assert!(checked(&model("wind generation without pressure"))[0]["message"].as_str().unwrap().contains("pressure"));
    assert_eq!(ids("wind generation marking an absent pipe").len(), 1);
    let two = blocking(&checked(&model("two generating cases, densities absent"))).iter()
        .map(|d| d["affected_refs"].as_array().unwrap().last().unwrap().as_str().unwrap().to_string()).collect::<Vec<_>>();
    assert_eq!(two, ["case:md-seismic", "case:md-seismic", "case:md-seismic-2", "case:md-seismic-2"]);
    assert_eq!(blocking(&checked(&model("an authored case before a generating case, densities absent"))).len(), 2);
    // The generated id form `<case>:generated:seismic:<axis>:element-<index>` is DEL-05-01's,
    // pinned independently by `w16_exact_equivalent_static.rs`'s frozen references.
    let collision = checked(&model("an authored load named like a generated one"));
    assert_eq!(blocking(&collision).iter().map(|d| d["code"].as_str().unwrap()).collect::<Vec<_>>(),
        ["EQUIVALENT_STATIC_GENERATED_ID_COLLISION"],
        "the authored load's collision is found before any solve, by the generator that keeps the authored loads: {collision:#?}");
    // The checks stop where the solve stops: an earlier stop hides the densities.
    let earlier = checked(&model("an unknown support family before absent densities"));
    assert!(earlier.iter().any(|d| d["code"] == "SUPPORT_FAMILY_INVALID"));
    assert!(!earlier.iter().any(|d| d["code"] == "EQUIVALENT_STATIC_INPUT_MISSING"), "{earlier:#?}");
    for name in ["seismic generation with one axis factor entered", "wind generation with its inputs entered"] {
        assert!(blocking(&checked(&model(name))).is_empty(), "{name}");
        for mode in MODES {
            assert_eq!(mechanics(&model(name), mode), "MECHANICS_SOLVED", "{name} ({mode:?})");
        }
    }
}

/// A load/reference-state document exists only on the exact route, which
/// refuses generation on it by name before any generation runs (DEL-05-01,
/// `w16_exact_equivalent_static.rs`). The checks give that refusal, as the solve
/// does, whether or not the densities are entered; no generation finding is
/// reached, so no effective case is generated for today.
#[test]
fn a_load_state_document_with_generation_is_refused_by_name_before_generation() {
    for wire in [without_densities(as_load_state(missing_data_entered())), as_load_state(missing_data_entered())] {
        let findings = checked(&wire);
        let codes = blocking(&findings).iter().map(|d| d["code"].as_str().unwrap()).collect::<Vec<_>>();
        assert_eq!(codes, ["EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED"], "{findings:#?}");
        for mode in MODES {
            let ids = |found: &[Value]| found.iter().map(|d| d["id"].clone()).collect::<Vec<_>>();
            assert_eq!(ids(&solved(&wire, mode)), ids(&findings), "{mode:?}");
        }
    }
}
