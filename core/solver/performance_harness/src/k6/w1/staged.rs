//! K6b's staged W1 sequence, its work accounting and the budget-truncated
//! prefixes (T3 K6b plan §3.3, Q1(c); ROOT's rulings on I16's plan).
//!
//! K4's entry is one call, so the stages the binary can time are the adapter
//! (`w1_source`) and the call as a whole (`w1_solve`). K4's evidence reports
//! the work of each attempt by stage.
//!
//! The prefixes re-run `solve_case` under `CaseLimit::new(b_j)`, where b_j is
//! the case work charged through the j-th segment of the full call, in the
//! schedule's time order (`K4R/adaptive.rs:2683-2805`): each attempt's solve
//! (its shared build and its own solve), then, for an attempt used as a
//! verification, its verification pass (shared data and pass), then the
//! decision on the attempt before it (the stop rule, charged to that
//! candidate). The next segment stops at its first budget check
//! (`StageGuard::test`) with `Unresolved(Budget(Case))`. So a prefix's heap
//! peak is the high-water mark through segment j, plus the allocations the
//! next segment makes before that check (derived in RETURN).

use super::super::models::K6Model;
use super::super::staged::{Stage, StageObserver};
use super::adapter;
use open_pipe_stress_frame_kernel::structural::retained_api::{
    solve_case, AttemptOutcome, AttemptReason, AttemptRecord, CaseLimit, CaseOutcome,
    InvocationMeter, PrimitiveSource, SourceError, StageWork, UnresolvedReason, WorkFault,
    WorkTotal,
};

/// The per-case limit and the invocation meter's limit (ROOT's Q3 ruling:
/// both `u64::MAX`, large enough never to stop a run; recorded).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct W1Limits {
    pub case: u64,
    pub invocation: u64,
}

impl Default for W1Limits {
    fn default() -> Self {
        Self {
            case: u64::MAX,
            invocation: u64::MAX,
        }
    }
}

/// One `solve_case` call and the invocation meter's charge.
#[derive(Debug)]
pub struct W1Solve {
    pub outcome: CaseOutcome,
    pub charged: WorkTotal,
    pub exhausted: bool,
}

/// `w1_source`: the adapter and `PrimitiveSource::new`.
pub fn w1_source(
    model: &K6Model,
    obs: &mut dyn StageObserver,
) -> Result<PrimitiveSource, SourceError> {
    obs.begin(Stage::W1Source);
    let result = adapter::source(model);
    obs.end(
        Stage::W1Source,
        result.is_ok(),
        result.as_ref().err().map(|e| format!("{e:?}")),
    );
    result
}

/// One `solve_case` call under `limits`, observed as `stage`.
pub fn w1_solve(
    source: PrimitiveSource,
    limits: W1Limits,
    stage: Stage,
    obs: &mut dyn StageObserver,
) -> W1Solve {
    obs.begin(stage);
    let mut meter = InvocationMeter::new(limits.invocation);
    let outcome = solve_case(source, CaseLimit::new(limits.case), &mut meter);
    let error = match &outcome {
        CaseOutcome::Selected(_) => None,
        CaseOutcome::Refused { refusal, .. } => Some(format!("Refused({refusal:?})")),
        CaseOutcome::Unresolved { reason, .. } => Some(format!("Unresolved({reason:?})")),
    };
    obs.end(stage, meter.checked_charged().status().is_exact(), error);
    W1Solve {
        outcome,
        charged: meter.checked_charged(),
        exhausted: meter.exhausted(),
    }
}

/// The outcome's class.
pub fn outcome_class(outcome: &CaseOutcome) -> &'static str {
    match outcome {
        CaseOutcome::Selected(_) => "Selected",
        CaseOutcome::Refused { .. } => "Refused",
        CaseOutcome::Unresolved { .. } => "Unresolved",
    }
}

/// The attempts of an outcome (none for a refusal).
pub fn attempts_of(outcome: &CaseOutcome) -> &[AttemptRecord] {
    match outcome {
        CaseOutcome::Selected(solve) => &solve.evidence().attempts,
        CaseOutcome::Refused { .. } => &[],
        CaseOutcome::Unresolved { attempts, .. } => attempts,
    }
}

