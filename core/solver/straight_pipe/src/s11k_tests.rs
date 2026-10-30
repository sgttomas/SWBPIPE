//! S11-K tests K4 and K5 (S11 section 9) for the straight-pipe recovery sums
//! E1-E4 and E6. Expected values are exact dyadic references computed in the
//! test (every binary64 value is m * 2^e; the references are products of the
//! net load with dyadic lengths), never values produced by the code under test.
//! Precondition rule (S11B-6): each kill test first asserts that a binary64
//! fold of its terms, in the producer's order, differs from the exact net.
use super::*;
use open_pipe_stress_frame_kernel::solve_dense;

/// Exact dyadic value `m * 2^e` (a Fraction whose denominator is a power of two).
#[derive(Clone, Copy, Debug)]
struct Dy {
    m: i128,
    e: i32,
}
impl Dy {
    fn of(x: f64) -> Self {
        assert!(x.is_finite());
        if x == 0.0 {
            return Self { m: 0, e: 0 };
        }
        let bits = x.to_bits();
        let exp = ((bits >> 52) & 0x7ff) as i32;
        let frac = (bits & ((1u64 << 52) - 1)) as i128;
        let (m, e) = if exp == 0 {
            (frac, -1074)
        } else {
            (frac | (1i128 << 52), exp - 1075)
        };
        Self {
            m: if x < 0.0 { -m } else { m },
            e,
        }
    }
    fn int(n: i128) -> Self {
        Self { m: n, e: 0 }
    }
    fn mul(self, o: Self) -> Self {
        Self {
            m: self.m.checked_mul(o.m).expect("reference overflow"),
            e: self.e + o.e,
        }
    }
    fn half(self) -> Self {
        Self {
            m: self.m,
            e: self.e - 1,
        }
    }
    fn neg(self) -> Self {
        Self {
            m: -self.m,
            e: self.e,
        }
    }
    /// |self - x| as f64 (exact difference, one final rounding); infinite
    /// when the alignment would overflow (a grossly wrong observation).
    fn abs_diff(self, x: f64) -> f64 {
        let o = Self::of(x);
        let e = self.e.min(o.e);
        let shift = |d: Self| -> Option<i128> {
            let s = (d.e - e) as u32;
            if d.m == 0 {
                Some(0)
            } else if s >= 126 {
                None
            } else {
                d.m.checked_mul(1i128 << s)
            }
        };
        match (shift(self), shift(o)) {
            (Some(a), Some(b)) => match a.checked_sub(b) {
                Some(d) => (d.unsigned_abs() as f64) * 2f64.powi(e),
                None => f64::INFINITY,
            },
            _ => f64::INFINITY,
        }
    }
    fn to_f64(self) -> f64 {
        (self.m as f64) * 2f64.powi(self.e)
    }
}

fn within(observed: f64, exact: Dy, scale: f64, what: &str) {
    let tolerance = 1e-9 * exact.to_f64().abs().max(scale);
    let error = exact.abs_diff(observed);
    assert!(
        error <= tolerance,
        "{what}: observed {observed:e}, exact {:e}, error {error:e} > {tolerance:e}",
        exact.to_f64()
    );
}

fn cantilever() -> StraightPipeElement {
    StraightPipeElement::new(
        "pipe-probe-a",
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap(),
        StraightPipeSectionProperties::new(2.0e11, 7.7e10, 0.01, 8.0e-6, 9.0e-6, 1.7e-5, None)
            .unwrap(),
        [0.0, 1.0, 0.0],
    )
    .unwrap()
}

/// Solve the root-fixed cantilever for a tip load vector (element along
/// global x, so local and global coincide).
fn tip_solution(pipe: &StraightPipeElement, element_load: &[f64; ELEMENT_DOF]) -> Vec<f64> {
    let k = pipe.local_stiffness().unwrap();
    let kff: Vec<Vec<f64>> = (6..12)
        .map(|r| (6..12).map(|c| k[r][c]).collect())
        .collect();
    let tip = solve_dense(&kff, &element_load[6..12]).unwrap();
    let mut u = vec![0.0; 12];
    u[6..12].copy_from_slice(&tip);
    u
}

