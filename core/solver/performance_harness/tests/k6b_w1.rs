//! K6b: the W1 mode's library side (T3 K6b plan §6; ROOT's rulings on I16's
//! plan). Debug tests stay at 100 members or fewer. No time or memory bound is
//! asserted anywhere.
//!
//! - Determinism of outcomes, publications, evidence and work across two runs.
//! - The staged `w1_solve` equals K4's own `solve_case` and `solve_cases`.
//! - R1's unchanged predicate at 10 and 100 members, on every R1 row of the
//!   twelve RF-LARGE frames (`K4T/r1_large.txt`, its sha256 asserted).
//! - W1's counts against every attempt's storage; the work closure; the work by
//!   precision; the prefixes; the estimate against a hand derivation.
//! - A build that stops partway (ROOT's ruling on the K6B-S3 stop): its stages
//!   are checked against its charged total, and the remainder is unstaged.

mod k6b_support;

use k6b_support::{r1_rows, sha256_hex, R1_LARGE, R1_LARGE_SHA256};
use open_pipe_stress_frame_kernel::structural::retained_api::{
    solve_case, solve_cases, AttemptOutcome, AttemptReason, AttemptRecord, AttemptRole,
    AttemptStop, BudgetScope, CaseLimit, CaseOutcome, InvocationMeter, RetainedSolve, StageWork,
    UnresolvedReason,
};
use open_pipe_stress_solver_performance_harness::k6::counts::parse_counts_line;
use open_pipe_stress_solver_performance_harness::k6::models::model;
use open_pipe_stress_solver_performance_harness::k6::staged::{NoObserver, Stage};
use open_pipe_stress_solver_performance_harness::k6::w1::adapter::source;
use open_pipe_stress_solver_performance_harness::k6::w1::counts::{
    compute, estimate, limbs_per_entry, W1Counts, W1SizeFacts,
};
use open_pipe_stress_solver_performance_harness::k6::w1::rows::{r1_passes, r1_values};
use open_pipe_stress_solver_performance_harness::k6::w1::staged::{
    attempts_of, builds_completed, own_total, prefix_limits, prefix_matches, segments,
    shared_total, stage_sum, stages_complete, stages_equal_totals, unstaged, w1_solve,
    work_by_precision, work_closes, W1Limits, W1Solve,
};

fn rf(size: usize) -> Vec<String> {
    let mut ids = Vec::new();
    for family in ["CHAIN", "TREE", "CONT"] {
        for orientation in ["AX", "ROT"] {
            ids.push(format!("RF-LARGE-{family}-n{size:05}-{orientation}"));
        }
    }
    ids
}

fn run(id: &str, limits: W1Limits) -> W1Solve {
    let m = model(id).expect("model");
    let src = source(&m).expect("source");
    w1_solve(src, limits, Stage::W1Solve, &mut NoObserver)
}

fn selected(solve: &W1Solve) -> &RetainedSolve {
    match &solve.outcome {
        CaseOutcome::Selected(s) => s,
        other => panic!("not selected: {other:?}"),
    }
}

#[test]
fn outcomes_publications_evidence_and_work_are_deterministic() {
    for id in rf(10) {
        let (a, b) = (run(&id, W1Limits::default()), run(&id, W1Limits::default()));
        let (sa, sb) = (selected(&a), selected(&b));
        assert_eq!(sa.publish(), sb.publish(), "{id}");
        assert_eq!(sa.evidence(), sb.evidence(), "{id}");
        assert_eq!(a.charged, b.charged, "{id}");
    }
}

#[test]
fn the_staged_solve_equals_k4s_own_entries() {
    let id = "RF-LARGE-TREE-n00010-ROT";
    let staged = run(id, W1Limits::default());
    let m = model(id).unwrap();
    let mut meter = InvocationMeter::new(u64::MAX);
    let direct = solve_case(source(&m).unwrap(), CaseLimit::new(u64::MAX), &mut meter);
    let mut meter_all = InvocationMeter::new(u64::MAX);
    let all = solve_cases(
        &[source(&m).unwrap()],
        CaseLimit::new(u64::MAX),
        &mut meter_all,
    );
    assert_eq!(all.len(), 1);
    for other in [&direct, &all[0]] {
        let CaseOutcome::Selected(o) = other else {
            panic!("{id}: not selected")
        };
        assert_eq!(o.publish(), selected(&staged).publish());
        assert_eq!(o.evidence(), selected(&staged).evidence());
    }
    assert_eq!(meter.charged(), staged.charged);
    assert_eq!(meter_all.charged(), staged.charged);
}

