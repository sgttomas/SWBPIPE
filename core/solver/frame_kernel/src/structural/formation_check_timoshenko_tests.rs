//! T4-U8 (D-6): K-D5 with a Timoshenko straight member (`DEL-05-03` Design
//! `section-bases-and-timoshenko.md` §4 items 1, 2 and 4, §8). Invented
//! inputs; the S-B section is SHEAR_REFERENCE §5's (ro .1, ri .09, E 200 GPa,
//! nu .3, G = E/2.6).
use super::super::{SolveQuality, StiffnessContribution, StructuralSystem};
use super::*;
use crate::load_ledger::{AssembledForce, Formation, LoadLedger};
use crate::{FrameNode, FrameSection, Matrix12};

fn round(w: &Wide2) -> f64 {
    let mut acc = ExactAccumulator::new();
    for &t in w.split_binary64().unwrap().terms() {
        acc.add(t).unwrap();
    }
    acc.round().unwrap()
}

/// The S-B section with the energy-matched shear area (`shear_area` scales
/// it, 1.0 for the selected factor).
fn s_b_section(scale: f64) -> FrameSection {
    let pi = std::f64::consts::PI;
    let (a, i) = (0.0019 * pi, 0.0000085975 * pi);
    let kappa = crate::shear::annulus_energy_shear_factor(0.09, 0.1, 0.3).unwrap();
    FrameSection::timoshenko(200e9, 1000e9 / 13.0, a, i, i, 2.0 * i, kappa * a * scale).unwrap()
}
fn euler_bernoulli_twin(section: FrameSection) -> FrameSection {
    FrameSection {
        beam_theory: crate::BeamTheory::EulerBernoulli,
        ..section
    }
}
fn frame(i: usize, j: usize, xi: [f64; 3], xj: [f64; 3], section: FrameSection, y: [f64; 3]) -> FrameElement {
    FrameElement::new(
        FrameNode::new(i, xi).unwrap(),
        FrameNode::new(j, xj).unwrap(),
        section,
        y,
    )
    .unwrap()
}
fn skew(section: FrameSection) -> FrameElement {
    frame(0, 1, [0.0; 3], [1.0, 2.0, 2.0], section, [1.0, 0.0, 0.0])
}
/// A 2 m cantilever along x (node 0 clamped).
fn cantilever(section: FrameSection) -> FrameElement {
    frame(0, 1, [0.0; 3], [2.0, 0.0, 0.0], section, [0.0, 1.0, 0.0])
}
fn uniform(frame: FrameElement, start: f64, end: f64) -> LoadFormula {
    LoadFormula::UniformStraight {
        frame,
        intensity: IntensityPrimitive::Authored([0.0, -1500.0, 400.0]),
        start_fraction: start,
        end_fraction: end,
    }
}
fn primitive(formula: LoadFormula, row: usize, source: &str) -> LoadPrimitive {
    LoadPrimitive {
        term_index: row,
        identity: LoadSummandIdentity {
            family: "uniform_straight".into(),
            source: source.into(),
            ordinal: row,
        },
        local_row: row,
        formula,
        force_scale_exponent: 0,
    }
}

/// A ledger with one formed term per element row, valued at `values` and
/// closed by `formula`'s row primitive (manifest and supplied primitive
/// enumerated from the same recipe).
fn ledger(formula: &LoadFormula, values: [f64; 12], source: &str) -> AssembledForce {
    let mut ledger = LoadLedger::new();
    for row in 0..12 {
        ledger.push_formed("uniform", row, values[row], Formation::CannotBound, 0.0, false);
        let p = primitive(formula.clone(), row, source);
        assert!(ledger.set_closure_manifest(row, vec![p.closure_entry()]));
        ledger.attach_load_primitive(p);
    }
    ledger.finish(12).unwrap()
}
/// The binary64 re-formation of `formula` (the force the solve uses).
fn formed(formula: &LoadFormula) -> [f64; 12] {
    let (v, _) = load_reformation(&primitive(formula.clone(), 0, "member")).unwrap();
    v.map(|w| round(&w))
}