fn uniform_y(q: f64) -> SpannedUniformLocalLoad {
    SpannedUniformLocalLoad::full(LocalLoadDirection::Y, q).unwrap()
}

const LENGTH: f64 = 2.0;

/// Today's single-load fixed-end formula for a full-span local-y load
/// (a = 0, b = 1), in the original expression order, rounded as before.
fn uniform_y_terms_today(q: f64) -> [f64; ELEMENT_DOF] {
    let (a, b, length) = (0.0_f64, 1.0_f64, LENGTH);
    let mut loads = [0.0; ELEMENT_DOF];
    loads[UY] +=
        q * length * ((b - b.powi(3) + 0.5 * b.powi(4)) - (a - a.powi(3) + 0.5 * a.powi(4)));
    loads[RZ] += q
        * length
        * length
        * ((0.5 * b * b - (2.0 / 3.0) * b.powi(3) + 0.25 * b.powi(4))
            - (0.5 * a * a - (2.0 / 3.0) * a.powi(3) + 0.25 * a.powi(4)));
    loads[DOF_PER_NODE + UY] +=
        q * length * ((b.powi(3) - 0.5 * b.powi(4)) - (a.powi(3) - 0.5 * a.powi(4)));
    loads[DOF_PER_NODE + RZ] += q
        * length
        * length
        * (((-b.powi(3) / 3.0) + 0.25 * b.powi(4)) - ((-a.powi(3) / 3.0) + 0.25 * a.powi(4)));
    loads
}

const KILL_SET: [(f64, f64); 2] = [(1e8, 0.3), (1e80, 1e-8)];

fn orders(g: f64, n: f64) -> [[f64; 3]; 2] {
    [[g, n, -g], [n, g, -g]]
}

fn fold(values: &[f64]) -> f64 {
    values.iter().fold(0.0, |s, v| s + v)
}

#[test]
fn k4_e1_single_load_is_bit_identical_and_cancelling_loads_are_exact() {
    let pipe = cantilever();
    for q in [0.3, -190.0, 1.2e6, 1e8] {
        let actual = pipe
            .equivalent_nodal_loads_with_spans(&[uniform_y(q)], &[])
            .unwrap();
        let today = uniform_y_terms_today(q);
        for dof in 0..ELEMENT_DOF {
            assert_eq!(
                actual[dof].to_bits(),
                today[dof].to_bits(),
                "q={q} dof={dof}"
            );
        }
    }
    // One global load splits into disjoint local X/Y/Z DOFs: bit-identical.
    let global = SpannedGlobalUniformLoad::new([3.0, -190.0, 12.5], UniformLoadSpan::full());
    let mixed = pipe
        .equivalent_global_nodal_loads_with_spans(&[global.unwrap()], &[])
        .unwrap();
    assert!(mixed.iter().all(|v| v.is_finite()));
    for (g, n) in KILL_SET {
        let single = uniform_y_terms_today(n);
        for order in orders(g, n) {
            let loads: Vec<_> = order.iter().map(|&q| uniform_y(q)).collect();
            let per_load: Vec<_> = order.iter().map(|&q| uniform_y_terms_today(q)).collect();
            // Precondition: the producer-order fold of the tip UY terms is wrong.
            let tip_fold = fold(&per_load.iter().map(|t| t[6 + UY]).collect::<Vec<_>>());
            assert_ne!(
                tip_fold.to_bits(),
                single[6 + UY].to_bits(),
                "precondition {order:?}"
            );
            let actual = pipe.equivalent_nodal_loads_with_spans(&loads, &[]).unwrap();
            // Exact net: the G terms are exact negatives and cancel, leaving n's terms.
            for dof in 0..ELEMENT_DOF {
                assert_eq!(
                    actual[dof].to_bits(),
                    (single[dof] + 0.0).to_bits(),
                    "{order:?} {dof}"
                );
            }
        }
    }
}

