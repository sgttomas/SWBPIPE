//! Derived solves through the product's public JSON entry (DEL-04-11 Design §4,
//! §5, §7): candidate and case checks, derived documents, stored-model
//! invariance, and the closed-form references R-1, R-2 and R-5 frozen under
//! `validation/references/hanger_sizing/` by a generator that reads no product code.

use open_pipe_stress_hanger_sizing::derived::*;
use open_pipe_stress_hanger_sizing::*;
use open_pipe_stress_product_physics::PreviewSolverMode;
use serde_json::{json, Value};

pub mod model_builder {
    use serde_json::{json, Value};

    pub struct Member<'a> {
        pub id: &'a str,
        pub from: &'a str,
        pub to: &'a str,
        pub outside_diameter_m: f64,
        pub wall_m: f64,
        pub elastic_modulus_pa: f64,
        pub expansion_per_k: f64,
    }

    pub fn material_id(member: &str) -> String {
        format!("material:{member}")
    }

    /// An invented pressure-free solve request: `{model, materials}`.
    pub fn request(nodes: &[(&str, [f64; 3])], members: &[Member], supports: Vec<Value>, load_cases: Vec<Value>) -> Value {
        let materials: Vec<Value> = members
            .iter()
            .map(|m| json!({
                "id": material_id(m.id),
                "label": "Invented test material",
                "elastic_modulus": {"value": m.elastic_modulus_pa, "unit": "Pa"},
                "shear_modulus": {"value": m.elastic_modulus_pa / 2.6, "unit": "Pa"},
                "thermal_expansion_coefficient": {"value": m.expansion_per_k, "unit": "1/degC"},
                "provenance": "invented_test_value"
            }))
            .collect();
        let pipes: Vec<Value> = members
            .iter()
            .map(|m| {
                let a = nodes.iter().find(|(id, _)| *id == m.from).unwrap().1;
                let b = nodes.iter().find(|(id, _)| *id == m.to).unwrap().1;
                let vertical = (a[0] - b[0]).abs() < 1e-12 && (a[1] - b[1]).abs() < 1e-12;
                let y_reference = if vertical { json!({"x": 1, "y": 0, "z": 0}) } else { json!({"x": 0, "y": 0, "z": 1}) };
                json!({
                    "id": m.id, "from": m.from, "to": m.to,
                    "section": {"outside_diameter": {"value": m.outside_diameter_m, "unit": "m"}, "wall_thickness": {"value": m.wall_m, "unit": "m"}},
                    "material": material_id(m.id), "y_reference": y_reference, "provenance": "invented_test_value"
                })
            })
            .collect();
        let nodes: Vec<Value> = nodes
            .iter()
            .map(|(id, p)| json!({"id": id, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": "invented_test_value"}))
            .collect();
        json!({
            "model": {
                "schema_version": "0.9.0",
                "document_kind": "openpipestress.product_preview.model",
                "data_boundary": {
                    "public_examples_policy": "invented_or_cleared_data_only",
                    "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
                    "private_data_policy": "no_private_project_data",
                    "professional_boundary": "technical_preview_requires_human_engineering_review"
                },
                "project": {"id": "project:hanger-sizing-test", "name": "Invented hanger sizing test",
                            "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC"}},
                "materials": materials,
                "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                                    "rule_check": "not_performed_user_rule_inputs_missing",
                                    "professional_acceptance": "not_provided"},
                "nodes": nodes,
                "pipe_segments": pipes,
                "supports": supports,
                "components": [],
                "load_cases": load_cases,
                "combinations": []
            },
            "materials": []
        })
    }

    pub fn support(id: &str, node: &str, family: &str, restraints: &[&str]) -> Value {
        json!({"id": id, "node": node, "family": family, "restraints": restraints, "provenance": "invented_test_value"})
    }

    /// A stiff rotational spring that removes a beam's torsional mechanism.
    pub fn torsion_spring(id: &str, node: &str) -> Value {
        json!({"id": id, "node": node, "family": "spring", "restraints": ["RX"],
               "stiffness": {"dof": "RX", "value": {"value": 1.0e9, "unit": "N*m/rad"}}, "provenance": "invented_test_value"})
    }

    pub fn lift_off(id: &str, node: &str) -> Value {
        json!({"id": id, "node": node, "family": "nonlinear", "restraints": [],
               "nonlinear": {"behavior": "lift_off", "dof": "UZ", "initial_state": "active", "contact_when": "positive_reaction"},
               "provenance": "invented_test_value"})
    }

    pub fn line_load(id: &str, pipe: &str, w_down_n_per_m: f64) -> Value {
        json!({"id": id, "category": "weight", "target": {"type": "element", "pipe": pipe}, "direction": "global_z",
               "magnitude": {"value": -w_down_n_per_m, "unit": "N/m"}, "dimension": "force_per_length", "provenance": "invented_test_value"})
    }

    pub fn thermal(id: &str, pipe: &str, delta_t: f64) -> Value {
        json!({"id": id, "category": "thermal", "target": {"type": "element", "pipe": pipe}, "direction": "global_z",
               "magnitude": {"value": delta_t, "unit": "degC"}, "dimension": "temperature_interval", "provenance": "invented_test_value"})
    }

    pub fn nodal_fz(id: &str, node: &str, fz_n: f64) -> Value {
        json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": node}, "direction": "global_z",
               "magnitude": {"value": fz_n, "unit": "N"}, "dimension": "force", "provenance": "invented_test_value"})
    }

    pub fn case(id: &str, loads: Vec<Value>) -> Value {
        json!({"id": id, "kind": "primitive_user_load", "primitive_loads": loads, "provenance": "invented_test_value"})
    }
}

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
    model_builder::request(&nodes, &members, supports, cases)
}

