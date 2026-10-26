//! W-15 slice b behaviour (DEL-03-05 Design `rigid-component-stiffness-scaling.md`
//! §3, as implemented): which selected members carry their component's weight,
//! the warning when a component fails a weight condition, mapping conflicts,
//! deterministic identities, and lineage and staleness of node-targeted generated
//! loads after each kind of edit. The numerical values are the frozen references'
//! (`w15_component_weight_benchmarks.rs`); here only selection, identity and state
//! are asserted. All inputs are invented.
use open_pipe_stress_product_physics::self_weight::{
    component_load_id, generate_self_weight_operations, inspect_applied_self_weight, AppliedSelfWeightState,
    SelfWeightGravity, SelfWeightOperationPlan, SelfWeightRequest, COMPONENT_WEIGHT_METHOD, COMPONENT_WEIGHT_NOT_APPLIED,
    COMPONENT_WEIGHT_SCOPE, GENERATED_METHOD, MANUAL_OVERRIDE_METHOD, PIPE_MASS_SCOPE,
};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewModel, PreviewSolverMode};
use serde_json::{json, Value};

const PROV: &str = "w15b invented behaviour input";
const CASE: &str = "case:w";
const VALVE: &str = "component:valve";
const HASH: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

/// a(0) - b(1) - c(2) along X, anchored at a; pipe:p1 a-b, body:v b-c; a valve at c
/// mapping body:v with weight 500 N and centre of gravity (-0.5, 0.25, 0) m.
fn model() -> Value {
    let section = json!({"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"},
        "material_density": {"value": 7850, "unit": "kg/m^3"}});
    json!({"schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w15b", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "a", "position": {"x": 0, "y": 0, "z": 0}, "provenance": PROV}, {"id": "b", "position": {"x": 1, "y": 0, "z": 0}, "provenance": PROV}, {"id": "c", "position": {"x": 2, "y": 0, "z": 0}, "provenance": PROV}],
        "pipe_segments": [
            {"id": "pipe:p1", "from": "a", "to": "b", "section": section, "material": "m", "y_reference": {"x": 0, "y": 1, "z": 0}, "provenance": PROV},
            {"id": "body:v", "from": "b", "to": "c", "section": section, "material": "m", "y_reference": {"x": 0, "y": 1, "z": 0}, "provenance": PROV}],
        "materials": [{"id": "m", "elastic_modulus": {"value": 200e9, "unit": "Pa"}, "shear_modulus": {"value": 80e9, "unit": "Pa"}, "provenance": PROV}],
        "supports": [{"id": "s", "node": "a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}],
        "components": [{"id": VALVE, "kind": "valve", "node": "c",
            "geometry": {"rigid_pipe_ref": "body:v", "weight": {"value": 500, "unit": "N"}, "center_of_gravity": {"x": -0.5, "y": 0.25, "z": 0, "unit": "m"}},
            "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROV}],
        "combinations": [], "load_cases": []})
}

fn request(pipes: &[&str]) -> SelfWeightRequest {
    SelfWeightRequest {
        case_id: CASE.into(),
        label: "invented self-weight".into(),
        pipe_refs: pipes.iter().map(|p| p.to_string()).collect(),
        gravity: SelfWeightGravity { value: -9.80665, unit: "m/s^2".into(), axis: "global_y".into() },
        provenance: PROV.into(),
        source_model_hash: HASH.into(),
    }
}

fn typed(model: &Value) -> PreviewModel {
    serde_json::from_value(model.clone()).unwrap()
}

fn plan(model: &Value, pipes: &[&str]) -> SelfWeightOperationPlan {
    generate_self_weight_operations(&typed(model), &request(pipes)).unwrap_or_else(|d| panic!("{d:?}"))
}

fn loads(plan: &SelfWeightOperationPlan) -> Vec<Value> {
    plan.changes[1..].iter().map(|c| serde_json::from_str(&c.after).unwrap()).collect()
}

fn ids(plan: &SelfWeightOperationPlan) -> Vec<String> {
    loads(plan).iter().map(|l| l["id"].as_str().unwrap().to_string()).collect()
}

fn pipe_id(pipe: &str) -> String {
    format!("load:self-weight:{}:{}:{}:{}", CASE.len(), CASE, pipe.len(), pipe)
}

/// The model with the plan applied as drafted.
fn applied(model: &Value, pipes: &[&str]) -> Value {
    let p = plan(model, pipes);
    let mut case: Value = serde_json::from_str(&p.changes[0].after).unwrap();
    case["primitive_loads"] = json!(loads(&p));
    let mut m = model.clone();
    m["load_cases"] = json!([case]);
    m
}

fn states(model: &Value) -> Vec<(String, AppliedSelfWeightState)> {
    let m = typed(model);
    inspect_applied_self_weight(&m, None)
        .into_iter()
        .map(|s| (m.load_cases[0].primitive_loads[s.primitive_index].id.clone(), s.state))
        .collect()
}

fn state_of(model: &Value, id: &str) -> AppliedSelfWeightState {
    states(model).into_iter().find(|(i, _)| i == id).unwrap_or_else(|| panic!("no state for {id}")).1
}

fn force_id() -> String {
    component_load_id(CASE, VALVE, "force", "y")
}

fn moment_id(axis: &str) -> String {
    component_load_id(CASE, VALVE, "moment", axis)
}

#[test]
fn component_weight_replaces_the_mapped_member_and_keeps_the_others() {
    let m = model();
    let p = plan(&m, &["pipe:p1", "body:v"]);
    assert_eq!(p.scope_label, COMPONENT_WEIGHT_SCOPE);
    assert!(p.warnings.is_empty());
    // r = (-0.5, 0.25, 0), F = (0, -500, 0): M = (-r_z f, 0, r_x f) = (0, 0, 250).
    assert_eq!(ids(&p), vec![pipe_id("pipe:p1"), force_id(), moment_id("z")]);
    let l = loads(&p);
    assert_eq!(l[0]["target"], json!({"type": "element", "pipe": "pipe:p1"}));
    assert_eq!((l[1]["category"].as_str(), l[1]["direction"].as_str(), l[1]["magnitude"]["value"].as_f64()),
        (Some("weight"), Some("global_y"), Some(-500.0)));
    assert_eq!((l[2]["category"].as_str(), l[2]["direction"].as_str(), l[2]["magnitude"]["value"].as_f64()),
        (Some("weight"), Some("rotation_z"), Some(250.0)));
    assert!(l[1..].iter().all(|l| l["target"] == json!({"type": "node", "node": "c"})));
    assert_eq!(p.source_evidence[0]["method"], GENERATED_METHOD);
    for e in &p.source_evidence[1..] {
        assert_eq!(e["method"], COMPONENT_WEIGHT_METHOD);
        assert_eq!(e["component_id"], VALVE);
        assert_eq!(e["omitted_pipe_id"], "body:v");
    }
    // The other member's distributed load is the one a pipe-only plan gives.
    let alone = plan(&m, &["pipe:p1"]);
    assert_eq!(alone.scope_label, PIPE_MASS_SCOPE);
    assert_eq!(loads(&alone)[0], l[0]);
    // The case's draft evidence carries the scope and every row.
    let note: Value = serde_json::from_str(&p.changes[0].source_note).unwrap();
    assert_eq!(note["scope_label"], COMPONENT_WEIGHT_SCOPE);
    assert_eq!(note["source_evidence"].as_array().unwrap().len(), 3);
}

#[test]
fn the_body_needs_no_section_mass_input_and_no_stiffness_scale() {
    let mut m = model();
    m["pipe_segments"][1]["section"].as_object_mut().unwrap().remove("material_density");
    assert_eq!(ids(&plan(&m, &["body:v"])), vec![force_id(), moment_id("z")]);
}

#[test]
fn each_failed_weight_condition_warns_and_keeps_the_pipe_weight() {
    let cases: Vec<(&str, Box<dyn Fn(&mut Value)>)> = vec![
        ("no mechanics_interface.solver_consumption", Box::new(|c: &mut Value| { c.as_object_mut().unwrap().remove("mechanics_interface"); })),
        ("not mechanics_geometry_only", Box::new(|c: &mut Value| c["mechanics_interface"]["solver_consumption"] = json!("mechanics_geometry_and_user_flexibility"))),
        ("does not end at its node", Box::new(|c: &mut Value| c["node"] = json!("a"))),
        ("geometry.weight", Box::new(|c: &mut Value| { c["geometry"].as_object_mut().unwrap().remove("weight"); })),
        ("geometry.weight", Box::new(|c: &mut Value| c["geometry"]["weight"]["value"] = json!(0))),
        ("geometry.weight", Box::new(|c: &mut Value| c["geometry"]["weight"]["value"] = json!(-500))),
        ("geometry.weight", Box::new(|c: &mut Value| c["geometry"]["weight"]["unit"] = json!("m"))),
        ("geometry.weight", Box::new(|c: &mut Value| c["geometry"]["weight"]["unit"] = json!("kg"))),
        ("geometry.center_of_gravity", Box::new(|c: &mut Value| { c["geometry"].as_object_mut().unwrap().remove("center_of_gravity"); })),
        ("geometry.center_of_gravity", Box::new(|c: &mut Value| c["geometry"]["center_of_gravity"]["unit"] = json!("N"))),
        ("geometry.center_of_gravity", Box::new(|c: &mut Value| c["geometry"]["center_of_gravity"]["unit"] = json!("TBD"))),
    ];
    for (reason, edit) in cases {
        let mut m = model();
        edit(&mut m["components"][0]);
        let p = plan(&m, &["body:v"]);
        assert_eq!(ids(&p), vec![pipe_id("body:v")], "{reason}");
        assert_eq!(p.scope_label, PIPE_MASS_SCOPE, "{reason}");
        assert_eq!(p.warnings.len(), 1, "{reason}");
        let w = &p.warnings[0];
        assert_eq!((w.code.as_str(), w.severity.as_str()), (COMPONENT_WEIGHT_NOT_APPLIED, "warning"), "{reason}");
        assert!(w.message.contains(VALVE) && w.message.contains(reason) && w.message.contains("generated in its place"), "{reason}: {}", w.message);
        assert_eq!(w.affected_refs, vec![VALVE.to_string(), "body:v".to_string()]);
    }
    // A non-finite weight or centre of gravity cannot be authored in JSON; the
    // typed view's own non-finite values warn the same way.
    for field in ["weight", "center_of_gravity"] {
        let mut t = typed(&model());
        let g = t.components[0].geometry.as_mut().unwrap();
        if field == "weight" {
            g.weight.as_mut().unwrap().value = f64::NAN;
        } else {
            g.center_of_gravity.as_mut().unwrap().y = f64::INFINITY;
        }
        let p = generate_self_weight_operations(&t, &request(&["body:v"])).unwrap();
        assert_eq!(p.warnings.len(), 1, "{field}");
        assert!(p.warnings[0].message.contains(&format!("geometry.{field}")), "{field}");
    }
}

#[test]
fn not_solver_consumed_is_honoured_without_a_warning() {
    let mut m = model();
    m["components"][0]["mechanics_interface"]["solver_consumption"] = json!("not_solver_consumed");
    let p = plan(&m, &["body:v"]);
    assert_eq!(ids(&p), vec![pipe_id("body:v")]);
    assert!(p.warnings.is_empty());
}

#[test]
fn an_unselected_member_brings_no_component_weight() {
    let p = plan(&model(), &["pipe:p1"]);
    assert_eq!(ids(&p), vec![pipe_id("pipe:p1")]);
    assert!(p.warnings.is_empty());
}

#[test]
fn a_mapping_conflict_on_a_selected_member_blocks() {
    let mut m = model();
    let mut second = m["components"][0].clone();
    second["id"] = json!("component:flange");
    second["kind"] = json!("flange");
    second["mechanics_interface"]["solver_consumption"] = json!("not_solver_consumed");
    m["components"].as_array_mut().unwrap().push(second);
    let blocked = generate_self_weight_operations(&typed(&m), &request(&["body:v"])).unwrap_err();
    assert!(blocked.iter().any(|d| d.code == "RIGID_COMPONENT_MAPPING_CONFLICT" && d.severity == "blocking"), "{blocked:?}");
    // The conflict does not involve an unselected member's generation.
    assert_eq!(ids(&plan(&m, &["pipe:p1"])), vec![pipe_id("pipe:p1")]);
}

#[test]
fn identities_are_deterministic_and_length_prefixed() {
    assert_eq!(force_id(), "load:self-weight:6:case:w:component:15:component:valve:force:y");
    let m = model();
    assert_eq!(plan(&m, &["body:v", "pipe:p1"]).changes.iter().map(|c| c.after.clone()).collect::<Vec<_>>(),
        plan(&m, &["pipe:p1", "body:v"]).changes.iter().map(|c| c.after.clone()).collect::<Vec<_>>());
    // A colliding existing primitive identity blocks.
    let mut collide = m.clone();
    collide["load_cases"] = json!([{"id": "other", "primitive_loads": [{"id": force_id(), "category": "weight",
        "target": {"type": "node", "node": "c"}, "direction": "global_y", "magnitude": {"value": 1, "unit": "N"}, "dimension": "force"}]}]);
    let blocked = generate_self_weight_operations(&typed(&collide), &request(&["body:v"])).unwrap_err();
    assert!(blocked.iter().any(|d| d.message.contains("identity collision")), "{blocked:?}");
}

#[test]
fn applied_component_weight_is_fresh_and_solves() {
    let m = applied(&model(), &["pipe:p1", "body:v"]);
    assert!(states(&m).iter().all(|(_, s)| *s == AppliedSelfWeightState::Fresh), "{:?}", states(&m));
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let out = run_linear_static_preview_value_with_mode(json!({"model": m, "materials": []}), mode).unwrap();
        assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{:?}", out.diagnostics);
        assert!(!out.diagnostics.iter().any(|d| d.code.starts_with("SELF_WEIGHT")));
        // The nodal weight primitives are weight, not a mapped preview
        // category; only the member's distributed_force is mapped.
        let mapped: Vec<_> = out.diagnostics.iter().filter(|d| d.code == "LOAD_CATEGORY_PREVIEW_MAPPED").collect();
        assert!(mapped.iter().all(|d| d.affected_refs[0] == pipe_id("pipe:p1")), "{mapped:?}");
    }
}

