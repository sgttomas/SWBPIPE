//! K4 tests of `retained/assemble.rs` (the brief's E): formation, assembly and
//! reduction at p.
use super::super::adaptive::{solve_case, CaseLimit, CaseOutcome, InvocationMeter, StageGuard};
use super::super::ledger::RetainedLedger;
use super::super::source::{PrimitiveSource, SourceParts};
use super::super::wide::multi::{Binary64Outcome, SupportedWidth, WideContext};
use super::super::wide::{Wide, Wide2, WideArith};
use super::super::wide_sum::ExactWideSum;
use super::{assemble, form_directional, form_members, reduced_rhs, MemberOperators, Structure};
use crate::structural::SparsePattern;
use std::collections::BTreeMap;

#[allow(dead_code)]
#[path = "models.rs"]
mod models;
#[allow(dead_code)]
#[path = "support.rs"]
mod support;
use support::{ctx, tok};

/// The assembled K at p (width L): the structure, the values and the members.
fn assembled<const L: usize>(
    source: &PrimitiveSource,
    p: u32,
) -> (Structure, Vec<Wide<L>>, Vec<MemberOperators<L>>)
where
    Wide<L>: SupportedWidth,
{
    let mut c = ctx::<L>(p);
    let mut sum = ExactWideSum::new();
    let guard = StageGuard::unlimited();
    let structure = Structure::new(source).unwrap();
    let members = form_members(&mut c, &mut sum, &guard, source).unwrap();
    let directional = form_directional(&mut c, &mut sum, source).unwrap();
    let k = assemble(
        &mut c,
        &mut sum,
        &guard,
        source,
        &structure,
        &members,
        &directional,
    )
    .unwrap();
    (structure, k, members)
}

/// (r, c) → token of every upper-triangle entry.
fn upper_tokens<const L: usize>(
    source: &PrimitiveSource,
    p: u32,
) -> BTreeMap<(usize, usize), String>
where
    Wide<L>: SupportedWidth,
{
    let (structure, k, _) = assembled::<L>(source, p);
    structure
        .entries()
        .filter(|&(r, c, _)| r <= c)
        .map(|(r, c, index)| ((r, c), tok(&k[index])))
        .collect()
}

fn upper_tokens_at(source: &PrimitiveSource, p: u32) -> BTreeMap<(usize, usize), String> {
    match p {
        53 | 128 | 192 | 256 => upper_tokens::<4>(source, p),
        320 | 512 => upper_tokens::<8>(source, p),
        _ => upper_tokens::<16>(source, p),
    }
}

#[test]
fn assembled_entries_equal_the_fraction_emulation_bit_for_bit() {
    let formation = models::parse_models(models::FORMATION);
    assert_eq!(formation.len(), 7);
    let mut expected: BTreeMap<(String, u32), BTreeMap<(usize, usize), String>> = BTreeMap::new();
    for line in models::FORMATION.lines().filter(|l| l.starts_with("k ")) {
        let f: Vec<&str> = line.split_whitespace().collect();
        expected
            .entry((f[1].to_string(), f[2].parse().unwrap()))
            .or_default()
            .insert(
                (f[4].parse().unwrap(), f[5].parse().unwrap()),
                f[6].to_string(),
            );
    }
    assert_eq!(expected.len(), 7 * 8);
    let mut entries = 0;
    for m in &formation {
        let source = m.source();
        for p in [53u32, 128, 192, 256, 320, 512, 576, 1024] {
            let want = &expected[&(m.name.clone(), p)];
            let got = upper_tokens_at(&source, p);
            assert_eq!(got.len(), want.len(), "{} p {p}", m.name);
            for (rc, token) in want {
                assert_eq!(&got[rc], token, "{} p {p} entry {rc:?}", m.name);
            }
            entries += want.len();
        }
    }
    assert!(entries > 5000, "{entries}");
}