/// Every R1 row of `id` under the unchanged predicate; W1's counts against the
/// attempts' storage; the closure identities.
fn check_r1_and_counts(id: &str) {
    let m = model(id).unwrap();
    let solve = run(id, W1Limits::default());
    let s = selected(&solve);
    assert_eq!(s.selected_precision(), 128, "{id}");
    let obs = r1_values(&m, s.publish());
    let rows = r1_rows(id);
    assert!(rows.len() > 100, "{id}: {} R1 rows", rows.len());
    for row in &rows {
        let o = *obs
            .get(&row.key)
            .unwrap_or_else(|| panic!("{id}: no published value for R1's {}", row.key));
        assert!(
            r1_passes(o, row.exp, row.scale),
            "{id} {}: obs {o:e}, exp {:e}, scale {:e}",
            row.key,
            row.exp,
            row.scale
        );
    }
    let (w1, error) = compute(&m);
    assert!(w1.source_ok && error.is_none(), "{id}");
    let attempts = attempts_of(&solve.outcome);
    for a in attempts {
        assert_eq!(a.storage.profile_entries, w1.profile_entries, "{id}");
        assert_eq!(a.storage.pattern_entries, w1.pattern_entries, "{id}");
        assert_eq!(
            a.storage.limbs_per_entry,
            limbs_per_entry(a.precision),
            "{id}"
        );
    }
    assert!(stages_equal_totals(attempts), "{id}");
    assert!(work_closes(attempts, solve.charged), "{id}");
    assert_eq!(w1.rows, s.publish().rows.len(), "{id}");
}

#[test]
fn r1_large_txt_is_the_pinned_file() {
    assert_eq!(sha256_hex(R1_LARGE.as_bytes()), R1_LARGE_SHA256);
}

#[test]
fn r1_predicate_and_counts_at_10_members() {
    for id in rf(10) {
        check_r1_and_counts(&id);
    }
}

#[test]
fn r1_predicate_and_counts_at_100_members_chain() {
    for id in ["RF-LARGE-CHAIN-n00100-AX", "RF-LARGE-CHAIN-n00100-ROT"] {
        check_r1_and_counts(id);
    }
}

#[test]
fn r1_predicate_and_counts_at_100_members_tree() {
    for id in ["RF-LARGE-TREE-n00100-AX", "RF-LARGE-TREE-n00100-ROT"] {
        check_r1_and_counts(id);
    }
}

#[test]
fn r1_predicate_and_counts_at_100_members_cont() {
    for id in ["RF-LARGE-CONT-n00100-AX", "RF-LARGE-CONT-n00100-ROT"] {
        check_r1_and_counts(id);
    }
}

fn stage_array(w: &StageWork) -> [u64; 19] {
    [
        w.formation,
        w.assembly,
        w.residual_formation,
        w.factor,
        w.condition,
        w.rhs,
        w.solve,
        w.refinement,
        w.recovery,
        w.stop_rule,
        w.bounded_gate,
        w.scale,
        w.estimate,
        w.charge,
        w.bound,
        w.shift,
        w.bounded_formation,
        w.wide_formation,
        w.uc,
    ]
}

