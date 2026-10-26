//! K-D5 on the converged active set (DEL-04-01 Design
//! kd5-nonlinear-active-set.md, revision 1): one confirmation attempt after
//! the loop has converged and the product qualification has passed, the
//! coupled derived-normal friction force (k = 1, Stage A), and the active-set
//! certificate (§2). It is never called from the loop, whose iterations keep
//! `solve_binary64` (S11-K, ROOT option (c)); R2 supersedes ROOT's selection
//! ruling for this single attempt only.
use crate::structural_adapter::{AssemblyEvidence, ConfirmationContext};
use crate::{
    AppliedSlidingFrictionForce, LinearSolveMode, NonlinearFrameSolveInput,
    NonlinearFrameSolveResult, NonlinearIntegrationError,
};
use open_pipe_stress_curved_bend::CurvedBendMacroElement;
use open_pipe_stress_frame_kernel::load_ledger::{AssembledForce, Formation, LoadLedger};
use open_pipe_stress_frame_kernel::structural::{
    certify_load_primitive, CertifiedRow, CertifiedValue, FormationCheck, FormationCheckReason,
    LoadFormula, LoadPrimitive, LoadSummandIdentity, MemberBoundInputs, SolveQuality,
};
use open_pipe_stress_frame_kernel::{node_dof_index, FrameDof};
use open_pipe_stress_nonlinear_supports::{
    ActivationSense, ActiveSetState, NonlinearSupport, NonlinearSupportBehavior,
};
use std::collections::HashMap;

/// Closure family of the sliding-friction summands (§1.6).
pub const SLIDING_FRICTION_FAMILY: &str = "nonlinear_sliding_friction";

/// One nonlinear support's published slot row (§3.1): the certified reaction
/// (active, sticking), the certified friction force (sliding, mu > 0), or an
/// exact zero (inactive, sliding with mu = 0).
#[derive(Debug, Clone, Copy)]
pub struct ConfirmedRow {
    pub global_dof: usize,
    pub row: CertifiedRow,
}

/// A Passed confirmation: Stage B's recovery inputs and force, and the
/// certified nonlinear rows.
#[derive(Debug)]
pub struct ConfirmedState {
    pub member_bound: MemberBoundInputs,
    /// The ledger force of Stage B: the case ledger plus one formed term per
    /// sliding support with mu > 0 (§1.6).
    pub force: AssembledForce,
    /// Every nonlinear support's slot row, by support id.
    pub supports: HashMap<String, ConfirmedRow>,
    /// Each derived-normal friction support's source reaction (Stage B), by
    /// the friction support's id.
    pub derived_normals: HashMap<String, ConfirmedRow>,
}

