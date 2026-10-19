//! O4 (DEL-04-08 Design `o4-source-extension.md` §4.2–§4.4, §6): the retained
//! Timoshenko straight member. Expected values are closed forms of the 1D
//! model (v_tip = FL³/(3EI) + FL/Ks, θ_tip = FL²/(2EI)) or the independent
//! outward reference `timoshenko_member.txt` (`gen_timoshenko_member_vectors.py`),
//! never the production element. Invented inputs only.
use super::super::adaptive::{CaseLimit, ExecutionOutcome, StageGuard};
use super::super::origins::{OriginCapacity, RecordedInvocation};
use super::super::recover::QuantityId;
use super::super::source::*;
use super::super::wide::multi::WideContext;
use super::super::wide::Wide;
use super::super::wide_sum::ExactWideSum;
use super::{form_members_with, MemberOperators};

fn member(id: u32, i: u32, j: u32, iy: f64, iz: f64) -> StraightMember {
    StraightMember {
        id,
        node_i: i,
        node_j: j,
        elastic_modulus: 1.0,
        shear_modulus: 1.0,
        area: 1.0,
        second_moment_y: iy,
        second_moment_z: iz,
        torsion_constant: 1.0,
        y_reference: [0.0, 1.0, 0.0],
    }
}

/// A cantilever along x of `n` equal members, clamped at node 0, tip load
/// `force` on `dof` (local component index at the tip node).
fn cantilever(length: f64, n: u32, iy: f64, iz: f64, dof: usize, force: f64) -> PrimitiveSource {
    let nodes = (0..=n).map(|k| [length * f64::from(k) / f64::from(n), 0.0, 0.0]).collect();
    let members = (0..n).map(|k| member(k, k, k + 1, iy, iz)).collect();
    PrimitiveSource::new(SourceParts {
        nodes,
        members,
        constraints: (0..6).map(|g| Constraint { dof: Dof::from_global(g), value: 0.0 }).collect(),
        loads: vec![NodalLoad {
            dof: Dof::from_global(n as usize * 6 + dof),
            value: force,
            source_id: "tip".into(),
        }],
        ..Default::default()
    })
    .unwrap()
}

fn leaves(source: &PrimitiveSource, shear_area: f64) -> TimoshenkoLeafSet {
    let leaves = source.members().iter().map(|m| TimoshenkoLeaf { member: m.id, shear_area }).collect();
    TimoshenkoLeafSet::new(source, leaves).unwrap()
}

fn solve(source: &PrimitiveSource, shear: Option<&TimoshenkoLeafSet>) -> super::super::adaptive::RetainedSolve {
    let mut invocation =
        RecordedInvocation::new(u64::MAX, OriginCapacity::for_calls(&[1], &[]).unwrap()).unwrap();
    let refs = [MemberSourceRef::nodal(source)];
    let mut cases = match shear {
        Some(set) => invocation.solve_timoshenko_cases(&refs, set, CaseLimit::new(u64::MAX)).unwrap(),
        None => invocation.solve_member_cases(&refs, CaseLimit::new(u64::MAX)).unwrap(),
    };
    match cases.remove(0).outcome {
        ExecutionOutcome::Selected(owner) => *owner,
        other => panic!("{other:?}"),
    }
}

fn displacement(owner: &super::super::adaptive::RetainedSolve, global: usize) -> f64 {
    owner
        .publish()
        .rows
        .iter()
        .find(|r| r.id == QuantityId::Displacement(Dof::from_global(global)))
        .unwrap()
        .value
        .value()
        .unwrap()
}

/// |actual − expected| ≤ tol·|expected|.
fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!((actual - expected).abs() <= tol * expected.abs(), "{what}: {actual:e} vs {expected:e}");
}

#[test]
fn retained_timoshenko_cantilever_matches_the_closed_form() {
    // E = G = 1, L = 2, I_z = 1/2, I_y = 3/4, A_s = 1/4: Ks = 1/4.
    let (length, iy, iz, a_s, f) = (2.0, 0.75, 0.5, 0.25, 3.0);
    for n in [1u32, 3] {
        let source = cantilever(length, n, iy, iz, 1, f);
        let shear = leaves(&source, a_s);
        let owner = solve(&source, Some(&shear));
        assert!(owner.timoshenko().is_some());
        let tip = n as usize * 6;
        // v_tip (y direction bends about z): FL³/(3EI_z) + FL/Ks.
        let v = f * length.powi(3) / (3.0 * iz) + f * length / a_s;
        close(displacement(&owner, tip + 1), v, 1e-14, "v_tip");
        close(displacement(&owner, tip + 5), f * length * length / (2.0 * iz), 1e-14, "theta_z tip");
        // The Euler–Bernoulli twin has no shear term.
        let eb = solve(&source, None);
        assert!(eb.timoshenko().is_none());
        close(displacement(&eb, tip + 1), f * length.powi(3) / (3.0 * iz), 1e-14, "v_tip EB");
        // The other plane bends about y with I_y.
        let source = cantilever(length, n, iy, iz, 2, f);
        let owner = solve(&source, Some(&leaves(&source, a_s)));
        let w = f * length.powi(3) / (3.0 * iy) + f * length / a_s;
        close(displacement(&owner, tip + 2), w, 1e-14, "w_tip");
    }
}

