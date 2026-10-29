//! W-16t TPS-G (DEL-05-03 Design `section-bases-and-timoshenko.md` §4 item 4
//! (i); W-16t design §5.2): the S11-G bound of a Timoshenko partial-span
//! uniform load, judged by an independent oracle in this file.
//!
//! The oracle interprets every held binary64 operand exactly, phi_y and phi_z
//! included, and constructs the denominator-cleared defect
//! S_r = s_r D (v_r - I_r), D = (1+phi_y)(1+phi_z), through the correction's
//! g(a) - g(b) form (g(x) = (x - x^2)^2), with its own expansion arithmetic
//! and its own fixed-end polynomials; rho = phi/(1+phi) is never rounded. It
//! checks s_r D B_r >= |S_r| by exact comparison and the tightness
//! s_r D B_r <= (1 + 2^-49)|S_r| + s_r D 2^-1070 by cross-multiplication.
//! Invented inputs.
use super::*;

/// Exact components of an accumulator (round and subtract); panics outside
/// the binary64 range, which the oracle's operands never reach.
fn parts(mut acc: ExactAccumulator) -> Vec<f64> {
    let mut out = Vec::new();
    while !acc.is_zero() {
        let head = acc.round().unwrap();
        assert!(head != 0.0, "oracle remainder below the binary64 range");
        acc.add(-head).unwrap();
        out.push(head);
        assert!(out.len() < 200);
    }
    out
}
fn sum(terms: &[&[f64]], signs: &[f64]) -> Vec<f64> {
    let mut acc = ExactAccumulator::new();
    for (t, s) in terms.iter().zip(signs) {
        for &c in *t {
            acc.add(s * c).unwrap();
        }
    }
    parts(acc)
}
fn mul(x: &[f64], y: &[f64]) -> Vec<f64> {
    let mut acc = ExactAccumulator::new();
    for &a in x {
        for &b in y {
            acc.add_product(a, b).unwrap();
        }
    }
    parts(acc)
}
fn sign_of(x: &[f64]) -> i8 {
    let mut acc = ExactAccumulator::new();
    for &c in x {
        acc.add(c).unwrap();
    }
    acc.signum()
}
fn neg(x: &[f64]) -> Vec<f64> {
    x.iter().map(|c| -c).collect()
}

/// The oracle's local fixed-end slots of one load, times D, rotation slots
/// times 3 (the S11-G slot scale): [UX, UY, UZ, RX, RY, RZ] at i then j.
fn oracle_cleared_local(
    member: &StraightPipeElement,
    load: &SpannedGlobalUniformLoad,
    phi: [f64; 2],
) -> Vec<Vec<f64>> {
    let orientation = member.frame_element().unwrap().orientation().unwrap();
    let l = [member.length().unwrap()];
    let (a, b) = (load.span.start_fraction, load.span.end_fraction);
    // [P(x)]_a^b with P(x) = sum_k c_k x^k.
    let poly = |coefficients: &[(f64, u32)]| -> Vec<f64> {
        let eval = |x: f64| {
            let mut acc = ExactAccumulator::new();
            for &(c, k) in coefficients {
                let mut power = vec![1.0];
                for _ in 0..k {
                    power = mul(&power, &[x]);
                }
                for p in power {
                    acc.add_product(c, p).unwrap();
                }
            }
            parts(acc)
        };
        sum(&[&eval(b), &eval(a)], &[1.0, -1.0])
    };
    let axial_i = poly(&[(1.0, 1), (-0.5, 2)]);
    let axial_j = poly(&[(0.5, 2)]);
    let v_i = poly(&[(1.0, 1), (-1.0, 3), (0.5, 4)]);
    let v_j = poly(&[(1.0, 3), (-0.5, 4)]);
    let m3_i = poly(&[(1.5, 2), (-2.0, 3), (0.75, 4)]);
    let m3_j = poly(&[(-1.0, 3), (0.75, 4)]);
    // (g(a) - g(b)) / 2, g(x) = (x - x^2)^2: the shear-flexibility functional.
    let g = |x: f64| {
        let d = sum(&[&[x], &mul(&[x], &[x])], &[1.0, -1.0]);
        mul(&d, &d)
    };
    let j_half = mul(&sum(&[&g(a), &g(b)], &[1.0, -1.0]), &[0.5]);
    let one = |p: f64| sum(&[&[1.0], &[p]], &[1.0, 1.0]);
    let (d_y, d_z) = (one(phi[0]), one(phi[1]));
    let d = mul(&d_y, &d_z);
    let mut out = vec![Vec::new(); 12];
    let mut add = |slot: usize, x: Vec<f64>| {
        let prior = std::mem::take(&mut out[slot]);
        out[slot] = sum(&[&prior, &x], &[1.0, 1.0]);
    };
    for dir in 0..3 {
        let mut acc = ExactAccumulator::new();
        for axis in 0..3 {
            acc.add_product(orientation.local_axes[dir][axis], load.force_per_length[axis]).unwrap();
        }
        let q = parts(acc);
        let ql = mul(&q, &l);
        let ql2 = mul(&ql, &l);
        let cleared = |x: &[f64]| mul(x, &d);
        if dir == 0 {
            add(UX, cleared(&mul(&ql, &axial_i)));
            add(DOF_PER_NODE + UX, cleared(&mul(&ql, &axial_j)));
            continue;
        }
        // D rho = phi_p D_other: local y uses phi_z, local z phi_y.
        let d_rho = if dir == 1 { mul(&[phi[1]], &d_y) } else { mul(&[phi[0]], &d_z) };
        let c = mul(&mul(&ql, &j_half), &d_rho);
        let c3m = mul(&mul(&c, &l), &[1.5]);
        let (v, r, s) = if dir == 1 { (UY, RZ, 1.0) } else { (UZ, RY, -1.0) };
        add(v, sum(&[&cleared(&mul(&ql, &v_i)), &c], &[1.0, 1.0]));
        add(DOF_PER_NODE + v, sum(&[&cleared(&mul(&ql, &v_j)), &c], &[1.0, -1.0]));
        add(r, mul(&sum(&[&cleared(&mul(&ql2, &m3_i)), &c3m], &[1.0, 1.0]), &[s]));
        add(DOF_PER_NODE + r, mul(&sum(&[&cleared(&mul(&ql2, &m3_j)), &c3m], &[1.0, 1.0]), &[s]));
    }
    out
}

