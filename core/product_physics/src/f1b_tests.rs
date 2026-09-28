//! F1b (T3 D1 revision 5a.2 §4.8, W3, and ROOT's F1b rulings of 2026-09-28):
//! unit tests of the facade's sparse wiring and the dense-scrutiny resource
//! guard. All inputs are invented (no library, catalogue or code-rule data):
//! committed invented fixtures, and models built here.
use super::*;
use open_pipe_stress_frame_kernel::structural::exact_boundary as exact;
use open_pipe_stress_frame_kernel::ReducedAssembledSystem;
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
const PROV: &str = "invented_t3_f1b_test_input_no_library_data";

// ------------------------------------------------------------------ models

/// A model document as a request, or a request unchanged.
fn as_request(value: Value) -> Value {
    if value.get("model").is_some_and(Value::is_object) {
        value
    } else {
        json!({"model": value, "materials": []})
    }
}

/// A straight frame model: nodes (id, position), members (id, from, to),
/// supports as JSON, and nodal loads (id, node, DOF, value in N or N*m).
fn frame_request(
    id: &str,
    nodes: &[(&str, [f64; 3])],
    members: &[(&str, &str, &str)],
    supports: Vec<Value>,
    loads: &[(&str, &str, &str, f64)],
) -> Value {
    let loads: Vec<Value> = loads
        .iter()
        .map(|&(load, node, dof, value)| {
            if dof.starts_with('R') {
                json!({"id": load, "category": "concentrated_moment", "target": {"type": "node", "node": node},
                       "direction": dof, "magnitude": {"value": value, "unit": "N*m"}, "dimension": "moment",
                       "provenance": PROV})
            } else {
                json!({"id": load, "category": "concentrated_force", "target": {"type": "node", "node": node},
                       "direction": format!("global_{}", dof[1..].to_lowercase()),
                       "magnitude": {"value": value, "unit": "N"}, "dimension": "force", "provenance": PROV})
            }
        })
        .collect();
    json!({"model": {
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": format!("invented:t3-f1b:{id}"),
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "Pa"}},
        "nodes": nodes.iter().map(|(n, p)| json!({"id": n, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": PROV})).collect::<Vec<_>>(),
        "pipe_segments": members.iter().map(|(m, a, b)| json!({"id": m, "from": a, "to": b, "material": "mat:F1B",
            "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
            "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}},
            "provenance": PROV})).collect::<Vec<_>>(),
        "materials": [{"id": "mat:F1B", "elastic_modulus": {"value": 2.0e11, "unit": "Pa"},
                       "shear_modulus": {"value": 8.0e10, "unit": "Pa"}, "provenance": PROV}],
        "supports": supports,
        "load_cases": [{"id": "case", "label": id, "kind": "primitive_user_load",
                        "primitive_loads": loads, "provenance": PROV}],
        "combinations": []
    }, "materials": []})
}

fn anchor(node: &str) -> Value {
    json!({"id": format!("anchor:{node}"), "node": node, "family": "anchor",
           "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV})
}

/// A chain of `members` members along x (1 m each, with a small y and z
/// offset so that every member is skew), anchored at N0, with a tip load.
fn chain_request(members: usize) -> Value {
    let names: Vec<String> = (0..=members).map(|i| format!("N{i}")).collect();
    let nodes: Vec<(&str, [f64; 3])> = names
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let x = i as f64;
            (n.as_str(), [x, 0.125 * x, -0.0625 * x])
        })
        .collect();
    let member_names: Vec<String> = (1..=members).map(|i| format!("M{i}")).collect();
    let pairs: Vec<(&str, &str, &str)> = member_names
        .iter()
        .enumerate()
        .map(|(i, m)| (m.as_str(), names[i].as_str(), names[i + 1].as_str()))
        .collect();
    let tip = names[members].as_str();
    frame_request(
        &format!("chain-{members}"),
        &nodes,
        &pairs,
        vec![anchor("N0")],
        &[
            ("load:tip-y", tip, "UY", 1250.0),
            ("load:tip-rx", tip, "RX", 40.0),
        ],
    )
}

/// K-D5's PP-route elbow (`tests/formation_check_runtime.rs`,
/// `pp_route_elbow_request`, PP_UTM_5E5): one realized curved bend.
fn curved_elbow_request() -> Value {
    let p = PROV;
    let spring = |dof: &str| {
        json!({"id": format!("spring:N0:{dof}"), "node": "N0", "family": "spring", "restraints": [dof],
               "stiffness": {"dof": dof, "value": {"value": 1.0e6, "unit": "N*m/rad"}}, "provenance": p})
    };
    let moment = |dof: &str| {
        json!({"id": format!("load:{dof}"), "category": "concentrated_moment", "target": {"type": "node", "node": "N1"},
               "direction": dof, "magnitude": {"value": 1.0, "unit": "N*m"}, "dimension": "moment", "provenance": p})
    };
    json!({"model": {
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": "invented:t3-f1b:elbow",
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "Pa"}},
        "nodes": [
            {"id": "N0", "position": {"x": 500000.0, "y": 350000.0, "z": 0.0}, "provenance": p},
            {"id": "N1", "position": {"x": 500000.010469849, "y": 350000.0001827519, "z": 0.0}, "provenance": p}],
        "pipe_segments": [{"id": "M1", "from": "N0", "to": "N1", "material": "mat:N",
                           "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
                           "section": {"outside_diameter": {"value": 0.2, "unit": "m"},
                                       "wall_thickness": {"value": 0.01, "unit": "m"}},
                           "provenance": p}],
        "materials": [{"id": "mat:N", "elastic_modulus": {"value": 2.0e11, "unit": "Pa"},
                       "shear_modulus": {"value": 8.0e10, "unit": "Pa"}, "provenance": p}],
        "components": [{"id": "component:bend", "label": "F1b elbow", "kind": "bend", "node": "N1",
                        "geometry": {"bend_pipe_ref": "M1", "bend_radius": {"value": 0.3, "unit": "m"},
                                     "bend_plane_orientation": "global_xy_preview",
                                     "bend_geometry_source_reference": "invented"},
                        "modifiers": {"flexibility_factor_user_value": {"value": 1.0, "unit": "none"},
                                      "source_reference": "invented"},
                        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element",
                                                "rule_check_consumption": "user_rule_pack_inputs_only"},
                        "provenance": p}],
        "supports": [
            {"id": "rigid:N0", "node": "N0", "restraints": ["UX", "UY", "UZ"], "provenance": p},
            spring("RX"), spring("RY"), spring("RZ")],
        "load_cases": [{"id": "case", "label": "elbow", "kind": "primitive_user_load",
                        "primitive_loads": [moment("RX"), moment("RY"), moment("RZ")], "provenance": p}],
        "combinations": []
    }, "materials": []})
}

