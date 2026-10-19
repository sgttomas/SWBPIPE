//! T4-U5b: the explicit elastic tie rod element (DEL-04-07 Design
//! `u4b-u5b-implementation.md` §3.2) at kernel level.
//!
//! - All six infinitesimal rigid modes give C·d = 0 to the formation bound on
//!   generic skew rods (skew Q, offsets at both ends, UTM coordinates); the
//!   value of C·d is evaluated exactly.
//! - Cᵀk C, +Cᵀk e_ref and T = k(C d − e_ref) against exact rational values
//!   (n = (2, 3, 6)/7), and the reference-load bound contains the exact value.
//! - The t4_u5b element-level checks (validation/references/t4_u5b
//!   `element_controls`): a ±a pair adds K_xx = 2k and K_rz = 2k·a² with
//!   cancelling cross terms, one rod at +a adds K_x,rz = −k·a, four rods add
//!   both bending rotations and no torsion, C = c·B for zero connector
//!   offsets, Bᵀ(Σ k c cᵀ)B = Σ CᵀkC, and the end-i arm-sign control's
//!   extensions under unit rigid rotations (16/35, −18/35, −4/7).
//! - Assembly: dense and sparse agree bit for bit with rods, K2b scales the
//!   rods' Ke exactly, the census takes k and Ke, and a connector without
//!   rods is unchanged bit for bit.
//!
//! Invented inputs only.
use open_pipe_stress_frame_kernel::connector::{
    ConnectorAttachment, ObjectiveConnector, ScaledWorkMatrix,
};
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_frame_kernel::structural::{assemble_sparse_stiffness, SparseAssemblyOptions};
use open_pipe_stress_frame_kernel::tie_rod::{TieRod, TieRodError, TieRodInput};
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness_with_connectors, ForceScale, ForceScaleCensus, FrameElement,
    FrameNode, FrameSection, Matrix12, Matrix3,
};

const U: f64 = f64::EPSILON / 2.0;
const IDENTITY: Matrix3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

fn node(index: usize, x: [f64; 3]) -> FrameNode {
    FrameNode::new(index, x).unwrap()
}

/// A diagonal-H connector with zero attachment offsets, Ls = 1.
fn connector(
    ni: FrameNode,
    nj: FrameNode,
    axes: Matrix3,
    diagonal: [f64; 6],
) -> ObjectiveConnector {
    let mut upper = [0.0; 21];
    let mut at = 0;
    for r in 0..6 {
        for c in r..6 {
            if r == c {
                upper[at] = diagonal[r];
            }
            at += 1;
        }
    }
    with_upper(ni, nj, axes, upper)
}

fn with_upper(ni: FrameNode, nj: FrameNode, axes: Matrix3, upper: [f64; 21]) -> ObjectiveConnector {
    ObjectiveConnector::new(
        ni,
        nj,
        ConnectorAttachment::global([0.0; 3]),
        ConnectorAttachment::global([0.0; 3]),
        axes,
        ScaledWorkMatrix {
            upper_triangle: upper,
            translation_scale: 1.0,
        },
        [0.0; 6],
    )
    .unwrap()
}

/// The index of (r, c), r ≤ c, in the 21-entry upper triangle.
fn upper_index(r: usize, c: usize) -> usize {
    (0..r).map(|k| 6 - k).sum::<usize>() + (c - r)
}

fn rod(
    offset_i: [f64; 3],
    offset_j: [f64; 3],
    stiffness: f64,
    reference_extension: f64,
) -> TieRodInput {
    TieRodInput {
        offset_i,
        offset_j,
        stiffness,
        reference_extension,
    }
}

/// e_k × v (exact: signed components of v).
fn unit_cross(k: usize, v: [f64; 3]) -> [f64; 3] {
    match k {
        0 => [0.0, -v[2], v[1]],
        1 => [v[2], 0.0, -v[0]],
        _ => [-v[1], v[0], 0.0],
    }
}

/// The exact value of C·d for rigid mode `mode` (0..3: unit translation
/// along e_mode; 3..6: unit rotation about e_(mode-3) through `origin`):
/// u = θ × (x − origin) enters as θ × x − θ × origin, so every product is an
/// exact product of binary64 values. Rounded once for the comparison.
fn rigid_extension(
    c: &[f64; 12],
    xi: [f64; 3],
    xj: [f64; 3],
    origin: [f64; 3],
    mode: usize,
) -> f64 {
    let mut sum = ExactAccumulator::new();
    if mode < 3 {
        sum.add(c[mode]).unwrap();
        sum.add(c[6 + mode]).unwrap();
    } else {
        let k = mode - 3;
        for (block, x) in [(0, xi), (6, xj)] {
            let ux = unit_cross(k, x);
            let uo = unit_cross(k, origin);
            for m in 0..3 {
                sum.add_product(c[block + m], ux[m]).unwrap();
                sum.add_product(-c[block + m], uo[m]).unwrap();
            }
            sum.add(c[block + 3 + k]).unwrap();
        }
    }
    sum.round().unwrap()
}