/// The cantilever member's identity (`FormationSource::frame_members`): its
/// primitives' sources begin with it (`member:1:T:w`).
const MEMBER_T: &str = "member:1:T:";
/// The one-member formation source of `frame` under `MEMBER_T`.
fn member_t_source(frame: &FrameElement) -> FormationSource<'static> {
    FormationSource {
        node_count: 2,
        frames: vec![*frame],
        frame_members: vec![MEMBER_T.into()],
        ..FormationSource::default()
    }
}

/// The K-D5 checked cantilever: `stiffness_frame` in K and the source,
/// `force` closed by its own primitives.
fn checked_cantilever(
    stiffness_frame: &FrameElement,
    force: &AssembledForce,
) -> super::super::StructuralSolution {
    let k: Matrix12 = stiffness_frame.global_stiffness().unwrap();
    let dense: Vec<Vec<f64>> = k.iter().map(|r| r.to_vec()).collect();
    let mut contributions = Vec::new();
    for r in 0..12 {
        for c in 0..12 {
            contributions.push(StiffnessContribution {
                row: r,
                col: c,
                value: k[r][c],
            });
        }
    }
    let source = FormationSource {
        node_count: 2,
        loads: force.load_primitives().into(),
        frames: vec![*stiffness_frame],
        frame_members: vec![MEMBER_T.into()],
        ..FormationSource::default()
    };
    let prescribed: Vec<_> = (0..6).map(|i| (i, 0.0)).collect();
    let free: Vec<usize> = (6..12).collect();
    let checked =
        StructuralSystem::assembled(&dense, force, &free, &prescribed, Some(&contributions), None)
            .with_formation_source(&source);
    super::super::solve_formation_checked_structural_dense(&checked).unwrap()
}
fn unavailable_detail(solution: &super::super::StructuralSolution) -> String {
    match &solution.formation_check.as_ref().expect("a formation finding").reason {
        FormationCheckReason::FormationCheckUnavailable { detail } => detail.clone(),
        other => panic!("{other:?}"),
    }
}

/// Design §4 item 1: frame_matrix forms phi, 1+phi, 4+phi, 2-phi at p from
/// the frame's own leaves; it agrees with the binary64 formation to
/// round-off, is not the Euler–Bernoulli matrix, is exact where binary64 is
/// (phi = 2, k2 = 0), scales exactly with 2^b, and keeps the rigid null space.
#[test]
fn kd5_timoshenko_frame_reformation_agrees_with_binary64() {
    let mut arith = WideArith::new(FORMATION_PRECISION).unwrap();
    for scale in [1.0, 0.01, 1e3] {
        let e = skew(s_b_section(scale));
        let wide = frame_matrix(&mut arith, &e).unwrap();
        let binary64 = e.global_stiffness().unwrap();
        let eb = frame_matrix(&mut arith, &skew(euler_bernoulli_twin(s_b_section(scale)))).unwrap();
        let size = binary64.iter().flatten().fold(0.0_f64, |m, v| m.max(v.abs()));
        let mut apart = 0.0_f64;
        for r in 0..12 {
            for c in 0..12 {
                let w = round(&wide[r][c]);
                assert!((w - binary64[r][c]).abs() <= 1e-13 * size, "[{r}][{c}] {w} {}", binary64[r][c]);
                apart = apart.max((w - round(&eb[r][c])).abs());
            }
        }
        // The shear correction (about phi times the bending entries, which
        // are below the axial entries here) is visible in every case.
        let phi = e.properties().unwrap().timoshenko_phi().unwrap().unwrap()[0];
        assert!(apart > 1e-3 * phi * size, "scale {scale}: the Timoshenko matrix is not the EB one");
    }
    // phi_y = 2 exactly: (2 - phi) = 0 at p and in binary64; the bits agree.
    let exact = frame(
        0,
        1,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        FrameSection::timoshenko(1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 12.0).unwrap(),
        [0.0, 1.0, 0.0],
    );
    let wide = frame_matrix(&mut arith, &exact).unwrap();
    let binary64 = exact.global_stiffness().unwrap();
    for r in 0..12 {
        for c in 0..12 {
            assert_eq!(round(&wide[r][c]).to_bits(), binary64[r][c].to_bits(), "[{r}][{c}]");
        }
    }
    assert!(wide[crate::RY][crate::RY + 6].is_zero());
    // K2b at p: E and G times 2^b give exactly 2^b times every entry.
    let e = skew(s_b_section(1.0));
    let k0 = frame_matrix(&mut arith, &e).unwrap();
    for b in [-64, 64] {
        let kb = frame_matrix(&mut arith, &e.force_scaled(crate::ForceScale::new(b).unwrap()).unwrap()).unwrap();
        for r in 0..12 {
            for c in 0..12 {
                assert_eq!(kb[r][c].cmp_value(&k0[r][c].mul_pow2(i64::from(b)).unwrap()), std::cmp::Ordering::Equal);
            }
        }
    }
    // The rigid null space at p: a dyadic translation and rotation.
    let (omega, t) = ([0.25, 0.5, -0.125], [1.0, -2.0, 0.5]);
    let motion = |x: [f64; 3]| {
        [
            t[0] + (omega[1] * x[2] - omega[2] * x[1]),
            t[1] + (omega[2] * x[0] - omega[0] * x[2]),
            t[2] + (omega[0] * x[1] - omega[1] * x[0]),
            omega[0],
            omega[1],
            omega[2],
        ]
    };
    let mut u = [0.0; 12];
    u[..6].copy_from_slice(&motion(e.node_i.coordinates));
    u[6..].copy_from_slice(&motion(e.node_j.coordinates));
    let size = k0.iter().flatten().map(|v| round(v).abs()).fold(0.0, f64::max);
    for row in &k0 {
        let mut acc = ExactAccumulator::new();
        for (kv, &x) in row.iter().zip(&u) {
            kv.add_product_to(&mut acc, x).unwrap();
        }
        assert!(acc.round().unwrap().abs() <= 1e-25 * size);
    }
}

