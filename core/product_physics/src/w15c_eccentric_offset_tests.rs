//! W-15 slice c (DEL-03-05 Design `eccentric-reducer-offset.md` §3.2-§3.4, §9
//! P2 and P3, §12): the pure parts of the eccentric offset check. The run-axis
//! decision and the judgement are exercised with the decided tolerance rule's
//! own examples, the bounds from both sides, and the convention-invariance case
//! (a sign-only convention change gives bit-identical measures). Every input is
//! invented.
use crate::rigid_component::{
    arc_evidence, decide_axis, judge_offset, straight_beta, EndStatus, Evidence, OffsetOutcome, AXIS_ANGLE_BASE_RAD,
    COORDINATE_ROUNDING_ALLOWANCE_M, TRANSVERSE_RATIO_BASE,
};

const MM: f64 = 1.0e-3;
const TAU: f64 = 5.0e-3;

fn unit(v: [f64; 3]) -> [f64; 3] {
    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / n, v[1] / n, v[2] / n]
}

fn len(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

/// A straight run member's end status from its two endpoints.
fn run(from: [f64; 3], to: [f64; 3]) -> EndStatus {
    let d = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
    EndStatus::Direction { direction: unit(d), beta: straight_beta(len(d)).expect("resolvable") }
}

fn mm(v: [f64; 3]) -> [f64; 3] {
    [v[0] * MM, v[1] * MM, v[2] * MM]
}

fn outcome_name(o: &OffsetOutcome) -> String {
    match o {
        OffsetOutcome::Realized(_) => "realized".into(),
        OffsetOutcome::Invalid(reason, _) => format!("invalid:{reason}"),
        OffsetOutcome::BelowPrecision(_) => "below_precision".into(),
        OffsetOutcome::NotTransverse(_) => "not_transverse".into(),
        OffsetOutcome::Mismatch(_) => "mismatch".into(),
    }
}

/// Decide and judge for ends (N, F) with their statuses.
fn check(x_n: [f64; 3], x_f: [f64; 3], n: &EndStatus, f: &EndStatus, e: [f64; 3]) -> OffsetOutcome {
    match decide_axis(n, f) {
        Ok(axis) => judge_offset(x_n, x_f, axis, e, TAU),
        Err((reason, detail)) => OffsetOutcome::Invalid(reason, detail),
    }
}

#[test]
fn the_constants_are_the_decided_rule() {
    assert_eq!(COORDINATE_ROUNDING_ALLOWANCE_M, 1.0e-3);
    assert_eq!(TRANSVERSE_RATIO_BASE, 1.0e-3);
    assert_eq!(AXIS_ANGLE_BASE_RAD, 1.0e-3);
    // β of a 1 m span: asin(√3 · 1 mm / 1 m); a span shorter than √3 mm gives none.
    assert_eq!(straight_beta(1.0), Some((3f64.sqrt() * 1.0e-3).asin()));
    assert_eq!(straight_beta(1.7e-3), None);
}

/// P2 decision example: a 30° run entered as (866, 500, 0) mm with the offset
/// (−25, 43, 0) mm has axial share 0.003015789. The base ratio alone (1e-3)
/// would refuse it; the rounding allowance accepts it.
#[test]
fn the_thirty_degree_run_with_rounded_entry_is_accepted() {
    let (n0, a, b, c) = (mm([0.0, 0.0, 0.0]), mm([866.0, 500.0, 0.0]), mm([945.0, 603.0, 0.0]), mm([1811.0, 1103.0, 0.0]));
    let e = mm([-25.0, 43.0, 0.0]);
    let (sn, sf) = (run(n0, a), run(b, c));
    let (axis, _) = decide_axis(&sn, &sf).unwrap();
    let share = (e[0] * axis[0] + e[1] * axis[1]).abs() / len(e);
    assert!((share - 0.003015789).abs() < 1e-8, "axial share {share}");
    assert!(share > TRANSVERSE_RATIO_BASE, "the base ratio alone refuses it");
    let outcome = check(a, b, &sn, &sf, e);
    assert_eq!(outcome_name(&outcome), "realized", "{outcome:?}");
}

/// P3 decision example: two parallel 100 mm spans rounded to (87, 50) and
/// (86, 50) mm are 0.005008974 rad apart. θ_base alone would refuse them; the
/// spans' direction uncertainty accepts them.
#[test]
fn two_rounded_parallel_hundred_millimetre_spans_agree() {
    let (n0, a, b, c) = (mm([0.0, 0.0, 0.0]), mm([87.0, 50.0, 0.0]), mm([166.0, 153.0, 0.0]), mm([252.0, 203.0, 0.0]));
    let (sn, sf) = (run(n0, a), run(b, c));
    let (EndStatus::Direction { direction: dn, .. }, EndStatus::Direction { direction: df, .. }) = (&sn, &sf) else { unreachable!() };
    let cross = [dn[1] * df[2] - dn[2] * df[1], dn[2] * df[0] - dn[0] * df[2], dn[0] * df[1] - dn[1] * df[0]];
    let theta = len(cross).atan2(dn[0] * df[0] + dn[1] * df[1] + dn[2] * df[2]);
    assert!((theta - 0.005008974).abs() < 1e-8, "theta {theta}");
    assert!(theta > AXIS_ANGLE_BASE_RAD, "θ_base alone refuses it");
    let outcome = check(a, b, &sn, &sf, mm([-25.0, 43.0, 0.0]));
    assert_eq!(outcome_name(&outcome), "realized", "{outcome:?}");
}

/// The §7.1 geometry: 1 m runs on X, the body chord (0.12, −0.05, 0) m.
fn axis_x() -> (EndStatus, EndStatus, [f64; 3], [f64; 3]) {
    let (n0, a, b, c) = ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.12, -0.05, 0.0], [2.12, -0.05, 0.0]);
    (run(n0, a), run(b, c), a, b)
}

