//! Derived solves through the product's public JSON entry (DEL-04-11 Design §4,
//! §5, §7): candidate and case checks, derived documents, stored-model
//! invariance, and the closed-form references R-1, R-2 and R-5 frozen under
//! `validation/references/hanger_sizing/` by a generator that reads no product code.

use open_pipe_stress_hanger_sizing::derived::*;
use open_pipe_stress_hanger_sizing::*;
use open_pipe_stress_product_physics::PreviewSolverMode;
use serde_json::{json, Value};

#[path = "common/model_builder.rs"]
pub mod model_builder;

use model_builder::*;

const STEEL_E: f64 = 2.0e11;
const ALPHA: f64 = 1.2e-5;

fn method() -> Method {
    Method {
        load_design: LoadDesign::Hot,
        variation_reference: VariationReference::OperatingLoad,
        variation_limit: 0.25,
        travel_allowance_m: 0.0,
        selection_rule: SelectionRule::SmallestCapacity,
    }
}

fn request(candidates: &[&str], operating: &str) -> SizingRequest {
    SizingRequest {
        weight_case: "load:W".into(),
        weight_support_state: WeightSupportState::OwnSolve,
        design_operating_case: operating.into(),
        check_operating_cases: Vec::new(),
        candidates: candidates.iter().map(|c| RequestCandidate { support: c.to_string(), family_policy: FamilyPolicy::Variable }).collect(),
        method: method(),
        solver_mode: PreviewSolverMode::DenseScrutiny,
    }
}

/// Two equal spans of 4 m on three simple supports, w = 1000 N/m, the interior a
/// candidate. The ends are pinned (no rotational restraint in bending), so the
/// classical continuous-beam result applies; Y at A and C and a torsion spring at A
/// remove the out-of-plane and torsional mechanisms.
fn two_span() -> Value {
    two_span_on(Route::PressureFree)
}

/// [`two_span`] on a route: PF, its EX twin, or its EX load/reference-state twin.
fn two_span_on(route: Route) -> Value {
    let member = |id, from, to| Member { id, from, to, outside_diameter_m: 0.2, wall_m: 0.01, elastic_modulus_pa: STEEL_E, expansion_per_k: ALPHA };
    let nodes = [("node:A", [0.0, 0.0, 0.0]), ("node:B", [4.0, 0.0, 0.0]), ("node:C", [8.0, 0.0, 0.0])];
    let members = [member("pipe:AB", "node:A", "node:B"), member("pipe:BC", "node:B", "node:C")];
    let supports = vec![
        support("support:A", "node:A", "guide", &["UX", "UY", "UZ"]),
        torsion_spring("support:A-torsion", "node:A"),
        support("support:B", "node:B", "vertical_support", &["UZ"]),
        support("support:C", "node:C", "guide", &["UY", "UZ"]),
    ];
    let cases = vec![
        case("load:W", vec![line_load("load:W-AB", "pipe:AB", 1000.0), line_load("load:W-BC", "pipe:BC", 1000.0)]),
        case("load:O", vec![line_load("load:O-AB", "pipe:AB", 1000.0), line_load("load:O-BC", "pipe:BC", 1000.0),
                            nodal_fz("load:O-lift", "node:B", 500.0)]),
    ];
    on(route, model_builder::request(&nodes, &members, supports, cases))
}

fn close(a: f64, b: f64, rel: f64) -> bool {
    (a - b).abs() <= rel * a.abs().max(b.abs()) + 1e-12
}

#[test]
fn two_span_interior_reaction_is_the_hot_load_basis_and_the_stored_request_is_unchanged() {
    for route in ROUTES {
        two_span_sized_on(route);
    }
}

fn two_span_sized_on(route: Route) {
    let doc = two_span_on(route);
    let before = serde_json::to_vec(&doc).unwrap();
    let records = vec![Record {
        hanger_id: "VS-1".into(),
        hanger_type: HangerType::VariableSpring,
        stiffness_n_per_m: Some(1.0e6),
        stiffness_dof: Some(Dof::Uz),
        working_load_range_n: Some((1000.0, 20000.0)),
        travel_range_m: Some(0.1),
    }];
    let run = run(&doc, &records, &request(&["support:B"], "load:O")).unwrap_or_else(|r| panic!("{r:?}"));
    assert_eq!(serde_json::to_vec(&doc).unwrap(), before, "a sizing run never changes the stored request");
    // W = 5wL/4 = 5000 N; with W as a force the cold position is the rigid one, u_C = 0.
    assert!(close(run.candidates[0].w_n, 5000.0, 1e-9), "W = {}", run.candidates[0].w_n);
    assert!(run.candidates[0].positions.u_cold_m.abs() < 1e-9);
    // The operating case adds 500 N up at B with B as the force W: it moves up.
    assert!(run.candidates[0].positions.u_operating_m[0] > 0.0);
    assert_eq!(run.results[0].outcome, Outcome::Selected);
    assert!(run.verifications[0].is_some());
    assert_eq!(run.standing, VerificationStanding::Unconditional);
    assert_eq!(run.case_load_categories["load:W"], ["weight"], "{route:?}");
}