#[test]
fn equivalent_units_and_unconsumed_edits_stay_fresh() {
    let mut m = applied(&model(), &["body:v"]);
    m["components"][0]["geometry"]["weight"] = json!({"value": 0.5, "unit": "kN"});
    m["components"][0]["geometry"]["center_of_gravity"] = json!({"x": -500, "y": 250, "z": 0, "unit": "mm"});
    m["components"][0]["label"] = json!("renamed");
    m["components"][0]["geometry"]["rigid_body_length"] = json!({"value": 1, "unit": "m"});
    m["components"][0]["modifiers"] = json!({"stiffness_scaling_user_value": {"value": 4, "unit": "1"}, "source_reference": "invented"});
    assert!(states(&m).iter().all(|(_, s)| *s == AppliedSelfWeightState::Fresh), "{:?}", states(&m));
}

#[test]
fn a_weight_edit_is_stale_and_refreshes_in_place() {
    let mut m = applied(&model(), &["body:v"]);
    m["components"][0]["geometry"]["weight"]["value"] = json!(800);
    assert_eq!(state_of(&m, &force_id()), AppliedSelfWeightState::Stale);
    assert_eq!(state_of(&m, &moment_id("z")), AppliedSelfWeightState::Stale);
    let t = typed(&m);
    let checked = inspect_applied_self_weight(&t, Some(HASH));
    assert!(checked.iter().all(|s| s.diagnostics.iter().any(|d| d.code == "SELF_WEIGHT_INPUTS_STALE" && d.severity == "blocking")));
    let blocked = run_linear_static_preview_value_with_mode(json!({"model": m, "materials": []}), PreviewSolverMode::SparseInteractive).unwrap();
    assert!(blocked.diagnostics.iter().any(|d| d.code == "SELF_WEIGHT_INPUTS_STALE"));
    for status in checked {
        let replacement = status.replacement.expect("an in-place refresh");
        let load = &mut m["load_cases"][0]["primitive_loads"][status.primitive_index];
        assert_eq!(replacement["id"], load["id"]);
        assert_eq!(replacement["target"], load["target"]);
        *load = replacement;
    }
    let l = &m["load_cases"][0]["primitive_loads"];
    assert_eq!((l[0]["magnitude"]["value"].as_f64(), l[1]["magnitude"]["value"].as_f64()), (Some(-800.0), Some(400.0)));
    assert!(states(&m).iter().all(|(_, s)| *s == AppliedSelfWeightState::Fresh));
}