fn close(a: f64, b: f64, rel: f64) -> bool {
    (a - b).abs() <= rel * a.abs().max(b.abs()) + 1e-12
}

#[test]
fn two_span_interior_reaction_is_the_hot_load_basis_and_the_stored_request_is_unchanged() {
    let doc = two_span();
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
    assert_eq!(run.case_load_categories["load:W"], ["weight"]);
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

#[test]
fn candidate_and_case_refusals_are_listed_together() {
    let mut doc = two_span();
    // A second support sharing B's DOF, and a candidate that is an anchor.
    doc["model"]["supports"].as_array_mut().unwrap().push(support("support:B2", "node:B", "vertical_support", &["UZ"]));
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
        assert!(codes.contains(&code), "{code} missing from {codes:?}");
    }
}

#[test]
fn a_singular_force_replacement_refuses_the_request_with_the_solve_diagnostics() {
    // A single span pinned about Y at A with the candidate at B: rigid, it is a
    // simple beam; replaced by a force, the span rotates freely about A, a
    // mechanism the derived solve refuses.
    let member = Member { id: "pipe:AB", from: "node:A", to: "node:B", outside_diameter_m: 0.2, wall_m: 0.01,
                          elastic_modulus_pa: STEEL_E, expansion_per_k: ALPHA };
    let doc = model_builder::request(
        &[("node:A", [0.0, 0.0, 0.0]), ("node:B", [4.0, 0.0, 0.0])],
        &[member],
        vec![support("support:A", "node:A", "guide", &["UX", "UY", "UZ"]), torsion_spring("support:A-torsion", "node:A"),
             support("support:B-lateral", "node:B", "guide", &["UY"]), support("support:B", "node:B", "vertical_support", &["UZ"])],
        vec![case("load:W", vec![line_load("load:W-AB", "pipe:AB", 100.0)]), case("load:O", vec![line_load("load:O-AB", "pipe:AB", 100.0)])],
    );
    let refusals = run(&doc, &[], &request(&["support:B"], "load:O")).unwrap_err();
    assert_eq!(refusals[0].code, "HANGER_SIZING_DERIVED_SOLVE_FAILED");
    assert_eq!(refusals[0].names[0], "S_C");
}