#[test]
fn derived_documents_keep_one_case_and_put_each_candidate_in_its_role() {
    let doc = two_span();
    let site = Site { support: "support:B".into(), node: "node:B".into(), dof: Dof::Uz };
    let rigid = derived_document(&doc, "load:W", &[site.clone()], &[Role::Rigid]);
    let model = &rigid["model"];
    assert_eq!(model["load_cases"].as_array().unwrap().len(), 1);
    let b = model["supports"].as_array().unwrap().iter().find(|s| s["id"] == "support:B").unwrap();
    assert_eq!(b["family"], "guide");
    assert_eq!(b["restraints"], json!(["UZ"]));
    let force = derived_document(&doc, "load:O", &[site.clone()], &[Role::Force(5000.0)]);
    let model = &force["model"];
    assert!(model["supports"].as_array().unwrap().iter().all(|s| s["id"] != "support:B"));
    let loads = model["load_cases"][0]["primitive_loads"].as_array().unwrap();
    let added = loads.iter().find(|l| l["id"] == "load:hanger-sizing:support:B").unwrap();
    assert_eq!(added["magnitude"]["value"], 5000.0);
    assert_eq!(added["direction"], "global_z");
    let spring = derived_document(&doc, "load:W", &[site], &[Role::Spring { k: 2.0e5, f_cold: 5100.0 }]);
    let b = spring["model"]["supports"].as_array().unwrap().iter().find(|s| s["id"] == "support:B").unwrap().clone();
    assert_eq!(b["family"], "spring");
    assert_eq!(b["stiffness"]["value"]["value"], 2.0e5);
}

fn support_refs(doc: &Value) -> Vec<String> {
    doc["model"]["load_cases"][0]["analysis_state"]["support_states"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["support_ref"].as_str().unwrap().to_string())
        .collect()
}

fn ledger(doc: &Value) -> Vec<Value> {
    doc["model"]["load_cases"][0]["analysis_state"]["load_sources"].as_array().unwrap().clone()
}

fn admitted(doc: Value, what: &str) {
    let envelope = open_pipe_stress_product_physics::run_linear_static_preview_value_with_mode(doc, PreviewSolverMode::DenseScrutiny)
        .unwrap_or_else(|e| panic!("{what}: refused: {e}"));
    let blocking: Vec<&str> = envelope.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect();
    assert!(blocking.is_empty(), "{what}: {blocking:?}");
}

/// AC-003 on EX: on a load/reference-state case a rigid or spring candidate keeps
/// its support state, a force candidate loses it with the support, and each added
/// force is entered in the case's ledger at factor 1; every derived document is
/// admitted by the solve. On an EX case without an analysis state, and on PF,
/// nothing of this applies and the documents are admitted too.
#[test]
fn ex_derived_documents_reconcile_support_states_and_the_ledger_and_are_admitted() {
    let site = Site { support: "support:B".into(), node: "node:B".into(), dof: Dof::Uz };
    let doc = two_span_on(Route::ExactLoadReference);
    let stored_ledger = ledger(&derived_document(&doc, "load:O", &[], &[]));
    assert_eq!(stored_ledger.len(), 3, "the operating case's three stored loads");
    let entry = json!({"source_ref": "load:hanger-sizing:support:B", "factor": 1.0});

    let rigid = derived_document(&doc, "load:W", &[site.clone()], &[Role::Rigid]);
    assert!(support_refs(&rigid).contains(&"support:B".to_string()));
    assert!(!ledger(&rigid).contains(&entry));
    admitted(rigid, "rigid");

    let force = derived_document(&doc, "load:O", &[site.clone()], &[Role::Force(5000.0)]);
    assert!(!support_refs(&force).contains(&"support:B".to_string()));
    assert_eq!(support_refs(&force).len(), 3, "every other support keeps its state");
    let mut expected = stored_ledger.clone();
    expected.push(entry.clone());
    assert_eq!(ledger(&force), expected, "the stored ledger, then the added force at factor 1");
    admitted(force, "force");

    let spring = derived_document(&doc, "load:O", &[site.clone()], &[Role::Spring { k: 2.0e5, f_cold: 5100.0 }]);
    assert!(support_refs(&spring).contains(&"support:B".to_string()));
    assert_eq!(ledger(&spring), expected, "the spring's cold-load force is in the ledger");
    admitted(spring, "spring");

    // EX without an analysis state, and PF: no analysis state is formed.
    for route in [Route::Exact, Route::PressureFree] {
        let doc = two_span_on(route);
        for (case, role) in [("load:W", Role::Rigid), ("load:O", Role::Force(5000.0)), ("load:O", Role::Spring { k: 2.0e5, f_cold: 5100.0 })] {
            let derived = derived_document(&doc, case, &[site.clone()], &[role]);
            assert!(derived["model"]["load_cases"][0].get("analysis_state").is_none(), "{route:?}");
            admitted(derived, &format!("{route:?} {role:?}"));
        }
    }
}

#[test]
fn candidate_and_case_refusals_are_listed_together() {
    for route in ROUTES {
        let mut doc = two_span_on(route);
        // A second support sharing B's DOF, and a candidate that is an anchor.
        add_support(&mut doc, support("support:B2", "node:B", "vertical_support", &["UZ"]));
        let mut asked = request(&["support:B", "support:A", "support:missing", "support:B"], "load:W");
        asked.check_operating_cases = vec!["load:nope".into()];
        let refusals = run(&doc, &[], &asked).unwrap_err();
        let codes: Vec<&str> = refusals.iter().map(|r| r.code).collect();
        for code in [
            "HANGER_SIZING_CASE_UNKNOWN",
            "HANGER_SIZING_CASE_ROLES_OVERLAP",
            "HANGER_SIZING_CANDIDATE_DUPLICATE",
            "HANGER_SIZING_CANDIDATE_UNKNOWN",
            "HANGER_SIZING_CANDIDATE_DOF_UNSUPPORTED",
            "HANGER_SIZING_CANDIDATE_DOF_SHARED",
        ] {
            assert!(codes.contains(&code), "{route:?}: {code} missing from {codes:?}");
        }
    }
}