#[derive(Debug)]
pub enum Confirmation {
    Established(Box<ConfirmedState>),
    /// The case fails closed with this named reason (§2.4).
    NotEstablished { reason: String },
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Normal {
    Explicit(f64),
    Derived { source_dof: usize },
}

/// One sliding support with mu > 0, as the converged state enumerates it.
#[derive(Debug, Clone)]
struct SlidingTerm {
    support_id: String,
    global_dof: usize,
    coefficient: f64,
    normal: Normal,
    /// The expected slip sign: the sign of the loop's final u_t (§1.6).
    direction: f64,
}

/// Single entry (R2): `confirm_converged_state` holds the nonlinear route's
/// only call to the adapter's formation-check entry.
pub fn confirm_converged_state(
    input: &NonlinearFrameSolveInput,
    springs: &[(usize, f64)],
    solve: &NonlinearFrameSolveResult,
    force: &AssembledForce,
    curved_sources: &[CurvedBendMacroElement],
    // T4-U8 (DEL-05-03 Design §4 item 2): each of `input.elements`' member
    // identity (`AssemblyEvidence::with_frame_members`).
    frame_members: &[String],
    mode: LinearSolveMode,
) -> Result<Confirmation, NonlinearIntegrationError> {
    let applied = solve
        .iterations
        .last()
        .map(|last| last.applied_sliding_friction_forces.clone())
        .unwrap_or_default();
    confirm_with_applied(input, springs, solve, force, curved_sources, frame_members, mode, &applied)
}

fn not_established(reason: impl Into<String>) -> Result<Confirmation, NonlinearIntegrationError> {
    Ok(Confirmation::NotEstablished {
        reason: reason.into(),
    })
}

fn is_translational(dof: FrameDof) -> bool {
    matches!(dof, FrameDof::Ux | FrameDof::Uy | FrameDof::Uz)
}

fn support_dof(support: &NonlinearSupport) -> usize {
    node_dof_index(support.node_index, support.dof)
}

/// The supplied side of the closure takes the loop's applied-force evidence;
/// tests pass a mutated list to exercise the closure (§6).
#[allow(clippy::too_many_arguments)]
pub(crate) fn confirm_with_applied(
    input: &NonlinearFrameSolveInput,
    springs: &[(usize, f64)],
    solve: &NonlinearFrameSolveResult,
    force: &AssembledForce,
    curved_sources: &[CurvedBendMacroElement],
    // T4-U8 (DEL-05-03 Design §4 item 2): each of `input.elements`' member
    // identity (`AssemblyEvidence::with_frame_members`).
    frame_members: &[String],
    mode: LinearSolveMode,
    applied: &[AppliedSlidingFrictionForce],
) -> Result<Confirmation, NonlinearIntegrationError> {
    crate::require_assembled_force(input, force)?;
    if input
        .nonlinear_supports
        .iter()
        .any(|s| matches!(s.behavior, NonlinearSupportBehavior::Gap { .. }))
    {
        // R1: gap-bearing cases keep their own branches; they are never confirmed.
        return not_established("confirmation attempt: gap supports keep their recovery-basis branch");
    }
    if !crate::product_selected_state_is_qualified(input, springs, solve)? {
        return not_established("confirmation attempt: the converged state is not qualified");
    }
    let (Some(receipt), Some(last)) = (solve.completion.as_ref(), solve.iterations.last()) else {
        return not_established("confirmation attempt: the converged state is not qualified");
    };
    let u = &solve.displacements;
    let n = u.len();
    let prescribed = last
        .active_restrained_dofs
        .iter()
        .copied()
        .zip(last.active_prescribed_displacements.iter().copied())
        .collect::<Vec<_>>();
    // §2.4 item 2: the loop's u against the state's prescribed map, bit for bit.
    if prescribed_row_mismatch(u, &prescribed) {
        return not_established("prescribed row mismatch");
    }
    // §2.4 item 2, revision 2 (W-16 stage 2): that map against the case's
    // motion, built here independently of the loop's boundary, bit for bit.
    if prescribed_map_mismatch(
        intended_prescribed_map(input, &solve.final_states).as_deref(),
        &prescribed,
    ) {
        return not_established("prescribed row mismatch");
    }
    let free = (0..n)
        .filter(|dof| !last.active_restrained_dofs.contains(dof))
        .collect::<Vec<_>>();

    // §2.4 item 3: preconditions, per support in support-id order.
    let states = solve
        .final_states
        .iter()
        .map(|state| (state.support_id.as_str(), state.state))
        .collect::<HashMap<_, _>>();
    let explicit = input
        .friction_normal_reactions
        .iter()
        .map(|r| (r.support_id.as_str(), r.normal_reaction))
        .collect::<HashMap<_, _>>();
    let derived = input
        .derived_friction_normal_reactions
        .iter()
        .map(|r| {
            (
                r.support_id.as_str(),
                node_dof_index(r.source_node_index, r.source_dof),
            )
        })
        .collect::<HashMap<_, _>>();
    let mut supports = input.nonlinear_supports.iter().collect::<Vec<_>>();
    supports.sort_by(|a, b| a.support_id.cmp(&b.support_id));
    for support in &supports {
        let id = &support.support_id;
        let dof = support_dof(support);
        if input.base_restrained_dofs.contains(&dof)
            || supports
                .iter()
                .any(|other| other.support_id != *id && support_dof(other) == dof)
        {
            return not_established(format!("support {id}: shares its DOF with another restraint"));
        }
        let Some(&state) = states.get(id.as_str()) else {
            return not_established("confirmation attempt: the converged state is not qualified");
        };
        if matches!(support.behavior, NonlinearSupportBehavior::Friction) {
            if state == ActiveSetState::Active {
                return not_established(format!(
                    "support {id}: friction state active has no confirmation"
                ));
            }
            if !is_translational(support.dof) {
                return not_established(format!(
                    "support {id}: rotational friction has no confirmation"
                ));
            }
            if let Some(&source) = derived.get(id.as_str()) {
                if !input.base_restrained_dofs.contains(&source) {
                    return not_established(format!(
                        "support {id}: friction normal source is not a base restraint"
                    ));
                }
            }
        }
        // An active or sticking support's DOF is a prescribed row of the state
        // (§1.1, C); the reaction ball is formed there (review M6).
        if matches!(state, ActiveSetState::Active | ActiveSetState::Sticking)
            && !last.active_restrained_dofs.contains(&dof)
        {
            return not_established(format!("support {id}: constrained state is not a prescribed row"));
        }
    }
    // The sliding set L (mu > 0), enumerated from the converged state (§1.6).
    let mut sliding = Vec::new();
    for support in &supports {
        let mu = support.friction_coefficient.unwrap_or(0.0);
        if !matches!(support.behavior, NonlinearSupportBehavior::Friction)
            || states.get(support.support_id.as_str()) != Some(&ActiveSetState::Sliding)
            || !(mu > 0.0)
        {
            continue;
        }
        let dof = support_dof(support);
        let normal = match (
            explicit.get(support.support_id.as_str()),
            derived.get(support.support_id.as_str()),
        ) {
            (Some(&value), None) => Normal::Explicit(value),
            (None, Some(&source_dof)) => Normal::Derived { source_dof },
            _ => {
                return not_established(format!(
                    "support {}: friction normal not established",
                    support.support_id
                ))
            }
        };
        sliding.push(SlidingTerm {
            support_id: support.support_id.clone(),
            global_dof: dof,
            coefficient: mu,
            normal,
            direction: sign_or_zero(u[dof]),
        });
    }
    let coupled = sliding
        .iter()
        .filter(|t| matches!(t.normal, Normal::Derived { .. }))
        .collect::<Vec<_>>();
    if coupled.len() > 1 {
        return not_established(format!(
            "coupled friction with {} derived-normal sliding supports is not supported",
            coupled.len()
        ));
    }
    let coupled = coupled.first().map(|t| (*t).clone());
    let phi0 = coupled.as_ref().map(|t| applied_force(applied, &t.support_id));

    // Stage A's force (k = 1) or Stage B's (k = 0) gates the attempt.
    let gate_force = match (&coupled, phi0) {
        (Some(term), Some(Some(phi0))) => {
            force.with_appended(appended_ledger(&sliding, applied, Some(StageA { term, phi0 }), None))
        }
        (Some(term), _) => {
            return not_established(format!(
                "formation check: stage A: load closure: {}: missing applied force",
                term.support_id
            ))
        }
        (None, _) => force.with_appended(appended_ledger(&sliding, applied, None, None)),
    }
    .map_err(|error| NonlinearIntegrationError::InvalidInput {
        detail: format!("confirmation force: {error}"),
    })?;

    let evidence = AssemblyEvidence::new(
        input.node_count,
        &input.elements,
        &input.connectors,
        &input.curved_bend_elements,
        springs,
    )?
    .with_frame_members(frame_members);
    let attempt = evidence.confirm_with_formation_check(
        &receipt.original_stiffness,
        &gate_force,
        &free,
        &prescribed,
        mode,
        curved_sources,
        |gate, context| {
            if gate.report.quality != SolveQuality::Passed || gate.load_fidelity.is_some() {
                return Confirmation::NotEstablished {
                    reason: format!(
                        "confirmation attempt: gate Sensitive (reciprocal condition estimate {:?})",
                        gate.report.reciprocal_condition_estimate
                    ),
                };
            }
            let (coupled_ball, stage_b) = match (&coupled, phi0) {
                (Some(term), Some(Some(phi0))) => {
                    let ball = match stage_a(context, &gate_force, term, phi0, u, n) {
                        Ok(ball) => ball,
                        Err(reason) => return Confirmation::NotEstablished { reason },
                    };
                    (
                        Some(ball),
                        force.with_appended(appended_ledger(&sliding, applied, None, Some(ball))),
                    )
                }
                _ => (None, force.with_appended(appended_ledger(&sliding, applied, None, None))),
            };
            let Ok(stage_b) = stage_b else {
                return Confirmation::NotEstablished {
                    reason: "friction coupling not established".into(),
                };
            };
            let bound = match context.evaluate_at(&stage_b, u, true) {
                Ok((None, Some(bound))) => bound,
                Ok((check, _)) => {
                    return Confirmation::NotEstablished {
                        reason: format!("formation check: stage B: {}", render_check(check.as_ref())),
                    }
                }
                Err(error) => {
                    return Confirmation::NotEstablished {
                        reason: format!("formation check: stage B: {error}"),
                    }
                }
            };
            certify_state(
                context,
                &supports,
                &states,
                &explicit,
                &derived,
                &sliding,
                applied,
                coupled_ball,
                stage_b,
                bound,
                u,
            )
        },
    );
    match attempt {
        Ok(confirmation) => Ok(confirmation),
        Err(error) => not_established(format!("confirmation attempt: {error}")),
    }
}

/// §2.4 item 2: a constrained DOF whose published displacement is not its
/// prescribed value bit for bit (a -0.0 against a +0.0 is a mismatch).
pub(crate) fn prescribed_row_mismatch(u: &[f64], prescribed: &[(usize, f64)]) -> bool {
    prescribed
        .iter()
        .any(|&(dof, value)| dof >= u.len() || u[dof].to_bits() != value.to_bits())
}

/// §2.4 item 2, revision 2: the intended prescribed map of the case's motion
/// and the converged state, formed without the loop's boundary code: every
/// base-restrained DOF at its case value g_b (+0 when the case prescribes
/// none), and every Active or Sticking support's DOF at its prescribed value
/// (+0, or ±c for an active gap; a support's ground is fixed under R-a). The
/// map is sorted by DOF. `None` when it cannot be formed: a state missing or
/// repeated, a base value off the base set or repeated, an active gap without
/// its clearance, or a DOF prescribed twice.
pub(crate) fn intended_prescribed_map(
    input: &NonlinearFrameSolveInput,
    final_states: &[open_pipe_stress_nonlinear_supports::SupportStateRecord],
) -> Option<Vec<(usize, f64)>> {
    let mut map = Vec::with_capacity(input.base_restrained_dofs.len());
    for &dof in &input.base_restrained_dofs {
        let mut values = input.base_prescribed.iter().filter(|(d, _)| *d == dof);
        let value = match (values.next(), values.next()) {
            (None, _) => 0.0,
            (Some(&(_, value)), None) => value,
            (Some(_), Some(_)) => return None,
        };
        map.push((dof, value));
    }
    if input
        .base_prescribed
        .iter()
        .any(|(dof, _)| !input.base_restrained_dofs.contains(dof))
    {
        return None;
    }
    for support in &input.nonlinear_supports {
        let mut states = final_states.iter().filter(|s| s.support_id == support.support_id);
        let state = match (states.next(), states.next()) {
            (Some(record), None) => record.state,
            _ => return None,
        };
        let value = match (state, support.behavior) {
            (ActiveSetState::Inactive | ActiveSetState::Sliding, _) => continue,
            (ActiveSetState::Active, NonlinearSupportBehavior::Gap { closes_when }) => {
                let gap = support.gap?;
                match closes_when {
                    open_pipe_stress_nonlinear_supports::GapDirection::PositiveDisplacement => gap,
                    open_pipe_stress_nonlinear_supports::GapDirection::NegativeDisplacement => -gap,
                }
            }
            (ActiveSetState::Active | ActiveSetState::Sticking, _) => 0.0,
        };
        map.push((support_dof(support), value));
    }
    if final_states.len() != input.nonlinear_supports.len() {
        return None;
    }
    map.sort_by_key(|&(dof, _)| dof);
    if map.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return None;
    }
    Some(map)
}