fn norm1(v: [f64; 3]) -> f64 {
    v.iter().map(|x| x.abs()).sum()
}

/// A proper rotation from a unit axis and an angle (Rodrigues), row-major
/// with the axes as columns, in binary64 (orthonormal to roundoff).
fn rotation(axis: [f64; 3], angle: f64) -> Matrix3 {
    let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let (x, y, z) = (axis[0] / n, axis[1] / n, axis[2] / n);
    let (s, c) = angle.sin_cos();
    let t = 1.0 - c;
    [
        [t * x * x + c, t * x * y - s * z, t * x * z + s * y],
        [t * x * y + s * z, t * y * y + c, t * y * z - s * x],
        [t * x * z - s * y, t * y * z + s * x, t * z * z + c],
    ]
}

/// Generic skew connectors: a skew Q, x_j − x_i along Q.x, at the origin and
/// at UTM-like coordinates, each with generic rods offset at both ends.
fn skew_connectors() -> Vec<ObjectiveConnector> {
    let (third, two_thirds) = (1.0 / 3.0, 2.0 / 3.0);
    let triad = [
        [third, two_thirds, -two_thirds],
        [two_thirds, third, two_thirds],
        [two_thirds, -two_thirds, -third],
    ];
    let mut out = Vec::new();
    for (axes, length) in [(triad, 0.45), (rotation([0.3, -0.7, 0.5], 1.1), 0.62)] {
        for base in [
            [0.0, 0.0, 0.0],
            [512_345.25, 6_123_456.5, 87.125],
            [-3.7, 2.9, 1.3],
        ] {
            let xj = [
                base[0] + length * axes[0][0],
                base[1] + length * axes[1][0],
                base[2] + length * axes[2][0],
            ];
            let c = connector(
                node(0, base),
                node(1, xj),
                axes,
                [5e6, 4e6, 3e6, 2e3, 1e3, 1.5e3],
            )
            .with_tie_rods(&[
                rod([-0.05, 0.13, -0.07], [0.06, -0.11, 0.09], 2.5e7, -1.25e-3),
                rod([0.0, -0.09, 0.08], [0.11, 0.1, -0.03], 7.0e6, 0.0),
                rod([0.21, 0.0, 0.0], [-0.17, 0.02, 0.0], 1.0e5, 4.0e-4),
            ])
            .unwrap();
            out.push(c);
        }
    }
    out
}

#[test]
fn all_six_rigid_modes_vanish_to_the_formation_bound_on_generic_skew_rods() {
    let origin = [7.0, -3.0, 5.0];
    let mut nonzero = 0;
    for c in skew_connectors() {
        for rod in c.tie_rods() {
            let coefficients = rod.c();
            let (xi, xj) = (rod.node_i().coordinates, rod.node_j().coordinates);
            let (oi, oj) = rod.offsets();
            let dx = [xj[0] - xi[0], xj[1] - xi[1], xj[2] - xi[2]];
            let bound = 16.0 * U * (norm1(dx) + norm1(oi) + norm1(oj));
            for mode in 0..6 {
                let e = rigid_extension(&coefficients, xi, xj, origin, mode);
                if mode < 3 {
                    assert_eq!(e, 0.0, "translation {mode} is exactly null");
                } else {
                    assert!(
                        e.abs() <= bound,
                        "rotation {mode}: |C d| = {e:e} > {bound:e}"
                    );
                    nonzero += usize::from(e != 0.0);
                }
                // The assembled element: Ke·d = k C (C d) is null to the same order.
            }
            let ke = rod.global_stiffness().unwrap();
            for a in 0..12 {
                for b in 0..12 {
                    assert_eq!(ke[a][b].to_bits(), ke[b][a].to_bits(), "Ke symmetric");
                }
            }
        }
    }
    // The values are evaluated exactly, so roundoff residues are visible.
    assert!(
        nonzero > 0,
        "the exact evaluation sees the formation residue"
    );
}

/// The rod of the t4_u5b `arm_sign_end_i_breaks_rigid_modes` control:
/// b_i = (1.2, 2.4, 2.8), b_j = (1.8, 2.6, 3.1), n = (6, 2, 3)/7, l = 0.7.
fn control_rod() -> TieRod {
    let c = connector(
        node(0, [1.0, 2.0, 3.0]),
        node(1, [1.9, 2.3, 2.6]),
        // Q.x along x_j − x_i = (0.9, 0.3, −0.4) is not required by the rod;
        // the connector needs it, so the control uses Q with that first column.
        rotation_to([0.9, 0.3, -0.4]),
        [1.0; 6],
    );
    // Offsets are authored in Q; the control's offsets are global, so they
    // are re-expressed by Qᵀ (the builder's conversion).
    let q = c.axes();
    let local = |g: [f64; 3]| {
        [
            q[0][0] * g[0] + q[1][0] * g[1] + q[2][0] * g[2],
            q[0][1] * g[0] + q[1][1] * g[1] + q[2][1] * g[2],
            q[0][2] * g[0] + q[1][2] * g[1] + q[2][2] * g[2],
        ]
    };
    let c = c
        .with_tie_rods(&[rod(
            local([0.2, 0.4, -0.2]),
            local([-0.1, 0.3, 0.5]),
            1.0,
            0.0,
        )])
        .unwrap();
    c.tie_rods()[0]
}

