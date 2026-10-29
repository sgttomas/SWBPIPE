//! T4-U8 (D-6): the Timoshenko straight member as a re-priced FrameElement
//! (`DEL-05-03` Design `section-bases-and-timoshenko.md` §§3.1–3.3, §8).
//!
//! Expected values are SHEAR_REFERENCE's frozen §5 values (CONTRACT.md and
//! `reference_arithmetic.py(.stdout.txt)` at f2211ecef6), quoted verbatim as
//! rationals, or closed forms of the 1D model (CONTRACT §§2–3: v_tip =
//! FL³/(3EI) + FL/Ks, theta_tip = FL²/(2EI), U = ∫M²/2EI + ∫V²/2Ks), evaluated
//! here. None is computed through the production element. Invented inputs
//! only; no pipe table or code value.
use open_pipe_stress_frame_kernel::shear::{annulus_energy_shear_factor, shear_area, ShearFactorError};
use open_pipe_stress_frame_kernel::structural::retained_api::{
    RetainedMemberRefusal, StraightMember, TimoshenkoLeaf, TimoshenkoLeafError,
    RETAINED_TIMOSHENKO_REFUSAL,
};
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness, local_stiffness, reduce_system, solve_dense, BeamTheory,
    ForceScale, ForceScaleCensus, FrameElement, FrameKernelError, FrameNode, FrameProperties,
    FrameSection, Matrix12, DOF_PER_NODE, ELEMENT_DOF, RX, RY, RZ, UX, UY, UZ,
};

// ---------------------------------------------------------------- helpers

/// |actual - expected| <= tol * max(1, |expected|).
fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!(
        (actual - expected).abs() <= tol * expected.abs().max(1.0),
        "{what}: {actual:e} vs {expected:e}"
    );
}

/// Units in the last place between two finite binary64 values of one sign.
fn ulps(a: f64, b: f64) -> u64 {
    assert!(a.is_finite() && b.is_finite() && a.signum() == b.signum());
    a.to_bits().abs_diff(b.to_bits())
}

/// EI = Iz (and Iy), Ks = G*A_s with E = G = A = J = 1.
fn abstract_section(iy: f64, iz: f64, shear: f64) -> FrameSection {
    FrameSection::timoshenko(1.0, 1.0, 1.0, iy, iz, 1.0, shear).unwrap()
}

fn element(i: usize, j: usize, xi: [f64; 3], xj: [f64; 3], section: FrameSection, y: [f64; 3]) -> FrameElement {
    FrameElement::new(
        FrameNode::new(i, xi).unwrap(),
        FrameNode::new(j, xj).unwrap(),
        section,
        y,
    )
    .unwrap()
}

/// A straight run along local x from the origin, `n` equal elements.
fn run(section: FrameSection, length: f64, n: usize) -> Vec<FrameElement> {
    (0..n)
        .map(|k| {
            let a = length * k as f64 / n as f64;
            let b = length * (k + 1) as f64 / n as f64;
            element(k, k + 1, [a, 0.0, 0.0], [b, 0.0, 0.0], section, [0.0, 1.0, 0.0])
        })
        .collect()
}

/// Displacements (full vector) and K*u - f (reactions, full vector) of a
/// model with node 0 clamped, optional ground springs (global DOF, k).
fn solve(
    elements: &[FrameElement],
    node_count: usize,
    springs: &[(usize, f64)],
    force: &[f64],
) -> (Vec<f64>, Vec<f64>) {
    let mut k = assemble_global_stiffness(node_count, elements).unwrap();
    for &(dof, stiffness) in springs {
        k[dof][dof] += stiffness;
    }
    let fixed: Vec<usize> = (0..DOF_PER_NODE).collect();
    let reduced = reduce_system(&k, force, &fixed).unwrap();
    let free = solve_dense(&reduced.stiffness, &reduced.force).unwrap();
    let mut u = vec![0.0; node_count * DOF_PER_NODE];
    for (value, &dof) in free.iter().zip(&reduced.free_dofs) {
        u[dof] = *value;
    }
    let reactions = (0..u.len())
        .map(|r| k[r].iter().zip(&u).map(|(a, b)| a * b).sum::<f64>() - force[r])
        .collect();
    (u, reactions)
}

fn tip_force(node_count: usize, dof: usize, value: f64) -> Vec<f64> {
    let mut f = vec![0.0; node_count * DOF_PER_NODE];
    f[(node_count - 1) * DOF_PER_NODE + dof] = value;
    f
}

fn properties(section: FrameSection, length: f64) -> FrameProperties {
    FrameProperties::new(section, length).unwrap()
}

/// The (v, theta) block of one plane in local K: Iz plane [uy, rz], Iy plane
/// [uz, ry].
fn plane_block(k: &Matrix12, z_plane: bool) -> [[f64; 4]; 4] {
    let idx = if z_plane {
        [UY, RZ, UY + DOF_PER_NODE, RZ + DOF_PER_NODE]
    } else {
        [UZ, RY, UZ + DOF_PER_NODE, RY + DOF_PER_NODE]
    };
    let mut out = [[0.0; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            out[r][c] = k[idx[r]][idx[c]];
        }
    }
    out
}

// ------------------------------------------------- S-A rational controls

