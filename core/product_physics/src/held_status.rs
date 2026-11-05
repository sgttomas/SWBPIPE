//! RT-A5 `minuend_final` (DEL-05-02 Work `operating-case.md` §7): the held-status
//! solve's product layer.
//!
//! - **The source record** (§7.4): what a completed source case lends a held solve. It
//!   holds an invocation token, an immutable copy of the case's typed #97 evidence, the
//!   source's own loop input (both normal maps included), its springs, its receipt-bound
//!   result and the built model's node ids. It is formed only by
//!   [`capture_held_status_source`], in the source case's own solve.
//! - **The binding** ([`bind`]): §7.4's nine checks in order, each refusing by its
//!   `HELD_STATUS_*` name. The orienting iterate is taken from the authenticated result,
//!   never supplied separately.
//! - **The evidence** ([`derive`]): per support, the held and classified states and the
//!   decision with exactly the inputs the classifier read. There are two contracts:
//!   `ordinary`, the binary64 trial values, and `strict_gap`, the exact proof's signs.
//!   The record also carries the release kind, E's per-support conditions by class, and
//!   the outcome and standing (§7.3).
use crate::document_members::SubtrahendSupportStateSetting;
use crate::{
    AssembledForce, BuiltModel, LoadCaseSolve, NonlinearQualification, NonlinearStanding, NonlinearStateEvidence, NonlinearStateStanding,
    PreviewCombination, PreviewLoadCase, PreviewModel, PreviewSolverMode, ResultBasisRef, ResultItem, SpringEntry,
};
use open_pipe_stress_solver_diagnostics::SolverDiagnosticCode;
use open_pipe_stress_frame_kernel::{node_dof_index, FrameDof};
use open_pipe_stress_nonlinear_integration::confirmation::Confirmation;
use open_pipe_stress_nonlinear_integration::structural_adapter::StrictGapEvidence;
use open_pipe_stress_nonlinear_integration::{
    product_selected_state_is_qualified, solve_held_status_iteration, HeldConfirmation, HeldStatusSolve,
    NonlinearFrameIteration, NonlinearFrameSolveInput, NonlinearFrameSolveResult,
};
use open_pipe_stress_nonlinear_supports::{
    ActivationSense, ActiveSetState, GapDirection, NonlinearSupportBehavior, SupportStateRecord,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};

/// The minuend has no converged nonlinear support state to lend.
pub(crate) const SOURCE_UNAVAILABLE: &str = "HELD_STATUS_SOURCE_UNAVAILABLE";
/// The minuend's converged state is not qualified (an inspectable unqualified gap state).
pub(crate) const SOURCE_UNQUALIFIED: &str = "HELD_STATUS_SOURCE_UNQUALIFIED";
/// The minuend's state does not bind to this invocation, its own receipt-bound solve, or the
/// subtrahend's model context and nonlinear supports one for one.
pub(crate) const BINDING: &str = "HELD_STATUS_BINDING";
/// A support held sliding has a different ground offset in the two cases.
pub(crate) const SLIDING_GROUND_DIFFERS: &str = "HELD_STATUS_SLIDING_GROUND_DIFFERS";
/// The held solve was refused, blocked or singular.
pub(crate) const SOLVE_FAILED: &str = "HELD_STATUS_SOLVE_FAILED";
/// Disclosed with the rows: the target's classifier releases held supports (outcomes R, G).
pub(crate) const RELEASED: &str = "HELD_STATUS_RELEASED";
/// `contract_evidence`'s member (§7.6), beside `difference_basis_derivation`.
pub(crate) const DERIVATION_MEMBER: &str = "held_status_derivation";
/// The closed typed basis of a held-operand row (§7.6).
pub(crate) const OPERAND_BASIS: &str = "held_status_operand";

/// The sign convention of a `minuend_final` subtraction's rows (§7.8 item 5).
pub(crate) const SIGN: &str = "positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions; operating minus sustained difference at the operating support status: the minuend case's own solved state minus the subtrahend case's loads solved with each nonlinear support held at the minuend's final status, a held-constraint comparison that can need a reaction a unilateral support cannot supply and is not an admissible sustained state, under the static, history-free friction model; derived from case results; not an equilibrium state";

/// The held-operand rows' clause (§7.8 item 5).
pub(crate) fn operand_clause(combination_id: &str) -> String {
    format!("held-status operand of {combination_id}: the subtrahend case's loads solved with the minuend's final support status held; not the subtrahend case's result")
}

/// Outcome E's sentence and caveat, Agent 0's wording verbatim (§7.3).
pub(crate) const ESTABLISHED: &str = "Established within its stated numerical bound for the explicitly held system at this case's loads. The verdict does not establish this case's own support state, an admissible sustained state, a load history or uniqueness.";
const RELEASED_STANDING: &str =
    "Not established: the held status is not consistent with this case's loads; rows are inspection evidence.";
const GAP_STANDING: &str = "Gap supports keep their recovery-basis branch (R1); not established.";