/// The six rigid motions of the binary64 geometry at a member's two nodes:
/// three unit translations and three unit rotations about the global axes
/// through the origin (translations ω × x, exact in binary64 here).
fn rigid_motions(xi: [f64; 3], xj: [f64; 3]) -> Vec<[f64; 12]> {
    let mut out = Vec::new();
    for k in 0..3 {
        let mut r = [0.0; 12];
        r[k] = 1.0;
        r[6 + k] = 1.0;
        out.push(r);
    }
    for k in 0..3 {
        let mut r = [0.0; 12];
        for (base, x) in [(0usize, xi), (6, xj)] {
            let t = match k {
                0 => [0.0, -x[2], x[1]],
                1 => [x[2], 0.0, -x[0]],
                _ => [-x[1], x[0], 0.0],
            };
            r[base..base + 3].copy_from_slice(&t);
            r[base + 3 + k] = 1.0;
        }
        out.push(r);
    }
    out
}

fn f64_of<const L: usize>(w: &Wide<L>) -> f64
where
    Wide<L>: SupportedWidth,
{
    match w.to_binary64() {
        Binary64Outcome::Normal(x) => x,
        other => panic!("{other:?}"),
    }
}

/// The rigid-mode check of every member at p: |K_e·r|_a ≤ 64·2^-p·A_a for
/// each rigid motion r and row a, where A_a = Σ_b (|B|ᵀ|D||B|)_ab·|r_b| is the
/// magnitude of the terms whose exact sum K_e·r would vanish (a bound on
/// Σ|K_e||r| is not enough: an entry that is an exact zero, such as a
/// torsion–translation coupling of a skew member, is formed as a cancelled
/// expansion and its rounding is relative to its terms, not to itself).
/// |K_e·r| is exact; A_a is evaluated in binary64 (its relative error, below
/// 2^-40, is covered by the factor 1 + 2^-30). Returns the worst
/// |K_e·r|_a / (2^-p·A_a) (evidence).
fn rigid_mode_check<const L: usize>(source: &PrimitiveSource, p: u32) -> f64
where
    Wide<L>: SupportedWidth,
{
    let (_, _, members) = assembled::<L>(source, p);
    let mut c = ctx::<L>(p);
    let mut c16 = ctx::<16>(1024);
    let mut worst = 0.0f64;
    for (op, m) in members.iter().zip(source.members()) {
        let xi = source.nodes()[m.node_i as usize];
        let xj = source.nodes()[m.node_j as usize];
        let b: Vec<Vec<f64>> = (0..6)
            .map(|r| (0..12).map(|col| f64_of(&op.b[r][col]).abs()).collect())
            .collect();
        let (axial, torsion, bz, by) = (
            f64_of(&op.axial),
            f64_of(&op.torsion),
            f64_of(&op.bend_z),
            f64_of(&op.bend_y),
        );
        let d = [
            [axial, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, torsion, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 4.0 * bz, 2.0 * bz, 0.0, 0.0],
            [0.0, 0.0, 2.0 * bz, 4.0 * bz, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 4.0 * by, 2.0 * by],
            [0.0, 0.0, 0.0, 0.0, 2.0 * by, 4.0 * by],
        ];
        for r in rigid_motions(xi, xj) {
            for a in 0..12 {
                let mut t = ExactWideSum::new();
                let mut bound = 0.0f64;
                for (col, &rb) in r.iter().enumerate() {
                    if rb == 0.0 {
                        continue;
                    }
                    t.add_product(&mut c, op.ke(a, col), &support::lift::<L>(rb), false)
                        .unwrap();
                    for (x, dx) in d.iter().enumerate() {
                        for (y, dxy) in dx.iter().enumerate() {
                            bound += b[x][a] * dxy * b[y][col] * rb.abs();
                        }
                    }
                }
                t.make_absolute();
                let mut scaled = ExactWideSum::new();
                scaled.add_scaled(&t, false, 1, i64::from(p)).unwrap();
                let lhs = if scaled.is_zero() {
                    0.0
                } else {
                    f64_of::<16>(&scaled.round::<16>(&mut c16).unwrap())
                };
                let limit = 64.0 * bound * (1.0 + f64::from_bits(0x3E10_0000_0000_0000));
                assert!(
                    lhs <= limit,
                    "member {} p {p} row {a} motion {r:?}: {lhs:e} > {limit:e}",
                    m.id
                );
                if bound > 0.0 {
                    worst = worst.max(lhs / bound);
                } else {
                    assert_eq!(lhs, 0.0);
                }
            }
        }
    }
    worst
}