/// SHEAR_REFERENCE §5 S-A: EI = 1 N·m², Ks = 12 N, L = 1 m, phi = 1.
#[test]
fn timoshenko_s_a_rational_cantilever_controls() {
    let section = abstract_section(1.0, 1.0, 12.0);
    // The S-A K rows (reference_arithmetic.stdout.txt): exact in binary64.
    let k = local_stiffness(properties(section, 1.0)).unwrap();
    let s_a = [
        [6.0, 3.0, -6.0, 3.0],
        [3.0, 2.5, -3.0, 0.5],
        [-6.0, -3.0, 6.0, -3.0],
        [3.0, 0.5, -3.0, 2.5],
    ];
    assert_eq!(plane_block(&k, true), s_a, "Iz plane (v = u_y, theta = r_z)");
    // The Iy plane (v = u_z, theta = -r_y): the same K with the existing
    // frame convention's sign on the r_y rows and columns.
    let sign = [1.0, -1.0, 1.0, -1.0];
    let mut s_a_y = s_a;
    for r in 0..4 {
        for c in 0..4 {
            s_a_y[r][c] = sign[r] * sign[c] * s_a[r][c];
        }
    }
    assert_eq!(plane_block(&k, false), s_a_y, "Iy plane (v = u_z, theta = -r_y)");
    assert_eq!(
        properties(section, 1.0).timoshenko_phi().unwrap(),
        Some([1.0, 1.0])
    );

    let model = run(section, 1.0, 1);
    let tip = DOF_PER_NODE;
    // +1 N tip force: v 5/12, theta 1/2; root Fy = -1, Mz = -1.
    let (u, r) = solve(&model, 2, &[], &tip_force(2, UY, 1.0));
    close(u[tip + UY], 5.0 / 12.0, 1e-15, "tip v");
    close(u[tip + RZ], 0.5, 1e-15, "tip theta");
    close(r[UY], -1.0, 1e-15, "root Fy");
    close(r[RZ], -1.0, 1e-15, "root Mz");
    // Section rotation theta is not the slope: the slope is theta + V/Ks
    // with V the member shear (1 N), 7/12 (kills theta = v').
    let v = -r[UY];
    close(u[tip + RZ] + v / 12.0, 7.0 / 12.0, 1e-15, "tip slope");
    assert!((u[tip + RZ] - 7.0 / 12.0).abs() > 0.08);

    // +1 N·m tip couple: v 1/2, theta 1, V = 0, independent of Ks.
    for shear in [1.2, 12.0, 120.0] {
        let model = run(abstract_section(1.0, 1.0, shear), 1.0, 1);
        let (u, r) = solve(&model, 2, &[], &tip_force(2, RZ, 1.0));
        close(u[tip + UY], 0.5, 1e-14, "couple v");
        close(u[tip + RZ], 1.0, 1e-14, "couple theta");
        close(r[UY], 0.0, 1e-14, "couple root shear");
        close(r[RZ], -1.0, 1e-14, "couple root moment");
    }

    // +1 N/m over the full span: the full-span Timoshenko vector is the
    // Euler–Bernoulli one, [wL/2, wL²/12, wL/2, -wL²/12] (SHEAR_REFERENCE §4):
    // v 1/6, theta 1/6, root Fy = -1, Mz = -1/2.
    let mut f = vec![0.0; 12];
    f[UY] = 0.5;
    f[RZ] = 1.0 / 12.0;
    f[tip + UY] = 0.5;
    f[tip + RZ] = -1.0 / 12.0;
    let (u, r) = solve(&model, 2, &[], &f);
    close(u[tip + UY], 1.0 / 6.0, 1e-15, "uniform v");
    close(u[tip + RZ], 1.0 / 6.0, 1e-15, "uniform theta");
    close(r[UY], -1.0, 1e-15, "uniform root Fy");
    close(r[RZ], -0.5, 1e-15, "uniform root Mz");

    // Fixed-guided delta = 1 (both rotations zero): force 6 = 12EI/(L³(1+phi)),
    // the paired end moments -6EI/(L²(1+phi)) = -3 from the exact matrix, and
    // whole-body balance (moments about i: M_i + M_j + L F_j = 0).
    let mut d = [0.0; 12];
    d[tip + UY] = 1.0;
    let f: Vec<f64> = k.iter().map(|row| row.iter().zip(&d).map(|(a, b)| a * b).sum()).collect();
    assert_eq!([f[UY], f[RZ], f[tip + UY], f[tip + RZ]], [-6.0, -3.0, 6.0, -3.0]);
    assert_eq!(f[UY] + f[tip + UY], 0.0);
    assert_eq!(f[RZ] + f[tip + RZ] + 1.0 * f[tip + UY], 0.0);
}

// ----------------------------------------- axial, torsion and the planes

/// A is the metal area in EA and kappa never touches GJ (kills A_s in EA and
/// kappa applied to GJ).
#[test]
fn timoshenko_axial_and_torsion_are_unchanged() {
    let (e, g, a, iy, iz, j) = (200e9, 1000e9 / 13.0, 0.0019 * std::f64::consts::PI, 3e-5, 2e-5, 5e-5);
    let kappa = annulus_energy_shear_factor(0.09, 0.1, 0.3).unwrap();
    let a_s = shear_area(kappa, a).unwrap();
    let eb = FrameSection::new(e, g, a, iy, iz, j).unwrap();
    let ts = FrameSection::timoshenko(e, g, a, iy, iz, j, a_s).unwrap();
    for length in [0.25, 1.0, 7.5] {
        let keb = local_stiffness(properties(eb, length)).unwrap();
        let kt = local_stiffness(properties(ts, length)).unwrap();
        for (r, c) in [
            (UX, UX),
            (UX, UX + DOF_PER_NODE),
            (UX + DOF_PER_NODE, UX + DOF_PER_NODE),
            (RX, RX),
            (RX, RX + DOF_PER_NODE),
            (RX + DOF_PER_NODE, RX + DOF_PER_NODE),
        ] {
            assert_eq!(kt[r][c].to_bits(), keb[r][c].to_bits(), "[{r}][{c}] at L {length}");
            assert_eq!(kt[c][r].to_bits(), keb[c][r].to_bits());
        }
        assert_eq!(kt[UX][UX], e * a / length);
        assert_eq!(kt[RX][RX], g * j / length);
        assert_ne!(kt[UX][UX], e * a_s / length);
        // Axial and torsion rows couple to nothing else.
        for c in 0..ELEMENT_DOF {
            if c % DOF_PER_NODE != UX {
                assert_eq!(kt[UX][c], 0.0);
            }
            if c % DOF_PER_NODE != RX {
                assert_eq!(kt[RX][c], 0.0);
            }
        }
        // Bending is softer than Euler–Bernoulli (shear compliance).
        assert!(kt[UY][UY] < keb[UY][UY] && kt[UZ][UZ] < keb[UZ][UZ]);
    }
    assert!(kappa < 1.0 && a_s < a);
}

/// Both bending planes with their signs (Iy != Iz; phi_y = 2 exactly makes
/// 2 - phi an exact zero), and a spatially rotated model's response is the
/// rotated response (kills swapped bending-plane signs).
#[test]
fn timoshenko_bending_planes_and_rotation_covariance() {
    // EI_y = 2, EI_z = 1, Ks = 12, L = 1: phi_y = 2, phi_z = 1.
    let section = abstract_section(2.0, 1.0, 12.0);
    let k = local_stiffness(properties(section, 1.0)).unwrap();
    assert_eq!(k[RY][RY + DOF_PER_NODE], 0.0, "(2 - phi_y) EI_y/(L(1+phi_y)) = 0");
    assert_eq!(properties(section, 1.0).timoshenko_phi().unwrap(), Some([2.0, 1.0]));
    let model = run(section, 1.0, 1);
    let tip = DOF_PER_NODE;
    let ks = 12.0;
    // Tip force F_y: u_y = L³/(3EI_z) + L/Ks, r_z = +L²/(2EI_z).
    let (u, _) = solve(&model, 2, &[], &tip_force(2, UY, 1.0));
    close(u[tip + UY], 1.0 / 3.0 + 1.0 / ks, 1e-15, "F_y: u_y");
    close(u[tip + RZ], 0.5, 1e-15, "F_y: r_z");
    assert_eq!([u[tip + UZ], u[tip + RY]], [0.0, 0.0]);
    // Tip force F_z: u_z = L³/(3EI_y) + L/Ks, r_y = -L²/(2EI_y).
    let (u, _) = solve(&model, 2, &[], &tip_force(2, UZ, 1.0));
    close(u[tip + UZ], 1.0 / 6.0 + 1.0 / ks, 1e-15, "F_z: u_z");
    close(u[tip + RY], -0.25, 1e-15, "F_z: r_y");
    assert_eq!([u[tip + UY], u[tip + RZ]], [0.0, 0.0]);
    // Tip couples: M_z gives u_y = L²/(2EI_z), r_z = L/EI_z; M_y gives
    // u_z = -L²/(2EI_y), r_y = L/EI_y; neither depends on Ks.
    let (u, _) = solve(&model, 2, &[], &tip_force(2, RZ, 1.0));
    close(u[tip + UY], 0.5, 1e-15, "M_z: u_y");
    close(u[tip + RZ], 1.0, 1e-15, "M_z: r_z");
    let (u, _) = solve(&model, 2, &[], &tip_force(2, RY, 1.0));
    close(u[tip + UZ], -0.25, 1e-15, "M_y: u_z");
    close(u[tip + RY], 0.5, 1e-15, "M_y: r_y");

    // Rotation covariance: rotate nodes, y reference and loads by R.
    let rz = [[0.6, -0.8, 0.0], [0.8, 0.6, 0.0], [0.0, 0.0, 1.0]];
    let rx = [[1.0, 0.0, 0.0], [0.0, 0.6, -0.8], [0.0, 0.8, 0.6]];
    let mul = |a: [[f64; 3]; 3], b: [[f64; 3]; 3]| {
        let mut out = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                out[i][j] = (0..3).map(|k| a[i][k] * b[k][j]).sum();
            }
        }
        out
    };
    let rot = mul(rz, rx);
    let apply = |v: [f64; 3]| [0, 1, 2].map(|i| (0..3).map(|k| rot[i][k] * v[k]).sum::<f64>());
    let length = 2.0;
    let base: Vec<FrameElement> = run(section, length, 2);
    let rotated: Vec<FrameElement> = base
        .iter()
        .map(|e| {
            element(
                e.node_i.index,
                e.node_j.index,
                apply(e.node_i.coordinates),
                apply(e.node_j.coordinates),
                section,
                apply(e.y_reference),
            )
        })
        .collect();
    let (force, moment) = ([0.3, 1.0, -0.7], [0.2, -0.4, 0.9]);
    let mut f = vec![0.0; 18];
    let mut f_rot = vec![0.0; 18];
    for (base_dof, vector) in [(UX, force), (RX, moment)] {
        let turned = apply(vector);
        for k in 0..3 {
            f[12 + base_dof + k] = vector[k];
            f_rot[12 + base_dof + k] = turned[k];
        }
    }
    let (u, r) = solve(&base, 3, &[], &f);
    let (u_rot, r_rot) = solve(&rotated, 3, &[], &f_rot);
    let scale = u.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
    for node in 0..3 {
        for base_dof in [UX, RX] {
            let at = node * DOF_PER_NODE + base_dof;
            let expected = apply([u[at], u[at + 1], u[at + 2]]);
            let reaction = apply([r[at], r[at + 1], r[at + 2]]);
            for k in 0..3 {
                close(u_rot[at + k], expected[k], 1e-13 * scale, "rotated displacement");
                close(r_rot[at + k], reaction[k], 1e-12, "rotated reaction");
            }
        }
    }
}