fn not_established(reason: &str) -> String {
    format!("Not established: {reason}. The held status is kept by this case's classifier; rows are inspection evidence.")
}

/// A crate-private token, minted once per invocation context (a document solve or a sizing
/// request). It is never published, so it moves no output byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InvocationToken(u64);

impl InvocationToken {
    pub(crate) fn mint() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }
}

/// §7.4: what a completed source case lends a held solve. Formed only by
/// [`capture_held_status_source`]; no other constructor.
#[derive(Debug)]
pub(crate) struct HeldStatusSource {
    invocation: InvocationToken,
    evidence: NonlinearStateEvidence,
    input: NonlinearFrameSolveInput,
    springs: Vec<(usize, f64)>,
    solve: NonlinearFrameSolveResult,
    node_ids: Vec<String>,
}

/// §7.4: the record, formed only when the case is a `minuend_final` combination's minuend
/// (`wanted`) and the case published its #97 evidence (`evidence` is `Some` only for a
/// converged, qualified-or-inspectable case solve).
pub(crate) fn capture_held_status_source(
    invocation: InvocationToken,
    wanted: bool,
    evidence: Option<&NonlinearStateEvidence>,
    input: &NonlinearFrameSolveInput,
    springs: &[(usize, f64)],
    solve: Option<&NonlinearFrameSolveResult>,
    node_ids: &[String],
) -> Option<Box<HeldStatusSource>> {
    if !wanted {
        return None;
    }
    Some(Box::new(HeldStatusSource {
        invocation,
        evidence: evidence?.clone(),
        input: input.clone(),
        springs: springs.to_vec(),
        solve: solve?.clone(),
        node_ids: node_ids.to_vec(),
    }))
}

/// A named refusal: the combination is withheld (or a sizing request refused) by `code`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HeldStatusRefusal {
    pub code: &'static str,
    pub message: String,
}

fn refuse<T>(code: &'static str, message: String) -> Result<T, HeldStatusRefusal> {
    Err(HeldStatusRefusal { code, message })
}

/// What the calling context knows of the target and the minuend, for [`bind`].
pub(crate) struct BindingContext<'a> {
    pub invocation: InvocationToken,
    pub minuend_id: &'a str,
    /// The minuend `LoadCaseSolve`'s own `nonlinear_state`.
    pub minuend_evidence: Option<&'a NonlinearStateEvidence>,
    pub solver_mode: PreviewSolverMode,
    /// The target's assembled loop input (its own case members: force, base and ground motion).
    pub target_input: &'a NonlinearFrameSolveInput,
    pub target_springs: &'a [(usize, f64)],
    pub target_node_ids: &'a [String],
}

/// The bound source: the held records and the orienting iterate (the source's final one).
#[derive(Debug)]
pub(crate) struct Bound<'s> {
    pub held: &'s [SupportStateRecord],
    pub orienting: &'s NonlinearFrameIteration,
}

/// The friction slip sense the step would take from `iterate` (`sliding_direction`).
fn slip_sense(iterate: &NonlinearFrameIteration, global_dof: usize, ground: Option<f64>) -> f64 {
    let displacement = match ground {
        Some(ground) => iterate.displacements[global_dof] - ground,
        None => iterate.displacements[global_dof],
    };
    let reaction = iterate.reactions[global_dof];
    if displacement != 0.0 {
        displacement.signum()
    } else if reaction != 0.0 {
        -reaction.signum()
    } else {
        0.0
    }
}

fn ground_of(motion: &[(String, f64)], id: &str) -> Option<f64> {
    motion.iter().find(|(s, _)| s == id).map(|&(_, g)| g)
}