#[test]
fn a_center_of_gravity_edit_is_stale_and_a_changed_moment_set_needs_regeneration() {
    let mut m = applied(&model(), &["body:v"]);
    m["components"][0]["geometry"]["center_of_gravity"]["x"] = json!(-0.75);
    assert_eq!(state_of(&m, &moment_id("z")), AppliedSelfWeightState::Stale);
    assert!(inspect_applied_self_weight(&typed(&m), Some(HASH)).iter().any(|s| s.replacement.is_some()));
    // A z offset adds an x moment: the applied set no longer matches, and an
    // x offset of zero removes the z moment: no single replacement exists.
    let mut m = applied(&model(), &["body:v"]);
    m["components"][0]["geometry"]["center_of_gravity"]["x"] = json!(0);
    let checked = inspect_applied_self_weight(&typed(&m), Some(HASH));
    let moment = checked.iter().find(|s| s.primitive_index == 1).unwrap();
    assert_eq!(moment.state, AppliedSelfWeightState::Stale);
    assert!(moment.replacement.is_none());
    assert!(moment.diagnostics[0].message.contains("generate this self-weight case again"));
    // The force's value is unchanged, but its component's wrench set changed:
    // it is stale with no in-place refresh either.
    let force = checked.iter().find(|s| s.primitive_index == 0).unwrap();
    assert_eq!(force.state, AppliedSelfWeightState::Stale);
    assert!(force.replacement.is_none());
}