#[test]
fn every_element_annihilates_the_rigid_motions_to_within_64_units_of_2_to_minus_p() {
    for m in models::parse_models(models::FORMATION) {
        let source = m.source();
        let mut report = Vec::new();
        for p in [128u32, 192, 256] {
            report.push((p, rigid_mode_check::<4>(&source, p)));
        }
        for p in [320u32, 512] {
            report.push((p, rigid_mode_check::<8>(&source, p)));
        }
        for p in [576u32, 1024] {
            report.push((p, rigid_mode_check::<16>(&source, p)));
        }
        println!("{} worst |K_e r|_a / (2^-p A_a): {report:?}", m.name);
    }
}

#[test]
fn at_p53_the_axis_aligned_element_agrees_with_the_products_local_stiffness() {
    // Along x with y_reference (0, 1, 0), global = local.
    let m = models::parse_models(models::FORMATION)
        .into_iter()
        .find(|m| m.name == "M-AX")
        .unwrap();
    let source = m.source();
    let member = &source.members()[0];
    assert_eq!(member.y_reference, [0.0, 1.0, 0.0]);
    let (_, _, members) = assembled::<4>(&source, 53);
    let section = crate::FrameSection {
        elastic_modulus: member.elastic_modulus,
        shear_modulus: member.shear_modulus,
        area: member.area,
        second_moment_y: member.second_moment_y,
        second_moment_z: member.second_moment_z,
        torsion_constant: member.torsion_constant,
    };
    let product = crate::local_stiffness(crate::FrameProperties {
        section,
        length: 2.0,
    })
    .unwrap();
    let mut nonzero = 0;
    for a in 0..12 {
        for b in 0..12 {
            let ours = match members[0].ke(a, b).to_binary64() {
                Binary64Outcome::Normal(x) => x,
                other => panic!("{other:?}"),
            };
            let theirs = product[a][b];
            assert_eq!(ours == 0.0, theirs == 0.0, "zero pattern at ({a}, {b})");
            if theirs != 0.0 {
                nonzero += 1;
                assert!(
                    (ours - theirs).abs() <= theirs.abs() * f64::from_bits(0x3CD0_0000_0000_0000),
                    "({a}, {b}): {ours:e} vs {theirs:e}"
                );
            }
        }
    }
    assert_eq!(nonzero, 40);
}

// ---- a test-only port of K-D5's re-formation (`formation_check.rs`:
// `lift3`, `sub3`, `dot3`, `cross3`, `scale3`, `norm3`, `chord_axes`, `rotate`,
// `frame_matrix`), sequential K3a `WideArith` operations at 128 bits.
type Element = [[Wide2; 12]; 12];
type Vec3 = [Wide2; 3];