#[test]
fn transversality_bound_from_both_sides() {
    let (sn, sf, a, b) = axis_x();
    // a_e = 1e-3·‖e‖ + (√3/2)·1 mm + ‖e‖·sin β with β of a 1 m span: about 1.00266 mm.
    assert_eq!(outcome_name(&check(a, b, &sn, &sf, mm([1.00, -50.0, 0.0]))), "realized");
    assert_eq!(outcome_name(&check(a, b, &sn, &sf, mm([1.01, -50.0, 0.0]))), "not_transverse");
}

#[test]
fn the_collinear_offset_is_refused_as_not_transverse() {
    let (n0, a, b, c) = ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.12, 0.0, 0.0], [2.12, 0.0, 0.0]);
    // On concentric geometry its placement residual is exactly 5 mm, which alone would pass.
    let outcome = check(a, b, &run(n0, a), &run(b, c), mm([5.0, 0.0, 0.0]));
    assert_eq!(outcome_name(&outcome), "not_transverse", "{outcome:?}");
}

#[test]
fn precision_bound_from_both_sides() {
    let (n0, a) = ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let (b8, c8) = ([1.12, -0.0008, 0.0], [2.12, -0.0008, 0.0]);
    let below = check(a, b8, &run(n0, a), &run(b8, c8), mm([0.0, -0.8, 0.0]));
    assert_eq!(outcome_name(&below), "below_precision", "{below:?}");
    let (b9, c9) = ([1.12, -0.0009, 0.0], [2.12, -0.0009, 0.0]);
    let resolved = check(a, b9, &run(n0, a), &run(b9, c9), mm([0.0, -0.9, 0.0]));
    assert_eq!(outcome_name(&resolved), "realized", "{resolved:?}");
}

/// The far run rotated by θ in the XY plane.
fn rotated(theta: f64) -> (EndStatus, EndStatus, [f64; 3], [f64; 3]) {
    let (n0, a, b) = ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.12, -0.05, 0.0]);
    let c = [b[0] + theta.cos(), b[1] + theta.sin(), 0.0];
    (run(n0, a), run(b, c), a, b)
}

#[test]
fn agreement_bound_from_both_sides() {
    // θ_base + 2·asin(√3 mm / 1 m) = 4.4641e-3 rad.
    let (sn, sf, a, b) = rotated(-4.46e-3);
    assert_eq!(outcome_name(&check(a, b, &sn, &sf, mm([0.0, -50.0, 0.0]))), "realized");
    let (sn, sf, a, b) = rotated(-4.47e-3);
    assert_eq!(outcome_name(&check(a, b, &sn, &sf, mm([0.0, -50.0, 0.0]))), "invalid:run_directions_disagree");
}

fn bits(o: &OffsetOutcome) -> Vec<u64> {
    let m = match o {
        OffsetOutcome::Realized(m) | OffsetOutcome::BelowPrecision(m) | OffsetOutcome::NotTransverse(m) | OffsetOutcome::Mismatch(m) => m,
        OffsetOutcome::Invalid(..) => return vec![],
    };
    [m.axial_length, m.offset_axial, m.allowance, m.residual, m.beta_run].iter().map(|v| v.to_bits()).collect()
}

