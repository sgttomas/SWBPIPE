//! Product against the independent free-body/virtual-work oracle, not
//! against the producer's own closed forms. Inputs and results are public,
//! invented cases in validation/references/curved_bend_shear.
use super::*;

struct OracleCase {
    name: &'static str,
    xi: [f64; 3],
    xj: [f64; 3],
    reference: [f64; 3],
    radius: f64,
    kin: f64,
    kout: f64,
    load: [f64; 3],
    f_eb: Matrix6,
    f_v2: Matrix6,
    k_v2: Matrix12,
    p_v2: [f64; 12],
    d_v2: [f64; 6],
}

struct SweepCase {
    name: &'static str,
    xi: [f64; 3],
    xj: [f64; 3],
    reference: [f64; 3],
    radius: f64,
    f: [[f64; 3]; 3],
    d: [[f64; 3]; 3],
}

mod reference {
    include!("../../../../validation/references/curved_bend_shear/kernel_cases.rs");
}

fn close(name: &str, actual: f64, expected: f64, scale: f64) {
    assert!(
        (actual - expected).abs() <= 2e-11 * scale,
        "{name}: actual={actual:e}, expected={expected:e}, scale={scale:e}"
    );
}

fn element(c: &OracleCase) -> CurvedBendMacroElement {
    CurvedBendMacroElement::new(
        FrameNode::new(0, c.xi).unwrap(),
        FrameNode::new(1, c.xj).unwrap(),
        c.radius,
        c.reference,
        200_000_000_000.0,
        80_000_000_000.0,
        0.0078125,
        0.000030517578125,
        0.00006103515625,
        c.kin,
        c.kout,
    )
    .unwrap()
}

#[test]
fn transverse_shear_stiffness_and_consistent_load_match_independent_oracle() {
    for c in reference::CASES {
        let eb = element(c);
        assert_eq!(eb.transverse_shear_area, None);
        let v2 = eb.with_transverse_shear_area(0.00390625).unwrap();
        for (actual, expected) in [
            (eb.end_flexibility().unwrap(), c.f_eb),
            (v2.end_flexibility().unwrap(), c.f_v2),
        ] {
            for i in 0..6 {
                for j in 0..6 {
                    // The two action blocks have no cross terms. The
                    // independent decimal geometry may leave a tiny residue;
                    // bound it absolutely, while the product zero is exact.
                    let in_plane = |n| matches!(n, 0 | 1 | 5);
                    if in_plane(i) != in_plane(j) {
                        assert_eq!(actual[i][j], 0.0);
                        assert!(expected[i][j].abs() <= 1e-65);
                    } else {
                        close(c.name, actual[i][j], expected[i][j], expected[i][j].abs());
                    }
                }
            }
        }
        let k = v2.global_stiffness().unwrap();
        for i in 0..12 {
            let scale = c.k_v2[i].iter().fold(0.0_f64, |a, x| a.max(x.abs()));
            for j in 0..12 {
                close(c.name, k[i][j], c.k_v2[i][j], scale);
            }
        }
        let f = v2.end_flexibility().unwrap();
        let inverse = v2.tip_stiffness().unwrap();
        for i in 0..6 {
            for j in 0..6 {
                let mut product = 0.0;
                let mut scale = 0.0;
                for n in 0..6 {
                    product += f[i][n] * inverse[n][j];
                    scale += (f[i][n] * inverse[n][j]).abs();
                }
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (product - expected).abs() <= 512.0 * f64::EPSILON * scale.max(1.0),
                    "{} inverse residual [{i}][{j}]: {product:e}, scale {scale:e}",
                    c.name
                );
            }
        }
        let p = v2.consistent_uniform_nodal_loads(c.load).unwrap();
        let scale = c.p_v2.iter().fold(0.0_f64, |a, x| a.max(x.abs()));
        for i in 0..12 {
            close(c.name, p[i], c.p_v2[i], scale);
        }
        let geometry = v2.geometry().unwrap();
        let local = rotate_to_local(&geometry.local_axes, c.load);
        let d = v2.tip_deflection_under_uniform_load(&geometry, local);
        let scale = c.d_v2.iter().fold(0.0_f64, |a, x| a.max(x.abs()));
        for i in 0..6 {
            close(c.name, d[i], c.d_v2[i], scale);
        }
        // Opt-in must not mutate the original EB element's bytes/results.
        assert_eq!(eb, element(c));
        assert_eq!(
            eb.global_stiffness().unwrap(),
            element(c).global_stiffness().unwrap()
        );
    }
}

#[test]
fn transverse_shear_small_angle_switch_and_straight_limit_match_oracle() {
    for c in reference::SWEEPS {
        // Isolate shear without subtracting nearly equal flexibility
        // values. The other energy compliances are at most 1e-100.
        let m = CurvedBendMacroElement::new(
            FrameNode::new(0, c.xi).unwrap(),
            FrameNode::new(1, c.xj).unwrap(),
            c.radius,
            c.reference,
            1e100,
            1.0,
            1.0,
            1.0,
            1e100,
            1.0,
            1.0,
        )
        .unwrap()
        .with_transverse_shear_area(1.0)
        .unwrap();
        let f = m.end_flexibility().unwrap();
        for i in 0..3 {
            for j in 0..3 {
                close(
                    c.name,
                    f[i][j] / c.radius,
                    c.f[i][j],
                    c.f[i][j].abs().max(1e-90),
                );
            }
        }
        let geometry = m.geometry().unwrap();
        for j in 0..3 {
            let mut w = [0.0; 3];
            w[j] = 1.0;
            let d = m.tip_deflection_under_uniform_load(&geometry, w);
            for i in 0..3 {
                close(
                    c.name,
                    d[i] / (c.radius * c.radius),
                    c.d[i][j],
                    c.d[i][j].abs().max(1e-90),
                );
            }
        }
    }
}

#[test]
fn transverse_shear_area_refuses_nonpositive_and_nonfinite_values() {
    let eb = element(&reference::CASES[0]);
    for area in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(eb.with_transverse_shear_area(area).is_err());
    }
    assert_eq!(eb.transverse_shear_area, None);
}