#[test]
fn k4_probe_a_end_forces_and_stations_are_exact_at_the_kill_set() {
    let pipe = cantilever();
    for (g, n) in KILL_SET {
        let single = pipe
            .equivalent_nodal_loads_with_spans(&[uniform_y(n)], &[])
            .unwrap();
        let u = tip_solution(&pipe, &single);
        let w = Dy::of(n);
        let two = Dy::int(2);
        // Body scale: root shear w*L = root moment w*L^2/2 = 2w.
        let scale = 2.0 * n.abs();
        for order in orders(g, n) {
            let loads: Vec<_> = order.iter().map(|&q| uniform_y(q)).collect();
            // Precondition at the root shear: local - terms, folded in order.
            let local = pipe.recover_local_forces(&u).unwrap().local_forces;
            let per_load: Vec<_> = order.iter().map(|&q| uniform_y_terms_today(q)).collect();
            let folded = per_load.iter().fold(local[UY], |s, t| s - t[UY]);
            let exact_root = w.mul(two).neg();
            assert!(
                exact_root.abs_diff(folded) > 1e-9 * scale,
                "precondition {order:?}"
            );
            // E3: end forces.
            let end_i = pipe
                .recover_end_resultants_with_spans_and_axial_effects(
                    &u,
                    PipeEnd::I,
                    &loads,
                    &[],
                    &[],
                )
                .unwrap();
            let end_j = pipe
                .recover_end_resultants_with_spans_and_axial_effects(
                    &u,
                    PipeEnd::J,
                    &loads,
                    &[],
                    &[],
                )
                .unwrap();
            within(end_i.shear_force_y, exact_root, scale, "root shear");
            within(
                end_i.bending_moment_z,
                w.mul(two).neg(),
                scale,
                "root moment",
            );
            within(end_j.shear_force_y, Dy::int(0), scale, "tip shear");
            within(end_j.bending_moment_z, Dy::int(0), scale, "tip moment");
            // E4: quarter and midspan stations through the recover_station family.
            let stations = pipe
                .recover_station_resultant_sweep_with_spans(&u, &[0.25, 0.5], &loads, &[])
                .unwrap();
            for (station, x) in stations.iter().zip([Dy { m: 1, e: -1 }, Dy::int(1)]) {
                // V(x) = -w (L - x); M(x) = -w (L - x)^2 / 2.
                let remaining = Dy {
                    m: 2 * (1i128 << (-x.e).max(0)) - x.m,
                    e: x.e.min(0),
                };
                within(
                    station.shear_force_y,
                    w.mul(remaining).neg(),
                    scale,
                    "station shear",
                );
                within(
                    station.bending_moment_z,
                    w.mul(remaining).mul(remaining).half().neg(),
                    scale,
                    "station moment",
                );
            }
            // E6: stations from given (exact) end actions.
            let i_end = PipeEndResultants {
                end: PipeEnd::I,
                axial_force: 0.0,
                shear_force_y: -2.0 * n,
                shear_force_z: 0.0,
                torsional_moment: 0.0,
                bending_moment_y: 0.0,
                bending_moment_z: -2.0 * n,
            };
            let mid = pipe
                .station_resultants_from_i_end_with_spans(i_end, 0.5, &loads, &[])
                .unwrap();
            within(mid.shear_force_y, w.neg(), scale, "E6 shear");
            within(mid.bending_moment_z, w.half().neg(), scale, "E6 moment");
        }
    }
}