/// A proper triad whose first column is `x` normalized.
fn rotation_to(x: [f64; 3]) -> Matrix3 {
    let n = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
    let ex = [x[0] / n, x[1] / n, x[2] / n];
    let helper = if ex[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let dot = helper[0] * ex[0] + helper[1] * ex[1] + helper[2] * ex[2];
    let y = [
        helper[0] - dot * ex[0],
        helper[1] - dot * ex[1],
        helper[2] - dot * ex[2],
    ];
    let m = (y[0] * y[0] + y[1] * y[1] + y[2] * y[2]).sqrt();
    let ey = [y[0] / m, y[1] / m, y[2] / m];
    let ez = [
        ex[1] * ey[2] - ex[2] * ey[1],
        ex[2] * ey[0] - ex[0] * ey[2],
        ex[0] * ey[1] - ex[1] * ey[0],
    ];
    [
        [ex[0], ey[0], ez[0]],
        [ex[1], ey[1], ez[1]],
        [ex[2], ey[2], ez[2]],
    ]
}

#[test]
fn t4_u5b_arm_sign_control_breaks_the_rigid_modes_and_the_element_does_not() {
    let rod = control_rod();
    let n = rod.direction();
    for (k, expected) in [6.0 / 7.0, 2.0 / 7.0, 3.0 / 7.0].into_iter().enumerate() {
        assert!((n[k] - expected).abs() <= 4.0 * U, "n[{k}] = {}", n[k]);
    }
    assert!((rod.length() - 0.7).abs() <= 4.0 * U);
    let (xi, xj) = (rod.node_i().coordinates, rod.node_j().coordinates);
    let origin = [7.0, -3.0, 5.0];
    let c = rod.c();
    // The arm-sign mutant: the end-i rotational entries with the wrong sign.
    let mut mutant = c;
    for k in 3..6 {
        mutant[k] = -mutant[k];
    }
    // The oracle's extensions under unit rigid rotations about x, y, z.
    let frozen = [16.0 / 35.0, -18.0 / 35.0, -4.0 / 7.0];
    for k in 0..3 {
        let correct = rigid_extension(&c, xi, xj, origin, 3 + k);
        assert!(correct.abs() <= 1e-15, "element rotation {k}: {correct:e}");
        let wrong = rigid_extension(&mutant, xi, xj, origin, 3 + k);
        assert!(
            (wrong - frozen[k]).abs() <= 1e-14,
            "mutant rotation {k}: {wrong} vs {}",
            frozen[k]
        );
    }
}

/// n = (2, 3, 6)/7: x_j − x_i = (1, 0, 0) along Q = I, o_i = (1/8, −1/4,
/// −1/2), o_j = (−3/8, 1/2, 1), so d = (1/2, 3/4, 3/2) and l = 7/4. 7·C is
/// the dyadic vector N.
fn pythagorean_input() -> TieRodInput {
    rod([0.125, -0.25, -0.5], [-0.375, 0.5, 1.0], 3.0e6, -1.0e-3)
}
fn pythagorean() -> (ObjectiveConnector, [f64; 12]) {
    let (oi, oj) = ([0.125, -0.25, -0.5], [-0.375, 0.5, 1.0]);
    let c = connector(
        node(0, [0.0; 3]),
        node(1, [1.0, 0.0, 0.0]),
        IDENTITY,
        [8.0; 6],
    )
    .with_tie_rods(&[pythagorean_input()])
    .unwrap();
    let n7 = [2.0, 3.0, 6.0];
    let cross = |o: [f64; 3]| {
        [
            o[1] * n7[2] - o[2] * n7[1],
            o[2] * n7[0] - o[0] * n7[2],
            o[0] * n7[1] - o[1] * n7[0],
        ]
    };
    let (ai, aj) = (cross(oi), cross(oj));
    let mut numerator = [0.0; 12];
    for k in 0..3 {
        numerator[k] = -n7[k];
        numerator[3 + k] = -ai[k];
        numerator[6 + k] = n7[k];
        numerator[9 + k] = aj[k];
    }
    (c, numerator)
}

/// a·b·c exactly into `sum`.
fn add_triple(sum: &mut ExactAccumulator, a: f64, b: f64, c: f64) {
    let p = b * c;
    let e = b.mul_add(c, -p);
    sum.add_product(a, p).unwrap();
    sum.add_product(a, e).unwrap();
}

#[test]
fn stiffness_reference_load_and_recovery_match_exact_rational_values() {
    let (c, n7) = pythagorean();
    let rod = &c.tie_rods()[0];
    assert_eq!(rod.length(), 1.75);
    assert_eq!(rod.chord(), [0.5, 0.75, 1.5]);
    let k = rod.stiffness();
    let e_ref = rod.reference_extension();
    let cb = rod.c();
    for a in 0..12 {
        // |7·C_a − N_a| ≤ 2u·|N_a| + 2u·Σ|o||n| (one rounding of n, one of the dot).
        let scale = n7[a].abs() + 4.0;
        assert!((7.0 * cb[a] - n7[a]).abs() <= 4.0 * U * scale, "C[{a}]");
    }
    let ke = rod.global_stiffness().unwrap();
    for a in 0..12 {
        for b in 0..12 {
            // 49·Ke_ab − k N_a N_b, exactly, then rounded.
            let mut residual = ExactAccumulator::new();
            residual.add_product(49.0, ke[a][b]).unwrap();
            add_triple(&mut residual, -k, n7[a], n7[b]);
            let r = residual.round().unwrap();
            let tolerance = 16.0 * U * k * (n7[a].abs() + 4.0) * (n7[b].abs() + 4.0);
            assert!(r.abs() <= tolerance, "Ke[{a}][{b}]: {r:e}");
        }
    }
    let load = rod.reference_load().unwrap().unwrap();
    let mut tight = true;
    for a in 0..12 {
        // The bound contains the exact value: |7 v_a − k e_ref N_a| ≤ 7 bound_a.
        let mut above = ExactAccumulator::new();
        above.add_product(7.0, load.values[a]).unwrap();
        add_triple(&mut above, -k, e_ref, n7[a]);
        let mut low = above.clone();
        low.add_product(-7.0, load.bounds[a]).unwrap();
        above.add_product(7.0, load.bounds[a]).unwrap();
        assert!(
            low.signum() <= 0 && above.signum() >= 0,
            "reference load [{a}] outside its bound"
        );
        tight &= load.bounds[a] <= 1e-12 * (k * e_ref.abs() + 1.0);
    }
    assert!(tight, "the bound is of roundoff size");
    // Recovery at dyadic displacements: T = k(N·d/7 − e_ref).
    let d = [
        1.0e-4, -2.5e-4, 5.0e-4, 1.0e-3, -2.0e-3, 0.5e-3, 3.0e-4, 1.25e-4, -7.5e-4, -1.0e-3,
        2.5e-4, 1.5e-3,
    ];
    let recovered = rod.recover(&d).unwrap();
    let mut exact = ExactAccumulator::new();
    for l in 0..12 {
        add_triple(&mut exact, k, n7[l], d[l]);
    }
    add_triple(&mut exact, -7.0, k, e_ref);
    let t7 = exact.round().unwrap();
    assert!(
        (7.0 * recovered.tension - t7).abs() <= 1e-12 * t7.abs(),
        "T"
    );
    for k in 0..3 {
        assert_eq!(
            recovered.nodal_action[k],
            -recovered.nodal_action[6 + k],
            "F_i = −F_j"
        );
    }
    // The design's default (e_ref = 0) omits the reference load: "one ε".
    let zero = connector(
        node(0, [0.0; 3]),
        node(1, [1.0, 0.0, 0.0]),
        IDENTITY,
        [8.0; 6],
    )
    .with_tie_rods(&[TieRodInput {
        reference_extension: -0.0,
        ..pythagorean_input()
    }])
    .unwrap();
    assert!(zero.tie_rods()[0].reference_load().unwrap().is_none());
}

/// The generalized stiffness of a connector's rods, Σ k c cᵀ with
/// c = (Qᵀn, Qᵀ(m × n)) and m = (o_i + o_j)/2 (zero connector offsets).
fn generalized(c: &ObjectiveConnector) -> [[f64; 6]; 6] {
    let q = c.axes();
    let mut g = [[0.0; 6]; 6];
    for rod in c.tie_rods() {
        let n = rod.direction();
        let (oi, oj) = rod.offsets();
        let m = [
            0.5 * (oi[0] + oj[0]),
            0.5 * (oi[1] + oj[1]),
            0.5 * (oi[2] + oj[2]),
        ];
        let mn = [
            m[1] * n[2] - m[2] * n[1],
            m[2] * n[0] - m[0] * n[2],
            m[0] * n[1] - m[1] * n[0],
        ];
        let mut cv = [0.0; 6];
        for axis in 0..3 {
            cv[axis] = q[0][axis] * n[0] + q[1][axis] * n[1] + q[2][axis] * n[2];
            cv[3 + axis] = q[0][axis] * mn[0] + q[1][axis] * mn[1] + q[2][axis] * mn[2];
        }
        for r in 0..6 {
            for s in 0..6 {
                g[r][s] += rod.stiffness() * cv[r] * cv[s];
            }
        }
    }
    g
}

fn assembled(c: &[ObjectiveConnector]) -> Vec<Vec<f64>> {
    assemble_global_stiffness_with_connectors(2, &[], c).unwrap()
}

/// The rods' share of the assembled matrix, against a connector whose H is
/// the expected generalized stiffness: Bᵀ(Σ k c cᵀ)B = Σ CᵀkC.
fn assert_equivalent(with_rods: &ObjectiveConnector, upper: [f64; 21], exact: bool) {
    let plain = with_rods.clone().with_tie_rods(&[]).unwrap();
    let (ni, nj, q) = (with_rods.node_i(), with_rods.node_j(), with_rods.axes());
    let equivalent = with_upper(ni, nj, q, upper);
    let rods = assembled(std::slice::from_ref(with_rods));
    let reference = assembled(&[plain, equivalent]);
    let scale = rods.iter().flatten().fold(0.0_f64, |m, v| m.max(v.abs()));
    for (r, (x, y)) in rods
        .iter()
        .flatten()
        .zip(reference.iter().flatten())
        .enumerate()
    {
        if exact {
            assert_eq!(x.to_bits(), y.to_bits(), "entry {r}: {x} vs {y}");
        } else {
            assert!((x - y).abs() <= 1e-12 * scale, "entry {r}: {x} vs {y}");
        }
    }
}

#[test]
fn t4_u5b_offset_pairs_four_rods_and_a_single_rod_reduce_to_the_generalized_stiffness() {
    let k = 1.0e7;
    let a = 0.125;
    let (ni, nj) = (node(0, [0.0; 3]), node(1, [0.5, 0.0, 0.0]));
    let base = connector(ni, nj, IDENTITY, [8e5, 6e5, 6e5, 2e3, 3e3, 3e3]);
    let pair_y = [
        rod([-0.0625, a, 0.0], [0.0625, a, 0.0], k, 0.0),
        rod([-0.0625, -a, 0.0], [0.0625, -a, 0.0], k, 0.0),
    ];
    let pair_z = [
        rod([-0.0625, 0.0, a], [0.0625, 0.0, a], k, 0.0),
        rod([-0.0625, 0.0, -a], [0.0625, 0.0, -a], k, 0.0),
    ];
    // ±a e_y: K_xx = 2k, K_rz = 2k a², cross terms (∓k a) cancel.
    let c = base.clone().with_tie_rods(&pair_y).unwrap();
    let g = generalized(&c);
    assert_eq!(
        (g[0][0], g[5][5], g[0][5], g[5][0]),
        (2.0 * k, 2.0 * k * a * a, 0.0, 0.0)
    );
    for (r, row) in g.iter().enumerate() {
        for (s, v) in row.iter().enumerate() {
            if !matches!((r, s), (0, 0) | (5, 5)) {
                assert_eq!(*v, 0.0, "pair_y G[{r}][{s}]");
            }
        }
    }
    let mut upper = [0.0; 21];
    upper[upper_index(0, 0)] = 2.0 * k;
    upper[upper_index(5, 5)] = 2.0 * k * a * a;
    assert_equivalent(&c, upper, true);
    // C = c·B for zero connector offsets (exact here).
    let b = c.b();
    for rod in c.tie_rods() {
        let y = rod.offsets().0[1];
        let cv = [1.0, 0.0, 0.0, 0.0, 0.0, -y];
        let mut cb = [0.0; 12];
        for l in 0..12 {
            for (kk, ck) in cv.iter().enumerate() {
                cb[l] += ck * b[kk][l];
            }
        }
        assert_eq!(rod.c(), cb, "C = c B");
        assert_eq!(rod.axial_share(), 1.0);
    }
    // Four rods: K_xx = 4k, K_ry = K_rz = 2k a², no torsion stiffness.
    let four: Vec<_> = pair_y.iter().chain(&pair_z).copied().collect();
    let c = base.clone().with_tie_rods(&four).unwrap();
    let mut upper = [0.0; 21];
    upper[upper_index(0, 0)] = 4.0 * k;
    upper[upper_index(4, 4)] = 2.0 * k * a * a;
    upper[upper_index(5, 5)] = 2.0 * k * a * a;
    assert_eq!(generalized(&c)[3][3], 0.0, "K_rx unchanged");
    assert_equivalent(&c, upper, true);
    // One rod at +a e_y: K_xx = k, K_rz = k a², K_x,rz = −k a.
    let c = base.clone().with_tie_rods(&pair_y[..1]).unwrap();
    let mut upper = [0.0; 21];
    upper[upper_index(0, 0)] = k;
    upper[upper_index(0, 5)] = -k * a;
    upper[upper_index(5, 5)] = k * a * a;
    assert_equivalent(&c, upper, true);
    // The oracle's layout (a = 0.1 m, axial offsets ∓0.05 m, k = 1e7 N/m):
    // K_xx = 2e7 and K_rz = 2e5 within roundoff.
    let layout = [
        rod([-0.05, 0.1, 0.0], [0.05, 0.1, 0.0], 1.0e7, 0.0),
        rod([-0.05, -0.1, 0.0], [0.05, -0.1, 0.0], 1.0e7, 0.0),
    ];
    let c = base.clone().with_tie_rods(&layout).unwrap();
    let g = generalized(&c);
    assert!((g[0][0] - 2.0e7).abs() <= 1e-8 && (g[5][5] - 2.0e5).abs() <= 1e-9);
    assert!(g[0][5].abs() <= 1e-9);
    let mut upper = [0.0; 21];
    upper[upper_index(0, 0)] = 2.0e7;
    upper[upper_index(5, 5)] = 2.0e5;
    assert_equivalent(&c, upper, false);
}

#[test]
fn c_equals_c_times_b_on_skew_connectors_with_parallel_and_skew_rods() {
    for c in skew_connectors() {
        let b = c.b();
        let g = generalized(&c);
        // Σ CᵀkC = Bᵀ G B within roundoff, entry by entry.
        let mut rods: Matrix12 = [[0.0; 12]; 12];
        for rod in c.tie_rods() {
            let ke = rod.global_stiffness().unwrap();
            for r in 0..12 {
                for s in 0..12 {
                    rods[r][s] += ke[r][s];
                }
            }
        }
        let scale = rods.iter().flatten().fold(0.0_f64, |m, v| m.max(v.abs()));
        for r in 0..12 {
            for s in 0..12 {
                let mut btgb = 0.0;
                for p in 0..6 {
                    for t in 0..6 {
                        btgb += b[p][r] * g[p][t] * b[t][s];
                    }
                }
                assert!(
                    (btgb - rods[r][s]).abs() <= 1e-11 * scale,
                    "[{r}][{s}] {btgb} {}",
                    rods[r][s]
                );
            }
        }
    }
}

fn model() -> (Vec<FrameElement>, Vec<ObjectiveConnector>) {
    let section = FrameSection::new(200e9, 80e9, 6.0e-3, 2.7e-5, 2.7e-5, 5.4e-5).unwrap();
    let (third, two_thirds) = (1.0 / 3.0, 2.0 / 3.0);
    let q = [
        [third, two_thirds, -two_thirds],
        [two_thirds, third, two_thirds],
        [two_thirds, -two_thirds, -third],
    ];
    let p = [
        [0.0, 0.0, 0.0],
        [3.0, 0.0, 0.0],
        [3.5, 1.0, 1.0],
        [6.5, 1.0, 1.0],
    ];
    let n = |k: usize| node(k, p[k]);
    let frames = vec![
        FrameElement::new(n(0), n(1), section, [0.0, 0.0, 1.0]).unwrap(),
        FrameElement::new(n(2), n(3), section, [0.0, 0.0, 1.0]).unwrap(),
    ];
    let c = connector(n(1), n(2), q, [5e6, 4e6, 3e6, 2e3, 1e3, 1.5e3])
        .with_tie_rods(&[
            rod([-0.05, 0.13, -0.07], [0.06, -0.11, 0.09], 2.5e7, -1.25e-3),
            rod([0.0, -0.09, 0.08], [0.11, 0.1, -0.03], 7.0e6, 0.0),
        ])
        .unwrap();
    (frames, vec![c])
}

fn bits(m: &[Vec<f64>]) -> Vec<u64> {
    m.iter().flatten().map(|v| v.to_bits()).collect()
}

#[test]
fn dense_and_sparse_assemble_the_rods_bit_for_bit_after_the_connectors() {
    let (frames, connectors) = model();
    let dense = assemble_global_stiffness_with_connectors(4, &frames, &connectors).unwrap();
    let sparse = assemble_sparse_stiffness(
        4,
        &frames,
        &connectors,
        &[],
        &[],
        &SparseAssemblyOptions::new(),
    )
    .unwrap();
    assert_eq!(bits(&sparse.to_dense()), bits(&dense));
    // The slot order: frames, connectors, then each rod's own Ke.
    let plain = vec![connectors[0].clone().with_tie_rods(&[]).unwrap()];
    let mut expected = assemble_global_stiffness_with_connectors(4, &frames, &plain).unwrap();
    for rod in connectors[0].tie_rods() {
        let ke = rod.global_stiffness().unwrap();
        let map =
            open_pipe_stress_frame_kernel::element_dof_map(rod.node_i().index, rod.node_j().index);
        for r in 0..12 {
            for c in 0..12 {
                expected[map[r]][map[c]] += ke[r][c];
            }
        }
    }
    assert_eq!(bits(&dense), bits(&expected));
    // Without rods: the T4-U3 connector, bit for bit.
    let untied = connectors[0].clone().with_tie_rods(&[]).unwrap();
    let rebuilt = ObjectiveConnector::new(
        untied.node_i(),
        untied.node_j(),
        ConnectorAttachment::global(untied.offsets().0),
        ConnectorAttachment::global(untied.offsets().1),
        untied.axes(),
        untied.work_matrix(),
        untied.q_ref(),
    )
    .unwrap();
    assert_eq!(untied, rebuilt);
    assert!(rebuilt.tie_rods().is_empty());
}

#[test]
fn k2b_scales_the_rods_exactly_and_the_census_takes_k_and_ke() {
    let (frames, connectors) = model();
    let unscaled = assemble_sparse_stiffness(
        4,
        &frames,
        &connectors,
        &[],
        &[(0, 1.0e7)],
        &SparseAssemblyOptions::new(),
    )
    .unwrap();
    for b in [-40, 20, 64] {
        let scale = ForceScale::new(b).unwrap();
        let scaled = assemble_sparse_stiffness(
            4,
            &frames,
            &connectors,
            &[],
            &[(0, 1.0e7)],
            &SparseAssemblyOptions::new().with_force_scale(scale),
        )
        .unwrap();
        let factor = 2f64.powi(b);
        assert_eq!(scaled.pattern(), unscaled.pattern());
        for (s, u) in scaled.values().iter().zip(unscaled.values()) {
            assert_eq!(s.to_bits(), (u * factor).to_bits(), "b = {b}");
        }
        // K-D5's scaled source: the connector's rods carry k·2^b.
        let source = connectors[0].force_scaled(scale).unwrap();
        for (r, s) in connectors[0].tie_rods().iter().zip(source.tie_rods()) {
            assert_eq!(s.stiffness(), r.stiffness() * factor);
            let (x, y) = (r.global_stiffness().unwrap(), s.global_stiffness().unwrap());
            for (p, q) in x.iter().flatten().zip(y.iter().flatten()) {
                assert_eq!(q.to_bits(), (p * factor).to_bits());
            }
            assert_eq!(
                s.force_scaled_global_stiffness(ForceScale::UNSCALED)
                    .unwrap(),
                y
            );
        }
    }
    let mut census = ForceScaleCensus::new();
    census.connector(&connectors[0]).unwrap();
    let exponent = |v: f64| ((v.abs().to_bits() >> 52) as i32) - 1023;
    let mut values: Vec<f64> = connectors[0]
        .stiffness()
        .iter()
        .flatten()
        .copied()
        .collect();
    values.extend(connectors[0].global_stiffness().unwrap().iter().flatten());
    for rod in connectors[0].tie_rods() {
        values.push(rod.stiffness());
        values.extend(rod.global_stiffness().unwrap().iter().flatten());
    }
    let values: Vec<f64> = values.into_iter().filter(|v| *v != 0.0).collect();
    let low = values.iter().map(|v| exponent(*v)).min().unwrap();
    let high = values.iter().map(|v| exponent(*v)).max().unwrap();
    assert_eq!(census.span(), Some((low, high)));
    let mut untied = ForceScaleCensus::new();
    untied
        .connector(&connectors[0].clone().with_tie_rods(&[]).unwrap())
        .unwrap();
    assert_ne!(untied.span(), census.span(), "the rods enter the census");
}

#[test]
fn rods_are_refused_by_name() {
    let c = connector(
        node(0, [0.0; 3]),
        node(1, [1.0, 0.0, 0.0]),
        IDENTITY,
        [8.0; 6],
    );
    // b_j − b_i = (1, 0, 0) + ((−0.5, 0, 0) − (0.5, 0, 0)) = 0.
    let zero = rod([0.5, 0.0, 0.0], [-0.5, 0.0, 0.0], 1.0e6, 0.0);
    assert_eq!(TieRod::new(&c, zero), Err(TieRodError::ZeroLength));
    let good = rod([0.0, 0.1, 0.0], [0.0, 0.1, 0.0], 1.0e6, 0.0);
    assert_eq!(
        c.clone().with_tie_rods(&[good, zero]).unwrap_err(),
        (1, TieRodError::ZeroLength)
    );
    for k in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::MIN_POSITIVE / 2.0] {
        assert_eq!(
            TieRod::new(
                &c,
                TieRodInput {
                    stiffness: k,
                    ..good
                }
            ),
            Err(TieRodError::Stiffness)
        );
    }
    assert!(matches!(
        TieRod::new(
            &c,
            TieRodInput {
                reference_extension: f64::INFINITY,
                ..good
            }
        ),
        Err(TieRodError::NonFinite { .. })
    ));
    assert!(matches!(
        TieRod::new(
            &c,
            TieRodInput {
                offset_j: [0.0, f64::NAN, 0.0],
                ..good
            }
        ),
        Err(TieRodError::NonFinite { .. })
    ));
    // A rod nearly perpendicular to the joint axis is admitted (§3.4).
    let across = rod([0.0, -0.5, 0.0], [-1.0, 0.5, 0.0], 1.0e6, 0.0);
    let r = TieRod::new(&c, across).unwrap();
    assert_eq!(r.axial_share(), 0.0);
    assert_eq!(r.direction(), [0.0, 1.0, 0.0]);
}