/// Checks every slot's bound of `loads` against the oracle: soundness and
/// tightness. Returns the bounds.
fn judge(member: &StraightPipeElement, loads: &[SpannedGlobalUniformLoad]) -> Vec<f64> {
    let (values, formations) = member.equivalent_global_nodal_loads_with_spans_formed(loads).unwrap();
    let phi = member.timoshenko_phi().unwrap().unwrap();
    let orientation = member.frame_element().unwrap().orientation().unwrap();
    let transform = orientation.transformation_matrix();
    let mut local = vec![Vec::new(); 12];
    for load in loads {
        for (slot, x) in oracle_cleared_local(member, load, phi).into_iter().enumerate() {
            local[slot] = sum(&[&local[slot], &x], &[1.0, 1.0]);
        }
    }
    let one = |p: f64| sum(&[&[1.0], &[p]], &[1.0, 1.0]);
    let d = mul(&one(phi[0]), &one(phi[1]));
    let mut bounds = Vec::new();
    for row in 0..12 {
        let Formation::Bounded { bound } = formations[row] else {
            panic!("slot {row}: {:?}", formations[row]);
        };
        assert!(bound.is_finite() && bound >= 0.0, "slot {row}: {bound}");
        let s = if row % DOF_PER_NODE >= RX { 3.0 } else { 1.0 };
        let sd = mul(&d, &[s]);
        let mut acc = ExactAccumulator::new();
        for c in mul(&sd, &[values[row]]) {
            acc.add(c).unwrap();
        }
        for (col, comps) in local.iter().enumerate() {
            for &c in comps {
                acc.add_product(-transform[col][row], c).unwrap();
            }
        }
        let defect = parts(acc);
        let magnitude = if sign_of(&defect) < 0 { neg(&defect) } else { defect };
        let held = mul(&sd, &[bound]);
        // Soundness: s D B >= |S|, exactly.
        assert!(sign_of(&sum(&[&held, &magnitude], &[1.0, -1.0])) >= 0, "slot {row}: bound {bound} below the defect");
        // Tightness: s D B <= (1 + 2^-49)|S| + s D 2^-1070.
        let slack = sum(
            &[&mul(&magnitude, &[1.0 + 2f64.powi(-49)]), &mul(&sd, &[2f64.powi(-1070)])],
            &[1.0, 1.0],
        );
        assert!(sign_of(&sum(&[&held, &slack], &[1.0, -1.0])) <= 0, "slot {row}: bound {bound} is loose");
        bounds.push(bound);
    }
    bounds
}