/// §7.4's nine checks, in order. Every refusal names its check.
pub(crate) fn bind<'s>(source: Option<&'s HeldStatusSource>, ctx: &BindingContext<'_>) -> Result<Bound<'s>, HeldStatusRefusal> {
    let minuend = ctx.minuend_id;
    // 1. Presence; 2. qualification (R1: an inspectable unqualified gap state has nothing to lend).
    let source = bind_presence(source, ctx.minuend_evidence, minuend)?;
    let evidence = ctx.minuend_evidence.expect("present after check 1");
    // 3. The invocation.
    if source.invocation != ctx.invocation {
        return refuse(BINDING, format!("the minuend case {minuend}'s source is from another invocation"));
    }
    // 4. The typed #97 evidence, its case and mode.
    if source.evidence != *evidence {
        return refuse(BINDING, format!("the minuend case {minuend}'s source evidence is not the case's own"));
    }
    if source.evidence.case_id != minuend {
        return refuse(BINDING, format!("the source case {} is not the minuend case {minuend}", source.evidence.case_id));
    }
    if source.evidence.solver_mode != ctx.solver_mode {
        return refuse(BINDING, format!("the minuend case {minuend}'s source mode {} is not this solve's", source.evidence.solver_mode.as_str()));
    }
    // 5. The receipt: the solution vectors are the loop's own output for this input.
    if !matches!(product_selected_state_is_qualified(&source.input, &source.springs, &source.solve), Ok(true))
        || source.solve.final_states != source.evidence.final_states
    {
        return refuse(BINDING, format!("the minuend case {minuend}'s source solution is not receipt-bound to its input"));
    }
    // 6. The model context: every input member but the case's own, the springs and the node ids.
    let (s, t) = (&source.input, ctx.target_input);
    let members: [(&str, bool); 9] = [
        ("node_count", s.node_count == t.node_count),
        ("elements", s.elements == t.elements),
        ("connectors", s.connectors == t.connectors),
        ("curved_bend_elements", s.curved_bend_elements == t.curved_bend_elements),
        ("base_restrained_dofs", s.base_restrained_dofs == t.base_restrained_dofs),
        ("nonlinear_supports", s.nonlinear_supports == t.nonlinear_supports),
        ("friction_normal_reactions", s.friction_normal_reactions == t.friction_normal_reactions),
        ("derived_friction_normal_reactions", s.derived_friction_normal_reactions == t.derived_friction_normal_reactions),
        ("convergence", s.convergence == t.convergence),
    ];
    if let Some((member, _)) = members.iter().find(|(_, equal)| !equal) {
        return refuse(BINDING, format!("the minuend case {minuend}'s model context differs: {member}"));
    }
    if source.springs.as_slice() != ctx.target_springs {
        return refuse(BINDING, format!("the minuend case {minuend}'s model context differs: springs"));
    }
    if source.node_ids.as_slice() != ctx.target_node_ids {
        return refuse(BINDING, format!("the minuend case {minuend}'s model context differs: node ids"));
    }
    // 7. One record per support, each a state of its behaviour.
    let held = source.evidence.final_states.as_slice();
    let mut seen = HashSet::new();
    for record in held {
        let Some(support) = t.nonlinear_supports.iter().find(|s| s.support_id == record.support_id) else {
            return refuse(BINDING, format!("held support {} is not in the target", record.support_id));
        };
        if !seen.insert(record.support_id.as_str()) {
            return refuse(BINDING, format!("held support {} is repeated", record.support_id));
        }
        let friction = matches!(support.behavior, NonlinearSupportBehavior::Friction);
        let valid = match record.state {
            ActiveSetState::Active | ActiveSetState::Inactive => !friction || record.state == ActiveSetState::Inactive,
            ActiveSetState::Sticking | ActiveSetState::Sliding => friction,
        };
        if !valid {
            return refuse(BINDING, format!("held support {} state {} is not a state of its behaviour", record.support_id, record.state.as_str()));
        }
    }
    if let Some(missing) = t.nonlinear_supports.iter().find(|s| !seen.contains(s.support_id.as_str())) {
        return refuse(BINDING, format!("held support {} is missing", missing.support_id));
    }
    // 8. The orienting iterate: the authenticated result's last; a held sliding support needs a slip sense there.
    let Some(orienting) = source.solve.iterations.last() else {
        return refuse(BINDING, format!("the minuend case {minuend}'s source has no iterate"));
    };
    let sliding = |id: &str| held.iter().any(|r| r.support_id == id && r.state == ActiveSetState::Sliding);
    for support in t.nonlinear_supports.iter().filter(|s| sliding(&s.support_id)) {
        let dof = node_dof_index(support.node_index, support.dof);
        if slip_sense(orienting, dof, ground_of(&s.ground_motion, &support.support_id)) == 0.0 {
            return refuse(BINDING, format!("held support {} has no slip sense", support.support_id));
        }
    }
    // 9. A held sliding support's ground, bit-equal in both cases.
    for support in t.nonlinear_supports.iter().filter(|s| sliding(&s.support_id)) {
        let (gs, gt) = (ground_of(&s.ground_motion, &support.support_id), ground_of(&t.ground_motion, &support.support_id));
        if gs.map(f64::to_bits) != gt.map(f64::to_bits) {
            return refuse(SLIDING_GROUND_DIFFERS, format!("held sliding support {} has a different ground offset in the two cases", support.support_id));
        }
    }
    Ok(Bound { held, orienting })
}

/// §7.3's outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Outcome {
    Established,
    NotEstablished,
    Released,
    GapInspection,
}

impl Outcome {
    pub(crate) fn kind(self) -> &'static str {
        match self {
            Self::Established => "established",
            Self::NotEstablished | Self::Released => "not_established",
            Self::GapInspection => "gap_inspection",
        }
    }
}

/// One released support (§7.3 R, §7.6 `released`).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ReleasedSupport {
    pub support_id: String,
    pub held: ActiveSetState,
    pub classified: ActiveSetState,
    /// `changed_state` or `branch`.
    pub reason: &'static str,
    /// The deciding quantity, in words with its value.
    pub quantity: String,
}

/// The held evidence of one held solve (§7.6).
#[derive(Debug, Clone)]
pub(crate) struct HeldEvidence {
    pub outcome: Outcome,
    pub standing: String,
    pub supports: Vec<Value>,
    pub conditions: Option<Vec<Value>>,
    pub released: Vec<ReleasedSupport>,
}