#[test]
fn work_by_precision_files_each_attempt_under_its_own_precision() {
    let solve = run("RF-LARGE-CHAIN-n00010-ROT", W1Limits::default());
    let attempts: &[AttemptRecord] = attempts_of(&solve.outcome);
    let by = work_by_precision(attempts);
    assert_eq!(
        by.iter().map(|w| w.precision).collect::<Vec<_>>(),
        vec![128, 256]
    );
    for w in &by {
        let mine: Vec<&AttemptRecord> = attempts
            .iter()
            .filter(|a| a.precision == w.precision)
            .collect();
        assert_eq!(w.attempts, mine.len());
        let mut own = [0u64; 19];
        let mut shared = [0u64; 19];
        for a in &mine {
            for (k, v) in stage_array(&a.stages).iter().enumerate() {
                own[k] += v;
            }
            for (k, v) in stage_array(&a.shared_stages).iter().enumerate() {
                shared[k] += v;
            }
        }
        assert_eq!(stage_array(&w.own), own, "{}", w.precision);
        assert_eq!(stage_array(&w.shared), shared, "{}", w.precision);
        // Completed builds: the charged totals equal the stage sums.
        assert_eq!(w.own_total, own.iter().sum::<u64>(), "{}", w.precision);
        assert_eq!(
            w.shared_total,
            shared.iter().sum::<u64>(),
            "{}",
            w.precision
        );
    }
    // The verification pass and the stop rule are where K4 files them: the
    // 256 verification carries the pass (and the shift), the 128 candidate the
    // stop rule.
    assert!(by[1].own.scale > 0 && by[1].own.shift > 0);
    assert!(by[0].own.stop_rule > 0 && by[1].own.stop_rule == 0);
    // Each attempt's own stages add up to its own work.
    for a in attempts {
        assert!(builds_completed(a));
        assert_eq!(stage_sum(&a.stages), own_total(a));
        assert_eq!(unstaged(a), (0, 0));
    }
}

/// ROOT's ruling on the K6B-S3 stop: a build that stops partway. A case limit
/// inside the 256 verification's `uc` stage stops its shared build with
/// `Budget(Case)` on the error path row 247 took with `Span`
/// (`K4R/verify.rs:471-485`): the charged total holds the partial `uc` work,
/// and no stage records it.
#[test]
fn a_build_that_stops_partway_is_checked_against_its_charged_total() {
    let id = "RF-LARGE-CHAIN-n00010-AX";
    let full = run(id, W1Limits::default());
    let full_attempts = attempts_of(&full.outcome);
    let v = &full_attempts[1];
    assert_eq!((v.precision, v.role), (256, AttemptRole::Verification));
    assert!(v.verification_shared_built_here && v.shared_stages.uc > 1);
    // The verification's shared build starts where solve_256 ends and runs
    // bounded_formation, wide_formation, then uc.
    let segs = segments(full_attempts);
    assert_eq!(segs[1].label, "solve_256");
    let into_uc = segs[1].end
        + v.shared_stages.bounded_formation
        + v.shared_stages.wide_formation
        + v.shared_stages.uc / 2;
    let stopped = run(
        id,
        W1Limits {
            case: into_uc,
            invocation: u64::MAX,
        },
    );
    assert!(
        matches!(
            stopped.outcome,
            CaseOutcome::Unresolved {
                reason: UnresolvedReason::Budget(BudgetScope::Case),
                ..
            }
        ),
        "{:?}",
        stopped.outcome
    );
    let attempts = attempts_of(&stopped.outcome);
    assert_eq!(attempts.len(), 2);
    let (c, a) = (&attempts[0], &attempts[1]);
    assert_eq!(
        c.outcome,
        AttemptOutcome::Rejected(AttemptReason::VerificationFailed)
    );
    assert_eq!(
        a.outcome,
        AttemptOutcome::Failed(AttemptReason::Stop(AttemptStop::Budget(BudgetScope::Case)))
    );
    assert!(builds_completed(c) && !builds_completed(a));
    assert_eq!(unstaged(c), (0, 0));
    // The stopped build: its stages stop before uc, and the charged total
    // holds the partial uc work.
    assert_eq!(a.shared_stages.uc, 0);
    assert_eq!(
        a.shared_stages.wide_formation,
        v.shared_stages.wide_formation
    );
    let (own_rest, shared_rest) = unstaged(a);
    assert_eq!(own_rest, 0);
    assert!(shared_rest > 0);
    assert_eq!(stage_sum(&a.shared_stages) + shared_rest, shared_total(a));
    // The equality does not hold on it; the check does, and the work closes.
    assert_ne!(stage_sum(&a.shared_stages), shared_total(a));
    assert!(stages_equal_totals(attempts));
    assert!(work_closes(attempts, stopped.charged));
    // The work by precision reports the charged totals.
    let by = work_by_precision(attempts);
    assert_eq!(by[1].precision, 256);
    assert_eq!(by[1].shared_total, shared_total(a));
    assert_eq!(by[1].shared_total, stage_sum(&by[1].shared) + shared_rest);
    assert_eq!(by[1].own_total, own_total(a));
    // A stopped build's stage sum above its charged total fails the check.
    let mut over = attempts.to_vec();
    over[1].shared_stages.uc = shared_rest + 1;
    assert!(!stages_equal_totals(&over));
    // One build stopped, so at most one side is short (RV22-1): 1 LME off the
    // complete own side fails too.
    let mut both_short = attempts.to_vec();
    both_short[1].stages.rhs -= 1;
    assert!(!stages_equal_totals(&both_short));
    assert!(stages_complete(c) && !stages_complete(a));
    // A completed build is still held to the equality.
    let mut short = full_attempts.to_vec();
    short[0].stages.rhs -= 1;
    assert!(!stages_equal_totals(&short));
}