fn section(e: f64, g: f64, iy: f64, iz: f64, shear_area: f64) -> StraightPipeSectionProperties {
    StraightPipeSectionProperties::timoshenko(e, g, 1.0, iy, iz, iy + iz, shear_area, None).unwrap()
}
fn element(xj: [f64; 3], section: StraightPipeSectionProperties, y: [f64; 3]) -> StraightPipeElement {
    StraightPipeElement::new("T", FrameNode::new(0, [0.0; 3]).unwrap(), FrameNode::new(1, xj).unwrap(), section, y).unwrap()
}
fn spanned(q: [f64; 3], a: f64, b: f64) -> SpannedGlobalUniformLoad {
    SpannedGlobalUniformLoad::new(q, UniformLoadSpan::new(a, b).unwrap()).unwrap()
}

/// E1 (phi = 1, L = 1), E3 (interior, skew, unequal phi), phi = 2 exactly, a
/// phi for which 1 + phi rounds (2^-60), a large phi, and a slice that mixes a
/// full span with proper spans: every slot is `Bounded`, sound and tight.
#[test]
fn timoshenko_partial_span_formation_is_bounded() {
    // E1: EI = 1, Ks = 12, L = 1: phi = 1; the distal half along y and z.
    let e1 = element([1.0, 0.0, 0.0], section(1.0, 1.0, 1.0, 1.0, 12.0), [0.0, 1.0, 0.0]);
    assert_eq!(e1.timoshenko_phi().unwrap(), Some([1.0, 1.0]));
    judge(&e1, &[spanned([0.0, 1.0, 0.0], 0.5, 1.0)]);
    judge(&e1, &[spanned([0.0, 0.0, -1.0], 0.5, 1.0)]);
    judge(&e1, &[spanned([0.0, 1.0, 0.0], 0.0, 0.5)]);
    // E3/E9: the rational skew frame x = (2,3,6)/7, y = (3,-6,2)/7, L = 7/2,
    // I_y != I_z (unequal phi), a 3D intensity, the interior span.
    let skew = element([1.0, 1.5, 3.0], section(200e9, 200e9 / 2.6, 2.7e-5, 4.1e-5, 3.0e-3), [3.0, -6.0, 2.0]);
    let [py, pz] = skew.timoshenko_phi().unwrap().unwrap();
    assert!(py != pz);
    judge(&skew, &[spanned([0.3, -250.0, 1200.0], 0.2, 0.7)]);
    judge(&skew, &[spanned([252.0, 0.0, 252.0], 0.0, 0.1), spanned([0.0, 0.0, 252.0], 0.0, 1.0), spanned([-17.0, 3.0, 252.0], 0.6, 1.0)]);
    // phi = 2 exactly (2 - phi = 0 in K): EI = 1, Ks = 6, L = 1.
    let two = element([1.0, 0.0, 0.0], section(1.0, 1.0, 1.0, 1.0, 6.0), [0.0, 1.0, 0.0]);
    assert_eq!(two.timoshenko_phi().unwrap(), Some([2.0, 2.0]));
    judge(&two, &[spanned([0.0, 3.0, -5.0], 0.2, 0.7)]);
    // phi = 2^-60: 1 + phi is not a binary64 value (D has two components).
    let tiny = element([1.0, 0.0, 0.0], section(1.0, 1.0, 1.0, 1.0, 12.0 * 2f64.powi(60)), [0.0, 1.0, 0.0]);
    let [p, _] = tiny.timoshenko_phi().unwrap().unwrap();
    assert!(p > 0.0 && 1.0 + p == 1.0, "precondition: 1 + phi rounds ({p:e})");
    judge(&tiny, &[spanned([0.0, 1.0, 1.0], 0.2, 0.7)]);
    // A large phi (short stubby member).
    let stub = element([0.05, 0.0, 0.0], section(200e9, 200e9 / 2.6, 2.7e-5, 2.7e-5, 3.0e-3), [0.0, 1.0, 0.0]);
    assert!(stub.timoshenko_phi().unwrap().unwrap()[0] > 10.0);
    judge(&stub, &[spanned([0.0, 0.0, 252.0], 0.2, 0.7)]);
}