/// Design §4 items 1 and 4: a closed Timoshenko cantilever with a full-span
/// uniform load passes K-D5, and its full-span load vector is the
/// Euler–Bernoulli re-formation bit for bit (uniform_straight_balls keeps its
/// exact formula).
#[test]
fn kd5_timoshenko_full_span_closes_and_passes() {
    let t = cantilever(s_b_section(1.0));
    let eb = cantilever(euler_bernoulli_twin(s_b_section(1.0)));
    let recipe = uniform(t, 0.0, 1.0);
    let (vt, rt) = load_reformation(&primitive(recipe.clone(), 0, "member")).unwrap();
    let (ve, re) = load_reformation(&primitive(uniform(eb, 0.0, 1.0), 0, "member")).unwrap();
    for k in 0..12 {
        assert_eq!(vt[k].cmp_value(&ve[k]), std::cmp::Ordering::Equal);
        assert_eq!(rt[k].to_bits(), re[k].to_bits());
    }
    let force = ledger(&recipe, formed(&recipe), "member:1:T:w");
    let solution = checked_cantilever(&t, &force);
    assert_eq!(solution.report.quality, SolveQuality::Passed, "{:?}", solution.formation_check);
    // The certified recovery of the member (frame_matrix at p, the same
    // Timoshenko element) gives the root actions of statics: w = (0, -1500,
    // 400) N/m over L = 2 m, so the root carries Fy = 3000 N, Fz = -800 N,
    // Mz = 3000 N·m and My = 800 N·m, and the free tip carries nothing.
    let bound = solution.member_bound.as_ref().unwrap();
    let source = member_t_source(&t);
    let loads = CertifiedMemberLoads::from_member_primitives(0, 1, force.load_primitives(), MEMBER_T)
        .unwrap()
        .with_member_source(&source, MEMBER_T);
    let rows = certify_straight_recovery(&t, &loads, &solution.displacements, bound).unwrap();
    for (row, expected) in [
        (crate::UY, 3000.0),
        (crate::UZ, -800.0),
        (crate::RZ, 3000.0),
        (crate::RY, 800.0),
        (6 + crate::UY, 0.0),
        (6 + crate::UZ, 0.0),
        (6 + crate::RZ, 0.0),
        (6 + crate::RY, 0.0),
    ] {
        let r = rows.ends[row];
        assert!(
            (r.published - expected).abs() <= r.bound() + 1e-9 * 3000.0,
            "row {row}: {} vs {expected} (bound {})",
            r.published,
            r.bound()
        );
    }
}