/// RV22-1: a candidate stopped in its stop rule (a case limit at the end of
/// the 256 verification) had every build complete, and K4 files the stop
/// rule's partial work in its stages. So it is held to equality: RV22's probe,
/// 1 LME under-recorded on either side, fails.
#[test]
fn a_stop_in_the_stop_rule_is_held_to_equality() {
    let id = "RF-LARGE-CHAIN-n00010-AX";
    let full = run(id, W1Limits::default());
    let segs = segments(attempts_of(&full.outcome));
    assert_eq!(segs[2].label, "verify_256");
    let stopped = run(
        id,
        W1Limits {
            case: segs[2].end,
            invocation: u64::MAX,
        },
    );
    let attempts = attempts_of(&stopped.outcome);
    let c = &attempts[0];
    assert_eq!(c.role, AttemptRole::Candidate);
    assert_eq!(
        c.outcome,
        AttemptOutcome::Failed(AttemptReason::Stop(AttemptStop::Budget(BudgetScope::Case)))
    );
    assert!(c.stop_rule_work > 0 && c.stages.stop_rule == c.stop_rule_work);
    assert!(builds_completed(c) && stages_complete(c));
    assert!(stages_equal_totals(attempts));
    assert!(work_closes(attempts, stopped.charged));
    let mut own_short = attempts.to_vec();
    own_short[0].stages.rhs -= 1;
    assert!(!stages_equal_totals(&own_short));
    let mut shared_short = attempts.to_vec();
    shared_short[0].shared_stages.factor -= 1;
    assert!(!stages_equal_totals(&shared_short));
}

#[test]
fn prefixes_stop_on_the_case_budget_after_each_segment() {
    let id = "RF-LARGE-CHAIN-n00010-AX";
    let full = run(id, W1Limits::default());
    let full_attempts = attempts_of(&full.outcome).to_vec();
    let segs = segments(&full_attempts);
    assert_eq!(
        segs.iter().map(|s| s.label.as_str()).collect::<Vec<_>>(),
        vec!["solve_128", "solve_256", "verify_256", "decide_128"]
    );
    assert_eq!(segs.last().unwrap().end, full.charged);
    let limits = prefix_limits(&full_attempts);
    assert_eq!(limits.len(), 3);
    // Each prefix limit is its segment's exact end (b_j), not a neighbour of it.
    assert_eq!(
        limits.iter().map(|l| l.1).collect::<Vec<_>>(),
        segs[..3].iter().map(|s| s.end).collect::<Vec<_>>()
    );
    for (j, (_, limit)) in limits.iter().enumerate() {
        let prefix = run(
            id,
            W1Limits {
                case: *limit,
                invocation: u64::MAX,
            },
        );
        assert!(
            matches!(
                prefix.outcome,
                CaseOutcome::Unresolved {
                    reason: UnresolvedReason::Budget(BudgetScope::Case),
                    ..
                }
            ),
            "prefix {}: {:?}",
            j + 1,
            prefix.outcome
        );
        let own = segments(attempts_of(&prefix.outcome));
        assert!(own.len() > j, "prefix {}", j + 1);
        assert_eq!(own[..=j], segs[..=j], "prefix {}", j + 1);
        // The binary's `w1_prefix_segments` (RV22-N3): true for prefix j + 1,
        // false one segment further (that segment was cut short).
        assert!(prefix_matches(j + 1, &full_attempts, &prefix.outcome));
        assert!(!prefix_matches(j + 2, &full_attempts, &prefix.outcome));
        assert!(
            prefix.charged > *limit,
            "the next segment starts before it stops"
        );
    }
    // With the full call's own work as the limit, the call completes.
    let exact = run(
        id,
        W1Limits {
            case: full.charged,
            invocation: u64::MAX,
        },
    );
    assert_eq!(selected(&exact).publish(), selected(&full).publish());
    // A completed call is not a budget-truncated prefix.
    assert!(!prefix_matches(1, &full_attempts, &full.outcome));
}