/// §4.2: the formed d₁, d₂ and d̄ at p follow the fixed rounding schedule;
/// at φ = 2 exactly d₂ is an exact zero.
#[test]
fn retained_timoshenko_entries_follow_the_fixed_schedule() {
    // L = 1, E = G = 1, A_s = 3, I_z = 1/2: φ_z = 12·(1/2)/3 = 2; I_y = 3: φ_y = 12.
    let source = cantilever(1.0, 1, 3.0, 0.5, 1, 1.0);
    let shear = leaves(&source, 3.0);
    let mut ctx = WideContext::<4>::new(128).unwrap();
    let mut sum = ExactWideSum::new();
    let members: Vec<MemberOperators<4>> =
        form_members_with(&mut ctx, &mut sum, &StageGuard::unlimited(), &source, Some(&shear)).unwrap();
    let t = members[0].timoshenko.unwrap();
    let w = |x: f64| Wide::<4>::from_f64(x).unwrap();
    // c_z = 1/2, t₃ = 3/2, sl = 3, φ = 2, op = 3, c_s = 1/2: d₁ = 1, d₂ = 0, d̄ = 2.
    assert_eq!(t.d1[0], w(1.0));
    assert!(t.d2[0].is_zero());
    assert_eq!(t.dbar[0], w(2.0));
    // c_y = 3, t₃ = 9, φ = 12, op = 13, c_s = RN(9/13) at 128 bits.
    let cs = ctx.div(&w(9.0), &w(13.0)).unwrap();
    assert_eq!(t.d1[1], ctx.add(&w(3.0), &cs).unwrap());
    assert_eq!(t.d2[1], ctx.sub(&cs, &w(3.0)).unwrap());
    // d̄ = RU(c + c_s)·2 exactly (λ = 2, rounded upward).
    let mut s2 = ExactWideSum::new();
    let up = super::super::directed::add_toward(&mut ctx, &mut s2, &w(3.0), &cs, super::super::directed::Toward::Up).unwrap();
    assert_eq!(t.dbar[1], up.mul_pow2(1).unwrap());
    assert!(t.dbar[1].cmp_value(&ctx.add(&w(3.0), &cs).unwrap().mul_pow2(1).unwrap()).is_gt(), "9/13 is inexact: RU > RN");
    // The schedule at every precision on a skew member (each step one rounding).
    for p in [128u32, 256, 512, 1024] {
        let skew = PrimitiveSource::new(SourceParts {
            nodes: vec![[0.0; 3], [1.0, 2.0, 3.0]],
            members: vec![StraightMember { id: 0, node_i: 0, node_j: 1, elastic_modulus: 2.0, shear_modulus: 5.0,
                area: 3.0, second_moment_y: 2.0, second_moment_z: 35.0 / 12.0, torsion_constant: 2.0,
                y_reference: [0.0, 1.0, 0.0] }],
            ..Default::default()
        }).unwrap();
        let set = TimoshenkoLeafSet::new(&skew, vec![TimoshenkoLeaf { member: 0, shear_area: 1.0 }]).unwrap();
        let mut c = WideContext::<16>::new(p).unwrap();
        let mut sm = ExactWideSum::new();
        let ops: Vec<MemberOperators<16>> =
            form_members_with(&mut c, &mut sm, &StageGuard::unlimited(), &skew, Some(&set)).unwrap();
        let (m, t) = (&ops[0], ops[0].timoshenko.unwrap());
        let s = c.mul(&Wide::<16>::from_f64(5.0).unwrap(), &Wide::<16>::from_f64(1.0).unwrap()).unwrap();
        let sl = c.mul(&s, &m.length).unwrap();
        for (plane, cc) in [m.bend_z, m.bend_y].iter().enumerate() {
            let t3 = c.mul(&Wide::<16>::from_f64(3.0).unwrap(), cc).unwrap();
            let phi = c.div(&t3.mul_pow2(2).unwrap(), &sl).unwrap();
            let op = c.add(&Wide::<16>::ONE, &phi).unwrap();
            let cs = c.div(&t3, &op).unwrap();
            assert_eq!(t.d1[plane], c.add(cc, &cs).unwrap(), "d1 p={p}");
            assert_eq!(t.d2[plane], c.sub(&cs, cc).unwrap(), "d2 p={p}");
            let up = super::super::directed::add_toward(&mut c, &mut sm, cc, &cs, super::super::directed::Toward::Up).unwrap();
            assert_eq!(t.dbar[plane], up.mul_pow2(1).unwrap(), "dbar p={p}");
        }
    }
    // The Euler–Bernoulli member keeps no Timoshenko entries.
    let eb: Vec<MemberOperators<4>> =
        form_members_with(&mut ctx, &mut sum, &StageGuard::unlimited(), &source, None).unwrap();
    assert!(eb[0].timoshenko.is_none());
    assert!(eb[0].bounded().dbar.is_none());
}

// ------------------------------------------- §4.4 item 7: exact/directed controls

#[allow(dead_code)]
#[path = "support.rs"]
mod support;