/// The 19 stage fields of a `StageWork`, by name (K4 RETURN §14).
pub fn stage_fields(w: &StageWork) -> [(&'static str, u64); 19] {
    [
        ("formation", w.formation),
        ("assembly", w.assembly),
        ("residual_formation", w.residual_formation),
        ("factor", w.factor),
        ("condition", w.condition),
        ("rhs", w.rhs),
        ("solve", w.solve),
        ("refinement", w.refinement),
        ("recovery", w.recovery),
        ("stop_rule", w.stop_rule),
        ("bounded_gate", w.bounded_gate),
        ("scale", w.scale),
        ("estimate", w.estimate),
        ("charge", w.charge),
        ("bound", w.bound),
        ("shift", w.shift),
        ("bounded_formation", w.bounded_formation),
        ("wide_formation", w.wide_formation),
        ("uc", w.uc),
    ]
}

/// The sum of a `StageWork`'s fields (saturating).
pub fn stage_sum(w: &StageWork) -> u64 {
    w.checked_total().exact().unwrap_or(u64::MAX)
}

/// An attempt's own work: its contexts and its K4 sums (K4 RETURN §14's
/// "own + K4 sum"), which hold its solve, its verification pass and its stop
/// rule.
pub fn own_total(a: &AttemptRecord) -> u64 {
    a.work
        .limb_multiply_equivalents()
        .saturating_add(a.k4_work.limb_multiply_equivalents())
}

/// The work an attempt charged to the invocation: its own work, plus the
/// shared builds it built.
pub fn charged_by(a: &AttemptRecord) -> u64 {
    let mut total = own_total(a);
    if a.shared_built_here {
        total = total.saturating_add(a.shared_work);
    }
    if a.verification_shared_built_here {
        total = total.saturating_add(a.verification_shared_work);
    }
    total
}

/// Parity `w1_work_closes`: the attempts' charges add up to the meter's.
pub fn validate_outcome(outcome: &CaseOutcome) -> Result<(), WorkFault> {
    if let CaseOutcome::Unresolved {
        reason: UnresolvedReason::WorkAccounting { fault, .. },
        ..
    } = outcome
    {
        return Err(*fault);
    }
    validate_attempts(attempts_of(outcome))
}

pub fn validate_attempts(attempts: &[AttemptRecord]) -> Result<(), WorkFault> {
    let mut case = 0u64;
    let mut invocation = 0u64;
    for a in attempts {
        case = case
            .checked_add(a.checked_case_charge().exact()?)
            .ok_or(WorkFault::Overflow)?;
        invocation = invocation
            .checked_add(a.checked_invocation_increment().exact()?)
            .ok_or(WorkFault::Overflow)?;
        a.checked_verification_work().exact()?;
        a.checked_stop_rule_work().exact()?;
        a.stages.checked_total().exact()?;
        a.shared_stages.checked_total().exact()?;
    }
    Ok(())
}
pub fn work_closes(attempts: &[AttemptRecord], charged: WorkTotal) -> bool {
    let checked = || -> Result<bool, WorkFault> {
        validate_attempts(attempts)?;
        let total = attempts.iter().try_fold(0u64, |sum, a| {
            sum.checked_add(a.checked_invocation_increment().exact()?)
                .ok_or(WorkFault::Overflow)
        })?;
        Ok(total == charged.exact()?)
    };
    checked().unwrap_or(false)
}

/// The shared work charged to an attempt's case: its build and the
/// verification's shared data, each counted in full against the case (K4
/// RETURN §14).
pub fn shared_total(a: &AttemptRecord) -> u64 {
    a.shared_work.saturating_add(a.verification_shared_work)
}

/// Whether an attempt's builds all completed: not `Failed(Stop(_))` inside a
/// build (`K4R/adaptive.rs`), or stopped in its stop rule with every build
/// complete (a positive `stop_rule_work`, RV22-1). Evidence only: since T3
/// KF3 every build stages the work of the stage it stopped in, so
/// `stages_equal_totals` holds every attempt to equality, completed or
/// stopped (ROOT's ruling "KF3: main merged; K6b's parity restored in KF3").
pub fn builds_completed(a: &AttemptRecord) -> bool {
    validate_attempts(std::slice::from_ref(a)).is_ok()
        && (!matches!(a.outcome, AttemptOutcome::Failed(AttemptReason::Stop(_)))
            || a.stop_rule_work > 0)
}

/// The work an attempt was charged that no stage records: (own, shared). Since
/// T3 KF3 (every build stages its partial work when it stops) this is (0, 0)
/// on every attempt, stopped builds included; the attempt line keeps it
/// (RV22-1). Saturating: a stage sum above its total shows as zero here and
/// fails `stages_equal_totals`.
pub fn unstaged(a: &AttemptRecord) -> (u64, u64) {
    (
        own_total(a).saturating_sub(stage_sum(&a.stages)),
        shared_total(a).saturating_sub(stage_sum(&a.shared_stages)),
    )
}

/// Whether no charged work is unstaged: the attempt line's `stages_complete`
/// (RV22-1); true on every attempt since T3 KF3.
pub fn stages_complete(a: &AttemptRecord) -> bool {
    validate_attempts(std::slice::from_ref(a)).is_ok() && unstaged(a) == (0, 0)
}

/// Parity `w1_stages_equal_totals`: on every attempt, completed or stopped,
/// the own stages add up to the own work and the shared stages to the shared
/// work (the attempt's build and the verification's shared data). T3 KF3
/// (ROOT's ruling "KF3: main merged; K6b's parity restored in KF3") makes
/// every build that stops add the work its stages do not record to the stage
/// in progress, so the relaxation for a stopped build (ROOT's ruling on the
/// K6B-S3 stop; RV22's review) is withdrawn: one short side fails.
pub fn stages_equal_totals(attempts: &[AttemptRecord]) -> bool {
    validate_attempts(attempts).is_ok()
        && attempts.iter().all(|a| {
            stage_sum(&a.stages) == own_total(a) && stage_sum(&a.shared_stages) == shared_total(a)
        })
}

/// Work by precision: each precision's attempts, their own stages and their
/// shared stages (counted in full against the case, K4 RETURN §14), and the
/// charged totals, which equal the stage sums on every attempt (T3 KF3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrecisionWork {
    pub precision: u32,
    pub attempts: usize,
    pub own: StageWork,
    pub shared: StageWork,
    /// The attempts' `own_total`s.
    pub own_total: u64,
    /// The attempts' `shared_total`s.
    pub shared_total: u64,
}

