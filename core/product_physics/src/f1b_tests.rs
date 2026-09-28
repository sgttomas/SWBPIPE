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

// ============================================================ A2: W2 (unit)

/// A three-node model: DOF labels N0:UX to N2:RZ (18 DOFs).
fn w2_unit_model() -> PreviewModel {
    serde_json::from_value::<LinearStaticPreviewRequest>(chain_request(2))
        .unwrap()
        .model
}

/// F1a's unit report (`f1a_tests::unit_report`), repeated here.
fn w2_unit_report(quality: SolveQuality) -> StructuralReport {
    StructuralReport {
        policy: "M03-INTEGRITY-v1",
        quality,
        factorization: "unit",
        scale_exponents: vec![],
        pivots: vec![],
        condition_estimator: "unit",
        reciprocal_condition_estimate: 0.5,
        residual_rows: vec![],
        refinement_attempts: 0,
        contribution_audit_performed: true,
        contribution_rounding: vec![],
        assembly_relative_perturbation_estimate: 0.0,
        assembly_amplification_estimate: 0.0,
        assembly_load_perturbation_estimate: 0.0,
        intended_residual_rows: vec![],
        symmetry_projection_performed: false,
        maximum_scaled_skew: 0.0,
        symmetry_basis: None,
    }
}

fn w2_estimate() -> FormationCheck {
    FormationCheck {
        reason: FormationCheckReason::Estimate,
        global_dof: Some(7),
        doubled_correction: 3e-9,
        scale: 1.0,
        ratio: 3.0,
    }
}

fn w2_load_row_finding() -> formation_guard::FormationFinding {
    formation_guard::FormationFinding {
        guard: formation_guard::Guard::LoadRow,
        sentence: "S11-G formation-noise guard: load case c is Sensitive".into(),
        fired: vec![],
    }
}

fn w2_recovery_finding() -> formation_guard::FormationFinding {
    formation_guard::FormationFinding {
        guard: formation_guard::Guard::Recovery,
        sentence: "S11-G recovery guard (R-b'): load case c is Sensitive".into(),
        fired: vec![],
    }
}

fn normal(value: f64) -> PublishedValue {
    PublishedValue {
        value,
        representability: Representability::Normal,
    }
}

fn subnormal(value: f64, relative_precision: f64) -> PublishedValue {
    PublishedValue {
        value,
        representability: Representability::Subnormal { relative_precision },
    }
}

/// A publication at b with every value normal: N0's six reactions, one spring
/// at N1:UY and one member.
fn plain_publication(b: i32) -> ForceScaledPublication {
    ForceScaledPublication {
        force_scale_exponent: b,
        reactions: (0..DOF_PER_NODE)
            .map(|dof| (dof, normal(dof as f64)))
            .collect(),
        spring_actions: vec![normal(2.0)],
        end_actions: vec![[normal(1.0); ELEMENT_DOF]],
        members: vec!["M1".into()],
        spring_dofs: vec![7],
        records: vec![],
    }
}

/// Main's `append_integrity_report` at `e7d930d49` (after F1a), verbatim
/// (kept unformatted, as main has it).
#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
fn main_append_integrity_report_e7d930d49(
    diagnostics: &mut Vec<Diagnostic>,
    case_id: &str,
    report: &StructuralReport,
    model: &PreviewModel,
    equilibrium: Option<
        &open_pipe_stress_nonlinear_integration::product_equilibrium::ProductEquilibriumReport,
    >,
    formation: Option<&formation_guard::FormationFinding>,
    formation_check: Option<&FormationCheck>,
) {
    let code = if report.quality == SolveQuality::Sensitive {
        "NUMERICAL_INTEGRITY_SENSITIVE"
    } else {
        "NUMERICAL_INTEGRITY_CHECKS_PASSED"
    };
    diagnostics.push(diag(&integrity_diagnostic_id(case_id), code, if report.quality == SolveQuality::Sensitive { "warning" } else { "info" },
        format!("{} represented original-equation structural evidence for load case {}: {:?}; global_dof_map={:?}. {} No certified inertia, guaranteed forward accuracy, or pressure/component/stress engineering qualification is claimed.", report.policy, case_id, report, integrity_dof_map(model),
            equilibrium.map(|e|format!("{} final same-state evaluated equilibrium and derived residual-work evidence: {:?}; residual units are N for global DOF%6<3 and N*m otherwise; work units N*m; observed maximum only, exact represented maximum not claimed; general-energy historical alias is residual work, not total energy balance; separate zero count/cap/contact/sliding checks passed",e.policy,e)).unwrap_or_else(||"The contribution audit distinguishes intended assembly from stored equations; physical formulation limitations remain applicable.".into())),
        vec![case_id.to_string()]));
    if let (Some(finding), Some(record)) = (formation, diagnostics.last_mut()) {
        formation_guard::demote(record, finding);
    }
    if let (Some(check), Some(record)) = (formation_check, diagnostics.last_mut()) {
        record.message = format!(
            "{} {}",
            record.message,
            formation_check_evidence_line(model, check)
        );
    }
}

/// A-ORACLE: with no range record, `append_integrity_report` is byte-identical
/// to main's on every combination of quality, S11-G finding and K-D5 record.
#[test]
fn f1b_no_range_record_is_byte_identical_to_main() {
    let model = w2_unit_model();
    let findings = [
        None,
        Some(w2_load_row_finding()),
        Some(w2_recovery_finding()),
    ];
    let checks = [None, Some(w2_estimate())];
    let existing = diag("diagnostic:x", "X", "info", "x", vec![]);
    let mut compared = 0;
    for quality in [SolveQuality::Passed, SolveQuality::Sensitive] {
        for finding in &findings {
            for check in &checks {
                let report = w2_unit_report(quality);
                let mut ours = vec![existing.clone()];
                let mut mains = vec![existing.clone()];
                append_integrity_report(
                    &mut ours,
                    "c",
                    &report,
                    &model,
                    None,
                    finding.as_ref(),
                    check.as_ref(),
                    None,
                );
                main_append_integrity_report_e7d930d49(
                    &mut mains,
                    "c",
                    &report,
                    &model,
                    None,
                    finding.as_ref(),
                    check.as_ref(),
                );
                assert_eq!(
                    serde_json::to_string(&ours).unwrap(),
                    serde_json::to_string(&mains).unwrap(),
                    "{quality:?} {finding:?} {check:?}"
                );
                compared += 1;
            }
        }
    }
    assert_eq!(compared, 12);
}