/// The declared B1–B3 subset (ROOT's brief, test B1): realized curved bends,
/// user elements, several modulus bases, 0.4.0 prescribed motion, springs.
fn declared_subset() -> Vec<(&'static str, Value)> {
    let parse = |text: &str| as_request(serde_json::from_str(text).unwrap());
    vec![
        ("k-d5 curved elbow", curved_elbow_request()),
        (
            "invented preview model (curved bends, user element, nonlinear supports)",
            parse(include_str!(
                "../../../fixtures/product_preview/invented_preview_model.json"
            )),
        ),
        (
            "preview-physics invented model",
            parse(include_str!(
                "../tests/fixtures/preview_physics_invented_model.json"
            )),
        ),
        (
            "dec092 temperature bases (three modulus bases)",
            parse(include_str!(
                "../../../fixtures/product_preview/invented_dec092_temperature_g_request.json"
            )),
        ),
        (
            "0.4.0 prescribed support motion",
            parse(include_str!(
                "../../../fixtures/product_preview/load_reference_source/eigen_motion.request.json"
            )),
        ),
        (
            "0.4.0 connected",
            parse(include_str!(
                "../../../fixtures/product_preview/load_reference/connected.request.json"
            )),
        ),
        ("skew chain", chain_request(7)),
    ]
}

/// One basis the product assembles for a request, formed by the product's own
/// front end in `run_linear_static_preview_captured_once`'s order, with each
/// case's ledger force (nodal and 0.4.0 eigen terms) and prescribed values.
struct Basis {
    label: String,
    built: BuiltModel,
    springs: Vec<SpringEntry>,
    restrained: Vec<usize>,
    cases: Vec<(String, AssembledForce, Option<Vec<f64>>)>,
}

