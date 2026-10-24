//! W-15 slice a (DEL-03-05 Design `rigid-component-stiffness-scaling.md` §2):
//! focused checks of the stiffness scale at member formation, both thermal
//! producers, the PF Poisson ratio, the eligibility predicate, the mapping
//! conflicts and the diagnostics table. Every input is invented; the closed
//! forms below use no product code. The rigid-link benchmark with its
//! independent exact-rational generator is §6's separate reference.
use super::*;
use serde_json::{json, Value};

const PROVENANCE: &str = "invented W-15 rigid-scaling test input; not library, catalog or code data";
const E: f64 = 200.0e9;
const G: f64 = 80.0e9;
const ALPHA: f64 = 1.2e-5;
const OD: f64 = 0.12;
const WALL: f64 = 0.01;
const PIPE: &str = "pipe:W15-PIPE";
const BODY: &str = "pipe:W15-BODY";
const VALVE: &str = "component:W15-VALVE";
const NODE_A: &str = "node:W15-A";
const NODE_B: &str = "node:W15-B";
const NODE_C: &str = "node:W15-C";
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

/// Annulus area of the invented section, independent of the product.
fn area() -> f64 {
    let inner = OD - 2.0 * WALL;
    std::f64::consts::PI / 4.0 * (OD * OD - inner * inner)
}

fn close(actual: f64, expected: f64, label: &str) {
    let scale = expected.abs().max(1e-300);
    assert!((actual - expected).abs() <= 1e-9 * scale, "{label}: {actual} vs {expected}");
}