fn dyad(value: &str) -> (bool, Vec<u64>, i64) {
    let (m, e) = value[1..].split_once('@').unwrap();
    (value.starts_with('-'), support::hex_limbs(m), e.parse().unwrap())
}
/// Σ terms ≤ 0, decided exactly over unbounded dyadics (test-only).
fn exact_nonpositive(terms: &[(bool, Vec<u64>, i64)]) -> bool {
    let anchor = terms.iter().filter(|(_, m, _)| m.iter().any(|x| *x != 0)).map(|t| t.2).min().unwrap_or(0);
    let mut signs = [Vec::<u64>::new(), Vec::<u64>::new()];
    for (negative, mag, exponent) in terms {
        if mag.iter().all(|x| *x == 0) {
            continue;
        }
        let shift = usize::try_from(exponent - anchor).unwrap();
        let (word, bit) = (shift / 64, shift % 64);
        let out = &mut signs[usize::from(*negative)];
        out.resize(out.len().max(word + mag.len() + 2), 0);
        let mut carry = 0u128;
        for (i, limb) in mag.iter().enumerate() {
            let value = (u128::from(*limb) << bit) + carry;
            let at = word + i;
            let total = u128::from(out[at]) + u128::from(value as u64);
            out[at] = total as u64;
            carry = (value >> 64) + (total >> 64);
        }
        let mut at = word + mag.len();
        while carry != 0 {
            let total = u128::from(out[at]) + carry;
            out[at] = total as u64;
            carry = total >> 64;
            at += 1;
        }
    }
    let size = signs[0].len().max(signs[1].len());
    for i in (0..size).rev() {
        let (a, b) = (signs[0].get(i).copied().unwrap_or(0), signs[1].get(i).copied().unwrap_or(0));
        if a != b {
            return a < b;
        }
    }
    true
}
fn wide_dyad(value: Wide<16>, negate: bool, factor: u64) -> (bool, Vec<u64>, i64) {
    let (negative, exponent, limbs) = value.parts();
    let mut mag = Vec::with_capacity(17);
    let mut carry = 0u128;
    for limb in limbs {
        let v = u128::from(limb) * u128::from(factor) + carry;
        mag.push(v as u64);
        carry = v >> 64;
    }
    if carry != 0 {
        mag.push(carry as u64);
    }
    (negative != negate, mag, exponent - 1023)
}
/// |actual − x| ≤ factor·bound for both endpoints x of the reference.
fn within(actual: Wide<16>, lo: &str, hi: &str, bound: Wide<16>, factor: u64) -> bool {
    [lo, hi].iter().all(|endpoint| {
        let (neg, mag, exp) = dyad(endpoint);
        [false, true].iter().all(|&direction| {
            exact_nonpositive(&[(neg != direction, mag.clone(), exp), wide_dyad(actual, !direction, 1),
                wide_dyad(bound, true, factor)])
        })
    })
}
/// |actual − midpoint| / bound, approximate (reporting only).
fn ratio(actual: Wide<16>, lo: &str, bound: Wide<16>) -> f64 {
    if bound.is_zero() {
        return 0.0;
    }
    let mut ctx = WideContext::<16>::new(1024).unwrap();
    let mut sum = ExactWideSum::new();
    let (neg, mag, exp) = dyad(lo);
    sum.add_wide(&actual, false).unwrap();
    if mag.iter().any(|x| *x != 0) {
        sum.add_integer(!neg, &mag, exp).unwrap();
    }
    let diff = sum.round(&mut ctx).unwrap().abs();
    let r = ctx.div(&diff, &bound).unwrap();
    // A ratio below the binary64 range is reported as 0.
    r.to_binary64().value().unwrap_or(if r.exponent() < 0 { 0.0 } else { f64::INFINITY })
}

struct Case {
    name: String,
    precisions: Vec<u32>,
    source: PrimitiveSource,
    shear: TimoshenkoLeafSet,
    u: Vec<f64>,
    ke: Vec<(usize, usize, String, String)>,
    rows: Vec<(String, String)>,
}

const FRACTIONS: [f64; 5] = [0.0, 0.25, 1.0 / 3.0, 1.0 - f64::EPSILON, 1.0];

fn reference_cases() -> Vec<Case> {
    let text = include_str!("timoshenko_member.txt");
    let mut out = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        let f: Vec<_> = line.split_whitespace().collect();
        assert_eq!(f[0], "case");
        let v: Vec<f64> = f[3..].iter().map(|s| support::f64_bits(s)).collect();
        let (xi, xj, yr) = ([v[0], v[1], v[2]], [v[3], v[4], v[5]], [v[6], v[7], v[8]]);
        let (e, g, a, iy, iz, j, a_s) = (v[9], v[10], v[11], v[12], v[13], v[14], v[15]);
        let u = v[16..28].to_vec();
        let member = StraightMember {
            id: 0, node_i: 0, node_j: 1, elastic_modulus: e, shear_modulus: g, area: a,
            second_moment_y: iy, second_moment_z: iz, torsion_constant: j, y_reference: yr,
        };
        let source = PrimitiveSource::new(SourceParts {
            nodes: vec![xi, xj],
            members: vec![member],
            constraints: (0..12).map(|k| Constraint { dof: Dof::from_global(k), value: u[k] }).collect(),
            stations: FRACTIONS.iter().enumerate()
                .map(|(i, &fraction)| Station { id: i as u32, member: 0, fraction }).collect(),
            supports: (0..2).map(|n| SupportGroup { id: n, node: n, restrained: [true; 6],
                springs: vec![], directional_springs: vec![] }).collect(),
            ..Default::default()
        })
        .unwrap();
        let shear = TimoshenkoLeafSet::new(&source, vec![TimoshenkoLeaf { member: 0, shear_area: a_s }]).unwrap();
        let mut ke = Vec::new();
        let mut rows = Vec::new();
        for line in lines.by_ref() {
            let f: Vec<_> = line.split_whitespace().collect();
            match f[0] {
                "ke" => ke.push((f[1].parse().unwrap(), f[2].parse().unwrap(), f[3].to_string(), f[4].to_string())),
                "row" => rows.push((f[2].to_string(), f[3].to_string())),
                "end" => break,
                other => panic!("{other}"),
            }
        }
        assert_eq!((ke.len(), rows.len()), (78, 58));
        out.push(Case {
            name: f[1].to_string(),
            precisions: f[2].split(',').map(|p| p.parse().unwrap()).collect(),
            source, shear, u, ke, rows,
        });
    }
    out
}