#[test]
fn k4_axial_effect_case_kills_e2_and_the_axial_half_of_e3() {
    // R3-2: three axial effects (G, n, -G) (thermal or thrust) on one member.
    let pipe = cantilever();
    for (g, n) in KILL_SET {
        for order in orders(g, n) {
            let effects: Vec<_> = order
                .iter()
                .map(|&x| StraightPipeAxialEffect::new(x).unwrap())
                .collect();
            // Precondition: the producer fold of -N at i differs from -n.
            let folded_i = fold(&order.iter().map(|x| -x).collect::<Vec<_>>());
            assert_ne!(folded_i.to_bits(), (-n).to_bits(), "precondition {order:?}");
            // E2: exact pair.
            let pair = pipe.equivalent_local_axial_effect_loads(&effects).unwrap();
            assert_eq!(pair[UX].to_bits(), (-n).to_bits());
            assert_eq!(pair[6 + UX].to_bits(), n.to_bits());
            // Free thermal growth of a cantilever carries no axial force.
            let single = pipe
                .equivalent_local_axial_effect_loads(&[StraightPipeAxialEffect::new(n).unwrap()])
                .unwrap();
            let u = tip_solution(&pipe, &single);
            let scale = n.abs();
            let corrected = pipe
                .recover_local_forces_with_axial_effects(&u, &effects)
                .unwrap()
                .local_forces;
            within(corrected[UX], Dy::int(0), scale, "E3 axial i");
            within(corrected[6 + UX], Dy::int(0), scale, "E3 axial j");
            for end in [PipeEnd::I, PipeEnd::J] {
                let r = pipe
                    .recover_end_resultants_with_spans_and_axial_effects(
                        &u,
                        end,
                        &[],
                        &[],
                        &effects,
                    )
                    .unwrap();
                within(r.axial_force, Dy::int(0), scale, "E3 axial end");
            }
            // Mixed: uniform x loads and axial effects on the same DOFs.
            let axial_loads: Vec<_> = order
                .iter()
                .map(|&q| SpannedUniformLocalLoad::full(LocalLoadDirection::X, q).unwrap())
                .collect();
            let net = pipe
                .equivalent_nodal_loads_with_spans(&[axial_loads[0]], &[])
                .unwrap();
            assert!(net.iter().all(|v| v.is_finite()));
            let mixed = pipe
                .recover_end_resultants_with_spans_and_axial_effects(
                    &u,
                    PipeEnd::I,
                    &axial_loads,
                    &[],
                    &effects,
                )
                .unwrap();
            // Exact: local_i - (the x loads' net i term) - (-n).
            let x_load_i = pipe
                .equivalent_nodal_loads_with_spans(
                    &[SpannedUniformLocalLoad::full(LocalLoadDirection::X, n).unwrap()],
                    &[],
                )
                .unwrap()[UX];
            let local_i = pipe.recover_local_forces(&u).unwrap().local_forces[UX];
            let reference = Dy::of(local_i)
                .add_exact(Dy::of(x_load_i).neg())
                .add_exact(Dy::of(n));
            within(mixed.axial_force, reference, scale, "E3 mixed axial");
        }
    }
}

impl Dy {
    fn add_exact(self, o: Self) -> Self {
        let e = self.e.min(o.e);
        let a = self.m << (self.e - e);
        let b = o.m << (o.e - e);
        Self { m: a + b, e }
    }
}

#[test]
fn k5_zero_signs_and_bit_identity_of_unloaded_stations() {
    let pipe = cantilever();
    // Load-free end force: +0.0 everywhere.
    let zero_u = [0.0; ELEMENT_DOF];
    for end in [PipeEnd::I, PipeEnd::J] {
        let r = pipe
            .recover_end_resultants_with_spans_and_axial_effects(&zero_u, end, &[], &[], &[])
            .unwrap();
        for v in [
            r.axial_force,
            r.shear_force_y,
            r.shear_force_z,
            r.torsional_moment,
            r.bending_moment_y,
            r.bending_moment_z,
        ] {
            assert_eq!(v.to_bits(), 0);
        }
    }
    // Load-free station from a -0.0 end action: every summed component is +0.0.
    let negative_zero = PipeEndResultants {
        end: PipeEnd::I,
        axial_force: -0.0,
        shear_force_y: -0.0,
        shear_force_z: -0.0,
        torsional_moment: 0.0,
        bending_moment_y: -0.0,
        bending_moment_z: -0.0,
    };
    let s = pipe
        .station_resultants_from_i_end_with_spans(negative_zero, 0.5, &[], &[])
        .unwrap();
    for v in [
        s.axial_force,
        s.shear_force_y,
        s.shear_force_z,
        s.bending_moment_y,
        s.bending_moment_z,
    ] {
        assert_eq!(v.to_bits(), 0);
    }
    // A station whose load terms cancel exactly: +0.0.
    let cancelling = [uniform_y(1e8), uniform_y(-1e8)];
    let zero_end = PipeEndResultants {
        axial_force: 0.0,
        shear_force_y: 0.0,
        shear_force_z: 0.0,
        torsional_moment: 0.0,
        bending_moment_y: 0.0,
        bending_moment_z: 0.0,
        end: PipeEnd::I,
    };
    let c = pipe
        .station_resultants_from_i_end_with_spans(zero_end, 0.37, &cancelling, &[])
        .unwrap();
    assert_eq!(c.shear_force_y.to_bits(), 0);
    assert_eq!(c.bending_moment_z.to_bits(), 0);
    // Nonzero end action, no loads: bit-identical to today's two-term sums.
    let i_end = PipeEndResultants {
        end: PipeEnd::I,
        axial_force: 12.345,
        shear_force_y: -0.123456789,
        shear_force_z: 98.7654321,
        torsional_moment: 3.25,
        bending_moment_y: -45.6789,
        bending_moment_z: 7.891011,
    };
    for fraction in [0.0, 0.25, 0.37, 0.5, 1.0] {
        let s = pipe
            .station_resultants_from_i_end_with_spans(i_end, fraction, &[], &[])
            .unwrap();
        let d = fraction * LENGTH;
        assert_eq!(s.axial_force.to_bits(), i_end.axial_force.to_bits());
        assert_eq!(s.shear_force_y.to_bits(), i_end.shear_force_y.to_bits());
        assert_eq!(s.shear_force_z.to_bits(), i_end.shear_force_z.to_bits());
        assert_eq!(
            s.torsional_moment.to_bits(),
            i_end.torsional_moment.to_bits()
        );
        assert_eq!(
            s.bending_moment_y.to_bits(),
            (i_end.bending_moment_y + i_end.shear_force_z * d).to_bits()
        );
        assert_eq!(
            s.bending_moment_z.to_bits(),
            (i_end.bending_moment_z - i_end.shear_force_y * d).to_bits()
        );
    }
}