fn kd5_lift3(v: [f64; 3]) -> Vec3 {
    [
        Wide2::from_f64(v[0]).unwrap(),
        Wide2::from_f64(v[1]).unwrap(),
        Wide2::from_f64(v[2]).unwrap(),
    ]
}
fn kd5_dot3(a: &mut WideArith, x: &Vec3, y: &Vec3) -> Wide2 {
    let p0 = a.mul(&x[0], &y[0]).unwrap();
    let p1 = a.mul(&x[1], &y[1]).unwrap();
    let p2 = a.mul(&x[2], &y[2]).unwrap();
    let s = a.add(&p0, &p1).unwrap();
    a.add(&s, &p2).unwrap()
}
fn kd5_scale3(a: &mut WideArith, x: &Vec3, d: &Wide2) -> Vec3 {
    [
        a.div(&x[0], d).unwrap(),
        a.div(&x[1], d).unwrap(),
        a.div(&x[2], d).unwrap(),
    ]
}
fn kd5_norm3(a: &mut WideArith, x: &Vec3) -> Wide2 {
    let s = kd5_dot3(a, x, x);
    a.sqrt(&s).unwrap()
}
fn kd5_chord_axes(
    a: &mut WideArith,
    xi: [f64; 3],
    xj: [f64; 3],
    yr: [f64; 3],
) -> ([Vec3; 3], Wide2) {
    let (pj, pi) = (kd5_lift3(xj), kd5_lift3(xi));
    let d = [
        a.sub(&pj[0], &pi[0]).unwrap(),
        a.sub(&pj[1], &pi[1]).unwrap(),
        a.sub(&pj[2], &pi[2]).unwrap(),
    ];
    let length = kd5_norm3(a, &d);
    let ex = kd5_scale3(a, &d, &length);
    let yr = kd5_lift3(yr);
    let projection = kd5_dot3(a, &yr, &ex);
    let mut yc = [Wide2::ZERO; 3];
    for k in 0..3 {
        let t = a.mul(&projection, &ex[k]).unwrap();
        yc[k] = a.sub(&yr[k], &t).unwrap();
    }
    let ym = kd5_norm3(a, &yc);
    let ey = kd5_scale3(a, &yc, &ym);
    let c = |a: &mut WideArith, p: usize, q: usize| {
        let l = a.mul(&ex[p], &ey[q]).unwrap();
        let r = a.mul(&ex[q], &ey[p]).unwrap();
        a.sub(&l, &r).unwrap()
    };
    let ez = [c(a, 1, 2), c(a, 2, 0), c(a, 0, 1)];
    ([ex, ey, ez], length)
}
fn kd5_rotate(a: &mut WideArith, k: &Element, axes: &[Vec3; 3]) -> Element {
    let mut out = [[Wide2::ZERO; 12]; 12];
    for bi in 0..4 {
        for bj in 0..4 {
            let mut empty = true;
            for p in 0..3 {
                for q in 0..3 {
                    empty &= k[3 * bi + p][3 * bj + q].is_zero();
                }
            }
            if empty {
                continue;
            }
            let mut temp = [[Wide2::ZERO; 3]; 3];
            for p in 0..3 {
                for c in 0..3 {
                    let mut s = Wide2::ZERO;
                    for q in 0..3 {
                        let kv = &k[3 * bi + p][3 * bj + q];
                        if kv.is_zero() || axes[q][c].is_zero() {
                            continue;
                        }
                        let t = a.mul(kv, &axes[q][c]).unwrap();
                        s = a.add(&s, &t).unwrap();
                    }
                    temp[p][c] = s;
                }
            }
            for r in 0..3 {
                for c in 0..3 {
                    let mut s = Wide2::ZERO;
                    for p in 0..3 {
                        if axes[p][r].is_zero() || temp[p][c].is_zero() {
                            continue;
                        }
                        let t = a.mul(&axes[p][r], &temp[p][c]).unwrap();
                        s = a.add(&s, &t).unwrap();
                    }
                    out[3 * bi + r][3 * bj + c] = s;
                }
            }
        }
    }
    out
}
fn kd5_frame_matrix(
    a: &mut WideArith,
    xi: [f64; 3],
    xj: [f64; 3],
    m: &super::super::source::StraightMember,
) -> Element {
    let (axes, length) = kd5_chord_axes(a, xi, xj, m.y_reference);
    let lift = |x: f64| Wide2::from_f64(x).unwrap();
    let (em, gm) = (lift(m.elastic_modulus), lift(m.shear_modulus));
    let l2 = a.mul(&length, &length).unwrap();
    let l3 = a.mul(&l2, &length).unwrap();
    let ea = a.mul(&em, &lift(m.area)).unwrap();
    let axial = a.div(&ea, &length).unwrap();
    let gj = a.mul(&gm, &lift(m.torsion_constant)).unwrap();
    let torsion = a.div(&gj, &length).unwrap();
    let mut coefficients = |i: f64| -> [Wide2; 4] {
        let ei = a.mul(&em, &lift(i)).unwrap();
        let t12 = a.mul(&lift(12.0), &ei).unwrap();
        let t6 = a.mul(&lift(6.0), &ei).unwrap();
        let t4 = a.mul(&lift(4.0), &ei).unwrap();
        let t2 = a.mul(&lift(2.0), &ei).unwrap();
        [
            a.div(&t12, &l3).unwrap(),
            a.div(&t6, &l2).unwrap(),
            a.div(&t4, &length).unwrap(),
            a.div(&t2, &length).unwrap(),
        ]
    };
    let [z12, z6, z4, z2] = coefficients(m.second_moment_z);
    let [y12, y6, y4, y2] = coefficients(m.second_moment_y);
    let mut k = [[Wide2::ZERO; 12]; 12];
    let pair = |k: &mut Element, i: usize, j: usize, v: Wide2| {
        k[i][i] = v;
        k[j][j] = v;
        k[i][j] = v.neg();
        k[j][i] = v.neg();
    };
    pair(&mut k, 0, 6, axial);
    pair(&mut k, 3, 9, torsion);
    let bz = [
        [z12, z6, z12.neg(), z6],
        [z6, z4, z6.neg(), z2],
        [z12.neg(), z6.neg(), z12, z6.neg()],
        [z6, z2, z6.neg(), z4],
    ];
    let by = [
        [y12, y6.neg(), y12.neg(), y6.neg()],
        [y6.neg(), y4, y6, y2],
        [y12.neg(), y6, y12, y6],
        [y6.neg(), y2, y6, y4],
    ];
    for (idx, t) in [([1usize, 5, 7, 11], bz), ([2usize, 4, 8, 10], by)] {
        for r in 0..4 {
            for c in 0..4 {
                k[idx[r]][idx[c]] = t[r][c];
            }
        }
    }
    kd5_rotate(a, &k, &axes)
}