/// SHEAR_REFERENCE §5 S-A, +1 N/m over the distal half at phi = 1, with the
/// frozen vectors: the exact Timoshenko feq [7/64, 13/384, 25/64, -19/384]
/// gives tip v 53/384, theta 7/48, root Fy -1/2, Mz -3/8; the stale
/// Euler–Bernoulli vector [3/32, 5/192, 13/32, -11/192] with the same
/// Timoshenko K gives v 9/64 = 54/384 and the same theta (the killed mutant:
/// rotation and balance alone cannot see it). K-D5 re-forms a Timoshenko
/// partial span as a ball enclosing this vector
/// (`kd5_timoshenko_partial_span_closes_and_passes`).
#[test]
fn timoshenko_distal_half_load_kills_the_stale_vector() {
    let model = run(abstract_section(1.0, 1.0, 12.0), 1.0, 1);
    let tip = DOF_PER_NODE;
    let nodal = |v: [f64; 4]| {
        let mut f = vec![0.0; 12];
        f[UY] = v[0];
        f[RZ] = v[1];
        f[tip + UY] = v[2];
        f[tip + RZ] = v[3];
        f
    };
    let exact = nodal([7.0 / 64.0, 13.0 / 384.0, 25.0 / 64.0, -19.0 / 384.0]);
    let stale = nodal([3.0 / 32.0, 5.0 / 192.0, 13.0 / 32.0, -11.0 / 192.0]);
    for f in [&exact, &stale] {
        close(f[UY] + f[tip + UY], 0.5, 1e-16, "force balance");
        close(f[RZ] + f[tip + UY] + f[tip + RZ], 3.0 / 8.0, 1e-16, "moment balance");
    }
    let (u, r) = solve(&model, 2, &[], &exact);
    close(u[tip + UY], 53.0 / 384.0, 1e-15, "tip v");
    close(u[tip + RZ], 7.0 / 48.0, 1e-15, "tip theta");
    close(r[UY], -0.5, 1e-15, "root Fy");
    close(r[RZ], -3.0 / 8.0, 1e-15, "root Mz");
    let (wrong, _) = solve(&model, 2, &[], &stale);
    close(wrong[tip + UY], 54.0 / 384.0, 1e-15, "stale tip v");
    close(wrong[tip + RZ], 7.0 / 48.0, 1e-15, "stale tip theta");
    assert!((wrong[tip + UY] - u[tip + UY]).abs() > 2e-3);
}

/// SHEAR_REFERENCE §5 S-E: a fixed-root cantilever (EI = 1, Ks = 12, L = 1)
/// with a ground spring k = 12 at a = 1/2 and a tip force F = 1, assembled
/// with FK from two elements. Timoshenko: R = -7/8, u(a) = 7/96,
/// u(L) = 37/128, theta(L) = 25/64, root force -1/8, moment -9/16.
/// Euler–Bernoulli: R = -5/6, u(a) = 5/72, u(L) = 71/288, theta(L) = 19/48,
/// root -1/6, -7/12. The root element's end actions are recovered with its
/// own (Timoshenko) matrix, in binary64 and by K-D5's certified recovery;
/// the Euler–Bernoulli matrix on the Timoshenko displacements (a
/// stiffness-only replacement) does not give them.
#[test]
fn timoshenko_recovered_end_actions_use_the_timoshenko_matrix() {
    use open_pipe_stress_frame_kernel::structural::{
        certify_straight_recovery, CertifiedMemberLoads, MemberBoundInputs,
    };
    let eb = FrameSection::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let spring = [(DOF_PER_NODE + UY, 12.0)];
    let force = tip_force(3, UY, 1.0);
    let bound = MemberBoundInputs {
        w: vec![0.0; 18],
        w2: vec![0.0; 18],
        exponents: vec![i32::MIN; 18],
        lambda_sigma: 0.0,
    };
    let root_actions = |e: &FrameElement, u: &[f64]| {
        // Along x with y reference y: local = global.
        let k = e.local_stiffness().unwrap();
        [UY, RZ].map(|r| (0..12).map(|c| k[r][c] * u[c]).sum::<f64>())
    };
    for (section, expected) in [
        (
            abstract_section(1.0, 1.0, 12.0),
            [-7.0 / 8.0, 7.0 / 96.0, 37.0 / 128.0, 25.0 / 64.0, -1.0 / 8.0, -9.0 / 16.0],
        ),
        (eb, [-5.0 / 6.0, 5.0 / 72.0, 71.0 / 288.0, 19.0 / 48.0, -1.0 / 6.0, -7.0 / 12.0]),
    ] {
        let [reaction, ua, ul, theta, root_force, root_moment] = expected;
        let what = format!("{:?}", section.beam_theory());
        let model = run(section, 1.0, 2);
        let (u, r) = solve(&model, 3, &spring, &force);
        close(u[DOF_PER_NODE + UY], ua, 1e-14, &format!("{what} u(a)"));
        close(-12.0 * u[DOF_PER_NODE + UY], reaction, 1e-14, &format!("{what} R"));
        close(u[2 * DOF_PER_NODE + UY], ul, 1e-14, &format!("{what} u(L)"));
        close(u[2 * DOF_PER_NODE + RZ], theta, 1e-14, &format!("{what} theta(L)"));
        close(r[UY], root_force, 1e-14, &format!("{what} support force"));
        close(r[RZ], root_moment, 1e-14, &format!("{what} support moment"));
        let [f, m] = root_actions(&model[0], &u);
        close(f, root_force, 1e-14, &format!("{what} root force"));
        close(m, root_moment, 1e-14, &format!("{what} root moment"));
        // T4-U8 (Design §4 item 2): the recovery is bound to the member's
        // formation-source frame (the model's own root element).
        let source = open_pipe_stress_frame_kernel::structural::FormationSource {
            node_count: 3,
            frames: model.to_vec(),
            frame_members: (0..model.len()).map(|k| format!("member:1:{k}:")).collect(),
            ..Default::default()
        };
        let loads = CertifiedMemberLoads::default().with_member_source(&source, "member:1:0:");
        let certified = certify_straight_recovery(&model[0], &loads, &u, &bound).unwrap();
        close(certified.ends[UY].published, root_force, 1e-14, &format!("{what} certified force"));
        close(certified.ends[RZ].published, root_moment, 1e-14, &format!("{what} certified moment"));
        if section.beam_theory().is_timoshenko() {
            let stale = element(0, 1, [0.0; 3], [0.5, 0.0, 0.0], eb, [0.0, 1.0, 0.0]);
            let [f, _] = root_actions(&stale, &u);
            assert!((f - root_force).abs() > 1e-2, "stiffness-only replacement: {f}");
        }
    }
}

