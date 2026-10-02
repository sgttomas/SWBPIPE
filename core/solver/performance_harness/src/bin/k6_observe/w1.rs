//! K6b: the `w1a` mode's lines (T3 K6b plan §3.4; ROOT's rulings on I16's
//! plan, Q2): the outcome, one `attempt` line per attempt (a new kind; K6's
//! kinds keep their schema), the parity items, the rows dump and the prefix
//! lines.
//!
//! ROOT's rulings on the K6B-S3 stop and on RV22's review: the outcome's
//! per-precision work is the charged totals, and each attempt line carries the
//! work it was charged that no stage records (`own_unstaged`,
//! `shared_unstaged`; since KF3, zero on completed and stopped builds) and
//! whether that is none (`stages_complete`). Partial stages retain their
//! charged work, including a stop inside an own solve.

use super::Line;
use open_pipe_stress_frame_kernel::structural::retained_api::{
    AttemptRecord, CaseOutcome, GateTest, UnresolvedReason,
};
use open_pipe_stress_solver_performance_harness::k6::w1::counts::{
    fnv64, limbs_per_entry, W1Counts,
};
use open_pipe_stress_solver_performance_harness::k6::w1::rows::{class_counts, write_rows};
use open_pipe_stress_solver_performance_harness::k6::w1::staged::{
    attempts_of, charged_by, outcome_class, own_total, prefix_matches, stage_fields,
    stages_complete, stages_equal_totals, unstaged, work_by_precision, work_closes, W1Solve,
};
use open_pipe_stress_solver_performance_harness::k6::{debug_digest, Fnv64};
use std::io::Write;

/// At most this many characters of a `Debug` reason are printed.
const REASON_CHARS: usize = 240;

fn bounded(text: String) -> String {
    text.chars().take(REASON_CHARS).collect()
}

fn gate(g: &Option<GateTest>) -> Option<String> {
    g.as_ref().map(|g| match g {
        GateTest::Coalesced => "coalesced".to_string(),
        GateTest::Bounded { state, evaluated } => format!("bounded:{state}/{evaluated}"),
    })
}

/// Whether the call ended on a budget (a stop: the limits are `u64::MAX`).
pub fn budget_reached(solve: &W1Solve) -> bool {
    solve.exhausted
        || matches!(
            solve.outcome,
            CaseOutcome::Unresolved {
                reason: UnresolvedReason::Budget(_),
                ..
            }
        )
}

/// The repeat's determinism digest: the publication's and the evidence's
/// `Debug` digests (a selected case), or the outcome's (otherwise; it holds
/// only the reason and the attempts), and the charged work, folded into one
/// FNV-1a. The retained solve's caches are not formatted.
pub fn repeat_digest(solve: &W1Solve) -> (u64, u64) {
    let parts = match &solve.outcome {
        CaseOutcome::Selected(s) => vec![debug_digest(s.publish()), debug_digest(s.evidence())],
        other => vec![debug_digest(other)],
    };
    let mut h = Fnv64::new();
    let mut len = 0;
    for (l, f) in parts {
        len += l;
        h.update(&l.to_le_bytes());
        h.update(&f.to_le_bytes());
    }
    h.update(&solve.charged.to_le_bytes());
    (len, h.finish())
}

/// The outcome line of one repeat.
pub fn outcome_line(repeat: usize, solve: &W1Solve) {
    let attempts = attempts_of(&solve.outcome);
    let mut line = Line::new("outcome")
        .n("repeat", repeat)
        .s("class", outcome_class(&solve.outcome));
    line = match &solve.outcome {
        CaseOutcome::Selected(s) => {
            let publication = s.publish();
            let evidence = s.evidence();
            let counts = class_counts(publication);
            let (pub_len, pub_fnv) = debug_digest(publication);
            let (ev_len, ev_fnv) = debug_digest(evidence);
            line.n("selected_precision", s.selected_precision())
                .n("verification_precision", evidence.verification_precision)
                .null("reason")
                .n("rows", publication.rows.len())
                .n("rows_relative_verified", counts[0])
                .n("rows_absolute_verified", counts[1])
                .n("rows_input_derived", counts[2])
                .n("rows_unpublishable", counts[3])
                .n("publication_debug_len", pub_len)
                .hex("publication_fnv64", pub_fnv)
                .n("evidence_debug_len", ev_len)
                .hex("evidence_fnv64", ev_fnv)
                .hex(
                    "retained_state_fnv64",
                    fnv64(&evidence.retained_state_encoding),
                )
        }
        CaseOutcome::Refused { refusal, .. } => line
            .null("selected_precision")
            .null("verification_precision")
            .s("reason", &bounded(format!("{refusal:?}"))),
        CaseOutcome::Unresolved { reason, .. } => line
            .null("selected_precision")
            .null("verification_precision")
            .s("reason", &bounded(format!("{reason:?}"))),
    };
    line = line
        .n("attempts", attempts.len())
        .n("meter_charged", solve.charged)
        .b("budget_reached", budget_reached(solve));
    for w in work_by_precision(attempts) {
        line = line
            .n(&format!("work_{}_attempts", w.precision), w.attempts)
            .n(&format!("work_{}_own", w.precision), w.own_total)
            .n(&format!("work_{}_shared", w.precision), w.shared_total);
    }
    line.emit();
}