/// §2.4 item 2, revision 2: the complete intended map against the loop's
/// active prescribed map, entry for entry: the same DOFs in the same order
/// and the same bits (a -0.0 against a +0.0 is a mismatch). An intended map
/// that cannot be formed is a mismatch.
pub(crate) fn prescribed_map_mismatch(
    intended: Option<&[(usize, f64)]>,
    actual: &[(usize, f64)],
) -> bool {
    let Some(intended) = intended else {
        return true;
    };
    intended.len() != actual.len()
        || intended
            .iter()
            .zip(actual)
            .any(|(a, b)| a.0 != b.0 || a.1.to_bits() != b.1.to_bits())
}

fn sign_or_zero(value: f64) -> f64 {
    if value > 0.0 {
        1.0
    } else if value < 0.0 {
        -1.0
    } else {
        0.0
    }
}

fn applied_force(applied: &[AppliedSlidingFrictionForce], support_id: &str) -> Option<f64> {
    let mut found = applied.iter().filter(|a| a.support_id == support_id);
    let first = found.next()?;
    found.next().is_none().then_some(first.force)
}

fn render_check(check: Option<&FormationCheck>) -> String {
    match check.map(|c| (&c.reason, c)) {
        Some((FormationCheckReason::Estimate, c)) => format!(
            "estimate (global DOF {}, trigger ratio {:?})",
            c.global_dof.map_or_else(|| "none".to_string(), |d| d.to_string()),
            c.ratio
        ),
        Some((FormationCheckReason::FormationCheckUnavailable { detail }, _)) => detail.clone(),
        None => "no recovery inputs".to_string(),
    }
}