#[test]
fn a_singular_force_replacement_refuses_the_request_with_the_solve_diagnostics() {
    for route in ROUTES {
        // A single span pinned about Y at A with the candidate at B: rigid, it is a
        // simple beam; replaced by a force, the span rotates freely about A, a
        // mechanism the derived solve refuses.
        let member = Member { id: "pipe:AB", from: "node:A", to: "node:B", outside_diameter_m: 0.2, wall_m: 0.01,
                              elastic_modulus_pa: STEEL_E, expansion_per_k: ALPHA };
        let doc = on(route, model_builder::request(
            &[("node:A", [0.0, 0.0, 0.0]), ("node:B", [4.0, 0.0, 0.0])],
            &[member],
            vec![support("support:A", "node:A", "guide", &["UX", "UY", "UZ"]), torsion_spring("support:A-torsion", "node:A"),
                 support("support:B-lateral", "node:B", "guide", &["UY"]), support("support:B", "node:B", "vertical_support", &["UZ"])],
            vec![case("load:W", vec![line_load("load:W-AB", "pipe:AB", 100.0)]), case("load:O", vec![line_load("load:O-AB", "pipe:AB", 100.0)])],
        ));
        let refusals = run(&doc, &[], &request(&["support:B"], "load:O")).unwrap_err();
        assert_eq!(refusals[0].code, "HANGER_SIZING_DERIVED_SOLVE_FAILED", "{route:?}");
        assert_eq!(refusals[0].names[0], "S_C", "{route:?}");
    }
}

/// REQ-009: what the solve refuses refuses the request with the solve's code. A
/// non-candidate support with no state in the weight case is the solve's own
/// refusal, carried in the derived-solve refusal's message.
#[test]
fn a_solve_refusal_of_the_stored_model_refuses_the_request_with_the_solves_code() {
    let mut doc = two_span_on(Route::ExactLoadReference);
    doc["model"]["load_cases"][0]["analysis_state"]["support_states"].as_array_mut().unwrap().retain(|s| s["support_ref"] != "support:C");
    let refusals = run(&doc, &[], &request(&["support:B"], "load:O")).unwrap_err();
    assert_eq!(refusals[0].code, "HANGER_SIZING_DERIVED_SOLVE_FAILED");
    assert_eq!(refusals[0].names, ["S_W"]);
    assert!(refusals[0].message.contains("LOAD_STATE_SUPPORT_STATE_MISSING"), "{}", refusals[0].message);
}

fn hanger_variable(id: &str, node: &str, restraints: &[&str], dof: &str) -> Value {
    json!({"id": id, "node": node, "family": "variable_spring_hanger", "restraints": restraints,
           "hanger": {"hanger_type": "variable_spring_hanger",
                      "stiffness": {"dof": dof, "value": {"value": 1.0e5, "unit": "N/m"}},
                      "cold_load": {"value": 5000.0, "unit": "N"}, "installed_load": {"value": 5000.0, "unit": "N"},
                      "hot_load": {"value": 4800.0, "unit": "N"}, "travel_range": {"value": 0.05, "unit": "m"},
                      "source_reference": "invented-local:VS-1"},
           "provenance": "invented_test_value"})
}

/// Replaces support B of a two-span document by `replacement`, keeping its states.
fn with_b(route: Route, replacement: Value) -> Value {
    let mut doc = two_span_on(route);
    let supports = doc["model"]["supports"].as_array_mut().unwrap();
    let index = supports.iter().position(|s| s["id"] == "support:B").unwrap();
    supports[index] = replacement;
    doc
}

fn one_record() -> Vec<Record> {
    vec![variable_record("VS-1", 1.0e6, (1000.0, 20000.0)), constant_record("CE-1", (4000.0, 6000.0))]
}

fn codes_and_names(refusals: &[Refusal]) -> Vec<(&str, Vec<String>)> {
    refusals.iter().map(|r| (r.code, r.names.clone())).collect()
}

/// REQ-002 as decided: (a) an authored variable spring hanger with an empty
/// restraint list is a candidate on its stiffness DOF; (d) a support written as an
/// applied selection writes it (variable: one restraint equal to its stiffness DOF,
/// with hanger loads; constant: one restraint and a constant load) is sized again.
/// Each on PF, EX and an EX load/reference-state document, with W = 5wL/4.
#[test]
fn authored_and_applied_hangers_are_candidates_on_every_route() {
    let constant = json!({"id": "support:B", "node": "node:B", "family": "constant_effort_support", "restraints": ["UZ"],
        "hanger": {"hanger_type": "constant_effort_support", "constant_load": {"value": 5000.0, "unit": "N"},
                   "travel_range": {"value": 0.1, "unit": "m"}, "source_reference": "invented-local:CE-1"},
        "provenance": "invented_test_value"});
    let cases = [
        ("authored variable, no restraint", hanger_variable("support:B", "node:B", &[], "UZ"), FamilyPolicy::Variable),
        ("applied variable", hanger_variable("support:B", "node:B", &["UZ"], "UZ"), FamilyPolicy::Variable),
        ("applied constant", constant, FamilyPolicy::Constant),
    ];
    for route in ROUTES {
        for (what, replacement, policy) in &cases {
            let doc = with_b(route, replacement.clone());
            let mut asked = request(&["support:B"], "load:O");
            asked.candidates[0].family_policy = *policy;
            let run = run(&doc, &one_record(), &asked).unwrap_or_else(|r| panic!("{route:?} {what}: {r:?}"));
            assert_eq!(run.sites[0].dof, Dof::Uz, "{route:?} {what}");
            assert!(close(run.candidates[0].w_n, 5000.0, 1e-9), "{route:?} {what}: W = {}", run.candidates[0].w_n);
            assert_eq!(run.results[0].outcome, Outcome::Selected, "{route:?} {what}");
        }
    }
}