fn widen2(x: &Wide2) -> Wide<4> {
    let (negative, exponent, s) = x.parts();
    Wide::<4>::from_parts(negative, exponent, [0, 0, s[0], s[1]]).unwrap()
}

#[test]
fn at_128_the_element_agrees_with_k_d5s_re_formation_within_64_ulps_of_its_largest_entry() {
    let mut worst_ulps = 0.0f64;
    for m in models::parse_models(models::FORMATION) {
        let source = m.source();
        let (_, _, members) = assembled::<4>(&source, 128);
        for (op, member) in members.iter().zip(source.members()) {
            let xi = source.nodes()[member.node_i as usize];
            let xj = source.nodes()[member.node_j as usize];
            let mut arith = WideArith::new(128).unwrap();
            let theirs = kd5_frame_matrix(&mut arith, xi, xj, member);
            let mut top = i64::MIN;
            for a in 0..12 {
                for b in 0..12 {
                    if !op.ke(a, b).is_zero() {
                        top = top.max(op.ke(a, b).exponent());
                    }
                }
            }
            for a in 0..12 {
                for b in 0..12 {
                    let mut d = ExactWideSum::new();
                    d.add_wide(op.ke(a, b), false).unwrap();
                    d.add_wide(&widen2(&theirs[a][b]), true).unwrap();
                    d.make_absolute();
                    // |d| ≤ 64·2^(top − 127)
                    let mut test = ExactWideSum::new();
                    test.add_scaled(&d, false, 1, 127 - top).unwrap();
                    test.add_integer(true, &[64], 0).unwrap();
                    assert!(
                        test.signum() <= 0,
                        "{} member {} ({a}, {b})",
                        m.name,
                        member.id
                    );
                    let mut c16 = ctx::<16>(1024);
                    let mut scaled = ExactWideSum::new();
                    scaled.add_scaled(&d, false, 1, 127 - top).unwrap();
                    let ulps = scaled
                        .round(&mut c16)
                        .unwrap()
                        .to_binary64()
                        .value()
                        .unwrap_or(0.0);
                    worst_ulps = worst_ulps.max(ulps);
                }
            }
        }
    }
    println!("worst |K4 − K-D5| at 128, in ulps of the largest entry: {worst_ulps}");
}