/// The Stage-A coupled term: its support and the loop's applied force φ⁰.
#[derive(Clone, Copy)]
struct StageA<'a> {
    term: &'a SlidingTerm,
    phi0: f64,
}

fn friction_identity(support_id: &str) -> LoadSummandIdentity {
    LoadSummandIdentity {
        family: SLIDING_FRICTION_FAMILY.to_string(),
        source: support_id.to_string(),
        ordinal: 0,
    }
}

fn friction_primitive(term_index: usize, support_id: &str, formula: LoadFormula) -> LoadPrimitive {
    let local_row = match &formula {
        LoadFormula::SlidingFrictionExplicit { global_dof, .. }
        | LoadFormula::SlidingFrictionCoupled { global_dof, .. } => global_dof % 6,
        _ => usize::MAX,
    };
    LoadPrimitive {
        term_index,
        identity: friction_identity(support_id),
        local_row,
        formula,
        force_scale_exponent: 0,
    }
}

fn explicit_formula(dof: usize, coefficient: f64, normal: f64, direction: f64) -> LoadFormula {
    LoadFormula::SlidingFrictionExplicit {
        global_dof: dof,
        coefficient,
        normal,
        direction,
    }
}

fn coupled_formula(
    term: &SlidingTerm,
    direction: f64,
    beta: f64,
    midpoint: f64,
    radius: f64,
) -> LoadFormula {
    let Normal::Derived { source_dof } = term.normal else {
        unreachable!("coupled term has a derived normal")
    };
    LoadFormula::SlidingFrictionCoupled {
        global_dof: term.global_dof,
        coefficient: term.coefficient,
        direction,
        beta,
        source_dof,
        midpoint,
        radius,
    }
}

