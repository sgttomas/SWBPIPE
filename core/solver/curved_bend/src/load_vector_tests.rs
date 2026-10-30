//! T4-U1b (T4-RV8 C-1): the uniform-load vector at small included angles.
//! The reference is RV129's independent 120-digit method (Gauss–Legendre
//! quadrature of the 3D force method; no closed form shared with this
//! crate), frozen by T4-I17 in `validation/references/t4_i9/`
//! (`arc_load_references.txt`, generator `t4_i17_arc_refs.py`).
use super::*;
use open_pipe_stress_frame_kernel::FrameNode;

const REFERENCES: &str =
    include_str!("../../../../validation/references/t4_i9/arc_load_references.txt");

fn hex(token: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(token, 16).expect(token))
}

struct Case {
    label: String,
    args: Vec<f64>,
    reference: Vec<f64>,
}

fn cases() -> Vec<Case> {
    let mut out = Vec::new();
    let mut lines = REFERENCES.lines().filter(|l| !l.starts_with('#'));
    while let Some(head) = lines.next() {
        let label = head.strip_prefix("case ").expect(head).to_string();
        assert!(lines.next().unwrap().starts_with("phi "));
        let args: Vec<f64> = lines.next().unwrap()["args ".len()..]
            .split_whitespace()
            .map(hex)
            .collect();
        let mut reference = Vec::new();
        for _ in 0..12 {
            let f: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
            // The leading term carries the reference to binary64 precision.
            reference.push(hex(f[2]));
        }
        assert_eq!(lines.next(), Some("end"));
        out.push(Case {
            label,
            args,
            reference,
        });
    }
    out
}

fn element(a: &[f64]) -> Result<CurvedBendMacroElement, CurvedBendError> {
    CurvedBendMacroElement::new(
        FrameNode::new(0, [a[0], a[1], a[2]]).unwrap(),
        FrameNode::new(1, [a[3], a[4], a[5]]).unwrap(),
        a[6],
        [a[7], a[8], a[9]],
        a[10],
        a[11],
        a[12],
        a[13],
        a[14],
        a[15],
        a[16],
    )
}

/// max_i |v_i − f_i| / max_i |f_i|.
fn relative_defect(v: &[f64; ELEMENT_DOF], f: &[f64]) -> f64 {
    let scale = f.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
    v.iter()
        .zip(f)
        .fold(0.0_f64, |m, (a, b)| m.max((a - b).abs()))
        / scale
}

/// C-1: on a 0.3 m chord (RV8's body) from 1e-8 rad to 90°, in-plane and
/// out-of-plane, CB's binary64 vector agrees with the independent reference
/// to a few units of binary64 rounding of max|f|. Before T4-U1b the in-plane
/// defect was 1.3e-2 at 1e-4 rad (T4-RV8 C-1).
#[test]
fn stable_load_vector_against_the_independent_reference() {
    let mut checked = 0;
    for case in cases()
        .iter()
        .filter(|c| c.label.starts_with("C-1 chord 0.3 m"))
    {
        let bend = element(&case.args).unwrap();
        let v = bend
            .consistent_uniform_nodal_loads([case.args[17], case.args[18], case.args[19]])
            .unwrap();
        let defect = relative_defect(&v, &case.reference);
        println!("{}: max|v - f|/max|f| = {defect:.2e}", case.label);
        assert!(defect <= 1e-13, "{}: {defect:e}", case.label);
        checked += 1;
    }
    assert_eq!(checked, 18);
}

/// Every reference case CB admits at k ≤ 30 (the RV129 angle, offset, skew,
/// scale and random sets): the same accuracy class. Large-k cases are not
/// asserted here: their binary64 inverse loses digits like cond(F)·u, which
/// the certificate catches (T15d).
#[test]
fn load_vector_accuracy_over_the_reference_set() {
    let mut checked = 0;
    let mut worst = (0.0_f64, String::new());
    for case in cases() {
        let Ok(bend) = element(&case.args) else {
            continue;
        };
        let Ok(v) =
            bend.consistent_uniform_nodal_loads([case.args[17], case.args[18], case.args[19]])
        else {
            continue;
        };
        let defect = relative_defect(&v, &case.reference);
        let k = case.args[15].max(case.args[16]);
        println!("{} (k = {k:e}): {defect:.2e}", case.label);
        if k <= 30.0 && !case.label.starts_with("near pi") {
            if defect > worst.0 {
                worst = (defect, case.label.clone());
            }
            checked += 1;
        }
    }
    println!("worst at k <= 30: {:.2e} ({})", worst.0, worst.1);
    assert!(checked >= 50, "{checked}");
    assert!(worst.0 <= 1e-12, "{worst:?}");
}

/// The switch at s = 1/2 is continuous to rounding: just below it, the series
/// path and the Gram path agree.
#[test]
fn series_and_gram_deflections_agree_at_the_switch() {
    let (ro, ri) = (0.084_f64, 0.077_f64);
    let area = PI * (ro * ro - ri * ri);
    let inertia = PI * (ro.powi(4) - ri.powi(4)) / 4.0;
    for radius in [0.2286, 1.0, 25.0] {
        let s = SERIES_SWITCH_HALF_SINE * (1.0 - f64::EPSILON);
        let chord = 2.0 * radius * s;
        let bend = CurvedBendMacroElement::new(
            FrameNode::new(0, [1.0, 2.0, 3.0]).unwrap(),
            FrameNode::new(1, [1.0 + chord, 2.0, 3.0]).unwrap(),
            radius,
            [0.0, 1.0, 0.0],
            200e9,
            80e9,
            area,
            inertia,
            2.0 * inertia,
            1.7,
            0.6,
        )
        .unwrap();
        let geometry = bend.geometry().unwrap();
        assert!(geometry.half_angle().use_series());
        for w in [
            [0.0, 0.0, 450.0],
            [0.0, -450.0, 0.0],
            [300.0, 0.0, 0.0],
            [12.0, -34.0, 56.0],
        ] {
            let series = bend.tip_deflection_by_series(&geometry, w);
            let gram = bend.tip_deflection_by_gram(&geometry, w);
            let scale = gram.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
            for (a, b) in series.iter().zip(&gram) {
                assert!(
                    (a - b).abs() <= 1e-13 * scale,
                    "R {radius} w {w:?}: {a:e} vs {b:e}"
                );
            }
        }
    }
}

/// Above the switch the deflection is the Gram path, bit for bit (the T4-U1
/// vector is unchanged for φ ≥ π/3, e.g. T15's 90° body).
#[test]
fn gram_path_is_kept_above_the_switch() {
    let case = cases()
        .into_iter()
        .find(|c| c.label == "T15 body k=2")
        .unwrap();
    let bend = element(&case.args).unwrap();
    let geometry = bend.geometry().unwrap();
    assert!(!geometry.half_angle().use_series());
    let w = rotate_to_local(&geometry.local_axes, [0.0, 0.0, 0.3]);
    assert_eq!(
        bend.tip_deflection_under_uniform_load(&geometry, w),
        bend.tip_deflection_by_gram(&geometry, w)
    );
}