fn dof_token(dof: FrameDof) -> String {
    dof.as_str().to_ascii_uppercase()
}

fn hex(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}

fn sign_token(sign: impl std::fmt::Debug) -> String {
    format!("{sign:?}").to_ascii_lowercase()
}

/// The kd5 §2.2 condition applied to a support at E, with its class (§7.3).
fn condition(behavior: &NonlinearSupportBehavior, state: ActiveSetState, derived: bool, zero_limit: bool) -> (&'static str, &'static str) {
    match (behavior, state) {
        (NonlinearSupportBehavior::OneWay { .. } | NonlinearSupportBehavior::LiftOff { .. }, ActiveSetState::Active) => ("unilateral_contact_sign", "strict_ball"),
        (NonlinearSupportBehavior::OneWay { .. } | NonlinearSupportBehavior::LiftOff { .. }, _) => ("unilateral_separation", "strict_ball"),
        (NonlinearSupportBehavior::Friction, ActiveSetState::Inactive) => ("friction_normal_nonpositive", "input_decision"),
        (NonlinearSupportBehavior::Friction, ActiveSetState::Sticking) if derived => ("stick_capacity_derived", "strict_ball"),
        (NonlinearSupportBehavior::Friction, ActiveSetState::Sticking) => ("stick_capacity_explicit", "strict_ball"),
        (NonlinearSupportBehavior::Friction, _) if zero_limit => ("slip_zero_limit", "strict_ball"),
        (NonlinearSupportBehavior::Friction, _) if derived => ("slip_derived", "strict_ball"),
        (NonlinearSupportBehavior::Friction, _) => ("slip_explicit", "strict_ball"),
        (NonlinearSupportBehavior::Gap { .. }, _) => ("gap_r1_not_run", "not_run"),
    }
}