/// The W1 estimate derived by hand from the counts, term by term (plan §3.5;
/// KF1's bounds; ROOT's ruling on RV22-2), with the phase that sets E_max and
/// E_sel128. Every constant is written out here, independently of
/// `counts.rs`.
struct Hand {
    fixed: u128,
    shared: [u128; 4],
    state: [u128; 4],
    solve: [u128; 4],
    verify: [u128; 3],
    pass: [u128; 3],
    decide: u128,
    max: u128,
    sel128: u128,
    max_at: String,
    sel_at: String,
}

fn hand(c: &W1Counts, s: &W1SizeFacts) -> Hand {
    let z = |x: usize| x as u128;
    let (m, n, nf, nnz) = (
        z(c.members),
        z(c.dofs),
        z(c.free_dofs),
        z(c.pattern_entries),
    );
    let (p, b, rows) = (z(c.profile_entries), z(c.blocks.max(1)), z(c.rows));
    let (nodes, r, loads, stations, enc) = (
        z(c.nodes),
        z(c.constraints),
        z(c.loads),
        z(c.stations),
        z(c.source_encoding_len),
    );
    let w = |l: u128| 8 * l + 16;
    let harness = nodes * 64 + m * 80 + loads * 16 + m * z(s.frame_element);
    let src = 24 * nodes
        + m * z(s.straight_member)
        + r * z(s.constraint)
        + loads * (z(s.nodal_load) + 16)
        + stations * z(s.station)
        + 16 * n
        + 4 * nodes;
    let case = loads * 64 + r * 48 + enc + rows * z(s.quantity_meta) + 8 * z(c.bodies);
    let group =
        8 * (n + 1) + 16 * nnz + 8 * (nnz + 1) + m * 78 * 8 + 32 * nf + 8 * n + 12 * nf + 24 * b;
    let fixed = harness + 2 * src + case + group;
    let shared = |l: u128, r: u128| {
        m * (164 * w(l) + 16)
            + nnz * (w(l) + w(r))
            + m * (5 * w(r) + 8)
            + p * w(l)
            + nf * (2 * w(l) + 56)
            + b * w(l)
    };
    let state = |l: u128| (n + 6 * m + rows) * w(l);
    let verify = |l: u128, ww: u128| nnz * w(l) + m * 144 * w(ww) + b * (4 * w(l) + 8);
    // KF1's bounds (KF1 RETURN addendum 2): a row is 4,304 B, a table entry
    // 40 B; the stop rule peaks at G + T = 4,608 rows, the pivot margin's
    // tracker at 768, a solve attempt's trackers at 2,816. Tables: 2 × their
    // entries in place, plus one more entry per row for the one growing under
    // the move model.
    let row = 2 * 2144 + 16;
    let decide = 4608 * row + (3 * 2 + 1) * rows * 40;
    let pivot = 768 * row + (2 + 1) * nf * 40;
    // The solve: 4 working vectors at R over n, the gate's rows, the trackers
    // and tables; the fallback's u_free, 4 evaluated states and abar_q, then
    // the larger of assemble_bounded's member blocks and (a state's copy of u
    // with the row list: capacity C = 2^⌈log2 n_f⌉, C/2 + C rows at its last
    // growth under the move model).
    let mut cap = 1u128;
    while cap < nf {
        cap *= 2;
    }
    let row_list = if nf == 0 { 0 } else { cap + cap / 2 };
    let solve = |l: u128, r: u128| {
        4 * n * w(r)
            + nf * (16 + w(l))
            + 2816 * row
            + (2 * 5 + 1) * nf * 40
            + nf * w(l)
            + 4 * (nf * w(l) + 24)
            + nnz * w(r)
            + (m * 144 * w(r)).max(n * w(l) + row_list * row)
    };
    // The verification pass: 3 n and 8 n_f values, recover's rows + 6m, 50 B
    // per DOF and the prescribed terms, with either 3 row vectors and the
    // shift's two profiles or all 5 row vectors.
    let pass = |l: u128, ww: u128| {
        let live =
            3 * n * w(l) + 8 * nf * w(l) + (rows + 6 * m) * w(l) + 50 * n + r * (w(ww) + w(l));
        let at_shift =
            3 * rows * (w(l) + 8) + p * w(l) + 36 * nf + p * w(l) + nf * (32 + (w(l) + 8) + w(l));
        live + at_shift.max(5 * rows * (w(l) + 8))
    };
    let build =
        |l: u128, r: u128, same: bool| shared(l, r) + if same { 0 } else { m * (164 * w(r) + 16) };
    let vbuild = |l: u128, ww: u128, same: bool| {
        verify(l, ww)
            + m * (5 * w(l) + 8)
            + m * 144 * w(l)
            + if same { 0 } else { m * (164 * w(ww) + 16) }
    };
    let report = |l: u128| 5 * rows * (w(l) + 8) + 2 * nf * w(l);
    let end =
        rows * (z(s.published_row) + 40) + enc + (n + 6 * m) * (9 + 128) + 8 * z(s.attempt_record);
    let mut kept = fixed;
    let mut peak = (fixed + m * 78 * 32, "group".to_string());
    let mut sel = (0, String::new());
    let up = |peak: &mut (u128, String), v: u128, at: String| {
        if v > peak.0 {
            *peak = (v, at);
        }
    };
    // (p, L, R, shared build at R?, verification (L_P, W, member operators at W?))
    let steps: [(u32, u128, u128, bool, Option<(u128, u128, bool)>); 4] = [
        (128, 4, 4, false, None),
        (256, 4, 8, false, Some((4, 8, false))),
        (512, 8, 16, false, Some((8, 16, false))),
        (1024, 16, 16, true, Some((16, 16, true))),
    ];
    for (pr, l, r, same, v) in steps {
        up(
            &mut peak,
            kept + build(l, r, same) + pivot,
            format!("build_{pr}"),
        );
        kept += shared(l, r);
        up(
            &mut peak,
            kept + state(l) + solve(l, r),
            format!("solve_{pr}"),
        );
        kept += state(l);
        if let Some((vl, vw, vsame)) = v {
            up(
                &mut peak,
                kept + vbuild(vl, vw, vsame),
                format!("vbuild_{pr}"),
            );
            kept += verify(vl, vw);
            up(&mut peak, kept + pass(vl, vw), format!("pass_{pr}"));
            up(
                &mut peak,
                kept + report(vl) + decide,
                format!("decide_{pr}"),
            );
            if pr == 256 {
                sel = peak.clone();
                up(&mut sel, kept + report(vl) + end, "end_128".to_string());
            }
        }
    }
    up(&mut peak, kept + report(16) + end, "end".to_string());
    Hand {
        fixed,
        shared: [shared(4, 4), shared(4, 8), shared(8, 16), shared(16, 16)],
        state: [state(4), state(4), state(8), state(16)],
        solve: [solve(4, 4), solve(4, 8), solve(8, 16), solve(16, 16)],
        verify: [verify(4, 8), verify(8, 16), verify(16, 16)],
        pass: [pass(4, 8), pass(8, 16), pass(16, 16)],
        decide,
        max: peak.0,
        sel128: sel.0,
        max_at: peak.1,
        sel_at: sel.1,
    }
}

