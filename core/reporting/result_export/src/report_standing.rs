//! The standing record a report states (DEL-08-01 REQ-024; DEL-05-02
//! Standing), derived by the reader from the bound result and its model.
//!
//! This is the HISTORICAL reader. It reads results that carry no numerical
//! evidence member, which today is every result: before the producer emits
//! typed causes and confirmation outcomes, it recovers them from the result's
//! published integrity code, its `solve_quality`, and fixed diagnostic text at
//! fixed positions, under the `historical_*` vocabularies of
//! `open_pipe_stress_numerical_vocabulary`. It stays the supported way to read
//! results made before typed emission. What it recovers, and from what text:
//!
//! - Each load case's health: its `numerical_quality` record's `solve_quality`.
//! - Its integrity code: the code of its `diagnostic:numerical-integrity:<case>`
//!   diagnostic, one of the eight the producer publishes; any other code is
//!   refused by name, never given a known meaning.
//! - Its causes: `historical_health`'s parse, unchanged (conditioning from the
//!   integrity report's reciprocal condition estimate on a linear case; load
//!   fidelity and short curved bends from their own diagnostics; formation from
//!   the formation check's estimate line or the S11-G load-row sentence;
//!   formation recovery from the S11-G recovery-guard sentence or the
//!   member-relative check being unavailable on the route; the gap, mechanism
//!   and negative-energy causes from the code alone).
//! - Its support-state confirmation, from the end of the integrity message: the
//!   producer's confirmed-state line, alone or followed by the S11-G recovery
//!   sentence (`solver_reported_established`); or the producer's not-established
//!   sentence, whose reason is "confirmation attempt: not run"
//!   (`not_attempted_reason_not_recorded`: the text does not say why) or any
//!   other text (`not_established_detail_quoted`: the reason is quoted as the
//!   solver's detail and given no meaning; only the first failing support's
//!   reason is ever published). A case with nonlinear supports and neither
//!   sentence has `no_confirmation_record`; a case of a model with no nonlinear
//!   support, `not_applicable`.
//! - Each nonlinear support's computed state, from its
//!   `nonlinear_support_active_set_state_code` row in that case, and its
//!   behaviour and friction normal source from the model.
//!
//! Evidence status in this record, under DEL-08-01 REQ-024's present account:
//! with no evidence member, every member and support error figure is
//! unavailable (`older_document`); a confirmation the solver reports
//! established is conditional on the inverse-norm step; every other
//! confirmation outcome is not established. Nothing here is upgraded.
use crate::historical_health::{numerical_health, Location};
use open_pipe_stress_numerical_vocabulary::{
    CaseHealth, EvidenceStatus, FrictionNormal, HistoricalCause, HistoricalConfirmation, HistoricalIntegrityCode, NoFigureReason, Premise,
    SupportBehavior, SupportState,
};
use serde_json::{json, Value};
use std::collections::HashMap;

/// The record's kind and version: the historical reader's first form.
pub const RECORD_VERSION: &str = "historical_1";
/// A tag outside the vocabulary its reader supports.
pub const NUMERICAL_TAG_UNSUPPORTED: &str = "NUMERICAL_TAG_UNSUPPORTED";
/// product_physics `nonlinear_not_established_sentence`, as appended (after a
/// space) to the integrity message: `{LEAD}{reason}{TAIL}`.
const NOT_ESTABLISHED_LEAD: &str = " the converged support state is not established at 128-bit precision (";
const NOT_ESTABLISHED_TAIL: &str = "); member rows are binary64 recoveries";
/// The reason the producer gives when no confirmation was attempted.
const NOT_RUN: &str = "confirmation attempt: not run";
const RECOVERY_LEAD: &str = " S11-G recovery guard (R-b'): load case ";
const SENSITIVE_BECAUSE: &str = " is Sensitive because ";
const STATE_CODE_KIND: &str = "nonlinear_support_active_set_state_code";