/// The K-D5 check of `force` with `source_frame` in the formation source and
/// `dense_frame`'s binary64 stiffness in the solve (the same frame for an
/// ordinary case; distinct only to reach a re-formation failure whose frame
/// has no binary64 stiffness).
fn checked_with(
    dense_frame: &FrameElement,
    source_frame: &FrameElement,
    force: &AssembledForce,
) -> super::super::StructuralSolution {
    let k: Matrix12 = dense_frame.global_stiffness().unwrap();
    let dense: Vec<Vec<f64>> = k.iter().map(|r| r.to_vec()).collect();
    let mut contributions = Vec::new();
    for r in 0..12 {
        for c in 0..12 {
            contributions.push(StiffnessContribution { row: r, col: c, value: k[r][c] });
        }
    }
    let source = FormationSource {
        node_count: 2,
        loads: force.load_primitives().into(),
        frames: vec![*source_frame],
        frame_members: vec![MEMBER_T.into()],
        ..FormationSource::default()
    };
    let prescribed: Vec<_> = (0..6).map(|i| (i, 0.0)).collect();
    let free: Vec<usize> = (6..12).collect();
    let checked =
        StructuralSystem::assembled(&dense, force, &free, &prescribed, Some(&contributions), None)
            .with_formation_source(&source);
    super::super::solve_formation_checked_structural_dense(&checked).unwrap()
}

/// Whether the ball (m, r) encloses num/den (den > 0): with
/// offset = den * m - num formed exactly, den * r - |offset| >= 0.
fn encloses(m: &Wide2, r: f64, num: f64, den: f64) -> bool {
    let mut offset = ExactAccumulator::new();
    assert!(!m.add_product_to(&mut offset, den).unwrap(), "split truncated");
    offset.add(-num).unwrap();
    let sign = if offset.signum() < 0 { 1.0 } else { -1.0 };
    let mut slack = ExactAccumulator::new();
    slack.add_product(den, r).unwrap();
    // slack += -|offset|, term by term (round and subtract).
    let mut rest = offset;
    for _ in 0..8 {
        if rest.is_zero() {
            break;
        }
        let head = rest.round().unwrap();
        rest.add(-head).unwrap();
        slack.add(sign * head).unwrap();
    }
    assert!(rest.is_zero(), "offset needs more than eight terms");
    slack.signum() >= 0
}

/// W-16t TPS-K (Design §4 item 4 (ii)) and element controls E1 and E9: the
/// re-formed ball of a Timoshenko proper span encloses the exact rational
/// vector (T1)/(T2). With E = G = A = J = 1, A_s = 12 and L = 1: I_y = 1
/// gives phi_y = 1, I_z = 2 gives phi_z = 2 (unequal planes). Local y (phi_z
/// = 2) over the distal half: [11/96, 7/192, 37/96, -3/64]; local z (phi_y =
/// 1): [7/64, 13/384, 25/64, -19/384], with r_y taking the moments negated.
/// The Euler–Bernoulli twin encloses the stale [3/32, 5/192, 13/32,
/// -11/192]; the full span keeps it bit for bit.
#[test]
fn kd5_timoshenko_partial_span_ball_encloses_the_rational_vector() {
    let section = FrameSection::timoshenko(1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 12.0).unwrap();
    let t = frame(0, 1, [0.0; 3], [1.0, 0.0, 0.0], section, [0.0, 1.0, 0.0]);
    assert_eq!(t.properties().unwrap().timoshenko_phi().unwrap(), Some([1.0, 2.0]));
    let load = |f: FrameElement, a: f64, b: f64| LoadFormula::UniformStraight {
        frame: f,
        intensity: IntensityPrimitive::Authored([0.0, 1.0, 1.0]),
        start_fraction: a,
        end_fraction: b,
    };
    let (m, r) = load_reformation(&primitive(load(t, 0.5, 1.0), 0, "member")).unwrap();
    let j = DOF_PER_NODE;
    for (slot, num, den) in [
        (crate::UY, 11.0, 96.0),
        (crate::RZ, 7.0, 192.0),
        (j + crate::UY, 37.0, 96.0),
        (j + crate::RZ, -3.0, 64.0),
        (crate::UZ, 7.0, 64.0),
        (crate::RY, -13.0, 384.0),
        (j + crate::UZ, 25.0, 64.0),
        (j + crate::RY, 19.0, 384.0),
        (crate::UX, 0.0, 1.0),
    ] {
        assert!(encloses(&m[slot], r[slot], num, den), "slot {slot}: not {num}/{den}");
        assert!(r[slot] <= 1e-30, "slot {slot}: radius {}", r[slot]);
    }
    // The stale Euler–Bernoulli vector is not enclosed (it is 1/64 off).
    assert!(!encloses(&m[crate::UZ], r[crate::UZ], 3.0, 32.0));
    let eb = frame(0, 1, [0.0; 3], [1.0, 0.0, 0.0], euler_bernoulli_twin(section), [0.0, 1.0, 0.0]);
    let (me, re) = load_reformation(&primitive(load(eb, 0.5, 1.0), 0, "member")).unwrap();
    for (slot, num, den) in [(crate::UY, 3.0, 32.0), (crate::RZ, 5.0, 192.0), (j + crate::UY, 13.0, 32.0), (j + crate::RZ, -11.0, 192.0)] {
        assert!(encloses(&me[slot], re[slot], num, den), "EB slot {slot}");
    }
    // A full span keeps the Euler–Bernoulli re-formation bit for bit.
    let (mf, rf) = load_reformation(&primitive(load(t, 0.0, 1.0), 0, "member")).unwrap();
    let (mfe, rfe) = load_reformation(&primitive(load(eb, 0.0, 1.0), 0, "member")).unwrap();
    for k in 0..12 {
        assert_eq!(mf[k].cmp_value(&mfe[k]), std::cmp::Ordering::Equal);
        assert_eq!(rf[k].to_bits(), rfe[k].to_bits());
    }
}