// ------------------------------------------------- kappa (S-B and S-C)

/// SHEAR_REFERENCE §5 S-B/S-C (kills Cowper, solid-circle and 5/6 factors).
#[test]
fn timoshenko_kappa_is_energy_matched() {
    // S-B: ro .1, ri .09, nu .3: kappa_E = 16609827/33068756.
    let kappa = annulus_energy_shear_factor(0.09, 0.1, 0.3).unwrap();
    let selected = 16609827.0 / 33068756.0;
    assert!(ulps(kappa, selected) <= 4, "{kappa:e} vs {selected:e}");
    let cowper = 1277679.0 / 2397284.0;
    assert!((kappa - cowper).abs() > 0.03, "Cowper must not reproduce the selected factor");
    assert!((kappa - 5.0 / 6.0).abs() > 0.3);

    // The CONTRACT's expanded closed form, evaluated in another order.
    let expanded = |m: f64, nu: f64| {
        let s = (1.0 + m * m) * (1.0 + m * m);
        6.0 * (1.0 + nu) * (1.0 + nu) * s
            / ((7.0 + 14.0 * nu + 8.0 * nu * nu) * s + 4.0 * m * m * (5.0 + 10.0 * nu + 4.0 * nu * nu))
    };
    let cowper_form = |m: f64, nu: f64| {
        let s = (1.0 + m * m) * (1.0 + m * m);
        6.0 * (1.0 + nu) * s / ((7.0 + 6.0 * nu) * s + (20.0 + 12.0 * nu) * m * m)
    };
    // m = 0.9 and two nonzero nu (and an auxetic one): the selected form.
    for nu in [0.3, 0.49, -0.5] {
        let k = annulus_energy_shear_factor(0.9, 1.0, nu).unwrap();
        close(k, expanded(0.9, nu), 1e-14, "m = 0.9");
        assert!((k - cowper_form(0.9, nu)).abs() > 1e-3, "nu {nu}: Cowper differs");
    }
    // nu = 0: Cowper and energy coincide for every m.
    for m in [0.0, 0.25, 0.9] {
        close(annulus_energy_shear_factor(m, 1.0, 0.0).unwrap(), cowper_form(m, 0.0), 1e-15, "nu = 0");
    }
    // m -> 0: 6(1+nu)²/(7+14nu+8nu²) (the solid circle, ri = 0 admitted).
    for nu in [0.3, 0.1] {
        let solid = 6.0 * (1.0 + nu) * (1.0 + nu) / (7.0 + 14.0 * nu + 8.0 * nu * nu);
        close(annulus_energy_shear_factor(0.0, 1.0, nu).unwrap(), solid, 1e-15, "m = 0");
        close(annulus_energy_shear_factor(1e-9, 1.0, nu).unwrap(), solid, 1e-15, "m -> 0");
        // A genuinely hollow m rejects the solid-circle factor.
        assert!((annulus_energy_shear_factor(0.9, 1.0, nu).unwrap() - solid).abs() > 0.3);
    }
    // m -> 1: 1/2.
    close(annulus_energy_shear_factor(1.0 - 1e-9, 1.0, 0.3).unwrap(), 0.5, 1e-8, "m -> 1");
    // Independent integration of the Saint-Venant shear stress square
    // (INDEPENDENT_REFUTATION §2): kappa_E = 1/(A ∫(tau²/V²)dA), by composite
    // Simpson over ri <= r <= ro.
    for (ri, ro, nu) in [(0.09, 0.1, 0.3), (0.5, 1.0, 0.2), (0.25, 1.0, -0.3)] {
        let (a, b) = (ro, ri);
        let (s, p) = (a * a + b * b, a * a * b * b);
        let (c, e) = ((3.0 + 2.0 * nu) / 4.0, (1.0 + 2.0 * nu) / 4.0);
        let f = |r: f64| {
            let b0 = -c * s + r * r / 2.0;
            let b2 = c * p / (r * r) + e * r * r;
            (b0 * b0 + b2 * b2) * r
        };
        let n = 20_000;
        let h = (a - b) / n as f64;
        let mut radial = f(b) + f(a);
        for k in 1..n {
            radial += f(b + k as f64 * h) * if k % 2 == 1 { 4.0 } else { 2.0 };
        }
        radial *= h / 3.0;
        // With pi: integral = pi/(2(1+nu)² I²) * radial; A = pi(a²-b²),
        // I = pi(a⁴-b⁴)/4.
        let (area, inertia) = ((a * a - b * b), (a.powi(4) - b.powi(4)) / 4.0);
        let integral_without_pi = radial / (2.0 * (1.0 + nu) * (1.0 + nu) * inertia * inertia);
        let integrated = 1.0 / (area * integral_without_pi);
        close(annulus_energy_shear_factor(ri, ro, nu).unwrap(), integrated, 1e-9, "integrated");
    }
    // Unit scaling: m/mm and an exact 2^k scaling give the same factor.
    let mm = annulus_energy_shear_factor(90.0, 100.0, 0.3).unwrap();
    assert!(ulps(mm, kappa) <= 2);
    assert_eq!(
        annulus_energy_shear_factor(0.09 * 1024.0, 0.1 * 1024.0, 0.3).unwrap().to_bits(),
        kappa.to_bits()
    );
    // The fixed operation order (module documentation), reproduced exactly.
    let fixed = |ri: f64, ro: f64, nu: f64| {
        let m = ri / ro;
        let m2 = m * m;
        let a = 1.0 + m2;
        let b = 1.0 - m2;
        let n1 = 1.0 + nu;
        let c = (8.0 * nu + 14.0) * nu + 7.0;
        let d = c * (b * b) + 48.0 * (m2 * (n1 * n1));
        (6.0 * (n1 * n1)) * (a * a) / d
    };
    for (ri, ro, nu) in [(0.09, 0.1, 0.3), (0.0, 1.0, 0.3), (0.12, 0.2, 0.27), (0.9999, 1.0, -0.99)] {
        assert_eq!(
            annulus_energy_shear_factor(ri, ro, nu).unwrap().to_bits(),
            fixed(ri, ro, nu).to_bits()
        );
    }
    // A_s = kappa*A, one product.
    let area = 0.0019 * std::f64::consts::PI;
    assert_eq!(shear_area(kappa, area).unwrap(), kappa * area);
    // Refusals.
    for (ri, ro, nu) in [
        (f64::NAN, 1.0, 0.3),
        (0.5, f64::INFINITY, 0.3),
        (0.5, 1.0, f64::NAN),
    ] {
        assert!(matches!(
            annulus_energy_shear_factor(ri, ro, nu),
            Err(ShearFactorError::NonFiniteInput { .. })
        ));
    }
    for nu in [-1.0, 0.5, 0.75, -2.0] {
        assert_eq!(
            annulus_energy_shear_factor(0.5, 1.0, nu),
            Err(ShearFactorError::PoissonRatioOutOfRange { value: nu })
        );
    }
    for (ri, ro) in [(1.0, 1.0), (1.5, 1.0), (-0.1, 1.0)] {
        assert_eq!(
            annulus_energy_shear_factor(ri, ro, 0.3),
            Err(ShearFactorError::InnerRadiusOutOfRange {
                inner_radius: ri,
                outer_radius: ro
            })
        );
    }
    assert!(matches!(
        annulus_energy_shear_factor(0.0, 0.0, 0.3),
        Err(ShearFactorError::NonPositiveOuterRadius { .. })
    ));
    assert!(shear_area(kappa, 0.0).is_err() && shear_area(0.0, area).is_err());
    assert!(shear_area(f64::NAN, area).is_err() && shear_area(kappa, f64::INFINITY).is_err());
    assert!(matches!(
        shear_area(f64::MIN_POSITIVE, f64::MIN_POSITIVE),
        Err(ShearFactorError::NumericalRange { .. })
    ));
}