/// REQ-002 (b) and (c), on every route: a single restraint whose stiffness DOF
/// differs acts on two DOFs; a restraint-free support that is not a variable spring
/// hanger acts on none; and a candidate whose DOF comes from its stiffness shares
/// its node and DOF with another support's rigid restraint or spring.
#[test]
fn a_second_dof_or_a_shared_stiffness_dof_is_refused_by_name_on_every_route() {
    let b = |v: Vec<String>| v;
    let only_b = || b(vec!["support:B".to_string()]);
    for route in ROUTES {
        // (b) Restraint UZ with a hanger stiffness on UY, and with a top-level stiffness on UX.
        for replacement in [
            hanger_variable("support:B", "node:B", &["UZ"], "UY"),
            json!({"id": "support:B", "node": "node:B", "family": "spring", "restraints": ["UZ"],
                   "stiffness": {"dof": "UX", "value": {"value": 1.0e5, "unit": "N/m"}}, "provenance": "invented_test_value"}),
            json!({"id": "support:B", "node": "node:B", "family": "spring", "restraints": [],
                   "stiffness": {"dof": "UZ", "value": {"value": 1.0e5, "unit": "N/m"}}, "provenance": "invented_test_value"}),
        ] {
            let refusals = run(&with_b(route, replacement.clone()), &one_record(), &request(&["support:B"], "load:O")).unwrap_err();
            assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_CANDIDATE_DOF_UNSUPPORTED", only_b())], "{route:?} {replacement}");
        }
        // (c) The candidate's DOF from its stiffness, shared with a rigid restraint and with a spring.
        for other in [
            support("support:B-rigid", "node:B", "vertical_support", &["UZ"]),
            hanger_variable("support:B-spring", "node:B", &[], "UZ"),
        ] {
            let mut doc = with_b(route, hanger_variable("support:B", "node:B", &[], "UZ"));
            add_support(&mut doc, other.clone());
            let refusals = run(&doc, &one_record(), &request(&["support:B"], "load:O")).unwrap_err();
            assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_CANDIDATE_DOF_SHARED", only_b())], "{route:?} {}", other["id"]);
        }
    }
}

/// REQ-002 on an EX load/reference-state document: a candidate needs, in every
/// named case, exactly one ordinary support state. Each kind of departure is
/// refused by name (HANGER_SIZING_CANDIDATE_MOVED); a missing state is named by
/// sizing, because the operating cases' force solves remove the candidate and
/// would never observe it. The candidate's own entry is not a reference, but a
/// reference to it inside another support's state is.
#[test]
fn a_moved_or_stateless_candidate_is_refused_by_name_and_its_own_state_is_not_a_reference() {
    let records = one_record();
    let asked = request(&["support:B"], "load:O");
    assert!(run(&two_span_on(Route::ExactLoadReference), &records, &asked).is_ok(), "the candidate's own active state");
    let motion = json!([{"dof": "UZ", "value": {"value": 0.002, "unit": "m"}, "meaning": "absolute_reference_displacement"}]);
    let departures: Vec<(&str, Box<dyn Fn(&mut Value)>)> = vec![
        ("boundary motion", Box::new({ let m = motion.clone(); move |s: &mut Value| s["boundary_motion"] = m.clone() })),
        ("base motion", Box::new({ let m = motion.clone(); move |s: &mut Value| s["base_motion"] = m.clone() })),
        ("device reference", Box::new(|s: &mut Value| s["device_reference"] = json!({"kind": "unloaded_reference",
            "reference_position": {"value": 0.0, "unit": "m"}}))),
        ("inactive", Box::new(|s: &mut Value| s["participation"] = json!({"kind": "inactive"}))),
    ];
    for case in ["load:W", "load:O"] {
        for (what, depart) in &departures {
            let mut doc = two_span_on(Route::ExactLoadReference);
            depart(support_state_mut(&mut doc, case, "support:B"));
            let refusals = run(&doc, &records, &asked).unwrap_err();
            assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_CANDIDATE_MOVED", vec!["support:B".to_string()])], "{case} {what}");
        }
        // No state, and two states.
        let mut doc = two_span_on(Route::ExactLoadReference);
        let case_index = if case == "load:W" { 0 } else { 1 };
        let states = doc["model"]["load_cases"][case_index]["analysis_state"]["support_states"].as_array_mut().unwrap();
        states.retain(|s| s["support_ref"] != "support:B");
        let refusals = run(&doc, &records, &asked).unwrap_err();
        assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_CANDIDATE_MOVED", vec!["support:B".to_string()])], "{case} missing");
        let mut doc = two_span_on(Route::ExactLoadReference);
        let states = doc["model"]["load_cases"][case_index]["analysis_state"]["support_states"].as_array_mut().unwrap();
        states.push(json!({"support_ref": "support:B", "participation": {"kind": "active_model_device"}}));
        let refusals = run(&doc, &records, &asked).unwrap_err();
        assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_CANDIDATE_MOVED", vec!["support:B".to_string()])], "{case} repeated");
    }
    // An empty motion list moves nothing.
    let mut doc = two_span_on(Route::ExactLoadReference);
    support_state_mut(&mut doc, "load:O", "support:B")["boundary_motion"] = json!([]);
    assert!(run(&doc, &records, &asked).is_ok());
    // Another support's state that names the candidate is a reference.
    let mut doc = two_span_on(Route::ExactLoadReference);
    support_state_mut(&mut doc, "load:O", "support:C")["participation"] = json!({"kind": "locked_equivalent_support",
        "components": [{"dof": "UZ", "position_source": {"kind": "predecessor_value", "case_ref": "load:W",
            "state_hash": "0".repeat(64), "support_ref": "support:B", "dof": "UZ"}}]});
    let refusals = run(&doc, &records, &asked).unwrap_err();
    assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_CANDIDATE_REFERENCED", vec!["support:B".to_string()])]);
    // A moved non-candidate is the model's own and is sized around.
    let mut doc = two_span_on(Route::ExactLoadReference);
    support_state_mut(&mut doc, "load:O", "support:C")["boundary_motion"] = motion;
    assert!(run(&doc, &records, &asked).is_ok());
}