/// Review reproduction 1: W = 600 N, gravity -Y, r = (0, 0.125, 0) m gives a
/// force only; r_x = -0.75 m then requires M_z = r_x f = 450 N*m. The force
/// must not refresh in place and leave the moment out.
#[test]
fn a_newly_required_moment_makes_every_primitive_stale_without_a_replacement() {
    let mut m = model();
    m["components"][0]["geometry"]["weight"] = json!({"value": 600, "unit": "N"});
    m["components"][0]["geometry"]["center_of_gravity"] = json!({"x": 0, "y": 0.125, "z": 0, "unit": "m"});
    let mut m = applied(&m, &["body:v"]);
    assert_eq!(states(&m), vec![(force_id(), AppliedSelfWeightState::Fresh)]);
    m["components"][0]["geometry"]["center_of_gravity"]["x"] = json!(-0.75);
    let checked = inspect_applied_self_weight(&typed(&m), Some(HASH));
    assert_eq!(checked.len(), 1);
    assert_eq!(checked[0].state, AppliedSelfWeightState::Stale);
    assert!(checked[0].replacement.is_none());
    assert!(checked[0].diagnostics[0].message.contains("generate this self-weight case again"));
    let blocked = run_linear_static_preview_value_with_mode(json!({"model": m, "materials": []}), PreviewSolverMode::SparseInteractive).unwrap();
    assert!(blocked.diagnostics.iter().any(|d| d.code == "SELF_WEIGHT_INPUTS_STALE"));
    // The regenerated case carries the moment, and its support moment is the
    // corrected one: M_z(A) = -(M_z + L F_y) = -(450 + 2 * -600) = 750 N*m.
    m["load_cases"] = json!([]);
    let regenerated = applied(&m, &["body:v"]);
    assert_eq!(ids(&plan(&m, &["body:v"])), vec![force_id(), moment_id("z")]);
    let out = run_linear_static_preview_value_with_mode(json!({"model": regenerated, "materials": []}), PreviewSolverMode::SparseInteractive).unwrap();
    let mz = out.results.iter().find(|r| r.kind == "support_reaction_component_v2" && r.entity_ref == "s"
        && r.metadata.as_ref().is_some_and(|m| m.component == "Mz")).unwrap().value;
    assert!((mz - 750.0).abs() <= 1e-9 * 750.0, "{mz}");
}