/// A complete, eligible valve body on `BODY` at node B with scale `s`.
fn valve(s: f64) -> Value {
    json!({
        "id": VALVE, "kind": "valve", "node": NODE_B,
        "geometry": {
            "rigid_pipe_ref": BODY, "rigid_body_length": q(2.0, "m"),
            "end_a_size": q(OD, "m"), "end_b_size": q(OD, "m"), "weight": q(100.0, "N"),
            "center_of_gravity": {"x": 0.0, "y": 0.0, "z": 0.0, "unit": "m"},
            "connection_end_a_reference": "invented end a", "connection_end_b_reference": "invented end b",
            "stiffness_behavior_reference": "invented behaviour", "rigid_component_source_reference": "invented source"
        },
        "modifiers": {"stiffness_scaling_user_value": q(s, "none"), "source_reference": "invented scale source"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"},
        "provenance": PROVENANCE
    })
}

fn pipe(id: &str, from: &str, to: &str) -> Value {
    json!({"id": id, "from": from, "to": to,
        "section": {"outside_diameter": q(OD, "m"), "wall_thickness": q(WALL, "m")},
        "material": "material:w15", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROVENANCE})
}

fn anchor(node: &str) -> Value {
    json!({"id": format!("support:{node}"), "node": node, "family": "anchor",
        "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE})
}

fn force_x(node: &str, value: f64) -> Value {
    json!({"id": format!("load:fx:{node}"), "category": "concentrated_force", "target": {"type": "node", "node": node},
        "direction": "global_x", "magnitude": q(value, "N"), "dimension": "force", "provenance": PROVENANCE})
}

fn thermal(pipe: &str, delta: f64) -> Value {
    json!({"id": format!("load:dt:{pipe}"), "category": "thermal", "target": {"type": "element", "pipe": pipe},
        "direction": "global_x", "magnitude": q(delta, "degC"), "dimension": "temperature_change", "provenance": PROVENANCE})
}

/// The invented line A(0) - B(1 m) - C(3 m) on x: `PIPE` A-B and `BODY` B-C,
/// both of the same section and material, with the given anchors.
fn line(anchors: &[&str], components: Vec<Value>, loads: Vec<Value>) -> Value {
    json!({
        "model": {
            "schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
            "project": {"id": "project:w15", "units": {"length": "m"}},
            "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
            "nodes": [
                {"id": NODE_A, "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROVENANCE},
                {"id": NODE_B, "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": PROVENANCE},
                {"id": NODE_C, "position": {"x": 3.0, "y": 0.0, "z": 0.0}, "provenance": PROVENANCE}
            ],
            "pipe_segments": [pipe(PIPE, NODE_A, NODE_B), pipe(BODY, NODE_B, NODE_C)],
            "supports": anchors.iter().map(|node| anchor(node)).collect::<Vec<_>>(),
            "components": components,
            "load_cases": [{"id": "load:W15", "primitive_loads": loads, "provenance": PROVENANCE}],
            "combinations": []
        },
        "materials": [{"id": "material:w15", "elastic_modulus": q(E, "Pa"), "shear_modulus": q(G, "Pa"),
            "thermal_expansion_coefficient": q(ALPHA, "1/degC"), "provenance": PROVENANCE}]
    })
}

fn typed(request: &Value) -> LinearStaticPreviewRequest {
    serde_json::from_value(request.clone()).expect("the invented request parses")
}

/// The request normalized as the route does before building.
fn normalized(request: &Value) -> (PreviewModel, Vec<MaterialInput>) {
    let typed = typed(request);
    let (mut model, mut materials) = (typed.model, typed.materials);
    let mut diagnostics = Vec::new();
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    (model, materials)
}

fn built(request: &Value) -> BuiltModel {
    let (model, materials) = normalized(request);
    let mut diagnostics = Vec::new();
    let built = build_model(&model, &materials, &mut diagnostics);
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    built.expect("builds")
}

fn validation_diagnostics(request: &Value) -> Vec<Diagnostic> {
    let typed = typed(request);
    let mut diagnostics = Vec::new();
    validation::validate_model_inputs(&typed.model, &typed.materials, &mut diagnostics);
    diagnostics
}

fn rigid_codes(diagnostics: &[Diagnostic]) -> Vec<String> {
    diagnostics.iter().filter(|d| d.code.starts_with("RIGID_COMPONENT_")).map(|d| d.code.clone()).collect()
}

/// The member's axial force N, tension positive: the negated local end-i
/// force (the row's sign convention is the element end action).
fn axial(envelope: &MechanicsEnvelope, pipe: &str) -> f64 {
    -envelope
        .results
        .iter()
        .find(|row| row.kind == "element_local_axial_force" && row.entity_ref == pipe
            && row.metadata.as_ref().is_some_and(|m| m.location == "end_i"))
        .unwrap_or_else(|| panic!("axial force of {pipe}"))
        .value
}

fn solved(request: &Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let envelope = run_linear_static_preview_with_mode(typed(request), mode);
    assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED", "{:#?}", envelope.diagnostics);
    envelope
}

/// §2.2: an eligible body is formed with the products s*E and s*G, each
/// rounded once; every other member keeps its material E and G bit for bit,
/// and the body records the selected unscaled pair.
#[test]
fn w15_eligible_member_is_formed_with_the_scaled_moduli() {
    for s in [0.25, 1.0, 15.0, 1.0e6] {
        let built = built(&line(&[NODE_A, NODE_C], vec![valve(s)], vec![force_x(NODE_B, 3000.0)]));
        let member = |id: &str| built.pipes.iter().find(|p| p.element_id == id).unwrap();
        assert_eq!(member(BODY).section.elastic_modulus.to_bits(), (s * E).to_bits(), "s = {s}");
        assert_eq!(member(BODY).section.shear_modulus.to_bits(), (s * G).to_bits(), "s = {s}");
        assert_eq!(member(PIPE).section.elastic_modulus.to_bits(), E.to_bits());
        assert_eq!(member(PIPE).section.shear_modulus.to_bits(), G.to_bits());
        let frame = built.frame_elements.iter().find(|f| f.node_i.index == 1 && f.node_j.index == 2).unwrap();
        assert_eq!(frame.section.elastic_modulus.to_bits(), (s * E).to_bits(), "the assembled frame sees s*K");
        assert_eq!(
            built.scaled_members.get(BODY),
            Some(&ScaledMember { component_id: VALVE.to_string(), scale: s, elastic_modulus: E, shear_modulus: G })
        );
        assert!(!built.scaled_members.contains_key(PIPE));
        assert_eq!(built.unscaled_moduli(member(BODY)), (E, G));
        // The geometric section of stress recovery is the mapped pipe's.
        assert_eq!(built.sections[BODY].area.to_bits(), built.sections[PIPE].area.to_bits());
    }
}

/// §2.4's indeterminate pair (equal E*A, body twice the pipe's length, a
/// load at the joint, both far ends anchored): at s = 1 the pipe carries
/// 2F/3; at s = 4 the body does. s = 1 is plain pipe, bit for bit.
#[test]
fn w15_indeterminate_pair_redistributes_onto_the_stiffer_body() {
    let force = 3000.0;
    let plain = line(&[NODE_A, NODE_C], vec![], vec![force_x(NODE_B, force)]);
    for mode in MODES {
        let reference = solved(&plain, mode);
        let unit = solved(&line(&[NODE_A, NODE_C], vec![valve(1.0)], vec![force_x(NODE_B, force)]), mode);
        let kinds = |e: &MechanicsEnvelope| {
            e.results.iter().filter(|r| r.kind.starts_with("element_local_") || r.kind.starts_with("global_nodal_"))
                .map(|r| (r.id.clone(), r.value.to_bits())).collect::<Vec<_>>()
        };
        assert_eq!(kinds(&unit), kinds(&reference), "{mode:?}: s = 1 is plain pipe");
        // The pipe A-B is stretched, the body B-C compressed.
        close(axial(&unit, PIPE), 2.0 * force / 3.0, "s = 1 pipe");
        close(axial(&unit, BODY), -force / 3.0, "s = 1 body");
        let four = solved(&line(&[NODE_A, NODE_C], vec![valve(4.0)], vec![force_x(NODE_B, force)]), mode);
        close(axial(&four, PIPE), force / 3.0, "s = 4 pipe");
        close(axial(&four, BODY), -2.0 * force / 3.0, "s = 4 body");
        // Equilibrium at the joint is unchanged.
        close(axial(&four, PIPE) - axial(&four, BODY), force, "joint equilibrium");
        assert!(four.diagnostics.iter().any(|d| d.code == "RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED"));
    }
}

/// §2.2, §6: the element thermal load reads the built member's section E.
/// A series pipe and body anchored at both ends under one temperature change:
/// N = -sum(eps_i L_i) / sum(L_i / (s_i E A)), tension positive; and the
/// ledger's axial load is (s*E)*A*eps, the FreeStrainStraight frame's.
#[test]
fn w15_element_thermal_load_reads_the_section_modulus() {
    let delta = 40.0;
    let eps = ALPHA * delta;
    for s in [0.25, 1.0, 4.0, 1.0e3] {
        let request = line(&[NODE_A, NODE_C], vec![valve(s)], vec![thermal(PIPE, delta), thermal(BODY, delta)]);
        let (model, materials) = normalized(&request);
        let mut diagnostics = Vec::new();
        let built = build_model(&model, &materials, &mut diagnostics).unwrap();
        let material_map = materials.iter().map(|m| (m.id.as_str(), m)).collect::<HashMap<_, _>>();
        let pipe_map = model.pipe_segments.iter().enumerate().map(|(i, p)| (p.id.as_str(), i)).collect::<HashMap<_, _>>();
        let loads = build_thermal_element_loads(&model, &model.load_cases[0], &material_map, &pipe_map, &built, &mut diagnostics);
        assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
        let section_area = built.sections[BODY].area;
        let body = loads.iter().find(|l| l.element_index == 1).unwrap();
        let eps_product = ALPHA * delta;
        assert_eq!(body.axial_load.to_bits(), ((s * E) * section_area * eps_product).to_bits(), "s = {s}");
        let plain = loads.iter().find(|l| l.element_index == 0).unwrap();
        assert_eq!(plain.axial_load.to_bits(), (E * built.sections[PIPE].area * eps_product).to_bits());
        let expected = -(eps * 1.0 + eps * 2.0) / (1.0 / (E * area()) + 2.0 / (s * E * area()));
        for mode in MODES {
            let envelope = solved(&request, mode);
            close(axial(&envelope, PIPE), expected, &format!("s = {s} {mode:?} pipe"));
            close(axial(&envelope, BODY), expected, &format!("s = {s} {mode:?} body"));
        }
    }
}

/// §2.1: a free scaled body still grows alpha*dT*L under temperature,
/// whatever its scale, so its free end moves alpha*dT*L and it carries no
/// force.
#[test]
fn w15_free_scaled_body_grows_freely() {
    let delta = 25.0;
    for s in [0.25, 15.0, 1.0e6] {
        // Anchored at B only: the body B-C is free at C; the pipe A-B is free at A.
        let request = line(&[NODE_B], vec![valve(s)], vec![thermal(BODY, delta)]);
        for mode in MODES {
            let envelope = solved(&request, mode);
            let tip = envelope.results.iter()
                .find(|r| r.kind == "global_nodal_displacement_x" && r.entity_ref == NODE_C).unwrap();
            let metres = match tip.unit.as_str() { "mm" => tip.value / 1000.0, "m" => tip.value, other => panic!("{other}") };
            close(metres, ALPHA * delta * 2.0, &format!("s = {s} {mode:?} free growth"));
            assert!(axial(&envelope, BODY).abs() <= 1e-6 * s * E * area() * ALPHA * delta, "s = {s}: no force");
        }
    }
}

/// §2.2: the PF Poisson ratio of a body is that of the selected unscaled E/G
/// pair (base material and a named point), never of its scaled section;
/// the model-bound reader recomputes it from the model and checks the shear
/// rows bit for bit.
#[test]
fn w15_preview_poisson_ratio_uses_the_unscaled_pair() {
    // A scale whose scaled section gives a different nu in binary64 than
    // the unscaled pair, so the check discriminates.
    let named = (190.0e9, 73.0e9);
    let s = (1..2000)
        .map(|k| 1.0 + k as f64 / 7.0)
        .find(|s| [(E, G), named].iter().all(|&(e, g)| {
            u4b_rows::preview_poisson_ratio(s * e, s * g).to_bits() != u4b_rows::preview_poisson_ratio(e, g).to_bits()
        }))
        .expect("a discriminating scale");
    let mut request = line(&[NODE_A], vec![valve(s)], vec![]);
    let loads = |case: &str| vec![
        json!({"id": format!("load:fz:{case}"), "category": "concentrated_force", "target": {"type": "node", "node": NODE_C},
            "direction": "global_z", "magnitude": q(1500.0, "N"), "dimension": "force", "provenance": PROVENANCE}),
        json!({"id": format!("load:mx:{case}"), "category": "concentrated_moment", "target": {"type": "node", "node": NODE_C},
            "direction": "RX", "magnitude": q(900.0, "N*m"), "dimension": "moment", "provenance": PROVENANCE}),
    ];
    request["model"]["load_cases"] = json!([
        {"id": "load:W15-BASE", "primitive_loads": loads("base"), "provenance": PROVENANCE},
        {"id": "load:W15-NAMED", "modulus_basis_ref": "temperature-point:w15", "primitive_loads": loads("named"), "provenance": PROVENANCE},
    ]);
    request["materials"][0]["temperature_points"] = json!([{"id": "temperature-point:w15",
        "elastic_modulus": q(named.0, "Pa"), "shear_modulus": q(named.1, "Pa"),
        "thermal_expansion_coefficient": q(ALPHA, "1/degC"), "provenance": PROVENANCE}]);
    // §6: an optional authored nu, absent, consistent with E/(2G) - 1 and
    // inconsistent, on the material and the named point alike. PF never reads
    // it: nu is the selected unscaled pair's, so every variant gives the same
    // shear rows bit for bit and the model-bound reader admits each.
    let consistent = |(e, g): (f64, f64)| q(u4b_rows::preview_poisson_ratio(e, g), "1");
    let variants = [
        ("absent", None, None),
        ("consistent", Some(consistent((E, G))), Some(consistent(named))),
        ("inconsistent", Some(q(0.45, "1")), Some(q(0.05, "1"))),
    ];
    let mut reference: std::collections::HashMap<&str, Vec<u64>> = std::collections::HashMap::new();
    for (label, material_nu, point_nu) in variants {
        let mut request = request.clone();
        if let Some(nu) = material_nu {
            request["materials"][0]["poisson_ratio"] = nu;
        }
        if let Some(nu) = point_nu {
            request["materials"][0]["temperature_points"][0]["poisson_ratio"] = nu;
        }
        let mut model = request["model"].clone();
        model["materials"] = request["materials"].clone();
        for mode in MODES {
            let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
            assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED", "{label} {mode:?}: {:#?}", envelope.diagnostics);
            let shear: Vec<u64> = envelope.results.iter().filter(|r| r.kind == u4b_rows::SHEAR_MAX_KIND && r.entity_ref == BODY).map(|r| r.value.to_bits()).collect();
            assert!(shear.len() >= 2, "{label} {mode:?}: the body's shear rows on both bases");
            let first = reference.entry(mode.as_str()).or_insert_with(|| shear.clone());
            assert_eq!(&shear, first, "{label} {mode:?}: an authored nu leaves the PF shear rows unchanged");
            let verdict = crate::result_export::reader_verdict::validate("preview-physics", &serde_json::to_value(&envelope).unwrap(), Some(&model), None);
            assert_eq!(verdict["ok"], true, "{label} {mode:?}: {verdict}");
        }
    }
}

/// §2.2: the load-state member pair's eigenload (thermal and fit alike) is
/// formed with the built member's section E: s*E on the body, the resolved
/// pair's E bit for bit elsewhere.
#[test]
fn w15_load_state_eigenload_reads_the_section_modulus() {
    let mut request: Value =
        serde_json::from_str(include_str!("../../../fixtures/product_preview/pressure/load_reference/pressure.request.json")).unwrap();
    let s = 15.0;
    let model = &mut request["model"];
    model["nodes"].as_array_mut().unwrap().push(json!({"id": "node:tip", "position": {"x": 3, "y": 0, "z": 0}, "provenance": PROVENANCE}));
    let mut body = model["pipe_segments"][0].clone();
    body["id"] = json!("pipe:body");
    body["from"] = json!("node:middle");
    body["to"] = json!("node:tip");
    model["pipe_segments"].as_array_mut().unwrap().push(body);
    let mut reference = model["reference_configurations"][0]["member_references"][0].clone();
    reference["pipe_ref"] = json!("pipe:body");
    reference["fit"] = json!({"kind": "fit_strain", "strain": q(-2.0e-4, "1")});
    model["reference_configurations"][0]["member_references"].as_array_mut().unwrap().push(reference);
    for case in model["load_cases"].as_array_mut().unwrap() {
        let mut state = case["analysis_state"]["element_states"][0].clone();
        state["pipe_ref"] = json!("pipe:body");
        case["analysis_state"]["element_states"].as_array_mut().unwrap().push(state);
    }
    let mut component = valve(s);
    component["node"] = json!("node:middle");
    component["geometry"]["rigid_pipe_ref"] = json!("pipe:body");
    model["components"] = json!([component]);
    let typed = typed(&request);
    let (mut model, mut materials) = (typed.model, if typed.materials.is_empty() { Vec::new() } else { typed.materials });
    if materials.is_empty() {
        materials = model.materials.clone();
    }
    let mut diagnostics = Vec::new();
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    assert!(case_state::is_load_state(&model));
    let mut checked = 0;
    for case in model.load_cases.clone() {
        let resolved = case_state::resolve::resolve_case(&model, &materials, &case, &mut diagnostics).expect("resolves");
        let built = build_model_for_members(&model, &materials, Some(&resolved.pairs), &mut diagnostics).unwrap();
        assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
        let eigen = load_state_eigen_loads(&resolved, &built).unwrap();
        assert!(eigen.iter().any(|l| l.element_index == 1), "{}: the body's fit eigenload", case.id);
        for member in &resolved.members {
            let Some(load) = eigen.iter().find(|l| l.element_index == member.pipe_index) else {
                assert_eq!(member.strain.total_eigenstrain, 0.0, "{} {}", case.id, member.pipe_id);
                continue;
            };
            let pair_e = resolved.pairs[&member.pipe_id].elastic_modulus_pa();
            let section_e = if member.pipe_id == "pipe:body" { s * pair_e } else { pair_e };
            assert_eq!(built.pipes[member.pipe_index].section.elastic_modulus.to_bits(), section_e.to_bits());
            assert_eq!(
                load.axial_load.to_bits(),
                (section_e * built.sections[&member.pipe_id].area * member.strain.total_eigenstrain).to_bits(),
                "{} {}", case.id, member.pipe_id
            );
            checked += 1;
        }
        assert!(resolved.members.iter().any(|m| m.pipe_id == "pipe:body" && !matches!(m.fit_input, case_state::thermal::FitInput::None)));
    }
    assert!(checked >= 3, "{checked}");
}

/// §2.3, §2.6: each unmet eligibility condition leaves the member unscaled and
/// is named by the one warning of its condition, in the listed order.
#[test]
fn w15_each_unmet_condition_is_named_and_leaves_the_member_unscaled() {
    type Edit = fn(&mut Value);
    let cases: [(&str, Edit, &[&str]); 13] = [
        ("absent declaration with a scale", |c| { c.as_object_mut().unwrap().remove("mechanics_interface"); },
            &["RIGID_COMPONENT_CONSUMPTION_UNDECLARED"]),
        ("absent declaration and no scale", |c| {
            c.as_object_mut().unwrap().remove("mechanics_interface");
            c["modifiers"].as_object_mut().unwrap().remove("stiffness_scaling_user_value");
        }, &["RIGID_COMPONENT_STIFFNESS_INPUT_MISSING"]),
        ("not_solver_consumed", |c| { c["mechanics_interface"]["solver_consumption"] = json!("not_solver_consumed"); }, &[]),
        ("another declared mode", |c| { c["mechanics_interface"]["solver_consumption"] = json!("objective_connector"); }, &[]),
        ("no mapped member", |c| { c["geometry"].as_object_mut().unwrap().remove("rigid_pipe_ref"); },
            &["RIGID_COMPONENT_GEOMETRY_INPUT_MISSING", "RIGID_COMPONENT_MAPPING_INPUT_INVALID"]),
        ("unknown member", |c| { c["geometry"]["rigid_pipe_ref"] = json!("pipe:absent"); }, &["RIGID_COMPONENT_MAPPING_INPUT_INVALID"]),
        ("member not ending at the node", |c| { c["node"] = json!(NODE_A); }, &["RIGID_COMPONENT_MAPPING_INPUT_INVALID"]),
        ("mapping before the scale", |c| {
            c["node"] = json!(NODE_A);
            c["modifiers"]["stiffness_scaling_user_value"] = json!(q(-1.0, "none"));
        }, &["RIGID_COMPONENT_MAPPING_INPUT_INVALID"]),
        ("no scale", |c| { c["modifiers"].as_object_mut().unwrap().remove("stiffness_scaling_user_value"); },
            &["RIGID_COMPONENT_STIFFNESS_INPUT_MISSING"]),
        ("blank source reference", |c| { c["modifiers"]["source_reference"] = json!("  "); },
            &["RIGID_COMPONENT_STIFFNESS_INPUT_MISSING"]),
        ("zero scale", |c| { c["modifiers"]["stiffness_scaling_user_value"] = json!(q(0.0, "none")); },
            &["RIGID_COMPONENT_STIFFNESS_INPUT_INVALID"]),
        ("negative scale", |c| { c["modifiers"]["stiffness_scaling_user_value"] = json!(q(-2.0, "1")); },
            &["RIGID_COMPONENT_STIFFNESS_INPUT_INVALID"]),
        ("invalid optional stiffness", |c| { c["modifiers"]["rotational_stiffness_user_value"] = json!(q(-5.0, "N*m/rad")); },
            &["RIGID_COMPONENT_STIFFNESS_INPUT_INVALID"]),
    ];
    for (label, edit, expected) in cases {
        let mut component = valve(15.0);
        edit(&mut component);
        let request = line(&[NODE_A, NODE_C], vec![component], vec![force_x(NODE_B, 3000.0)]);
        let diagnostics = validation_diagnostics(&request);
        assert_eq!(rigid_codes(&diagnostics), expected.iter().map(|c| c.to_string()).collect::<Vec<_>>(), "{label}");
        assert!(!has_blocking(&diagnostics), "{label}: {diagnostics:?}");
        let warned = diagnostics.iter().filter(|d| d.code.starts_with("RIGID_COMPONENT_") && d.code != "RIGID_COMPONENT_GEOMETRY_INPUT_MISSING");
        for finding in warned {
            assert_eq!(finding.severity, "warning", "{label}");
            assert!(finding.message.contains("solves as pipe, unscaled"), "{label}: {}", finding.message);
        }
        let interface = diagnostics.iter().any(|d| d.code == "COMPONENT_MECHANICS_INTERFACE_UNSUPPORTED");
        assert_eq!(interface, label == "not_solver_consumed" || label == "another declared mode", "{label}");
        let built = built(&request);
        assert!(built.scaled_members.is_empty(), "{label}");
        assert!(built.pipes.iter().all(|p| p.section.elastic_modulus == E && p.section.shear_modulus == G), "{label}");
    }
    // A non-finite scale cannot be authored in JSON; the predicate refuses it.
    let typed = typed(&line(&[NODE_A, NODE_C], vec![valve(15.0)], vec![]));
    let pipes = typed.model.pipe_segments.iter().map(|p| (p.id.as_str(), p)).collect::<HashMap<_, _>>();
    for value in [f64::NAN, f64::INFINITY] {
        let mut component = typed.model.components[0].clone();
        component.modifiers.as_mut().unwrap().stiffness_scaling_user_value.as_mut().unwrap().value = value;
        assert_eq!(rigid_component::eligibility(&component, &pipes), Err(rigid_component::Unmet::ScaleInvalid));
    }
    let component = typed.model.components[0].clone();
    assert_eq!(rigid_component::eligibility(&component, &pipes), Ok((BODY, 15.0)));
    // Optional linear and rotational stiffness are not required.
    assert!(component.modifiers.as_ref().unwrap().linear_stiffness_user_value.is_none());
}

/// §2.6: the applied scale's info names the member and s, the reference
/// section and the unapplied linear/rotational stiffness; from 1e8 the
/// precision warning is added, with the numerical qualification unchanged.
#[test]
fn w15_applied_scale_info_and_precision_warning() {
    for (s, precision) in [(15.0, false), (9.99e7, false), (1.0e8, true), (1.0e10, true)] {
        let mut component = valve(s);
        component["modifiers"]["linear_stiffness_user_value"] = json!(q(1.5e7, "N/m"));
        let request = line(&[NODE_A, NODE_C], vec![component], vec![force_x(NODE_B, 3000.0)]);
        let diagnostics = validation_diagnostics(&request);
        let applied = diagnostics.iter().find(|d| d.code == "RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED").expect("applied");
        assert_eq!(applied.severity, "info");
        assert_eq!(applied.affected_refs, vec![VALVE.to_string(), BODY.to_string()]);
        assert!(applied.message.contains(&format!("scale {s} to mapped member {BODY}")), "{}", applied.message);
        assert!(applied.message.contains("reference section") && applied.message.contains("not a stress of the component body"));
        assert!(applied.message.contains("linear and rotational stiffness values, where supplied, are recorded and not applied"));
        let warning = diagnostics.iter().find(|d| d.code == "RIGID_COMPONENT_SCALE_PRECISION");
        assert_eq!(warning.is_some(), precision, "s = {s}");
        if let Some(warning) = warning {
            assert_eq!(warning.severity, "warning");
            assert!(warning.message.contains("not an accuracy bound or an engineering limit"));
        }
        assert!(!diagnostics.iter().any(|d| d.code == "RIGID_COMPONENT_STIFFNESS_SCALING_REVIEWED"));
        for mode in MODES {
            let envelope = solved(&request, mode);
            assert_eq!(envelope.diagnostics.iter().any(|d| d.code == "RIGID_COMPONENT_SCALE_PRECISION"), precision);
        }
    }
}

/// §2.5: s*E or s*G outside binary64's normal range is refused by name at
/// section formation; the member is not formed.
#[test]
fn w15_scaled_moduli_outside_the_normal_range_are_refused() {
    for s in [1.0e300, 1.0e-320] {
        let (model, materials) = normalized(&line(&[NODE_A, NODE_C], vec![valve(s)], vec![]));
        let mut diagnostics = Vec::new();
        let _ = build_model(&model, &materials, &mut diagnostics);
        let refusal = diagnostics.iter().find(|d| d.id == "diagnostic:pipe-section:pipe-W15-BODY:rigid-scale").expect("refused");
        assert_eq!((refusal.code.as_str(), refusal.severity.as_str()), ("PIPE_SECTION_INPUT_INVALID", "blocking"));
        let envelope = run_linear_static_preview_with_mode(typed(&line(&[NODE_A, NODE_C], vec![valve(s)], vec![force_x(NODE_B, 1.0)])), PreviewSolverMode::DenseScrutiny);
        assert_ne!(envelope.status.mechanics, "MECHANICS_SOLVED", "s = {s}");
        assert!(envelope.results.is_empty());
    }
}

/// §2.3: a shared member and a realized curved bend are refused by name,
/// counting every rigid-family component that maps a member, eligible or
/// not (a not_solver_consumed record sharing a member is a conflict).
#[test]
fn w15_mapping_conflicts_block_by_name() {
    let mut other = valve(4.0);
    other["id"] = json!("component:W15-FLANGE");
    other["kind"] = json!("flange");
    other["mechanics_interface"]["solver_consumption"] = json!("not_solver_consumed");
    let request = line(&[NODE_A, NODE_C], vec![valve(15.0), other], vec![force_x(NODE_B, 3000.0)]);
    let diagnostics = validation_diagnostics(&request);
    let conflict = diagnostics.iter().find(|d| d.code == "RIGID_COMPONENT_MAPPING_CONFLICT").expect("shared member");
    assert_eq!(conflict.severity, "blocking");
    assert_eq!(conflict.affected_refs, vec![BODY.to_string(), VALVE.to_string(), "component:W15-FLANGE".to_string()]);
    assert!(built(&request).scaled_members.is_empty(), "a conflicted member is never scaled");
    let envelope = run_linear_static_preview_with_mode(typed(&request), PreviewSolverMode::SparseInteractive);
    assert_ne!(envelope.status.mechanics, "MECHANICS_SOLVED");
    // A body mapped onto a realized curved bend.
    let bend = json!({"id": "component:W15-BEND", "kind": "bend", "node": NODE_B,
        "geometry": {"bend_pipe_ref": BODY, "bend_radius": q(1.5, "m"), "bend_geometry_source_reference": "invented"},
        "modifiers": {"flexibility_factor_user_value": q(1.0, "none"), "sif_user_value": q(1.0, "none"), "source_reference": "invented"},
        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"}, "provenance": PROVENANCE});
    let request = line(&[NODE_A, NODE_C], vec![bend, valve(15.0)], vec![force_x(NODE_B, 3000.0)]);
    let diagnostics = validation_diagnostics(&request);
    let conflict = diagnostics.iter().find(|d| d.code == "RIGID_COMPONENT_MAPPING_CONFLICT").expect("curved bend");
    assert_eq!(conflict.id, "diagnostic:component:component-W15-VALVE:rigid-mapping-conflict:curved-bend");
    assert_eq!(conflict.affected_refs, vec![VALVE.to_string(), BODY.to_string()]);
    assert!(conflict.message.contains("curved-bend macro-element"));
    // One component per member, as authored, is no conflict.
    let request = line(&[NODE_A, NODE_C], vec![valve(15.0)], vec![]);
    assert!(!validation_diagnostics(&request).iter().any(|d| d.code == "RIGID_COMPONENT_MAPPING_CONFLICT"));
}

/// §5: the producer's stress kinds are exactly the row kinds whose physical
/// dimension is stress in the two current tables (pressure-2 and
/// preview-physics-2), the set the readers derive from the table.
#[test]
fn stress_kinds_are_the_tables_stress_rows() {
    use crate::result_export::semantic_contract as sc;
    let mut tables = std::collections::BTreeSet::new();
    for table in [sc::pressure_rows_contract(), sc::preview_physics_contract()] {
        for row in table["rows"].as_array().expect("table rows") {
            if row["source_physical_semantic_dimension"] == "stress" {
                tables.insert(row["kind"].as_str().expect("row kind").to_string());
            }
        }
    }
    let producer: std::collections::BTreeSet<String> = rigid_component::STRESS_KINDS.iter().map(|k| k.to_string()).collect();
    assert_eq!(producer.len(), rigid_component::STRESS_KINDS.len(), "no duplicate kind");
    assert_eq!(producer, tables);
}