/// §7.6's per-support evidence and §7.3's outcome for one held solve.
pub(crate) fn derive(held: &HeldStatusSolve, orienting: &NonlinearFrameIteration, node_ids: &[String]) -> HeldEvidence {
    let input = &held.input;
    let last = held.result.iterations.last().expect("a held solve has its held iterate");
    let gap_report = match &last.strict_gap {
        StrictGapEvidence::Qualified(report) => Some(report),
        _ => None,
    };
    let has_gap = input.nonlinear_supports.iter().any(|s| matches!(s.behavior, NonlinearSupportBehavior::Gap { .. }));
    let mut supports = input.nonlinear_supports.iter().collect::<Vec<_>>();
    supports.sort_by(|a, b| a.support_id.cmp(&b.support_id));
    let mut out = Vec::new();
    let mut released = Vec::new();
    let mut conditions = Vec::new();
    for support in supports {
        let id = support.support_id.as_str();
        let dof = node_dof_index(support.node_index, support.dof);
        let held_state = held.held_states.iter().find(|r| r.support_id == id).map(|r| r.state).expect("held record");
        let classified = held.result.final_states.iter().find(|r| r.support_id == id).map(|r| r.state).expect("final record");
        let ground = ground_of(&input.ground_motion, id);
        let trial = match ground {
            Some(g) => last.displacements[dof] - g,
            None => last.displacements[dof],
        };
        let reaction = last.reactions[dof];
        let mut entry = json!({
            "support_id": id,
            "node": node_ids.get(support.node_index).cloned().unwrap_or_default(),
            "dof": dof_token(support.dof),
            "held_state": held_state.as_str(),
            "classified_state": classified.as_str(),
        });
        let mut branch_admissible = true;
        let mut zero_limit = false;
        let derived_source = input.derived_friction_normal_reactions.iter().find(|r| r.support_id == id);
        match support.behavior {
            NonlinearSupportBehavior::OneWay { active_when } | NonlinearSupportBehavior::LiftOff { contact_when: active_when } => {
                entry["behavior"] = json!(if matches!(support.behavior, NonlinearSupportBehavior::OneWay { .. }) { "one_way" } else { "lift_off" });
                entry["sense"] = json!(match active_when { ActivationSense::PositiveReaction => "positive_reaction", ActivationSense::NegativeReaction => "negative_reaction" });
            }
            NonlinearSupportBehavior::Gap { closes_when } => {
                entry["behavior"] = json!("gap");
                entry["closes_when"] = json!(match closes_when { GapDirection::PositiveDisplacement => "positive_displacement", GapDirection::NegativeDisplacement => "negative_displacement" });
                entry["gap_bits"] = json!(hex(support.gap.unwrap_or(0.0)));
            }
            NonlinearSupportBehavior::Friction => {
                entry["behavior"] = json!("friction");
            }
        }
        let decision_strict = matches!(support.behavior, NonlinearSupportBehavior::Gap { .. }) && gap_report.is_some();
        if decision_strict {
            let report = gap_report.expect("checked");
            let decision = report.decisions.iter().find(|d| d.support_id == id).expect("one decision per gap");
            entry["decision"] = json!("strict_gap");
            entry["strict_gap"] = json!({
                "prior": decision.prior.as_str(),
                "gap_sign": sign_token(&decision.gap_sign),
                "reaction_sign": sign_token(&decision.reaction_sign),
                "state": decision.state.as_str(),
            });
        } else {
            entry["decision"] = json!("ordinary");
            let mut inputs = json!({
                "ground_bits": hex(ground.unwrap_or(0.0)),
                "displacement_bits": hex(trial),
                "reaction_bits": hex(reaction),
            });
            if matches!(support.behavior, NonlinearSupportBehavior::Friction) {
                let mu = support.friction_coefficient.unwrap_or(0.0);
                let explicit = input.friction_normal_reactions.iter().find(|r| r.support_id == id);
                let (trial_normal, signed_normal, source) = match (explicit, derived_source) {
                    (Some(e), _) => (e.normal_reaction, e.normal_reaction, "explicit"),
                    (None, Some(d)) => {
                        let r = last.reactions[node_dof_index(d.source_node_index, d.source_dof)];
                        (r.abs(), r, "derived")
                    }
                    (None, None) => (0.0, 0.0, "missing"),
                };
                zero_limit = mu * trial_normal.abs() == 0.0;
                inputs["coefficient_bits"] = json!(hex(mu));
                inputs["normal_source"] = json!(source);
                // The derived normal's own source, typed and closed (review P2-1): the support, its
                // node and the DOF whose reaction the classifier read.
                if let (Some(d), "derived") = (derived_source, source) {
                    inputs["normal_source_ref"] = json!({
                        "support": d.source_ref,
                        "node": node_ids.get(d.source_node_index).cloned().unwrap_or_default(),
                        "dof": dof_token(d.source_dof),
                    });
                }
                inputs["normal_trial_bits"] = json!(hex(trial_normal));
                inputs["normal_signed_bits"] = json!(hex(signed_normal));
                let applied = last.applied_sliding_friction_forces.iter().find(|f| f.support_id == id).map_or(0.0, |f| f.force);
                inputs["applied_force_bits"] = json!(hex(applied));
                if held_state == ActiveSetState::Sliding {
                    let sense = slip_sense(orienting, dof, ground);
                    inputs["slip_sense"] = json!(sense as i8);
                    if let Some(d) = derived_source {
                        let src_dof = node_dof_index(d.source_node_index, d.source_dof);
                        let previous = orienting.reactions[src_dof];
                        let beta = if previous == 0.0 { 0.0 } else { previous.signum() };
                        inputs["beta"] = json!(beta as i8);
                        branch_admissible = if mu == 0.0 || sense == 0.0 {
                            true
                        } else if beta == 0.0 {
                            signed_normal == 0.0
                        } else {
                            beta * signed_normal >= 0.0
                        };
                        inputs["branch_admissible"] = json!(branch_admissible);
                    }
                }
            }
            entry["inputs"] = inputs;
        }
        let reason = if !branch_admissible {
            Some("branch")
        } else if classified != held_state {
            Some("changed_state")
        } else {
            None
        };
        entry["released"] = json!(reason.unwrap_or("none"));
        if let Some(reason) = reason {
            let quantity = if reason == "branch" {
                let normal = entry["inputs"]["normal_signed_bits"].as_str().and_then(|b| u64::from_str_radix(b, 16).ok()).map(f64::from_bits).unwrap_or(0.0);
                format!("derived normal {normal:e} against the held branch {}", entry["inputs"]["beta"])
            } else if matches!(held_state, ActiveSetState::Active | ActiveSetState::Sticking) {
                format!("reaction {reaction:e}")
            } else {
                format!("displacement {trial:e} from its ground")
            };
            released.push(ReleasedSupport { support_id: id.to_string(), held: held_state, classified, reason, quantity });
        }
        let (name, class) = condition(&support.behavior, held_state, derived_source.is_some(), zero_limit);
        let mut cond = json!({"support_id": id, "condition": name, "class": class});
        if let (Confirmation::Established(state), "strict_ball") = (&held.confirmation, class) {
            if let Some(row) = state.supports.get(id) {
                cond["published_bits"] = json!(hex(row.row.published));
                cond["radius_bits"] = json!(hex(row.row.radius));
            }
        }
        conditions.push(cond);
        out.push(entry);
    }
    let outcome = if has_gap {
        Outcome::GapInspection
    } else if !released.is_empty() {
        Outcome::Released
    } else if matches!(held.confirmation, Confirmation::Established(_)) {
        Outcome::Established
    } else {
        Outcome::NotEstablished
    };
    let standing = match (&held.confirmation, outcome) {
        (_, Outcome::Established) => ESTABLISHED.to_string(),
        (_, Outcome::Released) => RELEASED_STANDING.to_string(),
        (_, Outcome::GapInspection) => GAP_STANDING.to_string(),
        (Confirmation::NotEstablished { reason }, Outcome::NotEstablished) => not_established(reason),
        (Confirmation::Established(_), Outcome::NotEstablished) => unreachable!("established is E"),
    };
    HeldEvidence {
        outcome,
        standing,
        supports: out,
        conditions: (outcome == Outcome::Established).then_some(conditions),
        released,
    }
}