// ------------------------------------------- S-D: null space, patch, energy

/// SHEAR_REFERENCE §5 S-D: six rigid modes and no extra zero-energy mode, the
/// pure-curvature patch independent of A_s, and U = ½dᵀKd = ∫M²/2EI + V²/2Ks.
#[test]
fn timoshenko_null_space_patch_and_energy() {
    let kappa = annulus_energy_shear_factor(0.09, 0.1, 0.3).unwrap();
    let pi = std::f64::consts::PI;
    let (a, i) = (0.0019 * pi, 0.0000085975 * pi);
    let skew = |shear: f64| {
        let section = FrameSection::timoshenko(200e9, 1000e9 / 13.0, a, i, i, 2.0 * i, shear).unwrap();
        element(0, 1, [1.0, -2.0, 0.5], [2.0, 0.0, 2.5], section, [0.0, 0.0, 1.0])
    };
    let e = skew(kappa * a);
    let k = e.global_stiffness().unwrap();
    let scale = k.iter().flatten().fold(0.0_f64, |m, v| m.max(v.abs()));
    let (xi, xj) = (e.node_i.coordinates, e.node_j.coordinates);
    let d = [xj[0] - xi[0], xj[1] - xi[1], xj[2] - xi[2]];
    let mut modes = Vec::new();
    for axis in 0..3 {
        let mut t = [0.0; 12];
        t[axis] = 1.0;
        t[6 + axis] = 1.0;
        modes.push(t);
        let mut w = [0.0; 3];
        w[axis] = 1.0;
        let cross = [w[1] * d[2] - w[2] * d[1], w[2] * d[0] - w[0] * d[2], w[0] * d[1] - w[1] * d[0]];
        let mut r = [0.0; 12];
        for c in 0..3 {
            r[3 + c] = w[c];
            r[9 + c] = w[c];
            r[6 + c] = cross[c];
        }
        modes.push(r);
    }
    for mode in &modes {
        for row in &k {
            let action: f64 = row.iter().zip(mode).map(|(x, y)| x * y).sum();
            assert!(action.abs() <= 1e-12 * scale, "rigid mode action {action:e}");
        }
    }
    // No extra zero-energy mode: the 6x6 block with node i held is positive
    // definite (Cholesky with positive pivots).
    let mut m = [[0.0; 6]; 6];
    for r in 0..6 {
        for c in 0..6 {
            m[r][c] = k[6 + r][6 + c];
        }
    }
    for p in 0..6 {
        for q in 0..p {
            let factor = m[p][q] / m[q][q];
            for c in q..6 {
                m[p][c] -= factor * m[q][c];
            }
        }
        assert!(m[p][p] > 1e-9 * scale, "pivot {p}: {}", m[p][p]);
    }

    // Pure curvature d = [0, 0, cL²/2, cL] in the Iz plane: V0 = 0 and the
    // moment is EI·c whatever A_s (local axes along x).
    let c = 1e-4;
    for shear in [kappa * a, 10.0 * kappa * a, 0.1 * kappa * a] {
        let section = FrameSection::timoshenko(200e9, 1000e9 / 13.0, a, i, i, 2.0 * i, shear).unwrap();
        let length = 1.5;
        let k = local_stiffness(properties(section, length)).unwrap();
        let mut d = [0.0; 12];
        d[UY + DOF_PER_NODE] = c * length * length / 2.0;
        d[RZ + DOF_PER_NODE] = c * length;
        let f: Vec<f64> = k.iter().map(|row| row.iter().zip(&d).map(|(x, y)| x * y).sum()).collect();
        let moment = 200e9 * i * c;
        close(f[UY], 0.0, 1e-12 * moment, "patch V0 (i)");
        close(f[UY + DOF_PER_NODE], 0.0, 1e-12 * moment, "patch V0 (j)");
        close(f[RZ], -moment, 1e-12, "patch M (i)");
        close(f[RZ + DOF_PER_NODE], moment, 1e-12, "patch M (j)");
    }

    // Energy identity on the S-A cantilever (EI = 1, Ks = 12, L = 1, F = 1):
    // U = ½dᵀKd = F²L³/(6EI) + F²L/(2Ks) = 5/24.
    let model = run(abstract_section(1.0, 1.0, 12.0), 1.0, 1);
    let (u, _) = solve(&model, 2, &[], &tip_force(2, UY, 1.0));
    let k = assemble_global_stiffness(2, &model).unwrap();
    let energy: f64 = 0.5
        * (0..12)
            .map(|r| u[r] * (0..12).map(|c| k[r][c] * u[c]).sum::<f64>())
            .sum::<f64>();
    close(energy, 1.0 / 6.0 + 1.0 / 24.0, 1e-15, "energy");
}

