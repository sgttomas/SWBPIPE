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

mod k6b_support;

use k6b_support::{r1_rows, sha256_hex, R1_LARGE, R1_LARGE_SHA256};
use open_pipe_stress_frame_kernel::structural::retained_api::{
    solve_case, solve_cases, AttemptRecord, BudgetScope, CaseLimit, CaseOutcome, InvocationMeter,
    RetainedSolve, StageWork, UnresolvedReason,
};
use open_pipe_stress_solver_performance_harness::k6::models::model;
use open_pipe_stress_solver_performance_harness::k6::staged::{NoObserver, Stage};
use open_pipe_stress_solver_performance_harness::k6::w1::adapter::source;
use open_pipe_stress_solver_performance_harness::k6::w1::counts::{
    compute, estimate, limbs_per_entry, W1Counts, W1SizeFacts,
};
use open_pipe_stress_solver_performance_harness::k6::w1::rows::{r1_passes, r1_values};
use open_pipe_stress_solver_performance_harness::k6::w1::staged::{
    attempts_of, own_total, prefix_limits, segments, stage_sum, stages_equal_totals, w1_solve,
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
    }
    // The verification pass and the stop rule are where K4 files them: the
    // 256 verification carries the pass (and the shift), the 128 candidate the
    // stop rule.
    assert!(by[1].own.scale > 0 && by[1].own.shift > 0);
    assert!(by[0].own.stop_rule > 0 && by[1].own.stop_rule == 0);
    // Each attempt's own stages add up to its own work.
    for a in attempts {
        assert_eq!(stage_sum(&a.stages), own_total(a));
    }
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
}

/// The estimate by hand, term by term, for RF-LARGE-CHAIN-n00010-AX (plan
/// §3.5), with the sizes of this build.
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
    let s = W1SizeFacts::of_this_build();
    let e = estimate(&c, &s);
    let (m, n, nf, nnz, p, b, rows) = (10u128, 66u128, 60u128, 1116u128, 534u128, 1u128, 263u128);
    let w = |l: u128| 8 * l + 16;
    let z = |x: usize| x as u128;
    let harness = 11 * 64 + m * 80 + 6 * 16 + m * z(s.frame_element);
    let src = 24 * 11
        + m * z(s.straight_member)
        + 6 * z(s.constraint)
        + 6 * (z(s.nodal_load) + 16)
        + 10 * z(s.station)
        + 16 * n
        + 4 * 11;
    let case = 6 * 64 + 6 * 48 + 1512 + rows * z(s.quantity_meta) + 8;
    let group =
        8 * (n + 1) + 16 * nnz + 8 * (nnz + 1) + m * 78 * 8 + 32 * nf + 8 * n + 12 * nf + 24;
    let fixed = harness + 2 * src + case + group;
    assert_eq!(e.fixed, fixed);
    let shared = |l: u128, r: u128| {
        m * (164 * w(l) + 16)
            + nnz * (w(l) + w(r))
            + m * (5 * w(r) + 8)
            + p * w(l)
            + nf * (2 * w(l) + 56)
            + b * w(l)
    };
    assert_eq!(
        e.shared,
        [shared(4, 4), shared(4, 8), shared(8, 16), shared(16, 16)]
    );
    let state = |l: u128| (n + 6 * m + rows) * w(l);
    assert_eq!(e.state, [state(4), state(4), state(8), state(16)]);
    let verify = |l: u128, ww: u128| nnz * w(l) + m * 144 * w(ww) + b * (4 * w(l) + 8);
    assert_eq!(e.verify, [verify(4, 8), verify(8, 16), verify(16, 16)]);
    // KF1's bounds (KF1 RETURN addendum 2): a row is 4,304 B, a table entry
    // 40 B; the stop rule peaks at G + T = 4,608 rows, the pivot margin's
    // tracker at 768, a solve attempt's trackers at 2,816; the fallback's
    // per-state row list holds n_f rows in a growing Vec.
    let row = 2 * 2144 + 16;
    let decide = 4608 * row + 3 * 2 * rows * 40;
    assert_eq!(e.decide, decide);
    let pivot = 768 * row + 2 * nf * 40;
    let solve_trackers = 2816 * row + 2 * 5 * nf * 40 + 2 * nf * row;
    let solve = |l: u128, r: u128| 4 * n * w(r) + nf * (16 + w(l)) + solve_trackers;
    // The 128-selected path: build 128, state 128, build 256, state 256, the
    // v256 build, the pass, the decision, the end.
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
        rows * (z(s.published_row) + 40) + 1512 + (n + 6 * m) * (9 + 128) + 8 * z(s.attempt_record);
    let group_build = m * 78 * 32;
    let mut kept = fixed;
    let mut peak = fixed + group_build;
    let mut sel = 0;
    let steps: [(u128, u128, bool, Option<(u128, u128, bool)>); 4] = [
        (4, 4, false, None),
        (4, 8, false, Some((4, 8, false))),
        (8, 16, false, Some((8, 16, false))),
        (16, 16, true, Some((16, 16, true))),
    ];
    for (k, (l, r, same, v)) in steps.into_iter().enumerate() {
        peak = peak.max(kept + build(l, r, same) + pivot);
        kept += shared(l, r);
        peak = peak.max(kept + state(l) + solve(l, r));
        kept += state(l);
        if let Some((vl, vw, vsame)) = v {
            peak = peak.max(kept + vbuild(vl, vw, vsame));
            kept += verify(vl, vw);
            peak = peak.max(kept + report(vl) + p * w(vl));
            peak = peak.max(kept + report(vl) + decide);
            if k == 1 {
                sel = peak.max(kept + report(vl) + end);
            }
        }
    }
    assert_eq!(e.sel128, sel);
    assert_eq!(e.max, peak.max(kept + report(16) + end));
}
