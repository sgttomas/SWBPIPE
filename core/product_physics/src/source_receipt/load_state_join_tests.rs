//! Maintained closure tests for the resolved load/reference-state
//! retained-source join (CP3 review N-1). Each test replaces one of the
//! independent CP3 reviewer's probes with real assertions (P1, P2, P4, P5,
//! P6, P7, P11), plus a motion-ownership refusal. P3, P9 and P12 are pinned in
//! `load_state_fallback_tests`; P8 and P10 were characterizations only.
//!
//! Expected mechanics are closed-form and independent of the producer. All
//! inputs are invented for these tests (the maintained `eigen_motion` and
//! `mixed` witnesses plus invented edits); no material or component library
//! value, code rule or default is used.
use super::*;
use crate::case_state::resolve::ResolvedCase;
use crate::pressure_exact::IsotropicENu;
use open_pipe_stress_primitive_loads::LoadApplication;

/// Invented provenance for every value these tests add to a witness.
const INVENTED: &str = "invented_load_state_join_closure_test_not_library_data";
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
const CASE: &str = "case:join";
// The witness fixture's authored constants (asserted in `witness`).
const L: f64 = 2.0;
const E_BASE: f64 = 2.0e11;
const NU_BASE: f64 = 0.25;
const THERMAL: f64 = 6.0e-5;
const FIT: f64 = 4.0e-5;
const TIP_UX: f64 = 5.0e-5;
const ROOT_UY_MM: f64 = 1.0;
const ROOT_RZ: f64 = 1.0e-4;
const TORQUE: f64 = 1.0e-8;
const TORSION_SPRING: f64 = 1.0e-4;
// Invented edits.
const E_POINT: f64 = 1.5e11;
const NU_POINT: f64 = 0.3;
// Global DOF indices of the two-node witness (6 per node, root first).
const ROOT_UX_DOF: usize = 0;
const ROOT_RX_DOF: usize = 3;
const TIP_UY_DOF: usize = 7;
const DOF_COUNT: usize = 12;

fn witness() -> Value {
    let request: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/load_reference_source/eigen_motion.request.json"
    ))
    .unwrap();
    let model = &request["model"];
    assert_eq!(
        model["materials"][0]["elastic_modulus"]["value"],
        json!(E_BASE)
    );
    assert_eq!(
        model["materials"][0]["poisson_ratio"]["value"],
        json!(NU_BASE)
    );
    assert_eq!(model["nodes"][1]["position"]["x"], json!(L));
    assert_eq!(
        model["reference_configurations"][0]["member_references"][0]["fit"]["strain"]["value"],
        json!(FIT)
    );
    let case = &model["load_cases"][0];
    assert_eq!(case["id"], json!(CASE));
    assert_eq!(
        case["analysis_state"]["element_states"][0]["thermal_state"]["strain"]["value"],
        json!(THERMAL)
    );
    assert_eq!(
        case["primitive_loads"][0]["magnitude"]["value"],
        json!(TORQUE)
    );
    let motions = &case["analysis_state"]["support_states"];
    assert_eq!(
        motions[0]["boundary_motion"][0]["value"],
        json!({"value": ROOT_UY_MM, "unit": "mm"})
    );
    assert_eq!(
        motions[0]["boundary_motion"][1]["value"]["value"],
        json!(ROOT_RZ)
    );
    assert_eq!(
        motions[2]["boundary_motion"][0]["value"]["value"],
        json!(TIP_UX)
    );
    assert_eq!(
        model["supports"][1]["stiffness"]["value"]["value"],
        json!(TORSION_SPRING)
    );
    request
}

fn mixed_witness() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/load_reference_source/mixed.request.json"
    ))
    .unwrap()
}

/// An invented exact material point whose pair differs from the base pair,
/// selected by the first case's member.
fn with_exact_point(mut request: Value, e: f64, nu: f64) -> Value {
    request["model"]["materials"][0]["temperature_points"] = json!([{
        "id": "point:join-test", "temperature": {"value": 20, "unit": "degC"},
        "elastic_modulus": {"value": e, "unit": "Pa"}, "poisson_ratio": {"value": nu, "unit": "1"},
        "provenance": INVENTED}]);
    request["model"]["load_cases"][0]["analysis_state"]["element_states"][0]
        ["material_selection"] =
        json!({"kind": "exact_point", "material_ref": "material", "point_ref": "point:join-test"});
    request
}

/// An invented pressure region (possibly zero-valued) on case `index`.
fn with_pressure_region(mut request: Value, index: usize, pressure_pa: f64) -> Value {
    request["model"]["load_cases"][index]["pressure_regions"] = json!([{
        "id": "region:join-test", "member_pipe_ids": ["member"],
        "pressure_basis": "internal_differential_zero_external_v1",
        "pressure": {"value": pressure_pa, "unit": "Pa"},
        "terminals": [
            {"node_ref": "root", "closure_transfer": "transfers_to_wall", "provenance": INVENTED},
            {"node_ref": "tip", "closure_transfer": "transfers_to_wall", "provenance": INVENTED}],
        "provenance": INVENTED}]);
    request
}