fn assert_estimate_is_hand(c: &W1Counts, id: &str) -> Hand {
    let s = W1SizeFacts::of_this_build();
    let (e, h) = (estimate(c, &s), hand(c, &s));
    assert_eq!(e.fixed, h.fixed, "{id}");
    assert_eq!(e.shared, h.shared, "{id}");
    assert_eq!(e.state, h.state, "{id}");
    assert_eq!(e.solve, h.solve, "{id}");
    assert_eq!(e.verify, h.verify, "{id}");
    assert_eq!(e.pass, h.pass, "{id}");
    assert_eq!(e.decide, h.decide, "{id}");
    assert_eq!(e.sel128, h.sel128, "{id}");
    assert_eq!(e.max, h.max, "{id}");
    h
}

/// The estimate by hand for RF-LARGE-CHAIN-n00010-AX, from its counts.
#[test]
fn the_estimate_equals_a_hand_derivation() {
    let m = model("RF-LARGE-CHAIN-n00010-AX").unwrap();
    let (c, _) = compute(&m);
    let expected_counts = W1Counts {
        source_ok: true,
        nodes: 11,
        members: 10,
        stations: 10,
        constraints: 6,
        loads: 6,
        dofs: 66,
        free_dofs: 60,
        bodies: 1,
        pattern_entries: 1116,
        profile_entries: 534,
        half_bandwidth: 11,
        blocks: 1,
        rows: 263,
        source_encoding_len: 1512,
        source_encoding_fnv64: c.source_encoding_fnv64,
    };
    assert_eq!(c, expected_counts);
    let h = assert_estimate_is_hand(&c, "RF-LARGE-CHAIN-n00010-AX");
    // At 10 members the stop rule's tracker term sets both.
    assert_eq!(
        (h.max_at.as_str(), h.sel_at.as_str()),
        ("decide_1024", "decide_256")
    );
}