/// The coupled Stage-B ball (v, r') and β from Stage A.
#[derive(Debug, Clone, Copy)]
struct CoupledBall {
    beta: f64,
    midpoint: f64,
    radius: f64,
}

/// The appended friction terms (§1.6). The expected manifest is enumerated
/// from the converged state's sliding set; the supplied primitives come from
/// the loop's applied-force evidence, each re-indexed to its support's term
/// (an applied force with no term is an extra primitive). The derived force
/// is an input term φ⁰ (`stage_a`) or the formed Stage-B ball (`coupled`).
fn appended_ledger(
    sliding: &[SlidingTerm],
    applied: &[AppliedSlidingFrictionForce],
    stage_a: Option<StageA<'_>>,
    coupled: Option<CoupledBall>,
) -> LoadLedger {
    let mut ledger = LoadLedger::new();
    let mut term_of = HashMap::new();
    for term in sliding {
        let source = format!("nonlinear sliding friction {}", term.support_id);
        match term.normal {
            Normal::Explicit(normal) => {
                let index = ledger.term_count();
                ledger.push_formed(
                    source,
                    term.global_dof,
                    -term.direction * term.coefficient * normal.abs(),
                    Formation::RoundedProduct {
                        k: -term.direction,
                        a: term.coefficient,
                        b: normal.abs(),
                    },
                    0.0,
                    false,
                );
                let expected = friction_primitive(
                    index,
                    &term.support_id,
                    explicit_formula(term.global_dof, term.coefficient, normal, term.direction),
                );
                ledger.set_closure_manifest(index, vec![expected.closure_entry()]);
                term_of.insert(term.support_id.as_str(), index);
            }
            Normal::Derived { .. } => match (stage_a, coupled) {
                (Some(a), _) if a.term.support_id == term.support_id => {
                    ledger.push(source, term.global_dof, a.phi0);
                }
                (_, Some(ball)) => {
                    let index = ledger.term_count();
                    ledger.push_formed(
                        source,
                        term.global_dof,
                        ball.midpoint,
                        Formation::Bounded { bound: ball.radius },
                        0.0,
                        false,
                    );
                    let expected = friction_primitive(
                        index,
                        &term.support_id,
                        coupled_formula(term, term.direction, ball.beta, ball.midpoint, ball.radius),
                    );
                    ledger.set_closure_manifest(index, vec![expected.closure_entry()]);
                    term_of.insert(term.support_id.as_str(), index);
                }
                _ => {}
            },
        }
    }
    let past_end = ledger.term_count();
    for force in applied {
        let Some(term) = sliding.iter().find(|t| t.support_id == force.support_id) else {
            // An applied force outside the sliding set is an extra primitive.
            ledger.attach_load_primitive(friction_primitive(
                past_end,
                &force.support_id,
                explicit_formula(force.global_dof, 1.0, 0.0, -sign_or_zero(force.force)),
            ));
            continue;
        };
        let direction = -sign_or_zero(force.force);
        let formula = match (term.normal, coupled) {
            (Normal::Explicit(normal), _) => {
                explicit_formula(force.global_dof, term.coefficient, normal, direction)
            }
            (Normal::Derived { .. }, Some(ball)) if stage_a.is_none() => {
                coupled_formula(term, direction, ball.beta, ball.midpoint, ball.radius)
            }
            // Stage A's derived term is an input: it carries no primitive.
            (Normal::Derived { .. }, _) => continue,
        };
        let index = term_of.get(term.support_id.as_str()).copied().unwrap_or(past_end);
        ledger.attach_load_primitive(friction_primitive(index, &term.support_id, formula));
    }
    ledger
}