/// The source with every list permuted (reversed or rotated).
fn permuted(parts: &SourceParts) -> SourceParts {
    let mut p = parts.clone();
    p.members.reverse();
    let half = p.springs.len() / 2;
    p.springs.rotate_left(half);
    p.directional_springs.reverse();
    p.constraints.reverse();
    p.loads.reverse();
    p.stations.reverse();
    p.supports.reverse();
    p
}

fn rhs_tokens<const L: usize>(source: &PrimitiveSource, p: u32) -> Vec<String>
where
    Wide<L>: SupportedWidth,
{
    let (structure, k, _) = assembled::<L>(source, p);
    let ledger = RetainedLedger::from_source(source).unwrap();
    let mut c = ctx::<L>(p);
    let mut sum = ExactWideSum::new();
    let free = source.free_dofs();
    reduced_rhs(&mut c, &mut sum, source, &structure, &k, &ledger, &free)
        .unwrap()
        .iter()
        .map(tok)
        .collect()
}

#[test]
fn permuting_every_list_gives_bit_identical_k_rhs_and_encodings() {
    let mut checked = 0;
    for name in [
        "SKEW6-K1E-12",
        "DUPLICATE",
        "DIRECTIONAL-SPAN",
        "PRESCRIBED",
        "S8-W-1e-10",
        "N09-B",
    ] {
        let m = models::model(name);
        let a = m.source();
        let b = PrimitiveSource::new(permuted(&m.parts)).unwrap();
        assert_ne!(
            format!("{:?}", m.parts),
            format!("{:?}", permuted(&m.parts)),
            "{name}: a new list order"
        );
        assert_eq!(a.encoding(), b.encoding(), "{name}");
        assert_eq!(a.stiffness_encoding(), b.stiffness_encoding(), "{name}");
        let (la, lb) = (
            RetainedLedger::from_source(&a).unwrap(),
            RetainedLedger::from_source(&b).unwrap(),
        );
        assert_eq!(la.encoding(), lb.encoding(), "{name}");
        for p in [128u32, 512] {
            assert_eq!(
                upper_tokens_at(&a, p),
                upper_tokens_at(&b, p),
                "{name} K at {p}"
            );
        }
        assert_eq!(
            rhs_tokens::<4>(&a, 128),
            rhs_tokens::<4>(&b, 128),
            "{name} rhs"
        );
        assert_eq!(
            rhs_tokens::<8>(&a, 512),
            rhs_tokens::<8>(&b, 512),
            "{name} rhs"
        );
        checked += 1;
    }
    assert_eq!(checked, 6);
}

#[test]
fn the_duplicate_operand_entry_is_one_exact_sum_where_a_sequential_fold_loses_the_weak_member() {
    // DUPLICATE: members 1 (0–1) and 3 (1–2) are identical and collinear, so
    // their contributions to K(uy_1, rz_1) cancel exactly; member 2, 2^-300 as
    // stiff, sorts between them in the canonical (id) order. Entry (7, 11) =
    // node 1's (uy, rz).
    let m = models::model("DUPLICATE");
    let source = m.source();
    assert_eq!(
        source.members().iter().map(|m| m.id).collect::<Vec<_>>(),
        vec![1, 2, 3],
        "the weak member sits between the two"
    );
    fn check<const L: usize>(source: &PrimitiveSource, p: u32, fold_loses: bool)
    where
        Wide<L>: SupportedWidth,
    {
        let (structure, k, members) = assembled::<L>(source, p);
        let entry = k[structure.pattern.find(7, 11).unwrap()];
        // The contributions, in canonical order (member 1: node 1 is its j
        // end; members 2 and 3: node 1 is their i end).
        let parts = [
            members[0].ke(7, 11),
            members[1].ke(1, 5),
            members[2].ke(1, 5),
        ];
        let mut exact = ExactWideSum::new();
        for x in parts {
            exact.add_wide(x, false).unwrap();
        }
        let mut c: WideContext<L> = ctx::<L>(p);
        assert_eq!(tok(&entry), tok(&exact.round(&mut c).unwrap()), "p {p}");
        assert!(!entry.is_zero(), "p {p}: the weak member is kept");
        let folded = {
            let s = c.add(parts[0], parts[1]).unwrap();
            c.add(&s, parts[2]).unwrap()
        };
        assert_eq!(folded.is_zero(), fold_loses, "p {p}: sequential fold");
    }
    check::<4>(&source, 128, true);
    check::<4>(&source, 256, true);
    check::<8>(&source, 512, false);
}