#[test]
fn mapping_and_consumption_edits_make_both_sides_stale() {
    // The component loads become stale when the component no longer carries
    // the member's weight ...
    for edit in [
        json!({"solver_consumption": "not_solver_consumed"}),
        json!({"solver_consumption": "mechanics_geometry_and_user_flexibility"}),
    ] {
        let mut m = applied(&model(), &["body:v"]);
        m["components"][0]["mechanics_interface"] = edit.clone();
        assert!(states(&m).iter().all(|(_, s)| *s == AppliedSelfWeightState::Stale), "{edit}");
        assert!(inspect_applied_self_weight(&typed(&m), Some(HASH)).iter().all(|s| s.replacement.is_none()));
    }
    let mut m = applied(&model(), &["body:v"]);
    m["components"][0]["geometry"]["rigid_pipe_ref"] = json!("pipe:p1");
    assert!(states(&m).iter().all(|(_, s)| *s == AppliedSelfWeightState::Stale));
    // ... and a member's distributed self-weight becomes stale when its
    // component starts carrying the weight.
    let mut undeclared = model();
    undeclared["components"][0].as_object_mut().unwrap().remove("mechanics_interface");
    let mut m = applied(&undeclared, &["body:v"]);
    assert_eq!(state_of(&m, &pipe_id("body:v")), AppliedSelfWeightState::Fresh);
    m["components"][0]["mechanics_interface"] = json!({"solver_consumption": "mechanics_geometry_only"});
    assert_eq!(state_of(&m, &pipe_id("body:v")), AppliedSelfWeightState::Stale);
    let pipe = inspect_applied_self_weight(&typed(&m), Some(HASH)).remove(0);
    assert!(pipe.replacement.is_none() && pipe.diagnostics[0].message.contains("component weight now replaces"));
}