/// Stage A (§1.4, k = 1): N⁰ from the symmetric system with φ⁰ as an input,
/// g from the unjudged influence system at 0, β, and the ball of φ* =
/// φ⁰ + (c·N⁰ − φ⁰)/(1 − c·g), c = −dir·μ·β, published as (v, r').
fn stage_a(
    context: &ConfirmationContext<'_>,
    stage_a_force: &AssembledForce,
    term: &SlidingTerm,
    phi0: f64,
    u: &[f64],
    n: usize,
) -> Result<CoupledBall, String> {
    let Normal::Derived { source_dof } = term.normal else {
        return Err("friction coupling not established".into());
    };
    let id = &term.support_id;
    let bound_a = match context.evaluate_at(stage_a_force, u, true) {
        Ok((None, Some(bound))) => bound,
        Ok((check, _)) => {
            return Err(format!("formation check: stage A: {}", render_check(check.as_ref())))
        }
        Err(error) => return Err(format!("formation check: stage A: {error}")),
    };
    // The influence evaluation precedes A-3, so every formation-check failure
    // of Stage A's two evaluations is reported before β (§2.4 items 5 and 7).
    let mut unit = LoadLedger::new();
    unit.push("nonlinear influence unit", term.global_dof, 1.0);
    let unit = unit
        .finish(n)
        .map_err(|error| format!("formation check: influence: {error}"))?;
    let zero = vec![0.0; n];
    let bound_u = match context.evaluate_at(&unit, &zero, false) {
        Ok((None, Some(bound))) => bound,
        Ok((check, _)) => {
            return Err(format!("formation check: influence: {}", render_check(check.as_ref())))
        }
        Err(error) => return Err(format!("formation check: influence: {error}")),
    };
    let normal = context
        .reaction(stage_a_force, source_dof, u, &bound_a)
        .map_err(|_| format!("support {id}: friction normal not established"))?;
    let Some(beta) = normal.established_sign() else {
        return Err(format!("support {id}: friction normal not established"));
    };
    let gain = context
        .reaction(&unit, source_dof, &zero, &bound_u)
        .map_err(|_| "friction coupling not established".to_string())?;
    // The applied force opposes the slip: dir = -sign(φ⁰).
    let direction = -sign_or_zero(phi0);
    if direction == 0.0 {
        return Err("friction coupling not established".into());
    }
    coupled_ball(term.coefficient, direction, beta, phi0, &normal, &gain)
        .ok_or_else(|| "friction coupling not established".to_string())
}

/// δ = (c·N⁰ − φ⁰)/(1 − c·g) in ball arithmetic; 0 ∉ 1 − c·g is required.
fn coupled_ball(
    coefficient: f64,
    direction: f64,
    beta: f64,
    phi0: f64,
    normal: &CertifiedRow,
    gain: &CertifiedRow,
) -> Option<CoupledBall> {
    let c = -direction * coefficient * beta;
    let one = CertifiedValue::exact(1.0).ok()?;
    let m = one
        .add(CertifiedValue::exact(-c).ok()?.product(gain.enclosure()).ok()?)
        .ok()?;
    let b = CertifiedValue::exact(c)
        .ok()?
        .product(normal.enclosure())
        .ok()?
        .add(CertifiedValue::exact(-phi0).ok()?)
        .ok()?;
    let delta = b.divided(m).ok()?;
    let phi = CertifiedValue::exact(phi0).ok()?.add(delta).ok()?;
    let row = phi.into_row().ok()?;
    let radius = row.bound();
    (row.published.is_finite() && radius.is_finite()).then_some(CoupledBall {
        beta,
        midpoint: row.published,
        radius,
    })
}