/// `HELD_STATUS_RELEASED`'s message: each released support, its held and classified states,
/// the reason and the deciding quantity (§7.3).
pub(crate) fn released_message(combination_id: &str, released: &[ReleasedSupport]) -> String {
    let items = released
        .iter()
        .map(|r| format!("{} held {}, classified {} ({}: {})", r.support_id, r.held.as_str(), r.classified.as_str(), r.reason.replace('_', " "), r.quantity))
        .collect::<Vec<_>>()
        .join("; ");
    format!("Combination {combination_id} holds the minuend's support status in the subtrahend case, and that case's classifier releases: {items}. The rows are the held system's, published with this standing; the held status is not an admissible state of the subtrahend case.")
}

/// Whose support state a case solve forms (§7.4).
pub(crate) enum CaseRole<'r, 's> {
    /// The case's own loop. With the invocation's token, the case lends its state when it is
    /// a `minuend_final` combination's minuend; without one (callers outside a document solve),
    /// it never does.
    Own(Option<InvocationToken>),
    /// The case's loads solved with each nonlinear support held at the request's source.
    Held(&'r mut HeldRequest<'s>),
}

impl CaseRole<'_, '_> {
    pub(crate) fn held(&self) -> bool {
        matches!(self, Self::Held(_))
    }
}

/// One held solve's request: the minuend's record and evidence, and, once run, its outcome.
pub(crate) struct HeldRequest<'s> {
    pub invocation: InvocationToken,
    pub minuend_id: &'s str,
    pub minuend_evidence: Option<&'s NonlinearStateEvidence>,
    pub source: Option<&'s HeldStatusSource>,
    pub outcome: Option<Result<HeldRun, HeldStatusRefusal>>,
}

/// What a held solve that ran states: its evidence, and whether the held iterate converged,
/// with the loop's exit diagnostic when it did not (a release is never promoted).
#[derive(Debug, Clone)]
pub(crate) struct HeldRun {
    pub evidence: HeldEvidence,
    pub converged: bool,
    pub exit: Option<(&'static str, String)>,
}

/// A declared operating comparison whose subtrahend is held at the minuend's final status.
pub(crate) fn is_minuend_final(combination: &PreviewCombination) -> bool {
    crate::difference_basis::declared(combination)
        && combination.subtrahend_support_state == Some(SubtrahendSupportStateSetting::MinuendFinal)
}

/// Whether `case_id` is some `minuend_final` combination's minuend (its source is captured).
pub(crate) fn wanted(model: &PreviewModel, case_id: &str) -> bool {
    model.combinations.iter().any(|c| is_minuend_final(c) && c.minuend_id.as_deref() == Some(case_id))
}

/// The built model's node ids, by node index.
pub(crate) fn node_ids(model: &PreviewModel) -> Vec<String> {
    model.nodes.iter().map(|node| node.id.clone()).collect()
}

/// §7.4 checks 1 and 2, which need no target: presence, then qualification.
fn bind_presence<'s>(source: Option<&'s HeldStatusSource>, evidence: Option<&NonlinearStateEvidence>, minuend: &str) -> Result<&'s HeldStatusSource, HeldStatusRefusal> {
    let (Some(source), Some(_)) = (source, evidence) else {
        return refuse(SOURCE_UNAVAILABLE, format!("the minuend case {minuend} has no converged nonlinear support state to lend"));
    };
    if source.evidence.qualification != NonlinearQualification::Qualified {
        return refuse(SOURCE_UNQUALIFIED, format!("the minuend case {minuend}'s converged state is not qualified"));
    }
    Ok(source)
}

fn solve_failed(message: String) -> HeldStatusRefusal {
    HeldStatusRefusal { code: SOLVE_FAILED, message }
}