/// W-16t TPS-K (Design §4 item 4 (ii)): a closed Timoshenko cantilever with a
/// proper-span uniform load passes K-D5, and its certified recovery gives
/// the root actions of statics. w = (0, -1500, 400) N/m on [1, 2] m of a
/// 2 m cantilever: Fy = 1500 N, Fz = -400 N, Mz = 2250 N·m, My = 600 N·m.
#[test]
fn kd5_timoshenko_partial_span_closes_and_passes() {
    let t = cantilever(s_b_section(1.0));
    for (a, b, fy, fz, mz, my) in [
        (0.5, 1.0, 1500.0, -400.0, 2250.0, 600.0),
        (0.0, 0.5, 1500.0, -400.0, 750.0, 200.0),
        (0.2, 0.7, 1500.0, -400.0, 1350.0, 360.0),
    ] {
        let recipe = uniform(t, a, b);
        let force = ledger(&recipe, formed(&recipe), "member:1:T:w");
        let solution = checked_cantilever(&t, &force);
        assert_eq!(solution.report.quality, SolveQuality::Passed, "[{a}, {b}]: {:?}", solution.formation_check);
        let bound = solution.member_bound.as_ref().unwrap();
        let source = member_t_source(&t);
        let loads = CertifiedMemberLoads::from_member_primitives(0, 1, force.load_primitives(), MEMBER_T)
            .unwrap()
            .with_member_source(&source, MEMBER_T);
        let rows = certify_straight_recovery(&t, &loads, &solution.displacements, bound).unwrap();
        for (row, expected) in [
            (crate::UY, fy),
            (crate::UZ, fz),
            (crate::RZ, mz),
            (crate::RY, my),
            (6 + crate::UY, 0.0),
            (6 + crate::UZ, 0.0),
            (6 + crate::RZ, 0.0),
            (6 + crate::RY, 0.0),
        ] {
            let r = rows.ends[row];
            assert!(
                (r.published - expected).abs() <= r.bound() + 1e-9 * 3000.0,
                "[{a}, {b}] row {row}: {} vs {expected} (bound {})",
                r.published,
                r.bound()
            );
        }
    }
}

/// W-16t §2.4: the binary64 vector stale (the Euler–Bernoulli twin's values)
/// against the correct Timoshenko re-formation leaves Passed on a
/// nonsymmetric span: the closed residual measures the disagreement.
#[test]
fn kd5_timoshenko_partial_span_stale_binary64_leaves_passed() {
    let t = cantilever(s_b_section(1.0));
    let eb = cantilever(euler_bernoulli_twin(s_b_section(1.0)));
    for (a, b) in [(0.5, 1.0), (0.0, 0.5), (0.2, 0.7)] {
        let recipe = uniform(t, a, b);
        let stale = formed(&uniform(eb, a, b));
        let correct = formed(&recipe);
        assert_ne!(stale, correct, "[{a}, {b}]: precondition, the vectors differ");
        let solution = checked_cantilever(&t, &ledger(&recipe, stale, "member:1:T:w"));
        assert_ne!(solution.report.quality, SolveQuality::Passed, "[{a}, {b}]: the stale vector must not pass");
        assert!(solution.member_bound.is_none() || solution.formation_check.is_some());
    }
}