#[test]
fn library_records_normalize_to_si_and_a_record_without_a_range_is_not_sizable() {
    let document: Value = serde_json::from_str(include_str!("../../../../fixtures/hanger/invented_hanger_cases.json")).unwrap();
    let with_range = document.as_array().unwrap().iter().find(|c| c["name"] == "1.1.0 working load range valid").unwrap();
    let records = open_pipe_stress_hanger_sizing::library::records(&with_range["payload"]);
    let variable = records.iter().find(|r| r.hanger_type == HangerType::VariableSpring).unwrap();
    assert_eq!(variable.working_load_range_n, Some((10.0, 50.0)));
    assert_eq!(variable.travel_range_m, Some(0.12));
    let without: Value = serde_json::from_str(include_str!("../../../../fixtures/hanger/invented_hanger_library_valid.json")).unwrap();
    let records = open_pipe_stress_hanger_sizing::library::records(&without);
    assert!(records.iter().all(|r| r.working_load_range_n.is_none()));
    // The constant record's movement limit is in inches; its travel range is absent.
    let constant = records.iter().find(|r| r.hanger_type == HangerType::ConstantEffort).unwrap();
    assert_eq!(constant.travel_range_m, None);
}

const R125: &str = include_str!("../../../../validation/references/hanger_sizing/r125_derived_solves.json");

/// Builds the product solve request for one R-1/R-2/R-5 case. A reference guide's
/// RX restraint, which carries no load in these planar cases, becomes a stiff
/// torsion spring, because the product's guide restrains translations only.
fn reference_request(case: &Value) -> Value {
    let nodes: Vec<(String, [f64; 3])> = case["nodes"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(id, p)| (id.clone(), [p[0].as_f64().unwrap(), p[1].as_f64().unwrap(), p[2].as_f64().unwrap()]))
        .collect();
    let node_refs: Vec<(&str, [f64; 3])> = nodes.iter().map(|(id, p)| (id.as_str(), *p)).collect();
    let members: Vec<Member> = case["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| Member {
            id: m["id"].as_str().unwrap(),
            from: m["end_nodes"][0].as_str().unwrap(),
            to: m["end_nodes"][1].as_str().unwrap(),
            outside_diameter_m: m["outside_diameter_m"].as_f64().unwrap(),
            wall_m: m["wall_thickness_m"].as_f64().unwrap(),
            elastic_modulus_pa: m["elastic_modulus_pa"].as_f64().unwrap(),
            expansion_per_k: m["thermal_expansion_per_k"].as_f64().unwrap(),
        })
        .collect();
    let mut supports = Vec::new();
    for s in case["supports"].as_array().unwrap() {
        let (id, node) = (s["id"].as_str().unwrap(), s["node"].as_str().unwrap());
        let dofs: Vec<&str> = s["dofs"].as_array().unwrap().iter().map(|d| d.as_str().unwrap()).collect();
        match s["kind"].as_str().unwrap() {
            "anchor" => supports.push(support(id, node, "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"])),
            "rigid_uz" | "candidate" => supports.push(support(id, node, "vertical_support", &["UZ"])),
            "lift_off_uz" => supports.push(lift_off(id, node)),
            "guide" => {
                let translations: Vec<&str> = dofs.iter().copied().filter(|d| d.starts_with('U')).collect();
                supports.push(support(id, node, "guide", &translations));
                if dofs.contains(&"RX") {
                    supports.push(torsion_spring(&format!("{id}-torsion"), node));
                }
            }
            other => panic!("unknown support kind {other}"),
        }
    }
    let cases: Vec<Value> = case["load_cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|lc| {
            let id = lc["id"].as_str().unwrap();
            let mut loads = Vec::new();
            for (i, l) in lc["line_loads"].as_array().into_iter().flatten().enumerate() {
                loads.push(line_load(&format!("load:{id}-w{i}"), l["member"].as_str().unwrap(), l["w_n_per_m"].as_f64().unwrap()));
            }
            for (i, t) in lc["delta_t_k"].as_array().into_iter().flatten().enumerate() {
                loads.push(thermal(&format!("load:{id}-t{i}"), t["member"].as_str().unwrap(), t["value"].as_f64().unwrap()));
            }
            for (i, f) in lc["nodal_forces"].as_array().into_iter().flatten().enumerate() {
                loads.push(nodal_fz(&format!("load:{id}-f{i}"), f["node"].as_str().unwrap(), f["fz_n"].as_f64().unwrap()));
            }
            model_builder::case(id, loads)
        })
        .collect();
    model_builder::request(&node_refs, &members, supports, cases)
}