/// The per-support evidence the pure predicate decides (§2.2).
#[derive(Debug, Clone, Copy)]
pub(crate) enum SupportEvidence {
    /// One-way or lift-off; `positive` when it bears with a positive reaction.
    Unilateral {
        positive: bool,
        active: bool,
        ball: CertifiedRow,
    },
    ExplicitFriction {
        coefficient: f64,
        normal: f64,
        state: ActiveSetState,
        tangential: CertifiedRow,
        direction: f64,
    },
    DerivedFriction {
        coefficient: f64,
        state: ActiveSetState,
        normal: CertifiedRow,
        tangential: CertifiedRow,
        direction: f64,
        beta: Option<f64>,
    },
}

fn slip_established(tangential: &CertifiedRow, direction: f64) -> bool {
    if direction > 0.0 {
        tangential.lower_exceeds(0.0)
    } else if direction < 0.0 {
        tangential.upper_below(0.0)
    } else {
        false
    }
}

/// The pure §2.2 predicate: `None` when the support's state is established,
/// otherwise the named reason's suffix.
pub(crate) fn support_reason(evidence: &SupportEvidence) -> Option<&'static str> {
    match *evidence {
        SupportEvidence::Unilateral {
            positive,
            active,
            ball,
        } => {
            let holds = if positive {
                ball.lower_exceeds(0.0)
            } else {
                ball.upper_below(0.0)
            };
            (!holds).then_some(if active {
                "contact force sign not established"
            } else {
                "separation not established"
            })
        }
        SupportEvidence::ExplicitFriction {
            coefficient,
            normal,
            state,
            tangential,
            direction,
        } => match state {
            ActiveSetState::Inactive => (normal > 0.0).then_some("friction normal not established"),
            ActiveSetState::Sticking => (!CertifiedRow::stick_capacity_exceeds_explicit(
                coefficient,
                normal,
                &tangential,
            ))
            .then_some("stick capacity not established"),
            ActiveSetState::Sliding => {
                if !(normal > 0.0) {
                    Some("friction normal not established")
                } else if !slip_established(&tangential, direction) {
                    Some("slip direction not established")
                } else {
                    None
                }
            }
            ActiveSetState::Active => Some("friction state active has no confirmation"),
        },
        SupportEvidence::DerivedFriction {
            coefficient,
            state,
            normal,
            tangential,
            direction,
            beta,
        } => match state {
            ActiveSetState::Inactive => Some("friction normal not established"),
            ActiveSetState::Sticking => {
                if normal.established_sign().is_none() {
                    Some("friction normal not established")
                } else if !CertifiedRow::stick_capacity_exceeds(&normal, coefficient, &tangential) {
                    Some("stick capacity not established")
                } else {
                    None
                }
            }
            ActiveSetState::Sliding => {
                let sign = normal.established_sign();
                if sign.is_none() || (coefficient > 0.0 && sign != beta) {
                    Some("friction normal not established")
                } else if !slip_established(&tangential, direction) {
                    Some("slip direction not established")
                } else {
                    None
                }
            }
            ActiveSetState::Active => Some("friction state active has no confirmation"),
        },
    }
}