fn product_bases(request: &Value) -> Vec<Basis> {
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let mut model = parsed.model;
    let mut materials = if parsed.materials.is_empty() {
        model.materials.clone()
    } else {
        parsed.materials
    };
    let mut d = Vec::new();
    resolve_shared_sections(&mut model, &mut d);
    normalize_model_units(&mut model, &mut materials, &mut d);
    let load_state = case_state::is_load_state(&model);
    if !load_state {
        pressure_material::resolve_base(&model, &mut materials, &mut d);
    }
    let mut bases: Vec<Basis> = Vec::new();
    fn push(
        label: String,
        built: BuiltModel,
        case: (String, AssembledForce, Option<Vec<f64>>),
        bases: &mut Vec<Basis>,
    ) {
        if let Some(basis) = bases.iter_mut().find(|b| b.label == label) {
            basis.cases.push(case);
            return;
        }
        let boundary = prepare_boundary(built.nodes.len(), &built.supports);
        bases.push(Basis {
            label,
            springs: boundary.springs,
            restrained: boundary.restrained_dofs,
            built,
            cases: vec![case],
        });
    }
    for case in model.load_cases.clone() {
        if load_state {
            let resolved =
                case_state::resolve::resolve_case(&model, &materials, &case, &mut d).unwrap();
            let built =
                build_model_for_members(&model, &materials, Some(&resolved.pairs), &mut d).unwrap();
            let primitives =
                build_load_case_primitive_loads(&model, &resolved.effective_case, &mut d);
            let loads = prepare_loads(built.nodes.len(), built.pipes.len(), &primitives);
            let eigen = load_state_eigen_loads(&resolved, &built).unwrap();
            let force = nodal_and_eigen_case_force(&loads, &eigen, &built).unwrap();
            let restrained = prepare_boundary(built.nodes.len(), &built.supports).restrained_dofs;
            let prescribed = restrained
                .iter()
                .map(|dof| resolved.prescribed.get(dof).copied().unwrap_or(0.0))
                .collect();
            push(
                format!("load_state:{}", case.id),
                built,
                (case.id.clone(), force, Some(prescribed)),
                &mut bases,
            );
        } else {
            let key = modulus_basis_key(&case, &mut d);
            let (label, basis_materials) = match &key {
                Some(key) => (
                    key.clone(),
                    materials_for_modulus_basis(&model, &materials, &case, &mut d)
                        .unwrap()
                        .0,
                ),
                None => ("base".to_string(), materials.clone()),
            };
            let built = build_model(&model, &basis_materials, &mut d).unwrap();
            let primitives = build_load_case_primitive_loads(&model, &case, &mut d);
            let loads = prepare_loads(built.nodes.len(), built.pipes.len(), &primitives);
            let force = nodal_and_eigen_case_force(&loads, &[], &built).unwrap();
            push(label, built, (case.id.clone(), force, None), &mut bases);
        }
    }
    bases
}

/// Main's E12 (PP `restrained_reactions` and `multiply_matrix_vector` at
/// `e7d930d49`), verbatim, on the dense matrix.
fn main_dense_reactions(
    stiffness: &[Vec<f64>],
    displacements: &[f64],
    force: &AssembledForce,
) -> Vec<f64> {
    let product: Vec<f64> = stiffness
        .iter()
        .map(|row| row.iter().zip(displacements).map(|(a, b)| a * b).sum())
        .collect();
    product
        .into_iter()
        .enumerate()
        .map(|(dof, internal)| {
            let mut accumulator = ExactAccumulator::new();
            let summed = accumulator
                .add(internal)
                .and_then(|()| force.accumulate_dof(dof, &mut accumulator, true))
                .and_then(|()| accumulator.round());
            summed.unwrap_or(f64::NAN)
        })
        .collect()
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|v| v.to_bits()).collect()
}

/// A displacement vector with zeros of both signs, mixed signs and the
/// prescribed boundary values in place.
fn probe_displacements(n: usize, restrained: &[usize], prescribed: Option<&[f64]>) -> Vec<f64> {
    let mut u: Vec<f64> = (0..n)
        .map(|i| match i % 5 {
            0 => 0.0,
            1 => -0.0,
            2 => (i as f64 + 1.0) * 1.0e-3,
            3 => -(i as f64) * 3.7e-4,
            _ => 1.25e-2 / (i as f64 + 1.0),
        })
        .collect();
    for (k, &dof) in restrained.iter().enumerate() {
        u[dof] = prescribed.map_or(0.0, |values| values[k]);
    }
    u
}

/// Main's linear attempt's evidence (PP `solve_preview_reduced_system` at
/// `e7d930d49`): the dense `AssemblyEvidence` on the dense matrix.
fn main_dense_attempt(
    built: &BuiltModel,
    springs: &[SpringEntry],
    dense: &[Vec<f64>],
    force: &AssembledForce,
    prescribed: &[(usize, f64)],
    mode: PreviewSolverMode,
) -> String {
    let (curved, springs, free, curved_sources) = attempt_inputs(built, springs, force, prescribed);
    let solved = AssemblyEvidence::new(
        built.nodes.len(),
        &built.frame_elements,
        &built.user_stiffness_elements,
        &curved,
        &springs,
    )
    .and_then(|assembly| {
        assembly.solve_assembled_with_formation_check(
            dense,
            force,
            &free,
            prescribed,
            linear_mode(mode),
            &curved_sources,
            built.nonlinear_supports.is_empty(),
        )
    });
    format!("{solved:?}")
}