/// A second case copied from the first, with its own root UY motion, its own
/// thermal strain and the base pair explicitly selected.
fn with_second_case(mut request: Value, id: &str, root_uy_mm: f64, thermal: f64) -> Value {
    let mut case = request["model"]["load_cases"][0].clone();
    case["id"] = json!(id);
    let load = format!("torque:{id}");
    case["primitive_loads"][0]["id"] = json!(load);
    let state = &mut case["analysis_state"];
    state["load_sources"][0]["source_ref"] = json!(load);
    state["support_states"][0]["boundary_motion"][0]["value"]["value"] = json!(root_uy_mm);
    state["element_states"][0]["thermal_state"]["strain"]["value"] = json!(thermal);
    state["element_states"][0]["material_selection"] = json!({"kind": "explicit_base_properties",
        "material_ref": "material", "applicability_reference": INVENTED});
    request["model"]["load_cases"]
        .as_array_mut()
        .unwrap()
        .push(case);
    request
}

/// A declared primitive tip force whose ID collides with the member's
/// reserved eigen source ID `load_state_eigenstrain:<len>:<pipe>`.
fn with_colliding_primitive(mut request: Value) -> Value {
    let id = source_recovery::eigen_source_id("member");
    assert_eq!(id, "load_state_eigenstrain:6:member");
    let case = &mut request["model"]["load_cases"][0];
    let mut load = case["primitive_loads"][0].clone();
    load["id"] = json!(id);
    load["direction"] = json!("UZ");
    load["category"] = json!("concentrated_force");
    load["dimension"] = json!("force");
    load["magnitude"] = json!({"value": 1.0e-6, "unit": "N"});
    load["provenance"] = json!(INVENTED);
    case["primitive_loads"].as_array_mut().unwrap().push(load);
    case["analysis_state"]["load_sources"]
        .as_array_mut()
        .unwrap()
        .push(json!({"source_ref": id, "factor": 1.0}));
    request
}

fn wall_area() -> f64 {
    std::f64::consts::PI * (0.1_f64 * 0.1 - 0.09 * 0.09)
}
/// `lambda_fit*lambda_thermal - 1` for the witness fit strain.
fn eigenstrain(thermal: f64) -> f64 {
    (1.0 + thermal) * (1.0 + FIT) - 1.0
}
/// Member axial force `N = E*A*(delta/L - eps*)` (tension positive).
fn axial_force(e: f64, thermal: f64) -> f64 {
    e * wall_area() * (TIP_UX / L - eigenstrain(thermal))
}

fn close(actual: f64, expected: f64, scale: f64) {
    let allowance = if expected == 0.0 {
        scale * 1e-9
    } else {
        expected.abs() * 1e-9
    };
    assert!(
        (actual - expected).abs() <= allowance,
        "{actual:.17e} versus {expected:.17e}"
    );
}

fn case_row(envelope: &MechanicsEnvelope, case: &str, id_suffix: &str) -> f64 {
    let matched: Vec<_> = envelope
        .results
        .iter()
        .filter(|r| {
            r.basis_ref
                .as_ref()
                .is_some_and(|b| b.ref_type == "load_case" && b.ref_id == case)
                && r.id.ends_with(id_suffix)
        })
        .collect();
    assert_eq!(matched.len(), 1, "{case} {id_suffix}: {matched:?}");
    matched[0].value
}

fn diagnostics_for<'a>(
    envelope: &'a MechanicsEnvelope,
    code: &str,
    case: &str,
) -> Vec<&'a Diagnostic> {
    envelope
        .diagnostics
        .iter()
        .filter(|d| d.code == code && d.affected_refs.iter().any(|r| r == case))
        .collect()
}

fn assert_joined(envelope: &MechanicsEnvelope, cases: usize) {
    assert_eq!(
        envelope.producer.semantic_contract_id,
        case_state::LOAD_REFERENCE_SOURCE_SEMANTIC_CONTRACT_ID
    );
    assert_eq!(
        envelope.formulation_basis.profile_id,
        case_state::LOAD_STATE_SOURCE_PROFILE_ID
    );
    assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED");
    let receipt = envelope.source_block_recovery.as_ref().expect("receipt");
    assert_eq!(receipt["body"]["policy"], "LOAD-REFERENCE-SOURCE-1");
    assert_eq!(receipt["body"]["status"], "qualified");
    assert_eq!(receipt["body"]["cases"].as_array().unwrap().len(), cases);
    assert!(envelope
        .diagnostics
        .iter()
        .all(|d| d.severity != "blocking"));
}