/// K1 repair 1: finite offsets whose arm (o × n) overflows refuse by name,
/// never panic (Codex's example); Opus's example is degenerate under the
/// length guard (Δx = 1 against offsets of 1.7e308) and refuses as
/// `ZeroLength`. |k·e_ref| below the exact-split floor refuses at
/// construction.
#[test]
fn extreme_offsets_and_tiny_reference_loads_refuse_by_name() {
    let m = f64::MAX;
    let c = connector(
        node(0, [0.0; 3]),
        node(1, [1.0, 0.0, 0.0]),
        IDENTITY,
        [8.0; 6],
    );
    let codex = rod([m, m / 2.0, 0.0], [m / 2.0, m, 0.0], 1.0, 0.0);
    assert_eq!(
        TieRod::new(&c, codex),
        Err(TieRodError::Range {
            name: "offset arm (o x n)"
        })
    );
    assert_eq!(
        c.clone().with_tie_rods(&[codex]).unwrap_err(),
        (
            0,
            TieRodError::Range {
                name: "offset arm (o x n)"
            }
        )
    );
    let diagonal = node(1, [0.0, 1.0, 1.0]);
    let q = rotation_to([0.0, 1.0, 1.0]);
    let c2 = connector(node(0, [0.0; 3]), diagonal, q, [8.0; 6]);
    // Opus's global offsets (0, 1.7e308, −1.7e308) have no representable
    // offset in these connector axes (it would be −2.4e308 along Q.y); the
    // nearest authorable case is (0, −1.7e308, 0) in Q, giving
    // o_i = o_j ≈ (0, 1.2e308, −1.2e308) and the same Δx = (0, 1, 1).
    let o = [0.0, -1.7e308, 0.0];
    let r2 = c2.clone().with_tie_rods(&[]).unwrap();
    assert!(TieRod::new(&r2, rod(o, o, 1.0, 0.0)).is_err());
    assert_eq!(
        TieRod::new(&c2, rod(o, o, 1.0, 0.0)),
        Err(TieRodError::ZeroLength)
    );
    let tiny = rod([0.0, 0.1, 0.0], [0.0, 0.1, 0.0], 1.0, 1.0e-300);
    assert_eq!(
        TieRod::new(&c, tiny),
        Err(TieRodError::Range {
            name: "reference load"
        })
    );
}