/// The formed K_e at P against the exact element, within `factor`·2^-P·Ā,
/// Ā = g·B̄ᵀD̄B̄ the member's bounded operator (R7 Lemma B (i)'s 67 under t₁'s
/// 2^7, with D̄ in place of |D|, §4.4). Returns (entries tested, max ratio).
fn formation_against_exact(factor: u64, exponent: Option<i64>) -> (usize, f64) {
    use super::bounded_block;
    let (mut tested, mut worst) = (0usize, 0.0f64);
    let cases = reference_cases();
    assert_eq!(cases.len(), 16);
    for case in &cases {
        for &p in &case.precisions {
            let mut ctx = WideContext::<16>::new(p).unwrap();
            let mut sum = ExactWideSum::new();
            let members: Vec<MemberOperators<16>> = form_members_with(
                &mut ctx, &mut sum, &StageGuard::unlimited(), &case.source, Some(&case.shear)).unwrap();
            let m = &members[0];
            assert!(m.timoshenko.is_some(), "{}", case.name);
            if case.name.starts_with("g_boundary") {
                assert_eq!(m.g_exp, p - 16, "{}", case.name);
            }
            if case.name.starts_with("g_above") {
                assert!(m.g_exp > p - 16, "{}", case.name);
                continue;
            }
            let abar = bounded_block(&mut ctx, &mut sum, &m.bounded()).unwrap();
            for (a, b, lo, hi) in &case.ke {
                let bound = abar[*a][*b].mul_pow2(exponent.unwrap_or(-i64::from(p))).unwrap();
                let actual = *m.ke(*a, *b);
                assert!(within(actual, lo, hi, bound, factor),
                    "formed K_e[{a}][{b}] outside {factor}·2^-P·Ā: {} P={p}", case.name);
                worst = worst.max(ratio(actual, lo, bound));
                tested += 1;
            }
        }
    }
    (tested, worst)
}

/// §6: the retained Timoshenko formation against the exact element at
/// P = 128/256/512/1024 (φ = 2 − 2^-40, 2, 2 + 2^-40, ≈1e-6, > 10; skew
/// members, φ = 2 on a skew member; g at P − 16). A Euler–Bernoulli
/// formation, a wrong factor, A for A_s, swapped I_y/I_z or a flipped d₂
/// moves an entry by orders beyond 2^-100·Ā.
#[test]
fn retained_timoshenko_formation_matches_exact_element() {
    let (tested, worst) = formation_against_exact(1, Some(-100));
    // Eight geometries at four precisions and g at P − 16 at each precision.
    assert_eq!(tested, 36 * 78, "entries");
    eprintln!("matches_exact_element: {tested} entries, worst |K-K*|/(2^-100·Ā) = {worst:.3e}");
}

/// §4.4 item 7: the actual formation error against t₁'s allowance: every
/// formed entry within 67·2^-P·Ā_T (Lemma B (i)), Ā_T = g·B̄ᵀD̄B̄.
#[test]
fn retained_timoshenko_formation_error_is_within_t1() {
    let (tested, worst) = formation_against_exact(67, None);
    assert!(tested > 0);
    assert!(worst <= 67.0, "{worst}");
    eprintln!("formation_error_is_within_t1: {tested} entries, worst ratio {worst:.3} (allowance 67, t1 2^7)");
}