/// The `range_scaling:` line (ROOT Q7, fixed at checkpoint 0): the exact text
/// with every value normal; eight synthetic outcomes in DOF order (records,
/// then reactions, then spring actions at each DOF), then member end actions,
/// with S11-G's `NAMED` (6) named and `more=2`; the `global_dof=` label
/// fallback; and six outcomes with no `more=`.
#[test]
fn f1b_range_scaling_line_exact_text_named_and_more() {
    let model = w2_unit_model();
    assert_eq!(formation_guard::NAMED, 6);
    assert_eq!(
        range_scaling_evidence_line(&model, &plain_publication(-702)),
        "range_scaling: force_scale_exponent=-702; basis=exact power-of-two"
    );
    let record = |record: &'static str, global_dof: usize, representability| RecordOutcome {
        record,
        global_dof,
        value: 0.0,
        representability,
    };
    let mut publication = plain_publication(898);
    publication.records = vec![
        record(
            "residual_rows.residual",
            7,
            RecordRepresentability::Subnormal {
                relative_precision: 0.25,
            },
        ),
        record(
            "intended_residual_rows.evaluation_allowance",
            7,
            RecordRepresentability::Underflow,
        ),
        record(
            "residual_rows.denominator",
            1,
            RecordRepresentability::Overflow,
        ),
        record(
            "residual_rows.residual",
            40,
            RecordRepresentability::Subnormal {
                relative_precision: 0.125,
            },
        ),
    ];
    publication.reactions[1].1 = subnormal(1e-320, 0.5);
    publication.spring_actions[0] = subnormal(-1e-320, 0.0625);
    publication.end_actions[0][1] = subnormal(1e-321, 2.0);
    publication.end_actions[0][11] = subnormal(-1e-322, 4.0);
    publication.end_actions.push([normal(3.0); ELEMENT_DOF]);
    publication.members.push("M2".into());
    assert_eq!(
        range_scaling_evidence_line(&model, &publication),
        "range_scaling: force_scale_exponent=898; basis=exact power-of-two; \
         record=residual_rows.denominator@N0:UY:overflow; \
         subnormal=reaction@N0:UY:relative_precision=0.5; \
         record=residual_rows.residual@N1:UY:subnormal(relative_precision=0.25); \
         record=intended_residual_rows.evaluation_allowance@N1:UY:underflow; \
         subnormal=spring_action@N1:UY:relative_precision=0.0625; \
         record=residual_rows.residual@global_dof=40:subnormal(relative_precision=0.125); \
         more=2"
    );
    // Six outcomes: every one named, and no `more=`; the end actions follow
    // every DOF-located entry, in member and local order.
    publication.records.truncate(2);
    publication.spring_actions[0] = normal(-1.0);
    assert_eq!(
        range_scaling_evidence_line(&model, &publication),
        "range_scaling: force_scale_exponent=898; basis=exact power-of-two; \
         subnormal=reaction@N0:UY:relative_precision=0.5; \
         record=residual_rows.residual@N1:UY:subnormal(relative_precision=0.25); \
         record=intended_residual_rows.evaluation_allowance@N1:UY:underflow; \
         subnormal=end_action@M1.i:Fy:relative_precision=2.0; \
         subnormal=end_action@M1.j:Mz:relative_precision=4.0"
    );
}

fn w2_record(
    quality: SolveQuality,
    finding: Option<&formation_guard::FormationFinding>,
    check: Option<&FormationCheck>,
    publication: Option<&ForceScaledPublication>,
) -> Diagnostic {
    let mut diagnostics = Vec::new();
    append_integrity_report(
        &mut diagnostics,
        "c",
        &w2_unit_report(quality),
        &w2_unit_model(),
        None,
        finding,
        check,
        publication,
    );
    assert_eq!(diagnostics.len(), 1);
    diagnostics.pop().unwrap()
}