/// A near-full proper span selects TPS-G (`Bounded`), not the full-span
/// formula; exactly [0, 1] keeps `Formation::Exact` bit for bit, equal to
/// the Euler–Bernoulli twin's record.
#[test]
fn timoshenko_near_full_span_selects_tps_g_and_the_full_span_is_unchanged() {
    let t = element([2.5, 0.0, 0.0], section(200e9, 200e9 / 2.6, 2.7e-5, 2.7e-5, 3.0e-3), [0.0, 1.0, 0.0]);
    let near = 1.0 - 2f64.powi(-40);
    let bounds = judge(&t, &[spanned([0.0, 0.0, 252.0], 0.0, near)]);
    assert!(bounds.iter().all(|b| b.is_finite()));
    let eb = StraightPipeElement::new(
        "EB",
        FrameNode::new(0, [0.0; 3]).unwrap(),
        FrameNode::new(1, [2.5, 0.0, 0.0]).unwrap(),
        StraightPipeSectionProperties::new(200e9, 200e9 / 2.6, 1.0, 2.7e-5, 2.7e-5, 5.4e-5, None).unwrap(),
        [0.0, 1.0, 0.0],
    )
    .unwrap();
    let full = [SpannedGlobalUniformLoad::full([0.0, 0.0, 252.0]).unwrap()];
    let (tv, tf) = t.equivalent_global_nodal_loads_with_spans_formed(&full).unwrap();
    let (ev, ef) = eb.equivalent_global_nodal_loads_with_spans_formed(&full).unwrap();
    assert!(tf.iter().all(|f| matches!(f, Formation::Exact { .. })));
    assert_eq!(tf, ef);
    assert_eq!(tv.map(f64::to_bits), ev.map(f64::to_bits));
}

/// Range failures make every slot `CannotBound` (fail closed): a remainder
/// below the binary64 range inside TPS-G, an overflowing value, and a
/// non-positive phi.
#[test]
fn timoshenko_partial_span_range_failure_cannot_bound() {
    let t = element([2.5, 0.0, 0.0], section(200e9, 200e9 / 2.6, 2.7e-5, 2.7e-5, 3.0e-3), [0.0, 1.0, 0.0]);
    let (values, formations) = t
        .equivalent_global_nodal_loads_with_spans_formed(&[spanned([0.0, 1e-300, 0.0], 0.2, 0.7)])
        .unwrap();
    assert!(values.iter().all(|v| v.is_finite()));
    assert!(formations.iter().all(|f| *f == Formation::CannotBound), "{formations:?}");
    let orientation = t.frame_element().unwrap().orientation().unwrap();
    let load = [spanned([0.0, 1.0, 0.0], 0.2, 0.7)];
    let mut huge = [0.0; ELEMENT_DOF];
    huge[UY] = f64::MAX;
    assert!(timoshenko_partial_bounds(&orientation, 2.5, [0.1, 0.1], &load, &huge).is_none());
    let finite = [0.0; ELEMENT_DOF];
    assert!(timoshenko_partial_bounds(&orientation, 2.5, [0.1, 0.1], &load, &finite).is_some());
    for phi in [[0.0, 0.1], [0.1, -1.0], [f64::NAN, 0.1], [0.1, f64::INFINITY]] {
        assert!(timoshenko_partial_bounds(&orientation, 2.5, phi, &load, &finite).is_none());
    }
}

/// Both directed-quotient outcomes and the downward rounding of D.
#[test]
fn timoshenko_partial_span_directed_quotient_and_denominator() {
    // Exact: 6 / 3 = 2, not raised.
    assert_eq!(quotient_upward(6.0, 3.0), Some(2.0));
    // Inexact: 1 / 3 is raised when fl(1/3) * 3 < 1, and is then >= 1/3.
    for (x, y) in [(1.0, 3.0), (2.0, 3.0), (1.0, 10.0), (7.0, 1.0 + 2f64.powi(-52))] {
        let q = quotient_upward(x, y).unwrap();
        let mut over = ExactAccumulator::new();
        over.add_product(q, y).unwrap();
        over.add(-x).unwrap();
        assert!(over.signum() >= 0, "{x}/{y}: {q} is below the quotient");
        let mut under = ExactAccumulator::new();
        under.add_product(q.next_down(), y).unwrap();
        under.add(-x).unwrap();
        assert!(under.signum() < 0, "{x}/{y}: {q} is not the least upper binary64");
    }
    let raised = quotient_upward(1.0, 3.0).unwrap();
    assert_eq!(raised, (1.0f64 / 3.0).next_up(), "fl(1/3) * 3 < 1, so the quotient is raised");
    assert_eq!(quotient_upward(0.0, 3.0), Some(0.0));
    assert_eq!(quotient_upward(f64::MAX, 0.5), None);
    assert_eq!(quotient_upward(1.0, 0.0), None);
    // D = 1 + 2^-60 rounds down to 1; D = 3 is exact.
    assert_eq!(round_downward_positive(&[1.0, 2f64.powi(-60)]), Some(1.0));
    assert_eq!(round_downward_positive(&[1.0, -(2f64.powi(-60))]), Some(1.0f64.next_down()));
    assert_eq!(round_downward_positive(&[3.0]), Some(3.0));
    assert_eq!(round_downward_positive(&[]), None);
}