const COUNTS: &str = include_str!("../observations/k6b/counts.jsonl");

fn number(line: &str, key: &str) -> u128 {
    let pattern = format!("\"{key}\":");
    let start = line.find(&pattern).expect(key) + pattern.len();
    line[start..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .expect(key)
}

/// RV22-3: every committed counts line carries this code's E_max and
/// E_sel128, so the runner's admission (from the file) and the binary's
/// backstop (recomputed from the file's counts) use one figure.
#[test]
fn the_committed_counts_carry_this_codes_estimate() {
    let s = W1SizeFacts::of_this_build();
    let mut lines = 0;
    for line in COUNTS.lines().filter(|l| !l.trim().is_empty()) {
        let (id, counts, _) = parse_counts_line(line).expect("a counts line");
        let w1 = counts.w1.expect("W1 counts");
        let e = estimate(&w1, &s);
        assert_eq!(e.max, number(line, "estimate_adm_bytes_w1a"), "{id}");
        assert_eq!(e.sel128, number(line, "estimate_w1_sel128_bytes"), "{id}");
        lines += 1;
    }
    assert_eq!(lines, 33);
}

/// RV22-3: at 1,000 and 10,000 members (from the committed counts; nothing is
/// solved), the solve-phase terms set E_sel128, and at 10,000 members a solve
/// or pass term sets E_max; the estimate equals the hand derivation there.
#[test]
fn the_solve_and_pass_terms_bind_on_the_large_models() {
    for line in COUNTS.lines().filter(|l| !l.trim().is_empty()) {
        let (id, counts, _) = parse_counts_line(line).expect("a counts line");
        if !(id.contains("-n01000-") || id.contains("-n10000-")) {
            continue;
        }
        let h = assert_estimate_is_hand(&counts.w1.expect("W1 counts"), &id);
        assert_eq!(h.sel_at, "solve_256", "{id}");
        if id.contains("-n10000-") {
            assert!(
                h.max_at.starts_with("solve_") || h.max_at.starts_with("pass_"),
                "{id}: {}",
                h.max_at
            );
        }
    }
}