/// The line's composition: one space after the base text, S11-G's load-row
/// sentence or F1a's `formation_check:` line, whatever the code (it is method
/// evidence, not a demotion); R-b' follows it under S11-G's no-op rule.
#[test]
fn f1b_range_scaling_line_composition_and_no_op_rule() {
    let model = w2_unit_model();
    let publication = plain_publication(734);
    let line = range_scaling_evidence_line(&model, &publication);
    let json = |d: &Diagnostic| serde_json::to_string(d).unwrap();
    for quality in [SolveQuality::Passed, SolveQuality::Sensitive] {
        let base = w2_record(quality, None, None, None);
        let with = w2_record(quality, None, None, Some(&publication));
        let mut expected = base.clone();
        expected.message = format!("{} {line}", base.message);
        assert_eq!(json(&with), json(&expected), "{quality:?}");
        assert_eq!(with.message.matches("range_scaling:").count(), 1);
    }
    let passed = w2_record(SolveQuality::Passed, None, None, None);
    assert_eq!(passed.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED");
    // After S11-G's load-row sentence (a Passed record it demotes).
    let finding = w2_load_row_finding();
    let demoted = w2_record(
        SolveQuality::Passed,
        Some(&finding),
        None,
        Some(&publication),
    );
    assert_eq!(demoted.code, "NUMERICAL_INTEGRITY_SENSITIVE");
    assert_eq!(
        demoted.message,
        format!("{} {} {line}", passed.message, finding.sentence)
    );
    // After F1a's line.
    let check = w2_estimate();
    let sensitive = w2_record(SolveQuality::Sensitive, None, None, None);
    let both = w2_record(
        SolveQuality::Sensitive,
        None,
        Some(&check),
        Some(&publication),
    );
    assert_eq!(
        both.message,
        format!(
            "{} {} {line}",
            sensitive.message,
            formation_check_evidence_line(&model, &check)
        )
    );
    // R-b' after the recovery loop: it demotes a Passed record carrying the
    // line and follows it; a Sensitive one is left byte for byte (no-op).
    let recovery = w2_recovery_finding();
    let passed_with = w2_record(SolveQuality::Passed, None, None, Some(&publication));
    let mut after = vec![passed_with.clone()];
    assert!(formation_guard::amend_integrity_report(
        &mut after,
        &integrity_diagnostic_id("c"),
        &recovery
    ));
    assert_eq!(after[0].code, "NUMERICAL_INTEGRITY_SENSITIVE");
    assert_eq!(
        after[0].message,
        format!("{} {line} {}", passed.message, recovery.sentence)
    );
    let sensitive_with = w2_record(SolveQuality::Sensitive, None, None, Some(&publication));
    let mut unchanged = vec![sensitive_with.clone()];
    assert!(!formation_guard::amend_integrity_report(
        &mut unchanged,
        &integrity_diagnostic_id("c"),
        &recovery
    ));
    assert_eq!(json(&unchanged[0]), json(&sensitive_with));
}

/// The W2 refusal (ROOT Q6 with OQ1 c2 and OQ4; the template fixed at
/// checkpoint 0): the exact diagnostic of every failure the product publishes.
/// `force_scale_exponent=none` where no b exists (steps 2-3); b omitted where
/// the orchestrator does not return it (step 4, a non-range failure); b where
/// the outcome carries it (admission and publication).
#[test]
fn f1b_force_scaling_refusal_exact_template() {
    let model = w2_unit_model();
    let map = format!("{:?}", integrity_dof_map(&model));
    let evaluation =
        RangeTrigger::Evaluation(StructuralError::Range("arithmetic outside normal range"));
    let formation = RangeTrigger::Formation(FrameKernelError::NumericalRange { name: "GJ/L: G*J" });
    let refused = |reason: ForceScaleReason, trigger: &RangeTrigger| {
        ForceScalingFailure::Refused(ForceScalingRefusal {
            reason,
            trigger: Some(trigger.clone()),
        })
    };
    let cases: Vec<(ForceScalingFailure, &RangeTrigger, &str, String)> = vec![
        (
            refused(ForceScaleReason::SubnormalAtFormation, &evaluation),
            &evaluation,
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            "range: subnormal stiffness or load at formation; range_scaling: attempted; step1_trigger=Evaluation(Range(\"arithmetic outside normal range\")); force_scale_exponent=none".into(),
        ),
        (
            refused(
                ForceScaleReason::InfeasibleWindow {
                    e_min: -1082,
                    e_max: 40,
                },
                &formation,
            ),
            &formation,
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            "range: exponent span [-1082, 40] exceeds the binary64 normal window after exact power-of-two scaling; range_scaling: attempted; step1_trigger=Formation(NumericalRange { name: \"GJ/L: G*J\" }); force_scale_exponent=none".into(),
        ),
        (
            refused(ForceScaleReason::ScaledEvaluation, &formation),
            &formation,
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            "range: scaled evaluation outside normal range; range_scaling: attempted; step1_trigger=Formation(NumericalRange { name: \"GJ/L: G*J\" })".into(),
        ),
        (
            ForceScalingFailure::Failed(ForceScaledError::Structural(StructuralError::InvalidInput(
                "x",
            ))),
            &evaluation,
            "NUMERICAL_INTEGRITY_FAILED",
            format!(
                "{}; range_scaling: attempted; step1_trigger=Evaluation(Range(\"arithmetic outside normal range\"))",
                StructuralError::InvalidInput("x")
            ),
        ),
        (
            ForceScalingFailure::Failed(ForceScaledError::Formation(
                FrameKernelError::NumericalRange { name: "EA/L: E*A" },
            )),
            &formation,
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            format!(
                "{}; range_scaling: attempted; step1_trigger=Formation(NumericalRange {{ name: \"GJ/L: G*J\" }})",
                FrameKernelError::NumericalRange { name: "EA/L: E*A" }
            ),
        ),
        (
            ForceScalingFailure::NotAdmitted {
                family: "thermal_or_eigen_load",
                b: 536,
            },
            &evaluation,
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            "range: family not admitted under force scaling: thermal_or_eigen_load; range_scaling: attempted; step1_trigger=Evaluation(Range(\"arithmetic outside normal range\")); force_scale_exponent=536; basis=exact power-of-two".into(),
        ),
        (
            ForceScalingFailure::Publication {
                quantity: "reaction@N0:UY".into(),
                b: -702,
            },
            &formation,
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            "range: publication outside binary64; at=reaction@N0:UY; range_scaling: attempted; step1_trigger=Formation(NumericalRange { name: \"GJ/L: G*J\" }); force_scale_exponent=-702; basis=exact power-of-two".into(),
        ),
        (
            ForceScalingFailure::NotEngaged,
            &evaluation,
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            "range: force scaling did not engage after a range trigger; range_scaling: attempted; step1_trigger=Evaluation(Range(\"arithmetic outside normal range\"))".into(),
        ),
    ];
    for (failure, trigger, code, reason) in cases {
        let mut diagnostics = vec![diag("diagnostic:x", "X", "info", "x", vec![])];
        append_force_scaling_refusal(&mut diagnostics, "c", &failure, trigger, &model);
        assert_eq!(diagnostics.len(), 2);
        let d = &diagnostics[1];
        assert_eq!(d.id, "diagnostic:numerical-integrity:c", "{failure:?}");
        assert_eq!(d.code, code, "{failure:?}");
        assert_eq!(d.severity, "blocking");
        assert_eq!(d.affected_refs, vec!["c".to_string()]);
        assert_eq!(
            d.message,
            format!(
                "Load case c: {reason}; global_dof_map={map}; no structural rejection is bypassed by generic LU or output quantization"
            ),
            "{failure:?}"
        );
    }
}

/// The ordinary attempt's result (F1b's pattern evidence), as a value.
fn pattern_attempt_result(
    built: &BuiltModel,
    springs: &[SpringEntry],
    stiffness: &SparseStiffness,
    force: &AssembledForce,
    prescribed: &[(usize, f64)],
    mode: PreviewSolverMode,
) -> Result<open_pipe_stress_frame_kernel::structural::StructuralSolution, StructuralError> {
    let (curved, springs, free, curved_sources) = attempt_inputs(built, springs, force, prescribed);
    SparseAssemblyEvidence::new(
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
    })
}

/// A floating body (N2-N3 has no ground): a witnessed mechanism.
fn mechanism_request() -> Value {
    frame_request(
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
    )
}