/// The ordinary `load-reference-1` publication of an attempt that failed:
/// no receipt, and the case record and diagnostics say `unavailable`.
fn assert_not_joined_unavailable(envelope: &MechanicsEnvelope, request: &Value) {
    assert_eq!(
        envelope.producer.semantic_contract_id,
        case_state::LOAD_REFERENCE_SEMANTIC_CONTRACT_ID
    );
    assert_eq!(
        envelope.formulation_basis.profile_id,
        case_state::LOAD_STATE_PROFILE_ID
    );
    assert!(envelope.source_block_recovery.is_none());
    assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED");
    let record = &envelope.contract_evidence.as_ref().unwrap()["load_reference_states"][0];
    assert_eq!(
        record["source_recovery"],
        json!({"status": "not_joined", "code": "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"})
    );
    let not_joined = diagnostics_for(envelope, case_state::SOURCE_RECOVERY_NOT_JOINED, CASE);
    assert_eq!(not_joined.len(), 1);
    assert!(not_joined[0]
        .message
        .ends_with("retained_source_attempt=unavailable"));
    assert_eq!(
        diagnostics_for(envelope, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", CASE).len(),
        1
    );
    assert!(!envelope
        .diagnostics
        .iter()
        .any(|d| d.code == "SOURCE_BLOCK_RECOVERY_SELECTED"));
    // The published rows are the uncaptured ordinary route's rows, bit for bit.
    let typed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let mode = if record["solve"]["recovery_method"] == "ordinary_dense_structural_v1" {
        PreviewSolverMode::DenseScrutiny
    } else {
        PreviewSolverMode::SparseInteractive
    };
    let ordinary = run_linear_static_preview_with_mode(typed, mode);
    let bits = |e: &MechanicsEnvelope| {
        e.results
            .iter()
            .map(|r| (r.id.clone(), r.value.to_bits()))
            .collect::<Vec<_>>()
    };
    assert!(!ordinary.results.is_empty());
    assert_eq!(bits(envelope), bits(&ordinary));
}

/// The product route's own construction of the first resolved case, with an
/// optional perturbation of the resolved case before the build.
struct Parts {
    model: PreviewModel,
    resolved: ResolvedCase,
    built: BuiltModel,
    k: Vec<Vec<f64>>,
    f: Vec<f64>,
    free: Vec<usize>,
    prescribed: Vec<(usize, f64)>,
    springs: Vec<SpringEntry>,
    loads: LoadApplication,
    eigen: Vec<ThermalElementLoad>,
}
impl Parts {
    fn new(raw: &Value, perturb: impl FnOnce(&mut ResolvedCase)) -> Self {
        let request: LinearStaticPreviewRequest = serde_json::from_value(raw.clone()).unwrap();
        let supplied = !request.materials.is_empty();
        let mut model = request.model;
        let mut materials = model.materials.clone();
        let mut d = vec![];
        pressure_runtime::validate_profile(&model, &mut d);
        case_state::resolve::validate_document(&model, supplied, &mut d);
        resolve_shared_sections(&mut model, &mut d);
        normalize_model_units(&mut model, &mut materials, &mut d);
        let mut resolved =
            case_state::resolve::resolve_case(&model, &materials, &model.load_cases[0], &mut d)
                .unwrap();
        assert!(!has_blocking(&d), "{d:?}");
        perturb(&mut resolved);
        let built =
            build_model_for_members(&model, &materials, Some(&resolved.pairs), &mut d).unwrap();
        let boundary = prepare_boundary(built.nodes.len(), &built.supports);
        let k = assemble_case_stiffness(&built, &boundary.springs).unwrap();
        let primitive = build_load_case_primitive_loads(&model, &resolved.effective_case, &mut d);
        let loads = prepare_loads(built.nodes.len(), built.pipes.len(), &primitive);
        let eigen = load_state_eigen_loads(&resolved, &built).unwrap();
        let mut f = loads.global_load_vector(built.nodes.len());
        add_thermal_equivalent_loads(&mut f, &eigen, &built.pipes, &HashMap::new());
        let prescribed = boundary
            .restrained_dofs
            .iter()
            .map(|&dof| (dof, resolved.prescribed.get(&dof).copied().unwrap_or(0.0)))
            .collect();
        let free = (0..f.len())
            .filter(|dof| !boundary.restrained_dofs.contains(dof))
            .collect();
        assert!(!has_blocking(&d), "{d:?}");
        assert_eq!(f.len(), DOF_COUNT);
        Self {
            model,
            resolved,
            built,
            k,
            f,
            free,
            prescribed,
            springs: boundary.springs,
            loads,
            eigen,
        }
    }
    fn input(&self) -> source_recovery::Input<'_> {
        source_recovery::Input {
            model: &self.model,
            built: &self.built,
            stiffness: &self.k,
            force: &self.f,
            free: &self.free,
            prescribed: &self.prescribed,
            spring_entries: &self.springs,
            load_case: &self.resolved.effective_case,
            load_application: &self.loads,
            thermal_loads: &self.eigen,
            pressure_thrust_loads: &[],
            load_state: Some(&self.resolved),
        }
    }
}

fn limits() -> exact::Limits {
    exact::Limits {
        operations: PHYSICS_SOURCE_WORK_LIMIT,
        ..Default::default()
    }
}

fn closure_refusal(parts: &Parts) -> source_recovery::RecoveryFailure {
    match source_recovery::solve(parts.input(), limits()) {
        Err(failure) => failure,
        Ok(_) => panic!("source closure accepted the input"),
    }
}