/// §2.4 item 7: at most six named supports, in support-id order.
pub(crate) fn certificate_reason(failures: &[(String, &'static str)]) -> Option<String> {
    if failures.is_empty() {
        return None;
    }
    let mut items = failures
        .iter()
        .take(6)
        .map(|(id, reason)| format!("support {id}: {reason}"))
        .collect::<Vec<_>>();
    if failures.len() > 6 {
        items.push(format!("{} more", failures.len() - 6));
    }
    Some(items.join("; "))
}

/// §2.4 item 7: the name under which a support whose own ball (its reaction
/// when constrained, its displacement when free) cannot be formed fails, by
/// support kind and state (review M2). An inactive friction support is decided
/// by its normal, so it fails under that name.
pub(crate) fn ball_failure_reason(
    behavior: &NonlinearSupportBehavior,
    state: ActiveSetState,
) -> &'static str {
    match (behavior, state) {
        (NonlinearSupportBehavior::Friction, ActiveSetState::Sticking) => {
            "stick capacity not established"
        }
        (NonlinearSupportBehavior::Friction, ActiveSetState::Sliding) => {
            "slip direction not established"
        }
        (NonlinearSupportBehavior::Friction, _) => "friction normal not established",
        (NonlinearSupportBehavior::Gap { .. }, _) => "gap clearance not established",
        (_, ActiveSetState::Active | ActiveSetState::Sticking) => {
            "contact force sign not established"
        }
        _ => "separation not established",
    }
}

#[allow(clippy::too_many_arguments)]
fn certify_state(
    context: &ConfirmationContext<'_>,
    supports: &[&NonlinearSupport],
    states: &HashMap<&str, ActiveSetState>,
    explicit: &HashMap<&str, f64>,
    derived: &HashMap<&str, usize>,
    sliding: &[SlidingTerm],
    applied: &[AppliedSlidingFrictionForce],
    coupled: Option<CoupledBall>,
    force: AssembledForce,
    bound: MemberBoundInputs,
    u: &[f64],
) -> Confirmation {
    let mut rows = HashMap::new();
    let mut normals = HashMap::new();
    let mut failures = Vec::new();
    let zero = CertifiedRow::ZERO;
    for support in supports {
        let id = support.support_id.as_str();
        let dof = support_dof(support);
        let state = states[id];
        let constrained = matches!(state, ActiveSetState::Active | ActiveSetState::Sticking);
        let ball = if constrained {
            context.reaction(&force, dof, u, &bound)
        } else {
            context.displacement(dof, u, &bound)
        };
        let Ok(ball) = ball else {
            failures.push((id.to_string(), ball_failure_reason(&support.behavior, state)));
            continue;
        };
        let (evidence, slot) = match support.behavior {
            NonlinearSupportBehavior::OneWay { active_when }
            | NonlinearSupportBehavior::LiftOff {
                contact_when: active_when,
            } => (
                SupportEvidence::Unilateral {
                    positive: active_when == ActivationSense::PositiveReaction,
                    active: state == ActiveSetState::Active,
                    ball,
                },
                if constrained { ball } else { zero },
            ),
            NonlinearSupportBehavior::Gap { .. } => {
                failures.push((id.to_string(), "gap clearance not established"));
                continue;
            }
            NonlinearSupportBehavior::Friction => {
                let coefficient = support.friction_coefficient.unwrap_or(0.0);
                let term = sliding.iter().find(|t| t.support_id == id);
                // dir: opposite the applied force for mu > 0 (§1.1); the sign
                // of the loop's u_t for mu = 0.
                let direction = match term {
                    Some(_) => applied_force(applied, id).map_or(0.0, |f| -sign_or_zero(f)),
                    None => sign_or_zero(u[dof]),
                };
                let slot = match (state, term) {
                    (ActiveSetState::Sticking, _) => ball,
                    (ActiveSetState::Sliding, Some(term)) => {
                        let formula = match term.normal {
                            Normal::Explicit(normal) => {
                                explicit_formula(dof, coefficient, normal, direction)
                            }
                            Normal::Derived { .. } => match coupled {
                                Some(c) => coupled_formula(term, direction, c.beta, c.midpoint, c.radius),
                                None => {
                                    failures.push((id.to_string(), "friction normal not established"));
                                    continue;
                                }
                            },
                        };
                        match certify_load_primitive(&friction_primitive(0, id, formula)) {
                            Ok(row) => row,
                            Err(_) => {
                                failures.push((id.to_string(), "slip direction not established"));
                                continue;
                            }
                        }
                    }
                    _ => zero,
                };
                let evidence = match (explicit.get(id), derived.get(id)) {
                    (Some(&normal), _) => SupportEvidence::ExplicitFriction {
                        coefficient,
                        normal,
                        state,
                        tangential: ball,
                        direction,
                    },
                    (None, Some(&source)) => {
                        let Ok(normal) = context.reaction(&force, source, u, &bound) else {
                            failures.push((id.to_string(), "friction normal not established"));
                            continue;
                        };
                        normals.insert(
                            id.to_string(),
                            ConfirmedRow {
                                global_dof: source,
                                row: normal,
                            },
                        );
                        SupportEvidence::DerivedFriction {
                            coefficient,
                            state,
                            normal,
                            tangential: ball,
                            direction,
                            beta: coupled.filter(|_| term.is_some()).map(|c| c.beta),
                        }
                    }
                    (None, None) => {
                        failures.push((id.to_string(), "friction normal not established"));
                        continue;
                    }
                };
                (evidence, slot)
            }
        };
        if let Some(reason) = support_reason(&evidence) {
            failures.push((id.to_string(), reason));
        }
        rows.insert(
            id.to_string(),
            ConfirmedRow {
                global_dof: dof,
                row: slot,
            },
        );
    }
    match certificate_reason(&failures) {
        Some(reason) => Confirmation::NotEstablished { reason },
        None => Confirmation::Established(Box::new(ConfirmedState {
            member_bound: bound,
            force,
            supports: rows,
            derived_normals: normals,
        })),
    }
}

#[cfg(test)]
#[path = "confirmation_tests.rs"]
mod tests;