/// §4.4 item 7: member recovery (end actions, stations), reactions and
/// support magnitudes at P against the exact Timoshenko recovery of the
/// same u, within 69·2^-P·E_q, E formed with D̄ (Ā_T and Q̄ = D̄·ē).
/// Returns (rows tested, worst ratio).
fn recovery_against_exact() -> (usize, f64) {
    use super::super::ledger::RetainedLedger;
    use super::super::recover::{layout, recover};
    use super::super::verify::{formation_scale, StageRounding};
    use super::{assemble, assemble_bounded, Structure};
    let (mut tested, mut worst) = (0usize, 0.0f64);
    for case in reference_cases() {
        for &p in &case.precisions {
            if case.name.starts_with("g_above") {
                continue;
            }
            let mut ctx = WideContext::<16>::new(p).unwrap();
            let mut sum = ExactWideSum::new();
            let guard = StageGuard::unlimited();
            let source = &case.source;
            let members: Vec<MemberOperators<16>> =
                form_members_with(&mut ctx, &mut sum, &guard, source, Some(&case.shear)).unwrap();
            let structure = Structure::new(source).unwrap();
            let k = assemble(&mut ctx, &mut sum, &guard, source, &structure, &members, &[]).unwrap();
            let bounded: Vec<_> = members.iter().map(|m| m.bounded()).collect();
            let abar = assemble_bounded(&mut ctx, &mut sum, &guard, source, &structure, &bounded, &[]).unwrap();
            let ledger = RetainedLedger::from_source(source).unwrap();
            let layout = layout(source);
            let u: Vec<Wide<16>> = case.u.iter().map(|x| Wide::<16>::from_f64(*x).unwrap()).collect();
            let recovered =
                recover(&mut ctx, &mut sum, &guard, source, &layout, &structure, &k, &members, &[], &ledger, &u).unwrap();
            let w: Vec<Wide<16>> = u.iter().map(|x| x.abs()).collect();
            let e = formation_scale(&mut ctx, &mut sum, &guard, source, &layout, &structure, &abar, &members, &[],
                Some(&ledger), &w, StageRounding::Nearest).unwrap();
            assert_eq!(layout.len(), 14 + 58);
            for (row, (lo, hi)) in case.rows.iter().enumerate() {
                let index = 14 + row;
                let bound = e[index].unwrap().mul_pow2(-i64::from(p)).unwrap();
                let actual = recovered.values[index];
                assert!(within(actual, lo, hi, bound, 69),
                    "recovery row {row} outside 69·2^-P·E_q: {} P={p}", case.name);
                worst = worst.max(ratio(actual, lo, bound));
                tested += 1;
            }
            // Q̄ = D̄·ē (§4.2): on the aligned φ ≈ 2 designs e₂ = e₄ = 0, so the
            // end moments' E rows pin D̄: E(M_z,i) = g·RN(d̄_z·(ē₂ + ē₃)).
            if case.name.starts_with("phi2") {
                let t = members[0].timoshenko.unwrap();
                let block = |b: usize| (0..3).fold(0.0f64, |s, c| s + case.u[3 * b + c].abs());
                let (tri, roti, trj, rotj) = (block(0), block(1), block(2), block(3));
                let lift = |x: f64| Wide::<16>::from_f64(x).unwrap();
                let e2 = lift(roti + tri + trj);
                let e3 = lift(rotj + tri + trj);
                sum.clear();
                sum.add_product(&mut ctx, &t.dbar[0], &e2, false).unwrap();
                sum.add_product(&mut ctx, &t.dbar[0], &e3, false).unwrap();
                let qbar = sum.round(&mut ctx).unwrap();
                assert_eq!(e[14 + 5].unwrap(), qbar.mul_pow2(i64::from(members[0].g_exp)).unwrap(), "{}", case.name);
            }
        }
    }
    (tested, worst)
}

#[allow(non_snake_case)]
#[test]
fn retained_timoshenko_recovery_error_is_bounded_by_69uE() {
    let (tested, worst) = recovery_against_exact();
    assert_eq!(tested, 36 * 58);
    assert!(worst <= 69.0, "{worst}");
    eprintln!("recovery_error_is_bounded_by_69uE: {tested} rows, worst |q-R*|/(2^-P·E_q) = {worst:.3}");
}

#[allow(dead_code)]
#[path = "models.rs"]
mod models;

fn timoshenko_twin(source: &PrimitiveSource, shear_ratio: f64) -> TimoshenkoLeafSet {
    let leaves = source.members().iter()
        .map(|m| TimoshenkoLeaf { member: m.id, shear_area: m.area * shear_ratio }).collect();
    TimoshenkoLeafSet::new(source, leaves).unwrap()
}

/// §4.4 item 7 and §4.2's note on D̄: the D̄-inflated path through E, the
/// p = 512 Φ floor, body scales, classes and absolute bounds, exercised on a
/// whole retained solve. SKEW-K1E-60 selects at 512. Its Timoshenko twin with
/// a very stiff shear leaf (φ ≈ 0) solves the Euler–Bernoulli state to many
/// digits, while D̄ = RU(2(c + c_s)) ≈ 8c per entry replaces (4c, 2c): its
/// E_mo, so its Φ_mo, is larger (never smaller), and its published classes
/// and absolute bounds are exactly the floored classification of its own
/// values, extents and Φ.
#[test]
fn retained_timoshenko_d_bar_path_through_e_and_the_512_floor() {
    use super::super::adaptive::{classify_rows_floored, RowClass};
    use super::super::verify::{e_hat, phi_512};
    let source = models::model("SKEW-K1E-60").source();
    let eb = solve(&source, None);
    let stiff = solve(&source, Some(&timoshenko_twin(&source, 1e12)));
    let soft = solve(&source, Some(&timoshenko_twin(&source, 0.5)));
    assert_eq!(eb.selected_precision(), 512);
    for owner in [&stiff, &soft] {
        let p = owner.selected_precision();
        let evidence = owner.evidence();
        // Φ = fl↑(2^-438·ê) from E's own published bits, per body (512 only).
        assert_eq!(evidence.floor.is_some(), p == 512);
        let mut floors = Vec::new();
        for (b, fo, mo) in &evidence.resolution_scale {
            let eh = e_hat([f64::from_bits(*fo), f64::from_bits(*mo)], owner.prep.extents[*b as usize]);
            floors.push([phi_512(eh[0]), phi_512(eh[1])]);
            if let Some(floor) = &evidence.floor {
                let (_, ffo, fmo) = floor.iter().find(|f| f.0 == *b).unwrap();
                assert_eq!((*ffo, *fmo), (phi_512(eh[0]).to_bits(), phi_512(eh[1]).to_bits()));
            }
        }
        // Classes and absolute bounds: the floored classification of the
        // published values with the body extents and (at 512) Φ.
        let values: Vec<_> = owner.publish().rows.iter().map(|r| r.value).collect();
        let floor = evidence.floor.as_ref().map(|_| floors.as_slice());
        let again = classify_rows_floored(&owner.prep.layout, &values, &owner.prep.extents, floor);
        for (a, b) in owner.publish().rows.iter().zip(&again.rows) {
            assert_eq!(a.class, b.class, "{:?}", a.id);
        }
        let absolute = owner.publish().rows.iter()
            .filter(|r| matches!(r.class, RowClass::AbsoluteVerified { .. })).count();
        assert_eq!(absolute, evidence.absolute_verified.len());
    }
    // The D̄-inflated E: the near-Euler–Bernoulli state, a larger E_mo and Φ_mo.
    if stiff.selected_precision() == 512 {
        for ((b, _, mo_eb), (_, _, mo_t)) in eb.evidence().resolution_scale.iter().zip(&stiff.evidence().resolution_scale) {
            assert!(f64::from_bits(*mo_t) >= f64::from_bits(*mo_eb), "body {b}");
        }
        let (fe, ft) = (eb.evidence().floor.as_ref().unwrap(), stiff.evidence().floor.as_ref().unwrap());
        for (a, t) in fe.iter().zip(ft) {
            assert!(f64::from_bits(t.2) >= f64::from_bits(a.2));
        }
    }
    eprintln!("D-bar/E/floor: EB p={} stiff p={} soft p={}; E_mo EB {:?} stiff {:?}",
        eb.selected_precision(), stiff.selected_precision(), soft.selected_precision(),
        eb.evidence().resolution_scale.iter().map(|e| f64::from_bits(e.2)).collect::<Vec<_>>(),
        stiff.evidence().resolution_scale.iter().map(|e| f64::from_bits(e.2)).collect::<Vec<_>>());
}