/// F1b's linear attempt's evidence: the pattern evidence on the pattern.
fn pattern_attempt(
    built: &BuiltModel,
    springs: &[SpringEntry],
    stiffness: &SparseStiffness,
    force: &AssembledForce,
    prescribed: &[(usize, f64)],
    mode: PreviewSolverMode,
) -> String {
    let (curved, springs, free, curved_sources) = attempt_inputs(built, springs, force, prescribed);
    let solved = SparseAssemblyEvidence::new(
        stiffness.pattern(),
        built.nodes.len(),
        &built.frame_elements,
        &built.user_stiffness_elements,
        &curved,
        &springs,
    )
    .and_then(|assembly| {
        assembly.solve_assembled_with_formation_check(
            stiffness,
            force,
            &free,
            prescribed,
            linear_mode(mode),
            &curved_sources,
            built.nonlinear_supports.is_empty(),
        )
    });
    format!("{solved:?}")
}

type AttemptInputs = (
    Vec<CurvedBendStiffnessElement>,
    Vec<(usize, f64)>,
    Vec<usize>,
    Vec<CurvedBendMacroElement>,
);

fn attempt_inputs(
    built: &BuiltModel,
    springs: &[SpringEntry],
    force: &AssembledForce,
    prescribed: &[(usize, f64)],
) -> AttemptInputs {
    let curved = built
        .curved_bend_elements
        .iter()
        .map(|e| {
            CurvedBendStiffnessElement::from_macro_element(e.component_id.clone(), &e.macro_element)
                .unwrap()
        })
        .collect();
    let springs = springs
        .iter()
        .map(|e| (e.node_dof.global_index(), e.stiffness.value))
        .collect();
    let free = (0..force.len())
        .filter(|i| !prescribed.iter().any(|&(dof, _)| dof == *i))
        .collect();
    let curved_sources = built
        .curved_bend_elements
        .iter()
        .map(|e| e.macro_element)
        .collect();
    (curved, springs, free, curved_sources)
}

fn linear_mode(mode: PreviewSolverMode) -> LinearSolveMode {
    match mode {
        PreviewSolverMode::DenseScrutiny => LinearSolveMode::DenseScrutiny,
        PreviewSolverMode::SparseInteractive => LinearSolveMode::SparseInteractive,
    }
}