/// Replaces probe P1.
#[test]
fn a_selected_case_whose_member_pair_differs_from_the_base_qualifies_with_its_own_pair() {
    let n = axial_force(E_POINT, THERMAL);
    let area = wall_area();
    for mode in MODES {
        let envelope = run_linear_static_preview_value_with_mode(
            with_exact_point(witness(), E_POINT, NU_POINT),
            mode,
        )
        .unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_joined(&envelope, 1);
        let receipt = &envelope.source_block_recovery.as_ref().unwrap()["body"]["cases"][0];
        assert_eq!(
            receipt["selected_method"],
            "retained_source_blocks_exact_v1"
        );
        // The published pair is the selected point's pair, not the base pair.
        let evidence = envelope.contract_evidence.as_ref().unwrap();
        let material = &evidence["exact_cases"][0]["pipe_materials"][0];
        assert_eq!(material["material_selection_kind"], "exact_point");
        assert_eq!(material["E_pa"], json!(E_POINT));
        assert_eq!(material["nu"], json!(NU_POINT));
        close(
            material["G_pa"].as_f64().unwrap(),
            E_POINT / (2.0 * (1.0 + NU_POINT)),
            E_POINT,
        );
        let member = &evidence["load_reference_states"][0]["members"][0];
        assert_eq!(member["selected_E_pa"], json!(E_POINT));
        assert_eq!(member["selected_nu"], json!(NU_POINT));
        close(
            member["total_eigenstrain"].as_f64().unwrap(),
            eigenstrain(THERMAL),
            1.0,
        );
        assert_ne!(E_POINT, E_BASE);
        // Closed form N = E*A*(delta/L - eps*) with the selected E: end i
        // carries -N, end j and every station carry N.
        let scale = E_POINT * area * 1e-4;
        close(case_row(&envelope, CASE, "force:member:axial"), -n, scale);
        close(
            case_row(&envelope, CASE, "force:member:axial:end-j"),
            n,
            scale,
        );
        for station in ["quarter-1", "midspan", "quarter-3"] {
            close(
                case_row(&envelope, CASE, &format!("force:member:{station}:axial")),
                n,
                scale,
            );
        }
        close(
            case_row(&envelope, CASE, "stress:member:midspan:axial-normal"),
            n / area / 1.0e6,
            1.0,
        );
        // The pair changes no kinematics: prescribed motions still govern.
        close(
            case_row(&envelope, CASE, "disp:tip:ux"),
            TIP_UX * 1000.0,
            1.0,
        );
        close(
            case_row(&envelope, CASE, "disp:tip:uy"),
            ROOT_UY_MM + ROOT_RZ * L * 1000.0,
            1.0,
        );
    }
}

/// Replaces probe P2.
#[test]
fn two_selected_cases_with_different_pairs_and_motions_each_bind_their_own_state() {
    const SECOND: &str = "case:second";
    const SECOND_UY_MM: f64 = 2.0;
    const SECOND_THERMAL: f64 = -3.0e-5;
    let area = wall_area();
    for mode in MODES {
        let request = with_second_case(
            with_exact_point(witness(), E_POINT, NU_POINT),
            SECOND,
            SECOND_UY_MM,
            SECOND_THERMAL,
        );
        let envelope = run_linear_static_preview_value_with_mode(request, mode)
            .unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_joined(&envelope, 2);
        let receipt = envelope.source_block_recovery.as_ref().unwrap();
        for case in receipt["body"]["cases"].as_array().unwrap() {
            assert_eq!(case["selected_method"], "retained_source_blocks_exact_v1");
        }
        let records = envelope.contract_evidence.as_ref().unwrap()["load_reference_states"]
            .as_array()
            .unwrap()
            .clone();
        assert_eq!(records.len(), 2);
        for (record, (id, e, thermal, uy_mm)) in records.iter().zip([
            (CASE, E_POINT, THERMAL, ROOT_UY_MM),
            (SECOND, E_BASE, SECOND_THERMAL, SECOND_UY_MM),
        ]) {
            assert_eq!(record["load_case_id"], json!(id));
            assert_eq!(
                record["source_recovery"],
                json!({"status": "selected", "method": "retained_source_blocks_exact_v1"})
            );
            assert_eq!(record["members"][0]["selected_E_pa"], json!(e));
            close(
                record["members"][0]["total_eigenstrain"].as_f64().unwrap(),
                eigenstrain(thermal),
                1.0,
            );
            // Per-case closed forms: tip UY = g_root + theta_root*L, and
            // N = E*A*(delta/L - eps*) with that case's own pair and strain.
            close(
                case_row(&envelope, id, "disp:tip:uy"),
                uy_mm + ROOT_RZ * L * 1000.0,
                1.0,
            );
            close(case_row(&envelope, id, "disp:tip:ux"), TIP_UX * 1000.0, 1.0);
            let n = axial_force(e, thermal);
            close(
                case_row(&envelope, id, "force:member:midspan:axial"),
                n,
                e * area * 1e-4,
            );
        }
        assert!(
            (axial_force(E_POINT, THERMAL) - axial_force(E_BASE, SECOND_THERMAL)).abs() > 1.0,
            "the two cases are distinguishable"
        );
    }
}