/// Review reproduction 2: every record and load doubled consistently except
/// the entered weight (600 N kept); then the component's weight is set to the
/// forged value. Every forged record stays invalid, never stale or fresh.
#[test]
fn a_whole_case_forgery_keeping_the_entered_weight_is_invalid() {
    let mut m = model();
    m["components"][0]["geometry"]["weight"] = json!({"value": 600, "unit": "N"});
    m["components"][0]["geometry"]["center_of_gravity"] = json!({"x": -0.75, "y": 0.125, "z": 0, "unit": "m"});
    let mut m = applied(&m, &["body:v"]);
    for load in m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap() {
        let mut e: Value = serde_json::from_str(load["provenance"].as_str().unwrap()).unwrap();
        assert_eq!(e["weight_input"], json!({"value": 600.0, "unit": "N"}));
        let doubled = load["magnitude"]["value"].as_f64().unwrap() * 2.0;
        e["weight_N"] = json!(1200.0);
        e["normalized_dependencies"]["weight"]["value"] = json!(1200.0);
        for key in ["force_N", "moment_N_m"] {
            let v: Vec<f64> = serde_json::from_value(e[key].clone()).unwrap();
            e[key] = json!(v.iter().map(|c| c * 2.0).collect::<Vec<_>>());
        }
        e["generated_payload"]["magnitude"]["value"] = json!(doubled);
        load["magnitude"]["value"] = json!(doubled);
        load["provenance"] = json!(e.to_string());
    }
    assert!(states(&m).iter().all(|(_, s)| *s == AppliedSelfWeightState::Invalid), "{:?}", states(&m));
    m["components"][0]["geometry"]["weight"]["value"] = json!(1200);
    assert!(states(&m).iter().all(|(_, s)| *s == AppliedSelfWeightState::Invalid), "{:?}", states(&m));
}

