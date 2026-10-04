//! The per-case records for ROOT's W1 limits (brief Scope 11; plan §12):
//! schema `vk-case-record-v1`, one object per case, committed per family in
//! `observations/kernel_lane/<family>.json` and compared by CI.
//!
//! Only deterministic content: the outcome, the selected and verification
//! precisions, each attempt's precision, role, outcome, corrections, gate,
//! work by stage (all 19 `StageWork` fields) and in total (K4's own limb
//! work and its exact-sum work), storage, and the report counts. No time, and
//! no binary64 value computed through FK's geometry (whose `hypot` is the
//! platform's): a witness is recorded by body only.
use crate::cases::Case;
use crate::compare::Tally;
use crate::lane::refusal_text;
use open_pipe_stress_frame_kernel::structural::retained_api::{
    AttemptRecord, CaseOutcome, InvocationMeter, StageWork, UnresolvedReason, WorkFault,
};
use serde_json::{json, Map, Value};

pub const SCHEMA: &str = "vk-case-record-v1";

fn stages(s: &StageWork) -> Value {
    json!({
        "formation": s.formation, "assembly": s.assembly,
        "residual_formation": s.residual_formation, "factor": s.factor,
        "condition": s.condition, "rhs": s.rhs, "solve": s.solve,
        "refinement": s.refinement, "recovery": s.recovery, "stop_rule": s.stop_rule,
        "bounded_gate": s.bounded_gate, "scale": s.scale, "estimate": s.estimate,
        "charge": s.charge, "bound": s.bound, "shift": s.shift,
        "bounded_formation": s.bounded_formation, "wide_formation": s.wide_formation,
        "uc": s.uc,
    })
}

fn attempt(a: &AttemptRecord) -> Value {
    json!({
        "precision": a.precision,
        "role": format!("{:?}", a.role),
        "outcome": format!("{:?}", a.outcome),
        "residual_basis": a.residual_basis,
        "corrections": a.corrections,
        "gate": a.gate.as_ref().map(|g| format!("{g:?}")),
        "own_work": a.work.limb_multiply_equivalents(),
        "exact_sum_work": a.k4_work.limb_multiply_equivalents(),
        "stages": stages(&a.stages),
        "shared_work": a.shared_work,
        "shared_stages": stages(&a.shared_stages),
        "shared_built_here": a.shared_built_here,
        "stop_rule_work": a.stop_rule_work,
        "verification_work": a.verification_work,
        "verification_shared_work": a.verification_shared_work,
        "verification_shared_built_here": a.verification_shared_built_here,
        "storage": {
            "pattern_entries": a.storage.pattern_entries,
            "profile_entries": a.storage.profile_entries,
            "limbs_per_entry": a.storage.limbs_per_entry,
        },
        "verification": a.verification.as_ref().map(|v| json!({
            "data_blocks": v.data_blocks,
            "shift_factorizations": v.shift_factorizations,
            "uc_missing": v.uc_missing,
            "g_max": v.g_max,
            "g_violation": v.g_violation,
        })),
    })
}

fn report(t: &Tally) -> Value {
    json!({
        "rows": t.rows, "pass": t.pass, "pass_absolute_range": t.pass_absolute_range,
        "not_covered": t.not_covered, "structural_zero": t.structural_zero,
        "expected_unresolved": t.expected_unresolved, "fail": t.fail,
    })
}

/// The outcome as text, without platform-dependent values.
pub fn outcome_text(o: &CaseOutcome) -> String {
    match o {
        CaseOutcome::Selected(s) => format!("Selected at {}", s.selected_precision()),
        CaseOutcome::Refused { refusal, geometry } => {
            format!("Refused {} {geometry:?}", refusal_text(refusal))
        }
        CaseOutcome::Unresolved {
            reason, geometry, ..
        } => format!("Unresolved {reason:?} {geometry:?}"),
    }
}

pub fn case_record(
    case: &Case,
    o: &CaseOutcome,
    t: &Tally,
    meter: &InvocationMeter,
) -> Result<Value, WorkFault> {
    if let CaseOutcome::Unresolved {
        reason: UnresolvedReason::WorkAccounting { fault, .. },
        ..
    } = o
    {
        return Err(*fault);
    }
    meter.checked_charged().exact()?;
    let attempts = match o {
        CaseOutcome::Selected(s) => s.evidence().attempts.as_slice(),
        CaseOutcome::Unresolved { attempts, .. } => attempts.as_slice(),
        CaseOutcome::Refused { .. } => &[],
    };
    let mut charged = 0u64;
    for a in attempts {
        a.checked_case_charge().exact()?;
        charged = charged
            .checked_add(a.checked_invocation_increment().exact()?)
            .ok_or(WorkFault::Overflow)?;
        a.checked_verification_work().exact()?;
        a.checked_stop_rule_work().exact()?;
        a.stages.checked_total().exact()?;
        a.shared_stages.checked_total().exact()?;
    }
    let mut r = Map::new();
    r.insert("schema".into(), json!(SCHEMA));
    r.insert("id".into(), json!(case.id));
    r.insert("family".into(), json!(case.family));
    r.insert("k4src_sha256".into(), json!(case.k4src_sha256));
    r.insert("outcome".into(), json!(outcome_text(o)));
    r.insert("case_limit".into(), json!(u64::MAX));
    r.insert("invocation_charged".into(), json!(meter.charged()));
    let attempts: Vec<Value> = match o {
        CaseOutcome::Selected(s) => {
            let e = s.evidence();
            r.insert("selected_precision".into(), json!(e.selected_precision));
            r.insert(
                "verification_precision".into(),
                json!(e.verification_precision),
            );
            r.insert("corrections".into(), json!(e.corrections));
            r.insert("geometry".into(), json!(format!("{:?}", e.geometry)));
            r.insert(
                "absolute_verified_rows".into(),
                json!(e.absolute_verified.len()),
            );
            r.insert("unpublishable_rows".into(), json!(e.unpublishable.len()));
            r.insert("published_rows".into(), json!(s.publish().rows.len()));
            e.attempts.iter().map(attempt).collect()
        }
        CaseOutcome::Unresolved { attempts, .. } => attempts.iter().map(attempt).collect(),
        CaseOutcome::Refused { .. } => Vec::new(),
    };
    r.insert("attempts".into(), Value::Array(attempts));
    r.insert("report".into(), report(t));
    Ok(Value::Object(r))
}

pub fn source_refused_record(case: &Case, error: &str, t: &Tally) -> Value {
    json!({
        "schema": SCHEMA, "id": case.id, "family": case.family,
        "k4src_sha256": case.k4src_sha256,
        "outcome": format!("SourceRefused {error}"), "attempts": [], "report": report(t),
    })
}

/// A family's records, as committed: a JSON array, keys sorted (serde_json's
/// default map), pretty-printed, a final newline.
pub fn family_file(records: &[Value]) -> String {
    let mut s = serde_json::to_string_pretty(&Value::Array(records.to_vec())).unwrap();
    s.push('\n');
    s
}