/// The held path of a case solve, in place of the case's own loop: the target's loop input,
/// the binding, the NI held step with its one confirmation attempt, and the evidence. Every
/// refusal is recorded on the request; `None` then blocks this (local) solve.
#[allow(clippy::too_many_arguments)]
pub(crate) fn held_solve(
    request: &mut HeldRequest<'_>,
    model: &PreviewModel,
    built: &BuiltModel,
    restrained_dofs: &[usize],
    base_motion: Option<&BTreeMap<usize, f64>>,
    ground_motion: &[(String, f64)],
    force: &AssembledForce,
    solver_mode: PreviewSolverMode,
    spring_entries: &[SpringEntry],
    confirm: bool,
) -> Option<(NonlinearFrameSolveResult, NonlinearStanding, Option<NonlinearQualification>)> {
    let input = match crate::nonlinear_frame_input(built, restrained_dofs, base_motion, ground_motion, force) {
        Ok(input) => input,
        Err(error) => {
            request.outcome = Some(Err(solve_failed(format!("the held solve's input was refused: {error}"))));
            return None;
        }
    };
    let springs = crate::loop_springs(spring_entries);
    let node_ids = node_ids(model);
    let context = BindingContext {
        invocation: request.invocation,
        minuend_id: request.minuend_id,
        minuend_evidence: request.minuend_evidence,
        solver_mode,
        target_input: &input,
        target_springs: &springs,
        target_node_ids: &node_ids,
    };
    let bound = match bind(request.source, &context) {
        Ok(bound) => bound,
        Err(refusal) => {
            request.outcome = Some(Err(refusal));
            return None;
        }
    };
    let curved_sources = built.curved_bend_elements.iter().map(|e| e.macro_element).collect::<Vec<_>>();
    let members = crate::frame_member_identities(built);
    let confirmation = confirm.then_some(HeldConfirmation { force, curved_sources: &curved_sources, frame_members: &members });
    let held = match solve_held_status_iteration(&input, &springs, bound.held, Some(bound.orienting), solver_mode.nonlinear_mode(), confirmation) {
        Ok(held) => held,
        Err(error) => {
            request.outcome = Some(Err(solve_failed(format!("the held solve was refused: {error}"))));
            return None;
        }
    };
    let evidence = derive(&held, bound.orienting, &node_ids);
    let exit = held
        .result
        .diagnostics
        .iter()
        .find(|d| d.code == SolverDiagnosticCode::NonConvergence)
        .map(|d| (crate::solver_diag_code(d.code), d.message.clone()));
    let converged = held.result.converged;
    let HeldStatusSolve { result, confirmation, .. } = held;
    // Only E publishes the certified rows; every other outcome's rows are binary64 recoveries.
    let standing = match (evidence.outcome, confirmation) {
        (Outcome::Established, Confirmation::Established(state)) => NonlinearStanding::Confirmed(state),
        _ => NonlinearStanding::NotEstablished(evidence.standing.clone()),
    };
    request.outcome = Some(Ok(HeldRun { evidence, converged, exit }));
    Some((result, standing, None))
}

/// A `minuend_final` combination's held operand.
#[derive(Debug)]
pub(crate) enum HeldOutcome {
    /// Withheld by its `HELD_STATUS_*` gate.
    Withheld(HeldStatusRefusal),
    Admitted(Box<HeldOperand>),
}

/// An admitted held operand: the subtrahend case it holds, the run, and the held solve's
/// rows under their base (unqualified) ids, as the case's own rows are before qualification.
#[derive(Debug)]
pub(crate) struct HeldOperand {
    pub run: HeldRun,
    pub rows: Vec<ResultItem>,
}

/// Each `minuend_final` combination's held solve, in model order, after G-1 and G-2 (a
/// combination either gate withholds reaches no held gate). `solve_target` runs the
/// subtrahend's case solve on the held path with its own diagnostics, renders it, and
/// returns it unless it was blocked.
pub(crate) fn solve_held_combinations<'m>(
    model: &'m PreviewModel,
    invocation: InvocationToken,
    minuend_state: impl Fn(&str) -> (Option<&'m HeldStatusSource>, Option<&'m NonlinearStateEvidence>),
    mut modulus_key: impl FnMut(&PreviewLoadCase) -> Option<String>,
    mut solve_target: impl FnMut(&PreviewLoadCase, &mut HeldRequest<'_>) -> Option<LoadCaseSolve>,
) -> HashMap<String, HeldOutcome> {
    let mut outcomes = HashMap::new();
    for combination in model.combinations.iter().filter(|c| is_minuend_final(c)) {
        if crate::difference_basis::gate(model, combination, &mut modulus_key).is_some() {
            continue;
        }
        let case = |id: Option<&str>| id.and_then(|id| model.load_cases.iter().find(|c| c.id == id));
        let (Some(minuend), Some(target)) = (case(combination.minuend_id.as_deref()), case(combination.subtrahend_id.as_deref())) else {
            continue;
        };
        let (source, evidence) = minuend_state(&minuend.id);
        let outcome = match bind_presence(source, evidence, &minuend.id) {
            Err(refusal) => HeldOutcome::Withheld(refusal),
            Ok(_) => {
                let mut request = HeldRequest { invocation, minuend_id: &minuend.id, minuend_evidence: evidence, source, outcome: None };
                let solve = solve_target(target, &mut request);
                match (request.outcome.take(), solve) {
                    (Some(Err(refusal)), _) => HeldOutcome::Withheld(refusal),
                    (Some(Ok(run)), Some(solve)) => HeldOutcome::Admitted(Box::new(HeldOperand { run, rows: solve.results })),
                    _ => HeldOutcome::Withheld(solve_failed(format!("the held solve of {} at the minuend case {}'s status was blocked", target.id, minuend.id))),
                }
            }
        };
        outcomes.insert(combination.id.clone(), outcome);
    }
    outcomes
}

/// A held-operand row's id: `<combination-qualified>:held:<base id>`.
pub(crate) fn row_id(combination_id: &str, base_id: &str) -> String {
    crate::qualified_combination_result_id(combination_id, &format!("held:{}", crate::result_tail(base_id)))
}