// ------------------------------------------- §4.2: the sidecar and its identity

/// §4.2: association of the member-only sidecar. A sidecar entry without its
/// member, a Timoshenko document's member without its entry and a repeated
/// entry refuse with `RETAINED_TIMOSHENKO_REFUSAL`; a non-normal shear area
/// refuses by name; `from_frame` keeps refusing a Timoshenko frame and
/// `from_frame_with_leaf` builds the member with its entry.
#[test]
fn timoshenko_sidecar_association_and_constructors() {
    use super::{RetainedMemberRefusal, RETAINED_TIMOSHENKO_REFUSAL};
    let source = cantilever(2.0, 2, 0.75, 0.5, 1, 1.0);
    let leaf = |member, shear_area| TimoshenkoLeaf { member, shear_area };
    let ok = TimoshenkoLeafSet::new(&source, vec![leaf(1, 0.25), leaf(0, 0.25)]).unwrap();
    assert_eq!(ok.leaves().iter().map(|l| l.member).collect::<Vec<_>>(), vec![0, 1]);
    assert!(TimoshenkoLeafSet::new(&source, vec![]).unwrap().is_empty());
    for (leaves, want) in [
        (vec![leaf(0, 0.25), leaf(1, 0.25), leaf(9, 0.25)], TimoshenkoLeafError::UnknownMember(9)),
        (vec![leaf(0, 0.25)], TimoshenkoLeafError::MissingMember(1)),
        (vec![leaf(0, 0.25), leaf(0, 0.5), leaf(1, 0.25)], TimoshenkoLeafError::DuplicateMember(0)),
        (vec![leaf(0, 0.25), leaf(1, 0.0)], TimoshenkoLeafError::ShearArea(1)),
        (vec![leaf(0, f64::MIN_POSITIVE / 2.0), leaf(1, 0.25)], TimoshenkoLeafError::ShearArea(0)),
        (vec![leaf(0, f64::INFINITY), leaf(1, 0.25)], TimoshenkoLeafError::ShearArea(0)),
    ] {
        assert_eq!(TimoshenkoLeafSet::new(&source, leaves).unwrap_err(), want);
    }
    for e in [TimoshenkoLeafError::UnknownMember(9), TimoshenkoLeafError::MissingMember(1), TimoshenkoLeafError::DuplicateMember(0)] {
        assert_eq!(e.reason(), RETAINED_TIMOSHENKO_REFUSAL);
    }
    // A case call whose sidecar does not associate refuses before any record.
    let other = cantilever(2.0, 1, 0.75, 0.5, 1, 1.0);
    let mut invocation = RecordedInvocation::new(u64::MAX, OriginCapacity::for_calls(&[1], &[]).unwrap()).unwrap();
    let error = invocation
        .solve_timoshenko_cases(&[MemberSourceRef::nodal(&other)], &ok, CaseLimit::new(u64::MAX)).unwrap_err();
    assert!(matches!(error, super::super::adaptive::TimoshenkoCaseError::Association(TimoshenkoLeafError::UnknownMember(1))));
    assert!(invocation.calls().is_empty() && invocation.sources().is_empty());
    // FK frames.
    let section = crate::FrameSection::timoshenko(2.0, 5.0, 3.0, 11.0, 7.0, 2.0, 0.375).unwrap();
    let frame = crate::FrameElement::new(crate::FrameNode::new(0, [0.0; 3]).unwrap(),
        crate::FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap(), section, [0.0, 1.0, 0.0]).unwrap();
    assert_eq!(StraightMember::from_frame(4, &frame).unwrap_err(), RetainedMemberRefusal::Timoshenko);
    let (member, entry) = StraightMember::from_frame_with_leaf(4, &frame).unwrap();
    assert_eq!(entry, Some(TimoshenkoLeaf { member: 4, shear_area: 0.375 }));
    assert_eq!((member.area, member.second_moment_y, member.second_moment_z), (3.0, 11.0, 7.0));
    let eb = crate::FrameElement { section: crate::FrameSection::new(2.0, 5.0, 3.0, 11.0, 7.0, 2.0).unwrap(), ..frame };
    assert_eq!(StraightMember::from_frame_with_leaf(4, &eb).unwrap(), (StraightMember::from_frame(4, &eb).unwrap(), None));
    // The prepared leaf law: A_s,prep = RN(κ̂·A_prep) by FK's shear_area.
    let kappa = crate::shear::annulus_energy_shear_factor(0.077, 0.084, 0.3).unwrap();
    let prepared = TimoshenkoLeaf::prepared(3, kappa, 0.0035408).unwrap();
    assert_eq!(prepared.shear_area.to_bits(), (kappa * 0.0035408).to_bits());
}