#[test]
fn derived_solves_reproduce_the_closed_form_references_r1_r2_r5() {
    let document: Value = serde_json::from_str(R125).unwrap();
    assert_eq!(document["schema"], "swbpipe.hanger_sizing_reference.r125/1");
    // On PF, on its EX twin and on its EX load/reference-state twin, where R-2's
    // thermal primitive becomes the member's constant-α interval state.
    for (route, case) in ROUTES.iter().flat_map(|r| document["cases"].as_array().unwrap().iter().map(move |c| (*r, c))) {
        let id = format!("{}/{route:?}", case["id"].as_str().unwrap());
        let doc = on(route, reference_request(case));
        let weight = case["load_cases"].as_array().unwrap().iter().find(|c| c["role"] == "weight").unwrap()["id"].as_str().unwrap();
        let operating: Vec<&str> = case["load_cases"].as_array().unwrap().iter()
            .filter(|c| c["role"] == "operating").map(|c| c["id"].as_str().unwrap()).collect();
        let expected = case["expected"]["candidates"].as_array().unwrap();
        let candidates: Vec<&str> = expected.iter().map(|c| c["support"].as_str().unwrap()).collect();
        let mut asked = request(&candidates, operating[0]);
        asked.weight_case = weight.into();
        asked.check_operating_cases = operating[1..].iter().map(|s| s.to_string()).collect();
        let run = run(&doc, &[], &asked).unwrap_or_else(|r| panic!("{id}: {r:?}"));
        for e in expected {
            let who = format!("{id}/{}", e["support"].as_str().unwrap());
            let c = run.candidates.iter().find(|c| c.id == e["support"].as_str().unwrap()).unwrap();
            let w = e["w_n"].as_f64().unwrap();
            assert!(close(c.w_n, w, 1e-9), "{who} W {} against {w}", c.w_n);
            // Positions: 1e-9 relative, with a floor of 1e-9 of the case's largest
            // expected position or 1e-12 m, for values that are zero in closed form.
            let largest = e["u_operating_m"].as_object().unwrap().values().map(|v| v.as_f64().unwrap().abs()).fold(1e-3, f64::max);
            let near = |a: f64, b: f64| (a - b).abs() <= 1e-9 * a.abs().max(b.abs()) + 1e-9 * largest;
            let u_c = e["u_cold_m"].as_f64().unwrap();
            assert!(near(c.positions.u_cold_m, u_c), "{who} u_C {} against {u_c}", c.positions.u_cold_m);
            for (i, case_id) in operating.iter().enumerate() {
                let u = e["u_operating_m"][*case_id].as_f64().unwrap();
                assert!(near(c.positions.u_operating_m[i], u), "{who} u_{case_id} {} against {u}", c.positions.u_operating_m[i]);
            }
        }
        // R-5a: the lift-off is active in S_W and inactive in the design operating solve,
        // which is disclosed by name; the cold reference keeps S_W's state. R-5b: none.
        let differs: Vec<&Disclosure> = run.disclosures.iter().filter(|d| d.code == "HANGER_SIZING_SUPPORT_STATE_DIFFERS").collect();
        let lifting: Vec<String> = case["expected"]["nonlinear_states"].as_object().into_iter().flatten()
            .filter(|(_, by)| by["weight_rigid"] != by[operating[0]])
            .map(|(support, _)| support.clone())
            .collect();
        if lifting.is_empty() {
            assert!(differs.is_empty(), "{id}: unexpected {differs:?}");
        } else {
            assert_eq!(differs.len(), 1, "{id}");
            assert_eq!(differs[0].names, lifting, "{id}");
        }
        assert!(run.disclosures.iter().all(|d| d.code != "HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS"), "{id}");
        if let Some(states) = case["expected"]["nonlinear_states"].as_object() {
            for (support, by_solve) in states {
                for (solve, state) in by_solve.as_object().unwrap() {
                    let label = match solve.as_str() {
                        "weight_rigid" => "S_W".to_string(),
                        "cold" => "S_C".to_string(),
                        case_id => format!("S_O{}", operating.iter().position(|c| *c == case_id).unwrap() + 1),
                    };
                    let found = &run.states.iter().find(|s| s.solve == label).unwrap().states[support];
                    assert_eq!(found, state.as_str().unwrap(), "{id} {support} in {label}");
                }
            }
        }
    }
}

fn variable_record(id: &str, k: f64, range: (f64, f64)) -> Record {
    Record { hanger_id: id.into(), hanger_type: HangerType::VariableSpring, stiffness_n_per_m: Some(k),
             stiffness_dof: Some(Dof::Uz), working_load_range_n: Some(range), travel_range_m: Some(0.1) }
}

fn constant_record(id: &str, range: (f64, f64)) -> Record {
    Record { hanger_id: id.into(), hanger_type: HangerType::ConstantEffort, stiffness_n_per_m: None,
             stiffness_dof: None, working_load_range_n: Some(range), travel_range_m: Some(0.1) }
}