/// The standing record of `source` with its `model`, or the refusal
/// `NUMERICAL_TAG_UNSUPPORTED: <vocabulary>` for a tag outside the
/// historical vocabularies. Callers derive it only for a result the reader
/// reads (`reader_readability::readability`).
pub fn historical_standing_record(source: &Value, model: &Value) -> Result<Value, String> {
    let unsupported = |vocabulary: &str| format!("{NUMERICAL_TAG_UNSUPPORTED}: {vocabulary}");
    let supports = nonlinear_supports(model).map_err(|v| unsupported(v))?;
    let integrity_messages: HashMap<&str, &str> = source["diagnostics"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[])
        .iter()
        .filter_map(|d| Some((d["id"].as_str()?, d["message"].as_str()?)))
        .collect();
    let states = support_states(source);
    let health = numerical_health(source, None);
    let mut counts: Vec<(CaseHealth, usize)> = CaseHealth::ALL.iter().map(|h| (*h, 0)).collect();
    let mut cases = Vec::new();
    for case in &health.cases {
        let case_health = CaseHealth::parse(case.state).ok_or_else(|| unsupported(CaseHealth::VOCABULARY))?;
        counts.iter_mut().find(|(h, _)| *h == case_health).expect("every health is counted").1 += 1;
        let code = match &case.code {
            Some(code) => Some(HistoricalIntegrityCode::parse(code).ok_or_else(|| unsupported(HistoricalIntegrityCode::VOCABULARY))?),
            None => None,
        };
        let mut causes = Vec::new();
        for cause in &case.causes {
            let tag = HistoricalCause::parse(cause.kind).ok_or_else(|| unsupported(HistoricalCause::VOCABULARY))?;
            causes.push(json!({
                "cause": tag.as_str(),
                "diagnostic_id": cause.diagnostic_id,
                "locations": cause.locations.iter().map(location_json).collect::<Vec<_>>(),
            }));
        }
        let message = case.integrity_diagnostic_id.as_deref().and_then(|id| integrity_messages.get(id)).copied();
        let (outcome, detail) = if supports.is_empty() {
            (HistoricalConfirmation::NotApplicable, None)
        } else {
            confirmation(message.unwrap_or(""), &case.case_id)
        };
        let support_records: Vec<Value> = supports
            .iter()
            .map(|support| {
                let (state, row) = states
                    .get(&(case.case_id.as_str(), support.id))
                    .map(|(state, row)| (*state, Some(*row)))
                    .unwrap_or((SupportState::NotPublished, None));
                json!({
                    "support_id": support.id,
                    "behavior": support.behavior.as_str(),
                    "friction_normal": support.friction_normal.map(FrictionNormal::as_str),
                    "state": state.as_str(),
                    "state_row_id": row,
                })
            })
            .collect();
        cases.push(json!({
            "case_id": case.case_id,
            "health": case_health.as_str(),
            "integrity_code": code.map(HistoricalIntegrityCode::as_str),
            "integrity_diagnostic_id": case.integrity_diagnostic_id,
            "causes": causes,
            "confirmation": {
                "outcome": outcome.as_str(),
                "detail": detail,
                "status": confirmation_status(outcome).map(EvidenceStatus::as_str),
                "premises": if outcome == HistoricalConfirmation::SolverReportedEstablished { vec![Premise::InverseNormStep.as_str()] } else { Vec::new() },
            },
            "supports": support_records,
        }));
    }
    Ok(json!({
        "record_version": RECORD_VERSION,
        "evidence_member": "absent",
        "figures": {"status": EvidenceStatus::Unavailable.as_str(), "reason": NoFigureReason::OlderDocument.as_str()},
        "counts": counts.iter().map(|(h, n)| json!({"health": h.as_str(), "cases": n})).collect::<Vec<_>>(),
        "cases": cases,
    }))
}

/// The present account's status of a confirmation outcome (DEL-08-01 REQ-024):
/// the solver's reported establishment is conditional on the inverse-norm
/// step; every other attempted or missing outcome is not established; a case
/// without nonlinear supports has none.
fn confirmation_status(outcome: HistoricalConfirmation) -> Option<EvidenceStatus> {
    match outcome {
        HistoricalConfirmation::SolverReportedEstablished => Some(EvidenceStatus::Conditional),
        HistoricalConfirmation::NotAttemptedReasonNotRecorded
        | HistoricalConfirmation::NotEstablishedDetailQuoted
        | HistoricalConfirmation::NoConfirmationRecord => Some(EvidenceStatus::NotEstablished),
        HistoricalConfirmation::NotApplicable => None,
    }
}