/// §4.2: K4STF and K4SRC carry the Timoshenko block (count, then each entry's
/// member and shear-area bits) after the stiffness part, only for a non-empty
/// sidecar; every Euler–Bernoulli source keeps its bytes. The recorded case
/// sources, the case identity and the group identity bind every leaf.
#[test]
fn timoshenko_k4stf_k4src_block_encoding() {
    let source = cantilever(2.0, 2, 0.75, 0.5, 1, 1.0);
    let set = leaves(&source, 0.25);
    let empty = TimoshenkoLeafSet::new(&source, vec![]).unwrap();
    assert_eq!(source.encoding_with(None), source.encoding());
    assert_eq!(source.encoding_with(Some(&empty)), source.encoding());
    assert_eq!(source.stiffness_encoding_with(Some(&empty)), source.stiffness_encoding());
    let mut block = b"TSH\x01".to_vec();
    block.extend_from_slice(&2u32.to_le_bytes());
    for member in 0u32..2 {
        block.extend_from_slice(&member.to_le_bytes());
        block.extend_from_slice(&0.25f64.to_bits().to_le_bytes());
    }
    // Version 2 for a non-empty sidecar: the sixth byte, then the framed block.
    let stf = source.stiffness_encoding();
    let v2 = |mut bytes: Vec<u8>| {
        assert_eq!(bytes[5], 1);
        bytes[5] = TIMOSHENKO_VERSION;
        bytes
    };
    assert_eq!(source.stiffness_encoding_with(Some(&set)), [v2(stf.clone()), block.clone()].concat());
    // K4SRC: header and stiffness part (constraint values included), the block, then the rest.
    let split = stf.len() + 8 * source.constraints().len();
    let src = v2(source.encoding());
    assert_eq!(source.encoding_with(Some(&set)), [&src[..split], &block[..], &src[split..]].concat());
    // A different leaf moves both identities.
    let other = leaves(&source, 0.5);
    assert_ne!(source.stiffness_encoding_with(Some(&other)), source.stiffness_encoding_with(Some(&set)));
    // Recorded identities bind the block; the Euler–Bernoulli call keeps today's bytes.
    let mut invocation = RecordedInvocation::new(u64::MAX, OriginCapacity::for_calls(&[1, 1], &[]).unwrap()).unwrap();
    let refs = [MemberSourceRef::nodal(&source)];
    let timoshenko = invocation.solve_timoshenko_cases(&refs, &set, CaseLimit::new(u64::MAX)).unwrap();
    let eb = invocation.solve_member_cases(&refs, CaseLimit::new(u64::MAX)).unwrap();
    let origins = invocation.sources();
    assert_eq!(origins[0].identity, source.encoding_with(Some(&set)));
    assert_eq!(origins[0].stiffness, source.stiffness_encoding_with(Some(&set)));
    assert_eq!(origins[1].identity, source.encoding());
    assert_eq!(origins[1].stiffness, source.stiffness_encoding());
    let (ExecutionOutcome::Selected(t), ExecutionOutcome::Selected(e)) = (&timoshenko[0].outcome, &eb[0].outcome) else {
        panic!("both select");
    };
    assert_eq!(t.evidence().source_encoding, source.encoding_with(Some(&set)));
    assert_eq!(e.evidence().source_encoding, source.encoding());
    assert_eq!(t.prep.stiffness_identity(), source.stiffness_encoding_with(Some(&set)));
    assert_eq!(e.prep.stiffness_identity(), source.stiffness_encoding());
}