/// The recorded authored quantities are closed, as on the component: an
/// undeclared member in either is invalid lineage, never fresh (#1630).
#[test]
fn an_undeclared_member_in_the_recorded_weight_or_centre_of_gravity_is_invalid() {
    let edits: [fn(&mut Value); 3] = [
        |e| e["weight_input"]["extra"] = json!(1),
        |e| e["center_of_gravity_input"]["note"] = json!("x"),
        |e| e["center_of_gravity_input"]["w"] = json!(0),
    ];
    for edit in edits {
        let mut m = applied(&model(), &["body:v"]);
        assert!(states(&m).iter().all(|(_, s)| *s == AppliedSelfWeightState::Fresh));
        let load = &mut m["load_cases"][0]["primitive_loads"][0];
        let mut e: Value = serde_json::from_str(load["provenance"].as_str().unwrap()).unwrap();
        edit(&mut e);
        load["provenance"] = json!(e.to_string());
        assert_eq!(state_of(&m, &force_id()), AppliedSelfWeightState::Invalid, "{e}");
    }
}

/// Review reproduction 3: the mapped member is removed but the mapping kept,
/// and that member is requested. The planner refuses; it never panics.
#[test]
fn a_missing_selected_mapped_member_is_refused() {
    let mut m = model();
    m["pipe_segments"].as_array_mut().unwrap().remove(1);
    let refused = generate_self_weight_operations(&typed(&m), &request(&["body:v"])).unwrap_err();
    assert!(refused.iter().any(|d| d.code == "SELF_WEIGHT_INPUT_INVALID" && d.message.contains("resolve exactly once")), "{refused:?}");
}

/// Review reproduction 4: W = 1e300 N with r_x the smallest positive binary64
/// in mm. The conversion to m rounds r_x to zero, which would drop
/// M_z = r_x f (about -4.94e-27 N*m in exact terms); the planner refuses.
#[test]
fn a_nonzero_centre_of_gravity_component_normalizing_to_zero_is_refused() {
    let mut m = model();
    m["components"][0]["geometry"]["weight"] = json!({"value": 1e300, "unit": "N"});
    m["components"][0]["geometry"]["center_of_gravity"] = json!({"x": 5e-324, "y": 0, "z": 0, "unit": "mm"});
    let refused = generate_self_weight_operations(&typed(&m), &request(&["body:v"])).unwrap_err();
    assert!(refused.iter().any(|d| d.code == "SELF_WEIGHT_RANGE_INVALID" && d.affected_refs.contains(&VALVE.to_string())), "{refused:?}");
    // In metres the same lever survives normalization and its moment is kept.
    m["components"][0]["geometry"]["center_of_gravity"]["unit"] = json!("m");
    let p = plan(&m, &["body:v"]);
    assert_eq!(ids(&p), vec![force_id(), moment_id("z")]);
}