/// B1–B3 and the attempt's evidence for every basis and case of `request`.
/// Returns (bases, cases) checked.
fn assert_sparse_wiring_parity(name: &str, request: &Value) -> (usize, usize) {
    let bases = product_bases(request);
    assert!(!bases.is_empty(), "{name}: no basis");
    let mut cases = 0;
    for basis in &bases {
        let ctx = format!("{name} [{}]", basis.label);
        let sparse = assemble_basis_stiffness(&basis.built, &basis.springs).unwrap();
        let dense = assemble_case_stiffness(&basis.built, &basis.springs).unwrap();
        let n = dense.len();
        // B1: every entry, bit for bit (a stored value, or the dense +0.0).
        assert_eq!(sparse.dimension(), n, "{ctx}");
        for (i, row) in dense.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                assert_eq!(
                    sparse.get(i, j).to_bits(),
                    value.to_bits(),
                    "{ctx}: K[{i}][{j}]"
                );
            }
        }
        let view = sparse.to_dense();
        for (a, b) in view.iter().zip(&dense) {
            assert_eq!(bits(a), bits(b), "{ctx}: dense view");
        }
        // The guard's input is the assembly's dense entry count (OQ12).
        assert_eq!(
            sparse.storage_counts().dense_entries,
            (n as u128) * (n as u128),
            "{ctx}"
        );
        assert_eq!(basis.built.nodes.len() * DOF_PER_NODE, n, "{ctx}");
        for (case, force, prescribed) in &basis.cases {
            let ctx = format!("{ctx} case {case}");
            // B2: the partition and the reduced right-hand side (KS2).
            let pattern_reduced = reduce_assembled_sparse_system(
                &sparse,
                force,
                &basis.restrained,
                prescribed.as_deref(),
            )
            .unwrap();
            let dense_reduced: ReducedAssembledSystem = match prescribed {
                Some(values) => reduce_assembled_system_with_prescribed_displacements(
                    &dense,
                    force,
                    &basis.restrained,
                    values,
                ),
                None => reduce_assembled_system(&dense, force, &basis.restrained),
            }
            .unwrap();
            assert_eq!(
                pattern_reduced.free_dofs, dense_reduced.free_dofs,
                "{ctx}: free DOFs"
            );
            assert_eq!(
                bits(pattern_reduced.force.values()),
                bits(dense_reduced.force.values()),
                "{ctx}: reduced force"
            );
            // B3: E12 from sparse rows, sign of zero included.
            let u = probe_displacements(n, &basis.restrained, prescribed.as_deref());
            assert_eq!(
                bits(&restrained_reactions(&sparse, &u, force)),
                bits(&main_dense_reactions(&dense, &u, force)),
                "{ctx}: reactions"
            );
            // The observation lane reads the same K entries.
            let prescribed_pairs: Vec<(usize, f64)> = basis
                .restrained
                .iter()
                .enumerate()
                .map(|(k, &dof)| (dof, prescribed.as_ref().map_or(0.0, |v| v[k])))
                .collect();
            let observed = legacy_observation_force(
                force,
                &sparse,
                &basis.restrained,
                &prescribed_pairs,
                prescribed.is_some(),
            );
            let mut main_observed = force.values().to_vec();
            if prescribed.is_some() {
                for (row, value) in main_observed.iter_mut().enumerate() {
                    if basis.restrained.contains(&row) {
                        continue;
                    }
                    for &(column, displacement) in &prescribed_pairs {
                        *value -= dense[row][column] * displacement;
                    }
                }
            }
            assert_eq!(
                bits(&observed),
                bits(&main_observed),
                "{ctx}: observation force"
            );
            // The attempt: the pattern evidence's `Debug` equals the dense
            // evidence's, in both modes (Q8(d)); the solved u gives the same
            // E12 bits too.
            for mode in MODES {
                let pattern = pattern_attempt(
                    &basis.built,
                    &basis.springs,
                    &sparse,
                    force,
                    &prescribed_pairs,
                    mode,
                );
                let main = main_dense_attempt(
                    &basis.built,
                    &basis.springs,
                    &dense,
                    force,
                    &prescribed_pairs,
                    mode,
                );
                assert_eq!(pattern, main, "{ctx} {mode:?}: attempt");
            }
            cases += 1;
        }
    }
    (bases.len(), cases)
}

// ------------------------------------------------------------------ B1–B3

/// B1–B3 on the declared subset (the corpus-wide check is a scratch probe,
/// recorded in F1b's run records).
#[test]
fn f1b_sparse_wiring_is_bit_identical_to_the_dense_assembly_on_the_declared_subset() {
    let mut total = (0, 0);
    let mut curved = 0;
    let mut users = 0;
    let mut prescribed_motion = 0;
    for (name, request) in declared_subset() {
        let bases = product_bases(&request);
        curved += bases
            .iter()
            .map(|b| b.built.curved_bend_elements.len())
            .sum::<usize>();
        users += bases
            .iter()
            .map(|b| b.built.user_stiffness_elements.len())
            .sum::<usize>();
        prescribed_motion += bases
            .iter()
            .flat_map(|b| &b.cases)
            .filter(|(_, _, p)| p.as_ref().is_some_and(|v| v.iter().any(|g| *g != 0.0)))
            .count();
        let (b, c) = assert_sparse_wiring_parity(name, &request);
        total.0 += b;
        total.1 += c;
    }
    // The subset covers what B1 declares.
    assert!(curved > 0, "realized curved bends");
    assert!(users > 0, "user stiffness elements");
    assert!(prescribed_motion > 0, "nonzero 0.4.0 prescribed motion");
    assert!(total.0 >= 9 && total.1 >= total.0, "{total:?}");
}

/// B5 (mutation 11's control): one invocation with several modulus bases
/// gives each basis its own values.
#[test]
fn f1b_each_modulus_basis_has_its_own_sparse_values() {
    let request = as_request(
        serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/invented_dec092_temperature_g_request.json"
        ))
        .unwrap(),
    );
    let bases = product_bases(&request);
    assert_eq!(
        bases.len(),
        3,
        "base, an exact point and an interpolated temperature"
    );
    let values: Vec<Vec<u64>> = bases
        .iter()
        .map(|b| {
            bits(
                assemble_basis_stiffness(&b.built, &b.springs)
                    .unwrap()
                    .values(),
            )
        })
        .collect();
    for i in 0..values.len() {
        for j in 0..i {
            assert_ne!(values[i], values[j], "bases {i} and {j} share values");
        }
    }
    // Each basis's pattern is the same model's pattern.
    let patterns: Vec<_> = bases
        .iter()
        .map(|b| {
            assemble_basis_stiffness(&b.built, &b.springs)
                .unwrap()
                .pattern()
                .clone()
        })
        .collect();
    assert!(patterns.windows(2).all(|w| w[0] == w[1]));
}