/// W-16t §5.2: TPS-K under a nonzero force-scale exponent. The scaled
/// re-formation is 2^b times the unscaled one (midpoints exactly, radii
/// rounded upward), and the primitive binds to a W2 source that holds E and G
/// times 2^b, with its exponent in the closure entry.
#[test]
fn kd5_timoshenko_partial_span_force_scaled() {
    let t = cantilever(s_b_section(1.0));
    let base = primitive(uniform(t, 0.2, 0.7), 0, "member:1:T:w");
    let (m0, r0) = load_reformation(&base).unwrap();
    for b in [-40, 40] {
        let mut scaled = base.clone();
        scaled.force_scale_exponent = b;
        let (m, r) = load_reformation(&scaled).unwrap();
        for k in 0..12 {
            assert_eq!(m[k].cmp_value(&m0[k].mul_pow2(i64::from(b)).unwrap()), std::cmp::Ordering::Equal);
            assert!(r[k] >= r0[k] * 2f64.powi(b) && r[k].is_finite());
        }
        let entry = scaled.closure_entry();
        assert_eq!(*entry.operand_bits.last().unwrap(), b as i64 as u64);
        assert_ne!(entry, base.closure_entry());
        let w2 = FormationSource {
            node_count: 2,
            loads: vec![scaled.clone()].into(),
            frames: vec![t.force_scaled(crate::ForceScale::new(b).unwrap()).unwrap()],
            frame_members: vec![MEMBER_T.into()],
            ..FormationSource::default()
        };
        assert!(unbound_straight_primitive(&w2).is_none());
    }
}

/// W-16t §5.2: a failure inside the new ball correction is
/// FormationCheckUnavailable (fail closed). G A_s l^2 below the binary64
/// range makes the "timoshenko phi" divisor ball reach zero; the same frame's
/// full span (no correction) re-forms.
#[test]
fn kd5_timoshenko_partial_span_correction_failure_is_unavailable() {
    let i = 0.0000085975 * std::f64::consts::PI;
    let broken_section = FrameSection::timoshenko(200e9, 1e-200, 0.0019, i, i, 2.0 * i, 1e-200).unwrap();
    let broken = cantilever(broken_section);
    assert!(load_reformation(&primitive(uniform(broken, 0.0, 1.0), 0, "member")).is_ok());
    let failure = load_reformation(&primitive(uniform(broken, 0.5, 1.0), 0, "member")).unwrap_err();
    assert!(
        matches!(failure, LoadFailure::DivisorReachesZero { site: "timoshenko phi" }),
        "{failure:?}"
    );
    // The solve's binary64 stiffness is an ordinary frame's; the source frame
    // and the primitives are the broken one's, so the closure binds and the
    // re-formation fails inside the correction.
    let t = cantilever(s_b_section(1.0));
    let recipe = uniform(broken, 0.5, 1.0);
    let force = ledger(&recipe, formed(&uniform(t, 0.5, 1.0)), "member:1:T:w");
    let solution = checked_with(&t, &broken, &force);
    assert_eq!(solution.report.quality, SolveQuality::Sensitive);
    assert!(solution.member_bound.is_none());
    let detail = unavailable_detail(&solution);
    assert!(detail.starts_with("load closure:") && detail.contains("timoshenko phi"), "{detail}");
}