#[test]
fn a_selected_spring_is_verified_as_its_rate_plus_its_cold_load_in_closed_form() {
    // The two-span beam is one pinned span 2L = 8 m with the candidate at midspan.
    // Midspan flexibility δ = (2L)³/(48EI); the weight's midspan deflection is δ·W,
    // since W = 5wL/4. Operating adds P = 500 N up at B, so u_D = δ·P. Hot design:
    // F_cold = W + k·u_D. With the spring in place, u = δ·(F_cold − k·u − W + P_case),
    // so u = δ·(F_cold − W + P_case)/(1 + δ·k).
    let doc = two_span();
    let k = 1.0e6;
    let records = vec![variable_record("VS-1", k, (1000.0, 20000.0))];
    let run = run(&doc, &records, &request(&["support:B"], "load:O")).unwrap_or_else(|r| panic!("{r:?}"));
    let (d_out, d_in) = (0.2_f64, 0.2_f64 - 2.0 * 0.01);
    let ei = STEEL_E * std::f64::consts::PI / 64.0 * (d_out.powi(4) - d_in.powi(4));
    let delta = 8.0_f64.powi(3) / (48.0 * ei);
    let (w, p) = (5000.0, 500.0);
    let u_d = delta * p;
    assert!(close(run.candidates[0].positions.u_operating_m[0], u_d, 1e-9));
    let f_cold = w + k * u_d;
    let Some(Selected::Variable { loads, .. }) = &run.results[0].selected else { panic!("a variable selection") };
    assert!(close(loads.f_cold_n, f_cold, 1e-9));
    let v = run.verifications[0].as_ref().unwrap();
    let u_c_v = delta * (f_cold - w) / (1.0 + delta * k);
    let u_o_v = delta * (f_cold - w + p) / (1.0 + delta * k);
    assert!(close(v.positions.u_cold_m, u_c_v, 1e-9), "{} against {u_c_v}", v.positions.u_cold_m);
    assert!(close(v.positions.u_operating_m[0], u_o_v, 1e-9), "{} against {u_o_v}", v.positions.u_operating_m[0]);
    let vv = v.variable.as_ref().unwrap();
    assert!(close(vv.f_cold_condition_n, f_cold - k * u_c_v, 1e-9));
    assert!(close(vv.f_operating_n[0], f_cold - k * u_o_v, 1e-9));
    assert!(close(vv.departure_n, f_cold - k * u_o_v - w, 1e-9));
}

fn displacement_m(envelope: &open_pipe_stress_product_physics::MechanicsEnvelope, node: &str) -> f64 {
    let row = envelope.results.iter().find(|r| r.kind == "global_nodal_displacement_z" && r.entity_ref == node).unwrap();
    match row.unit.as_str() { "mm" => row.value / 1000.0, "m" => row.value, other => panic!("unit {other}") }
}

#[test]
fn verification_puts_each_selection_in_its_role_and_marks_a_no_fit_conditional() {
    // Four equal spans of 4 m: interior reactions 8wL/7 at N2 and N4 and 13wL/14 at N3.
    // N2 takes a variable record, N3 a constant record that fits 13wL/14 = 3714 N,
    // and N4 has no fit (8wL/7 = 4571 N is outside the constant record's range).
    let member = |id, from, to| Member { id, from, to, outside_diameter_m: 0.2, wall_m: 0.01, elastic_modulus_pa: STEEL_E, expansion_per_k: ALPHA };
    let nodes = [("N1", [0.0, 0.0, 0.0]), ("N2", [4.0, 0.0, 0.0]), ("N3", [8.0, 0.0, 0.0]), ("N4", [12.0, 0.0, 0.0]), ("N5", [16.0, 0.0, 0.0])];
    let members = [member("P1", "N1", "N2"), member("P2", "N2", "N3"), member("P3", "N3", "N4"), member("P4", "N4", "N5")];
    let supports = vec![
        support("S1", "N1", "guide", &["UX", "UY", "UZ"]), torsion_spring("S1-torsion", "N1"),
        support("H2", "N2", "vertical_support", &["UZ"]), support("H3", "N3", "vertical_support", &["UZ"]),
        support("H4", "N4", "vertical_support", &["UZ"]), support("S5", "N5", "guide", &["UY", "UZ"]),
    ];
    let weights = |c: &str| ["P1", "P2", "P3", "P4"].iter().map(|m| line_load(&format!("load:{c}-{m}"), m, 1000.0)).collect::<Vec<_>>();
    let mut operating = weights("O");
    operating.push(nodal_fz("load:O-lift", "N2", 400.0));
    let doc = model_builder::request(&nodes, &members, supports, vec![case("load:W", weights("W")), case("load:O", operating)]);
    // k = 1e5 N/m keeps N2's variation within 0.25 over its operating travel.
    let records = vec![variable_record("VS-1", 1.0e5, (1000.0, 20000.0)), constant_record("CE-1", (3000.0, 4000.0))];
    let mut asked = request(&["H2", "H3", "H4"], "load:O");
    asked.candidates[1].family_policy = FamilyPolicy::Constant;
    asked.candidates[2].family_policy = FamilyPolicy::Constant;
    let run = run(&doc, &records, &asked).unwrap_or_else(|r| panic!("{r:?}"));
    assert_eq!(run.results.iter().map(|r| r.outcome).collect::<Vec<_>>(), [Outcome::Selected, Outcome::Selected, Outcome::NoRecordFits]);
    assert_eq!(run.standing, VerificationStanding::Conditional);
    assert!(run.verifications[2].is_none());
    // The verification solves must be exactly the derived solves of the stated roles.
    let Some(Selected::Variable { k, loads, .. }) = &run.results[0].selected else { panic!("variable") };
    let roles = [Role::Spring { k: *k, f_cold: loads.f_cold_n }, Role::Force(run.candidates[1].w_n), Role::Force(run.candidates[2].w_n)];
    for (case_id, pick) in [("load:W", None), ("load:O", Some(0usize))] {
        let derived = derived_document(&doc, case_id, &run.sites, &roles);
        let envelope = open_pipe_stress_product_physics::run_linear_static_preview_value_with_mode(derived, PreviewSolverMode::DenseScrutiny).unwrap();
        for (i, site) in run.sites.iter().enumerate().take(2) {
            let expected = displacement_m(&envelope, &site.node);
            let verified = run.verifications[i].as_ref().unwrap();
            let actual = match pick { None => verified.positions.u_cold_m, Some(j) => verified.positions.u_operating_m[j] };
            assert!(close(actual, expected, 1e-12), "{} in {case_id}: {actual} against {expected}", site.support);
        }
    }
}