// ------------------------------------------------------------------ B6

/// The displacement component rows of a published envelope, by (node, row
/// kind).
fn displacement_components(envelope: &MechanicsEnvelope) -> BTreeMap<(String, String), f64> {
    envelope
        .results
        .iter()
        .filter(|r| r.kind.starts_with("global_nodal_"))
        .map(|r| ((r.entity_ref.clone(), r.kind.clone()), r.value))
        .collect()
}

/// B6: relabelling and permutation of nodes and members give the same
/// answers, within the unchanged 1e-9 criterion. (Byte identity with Mac main
/// for each numbering is the scratch T9 extension in F1b's run records.)
#[test]
fn f1b_relabelling_and_permutation_give_the_same_answers() {
    let base = chain_request(6);
    let reversed = {
        let mut r = base.clone();
        let nodes = r["model"]["nodes"].as_array_mut().unwrap();
        nodes.reverse();
        let members = r["model"]["pipe_segments"].as_array_mut().unwrap();
        members.reverse();
        r
    };
    let permuted = {
        let mut r = base.clone();
        let nodes = r["model"]["nodes"].as_array_mut().unwrap();
        let order = [3, 0, 5, 1, 6, 2, 4];
        let original = nodes.clone();
        for (slot, &from) in nodes.iter_mut().zip(&order) {
            *slot = original[from].clone();
        }
        let members = r["model"]["pipe_segments"].as_array_mut().unwrap();
        members.rotate_left(2);
        r
    };
    for mode in MODES {
        let reference = displacement_components(
            &run_linear_static_preview_value_with_mode(base.clone(), mode).unwrap(),
        );
        assert_eq!(reference.len(), 7 * 6, "{mode:?}");
        for (label, variant) in [("reversed", &reversed), ("permuted", &permuted)] {
            for envelope in [
                run_linear_static_preview_value_with_mode(variant.clone(), mode).unwrap(),
                run_linear_static_preview_with_mode(
                    serde_json::from_value(variant.clone()).unwrap(),
                    mode,
                ),
            ] {
                let got = displacement_components(&envelope);
                assert_eq!(got.len(), reference.len(), "{label} {mode:?}");
                let scale = reference.values().fold(0.0_f64, |m, v| m.max(v.abs()));
                for (key, expected) in &reference {
                    let observed = got[key];
                    assert!(
                        (observed - expected).abs() <= 1.0e-9 * expected.abs().max(scale),
                        "{label} {mode:?} {key:?}: {observed} against {expected}"
                    );
                }
            }
        }
    }
}

// ------------------------------------------------------------------ C