/// The confirmation the end of a case's integrity message records, with the
/// quoted reason where the outcome carries one. Recognized only at the
/// message's end, as the producer appends it.
fn confirmation(message: &str, case_id: &str) -> (HistoricalConfirmation, Option<String>) {
    let confirmed = format!(" {}", crate::historical_health::NONLINEAR_CONFIRMED_LINE);
    if let Some(head) = message.strip_suffix(NOT_ESTABLISHED_TAIL) {
        if message.matches(NOT_ESTABLISHED_LEAD).count() == 1 {
            let start = head.rfind(NOT_ESTABLISHED_LEAD).expect("counted once") + NOT_ESTABLISHED_LEAD.len();
            let reason = &head[start..];
            return if reason == NOT_RUN {
                (HistoricalConfirmation::NotAttemptedReasonNotRecorded, None)
            } else {
                (HistoricalConfirmation::NotEstablishedDetailQuoted, Some(reason.to_string()))
            };
        }
        return (HistoricalConfirmation::NoConfirmationRecord, None);
    }
    if message.matches(confirmed.as_str()).count() == 1 {
        let end = message.find(confirmed.as_str()).expect("counted once") + confirmed.len();
        let rest = &message[end..];
        let recovery = [RECOVERY_LEAD, case_id, SENSITIVE_BECAUSE].concat();
        if rest.is_empty() || rest.starts_with(recovery.as_str()) {
            return (HistoricalConfirmation::SolverReportedEstablished, None);
        }
    }
    (HistoricalConfirmation::NoConfirmationRecord, None)
}

struct NonlinearSupport<'a> {
    id: &'a str,
    behavior: SupportBehavior,
    friction_normal: Option<FrictionNormal>,
}

/// The model's nonlinear supports, in model order; the vocabulary name of an
/// unsupported behaviour spelling as the error.
fn nonlinear_supports(model: &Value) -> Result<Vec<NonlinearSupport<'_>>, &'static str> {
    let mut out = Vec::new();
    for support in model["supports"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        let nonlinear = &support["nonlinear"];
        if nonlinear.is_null() {
            continue;
        }
        let behavior = match nonlinear["behavior"].as_str() {
            Some("one_way" | "one-way" | "oneway") => SupportBehavior::OneWay,
            Some("lift_off" | "lift-off" | "liftoff") => SupportBehavior::LiftOff,
            Some("gap") => SupportBehavior::Gap,
            Some("friction") => SupportBehavior::Friction,
            _ => return Err(SupportBehavior::VOCABULARY),
        };
        let friction_normal = (behavior == SupportBehavior::Friction).then(|| {
            if nonlinear["normal_reaction_source"].is_object() { FrictionNormal::Derived } else { FrictionNormal::Explicit }
        });
        let Some(id) = support["id"].as_str() else { return Err(SupportBehavior::VOCABULARY) };
        out.push(NonlinearSupport { id, behavior, friction_normal });
    }
    Ok(out)
}

/// Each (load case, support) state row's state and row id. A state code outside
/// the published convention (0 inactive, 1 active, 2 sticking, 3 sliding) reads
/// as not published.
fn support_states(source: &Value) -> HashMap<(&str, &str), (SupportState, &str)> {
    let mut out = HashMap::new();
    for row in source["results"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        if row["kind"] != STATE_CODE_KIND || row["basis_ref"]["ref_type"] != "load_case" {
            continue;
        }
        let (Some(case), Some(support), Some(id)) = (row["basis_ref"]["ref_id"].as_str(), row["entity_ref"].as_str(), row["id"].as_str()) else {
            continue;
        };
        let state = match row["value"].as_f64() {
            Some(v) if v == 0.0 => SupportState::Inactive,
            Some(v) if v == 1.0 => SupportState::Active,
            Some(v) if v == 2.0 => SupportState::Sticking,
            Some(v) if v == 3.0 => SupportState::Sliding,
            _ => SupportState::NotPublished,
        };
        out.insert((case, support), (state, id));
    }
    out
}

fn location_json(location: &Location) -> Value {
    serde_json::to_value(location).expect("a location serializes")
}