#[test]
fn modified_component_loads_need_explicit_preservation() {
    let mut m = applied(&model(), &["body:v"]);
    m["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(-123.0);
    assert_eq!(state_of(&m, &force_id()), AppliedSelfWeightState::Modified);
    let original: Value = serde_json::from_str(m["load_cases"][0]["primitive_loads"][0]["provenance"].as_str().unwrap()).unwrap();
    m["load_cases"][0]["primitive_loads"][0]["provenance"] = json!(json!({"method": MANUAL_OVERRIDE_METHOD,
        "original_generation_provenance": original, "decision": "preserve_modified_generated_load", "source_model_hash": HASH}).to_string());
    assert_eq!(state_of(&m, &force_id()), AppliedSelfWeightState::ManualOverride);
    m["components"][0]["geometry"]["weight"]["value"] = json!(900);
    assert_eq!(state_of(&m, &force_id()), AppliedSelfWeightState::ManualOverride);
}

#[test]
fn forged_or_unrecognized_component_lineage_is_invalid() {
    let m = applied(&model(), &["body:v"]);
    let load = &m["load_cases"][0]["primitive_loads"][0];
    let evidence: Value = serde_json::from_str(load["provenance"].as_str().unwrap()).unwrap();
    // A snapshot that authenticates itself: the SI weight, the dependency
    // weight, the recorded wrench and the payload doubled, the entered weight
    // kept. The record contradicts its own authored input.
    let mut forged = m.clone();
    let mut e = evidence.clone();
    e["weight_N"] = json!(1000.0);
    e["normalized_dependencies"]["weight"]["value"] = json!(1000.0);
    e["force_N"][1] = json!(-1000.0);
    e["moment_N_m"][2] = json!(500.0);
    e["generated_payload"]["magnitude"]["value"] = json!(-1000.0);
    forged["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(-1000.0);
    forged["load_cases"][0]["primitive_loads"][0]["provenance"] = json!(e.to_string());
    assert_eq!(state_of(&forged, &force_id()), AppliedSelfWeightState::Invalid);
    // A recorded wrench that disagrees with its own inputs is invalid too.
    let mut wrench = m.clone();
    let mut e = evidence.clone();
    e["moment_N_m"][2] = json!(999.0);
    wrench["load_cases"][0]["primitive_loads"][0]["provenance"] = json!(e.to_string());
    assert_eq!(state_of(&wrench, &force_id()), AppliedSelfWeightState::Invalid);
    // A node load in the generated namespace without lineage is not ignored.
    let mut bare = m.clone();
    bare["load_cases"][0]["primitive_loads"][0]["provenance"] = json!("human note");
    assert_eq!(state_of(&bare, &force_id()), AppliedSelfWeightState::Invalid);
    // An ordinary manual node load is not lineage.
    let mut manual = m.clone();
    manual["load_cases"][0]["primitive_loads"][0]["id"] = json!("load:manual");
    manual["load_cases"][0]["primitive_loads"][0]["provenance"] = json!("human note");
    assert!(states(&manual).iter().all(|(id, _)| id != "load:manual"));
    // A near-prefix of the method family claims lineage.
    let mut near = m.clone();
    let mut e = evidence;
    e["method"] = json!("component_weight_at_center_of_gravity_global_offset/v9");
    near["load_cases"][0]["primitive_loads"][0]["provenance"] = json!(e.to_string());
    near["load_cases"][0]["primitive_loads"][0]["id"] = json!("load:manual-2");
    assert_eq!(state_of(&near, "load:manual-2"), AppliedSelfWeightState::Invalid);
    // Deleting the omitted member makes the lineage invalid, not silently fresh.
    let mut gone = m;
    gone["pipe_segments"].as_array_mut().unwrap().remove(1);
    gone["components"][0]["geometry"]["rigid_pipe_ref"] = json!("pipe:p1");
    assert_eq!(state_of(&gone, &force_id()), AppliedSelfWeightState::Invalid);
}