/// OQ4: a non-range failure at the chosen b keeps `append_integrity_failure`'s
/// code (one shared `integrity_failure_code`), here on a real mechanism and on
/// two constructed errors.
#[test]
fn f1b_non_range_failure_at_b_keeps_the_integrity_code() {
    let model = w2_unit_model();
    let bases = product_bases(&mechanism_request());
    let basis = &bases[0];
    let stiffness = assemble_basis_stiffness(&basis.built, &basis.springs).unwrap();
    let (_, force, _) = &basis.cases[0];
    let prescribed: Vec<(usize, f64)> = basis.restrained.iter().map(|&d| (d, 0.0)).collect();
    let mechanism = pattern_attempt_result(
        &basis.built,
        &basis.springs,
        &stiffness,
        force,
        &prescribed,
        PreviewSolverMode::SparseInteractive,
    )
    .unwrap_err();
    let trigger = RangeTrigger::Evaluation(StructuralError::Range("r"));
    let mut codes = Vec::new();
    for error in [
        mechanism,
        StructuralError::InvalidInput("x"),
        StructuralError::Range("r"),
    ] {
        let mut main_route = Vec::new();
        append_integrity_failure(&mut main_route, "c", &error, &model);
        let mut w2 = Vec::new();
        append_force_scaling_refusal(
            &mut w2,
            "c",
            &ForceScalingFailure::Failed(ForceScaledError::Structural(error.clone())),
            &trigger,
            &model,
        );
        assert_eq!(w2[0].code, main_route[0].code, "{error:?}");
        assert!(w2[0]
            .message
            .starts_with(&format!("Load case c: {error}; ")));
        codes.push(w2[0].code.clone());
    }
    assert_eq!(
        codes,
        [
            "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM",
            "NUMERICAL_INTEGRITY_FAILED",
            "NUMERICAL_INTEGRITY_UNRESOLVED"
        ]
    );
}

// ------------------------------------------------ D models (unit level)

fn pow2(exponent: i32) -> f64 {
    assert!((-1022..=1023).contains(&exponent));
    f64::from_bits(((exponent + 1023) as u64) << 52)
}

/// K2a's product-reach shape (`tests/k2a_formation_range_runtime.rs`
/// `request`): one member N0 -> N1 along x; N0 anchored; N1 anchored in every
/// DOF but `free`, which carries the load and, when given, a ground spring.
#[allow(clippy::too_many_arguments)]
fn reach_request(
    id: &str,
    length: f64,
    od: f64,
    wall: f64,
    e: f64,
    g: f64,
    free: &str,
    spring: Option<f64>,
    load: f64,
) -> Value {
    let all = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
    let anchored: Vec<&str> = all.iter().copied().filter(|d| *d != free).collect();
    let mut supports = vec![
        anchor("N0"),
        json!({"id": "rigid:N1", "node": "N1", "restraints": anchored, "family": "anchor", "provenance": PROV}),
    ];
    let rotational = free.starts_with('R');
    if let Some(k) = spring {
        let unit = if rotational { "N*m/rad" } else { "N/m" };
        supports.push(
            json!({"id": "spring:N1:0", "node": "N1", "family": "spring", "restraints": [free],
            "stiffness": {"dof": free, "value": {"value": k, "unit": unit}}, "provenance": PROV}),
        );
    }
    let mut request = frame_request(
        id,
        &[("N0", [0.0, 0.0, 0.0]), ("N1", [length, 0.0, 0.0])],
        &[("M1", "N0", "N1")],
        supports,
        &[("load:0", "N1", free, load)],
    );
    let model = &mut request["model"];
    model["pipe_segments"][0]["section"] = json!({
        "outside_diameter": {"value": od, "unit": "m"}, "wall_thickness": {"value": wall, "unit": "m"}});
    model["materials"][0]["elastic_modulus"]["value"] = json!(e);
    model["materials"][0]["shear_modulus"]["value"] = json!(g);
    request
}

/// The four K2a reach cases' linear variants (K2a's constants).
fn reach_requests() -> Vec<(&'static str, Value)> {
    vec![
        (
            "spring-carried",
            reach_request(
                "spring-carried",
                2.0,
                1.0e-6,
                1.0e-7,
                2.0e11,
                1.0e-300,
                "RX",
                Some(1.0),
                1.0,
            ),
        ),
        (
            "partial-underflow",
            reach_request(
                "partial-underflow",
                9.5367431640625e-07,
                3.0e-8,
                3.0e-9,
                1.3e-292,
                1.0e-200,
                "UY",
                None,
                1.0e-307,
            ),
        ),
        (
            "exact-zero",
            reach_request(
                "exact-zero",
                1.8189894035458565e-12,
                1.0e-11,
                1.0e-12,
                6.4e-280,
                1.0e-100,
                "UY",
                Some(3.7e-289),
                9.25e-290,
            ),
        ),
        (
            "least-subnormal",
            reach_request(
                "least-subnormal",
                1.8189894035458565e-12,
                1.0e-11,
                1.0e-12,
                9.6e-280,
                1.0e-100,
                "UY",
                None,
                2.05e-289,
            ),
        ),
    ]
}

/// A skew chain of `members` members scaled by exact powers of two (the
/// RF-RANGE LEF-large construction, K2b RETURN §9 C): lengths and section
/// dimensions by 2^pl, moduli by 2^pm, forces by 2^pf and moments by
/// 2^(pf+pl). (0, 0, 0) is the base model.
fn scaled_chain_request(members: usize, pl: i32, pm: i32, pf: i32) -> Value {
    let length = pow2(pl);
    let mut request = chain_request(members);
    let model = &mut request["model"];
    for node in model["nodes"].as_array_mut().unwrap() {
        for axis in ["x", "y", "z"] {
            let value = node["position"][axis].as_f64().unwrap();
            node["position"][axis] = json!(value * length);
        }
    }
    for member in model["pipe_segments"].as_array_mut().unwrap() {
        member["section"] = json!({
            "outside_diameter": {"value": 0.2 * length, "unit": "m"},
            "wall_thickness": {"value": 0.01 * length, "unit": "m"}});
    }
    model["materials"][0]["elastic_modulus"]["value"] = json!(2.0e11 * pow2(pm));
    model["materials"][0]["shear_modulus"]["value"] = json!(8.0e10 * pow2(pm));
    for load in model["load_cases"][0]["primitive_loads"]
        .as_array_mut()
        .unwrap()
    {
        let scale = if load["dimension"] == "moment" {
            pow2(pf + pl)
        } else {
            pow2(pf)
        };
        let value = load["magnitude"]["value"].as_f64().unwrap();
        load["magnitude"]["value"] = json!(value * scale);
    }
    request
}