/// S-D: the slender limit approaches Euler–Bernoulli smoothly (no locking:
/// the point-load compliance ratio is 1 + phi/4), and below phi = 2^-53 the
/// element is Euler–Bernoulli bit for bit; 1/2/4/8 elements agree.
#[test]
fn timoshenko_slender_limit_and_subdivision() {
    let eb = FrameSection::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let keb = local_stiffness(properties(eb, 1.0)).unwrap();
    let eb_tip = solve(&run(eb, 1.0, 1), 2, &[], &tip_force(2, UY, 1.0)).0[DOF_PER_NODE + UY];
    for phi in [1.0, 1e-2, 1e-6, 1e-12] {
        // phi = 12EI/(Ks L²) with EI = L = 1: Ks = 12/phi.
        let section = abstract_section(1.0, 1.0, 12.0 / phi);
        let tip = solve(&run(section, 1.0, 1), 2, &[], &tip_force(2, UY, 1.0)).0[DOF_PER_NODE + UY];
        close(tip / eb_tip, 1.0 + phi / 4.0, 1e-14, "compliance ratio");
        let k = local_stiffness(properties(section, 1.0)).unwrap();
        for r in 0..12 {
            for c in 0..12 {
                assert!((k[r][c] - keb[r][c]).abs() <= 2.0 * phi * 12.0, "[{r}][{c}] phi {phi}");
            }
        }
    }
    let k = local_stiffness(properties(abstract_section(1.0, 1.0, 2f64.powi(60)), 1.0)).unwrap();
    assert_eq!(k, keb, "phi = 12*2^-60 < 2^-53: the Euler–Bernoulli bits");

    // 1/2/4/8 exact-static elements: tip force and full-span uniform load
    // (each element its own full-span vector) give the same tip response
    // and root reactions.
    let section = abstract_section(1.0, 1.0, 12.0);
    for n in [1usize, 2, 4, 8] {
        let model = run(section, 1.0, n);
        let nodes = n + 1;
        let tip = n * DOF_PER_NODE;
        let (u, r) = solve(&model, nodes, &[], &tip_force(nodes, UY, 1.0));
        close(u[tip + UY], 5.0 / 12.0, 1e-14, "subdivided tip v");
        close(u[tip + RZ], 0.5, 1e-14, "subdivided tip theta");
        close(r[UY], -1.0, 1e-14, "subdivided root Fy");
        close(r[RZ], -1.0, 1e-14, "subdivided root Mz");
        let le = 1.0 / n as f64;
        let mut f = vec![0.0; nodes * DOF_PER_NODE];
        for k in 0..n {
            f[k * DOF_PER_NODE + UY] += le / 2.0;
            f[k * DOF_PER_NODE + RZ] += le * le / 12.0;
            f[(k + 1) * DOF_PER_NODE + UY] += le / 2.0;
            f[(k + 1) * DOF_PER_NODE + RZ] -= le * le / 12.0;
        }
        let (u, r) = solve(&model, nodes, &[], &f);
        close(u[tip + UY], 1.0 / 6.0, 1e-14, "subdivided uniform v");
        close(u[tip + RZ], 1.0 / 6.0, 1e-14, "subdivided uniform theta");
        close(r[UY], -1.0, 1e-14, "subdivided uniform root Fy");
        close(r[RZ], -0.5, 1e-14, "subdivided uniform root Mz");
    }
}

// ------------------------------------------------------------- K2a names

/// K2a: each reachable new Timoshenko intermediate is refused by name when it
/// leaves the binary64 normal range from nonzero finite operands; 2 - phi =
/// 0 is not an underflow.
#[test]
fn timoshenko_k2a_refuses_each_intermediate_by_name() {
    let refusal = |e: f64, g: f64, a: f64, iy: f64, iz: f64, j: f64, a_s: f64, l: f64| {
        let section = FrameSection::timoshenko(e, g, a, iy, iz, j, a_s).unwrap();
        match local_stiffness(properties(section, l)) {
            Err(FrameKernelError::NumericalRange { name }) => name,
            other => panic!("expected a range refusal, got {other:?}"),
        }
    };
    // G*As underflows (G*J, E*A in range).
    assert_eq!(refusal(1.0, 1e-200, 1.0, 1.0, 1.0, 1e200, 1e-200, 1.0), "phi: G*As");
    // (G*As)*L^2 overflows: G*As = 1e200, L^2 = 1e120.
    assert_eq!(refusal(1e-100, 1e-100, 1e100, 1.0, 1.0, 1e100, 1e300, 1e60), "phi: (G*As)*L^2");
    // 12*E overflows (E*A in range).
    assert_eq!(
        refusal(1e308, 1.0, 1e-10, 1.0, 1.0, 1.0, 1.0, 1.0),
        "phi and 12EI/(L^3(1+phi)): 12*E"
    );
    // (12*E)*Iy overflows.
    assert_eq!(
        refusal(1e300, 1.0, 1e-300, 1e10, 1.0, 1.0, 1.0, 1.0),
        "phi_y and 12EIy/(L^3(1+phi_y)): (12*E)*Iy"
    );
    // phi_y overflows: (12E)Iy = 1.2e300 over (G As) L^2 = 1e-10.
    assert_eq!(
        refusal(1e300, 1e-10, 1e-300, 1.0, 1.0, 1e10, 1.0, 1.0),
        "phi_y: ((12*E)*Iy)/((G*As)*L^2)"
    );
    // phi_z underflows while phi_y is in range.
    assert_eq!(
        refusal(1.0, 1.0, 1.0, 1.0, 1e-310, 1.0, 1.0, 1.0),
        "phi_z and 12EIz/(L^3(1+phi_z)): (12*E)*Iz"
    );
    assert_eq!(
        refusal(1.0, 1e300, 1.0, 1.0, 1e-20, 1e-300, 1.0, 1.0),
        "phi_z: ((12*E)*Iz)/((G*As)*L^2)"
    );
    // L^3*(1+phi_y) overflows: L^3 = 1e300, phi_y = 1.2e221.
    assert_eq!(
        refusal(1.0, 1e-210, 1e-100, 1e210, 1e210, 1e210, 1.0, 1e100),
        "12EIy/(L^3(1+phi_y)): L^3*(1+phi_y)"
    );
    // The coefficient 12EIy/(L^3(1+phi_y)) underflows (L = 1e100, phi tiny).
    assert_eq!(
        refusal(1.0, 1.0, 1e100, 1e-20, 1e-20, 1e-100, 1.0, 1e100),
        "12EIy/(L^3(1+phi_y)): ((12*E)*Iy)/(L^3*(1+phi_y))"
    );
    // E*Iy underflows while (12*E)*Iy and (6*E)*Iy stay normal.
    assert_eq!(
        refusal(1.0, 1.0, 1.0, 1e-308, 1.0, 1.0, 1.0, 1.0),
        "(4+phi_y)EIy/(L(1+phi_y)) and (2-phi_y)EIy/(L(1+phi_y)): E*Iy"
    );
    // (2-phi)*(E*I) is checked for finiteness only, so a tiny nonzero
    // (2-phi_y)*(E*Iy) passes and the coefficient (2-phi_y)EIy/(L(1+phi_y))
    // is refused by name when it underflows.
    let (e, iy) = (1e-295, 1.0 / 3.0);
    let twelve_ei = (12.0 * e) * iy;
    let g = twelve_ei / (2.0 - 2f64.powi(-50));
    let section = FrameSection::timoshenko(e, g, 1.0, iy, 1.0, 1.0, 1.0).unwrap();
    let phi_y = properties(section, 1.0).timoshenko_phi().unwrap().unwrap()[0];
    let two_minus_phi = 2.0 - phi_y;
    assert!(two_minus_phi != 0.0 && two_minus_phi.abs() < 1e-14, "precondition: {two_minus_phi:e}");
    assert!((two_minus_phi * (e * iy)).is_subnormal(), "precondition: a subnormal numerator");
    assert_eq!(
        refusal(e, g, 1.0, iy, 1.0, 1.0, 1.0, 1.0),
        "(2-phi_y)EIy/(L(1+phi_y)): ((2-phi_y)*(E*Iy))/(L*(1+phi_y))"
    );
    // An exact zero 2 - phi and its zero product are accepted: phi_y = 2.
    let section = abstract_section(2.0, 1.0, 12.0);
    let k = local_stiffness(properties(section, 1.0)).unwrap();
    assert_eq!(k[RY][RY + DOF_PER_NODE].to_bits(), 0.0_f64.to_bits());
}