#[test]
fn every_candidate_finding_is_listed_and_only_real_references_refuse() {
    for route in ROUTES {
        candidate_findings_on(route);
    }
}

fn candidate_findings_on(route: Route) {
    let mut doc = two_span_on(route);
    // A lift-off sharing B's node and UZ through its nonlinear DOF only.
    add_support(&mut doc, lift_off("support:B-lift", "node:B"));
    // Text equal to a candidate id is not a reference.
    doc["model"]["supports"][1]["provenance"] = json!("support:B");
    doc["model"]["pipe_segments"][0]["provenance"] = json!("support:B");
    let refusals = run(&doc, &[], &request(&["support:A", "support:B"], "load:O")).unwrap_err();
    assert_eq!(codes_and_names(&refusals), [
        ("HANGER_SIZING_CANDIDATE_DOF_SHARED", vec!["support:B".to_string()]),
        ("HANGER_SIZING_CANDIDATE_DOF_UNSUPPORTED", vec!["support:A".to_string()]),
    ], "{route:?}");
    // A co-located variable hanger that acts on UZ only through its hanger stiffness.
    let mut hung = two_span_on(route);
    add_support(&mut hung, json!({
        "id": "support:B-hanger", "node": "node:B", "family": "variable_spring_hanger", "restraints": [],
        "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "UZ", "value": {"value": 1.0e5, "unit": "N/m"}}},
        "provenance": "invented_test_value"}));
    let refusals = run(&hung, &[], &request(&["support:B"], "load:O")).unwrap_err();
    assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_CANDIDATE_DOF_SHARED", vec!["support:B".to_string()])], "{route:?}");
    // An anchor reports both its family and its DOFs.
    let mut anchored = two_span_on(route);
    anchored["model"]["supports"][0]["family"] = json!("anchor");
    anchored["model"]["supports"][0]["restraints"] = json!(["UX", "UY", "UZ", "RX", "RY", "RZ"]);
    let refusals = run(&anchored, &[], &request(&["support:A"], "load:O")).unwrap_err();
    let codes: Vec<&str> = refusals.iter().map(|r| r.code).collect();
    assert_eq!(codes, ["HANGER_SIZING_CANDIDATE_DOF_UNSUPPORTED", "HANGER_SIZING_CANDIDATE_FAMILY_UNSUPPORTED"], "{route:?}");
    // A skew direction.
    let mut skewed = two_span_on(route);
    skewed["model"]["supports"][2]["direction"] = json!({"x": 0.0, "y": 0.6, "z": 0.8});
    let refusals = run(&skewed, &[], &request(&["support:B"], "load:O")).unwrap_err();
    assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_CANDIDATE_SKEW_UNSUPPORTED", vec!["support:B".to_string()])], "{route:?}");
    // A support-targeted load and a friction normal-reaction source are references.
    let mut targeted = two_span_on(route);
    add_load(&mut targeted, "load:O", json!({
        "id": "load:O-imposed", "category": "imposed_displacement", "target": {"type": "support", "support": "support:B", "dof": "UZ"},
        "direction": "UZ", "magnitude": {"value": 0.001, "unit": "m"}, "dimension": "length", "provenance": "invented_test_value"}));
    let refusals = run(&targeted, &[], &request(&["support:B"], "load:O")).unwrap_err();
    assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_CANDIDATE_REFERENCED", vec!["support:B".to_string()])], "{route:?}");
    let mut sourced = two_span_on(route);
    add_support(&mut sourced, json!({
        "id": "support:F", "node": "node:C", "family": "nonlinear", "restraints": [],
        "nonlinear": {"behavior": "friction", "dof": "UX", "friction_coefficient": {"value": 0.3, "unit": "1"},
                      "normal_reaction_source": {"support_ref": "support:B", "dof": "UZ"}},
        "provenance": "invented_test_value"}));
    let refusals = run(&sourced, &[], &request(&["support:B"], "load:O")).unwrap_err();
    assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_CANDIDATE_REFERENCED", vec!["support:B".to_string()])], "{route:?}");
}

/// The run-level refusals, on every route: no candidate, a test-condition case, a
/// derived-load id already in the model, and a hot-load basis that is not positive.
#[test]
fn the_run_level_refusals_name_their_objects_on_every_route() {
    for route in ROUTES {
        let refusals = run(&two_span_on(route), &[], &request(&[], "load:O")).unwrap_err();
        assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_CANDIDATES_REQUIRED", vec![String::new()])], "{route:?}");

        let mut tested = two_span_on(route);
        tested["model"]["load_cases"][1]["test_condition"] = json!({"kind": "hydrotest"});
        let refusals = run(&tested, &[], &request(&["support:B"], "load:O")).unwrap_err();
        assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_TEST_CASE_UNSUPPORTED", vec!["load:O".to_string()])], "{route:?}");

        let mut colliding = two_span_on(route);
        add_load(&mut colliding, "load:O", nodal_fz("load:hanger-sizing:support:B", "node:C", 0.0));
        let refusals = run(&colliding, &[], &request(&["support:B"], "load:O")).unwrap_err();
        assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_ID_COLLISION", vec!["load:hanger-sizing:support:B".to_string()])],
            "{route:?}");

        // A weight case that lifts B: 12 kN up at B against 8 kN of weight.
        let mut lifting = two_span_on(route);
        add_load(&mut lifting, "load:W", nodal_fz("load:W-up", "node:B", 12000.0));
        let refusals = run(&lifting, &[], &request(&["support:B"], "load:O")).unwrap_err();
        assert_eq!(codes_and_names(&refusals), [("HANGER_SIZING_LOAD_NOT_POSITIVE", vec!["support:B".to_string()])], "{route:?}");
    }
}
