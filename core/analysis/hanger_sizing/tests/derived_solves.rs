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
        json!({"id": id, "node": node, "family": "nonlinear", "restraints": ["UZ"],
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