/// The held rows as the combination's lookup reads them: each under its base id, with its
/// published id, and with its own (case-form) metadata, so it pairs with the minuend's rows.
pub(crate) fn lookup_rows(combination_id: &str, operand: &HeldOperand) -> Vec<(String, ResultItem)> {
    operand
        .rows
        .iter()
        .map(|row| {
            let mut keyed = row.clone();
            keyed.id = row_id(combination_id, &row.id);
            keyed.source_result_refs = row.source_result_refs.iter().map(|r| row_id(combination_id, r)).collect();
            (row.id.clone(), keyed)
        })
        .collect()
}

/// The held operand's stated support state (slice 1a's operand record, §7.6).
pub(crate) const HELD_SUPPORT_STATE: &str = "held_at_minuend_final";

/// The held rows the combination's rows cite, with every held row those cite in turn, in
/// the held solve's order, as published: so each held row's own references resolve.
pub(crate) fn cited_rows(combination_id: &str, keyed: &[ResultItem], combined: &[ResultItem]) -> Vec<ResultItem> {
    let by_id: HashMap<&str, &ResultItem> = keyed.iter().map(|r| (r.id.as_str(), r)).collect();
    let mut wanted: HashSet<&str> = HashSet::new();
    let mut pending: Vec<&str> = combined.iter().flat_map(|r| r.source_result_refs.iter().map(String::as_str)).collect();
    while let Some(id) = pending.pop() {
        if let Some(row) = by_id.get(id) {
            if wanted.insert(row.id.as_str()) {
                pending.extend(row.source_result_refs.iter().map(String::as_str));
            }
        }
    }
    keyed.iter().filter(|r| wanted.contains(r.id.as_str())).map(|r| publish_row(combination_id, r)).collect()
}

/// A held-operand row as published (§7.6): under the combination, of basis
/// `held_status_operand`, its sign convention followed by the operand clause.
pub(crate) fn publish_row(combination_id: &str, keyed: &ResultItem) -> ResultItem {
    let mut row = keyed.clone();
    row.basis_ref = Some(ResultBasisRef { ref_type: "combination".to_string(), ref_id: combination_id.to_string() });
    let clause = operand_clause(combination_id);
    match row.metadata.as_mut() {
        Some(metadata) => {
            metadata.basis = OPERAND_BASIS.to_string();
            metadata.sign_convention = format!("{}; {clause}", metadata.sign_convention);
        }
        None => row.metadata = Some(crate::derived_magnitude_metadata(OPERAND_BASIS, &clause)),
    }
    row
}

/// The source's standing token for the record.
pub(crate) fn source_standing(evidence: Option<&NonlinearStateEvidence>) -> Value {
    match evidence {
        Some(e) => json!({
            "qualification": match e.qualification {
                NonlinearQualification::Qualified => "qualified",
                NonlinearQualification::InspectableUnqualifiedGap => "inspectable_unqualified_gap",
            },
            "standing": match e.standing {
                NonlinearStateStanding::Confirmed => "confirmed",
                NonlinearStateStanding::NotEstablished { .. } => "not_established",
            },
        }),
        None => json!({"qualification": null, "standing": null}),
    }
}

/// `contract_evidence.held_status_derivation`'s record of one combination (§7.6): withheld
/// (an empty `held`, no `conditions`) or admitted.
pub(crate) fn record(
    combination: &PreviewCombination,
    solver_mode: PreviewSolverMode,
    source: &Value,
    target_modulus_key: Option<String>,
    outcome: &HeldOutcome,
) -> Value {
    let mut record = json!({
        "combination_id": combination.id,
        "subtrahend_support_state": "minuend_final",
        "source": {
            "load_case_id": combination.minuend_id,
            "solver_mode": solver_mode.as_str(),
            "qualification": source["qualification"],
            "standing": source["standing"],
        },
        "target": {"load_case_id": combination.subtrahend_id, "modulus_basis_key": target_modulus_key},
    });
    match outcome {
        HeldOutcome::Withheld(refusal) => {
            if refusal.code == SOURCE_UNAVAILABLE {
                // No source was lent: the record states no qualification and no standing.
                record["source"]["qualification"] = Value::Null;
                record["source"]["standing"] = Value::Null;
            }
            record["outcome"] = json!(refusal.code);
            record["held"] = json!([]);
            record["held_solve"] = Value::Null;
            record["standing"] = json!({"kind": "withheld", "reason": refusal.message});
        }
        HeldOutcome::Admitted(operand) => {
            let run = &operand.run;
            record["outcome"] = json!("admitted");
            record["held"] = Value::Array(run.evidence.supports.clone());
            if let Some(conditions) = &run.evidence.conditions {
                record["conditions"] = Value::Array(conditions.clone());
            }
            record["held_solve"] = json!({
                "converged": run.converged,
                "exit_diagnostic": run.exit.as_ref().map(|(code, message)| json!({"code": code, "message": message})),
            });
            record["standing"] = json!({"kind": run.evidence.outcome.kind(), "reason": run.evidence.standing});
        }
    }
    record
}

#[cfg(test)]
#[path = "held_status_tests.rs"]
mod tests;