/// K1 repair 2 (manager ruling): l ≤ 2^-48·(‖Δx‖∞ + ‖o_i‖∞ + ‖o_j‖∞) is
/// `ZeroLength`, decided exactly. With Δx = (1, 0, 0), o_i = (0.5, 0, 0) and
/// o_j = (−0.5, t, 0), d = (0, t, 0) and the guard is 2^-48·2 = 2^-47.
#[test]
fn the_degenerate_length_guard_is_exact_at_its_threshold_and_refuses_cancellation() {
    let c = connector(
        node(0, [0.0; 3]),
        node(1, [1.0, 0.0, 0.0]),
        IDENTITY,
        [8.0; 6],
    );
    let guard = 2f64.powi(-47);
    for (t, admitted) in [
        (guard, false),
        (guard.next_down(), false),
        (guard.next_up(), true),
        (-guard, false),
        (-guard.next_up(), true),
        (0.25, true),
    ] {
        let r = TieRod::new(&c, rod([0.5, 0.0, 0.0], [-0.5, t, 0.0], 1.0e6, 0.0));
        if admitted {
            let r = r.unwrap();
            assert_eq!(r.length(), t.abs());
            assert_eq!(r.direction(), [0.0, t.signum(), 0.0]);
        } else {
            assert_eq!(r, Err(TieRodError::ZeroLength), "t = {t:e}");
        }
    }
    // Δx = −(o_j − o_i) to roundoff: d = 0.3 + fl(−0.2 − 0.1) ≈ −5.6e-17, a
    // noise direction the zero test alone admitted.
    let short = connector(
        node(0, [0.0; 3]),
        node(1, [0.3, 0.0, 0.0]),
        IDENTITY,
        [8.0; 6],
    );
    let cancelling = rod([0.1, 0.0, 0.0], [-0.2, 0.0, 0.0], 1.0e6, 0.0);
    assert_ne!(
        0.3 + (-0.2 - 0.1),
        0.0,
        "the represented difference is roundoff"
    );
    assert_eq!(
        TieRod::new(&short, cancelling),
        Err(TieRodError::ZeroLength)
    );
    // The same with a transverse roundoff residue.
    let skewed = rod([0.1, 0.1, 0.0], [-0.2, 0.1 + 1e-17, 0.0], 1.0e6, 0.0);
    assert_eq!(TieRod::new(&short, skewed), Err(TieRodError::ZeroLength));
}

/// K1 repair 4: the memory footprint (`cargo test -- --nocapture sizes`).
#[test]
fn sizes() {
    use open_pipe_stress_frame_kernel::structural::LoadFormula;
    println!(
        "size_of: ObjectiveConnector {} B, LoadFormula {} B, TieRod {} B",
        std::mem::size_of::<ObjectiveConnector>(),
        std::mem::size_of::<LoadFormula>(),
        std::mem::size_of::<TieRod>()
    );
    // A rod-reference primitive holds one Copy rod, never a connector.
    assert!(std::mem::size_of::<TieRod>() < std::mem::size_of::<ObjectiveConnector>());
}