/// One `attempt` line per attempt.
pub fn attempt_lines(repeat: usize, attempts: &[AttemptRecord]) {
    for (index, a) in attempts.iter().enumerate() {
        let mut line = Line::new("attempt")
            .n("repeat", repeat)
            .n("index", index)
            .n("precision", a.precision)
            .s("role", &format!("{:?}", a.role))
            .s("outcome", &bounded(format!("{:?}", a.outcome)))
            .n("residual_basis", a.residual_basis)
            .n("corrections", a.corrections)
            .opt_s("gate", gate(&a.gate).as_deref())
            .f("pivot_margin_min", a.pivot_margin_min)
            .f("rcond", a.rcond)
            .f("residual_worst", a.residual_worst)
            .n("storage_pattern_entries", a.storage.pattern_entries)
            .n("storage_profile_entries", a.storage.profile_entries)
            .n("storage_limbs_per_entry", a.storage.limbs_per_entry);
        for (name, value) in stage_fields(&a.stages) {
            line = line.n(&format!("own_{name}"), value);
        }
        for (name, value) in stage_fields(&a.shared_stages) {
            line = line.n(&format!("shared_{name}"), value);
        }
        let v = a.verification.as_ref();
        let (own_unstaged, shared_unstaged) = unstaged(a);
        line.b("stages_complete", stages_complete(a))
            .n("own_unstaged", own_unstaged)
            .n("shared_unstaged", shared_unstaged)
            .n("own_total", own_total(a))
            .n("charged_by", charged_by(a))
            .n("shared_work", a.shared_work)
            .b("shared_built_here", a.shared_built_here)
            .n("stop_rule_work", a.stop_rule_work)
            .n("verification_work", a.verification_work)
            .n("verification_shared_work", a.verification_shared_work)
            .b(
                "verification_shared_built_here",
                a.verification_shared_built_here,
            )
            .opt_n("verification_data_blocks", v.map(|v| v.data_blocks))
            .opt_n(
                "verification_shift_factorizations",
                v.map(|v| v.shift_factorizations),
            )
            .opt_n("verification_uc_missing", v.and_then(|v| v.uc_missing))
            .opt_n("verification_g_max", v.map(|v| v.g_max))
            .opt_n("verification_g_violation", v.and_then(|v| v.g_violation))
            .emit();
    }
}

fn parity(item: &str, repeat: usize, equal: bool) -> Line {
    Line::new("parity")
        .s("item", item)
        .n("repeat", repeat)
        .b("equal", equal)
}

/// The W1 parity items (plan §3.4). A false item is a stop for the runner.
pub fn parity_lines(repeat: usize, counts: &W1Counts, solve: &W1Solve) {
    let attempts = attempts_of(&solve.outcome);
    if !attempts.is_empty() {
        parity(
            "w1_profile_equals_storage",
            repeat,
            attempts
                .iter()
                .all(|a| a.storage.profile_entries == counts.profile_entries),
        )
        .n("counts_profile_entries", counts.profile_entries)
        .emit();
        parity(
            "w1_pattern_equals_storage",
            repeat,
            attempts
                .iter()
                .all(|a| a.storage.pattern_entries == counts.pattern_entries),
        )
        .n("counts_pattern_entries", counts.pattern_entries)
        .emit();
        parity(
            "w1_limbs_equal_table",
            repeat,
            attempts
                .iter()
                .all(|a| a.storage.limbs_per_entry == limbs_per_entry(a.precision)),
        )
        .emit();
        parity(
            "w1_stages_equal_totals",
            repeat,
            stages_equal_totals(attempts),
        )
        .emit();
        parity(
            "w1_work_closes",
            repeat,
            work_closes(attempts, solve.charged),
        )
        .n("meter_charged", solve.charged)
        .emit();
    }
    if let CaseOutcome::Selected(s) = &solve.outcome {
        let fnv = fnv64(&s.evidence().source_encoding);
        parity(
            "w1_source_encoding_equals_evidence",
            repeat,
            fnv == counts.source_encoding_fnv64,
        )
        .hex("evidence_source_fnv64", fnv)
        .emit();
    }
    parity("w1_budget_not_reached", repeat, !budget_reached(solve)).emit();
}

/// Writes the selected publication's rows (`k6b-rows v1`).
pub fn dump_rows(path: &str, model_id: &str, solve: &W1Solve) {
    let CaseOutcome::Selected(s) = &solve.outcome else {
        return;
    };
    let file = std::fs::File::create(path).unwrap_or_else(|e| {
        eprintln!("k6_observe: cannot write the rows dump: {e}");
        std::process::exit(4)
    });
    let mut out = std::io::BufWriter::new(file);
    if write_rows(model_id, s.publish(), &mut out).is_err() || out.flush().is_err() {
        eprintln!("k6_observe: cannot write the rows dump");
        std::process::exit(4);
    }
}

/// The line of one prefix call (`prefix`, a new kind): its limit and segment,
/// its outcome, and whether its completed segments equal the full call's.
pub fn prefix_line(j: usize, label: &str, limit: u64, full: &[AttemptRecord], solve: &W1Solve) {
    let reason = match &solve.outcome {
        CaseOutcome::Unresolved { reason, .. } => Some(bounded(format!("{reason:?}"))),
        _ => None,
    };
    Line::new("prefix")
        .n("prefix", j)
        .s("through_segment", label)
        .n("case_limit", limit)
        .s("class", outcome_class(&solve.outcome))
        .opt_s("reason", reason.as_deref())
        .n("attempts", attempts_of(&solve.outcome).len())
        .n("meter_charged", solve.charged)
        .emit();
    parity(
        "w1_prefix_segments",
        j,
        prefix_matches(j, full, &solve.outcome),
    )
    .emit();
}
