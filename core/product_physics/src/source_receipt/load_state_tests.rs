//! Implementer tests for the resolved load/reference-state retained-source
//! join. Expected mechanics are closed-form and independent of the producer;
//! all inputs are invented. Negative controls perturb one resolved-case operand
//! after capture and require receipt replay or source closure to refuse.
use super::*;
use crate::case_state::resolve::ResolvedCase;
use crate::pressure_exact::IsotropicENu;
use open_pipe_stress_primitive_loads::LoadApplication;

const L: f64 = 2.0;
const E: f64 = 2.0e11;
const NU: f64 = 0.25;
const THERMAL: f64 = 6.0e-5;
const FIT: f64 = 4.0e-5;
const TIP_UX: f64 = 5.0e-5;
const ROOT_UY: f64 = 1.0e-3;
const ROOT_RZ: f64 = 1.0e-4;
const TORQUE: f64 = 1.0e-8;
const SPRING: f64 = 1.0e-4;
const ROOT_UY_DOF: usize = 1;

/// A single straight member whose torsion block is ordinarily sensitive (soft
/// grounded torsion spring), carrying a thermal and fit eigenstrain, a tip
/// axial stop with prescribed motion and prescribed rigid root motion. The
/// request is the maintained witness fixture; `pressure` adds a region, which
/// retained source refuses.
fn witness(pressure: bool) -> Value {
    let mut request: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/load_reference_source/eigen_motion.request.json"
    ))
    .unwrap();
    let material = &request["model"]["materials"][0];
    assert_eq!(material["elastic_modulus"]["value"], json!(E));
    assert_eq!(material["poisson_ratio"]["value"], json!(NU));
    let case = &mut request["model"]["load_cases"][0];
    // The fixture's authored constants are these tests' closed-form inputs.
    assert_eq!(
        case["analysis_state"]["element_states"][0]["thermal_state"]["strain"]["value"],
        json!(THERMAL)
    );
    assert_eq!(
        case["primitive_loads"][0]["magnitude"]["value"],
        json!(TORQUE)
    );
    if pressure {
        let p = json!("invented_load_reference_join_control_not_library_data");
        case["pressure_regions"] = json!([{"id": "region:join", "member_pipe_ids": ["member"],
            "pressure_basis": "internal_differential_zero_external_v1", "pressure": {"value": 1.0e-5, "unit": "Pa"},
            "terminals": [{"node_ref": "root", "closure_transfer": "transfers_to_wall", "provenance": p},
                {"node_ref": "tip", "closure_transfer": "transfers_to_wall", "provenance": p}], "provenance": p}]);
    }
    assert_eq!(
        request["model"]["reference_configurations"][0]["member_references"][0]["fit"]["strain"]
            ["value"],
        json!(FIT)
    );
    request
}

/// The product route's own construction of one resolved case, with an
/// optional perturbation of the resolved case before the build.
struct Parts {
    model: PreviewModel,
    materials: Vec<MaterialInput>,
    restrained: Vec<usize>,
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
        Self {
            materials,
            restrained: boundary.restrained_dofs.clone(),
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
        self.input_with(&self.resolved, &self.eigen, &self.prescribed)
    }
    fn input_with<'a>(
        &'a self,
        resolved: &'a ResolvedCase,
        eigen: &'a [ThermalElementLoad],
        prescribed: &'a [(usize, f64)],
    ) -> source_recovery::Input<'a> {
        source_recovery::Input {
            model: &self.model,
            built: &self.built,
            stiffness: &self.k,
            force: &self.f,
            free: &self.free,
            prescribed,
            spring_entries: &self.springs,
            load_case: &resolved.effective_case,
            load_application: &self.loads,
            thermal_loads: eigen,
            pressure_thrust_loads: &[],
            load_state: Some(resolved),
        }
    }
}

fn limits() -> exact::Limits {
    exact::Limits {
        operations: PHYSICS_SOURCE_WORK_LIMIT,
        ..Default::default()
    }
}

