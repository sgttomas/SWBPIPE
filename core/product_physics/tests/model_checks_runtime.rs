//! On-demand model checks (DEL-04-06 Design `on-demand-model-checks.md`): the
//! checks are the solve's own checks before stiffness formation. For every
//! model below, in both solver modes, the findings equal the leading
//! diagnostics of the solve's envelope (id, code, severity and affected refs;
//! the solve may append to a branch warning's message once it has published
//! rows). Where a finding blocks, the solve publishes exactly those findings.
//! A request the solve refuses outright is refused with the same text. All
//! models are invented.
use open_pipe_stress_product_physics::model_settings::model_checks::preview_model_checks_value;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const BUNDLED_DEMO: &str = include_str!("../../../fixtures/product_preview/invented_demo_model.json");
const REFUSED_DEMO: &str = include_str!("../../../fixtures/product_preview/invented_preview_model.json");
/// Codex review r1 of #1665 (invented): an unknown support family, which
/// validation refuses, and a stale generated self-weight load, which the next
/// stage would refuse. The solve stops after validation.
const INTERMEDIATE_STOP: &str = include_str!("fixtures/model_checks/intermediate_stop_witness.json");

fn base(nodes: Value, pipes: Value, supports: Value, components: Value) -> Value {
    json!({"model":{"schema_version":"0.7.0","document_kind":"openpipestress.product_preview.model",
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
    cases.push(("an unknown support family with a stale generated self-weight load", serde_json::from_str(INTERMEDIATE_STOP).unwrap()));
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
    let wire: Value = serde_json::from_str(INTERMEDIATE_STOP).unwrap();
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