/// C-DECISION: the guard's decision just below, at and just above the
/// provisional ceiling (96 bytes x 2^26 entries = 6 GiB exactly), and without
/// overflow at 10,000 members.
#[test]
fn f1b_guard_decision_below_at_and_above_the_ceiling() {
    assert_eq!(DENSE_SCRUTINY_BYTES_PER_ENTRY, 96);
    assert_eq!(DENSE_SCRUTINY_CEILING_BYTES, 6_442_450_944);
    assert_eq!(
        dense_scrutiny_estimate_bytes(1 << 26),
        DENSE_SCRUTINY_CEILING_BYTES
    );
    // By dimension: 8,192 DOFs is exactly 2^26 entries (allowed); 8,193 is not.
    assert_eq!(
        dense_scrutiny_guard(8_192, DENSE_SCRUTINY_CEILING_BYTES),
        Ok(())
    );
    assert_eq!(
        dense_scrutiny_guard(8_191, DENSE_SCRUTINY_CEILING_BYTES),
        Ok(())
    );
    assert_eq!(
        dense_scrutiny_guard(8_193, DENSE_SCRUTINY_CEILING_BYTES),
        Err(DenseScrutinyRefusal {
            dimension: 8_193,
            dense_entries: 67_125_249,
            estimated_bytes: 6_444_023_904,
            ceiling_bytes: 6_442_450_944,
        })
    );
    // By entry count: 2^26 - 1 and 2^26 pass; 2^26 + 1 (estimate
    // 6,442,451,040) is refused. The ceiling is the refusal's bound.
    for (entries, refused) in [
        ((1u128 << 26) - 1, false),
        (1 << 26, false),
        ((1 << 26) + 1, true),
    ] {
        let estimate = dense_scrutiny_estimate_bytes(entries);
        assert_eq!(
            estimate > DENSE_SCRUTINY_CEILING_BYTES,
            refused,
            "{entries}"
        );
    }
    assert_eq!(dense_scrutiny_estimate_bytes((1 << 26) + 1), 6_442_451_040);
    // A 1,000-member chain (6,006 DOFs) runs; a 1,365-member one (8,196) and
    // a 10,000-member one (60,006) are refused, without overflow.
    assert_eq!(
        dense_scrutiny_guard(6_006, DENSE_SCRUTINY_CEILING_BYTES),
        Ok(())
    );
    assert_eq!(dense_scrutiny_estimate_bytes(6_006 * 6_006), 3_462_915_456);
    assert_eq!(
        dense_scrutiny_guard(8_196, DENSE_SCRUTINY_CEILING_BYTES)
            .unwrap_err()
            .estimated_bytes,
        6_448_743_936
    );
    assert_eq!(
        dense_scrutiny_guard(60_006, DENSE_SCRUTINY_CEILING_BYTES)
            .unwrap_err()
            .estimated_bytes,
        345_669_123_456
    );
    // The product's ceiling is the constant (no override outside the hook).
    assert_eq!(dense_scrutiny_ceiling_bytes(), DENSE_SCRUTINY_CEILING_BYTES);
}

/// Runs `f` with the dense-scrutiny ceiling lowered on this thread.
fn with_ceiling<T>(ceiling: u128, f: impl FnOnce() -> T) -> T {
    DENSE_SCRUTINY_CEILING_OVERRIDE.with(|cell| cell.set(Some(ceiling)));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    DENSE_SCRUTINY_CEILING_OVERRIDE.with(|cell| cell.set(None));
    result.unwrap_or_else(|panic| std::panic::resume_unwind(panic))
}

/// C-LOWERED: with the ceiling lowered through the test hook, a 2-node model
/// (12 DOFs, 144 dense entries, estimate 13,824 bytes) is refused in dense
/// scrutiny on both entries, with the estimate named and no mechanics row;
/// at a ceiling equal to the estimate it solves; sparse mode never uses it.
#[test]
fn f1b_lowered_ceiling_refuses_dense_scrutiny_before_any_mechanics() {
    let request = frame_request(
        "guard",
        &[("N0", [0.0, 0.0, 0.0]), ("N1", [2.0, 0.0, 0.0])],
        &[("M1", "N0", "N1")],
        vec![anchor("N0")],
        &[("load:y", "N1", "UY", 100.0)],
    );
    let entries = |ceiling: u128, mode: PreviewSolverMode| {
        with_ceiling(ceiling, || {
            [
                run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap(),
                run_linear_static_preview_with_mode(
                    serde_json::from_value(request.clone()).unwrap(),
                    mode,
                ),
            ]
        })
    };
    for envelope in entries(96 * 143, PreviewSolverMode::DenseScrutiny) {
        assert!(envelope.results.is_empty(), "no mechanics row");
        assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE");
        let guard: Vec<_> = envelope
            .diagnostics
            .iter()
            .filter(|d| d.id == "diagnostic:physics:dense-scrutiny-resource-guard")
            .collect();
        assert_eq!(guard.len(), 1, "{:?}", envelope.diagnostics);
        assert_eq!(guard[0].code, "SOLVER_SYSTEM_BLOCKED");
        assert_eq!(guard[0].severity, "blocking");
        assert_eq!(
            guard[0].message,
            "dense scrutiny resource guard: estimated dense-path peak 13824 bytes (96 bytes x 144 dense entries, 12 global DOFs squared) exceeds the provisional ceiling 13728 bytes; the model is refused before any n^2 allocation. The estimate is a stated formula, not a measurement; sparse_interactive does not use it, and no automatic dense fallback exists"
        );
        assert_eq!(guard[0].affected_refs, vec!["model".to_string()]);
        assert!(envelope
            .diagnostics
            .iter()
            .all(|d| d.code != "NUMERICAL_INTEGRITY_CHECKS_PASSED"));
    }
    for envelope in entries(96 * 144, PreviewSolverMode::DenseScrutiny)
        .into_iter()
        .chain(entries(96 * 143, PreviewSolverMode::SparseInteractive))
    {
        assert_eq!(
            envelope.status.mechanics, "MECHANICS_SOLVED",
            "{:?}",
            envelope.diagnostics
        );
        assert!(envelope
            .diagnostics
            .iter()
            .all(|d| d.id != "diagnostic:physics:dense-scrutiny-resource-guard"));
    }
}

