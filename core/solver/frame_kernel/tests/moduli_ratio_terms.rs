//! `shear::moduli_ratio_terms`: x = (2*G)/E and t = 1 - x, the moduli terms
//! shared by the pressure-free transverse-shear maximum (DEL-04-07 Design
//! `u4b-u5b-implementation.md` §2.4) and formed without 1 + nu.

use open_pipe_stress_frame_kernel::shear::{moduli_ratio_terms, ShearFactorError};

const U: f64 = f64::EPSILON / 2.0;

/// The rounding error of a + b, exactly (TwoSum).
fn two_sum_error(a: f64, b: f64) -> f64 {
    let s = a + b;
    let bv = s - a;
    (a - (s - bv)) + (b - bv)
}

fn pairs() -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for e in [200e9, 1e9, 7.3e4, 1.0, 3e-7] {
        for ratio in [
            1.4, 1.25, 1.0, 0.75, 0.6, 0.5, 0.49, 0.4, 0.25, 0.1, 1e-3, 1e-8, 1e-16,
        ] {
            out.push((e, e / (2.0 * ratio)));
        }
    }
    out.push((1e9, 6.755399441055744e24));
    out.push((1e-150, 5e149));
    out
}

#[test]
fn x_is_one_rounding_and_t_is_exact_on_the_sterbenz_range() {
    let mut sterbenz = 0;
    for (e, g) in pairs() {
        let (x, t) = moduli_ratio_terms(e, g).unwrap();
        assert_eq!(
            x.to_bits(),
            ((2.0 * g) / e).to_bits(),
            "x = (2*G)/E for E = {e}, G = {g}"
        );
        assert_eq!(t.to_bits(), (1.0 - x).to_bits(), "t = 1 - x");
        // x's residual against 2G/E, exact by fma: |x - 2G/E| <= u*(2G/E).
        let residual = (-x).mul_add(e, 2.0 * g) / e;
        assert!(residual.abs() <= U * x * (1.0 + 4.0 * U), "x: {e} {g}");
        if (0.5..=2.0).contains(&x) {
            assert_eq!(two_sum_error(1.0, -x), 0.0, "t exact for x = {x}");
            sterbenz += 1;
        } else if x > 2.0 {
            // t* = 1 - 2G/E = t + err - residual, err = (1 - x) - t exactly, so
            // t - t* = residual - err; |1 - x| >= x/2 bounds it by 3u|t|.
            let deviation = residual - two_sum_error(1.0, -x);
            assert!(
                deviation.abs() <= 3.0 * U * t.abs(),
                "t within 3u for x = {x}"
            );
        }
    }
    assert!(sterbenz >= 20, "{sterbenz} Sterbenz pairs");
}

#[test]
fn codex_pair_terms() {
    // E/(2G) = 1/(3*2^52): x = 3*2^52 exactly, t = 1 - 3*2^52 exactly.
    let (x, t) = moduli_ratio_terms(1e9, 6.755399441055744e24).unwrap();
    assert_eq!(x, 3.0 * 2f64.powi(52));
    assert_eq!(t, 1.0 - 3.0 * 2f64.powi(52));
}

#[test]
fn moduli_ratio_terms_refusals() {
    for (e, g) in [
        (f64::NAN, 1.0),
        (1.0, f64::INFINITY),
        (f64::NEG_INFINITY, 1.0),
    ] {
        assert!(
            matches!(
                moduli_ratio_terms(e, g),
                Err(ShearFactorError::NonFiniteInput { .. })
            ),
            "{e} {g}"
        );
    }
    for (e, g, name) in [
        (0.0, 1.0, "elastic_modulus"),
        (-1.0, 1.0, "elastic_modulus"),
        (1.0, 0.0, "shear_modulus"),
        (1.0, -2.0, "shear_modulus"),
    ] {
        assert!(
            matches!(moduli_ratio_terms(e, g), Err(ShearFactorError::NonPositiveModulus { name: n, .. }) if n == name),
            "{e} {g}"
        );
    }
    // 2*G overflows; x underflows.
    assert!(matches!(
        moduli_ratio_terms(1.0, f64::MAX),
        Err(ShearFactorError::NumericalRange { .. })
    ));
    assert!(matches!(
        moduli_ratio_terms(f64::MAX, 1e-300),
        Err(ShearFactorError::NumericalRange { .. })
    ));
    assert_eq!(
        ShearFactorError::NonPositiveModulus {
            name: "shear_modulus",
            value: -2.0
        }
        .to_string(),
        "shear_modulus must be positive, got -2"
    );
}