/// The constructors: `new` stays Euler–Bernoulli; `timoshenko` checks A_s.
#[test]
fn timoshenko_section_constructors_and_leaf() {
    let eb = FrameSection::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    assert_eq!(eb.beam_theory(), BeamTheory::EulerBernoulli);
    assert_eq!(eb.shear_area(), None);
    let ts = FrameSection::timoshenko(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5).unwrap();
    assert_eq!(ts.beam_theory(), BeamTheory::Timoshenko { shear_area: 0.5 });
    assert_eq!(ts.shear_area(), Some(0.5));
    assert!(ts.beam_theory().is_timoshenko() && !eb.beam_theory().is_timoshenko());
    for bad in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(FrameSection::timoshenko(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, bad).is_err());
        // A literal leaf is checked at formation.
        let literal = FrameSection {
            beam_theory: BeamTheory::Timoshenko { shear_area: bad },
            ..eb
        };
        assert!(local_stiffness(properties(literal, 1.0)).is_err());
        assert!(properties(literal, 1.0).timoshenko_phi().is_err());
    }
    assert_eq!(properties(eb, 1.0).timoshenko_phi().unwrap(), None);
    assert_eq!(std::mem::size_of::<FrameSection>(), 64);
    assert_eq!(std::mem::size_of::<FrameElement>(), 152);
}

// --------------------------------------------------------------- K2b

/// K2b (Design §4 item 3): A_s is an area, so phi is bit-invariant under the
/// exact 2^b scaling of E and G and the formed K is 2^b times K; the census
/// uses the actual phi, excludes an exact zero 2 - phi, and its b forms a
/// Timoshenko element that is out of range at b = 0.
#[test]
fn timoshenko_k2b_phi_is_scale_invariant_and_the_census_holds() {
    let kappa = annulus_energy_shear_factor(0.09, 0.1, 0.3).unwrap();
    let pi = std::f64::consts::PI;
    let (a, i) = (0.0019 * pi, 0.0000085975 * pi);
    let section = FrameSection::timoshenko(200e9, 1000e9 / 13.0, a, i, 2.0 * i, 3.0 * i, kappa * a).unwrap();
    let e = element(0, 1, [0.0, 0.0, 0.0], [1.5, 0.5, -0.25], section, [0.0, 0.0, 1.0]);
    let phi = e.properties().unwrap().timoshenko_phi().unwrap().unwrap();
    let k0 = e.local_stiffness().unwrap();
    for b in [-200, -40, 2, 40, 200] {
        let scaled = e.force_scaled(ForceScale::new(b).unwrap()).unwrap();
        assert_eq!(scaled.section.shear_area(), Some(kappa * a), "A_s is not scaled");
        let phi_b = scaled.properties().unwrap().timoshenko_phi().unwrap().unwrap();
        assert_eq!(phi_b.map(f64::to_bits), phi.map(f64::to_bits), "b = {b}");
        let kb = scaled.local_stiffness().unwrap();
        for r in 0..12 {
            for c in 0..12 {
                assert_eq!(kb[r][c], k0[r][c] * 2f64.powi(b), "[{r}][{c}] b = {b}");
            }
        }
    }
    // The census phi is the formation's.
    let l2 = e.length().unwrap() * e.length().unwrap();
    assert_eq!(ForceScaleCensus::timoshenko_phi(section, i, l2), Some(phi[0]));
    assert_eq!(ForceScaleCensus::timoshenko_phi(section, 2.0 * i, l2), Some(phi[1]));
    assert_eq!(
        ForceScaleCensus::timoshenko_phi(FrameSection::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0).unwrap(), 1.0, 1.0),
        None
    );
    // Every nonzero coefficient lies within the census window's margins.
    let mut census = ForceScaleCensus::new();
    census.frame(&e).unwrap();
    let (low, high) = census.span().unwrap();
    for value in k0.iter().flatten().filter(|v| **v != 0.0) {
        let exponent = value.abs().log2().floor() as i32;
        assert!(exponent >= low - 3 && exponent <= high + 2, "{value:e} outside [{low}, {high}]");
    }

    // An exact zero 2 - phi (phi_y = 2) is excluded: the span stays that of
    // the nonzero values.
    let zero = element(0, 1, [0.0; 3], [1.0, 0.0, 0.0], abstract_section(2.0, 1.0, 12.0), [0.0, 1.0, 0.0]);
    let mut census = ForceScaleCensus::new();
    census.frame(&zero).unwrap();
    let (low, high) = census.span().unwrap();
    assert!(low > -10 && high < 10, "[{low}, {high}]");

    // A Timoshenko element out of range at b = 0 ((12E)I overflows) forms at
    // the census b, and its phi there is the census phi.
    let huge = FrameSection::timoshenko(1e300, 4e299, 1e-6, 2e10, 3e10, 1e-6, 1e9).unwrap();
    let e = element(0, 1, [0.0; 3], [10.0, 0.0, 0.0], huge, [0.0, 1.0, 0.0]);
    assert!(matches!(e.local_stiffness(), Err(FrameKernelError::NumericalRange { .. })));
    let mut census = ForceScaleCensus::new();
    census.frame(&e).unwrap();
    let scale = census.force_scale().unwrap();
    assert!(scale.exponent() < 0);
    let scaled = e.force_scaled(scale).unwrap();
    assert!(scaled.local_stiffness().is_ok());
    let phi = scaled.properties().unwrap().timoshenko_phi().unwrap().unwrap();
    assert_eq!(ForceScaleCensus::timoshenko_phi(huge, 2e10, 100.0), Some(phi[0]));
    assert_eq!(ForceScaleCensus::timoshenko_phi(huge, 3e10, 100.0), Some(phi[1]));
}

/// The binary exponent of a normal binary64 value, read from its bits.
fn binary_exponent(value: f64) -> i32 {
    assert!(value.is_normal(), "{value:e}");
    ((value.abs().to_bits() >> 52) & 0x7ff) as i32 - 1023
}

/// The census span of one element, after checking that every nonzero
/// coefficient of its formed K lies within the span's margins.
fn census_span(element: &FrameElement) -> (i32, i32) {
    let mut census = ForceScaleCensus::new();
    census.frame(element).unwrap();
    let (low, high) = census.span().unwrap();
    for value in element.local_stiffness().unwrap().iter().flatten().filter(|v| **v != 0.0) {
        let exponent = binary_exponent(*value);
        assert!(exponent >= low - 3 && exponent <= high + 2, "{value:e} outside [{low}, {high}]");
    }
    (low, high)
}