/// §3.3, Codex: a sign-only convention change (the ends' names swapped and the
/// offset negated, as option A against option B with the member reversed) gives
/// the same outcome and bit-identical measures, because the axis is chosen
/// independently of endpoint naming and oriented by D.
#[test]
fn a_sign_only_convention_change_does_not_change_acceptance() {
    for (theta, e_mm) in [(-0.0005, [0.04, -50.0, 0.0]), (-4.0e-3, [0.95, -50.0, 0.0])] {
        let (sn, sf, a, b) = rotated(theta);
        let e = mm(e_mm);
        let named_a = check(a, b, &sn, &sf, e);
        let named_b = check(b, a, &sf, &sn, [-e[0], -e[1], -e[2]]);
        assert_eq!(outcome_name(&named_a), outcome_name(&named_b), "theta {theta}");
        assert_eq!(bits(&named_a), bits(&named_b), "theta {theta}: bit-identical measures");
    }
    // The near-bound case: the component end's own direction would accept it
    // (e·t_N = 0.95 mm), the far end's would refuse it (1.15 mm); the consensus
    // refuses it under either naming.
    let (sn, sf, a, b) = rotated(-4.0e-3);
    assert_eq!(outcome_name(&check(a, b, &sn, &sf, mm([0.95, -50.0, 0.0]))), "not_transverse");
}

#[test]
fn ambiguity_and_unsupported_evidence_are_refused_before_directions() {
    let ok = EndStatus::Direction { direction: [1.0, 0.0, 0.0], beta: 1e-3 };
    let tee = EndStatus::Ambiguous(vec!["pipe:x".into(), "pipe:y".into()]);
    let reducer = EndStatus::Unsupported { member: "body:r1".into(), reason: "the body of another reducer" };
    let short = EndStatus::Unresolvable { member: "pipe:short".into() };
    assert_eq!(decide_axis(&tee, &ok).unwrap_err().0, "ambiguous_run");
    assert_eq!(decide_axis(&ok, &reducer).unwrap_err().0, "unsupported_run_evidence");
    assert_eq!(decide_axis(&reducer, &tee).unwrap_err().0, "ambiguous_run");
    assert_eq!(decide_axis(&short, &ok).unwrap_err().0, "unresolvable_run_evidence");
    assert_eq!(decide_axis(&EndStatus::None, &EndStatus::None).unwrap_err().0, "standalone_body");
    assert_eq!(decide_axis(&EndStatus::None, &ok).unwrap().0, [1.0, 0.0, 0.0]);
}

/// A realized arc's tangent at its ends and β_arc (§3.3): a quarter circle of
/// radius 1 m in the XY plane bowing toward +Y.
#[test]
fn a_realized_arc_gives_its_end_tangents() {
    let r = 1.0f64;
    let (from, to) = ([0.0, 0.0, 0.0], [r * 2f64.sqrt(), 0.0, 0.0]);
    // chord √2 m, half-angle 45°, plane normal +Y: tangent (cos 45°, ±sin 45°, 0).
    let Evidence::Run { direction: t_from, beta } = arc_evidence(from, to, true, r, [0.0, 1.0, 0.0]) else { panic!() };
    let Evidence::Run { direction: t_to, .. } = arc_evidence(from, to, false, r, [0.0, 1.0, 0.0]) else { panic!() };
    let h = 0.5f64.sqrt();
    assert!((t_from[0] - h).abs() < 1e-15 && (t_from[1] - h).abs() < 1e-15, "{t_from:?}");
    assert!((t_to[0] - h).abs() < 1e-15 && (t_to[1] + h).abs() < 1e-15, "{t_to:?}");
    let q3 = 3f64.sqrt() * 1e-3;
    let expected = (q3 / 2f64.sqrt()).asin() + q3 / (4.0 * r * r - 2.0).sqrt();
    assert!((beta - expected).abs() < 1e-15, "beta {beta} expected {expected}");
    // A radius that cannot span the chord gives no direction.
    assert_eq!(arc_evidence(from, to, true, 0.7, [0.0, 1.0, 0.0]), Evidence::Unresolvable);
}

#[test]
fn placement_and_axial_length() {
    let (sn, sf, a, b) = axis_x();
    // 47 mm declared against 50 mm geometry: residual 3 mm, accepted.
    let OffsetOutcome::Realized(m) = check(a, b, &sn, &sf, mm([0.0, -47.0, 0.0])) else { panic!() };
    assert!((m.residual - 3.0e-3).abs() < 1e-12, "{}", m.residual);
    assert!((m.transverse[1] + 0.05).abs() < 1e-15, "the geometry's 50 mm is reported");
    assert_eq!(outcome_name(&check(a, b, &sn, &sf, mm([0.0, 50.0, 0.0]))), "mismatch");
    assert_eq!(outcome_name(&check(a, b, &sn, &sf, mm([0.0, -40.0, 0.0]))), "mismatch");
    // A zero offset on this slanted chord is a 50 mm mismatch.
    assert_eq!(outcome_name(&check(a, b, &sn, &sf, [0.0; 3])), "mismatch");
    // D·t = 0: the body has no length along the run.
    let side = [1.0, -0.05, 0.0];
    assert_eq!(outcome_name(&check(a, side, &sn, &EndStatus::None, mm([0.0, -50.0, 0.0]))), "invalid:no_axial_length");
}