/// K2b's documented-limitation chain (K2b RETURN §13.3) at product level:
/// N0-N1-N2 along x, 1 m bars, E = 2^440 Pa, G = 2^439 Pa, UX loads of
/// 2^-1010 N at N1 and 1 N at N2; N1 and N2 are free in UX only.
fn limitation_chain_request() -> Value {
    let free_ux = |node: &str| {
        json!({"id": format!("rigid:{node}"), "node": node, "family": "anchor",
               "restraints": ["UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV})
    };
    let mut request = frame_request(
        "limitation-chain",
        &[
            ("N0", [0.0, 0.0, 0.0]),
            ("N1", [1.0, 0.0, 0.0]),
            ("N2", [2.0, 0.0, 0.0]),
        ],
        &[("M1", "N0", "N1"), ("M2", "N1", "N2")],
        vec![anchor("N0"), free_ux("N1"), free_ux("N2")],
        &[
            ("load:n1", "N1", "UX", pow2(-1010)),
            ("load:n2", "N2", "UX", 1.0),
        ],
    );
    let model = &mut request["model"];
    model["materials"][0]["elastic_modulus"]["value"] = json!(pow2(440));
    model["materials"][0]["shear_modulus"]["value"] = json!(pow2(439));
    request
}

/// PHYS-R4 (`tests/pressure_membrane_range.rs` `fixture`, with its inputs):
/// OD 4e-77 m, wall 1e-77 m, L 1 m, E 1 Pa, nu 0.1. `pressurized`: both ends
/// fixed and p = 4.7e-170 Pa (the public fixture); otherwise a cantilever
/// with tip Fy = tip Mx = f64 bits 0x0031fa182c40c60d (about 1e-307).
fn phys_r4_request(pressurized: bool) -> Value {
    let a = "node:section-a";
    let b = "node:section-b";
    let anchor_at = |id: &str, node: &str| {
        json!({"id": id, "node": node, "family": "anchor",
               "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"],
               "provenance": "independent_section_geometry_control"})
    };
    let mut supports = vec![anchor_at("support:section-a", a)];
    let mut loads = Vec::new();
    let mut regions = Vec::new();
    if pressurized {
        supports.push(anchor_at("support:section-b", b));
        regions.push(json!({"id": "region:source-section", "member_pipe_ids": ["pipe:source-section"],
            "pressure_basis": "internal_differential_zero_external_v1",
            "pressure": {"value": 4.7e-170, "unit": "Pa"},
            "terminals": [
                {"node_ref": a, "closure_transfer": "transfers_to_wall", "provenance": "explicit_test_closure"},
                {"node_ref": b, "closure_transfer": "transfers_to_wall", "provenance": "explicit_test_closure"}],
            "provenance": "independent_geometry_pressure_reference"}));
    } else {
        let tip = f64::from_bits(0x0031fa182c40c60d);
        loads.push(json!({"id": "load:tip-y", "category": "concentrated_force",
            "target": {"type": "node", "node": b}, "direction": "global_y", "dimension": "force",
            "magnitude": {"value": tip, "unit": "N"}, "provenance": "independent_section_geometry_reference"}));
        loads.push(json!({"id": "load:tip-torque", "category": "concentrated_moment",
            "target": {"type": "node", "node": b}, "direction": "rotation_x", "dimension": "moment",
            "magnitude": {"value": tip, "unit": "N*m"}, "provenance": "independent_section_geometry_reference"}));
    }
    json!({"model": {
        "schema_version": "0.3.0", "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "2.0.0", "mode": "exact_straight_pressure_v2"},
        "project": {"id": "project:section-oracle", "units": {"length": "m", "force": "N", "angle": "rad",
            "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": a, "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": "synthetic"},
            {"id": b, "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": "synthetic"}],
        "pipe_segments": [{"id": "pipe:source-section", "from": a, "to": b, "section": {
            "outside_diameter": {"value": 4e-77, "unit": "m"},
            "wall_thickness": {"value": 1e-77, "unit": "m"}},
            "material": "material:section", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
            "provenance": "arithmetic_geometry_control_not_manufactured_pipe"}],
        "materials": [{"id": "material:section", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": {"value": 1.0, "unit": "Pa"},
            "poisson_ratio": {"value": 0.1, "unit": "1"},
            "provenance": "synthetic_isotropic_input"}],
        "supports": supports, "components": [],
        "load_cases": [{"id": "case:source-section", "primitive_loads": loads, "pressure_regions": regions,
            "provenance": "independent_reference"}], "combinations": []
    }, "materials": []})
}

/// The D models whose case ledger is nodal (so `product_bases` forms the
/// product's own ledger): K2a's reach set, the LEF-large analogue, the
/// limitation chain, PHYS-R4 without pressure, and a mechanism.
fn w2_nodal_models() -> Vec<(&'static str, Value)> {
    let mut models = reach_requests();
    models.push(("lef-large analogue", scaled_chain_request(5, 200, 300, 600)));
    models.push(("limitation chain", limitation_chain_request()));
    models.push(("phys-r4 without pressure", phys_r4_request(false)));
    models.push(("mechanism", mechanism_request()));
    models
}

/// The product's step 1 for one case: the ordinary attempt on the basis as
/// `form_basis_stiffness` formed it (a deferred formation refusal is the
/// attempt's failure).
fn step_one(
    basis: &Basis,
    stiffness: &BasisStiffness,
    force: &AssembledForce,
    prescribed: &[(usize, f64)],
    mode: PreviewSolverMode,
) -> Result<open_pipe_stress_frame_kernel::structural::StructuralSolution, OrdinaryFailure> {
    match stiffness {
        BasisStiffness::RangeDeferred(error) => Err(OrdinaryFailure::Formation(error.clone())),
        BasisStiffness::Formed(formed) => pattern_attempt_result(
            &basis.built,
            &basis.springs,
            formed,
            force,
            prescribed,
            mode,
        )
        .map_err(OrdinaryFailure::Structural),
    }
}

/// The orchestrator's case, formed as `force_scaling_attempt` forms it.
fn orchestrator_case<'a>(
    basis: &'a Basis,
    inputs: &'a AttemptInputs,
    force: &'a AssembledForce,
    prescribed: &'a [(usize, f64)],
    mode: PreviewSolverMode,
) -> ForceScalingCase<'a> {
    ForceScalingCase {
        node_count: basis.built.nodes.len(),
        frames: &basis.built.frame_elements,
        users: &basis.built.user_stiffness_elements,
        curved: &inputs.0,
        curved_sources: &inputs.3,
        springs: &inputs.1,
        force,
        prescribed,
        mode: linear_mode(mode),
        selected: true,
        representation: EvidenceRepresentation::Pattern,
    }
}

