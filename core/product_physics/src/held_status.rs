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
use crate::{NonlinearQualification, NonlinearStateEvidence, PreviewSolverMode};
use open_pipe_stress_frame_kernel::{node_dof_index, FrameDof};
use open_pipe_stress_nonlinear_integration::confirmation::Confirmation;
use open_pipe_stress_nonlinear_integration::structural_adapter::StrictGapEvidence;
use open_pipe_stress_nonlinear_integration::{
    product_selected_state_is_qualified, HeldStatusSolve, NonlinearFrameIteration, NonlinearFrameSolveInput,
    NonlinearFrameSolveResult,
};
use open_pipe_stress_nonlinear_supports::{
    ActivationSense, ActiveSetState, GapDirection, NonlinearSupportBehavior, SupportStateRecord,
};
use serde_json::{json, Value};
use std::collections::HashSet;
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
    // 1. Presence.
    let (Some(source), Some(evidence)) = (source, ctx.minuend_evidence) else {
        return refuse(SOURCE_UNAVAILABLE, format!("the minuend case {minuend} has no converged nonlinear support state to lend"));
    };
    // 2. Qualification (R1: an inspectable unqualified gap state has nothing to lend).
    if source.evidence.qualification != NonlinearQualification::Qualified {
        return refuse(SOURCE_UNQUALIFIED, format!("the minuend case {minuend}'s converged state is not qualified"));
    }
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
        if let Confirmation::Established(state) = &held.confirmation {
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

#[cfg(test)]
#[path = "held_status_tests.rs"]
mod tests;