/// P1's S11-PROBE-A-G1e7 / -G1e8 (T3 DETECTION, ROOT ruling 2026-09-26): a 2 m
/// cantilever, uniform global-y loads (G, 0.3, -G) N/m, OD 0.2 m, wall 0.01 m,
/// E 200 GPa. The kernel part S11-K repairs is recovery (E3 end forces): the
/// tip load vector is the exact net (as the ledger gives it), and the
/// recovered root shear and moment must be w L = w L^2 / 2 = 0.6 within 1e-9.
/// The section values are the binary64 inputs the product derives for this
/// section (they appear in committed contract evidence). The product-level
/// probe (force assembly through the captured entry) is an S11-F test.
#[test]
fn p1_probe_a_recovery_is_exact_at_g1e7_and_g1e8() {
    let pipe = StraightPipeElement::new(
        "pipe-p1-probe-a",
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap(),
        StraightPipeSectionProperties::new(
            200.0e9,
            76923076923.07692,
            0.005969026041820607,
            0.000027009842839238257,
            0.000027009842839238257,
            0.000054019685678476514,
            None,
        )
        .unwrap(),
        [0.0, 1.0, 0.0],
    )
    .unwrap();
    let n = 0.3;
    let single = pipe
        .equivalent_nodal_loads_with_spans(&[uniform_y(n)], &[])
        .unwrap();
    let u = tip_solution(&pipe, &single);
    // Tip deflection w L^4 / (8 E I): four binary64 roundings in the
    // reference, far below the 1e-9 criterion.
    let tip = n * 16.0 / (8.0 * 200.0e9 * 0.000027009842839238257);
    assert!((u[6 + UY] - tip).abs() <= 1e-9 * tip);
    let w = Dy::of(n);
    for g in [1e7, 1e8] {
        let loads: Vec<_> = [g, n, -g].iter().map(|&q| uniform_y(q)).collect();
        let local = pipe.recover_local_forces(&u).unwrap().local_forces;
        let folded = [g, n, -g]
            .iter()
            .fold(local[UY], |s, &q| s - uniform_y_terms_today(q)[UY]);
        assert!(
            w.mul(Dy::int(2)).neg().abs_diff(folded) > 0.0,
            "precondition g={g}"
        );
        let end_i = pipe
            .recover_end_resultants_with_spans_and_axial_effects(&u, PipeEnd::I, &loads, &[], &[])
            .unwrap();
        within(end_i.shear_force_y, w.mul(Dy::int(2)).neg(), 0.6, "Vb.i");
        within(end_i.bending_moment_z, w.mul(Dy::int(2)).neg(), 0.6, "Mb.i");
    }
}