#[test]
fn the_exact_route_waits_for_increment_four() {
    let mut doc = two_span();
    doc["model"]["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
    let refusals = run(&doc, &[], &request(&["support:B"], "load:O")).unwrap_err();
    assert!(refusals.iter().any(|r| r.code == "HANGER_SIZING_EXACT_ROUTE_PENDING"));
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
    for case in document["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let doc = reference_request(case);
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
    let mut doc = two_span();
    {
        let supports = doc["model"]["supports"].as_array_mut().unwrap();
        // A lift-off sharing B's node and UZ through its nonlinear DOF only.
        supports.push(lift_off("support:B-lift", "node:B"));
        // Text equal to a candidate id is not a reference.
        supports[1]["provenance"] = json!("support:B");
    }
    doc["model"]["pipe_segments"][0]["provenance"] = json!("support:B");
    let refusals = run(&doc, &[], &request(&["support:A", "support:B"], "load:O")).unwrap_err();
    let listed: Vec<(&str, Vec<String>)> = refusals.iter().map(|r| (r.code, r.names.clone())).collect();
    assert_eq!(listed, [
        ("HANGER_SIZING_CANDIDATE_DOF_SHARED", vec!["support:B".to_string()]),
        ("HANGER_SIZING_CANDIDATE_DOF_UNSUPPORTED", vec!["support:A".to_string()]),
    ]);
    // A co-located variable hanger that acts on UZ only through its hanger stiffness.
    let mut hung = two_span();
    hung["model"]["supports"].as_array_mut().unwrap().push(json!({
        "id": "support:B-hanger", "node": "node:B", "family": "variable_spring_hanger", "restraints": [],
        "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "UZ", "value": {"value": 1.0e5, "unit": "N/m"}}},
        "provenance": "invented_test_value"}));
    let refusals = run(&hung, &[], &request(&["support:B"], "load:O")).unwrap_err();
    assert!(refusals.iter().any(|r| r.code == "HANGER_SIZING_CANDIDATE_DOF_SHARED" && r.names == ["support:B"]), "{refusals:?}");
    // An anchor reports both its family and its DOFs.
    let mut anchored = two_span();
    anchored["model"]["supports"][0]["family"] = json!("anchor");
    anchored["model"]["supports"][0]["restraints"] = json!(["UX", "UY", "UZ", "RX", "RY", "RZ"]);
    let refusals = run(&anchored, &[], &request(&["support:A"], "load:O")).unwrap_err();
    let codes: Vec<&str> = refusals.iter().map(|r| r.code).collect();
    assert_eq!(codes, ["HANGER_SIZING_CANDIDATE_DOF_UNSUPPORTED", "HANGER_SIZING_CANDIDATE_FAMILY_UNSUPPORTED"]);
    // A support-targeted load and a friction normal-reaction source are references.
    let mut targeted = two_span();
    targeted["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap().push(json!({
        "id": "load:O-imposed", "category": "imposed_displacement", "target": {"type": "support", "support": "support:B", "dof": "UZ"},
        "direction": "UZ", "magnitude": {"value": 0.001, "unit": "m"}, "dimension": "length", "provenance": "invented_test_value"}));
    let refusals = run(&targeted, &[], &request(&["support:B"], "load:O")).unwrap_err();
    assert!(refusals.iter().any(|r| r.code == "HANGER_SIZING_CANDIDATE_REFERENCED" && r.names == ["support:B"]));
    let mut sourced = two_span();
    sourced["model"]["supports"].as_array_mut().unwrap().push(json!({
        "id": "support:F", "node": "node:C", "family": "nonlinear", "restraints": [],
        "nonlinear": {"behavior": "friction", "dof": "UX", "friction_coefficient": {"value": 0.3, "unit": "1"},
                      "normal_reaction_source": {"support_ref": "support:B", "dof": "UZ"}},
        "provenance": "invented_test_value"}));
    let refusals = run(&sourced, &[], &request(&["support:B"], "load:O")).unwrap_err();
    assert!(refusals.iter().any(|r| r.code == "HANGER_SIZING_CANDIDATE_REFERENCED" && r.names == ["support:B"]));
}