#[test]
fn a_prescribed_motion_control_matches_its_exact_reference() {
    let m = models::model("PRESCRIBED");
    let source = m.source();
    assert!(source.constraints().iter().any(|c| c.value != 0.0));
    assert!(source.loads().is_empty());
    let mut meter = InvocationMeter::new(u64::MAX);
    match solve_case(source.clone(), CaseLimit::new(u64::MAX), &mut meter) {
        CaseOutcome::Selected(solve) => {
            let (worst, at, compared) = models::compare(&source, &solve.publish().rows, &m.expect);
            assert!(compared > 40, "{compared}");
            assert!(worst <= 1.0, "{worst} at {at}");
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn the_pattern_from_positions_equals_the_connectivity_pattern_where_both_apply() {
    let mut checked = 0;
    for m in models::models() {
        if !m.parts.directional_springs.is_empty() {
            continue;
        }
        let Ok(source) = PrimitiveSource::new(m.parts.clone()) else {
            continue;
        };
        let structure = Structure::new(&source).unwrap();
        let elements: Vec<(usize, usize)> = source
            .members()
            .iter()
            .map(|m| (m.node_i as usize, m.node_j as usize))
            .collect();
        let diagonal: Vec<usize> = source.springs().iter().map(|s| s.dof.global()).collect();
        let connectivity =
            SparsePattern::from_connectivity(source.node_count(), &elements, &diagonal).unwrap();
        assert_eq!(structure.pattern, connectivity, "{}", m.name);
        checked += 1;
    }
    assert!(checked > 30, "{checked}");
}

#[test]
fn the_reduced_rhs_enters_prescribed_columns_exactly_and_rounds_once() {
    // Every rhs entry of PRESCRIBED equals the exact ledger − Σ K_ic·u_c
    // rounded once, where the sum is re-formed here term by term from the
    // assembled row (and differs from a binary64 rhs).
    let source = models::model("PRESCRIBED").source();
    let (structure, k, _) = assembled::<4>(&source, 128);
    let ledger = RetainedLedger::from_source(&source).unwrap();
    let free = source.free_dofs();
    let mut c = ctx::<4>(128);
    let mut sum = ExactWideSum::new();
    let rhs = reduced_rhs(&mut c, &mut sum, &source, &structure, &k, &ledger, &free).unwrap();
    let mut wider = 0;
    for (a, &i) in free.iter().enumerate() {
        let mut s = ExactWideSum::new();
        for index in structure.pattern.row_range(i) {
            let col = structure.pattern.column(index);
            if let Some(v) = source.constraint(col) {
                s.add_product(&mut c, &k[index], &support::lift::<4>(v), true)
                    .unwrap();
            }
        }
        assert_eq!(tok(&rhs[a]), tok(&s.round(&mut c).unwrap()), "row {i}");
        if !rhs[a].fits_precision(53) {
            wider += 1;
        }
    }
    // Entries wider than binary64: the rhs is never rounded back to 53 bits.
    assert!(wider >= 2, "{wider}");
}