/// D-CLASS (T3 D1 §4.7 step 1; F1b's derivation D1): PP's range
/// classification of the ordinary attempt equals the orchestrator's step 1 on
/// every linear case of the declared B subset and the D models, in both modes:
/// - PP finds no range trigger: the orchestrator returns PP's own b = 0
///   result (the same solution, byte for byte in `Debug`, with no record; or
///   the same error);
/// - PP finds a trigger: the orchestrator goes on past b = 0 (published at
///   b != 0, or refused with PP's trigger recorded, or a failure at the
///   chosen b or of the census).
#[test]
fn f1b_range_classification_equals_the_orchestrators() {
    let mut models = declared_subset();
    models.extend(w2_nodal_models());
    let (mut not_range, mut range) = (0, 0);
    for (name, request) in &models {
        for basis in product_bases(request) {
            if !basis.built.nonlinear_supports.is_empty() {
                continue; // W2 never engages there
            }
            let stiffness = form_basis_stiffness(&basis.built, &basis.springs, true).unwrap();
            for (case, force, values) in &basis.cases {
                let prescribed: Vec<(usize, f64)> = basis
                    .restrained
                    .iter()
                    .enumerate()
                    .map(|(k, &dof)| (dof, values.as_ref().map_or(0.0, |v| v[k])))
                    .collect();
                let inputs = attempt_inputs(&basis.built, &basis.springs, force, &prescribed);
                for mode in MODES {
                    let ctx = format!("{name} [{}] {case} {mode:?}", basis.label);
                    let ours = step_one(&basis, &stiffness, force, &prescribed, mode);
                    let trigger = ours.as_ref().err().and_then(ordinary_range_trigger);
                    let theirs = solve_with_force_scaling(&orchestrator_case(
                        &basis,
                        &inputs,
                        force,
                        &prescribed,
                        mode,
                    ));
                    match (&ours, &trigger, &theirs) {
                        (Ok(solution), None, Ok(outcome)) => {
                            assert!(outcome.solution.force_scale.is_unscaled(), "{ctx}");
                            assert!(outcome.solution.records.is_empty(), "{ctx}");
                            assert_eq!(
                                format!("{:?}", outcome.solution.solution),
                                format!("{solution:?}"),
                                "{ctx}"
                            );
                            not_range += 1;
                        }
                        (
                            Err(OrdinaryFailure::Structural(error)),
                            None,
                            Err(ForceScaledError::Structural(theirs)),
                        ) => {
                            assert_eq!(error, theirs, "{ctx}");
                            not_range += 1;
                        }
                        (Err(_), Some(_), Ok(outcome)) => {
                            assert!(!outcome.solution.force_scale.is_unscaled(), "{ctx}");
                            range += 1;
                        }
                        (Err(_), Some(ours), Err(ForceScaledError::Refused(refusal))) => {
                            assert_eq!(refusal.trigger.as_ref(), Some(ours), "{ctx}");
                            range += 1;
                        }
                        (Err(_), Some(_), Err(_)) => range += 1,
                        _ => panic!("{ctx}: PP {ours:?} (trigger {trigger:?}) against the orchestrator {theirs:?}"),
                    }
                }
            }
        }
    }
    // Not vacuous: both directions are exercised. Range: the seven D range
    // models (the mechanism is not one) in both modes; not range: the nine
    // linear cases of the B subset and the mechanism, in both modes.
    assert_eq!((not_range, range), (18, 14));
}

/// RV11-N4: the product passes the case's unscaled ledger to the orchestrator
/// (the NI pin keeps `force_scaled(` and `with_force_scale` out of every
/// product module, so no scaled ledger exists there). The value tests detect
/// a pre-scaled ledger: on K2a's exact-zero case (b = 734) the unscaled ledger
/// gives u_y within 1e-9 of the exact reference, and the same ledger scaled by
/// 2^b first gives a different u (here, 2^b times it at b = 0).
#[test]
fn f1b_rv11_n4_a_pre_scaled_ledger_is_detected_by_the_values() {
    let request = reach_requests().remove(2).1;
    let bases = product_bases(&request);
    let basis = &bases[0];
    let (_, force, _) = &basis.cases[0];
    let prescribed: Vec<(usize, f64)> = basis.restrained.iter().map(|&d| (d, 0.0)).collect();
    let inputs = attempt_inputs(&basis.built, &basis.springs, force, &prescribed);
    let reference = f64::from_bits(0x3fc001034445ee29);
    for mode in MODES {
        let outcome =
            solve_with_force_scaling(&orchestrator_case(basis, &inputs, force, &prescribed, mode))
                .unwrap();
        let scale = outcome.solution.force_scale;
        assert_eq!(scale.exponent(), 734, "{mode:?}");
        let u = outcome.solution.solution.displacements[7];
        assert!(((u - reference) / reference).abs() <= 1e-9, "{mode:?}: {u}");
        let prescaled = force.force_scaled(scale).unwrap();
        match solve_with_force_scaling(&orchestrator_case(
            basis,
            &inputs,
            &prescaled,
            &prescribed,
            mode,
        )) {
            Ok(twice) => {
                let moved = twice.solution.solution.displacements[7];
                assert!(
                    ((moved - reference) / reference).abs() > 1.0,
                    "{mode:?}: {moved}"
                );
            }
            Err(error) => panic!("{mode:?}: the pre-scaled ledger was refused: {error:?}"),
        }
    }
}

/// One case's admission inputs, formed by the product's front end.
struct AdmissionInputs {
    model: PreviewModel,
    built: BuiltModel,
    load_application: LoadApplication,
    force: AssembledForce,
}

fn admission_inputs(request: &Value) -> AdmissionInputs {
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
    let built = build_model(&model, &materials, &mut d).unwrap();
    let primitives = build_load_case_primitive_loads(&model, &model.load_cases[0], &mut d);
    let load_application = prepare_loads(built.nodes.len(), built.pipes.len(), &primitives);
    let force = nodal_and_eigen_case_force(&load_application, &[], &built).unwrap();
    AdmissionInputs {
        model,
        built,
        load_application,
        force,
    }
}