/// C-NOFALLBACK: a sparse integrity refusal is never rescued by the dense
/// path, and no envelope publishes mode code 3 or a dense fallback.
#[test]
fn f1b_no_automatic_dense_fallback() {
    // A floating body (N2-N3 has no ground): a witnessed mechanism.
    let mechanism = frame_request(
        "mechanism",
        &[
            ("N0", [0.0, 0.0, 0.0]),
            ("N1", [1.0, 0.0, 0.0]),
            ("N2", [0.0, 2.0, 0.0]),
            ("N3", [1.0, 2.0, 0.0]),
        ],
        &[("M1", "N0", "N1"), ("M2", "N2", "N3")],
        vec![anchor("N0")],
        &[("load:y", "N3", "UY", 10.0)],
    );
    for mode in MODES {
        let envelope = run_linear_static_preview_value_with_mode(mechanism.clone(), mode).unwrap();
        assert!(envelope.results.is_empty(), "{mode:?}");
        assert!(
            envelope
                .diagnostics
                .iter()
                .any(|d| d.code == "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM"
                    && d.severity == "blocking"),
            "{mode:?}: {:?}",
            envelope.diagnostics
        );
    }
    let mut requests: Vec<Value> = declared_subset().into_iter().map(|(_, r)| r).collect();
    requests.push(mechanism);
    for request in requests {
        for mode in MODES {
            let envelope =
                run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
            for row in envelope
                .results
                .iter()
                .filter(|r| r.kind == "linear_solver_mode_basis")
            {
                assert_eq!(row.value, mode.mode_code(), "{mode:?}");
                let basis = &row.metadata.as_ref().unwrap().basis;
                assert!(
                    basis.contains("dense_fallback=false; dense_fallback_message=none"),
                    "{basis}"
                );
            }
        }
    }
}

// ------------------------------------------------------------------ Q9

/// Q9(a): retained-source recovery refuses n > 256 by budget before any read
/// of `Input::stiffness`, so the product's empty dense view gives exactly the
/// refusal a full dense matrix gives (the same stage, error and work).
#[test]
fn f1b_source_recovery_budget_refusal_precedes_every_stiffness_read() {
    // 43 nodes: 258 DOFs, above the 256 limit.
    let request = chain_request(42);
    let bases = product_bases(&request);
    let basis = &bases[0];
    let n = basis.built.nodes.len() * DOF_PER_NODE;
    assert!(n > source_recovery::DENSE_SOURCE_DOF_LIMIT, "{n}");
    assert_eq!(source_recovery::DENSE_SOURCE_DOF_LIMIT, 256);
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let model = parsed.model;
    let (_, force, _) = &basis.cases[0];
    let free: Vec<usize> = (0..n).filter(|d| !basis.restrained.contains(d)).collect();
    let prescribed: Vec<(usize, f64)> = basis.restrained.iter().map(|&d| (d, 0.0)).collect();
    let mut d = Vec::new();
    let primitives = build_load_case_primitive_loads(&model, &model.load_cases[0], &mut d);
    let loads = prepare_loads(
        basis.built.nodes.len(),
        basis.built.pipes.len(),
        &primitives,
    );
    let dense = assemble_case_stiffness(&basis.built, &basis.springs).unwrap();
    let empty: Vec<Vec<f64>> = Vec::new();
    let attempt = |stiffness: &[Vec<f64>]| {
        source_recovery::solve(
            source_recovery::Input {
                model: &model,
                built: &basis.built,
                stiffness,
                force,
                free: &free,
                prescribed: &prescribed,
                spring_entries: &basis.springs,
                load_case: &model.load_cases[0],
                load_application: &loads,
                thermal_loads: &[],
                pressure_thrust_loads: &[],
                load_state: None,
            },
            exact::Limits::default(),
        )
        .map(|_| ())
        .unwrap_err()
    };
    let with_view = attempt(&dense);
    let without = attempt(&empty);
    assert_eq!(
        with_view.error,
        source_recovery::RecoveryError::Exact(exact::Error::Budget)
    );
    assert_eq!(
        (with_view.stage, &with_view.error, with_view.work),
        (without.stage, &without.error, without.work)
    );
}