/// Replaces probe P4 (and pins the refusal that mutant K6 removes).
#[test]
fn a_zero_valued_pressure_region_is_not_joined() {
    for mode in MODES {
        let request = with_pressure_region(witness(), 0, 0.0);
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode)
            .unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_not_joined_unavailable(&envelope, &request);
        let attempt =
            &diagnostics_for(&envelope, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", CASE)[0].message;
        assert!(
            attempt.contains("explicitly empty pressure regions"),
            "{attempt}"
        );
    }
}

/// Replaces probe P5: resolved-case perturbations after capture that the live
/// closure accepts (the perturbed invocation is self-consistent) but that
/// captured replay must refuse. Kills mutants K5 (identity without resolver
/// evidence), K10 (identity without the thermal/fit split) and, through the
/// explicit zero motion, K7 (motion-ownership loop removed).
#[test]
fn post_capture_perturbations_accepted_live_are_refused_by_captured_replay() {
    let raw = witness();
    let (_, capture) =
        CapturedInvocation::parse(raw.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
    let genuine = Parts::new(&raw, |_| {});
    let mut selected = source_recovery::solve(genuine.input(), limits()).unwrap();
    assert_eq!(
        capture.check_input(&genuine.input(), &mut selected),
        Ok(None)
    );
    let perturbations: [(&str, fn(&mut ResolvedCase)); 5] = [
        // Same total eigenstrain and operands, a different thermal/fit split.
        ("strain_split_same_total", |s| {
            let m = &mut s.members[0].strain;
            assert_ne!(m.fit_strain, 0.0);
            m.thermal_strain = m.total_eigenstrain;
            m.thermal_stretch = 1.0 + m.total_eigenstrain;
            m.fit_strain = 0.0;
            m.fit_stretch = 1.0;
        }),
        // The resolver's published evidence only; no operand changes.
        ("evidence_only", |s| {
            s.evidence["provenance"] = json!("tampered_after_capture");
        }),
        // An explicit zero motion on a rigid DOF that was an implicit zero.
        ("explicit_zero_motion", |s| {
            assert!(!s.prescribed.contains_key(&ROOT_UX_DOF));
            s.prescribed.insert(ROOT_UX_DOF, 0.0);
        }),
        // The declared ordinary source's effective magnitude.
        ("effective_primitive_magnitude", |s| {
            s.effective_case.primitive_loads[0].magnitude.value *= 2.0;
        }),
        // Poisson's ratio only: G changes, E does not.
        ("nu_only", |s| {
            let member = &mut s.members[0];
            let pair = member.material.pair;
            let changed = IsotropicENu::new(pair.elastic_modulus_pa(), NU_POINT).unwrap();
            assert_ne!(pair.poisson_ratio(), NU_POINT);
            member.material.pair = changed;
            s.pairs.insert(member.pipe_id.clone(), changed);
        }),
    ];
    for (name, perturb) in perturbations {
        let changed = Parts::new(&raw, perturb);
        // The live closure accepts the self-consistent perturbed invocation,
        // so the refusal below is captured replay's alone.
        let mut selected = source_recovery::solve(changed.input(), limits())
            .unwrap_or_else(|e| panic!("{name}: live closure refused: {e:?}"));
        let refused = capture.check_input(&changed.input(), &mut selected);
        assert!(
            refused
                .as_ref()
                .is_err_and(|e| e.0.contains("captured source replay")),
            "{name} accepted by captured replay: {refused:?}"
        );
        // The genuine selection is not current for the perturbed invocation.
        let mut genuine_selected = source_recovery::solve(genuine.input(), limits()).unwrap();
        assert!(
            genuine_selected
                .check_binding_against(changed.input())
                .is_err(),
            "{name}: current binding"
        );
    }
}

/// Motion ownership: a resolved support motion that is not owned by exactly
/// one rigid restrained DOF is refused at source closure, never dropped.
/// Kills mutant K7 (motion-ownership loop removed).
#[test]
fn a_resolved_motion_without_exactly_one_rigid_owner_is_refused_at_source_closure() {
    let raw = witness();
    // Control: the genuine case closes.
    assert!(source_recovery::solve(Parts::new(&raw, |_| {}).input(), limits()).is_ok());
    let unowned: [(&str, usize, f64); 4] = [
        // A free DOF: no support restrains tip UY.
        ("free_dof", TIP_UY_DOF, 1.0e-3),
        // Only the torsion spring owns root RX; a spring is not a rigid owner.
        ("spring_dof", ROOT_RX_DOF, 1.0e-3),
        // An explicit zero on a free DOF is still unowned.
        ("free_dof_zero", TIP_UY_DOF, 0.0),
        // Outside the model's DOF range.
        ("out_of_range", DOF_COUNT, 1.0e-3),
    ];
    for (name, dof, value) in unowned {
        let parts = Parts::new(&raw, |s| {
            assert!(s.prescribed.insert(dof, value).is_none());
        });
        // The actual partition never carries the unowned motion, so without
        // the ownership check it would silently vanish from the solve.
        assert!(parts.prescribed.iter().all(|&(d, _)| d != dof), "{name}");
        let failure = closure_refusal(&parts);
        assert_eq!(failure.stage, "source closure", "{name}");
        assert!(
            matches!(
                failure.error,
                source_recovery::RecoveryError::SourceMismatch(m)
                    if m == "resolved support motion lacks one rigid boundary owner"
            ),
            "{name}: {:?}",
            failure.error
        );
    }
}

/// Replaces probe P6. Kills mutant K8 (collision check removed): the refusal
/// must be the collision itself, at source closure, not a later accident.
#[test]
fn a_primitive_load_colliding_with_the_reserved_eigen_source_id_is_refused_at_closure() {
    let request = with_colliding_primitive(witness());
    // In module: the closure refuses the collision itself.
    let parts = Parts::new(&request, |_| {});
    assert_eq!(parts.eigen.len(), 1);
    assert!(parts
        .resolved
        .effective_case
        .primitive_loads
        .iter()
        .any(|p| p.id == source_recovery::eigen_source_id("member")));
    let failure = closure_refusal(&parts);
    assert_eq!(failure.stage, "source closure");
    assert!(matches!(
        failure.helper_stage,
        exact::functionals::AttemptStage::SourceClosure
    ));
    assert!(
        matches!(
            failure.error,
            source_recovery::RecoveryError::SourceMismatch(m)
                if m == "eigen source identity collides with a primitive load"
        ),
        "{:?}",
        failure.error
    );
    // Public route: the case publishes ordinarily, and the producer's attempt
    // diagnostic names that refusal.
    for mode in MODES {
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode)
            .unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_not_joined_unavailable(&envelope, &request);
        let attempt =
            &diagnostics_for(&envelope, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", CASE)[0].message;
        assert!(attempt.contains("stage: \"source closure\""), "{attempt}");
        assert!(
            attempt.contains("eigen source identity collides with a primitive load"),
            "{attempt}"
        );
    }
}

/// Runs the captured 0.4.0 route's per-case solves for `raw` exactly as
/// `run_linear_static_preview_captured_once` does, returning the finalized
/// cases that the invocation receipt consumes, and the ledger they charged.
fn route_cases(
    raw: &Value,
    mode: PreviewSolverMode,
    capture: &CapturedInvocation,
) -> (Vec<FinalizedSourceBlockCase>, SourceRecoveryBudget) {
    let request: LinearStaticPreviewRequest = serde_json::from_value(raw.clone()).unwrap();
    assert!(request.materials.is_empty());
    let mut model = request.model;
    let mut materials = model.materials.clone();
    let mut d = vec![];
    pressure_runtime::validate_profile(&model, &mut d);
    case_state::resolve::validate_document(&model, false, &mut d);
    resolve_shared_sections(&mut model, &mut d);
    normalize_model_units(&mut model, &mut materials, &mut d);
    let resolved: Vec<_> = model
        .load_cases
        .iter()
        .map(|case| case_state::resolve::resolve_case(&model, &materials, case, &mut d).unwrap())
        .collect();
    assert!(!has_blocking(&d), "{d:?}");
    let first =
        build_model_for_members(&model, &materials, Some(&resolved[0].pairs), &mut d).unwrap();
    let boundary = prepare_boundary(first.nodes.len(), &first.supports);
    let mut budget = SourceRecoveryBudget {
        per_case_limit: PHYSICS_SOURCE_WORK_LIMIT,
        ..Default::default()
    };
    let mut cases = Vec::new();
    for (case, state) in model.load_cases.iter().zip(&resolved) {
        let built =
            build_model_for_members(&model, &materials, Some(&state.pairs), &mut d).unwrap();
        let k = assemble_case_stiffness(&built, &boundary.springs).unwrap();
        let mut solve = solve_load_case(
            &model,
            &built,
            &materials,
            &k,
            &boundary.restrained_dofs,
            &boundary.springs,
            case,
            None,
            mode,
            Some(capture),
            &mut budget,
            Some(state),
            &mut d,
        )
        .unwrap();
        cases.push(solve.source_case.take().expect("finalized case"));
    }
    assert!(!has_blocking(&d), "{d:?}");
    (cases, budget)
}

/// Replaces probe P7, and kills mutant K9 (ordinary-case record binding
/// removed in a joined envelope): the joined `mixed` witness binds its
/// ordinary pressure case's published record, so a record changed between
/// the product and receipt finalization is refused.
#[test]
fn the_mixed_joined_envelope_binds_its_ordinary_pressure_case_record() {
    let raw = mixed_witness();
    for mode in MODES {
        let mut envelope = run_linear_static_preview_value_with_mode(raw.clone(), mode)
            .unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_joined(&envelope, 2);
        let published = envelope.source_block_recovery.take().unwrap();
        let body = &published["body"];
        assert_eq!(
            body["cases"][0]["selected_method"],
            "retained_source_blocks_exact_v1"
        );
        let ordinary_method = if mode == PreviewSolverMode::DenseScrutiny {
            "ordinary_dense_structural_v1"
        } else {
            "ordinary_sparse_structural_v1"
        };
        assert_eq!(body["cases"][1]["selected_method"], ordinary_method);
        assert_eq!(
            body["cases"][1]["ordinary_attempt"]["outcome"],
            "checks_passed"
        );
        let records = envelope.contract_evidence.as_ref().unwrap()["load_reference_states"]
            .as_array()
            .unwrap()
            .clone();
        assert_eq!(records.len(), 2);
        let ordinary_case = records[1]["load_case_id"].as_str().unwrap().to_string();
        assert_eq!(ordinary_case, "case:ordinary-pressure");
        assert_eq!(records[0]["source_recovery"]["status"], "selected");
        assert_eq!(
            records[1]["source_recovery"],
            json!({"status": "not_joined", "code": "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"})
        );
        assert_eq!(records[1]["solve"]["recovery_method"], ordinary_method);
        let not_joined = diagnostics_for(
            &envelope,
            case_state::SOURCE_RECOVERY_NOT_JOINED,
            &ordinary_case,
        );
        assert_eq!(not_joined.len(), 1);
        assert!(not_joined[0]
            .message
            .ends_with("retained_source_attempt=not_required_ordinary_checks_passed"));

        // Re-run the route's per-case finalization and check that it is the
        // product's: the receipt it finalizes is the published receipt.
        let (_, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let (cases, _) = route_cases(&raw, mode, &capture);
        assert_eq!(
            composite::validate_publication(&capture, &envelope, &cases),
            Ok(())
        );
        let (control, mut budget) = route_cases(&raw, mode, &capture);
        let receipt = FinalizedSourceBlockReceipt::finalize_composite(
            &capture,
            &envelope,
            control,
            &mut budget,
        )
        .unwrap_or_else(|e| panic!("{mode:?}: {e:?}"));
        assert_eq!(receipt.into_wire(), published, "{mode:?}: reconstruction");

        // Any change to a published case record after the product and before
        // receipt finalization is refused; the ordinary case's record is not
        // re-derived anywhere else in a joined envelope.
        let mut tampered_records = Vec::new();
        for (index, pointer, value) in [
            (1, "/source_recovery/status", json!("selected")),
            (
                1,
                "/solve/recovery_method",
                json!("retained_source_blocks_exact_v1"),
            ),
            (1, "/members/0/selected_E_pa", json!(E_BASE)),
            (1, "/provenance", json!("tampered_after_product")),
            (0, "/members/0/selected_E_pa", json!(1.0)),
        ] {
            let mut tampered = envelope.clone();
            let slot = tampered.contract_evidence.as_mut().unwrap()["load_reference_states"][index]
                .pointer_mut(pointer)
                .unwrap_or_else(|| panic!("{pointer}"));
            assert_ne!(*slot, value, "{index} {pointer}");
            *slot = value;
            tampered_records.push((index, pointer, tampered));
        }
        for (index, pointer, tampered) in &tampered_records {
            let refused = composite::validate_publication(&capture, tampered, &cases);
            assert!(
                refused
                    .as_ref()
                    .is_err_and(|e| e.0.contains("case load/reference-state record changed")),
                "{mode:?} record {index} {pointer}: {refused:?}"
            );
        }
        // Through the complete receipt finalization as well.
        let (fresh, mut budget) = route_cases(&raw, mode, &capture);
        let (_, _, tampered) = &tampered_records[0];
        assert!(
            FinalizedSourceBlockReceipt::finalize_composite(&capture, tampered, fresh, &mut budget)
                .is_err(),
            "{mode:?}: finalized a changed ordinary-case record"
        );
    }
}

/// Joined (value route, captured) against ordinary (typed route, no
/// capture): every shared row outside the soft torsion block agrees to 1e-6
/// relative, with a floor of 1e-9 of the row unit's largest magnitude (the
/// reviewer's comparison and tolerance).
fn assert_joined_rows_agree_with_ordinary(name: &str, request: &Value) {
    for mode in MODES {
        let joined = run_linear_static_preview_value_with_mode(request.clone(), mode)
            .unwrap_or_else(|e| panic!("{name} {mode:?}: {e}"));
        assert_joined(&joined, 1);
        let typed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
        let ordinary = run_linear_static_preview_with_mode(typed, mode);
        assert_eq!(
            ordinary.producer.semantic_contract_id,
            case_state::LOAD_REFERENCE_SEMANTIC_CONTRACT_ID
        );
        assert!(ordinary.source_block_recovery.is_none());
        let rows: HashMap<_, _> = ordinary
            .results
            .iter()
            .map(|r| (r.id.clone(), r.value))
            .collect();
        let mut scale: HashMap<String, f64> = HashMap::new();
        for r in &joined.results {
            let s = scale.entry(r.unit.clone()).or_insert(0.0);
            *s = s.max(r.value.abs());
        }
        let mut compared = 0;
        for r in &joined.results {
            let ordinary = *rows
                .get(&r.id)
                .unwrap_or_else(|| panic!("{name} {mode:?}: {} not ordinary", r.id));
            if r.kind.contains("rotation_x")
                || r.kind.contains("torsion")
                || r.kind.contains("moment_x")
            {
                continue;
            }
            compared += 1;
            let unit_scale = if r.unit == "N*m" {
                scale[&r.unit].max(2.0 * scale.get("N").copied().unwrap_or(0.0))
            } else {
                scale[&r.unit]
            };
            let floor = 1e-9 * unit_scale;
            let larger = r.value.abs().max(ordinary.abs());
            let relative = if larger <= floor {
                0.0
            } else {
                (r.value - ordinary).abs() / larger.max(floor * 1e3)
            };
            assert!(
                relative <= 1e-6,
                "{name} {mode:?}: {} {} joined={:e} ordinary={ordinary:e}",
                r.id,
                r.kind,
                r.value
            );
        }
        assert!(compared > 70, "{name} {mode:?}: compared {compared}");
    }
}

/// Replaces probe P11.
#[test]
fn joined_rows_agree_with_the_ordinary_route_and_closed_forms() {
    assert_joined_rows_agree_with_ordinary("eigen_motion", &witness());
    assert_joined_rows_agree_with_ordinary(
        "exact_point_pair",
        &with_exact_point(witness(), E_POINT, NU_POINT),
    );

    // Factor 2.5 on the torque, plus a stored 7 kN tip force that no load
    // source references (excluded).
    const FACTOR: f64 = 2.5;
    let mut factored = witness();
    let case = &mut factored["model"]["load_cases"][0];
    case["analysis_state"]["load_sources"][0]["factor"] = json!(FACTOR);
    let mut excluded = case["primitive_loads"][0].clone();
    excluded["id"] = json!("excluded:force");
    excluded["category"] = json!("concentrated_force");
    excluded["dimension"] = json!("force");
    excluded["direction"] = json!("UZ");
    excluded["magnitude"] = json!({"value": 7.0e3, "unit": "N"});
    excluded["provenance"] = json!(INVENTED);
    case["primitive_loads"]
        .as_array_mut()
        .unwrap()
        .push(excluded);
    assert_joined_rows_agree_with_ordinary("factored_with_excluded", &factored);
    for mode in MODES {
        let envelope = run_linear_static_preview_value_with_mode(factored.clone(), mode).unwrap();
        assert_joined(&envelope, 1);
        // Root RX = factor*T/k_spring; the excluded force moves nothing.
        close(
            case_row(&envelope, CASE, "disp:root:rx"),
            FACTOR * TORQUE / TORSION_SPRING,
            1.0,
        );
        let uz = case_row(&envelope, CASE, "disp:tip:uz");
        assert!(uz.abs() <= 1e-12, "tip UZ {uz:e} mm");
        let record = &envelope.contract_evidence.as_ref().unwrap()["load_reference_states"][0];
        let torque = record["contributions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["owner_kind"] == "stored_primitive")
            .unwrap();
        assert_eq!(torque["source_id"], "torque");
        assert_eq!(torque["factor"], json!(FACTOR));
        close(
            torque["applied_magnitude"].as_f64().unwrap(),
            FACTOR * TORQUE,
            TORQUE,
        );
        assert!(!record["contributions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["source_id"] == "excluded:force"));
        assert!(
            record["excluded_sources"]
                .to_string()
                .contains("excluded:force"),
            "{}",
            record["excluded_sources"]
        );
    }

    // A parallel grounded spring on the moving rigid root UY (g = 1 mm).
    const K_UY: f64 = 1.0e3;
    let mut parallel = witness();
    parallel["model"]["supports"]
        .as_array_mut()
        .unwrap()
        .push(json!({
        "id": "spring:uy", "node": "root", "family": "spring", "restraints": ["UY"],
        "stiffness": {"dof": "UY", "value": {"value": K_UY, "unit": "N/m"}},
        "provenance": INVENTED}));
    parallel["model"]["load_cases"][0]["analysis_state"]["support_states"]
        .as_array_mut()
        .unwrap()
        .push(
            json!({"support_ref": "spring:uy", "participation": {"kind": "active_model_device"}}),
        );
    assert_joined_rows_agree_with_ordinary("parallel_spring_on_moving_rigid_dof", &parallel);
    let spring_force = -K_UY * ROOT_UY_MM / 1000.0;
    assert_eq!(spring_force, -1.0);
    for mode in MODES {
        let envelope = run_linear_static_preview_value_with_mode(parallel.clone(), mode).unwrap();
        assert_joined(&envelope, 1);
        // The rigidly moved member carries no shear, so the anchor balances
        // the spring: spring -k*g = -1 N, anchor +1 N.
        close(
            case_row(&envelope, CASE, ":spring:uy:Fy"),
            spring_force,
            1.0,
        );
        close(case_row(&envelope, CASE, ":anchor:Fy"), -spring_force, 1.0);
        close(case_row(&envelope, CASE, "disp:root:uy"), ROOT_UY_MM, 1.0);
    }
}