/// Design §4 item 2 and §8 ("Timoshenko leaf missing from a recovery or
/// primitive frame"): straight primitive frames and certified-recovery frames
/// are bound to the FormationSource frame of the same member, leaf included;
/// a frame without the leaf, or with a different leaf, fails closed.
#[test]
fn kd5_straight_frames_are_bound_to_the_formation_source() {
    let t = cantilever(s_b_section(1.0));
    let other = cantilever(s_b_section(0.5));
    let eb = cantilever(euler_bernoulli_twin(s_b_section(1.0)));
    // The control closes and passes.
    let recipe = uniform(t, 0.0, 1.0);
    let control = ledger(&recipe, formed(&recipe), "member:1:T:w");
    assert_eq!(checked_cantilever(&t, &control).report.quality, SolveQuality::Passed);
    // A producer that drops or changes the leaf in its primitives (and in its
    // own manifest, so the closure itself matches) fails closed by name.
    for mutant in [eb, other] {
        let wrong = uniform(mutant, 0.0, 1.0);
        let force = ledger(&wrong, formed(&recipe), "member:1:T:w");
        let solution = checked_cantilever(&t, &force);
        assert_eq!(solution.report.quality, SolveQuality::Sensitive);
        assert!(solution.member_bound.is_none());
        let detail = unavailable_detail(&solution);
        assert!(detail.contains("straight primitive frame is not its formation-source frame"), "{detail}");
    }
    // Every straight family: free strain and straight pressure operands too.
    let strain = EigenstrainPrimitive {
        thermal: ThermalPrimitive::ExplicitIntervalStrain { strain: 1e-4 },
        fit: FitPrimitive::None,
    };
    let pressure = |frame: FrameElement| LoadFormula::PressureOperand {
        geometry: PressureGeometry::Straight { frame },
        pressure: 1e6,
        outer_diameter: 0.2,
        wall_thickness: 0.01,
        coefficient: PressureCoefficient::Constant(1.0),
        direction: PressureDirection::Chord { axis: 0 },
    };
    let free_strain = |frame: FrameElement| LoadFormula::FreeStrainStraight {
        frame,
        strain: strain.clone(),
    };
    // Each frame's member identity, parallel to the frames.
    let source = |loads: Vec<LoadPrimitive>, frames: Vec<FrameElement>, members: &[&str]| FormationSource {
        node_count: 2,
        loads: loads.into(),
        frames,
        frame_members: members.iter().map(|m| m.to_string()).collect(),
        ..FormationSource::default()
    };
    const M: &str = "member:1:m:";
    const OWN: &str = "member:1:m:load";
    for family in [&pressure as &dyn Fn(FrameElement) -> LoadFormula, &free_strain, &|f| uniform(f, 0.0, 1.0)] {
        assert!(unbound_straight_primitive(&source(vec![primitive(family(t), 0, OWN)], vec![t], &[M])).is_none());
        for mutant in [eb, other] {
            assert!(unbound_straight_primitive(&source(vec![primitive(family(mutant), 0, OWN)], vec![t], &[M])).is_some());
        }
        // No frame on the pair (a connector-replaced span): nothing to mix.
        assert!(unbound_straight_primitive(&source(vec![primitive(family(eb), 0, OWN)], vec![], &[])).is_none());
        // Keyed by member identity: without identities, with an empty one, or
        // beside another member's frame, a straight primitive fails closed.
        assert!(unbound_straight_primitive(&source(vec![primitive(family(t), 0, OWN)], vec![t], &[])).is_some());
        assert!(unbound_straight_primitive(&source(vec![primitive(family(t), 0, OWN)], vec![t], &[""])).is_some());
        assert!(unbound_straight_primitive(&source(vec![primitive(family(t), 0, OWN)], vec![t], &["member:1:n:"])).is_some());
    }
    // Reversed nodes are another frame.
    let reversed = frame(1, 0, [2.0, 0.0, 0.0], [0.0; 3], s_b_section(1.0), [0.0, 1.0, 0.0]);
    assert!(unbound_straight_primitive(&source(vec![primitive(uniform(reversed, 0.0, 1.0), 0, OWN)], vec![t], &[M])).is_some());
    // W2: the source holds E and G times 2^b; the primitive its leaves and b.
    for b in [-40, 40] {
        let scale = crate::ForceScale::new(b).unwrap();
        let mut bound = primitive(uniform(t, 0.0, 1.0), 0, OWN);
        bound.force_scale_exponent = b;
        assert!(unbound_straight_primitive(&source(vec![bound.clone()], vec![t.force_scaled(scale).unwrap()], &[M])).is_none());
        assert!(unbound_straight_primitive(&source(vec![bound.clone()], vec![t], &[M])).is_some());
        let mut wrong = bound;
        wrong.formula = uniform(eb, 0.0, 1.0);
        assert!(unbound_straight_primitive(&source(vec![wrong], vec![t.force_scaled(scale).unwrap()], &[M])).is_some());
    }
    // Parallel members on one node pair are admitted: each primitive binds
    // to its own member's frame, not to the other member's; a third leaf
    // binds to neither.
    let parallel = vec![t, other];
    let members = ["member:1:A:", "member:1:B:"];
    for (f, own) in [(t, "member:1:A:w"), (other, "member:1:B:w")] {
        assert!(unbound_straight_primitive(&source(vec![primitive(uniform(f, 0.0, 1.0), 0, own)], parallel.clone(), &members)).is_none());
    }
    for (f, swapped) in [(other, "member:1:A:w"), (t, "member:1:B:w")] {
        assert!(unbound_straight_primitive(&source(vec![primitive(uniform(f, 0.0, 1.0), 0, swapped)], parallel.clone(), &members)).is_some());
    }
    let third = cantilever(s_b_section(0.25));
    assert!(unbound_straight_primitive(&source(vec![primitive(uniform(third, 0.0, 1.0), 0, "member:1:A:w")], parallel.clone(), &members)).is_some());

    // Certified recovery, keyed by member identity: the recovery frame equals
    // its own member's formation-source frame and its member's primitive
    // frames, leaf included; member C has no load primitive.
    let mut primitives = Vec::new();
    for (prefix, f) in [("member:1:A:w", t), ("member:1:B:w", other)] {
        for row in 0..12 {
            let mut p = primitive(uniform(f, 0.0, 1.0), primitives.len(), prefix);
            p.local_row = row;
            primitives.push(p);
        }
    }
    let bound = MemberBoundInputs {
        w: vec![0.0; 12],
        w2: vec![0.0; 12],
        exponents: vec![i32::MIN; 12],
        lambda_sigma: 0.0,
    };
    let u = [0.0; 12];
    let k_source = source(primitives.clone(), vec![t, other, t], &["member:1:A:", "member:1:B:", "member:1:C:"]);
    let loads = |member: &'static str| {
        CertifiedMemberLoads::from_member_primitives(0, 1, &primitives, member).unwrap().with_member_source(&k_source, member)
    };
    let (loads_a, loads_b, loads_c) = (loads("member:1:A:"), loads("member:1:B:"), loads("member:1:C:"));
    assert!(certify_straight_recovery(&t, &loads_a, &u, &bound).is_ok());
    assert!(certify_straight_recovery(&other, &loads_b, &u, &bound).is_ok());
    assert!(certify_straight_recovery(&t, &loads_c, &u, &bound).is_ok());
    let unbound = CertifiedMemberLoads::from_member_primitives(0, 1, &primitives, "member:1:A:").unwrap();
    let absent = CertifiedMemberLoads::from_member_primitives(0, 1, &primitives, "member:1:D:")
        .unwrap()
        .with_member_source(&k_source, "member:1:D:");
    for (wrong, loads) in [(&eb, &loads_a), (&other, &loads_a), (&t, &loads_b), (&eb, &loads_c), (&other, &loads_c), (&t, &unbound), (&t, &absent)] {
        assert!(certify_straight_recovery(wrong, loads, &u, &bound).is_err());
        assert!(certify_straight_section(wrong, loads, &u, &bound, 0.5).is_err());
        assert!(certify_straight_stress_spans(wrong, loads, &u, &bound, 1.0, 1.0, None).is_err());
    }
    // A support reaction's straight primitives are bound to the source too.
    let force = ledger(&recipe, formed(&recipe), "member:1:T:w");
    let good = source(force.load_primitives().to_vec(), vec![t], &[MEMBER_T]);
    assert!(certify_support_reaction(&good, &force, 1, &u, &bound).is_ok());
    let drifted = ledger(&uniform(eb, 0.0, 1.0), formed(&recipe), "member:1:T:w");
    let bad = source(drifted.load_primitives().to_vec(), vec![t], &[MEMBER_T]);
    assert!(certify_support_reaction(&bad, &drifted, 1, &u, &bound).is_err());
    let anonymous = source(force.load_primitives().to_vec(), vec![t], &[]);
    assert!(certify_support_reaction(&anonymous, &force, 1, &u, &bound).is_err());
}

use super::super::certified_recovery::{
    certify_straight_recovery, certify_straight_section, certify_straight_stress_spans,
    certify_support_reaction, CertifiedMemberLoads,
};