impl AdmissionInputs {
    fn admit(
        &self,
        built: Option<&BuiltModel>,
        thermal: &[ThermalElementLoad],
        thrust: &[PressureThrustLoad],
        force: Option<&AssembledForce>,
    ) -> Result<(), ForceScalingFailure> {
        force_scaling_admission(
            &self.model,
            built.unwrap_or(&self.built),
            &self.model.load_cases[0],
            &self.load_application,
            thermal,
            thrust,
            None,
            force.unwrap_or(&self.force),
            734,
        )
    }
}

fn family_of(result: Result<(), ForceScalingFailure>) -> &'static str {
    match result {
        Ok(()) => "admitted",
        Err(ForceScalingFailure::NotAdmitted { family, b }) => {
            assert_eq!(b, 734);
            family
        }
        Err(other) => panic!("{other:?}"),
    }
}

/// Admission (ROOT Q3, OQ13 narrowed; the A2 order): each check names its
/// family, including the three the product cannot reach at b != 0 (a
/// user-stiffness element: every realizable one is refused by M07 containment
/// or input validation; a pressure thrust; a non-nodal ledger term), and a
/// thermal load is named as thermal although it is also an element primitive.
/// (The exact-pressure operand is reached at product level, in
/// `tests/f1b_w2_runtime.rs`.)
#[test]
fn f1b_admission_names_each_family() {
    let base = admission_inputs(&chain_request(2));
    assert_eq!(family_of(base.admit(None, &[], &[], None)), "admitted");
    // 1. A user-stiffness joint (M07 refuses its solve; the builder forms it).
    let mut joint = chain_request(2);
    joint["model"]["components"] = json!([{"id": "component:joint", "label": "invented joint",
        "kind": "expansion_joint", "node": "N1",
        "geometry": {"expansion_joint_pipe_ref": "M2", "effective_area": {"value": 0.01, "unit": "m^2"},
                     "expansion_joint_source_reference": "invented"},
        "modifiers": {"axial_stiffness_user_value": {"value": 3.2e6, "unit": "N/m"},
                      "lateral_stiffness_user_value": {"value": 9.0e5, "unit": "N/m"},
                      "angular_stiffness_user_value": {"value": 4.8e5, "unit": "N*m/rad"},
                      "torsional_stiffness_user_value": {"value": 6.2e5, "unit": "N*m/rad"},
                      "source_reference": "invented"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_and_user_flexibility",
                                "rule_check_consumption": "user_rule_pack_inputs_only"},
        "provenance": PROV}]);
    let joint = admission_inputs(&joint);
    assert!(!joint.built.user_stiffness_elements.is_empty());
    assert_eq!(
        family_of(base.admit(Some(&joint.built), &[], &[], None)),
        "user_stiffness_element"
    );
    // 2. A realized curved bend.
    let elbow = admission_inputs(&curved_elbow_request());
    assert_eq!(
        family_of(base.admit(Some(&elbow.built), &[], &[], None)),
        "curved_bend_macro_element"
    );
    // 3-5. Thermal (named first, with its element primitive), pressure
    // thrust, and an element primitive alone.
    let thermal = [ThermalElementLoad {
        element_index: 0,
        source: "load:t".into(),
        axial_load: 1.0,
        thermal_strain: 1.0e-5,
    }];
    let thrust = [PressureThrustLoad {
        element_index: 0,
        axial_load: 1.0,
        source_load_id: "load:p".into(),
        source: PressureThrustSource::PipeInternalArea,
    }];
    let mut uniform = chain_request(2);
    uniform["model"]["load_cases"][0]["primitive_loads"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id": "load:w", "category": "distributed_force",
            "target": {"type": "element", "pipe": "M1"}, "direction": "global_z",
            "magnitude": {"value": -50.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": PROV}));
    let uniform = admission_inputs(&uniform);
    assert_eq!(uniform.load_application.element_uniform_loads.len(), 1);
    assert_eq!(
        family_of(uniform.admit(None, &thermal, &[], None)),
        "thermal_or_eigen_load"
    );
    assert_eq!(
        family_of(uniform.admit(None, &[], &thrust, None)),
        "pressure_thrust_load"
    );
    assert_eq!(
        family_of(uniform.admit(None, &[], &[], None)),
        "uniform_element_load"
    );
    // 7. A consumed constant-effort support.
    let mut effort = chain_request(2);
    effort["model"]["supports"].as_array_mut().unwrap().push(json!({
        "id": "support:ce", "node": "N1", "family": "constant_effort_support", "restraints": ["UY"],
        "hanger": {"hanger_type": "constant_effort_support", "constant_load": {"value": 375.0, "unit": "N"},
                   "travel_range": {"value": 0.05, "unit": "m"}, "source_reference": "invented"},
        "provenance": PROV}));
    let effort = admission_inputs(&effort);
    assert_eq!(
        family_of(effort.admit(None, &[], &[], None)),
        "constant_effort_support"
    );
    // 8-9. A ledger term that is not an authored nodal load, and an authored
    // nodal term of exactly zero (OQ13 narrowed: refused, disclosed).
    let n = base.force.len();
    let ledger = |source: &str, value: f64| {
        let mut ledger = open_pipe_stress_frame_kernel::load_ledger::LoadLedger::new();
        ledger.push("load:tip-y", 13, 1250.0);
        ledger.push(source, 7, value);
        ledger.finish(n).unwrap()
    };
    assert_eq!(
        family_of(base.admit(None, &[], &[], Some(&ledger("support:x", 1.0)))),
        "non_nodal_load_term"
    );
    assert_eq!(
        family_of(base.admit(None, &[], &[], Some(&ledger("load:tip-rx", 0.0)))),
        "zero_nodal_load_term"
    );
    assert_eq!(
        family_of(base.admit(None, &[], &[], Some(&ledger("load:tip-rx", -0.0)))),
        "zero_nodal_load_term"
    );
    assert_eq!(
        family_of(base.admit(None, &[], &[], Some(&ledger("load:tip-rx", 1.0)))),
        "admitted"
    );
}

/// F-A2 at unit level (RV11D-1): on K2a's exact-zero case, the orchestrator's
/// own outcome with its displacements scaled by 2^-80 makes N0's reactions
/// underflow after unscaling; the publication refuses them by name (never a
/// flushed or wrong `Normal` value), and the unmodified outcome publishes.
#[test]
fn f1b_publication_refuses_an_underflowing_reaction_by_name() {
    let request = reach_requests().remove(2).1;
    let bases = product_bases(&request);
    let basis = &bases[0];
    let (_, force, _) = &basis.cases[0];
    let prescribed: Vec<(usize, f64)> = basis.restrained.iter().map(|&d| (d, 0.0)).collect();
    let inputs = attempt_inputs(&basis.built, &basis.springs, force, &prescribed);
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let model = parsed.model;
    let outcome = solve_with_force_scaling(&orchestrator_case(
        basis,
        &inputs,
        force,
        &prescribed,
        PreviewSolverMode::SparseInteractive,
    ))
    .unwrap();
    let published = force_scaled_publication(
        &model,
        &basis.built,
        &basis.springs,
        &basis.restrained,
        &outcome,
        force,
    )
    .unwrap();
    assert_eq!(published.force_scale_exponent, 734);
    assert!(published.reactions.iter().all(|(_, p)| p.value.is_finite()));
    let mut shrunk = outcome.clone();
    for u in &mut shrunk.solution.solution.displacements {
        *u *= pow2(-80);
    }
    match force_scaled_publication(
        &model,
        &basis.built,
        &basis.springs,
        &basis.restrained,
        &shrunk,
        force,
    ) {
        Err(ForceScalingFailure::Publication { quantity, b }) => {
            assert_eq!(b, 734);
            assert!(quantity.starts_with("reaction@"), "{quantity}");
        }
        other => panic!("{other:?}"),
    }
}

/// Publication at b != 0: an in-range reaction is 2^-b times the scaled exact
/// sum `sum_j K'_ij u_j - f'_i` (K' and f' formed at 2^b), rounded once, on
/// K2a's exact-zero case (b = 734), wherever the published value is normal.
#[test]
fn f1b_an_in_range_reaction_is_the_scaled_exact_sum_rounded_once() {
    let request = reach_requests().remove(2).1;
    let bases = product_bases(&request);
    let basis = &bases[0];
    let (_, force, _) = &basis.cases[0];
    let prescribed: Vec<(usize, f64)> = basis.restrained.iter().map(|&d| (d, 0.0)).collect();
    let inputs = attempt_inputs(&basis.built, &basis.springs, force, &prescribed);
    let model = serde_json::from_value::<LinearStaticPreviewRequest>(request.clone())
        .unwrap()
        .model;
    for mode in MODES {
        let outcome =
            solve_with_force_scaling(&orchestrator_case(basis, &inputs, force, &prescribed, mode))
                .unwrap();
        let scale = outcome.solution.force_scale;
        let b = scale.exponent();
        let scaled_force = force.force_scaled(scale).unwrap();
        let u = &outcome.solution.solution.displacements;
        let published = force_scaled_publication(
            &model,
            &basis.built,
            &basis.springs,
            &basis.restrained,
            &outcome,
            force,
        )
        .unwrap();
        let mut compared = 0;
        for &(dof, value) in &published.reactions {
            if value.representability != Representability::Normal || value.value == 0.0 {
                continue;
            }
            let mut sum = ExactAccumulator::new();
            for (column, displacement) in u.iter().enumerate() {
                sum.add_product(outcome.stiffness.get(dof, column), *displacement)
                    .unwrap();
            }
            scaled_force.accumulate_dof(dof, &mut sum, true).unwrap();
            let rounded = sum.round().unwrap();
            let expected = rounded * pow2(-b / 2) * pow2(-b + b / 2);
            assert_eq!(
                value.value.to_bits(),
                expected.to_bits(),
                "{mode:?} dof {dof}"
            );
            compared += 1;
        }
        assert!(compared >= 2, "{mode:?}: {compared}");
    }
}

// ------------------------------------------------------------------ E

/// E (ROOT OQ3): an input whose ordinary evidence was formed at b != 0 is
/// refused `Unsupported`, named, with nothing charged or rejected, before any
/// read of the input (here an empty stiffness, which `solve` refuses as a
/// dimension mismatch after charging); at b = 0 the wrapper is `solve`.
#[test]
fn f1b_solve_ordinary_refuses_force_scaled_evidence_with_zero_work() {
    let request = chain_request(2);
    let bases = product_bases(&request);
    let basis = &bases[0];
    let n = basis.built.nodes.len() * DOF_PER_NODE;
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
    let empty: Vec<Vec<f64>> = Vec::new();
    let limits = exact::Limits {
        operations: 1_000_000,
        ..Default::default()
    };
    let attempt = |scale: ForceScale| {
        source_recovery::solve_ordinary(
            source_recovery::Input {
                model: &model,
                built: &basis.built,
                stiffness: &empty,
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
            limits,
            scale,
        )
        .map(|_| ())
        .unwrap_err()
    };
    for b in [2, -2, 734, -702] {
        let refused = attempt(ForceScale::new(b).unwrap());
        assert_eq!(refused.stage, "source closure", "{b}");
        assert_eq!(
            refused.error,
            source_recovery::RecoveryError::Unsupported(
                "force-scaled ordinary evidence (b != 0) is not a retained-source input"
            ),
            "{b}"
        );
        assert_eq!((refused.work.charged, refused.work.rejected), (0, 0), "{b}");
    }
    // At b = 0 the wrapper is `solve`: the empty stiffness reaches its
    // dimension check (a different, charged refusal).
    let unscaled = attempt(ForceScale::UNSCALED);
    assert_eq!(
        unscaled.error,
        source_recovery::RecoveryError::SourceMismatch("actual model/source dimensions")
    );
    assert!(unscaled.work.charged > 0);
}

/// E (ROOT OQ2, option B): a formation-range case is declined without an
/// attempt, named, with zero work, like S11-G's load-row decline.
#[test]
fn f1b_range_formation_decline_is_named_and_charges_nothing() {
    let decline = source_recovery::range_formation_decline_without_attempt();
    let guard = source_recovery::formation_decline_without_attempt();
    assert_eq!(decline.stage, "range formation");
    assert_eq!(decline.helper_stage, guard.helper_stage);
    assert_eq!(
        decline.error,
        source_recovery::RecoveryError::Unsupported(
            "the ordinary stiffness was not formed (range at formation); no retained-source attempt"
        )
    );
    assert_eq!(
        (
            decline.work.charged,
            decline.work.rejected,
            decline.work.limit
        ),
        (0, 0, 0)
    );
}