fn perturb_pair(state: &mut ResolvedCase) {
    let member = &mut state.members[0];
    let pair = member.material.pair;
    let changed = IsotropicENu::new(
        pair.elastic_modulus_pa() * (1.0 + 1.0e-6),
        pair.poisson_ratio(),
    )
    .unwrap();
    member.material.pair = changed;
    state.pairs.insert(member.pipe_id.clone(), changed);
}
fn perturb_eigenstrain(state: &mut ResolvedCase) {
    state.members[0].strain.total_eigenstrain *= 1.0 + 1.0e-6;
}
fn perturb_motion(state: &mut ResolvedCase) {
    *state.prescribed.get_mut(&ROOT_UY_DOF).unwrap() += 1.0e-9;
}
fn flip_offset_sign(state: &mut ResolvedCase) {
    state.members[0].strain.total_eigenstrain = -state.members[0].strain.total_eigenstrain;
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

#[test]
fn unperturbed_capture_replays_and_binds_the_selected_joined_response() {
    let raw = witness(false);
    let (_, capture) =
        CapturedInvocation::parse(raw.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
    let parts = Parts::new(&raw, |_| {});
    assert_eq!(parts.eigen.len(), 1);
    let mut selected = source_recovery::solve(parts.input(), limits()).unwrap();
    assert_eq!(capture.check_input(&parts.input(), &mut selected), Ok(None));
    assert!(selected.check_binding_against(parts.input()).is_ok());
    // Eigen load E*A*eps* enters the force once as equal/opposite axial actions.
    let axial = parts.eigen[0].axial_load;
    assert_eq!(parts.f[0], -axial);
    assert_eq!(parts.f[6], axial);
    // The member-end axial rows carry the exact eigen offset: N = k*delta - a.
    let area = std::f64::consts::PI * (0.1_f64 * 0.1 - 0.09 * 0.09);
    let eps = (1.0 + THERMAL) * (1.0 + FIT) - 1.0;
    let n = E * area * (TIP_UX / L - eps);
    let member = &selected.members()[0];
    close(member.end_forces[0], -n, E * area * 1e-4);
    close(member.end_forces[6], n, E * area * 1e-4);
    let displacement = selected.displacements();
    close(displacement[6], TIP_UX, TIP_UX);
    close(displacement[7], ROOT_UY + ROOT_RZ * L, ROOT_UY);
    close(displacement[11], ROOT_RZ, ROOT_RZ);
    close(displacement[3], TORQUE / SPRING, TORQUE / SPRING);
    let reactions = selected.reactions();
    close(reactions[0], -n, E * area * 1e-4);
    close(reactions[6], n, E * area * 1e-4);
    // Rigid prescribed root motion carries no shear or moment.
    for dof in [1, 2, 4, 5] {
        close(reactions[dof], 0.0, E * area * 1e-4);
    }
}

#[test]
fn a_resolved_operand_perturbed_after_capture_is_refused_by_receipt_replay() {
    let raw = witness(false);
    let (_, capture) =
        CapturedInvocation::parse(raw.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
    let original = Parts::new(&raw, |_| {});
    let perturbations: [(&str, fn(&mut ResolvedCase)); 4] = [
        ("member_pair", perturb_pair),
        ("eigenstrain", perturb_eigenstrain),
        ("prescribed_motion", perturb_motion),
        ("offset_sign", flip_offset_sign),
    ];
    for (name, perturb) in perturbations {
        // A self-consistent live invocation built from the perturbed case
        // selects its own response, but the captured request resolves
        // differently: captured replay must refuse it.
        let changed = Parts::new(&raw, perturb);
        let mut selected = source_recovery::solve(changed.input(), limits())
            .unwrap_or_else(|e| panic!("{name}: {e:?}"));
        let refused = capture.check_input(&changed.input(), &mut selected);
        assert!(
            refused
                .as_ref()
                .is_err_and(|e| e.0.contains("captured source replay")),
            "{name}: {refused:?}"
        );
        // The genuine selection does not bind to the perturbed invocation.
        let mut genuine = source_recovery::solve(original.input(), limits()).unwrap();
        assert!(
            genuine.check_binding_against(changed.input()).is_err(),
            "{name}: current binding"
        );
    }
}

#[test]
fn source_closure_refuses_live_resolved_operands_that_did_not_reach_formation() {
    let raw = witness(false);
    let parts = Parts::new(&raw, |_| {});
    let refused = |input: source_recovery::Input<'_>| match source_recovery::solve(input, limits())
    {
        Err(failure) => failure.error,
        Ok(_) => panic!("closure accepted a changed operand"),
    };
    let mismatch = |error: source_recovery::RecoveryError, text: &str| {
        assert!(
            matches!(&error, source_recovery::RecoveryError::SourceMismatch(m) if m.contains(text)),
            "{error:?} lacks {text}"
        );
    };
    let pair = Parts::new(&raw, perturb_pair);
    mismatch(
        refused(parts.input_with(&pair.resolved, &parts.eigen, &parts.prescribed)),
        "did not reach formation",
    );
    let strain = Parts::new(&raw, perturb_eigenstrain);
    mismatch(
        refused(parts.input_with(&strain.resolved, &parts.eigen, &parts.prescribed)),
        "eigen element loads differ",
    );
    let mut flipped = parts.eigen.clone();
    flipped[0].axial_load = -flipped[0].axial_load;
    mismatch(
        refused(parts.input_with(&parts.resolved, &flipped, &parts.prescribed)),
        "eigen element loads differ",
    );
    let motion = Parts::new(&raw, perturb_motion);
    mismatch(
        refused(parts.input_with(&motion.resolved, &parts.eigen, &parts.prescribed)),
        "prescribed partition",
    );
    let mut moved = parts.prescribed.clone();
    moved
        .iter_mut()
        .find(|(dof, _)| *dof == ROOT_UY_DOF)
        .unwrap()
        .1 += 1.0e-9;
    mismatch(
        refused(parts.input_with(&parts.resolved, &parts.eigen, &moved)),
        "prescribed partition",
    );
    // A 0.4.0 invocation never omits its resolved case, and an eigen load
    // never enters without one.
    let mut orphan = parts.input();
    orphan.load_state = None;
    mismatch(refused(orphan), "ownership");
}

#[test]
fn joined_publication_uses_its_own_semantics_profile_policy_and_case_record() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let envelope = run_linear_static_preview_value_with_mode(witness(false), mode).unwrap();
        assert_eq!(
            envelope.producer.semantic_contract_id,
            case_state::LOAD_REFERENCE_SOURCE_SEMANTIC_CONTRACT_ID
        );
        assert_eq!(
            envelope.formulation_basis.profile_id,
            case_state::LOAD_STATE_SOURCE_PROFILE_ID
        );
        let receipt = envelope.source_block_recovery.as_ref().unwrap();
        assert_eq!(receipt["body"]["policy"], "LOAD-REFERENCE-SOURCE-1");
        assert_eq!(receipt["body"]["status"], "qualified");
        assert_eq!(
            receipt["body"]["cases"][0]["selected_method"],
            "retained_source_blocks_exact_v1"
        );
        let evidence = envelope.contract_evidence.as_ref().unwrap();
        let record = &evidence["load_reference_states"][0];
        assert_eq!(
            record["source_recovery"],
            json!({"status": "selected", "method": "retained_source_blocks_exact_v1"})
        );
        assert_eq!(
            record["solve"]["recovery_method"],
            "retained_source_blocks_exact_v1"
        );
        assert_eq!(
            evidence["exact_cases"][0]["material_basis"],
            "resolved_per_member_load_reference_state_v1"
        );
        assert!(!envelope
            .diagnostics
            .iter()
            .any(|d| d.code == case_state::SOURCE_RECOVERY_NOT_JOINED));
        // Published rows are the retained projections (mm for translations).
        let row = |kind: &str, entity: &str| {
            envelope
                .results
                .iter()
                .find(|r| r.kind == kind && r.entity_ref == entity)
                .unwrap_or_else(|| panic!("{kind} {entity}"))
                .value
        };
        close(
            row("global_nodal_displacement_x", "tip"),
            TIP_UX * 1000.0,
            1.0,
        );
        close(
            row("global_nodal_displacement_y", "tip"),
            (ROOT_UY + ROOT_RZ * L) * 1000.0,
            1.0,
        );
    }
}

#[test]
fn an_unavailable_join_keeps_the_ordinary_load_reference_publication() {
    // Retained source refuses pressure regions; the case stays ordinary.
    let envelope =
        run_linear_static_preview_value_with_mode(witness(true), PreviewSolverMode::DenseScrutiny)
            .unwrap();
    assert_eq!(
        envelope.producer.semantic_contract_id,
        case_state::LOAD_REFERENCE_SEMANTIC_CONTRACT_ID
    );
    assert_eq!(
        envelope.formulation_basis.profile_id,
        case_state::LOAD_STATE_PROFILE_ID
    );
    assert!(envelope.source_block_recovery.is_none());
    let record = &envelope.contract_evidence.as_ref().unwrap()["load_reference_states"][0];
    assert_eq!(
        record["source_recovery"],
        json!({"status": "not_joined", "code": "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"})
    );
    let codes: Vec<_> = envelope
        .diagnostics
        .iter()
        .map(|d| d.code.as_str())
        .collect();
    assert!(codes.contains(&"SOURCE_BLOCK_RECOVERY_UNAVAILABLE"));
    let not_joined: Vec<_> = envelope
        .diagnostics
        .iter()
        .filter(|d| d.code == case_state::SOURCE_RECOVERY_NOT_JOINED)
        .collect();
    assert_eq!(not_joined.len(), 1);
    assert!(not_joined[0]
        .message
        .contains("retained_source_attempt=unavailable"));
}

#[test]
fn physical_binding_refuses_a_changed_published_load_reference_record_or_member_material() {
    let raw = witness(false);
    let mode = PreviewSolverMode::SparseInteractive;
    let envelope = run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap();
    let (_, capture) = CapturedInvocation::parse(raw, mode).unwrap();
    let replay = capture.captured_load_state_case("case:join").unwrap();
    let evidence = envelope.contract_evidence.as_ref().unwrap();
    let exact_case = evidence["exact_cases"][0].clone();
    let record = evidence["load_reference_states"][0].clone();
    let rows: Vec<_> = envelope
        .results
        .iter()
        .filter(|r| {
            r.basis_ref
                .as_ref()
                .is_some_and(|b| b.ref_id == "case:join")
        })
        .cloned()
        .collect();
    let check = |exact_case: &Value, record: Option<&Value>| {
        composite::physical_source_built(
            &capture,
            &replay.model,
            &replay.materials,
            &replay.built,
            None,
            "case:join",
            exact_case,
            &[],
            &rows,
            true,
            Some((&replay.resolved, record)),
        )
    };
    assert_eq!(check(&exact_case, Some(&record)), Ok(()));
    assert!(check(&exact_case, None).is_err(), "record omitted");
    let owner = |kind: &str| {
        record["contributions"]
            .as_array()
            .unwrap()
            .iter()
            .position(|c| c["owner_kind"] == kind)
            .unwrap_or_else(|| panic!("{kind}"))
    };
    let mut changed: Vec<Value> = Vec::new();
    for pointer in [
        "/source_recovery/status".to_string(),
        "/solve/recovery_method".to_string(),
        "/members/0/selected_E_pa".to_string(),
        "/members/0/total_eigenstrain".to_string(),
        format!("/contributions/{}/value", owner("resolved_member_state")),
        format!("/contributions/{}/value", owner("support_state")),
        format!("/contributions/{}/factor", owner("stored_primitive")),
    ] {
        let pointer = pointer.as_str();
        let mut tampered = record.clone();
        let slot = tampered
            .pointer_mut(pointer)
            .unwrap_or_else(|| panic!("{pointer}"));
        *slot = match slot {
            Value::Number(n) => json!(n.as_f64().unwrap() * 2.0),
            _ => json!("tampered"),
        };
        changed.push(tampered);
    }
    for tampered in &changed {
        assert!(check(&exact_case, Some(tampered)).is_err(), "{tampered}");
    }
    for (pointer, value) in [
        ("/material_basis", json!("base_material_common_E_nu")),
        ("/pipe_materials/0/E_pa", json!(E * 2.0)),
        ("/pipe_materials/0/resolved_eigenstrain", json!(0.0)),
    ] {
        let mut tampered = exact_case.clone();
        *tampered.pointer_mut(pointer).unwrap() = value;
        assert!(check(&tampered, Some(&record)).is_err(), "{pointer}");
    }
}

#[test]
fn a_resolved_eigenload_without_its_built_section_blocks_instead_of_disappearing() {
    let mut parts = Parts::new(&witness(false), |_| {});
    parts.built.sections.remove("member");
    assert_eq!(
        load_state_eigen_loads(&parts.resolved, &parts.built).unwrap_err(),
        "member"
    );
    let mut diagnostics = Vec::new();
    let solve = solve_load_case(
        &parts.model,
        &parts.built,
        &parts.materials,
        &parts.k,
        &parts.restrained,
        &parts.springs,
        &parts.model.load_cases[0],
        None,
        PreviewSolverMode::DenseScrutiny,
        None,
        &mut SourceRecoveryBudget::default(),
        Some(&parts.resolved),
        &mut diagnostics,
    )
    .unwrap();
    assert!(solve.results.is_empty());
    assert!(diagnostics
        .iter()
        .any(|d| d.code == "LOAD_STATE_MEMBER_SECTION_MISSING" && d.severity == "blocking"));
}