/// §4.2, §4.5: the receipt-recorded layouts O4 must not grow, pinned at their
/// pre-O4 sizes (checked against base 3826a62881): the adapter's types
/// (`StraightMember`, `PrimitiveSource`, `SourceParts`, `MemberSourceRef`,
/// `ProductMemberFacts`), `ProductRecipe`, the `TraceCopyWork`-recorded kernel
/// structs, `SectionPreparationWork.layout_bytes` and the proof's seven
/// serialized capacities. The sidecars live beside them.
#[cfg(target_pointer_width = "64")]
#[test]
fn timoshenko_sidecar_preserves_receipt_layouts() {
    use std::mem::size_of;
    use crate::structural::retained_api as k;
    let sizes = [
        size_of::<StraightMember>(), size_of::<PrimitiveSource>(), size_of::<SourceParts>(),
        size_of::<MemberSourceRef<'static>>(), size_of::<k::ProductMemberFacts>(), size_of::<k::ProductRecipe>(),
        size_of::<k::LaneWorkTrace<'static>>(), size_of::<k::ProductProofTrace<'static>>(),
        size_of::<super::super::product_certificate::final_case::CoverageFacts>(),
        size_of::<k::ProductFailureView<'static>>(), size_of::<k::NumericTrace>(), size_of::<k::TraceCopyWork>(),
        size_of::<k::PreparationConversion>(), size_of::<k::PreparedAnnulus>(),
    ];
    eprintln!("LAYOUTS {sizes:?}");
    assert_eq!(sizes, PINNED_LAYOUTS);
    let spent = k::prepare_product_annulus(0.1, 0.005);
    assert_eq!(spent.work().layout_bytes, PINNED_SECTION_LAYOUT_BYTES);
}
/// Measured on base 3826a62881 (a scratch copy of the crate at that commit).
const PINNED_LAYOUTS: [usize; 14] = [88, 248, 192, 16, 160, 16, 448, 1328, 56, 24, 168, 32, 40, 56];
const PINNED_SECTION_LAYOUT_BYTES: [usize; 8] = [3888, 144, 400, 80, 2144, 56, 24, 8];

/// Codex review of 38ccfd4d8b (BLOCKING, repaired): with an untagged block
/// after the stiffness part, this Timoshenko source and this Euler–Bernoulli
/// source had identical 280-byte K4SRC encodings and prepared identities (the
/// sidecar read as the Euler–Bernoulli source's load prefix). With the
/// version-2 header and the framed block they differ, as do their K4STF
/// identities and every recorded identity derived from them.
#[test]
fn k4src_timoshenko_encodings_are_injective() {
    use super::super::adaptive::PreparedCaseSource;
    let common = SourceParts {
        nodes: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        members: vec![StraightMember { id: 0, node_i: 0, node_j: 1, elastic_modulus: 2.0, shear_modulus: 1.0,
            area: 1.0, second_moment_y: 1.0, second_moment_z: 1.0, torsion_constant: 1.0, y_reference: [0.0, 1.0, 0.0] }],
        constraints: Component::ALL.iter().map(|&component| Constraint { dof: Dof { node: 0, component }, value: 0.0 }).collect(),
        ..Default::default()
    };
    let t = PrimitiveSource::new(SourceParts {
        stations: vec![Station { id: 0, member: 0, fraction: f64::from_bits(0x3ff00000) }],
        ..common.clone()
    })
    .unwrap();
    let set = TimoshenkoLeafSet::new(&t, vec![TimoshenkoLeaf { member: 0, shear_area: f64::from_bits(0x3f62610000000f00) }]).unwrap();
    let source_id = String::from_utf8(vec![0x61, 0x62, 0x3f, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    let eb = PrimitiveSource::new(SourceParts {
        loads: vec![NodalLoad { dof: Dof { node: 0, component: Component::Ux }, source_id, value: 1.0 }],
        ..common
    })
    .unwrap();
    assert_ne!(t.encoding_with(Some(&set)), eb.encoding());
    assert_ne!(t.stiffness_encoding_with(Some(&set)), eb.stiffness_encoding());
    assert_eq!((t.encoding_with(Some(&set))[5], eb.encoding()[5]), (TIMOSHENKO_VERSION, 1));
    let pt = PreparedCaseSource::new_with_shear(MemberSourceRef::nodal(&t), &set).unwrap();
    let pe = PreparedCaseSource::new(eb.clone()).unwrap();
    assert_ne!(pt.identity(), pe.identity());
    assert_ne!(pt.stiffness_identity(), pe.stiffness_identity());
    // The recorded sources (SourceOrigin identity and stiffness) and the
    // selected owners' identities differ too.
    let mut invocation = RecordedInvocation::new(u64::MAX, OriginCapacity::for_calls(&[1, 1], &[]).unwrap()).unwrap();
    let a = invocation.solve_timoshenko_cases(&[MemberSourceRef::nodal(&t)], &set, CaseLimit::new(u64::MAX)).unwrap();
    let b = invocation.solve_member_cases(&[MemberSourceRef::nodal(&eb)], CaseLimit::new(u64::MAX)).unwrap();
    let origins = invocation.sources();
    assert_ne!(origins[0].identity, origins[1].identity);
    assert_ne!(origins[0].stiffness, origins[1].stiffness);
    if let (ExecutionOutcome::Selected(x), ExecutionOutcome::Selected(y)) = (&a[0].outcome, &b[0].outcome) {
        assert_ne!(x.evidence().source_encoding, y.evidence().source_encoding);
    }
    // The version-2 bytes begin with the version-1 header except the sixth byte,
    // and the block opens with its tag right after the stiffness part.
    let v2 = t.encoding_with(Some(&set));
    let stiffness_len = t.stiffness_encoding().len() + 8 * t.constraints().len();
    assert_eq!(&v2[..5], b"K4SRC");
    assert_eq!(&v2[stiffness_len..stiffness_len + 4], TIMOSHENKO_BLOCK_TAG);
}