/// K2b (Design §4 item 3; final review FINAL-A, F4): the census takes the
/// exponents of 1+phi, 4+phi and 2-phi from the actual binary64 phi, never
/// from the leaf exponents (1+phi as 1, 4+phi as 4, 2-phi as 2). With
/// E = G = A = J = 1 every census entry is a sum of the exponents listed in
/// the comments; each element puts one of those factors at an extreme of the
/// span, so substituting its leaf-predicted exponent moves the span.
#[test]
fn timoshenko_k2b_census_uses_the_actual_phi_exponents() {
    let along_x = |section: FrameSection, length: f64| {
        element(0, 1, [0.0; 3], [length, 0.0, 0.0], section, [0.0, 1.0, 0.0])
    };
    let phi = |e: &FrameElement| e.properties().unwrap().timoshenko_phi().unwrap().unwrap();

    // (a) phi within ulps of 2 on both sides: phi_y = 2 - 2^-50 and phi_z =
    // 2 + 2^-49 exactly (A_s = 12, L = 1). 2-phi_y has exponent -50 and
    // 1+phi_y exponent 1, so (2-phi_y)EIy/(L(1+phi_y)) predicts -51, the
    // span's low; the span's high is (12E)Iz, 3 + e(Iz) = 4. The leaf value
    // of 2-phi (exponent 1) would put the low at 0, above the formed
    // k_ry,ry' (2^-50 (2 - 2^-50) / (3 - 2^-50), exponent -51) by more than
    // the margin.
    let near_two = along_x(abstract_section(2.0 - 2f64.powi(-50), 2.0 + 2f64.powi(-49), 12.0), 1.0);
    let [phi_y, phi_z] = phi(&near_two);
    assert_eq!((phi_y, phi_z), (2.0 - 2f64.powi(-50), 2.0 + 2f64.powi(-49)), "precondition");
    assert_eq!((binary_exponent(2.0 - phi_y), binary_exponent(2.0 - phi_z)), (-50, -49));
    assert_eq!(binary_exponent(1.0 + phi_y), 1);
    assert_eq!(census_span(&near_two), (-51, 4));
    let k = near_two.local_stiffness().unwrap();
    assert_eq!(binary_exponent(k[RY][RY + DOF_PER_NODE]), -51, "the formed (2-phi_y) coefficient");
    // The census shifts exactly with the 2^b scaling of E and G: phi does
    // not move, and every entry carries exactly one of e(E), e(G).
    for b in [-40, 40] {
        let scaled = near_two.force_scaled(ForceScale::new(b).unwrap()).unwrap();
        assert_eq!(phi(&scaled).map(f64::to_bits), [phi_y, phi_z].map(f64::to_bits));
        assert_eq!(census_span(&scaled), (-51 + b, 4 + b), "b = {b}");
    }

    // (b) Large phi across a power of two: phi_y = 2^40 - 2 (Iy = 2^40 - 2,
    // A_s = 12, L = 1), phi_z = 1. 4+phi_y has exponent 40 and 2-phi_y 39, so
    // the high is (4+phi_y)(E Iy) at 40 + 39 = 79, alone; the leaf value of
    // 4+phi (exponent 2) would leave (2-phi_y)(E Iy) at 78. The low is
    // (2-phi_z)EIz/(L(1+phi_z)) at 0 - 1 = -1.
    let across = along_x(abstract_section(2f64.powi(40) - 2.0, 1.0, 12.0), 1.0);
    let [phi_y, phi_z] = phi(&across);
    assert_eq!((phi_y, phi_z), (2f64.powi(40) - 2.0, 1.0), "precondition");
    assert_eq!((binary_exponent(4.0 + phi_y), binary_exponent(2.0 - phi_y)), (40, 39));
    assert_eq!(census_span(&across), (-1, 79));

    // (c) Large phi on a short member: Iy = Iz = 1, A_s = 2^-20, L = 2^-10, so
    // phi = 12 * 2^40 = 1.5 * 2^43 and 1+phi, 4+phi and 2-phi all have
    // exponent 43. The high is (4+phi)(EI) = 43 and the low (G A_s)L^2 =
    // -40; the coefficients divided by L^n(1+phi) predict -10, -21 and 10.
    // The leaf value of 1+phi (exponent 0) would predict them as 33, 22 and
    // 53, moving the high to 53.
    let short = FrameSection::timoshenko(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2f64.powi(-20)).unwrap();
    let short = along_x(short, 2f64.powi(-10));
    let [phi_y, phi_z] = phi(&short);
    assert_eq!((phi_y, phi_z), (1.5 * 2f64.powi(43), 1.5 * 2f64.powi(43)), "precondition");
    assert_eq!(binary_exponent(1.0 + phi_y), 43);
    assert_eq!(census_span(&short), (-40, 43));
}

// --------------------------------------------- the W1 retained assembler

/// Design §3.6 and O4 §4.2 (DEL-04-08 `o4-source-extension.md`):
/// `StraightMember::from_frame` refuses a Timoshenko leaf by name; the
/// retained Timoshenko member is formed only with its sidecar entry
/// (`from_frame_with_leaf`), and the text stays the association refusal.
#[test]
fn retained_member_refuses_a_timoshenko_leaf_by_name() {
    let eb = FrameSection::new(2.0, 1.0, 3.0, 4.0, 5.0, 6.0).unwrap();
    let frame = element(3, 7, [0.0; 3], [1.0, 2.0, 3.0], eb, [0.0, 0.0, 1.0]);
    let member = StraightMember::from_frame(9, &frame).unwrap();
    assert_eq!(
        (member.id, member.node_i, member.node_j, member.elastic_modulus, member.shear_modulus),
        (9, 3, 7, 2.0, 1.0)
    );
    assert_eq!(
        (member.area, member.second_moment_y, member.second_moment_z, member.torsion_constant),
        (3.0, 4.0, 5.0, 6.0)
    );
    assert_eq!(member.y_reference, [0.0, 0.0, 1.0]);
    let ts = FrameSection::timoshenko(2.0, 1.0, 3.0, 4.0, 5.0, 6.0, 1.5).unwrap();
    let frame = element(3, 7, [0.0; 3], [1.0, 2.0, 3.0], ts, [0.0, 0.0, 1.0]);
    assert_eq!(StraightMember::from_frame(9, &frame), Err(RetainedMemberRefusal::Timoshenko));
    assert_eq!(RetainedMemberRefusal::Timoshenko.reason(), RETAINED_TIMOSHENKO_REFUSAL);
    let (with_leaf, leaf) = StraightMember::from_frame_with_leaf(9, &frame).unwrap();
    assert_eq!(with_leaf, member);
    assert_eq!(leaf, Some(TimoshenkoLeaf { member: 9, shear_area: 1.5 }));
    assert_eq!(TimoshenkoLeafError::MissingMember(9).reason(), RETAINED_TIMOSHENKO_REFUSAL);
    assert_eq!(
        RETAINED_TIMOSHENKO_REFUSAL,
        "timoshenko_annulus_energy_v1 straight members are not a retained-source formulation"
    );
    let _ = (RY, UZ, UX);
}