fn add_stages(a: &mut StageWork, b: &StageWork) -> Result<(), WorkFault> {
    a.merge(b)
}

/// Each attempt's work filed under its own precision, ascending.
pub fn work_by_precision(attempts: &[AttemptRecord]) -> Result<Vec<PrecisionWork>, WorkFault> {
    validate_attempts(attempts)?;
    let mut out: Vec<PrecisionWork> = Vec::new();
    for a in attempts {
        let entry = match out.iter().position(|w| w.precision == a.precision) {
            Some(k) => &mut out[k],
            None => {
                out.push(PrecisionWork {
                    precision: a.precision,
                    attempts: 0,
                    own: StageWork::default(),
                    shared: StageWork::default(),
                    own_total: 0,
                    shared_total: 0,
                });
                out.last_mut().expect("just pushed")
            }
        };
        entry.attempts += 1;
        add_stages(&mut entry.own, &a.stages)?;
        add_stages(&mut entry.shared, &a.shared_stages)?;
        entry.own_total = entry
            .own_total
            .checked_add(own_total(a))
            .ok_or(WorkFault::Overflow)?;
        entry.shared_total = entry
            .shared_total
            .checked_add(shared_total(a))
            .ok_or(WorkFault::Overflow)?;
    }
    out.sort_by_key(|w| w.precision);
    Ok(out)
}

/// A segment of the call in time order, with the case work charged through
/// its end.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    pub label: String,
    pub end: u64,
}

/// The call's segments in the schedule's time order (module documentation).
pub fn segments(attempts: &[AttemptRecord]) -> Result<Vec<Segment>, WorkFault> {
    validate_attempts(attempts)?;
    let mut out = Vec::new();
    let mut total = 0u64;
    for (i, a) in attempts.iter().enumerate() {
        let own_solve = own_total(a)
            .checked_sub(a.verification_work)
            .ok_or(WorkFault::Inconsistent)?
            .checked_sub(a.stop_rule_work)
            .ok_or(WorkFault::Inconsistent)?;
        total = total
            .checked_add(a.shared_work)
            .ok_or(WorkFault::Overflow)?
            .checked_add(own_solve)
            .ok_or(WorkFault::Overflow)?;
        out.push(Segment {
            label: format!("solve_{}", a.precision),
            end: total,
        });
        let verified =
            a.verification.is_some() || a.verification_work > 0 || a.verification_shared_work > 0;
        if verified {
            total = total
                .checked_add(a.verification_shared_work)
                .ok_or(WorkFault::Overflow)?
                .checked_add(a.verification_work)
                .ok_or(WorkFault::Overflow)?;
            out.push(Segment {
                label: format!("verify_{}", a.precision),
                end: total,
            });
            if i > 0 && attempts[i - 1].stop_rule_work > 0 {
                total = total
                    .checked_add(attempts[i - 1].stop_rule_work)
                    .ok_or(WorkFault::Overflow)?;
                out.push(Segment {
                    label: format!("decide_{}", attempts[i - 1].precision),
                    end: total,
                });
            }
        }
    }
    Ok(out)
}

/// Parity `w1_prefix_segments` for prefix j (1-based): the prefix call ended on
/// the case budget, and its first j segments equal the full call's (RV22-N3:
/// the binary's predicate, here so that it is tested).
pub fn prefix_matches(j: usize, full: &[AttemptRecord], prefix: &CaseOutcome) -> bool {
    let Ok(full_segments) = segments(full) else {
        return false;
    };
    let Ok(own) = segments(attempts_of(prefix)) else {
        return false;
    };
    let budget_stop = matches!(
        prefix,
        CaseOutcome::Unresolved {
            reason: UnresolvedReason::Budget(_),
            ..
        }
    );
    budget_stop && own.len() >= j && full_segments.len() > j && own[..j] == full_segments[..j]
}

/// The prefix limits: every segment's end but the last (the last is the full
/// call).
pub fn prefix_limits(attempts: &[AttemptRecord]) -> Result<Vec<(String, u64)>, WorkFault> {
    let segs = segments(attempts)?;
    Ok(segs
        .iter()
        .take(segs.len().saturating_sub(1))
        .map(|s| (s.label.clone(), s.end))
        .collect())
}
