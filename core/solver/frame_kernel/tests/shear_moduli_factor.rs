//! `shear::annulus_energy_shear_factor_of_moduli` (DEL-05-03 Design
//! `pf-timoshenko.md` §2): its fixed order bit for bit, its documented bound
//! |kappa/kappa* - 1| <= (21 + 6 beta) u against the exact contract value on a
//! grid of E/(2G) from 1.4 to 1e-300 (`shear_moduli_vectors.py`), its refusals,
//! and agreement with `annulus_energy_shear_factor` where 1 + nu is exact.
use open_pipe_stress_frame_kernel::shear::{
    annulus_energy_shear_factor, annulus_energy_shear_factor_of_moduli, ShearFactorError,
};

const VECTORS: &str = include_str!("shear_moduli_vectors.txt");
const U: f64 = 1.0 / 9_007_199_254_740_992.0; // 2^-53

fn float(hex: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(hex, 16).unwrap())
}

#[test]
fn the_grid_keeps_the_fixed_order_and_the_documented_bound() {
    let (mut checked, mut refused, mut worst) = (0, 0, 0.0f64);
    for line in VECTORS.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split(' ').collect();
        let [ri, ro, e, g] = [0, 1, 2, 3].map(|i| float(fields[i]));
        let got = annulus_energy_shear_factor_of_moduli(ri, ro, e, g);
        if fields[4] == "refused" {
            let kind = match got {
                Err(ShearFactorError::NumericalRange { .. }) => "NumericalRange",
                Err(ShearFactorError::PoissonRatioOutOfRange { .. }) => "PoissonRatioOutOfRange",
                other => panic!("{line}: {other:?}"),
            };
            assert_eq!(kind, fields[5], "{line}");
            refused += 1;
            continue;
        }
        let kappa = got.unwrap_or_else(|error| panic!("{line}: {error}"));
        assert_eq!(kappa.to_bits(), float(fields[4]).to_bits(), "{line}: the fixed order");
        let (hi, lo) = (float(fields[5]), float(fields[6]));
        // kappa and hi are within a factor of two, so kappa - hi is exact.
        let error = ((kappa - hi) - lo).abs() / hi;
        let m = ri / ro;
        let beta = m * m / (1.0 - m * m);
        let bound = 21.0 + 6.0 * beta;
        assert!(error <= bound * U, "{line}: {} u > {bound} u", error / U);
        worst = worst.max(error / U / bound);
        checked += 1;
    }
    eprintln!("annulus_energy_shear_factor_of_moduli: {checked} vectors within (21 + 6 beta) u (worst {worst:.3} of the bound), {refused} refused");
    assert!(checked > 1000 && refused > 0);
}

/// Where E/(2G) lies in [1/2, 2] and is binary64, 1 + nu = E/(2G) is exact in the existing
/// order too, so the two forms agree to their own few ulps; below 1/2 the moduli form keeps
/// its accuracy where `fl(E/(2G) - 1) + 1` does not.
#[test]
fn the_moduli_form_matches_the_poisson_form_where_one_plus_nu_is_exact() {
    for (ri, ro) in [(0.0, 1.0), (0.09, 0.1), (0.75, 1.0)] {
        let m = ri / ro;
        let bound = 2.0 * (21.0 + 6.0 * (m * m / (1.0 - m * m))) * U;
        for nu in [0.3, 0.0, -0.25, -0.5, 0.49, -5.0 / 9.0] {
            let g = 8e10;
            let e = 2.0 * g * (1.0 + nu);
            let ratio = e / (2.0 * g);
            if !(0.5..=1.5).contains(&ratio) {
                continue;
            }
            let poisson = annulus_energy_shear_factor(ri, ro, ratio - 1.0).unwrap();
            let moduli = annulus_energy_shear_factor_of_moduli(ri, ro, e, g).unwrap();
            assert!((poisson - moduli).abs() <= bound * poisson, "{ri} {ro} {nu}: {poisson} {moduli}");
        }
    }
    // E/(2G) = 1/(3*2^52), the review's near-minus-one moduli: 1 + nu re-formed from
    // fl(E/(2G) - 1) is 2^-53, 1.5 times the exact E/(2G), so the existing form's kappa
    // (proportional to (1+nu)^2 there) is 2.25 times the contract's.
    let (e, g) = (1e9, 6.755399441055744e24);
    let moduli = annulus_energy_shear_factor_of_moduli(0.09, 0.1, e, g).unwrap();
    let poisson = annulus_energy_shear_factor(0.09, 0.1, e / (2.0 * g) - 1.0).unwrap();
    assert!((poisson / moduli - 2.25).abs() < 1e-12, "{}", poisson / moduli);
}

#[test]
fn the_moduli_form_refuses_by_name() {
    for (ri, ro, e, g) in [(f64::NAN, 1.0, 1.0, 1.0), (0.5, 1.0, f64::INFINITY, 1.0), (0.5, 1.0, 1.0, f64::NAN)] {
        assert!(matches!(annulus_energy_shear_factor_of_moduli(ri, ro, e, g), Err(ShearFactorError::NonFiniteInput { .. })));
    }
    assert!(matches!(annulus_energy_shear_factor_of_moduli(0.0, 0.0, 1.0, 1.0), Err(ShearFactorError::NonPositiveOuterRadius { .. })));
    assert!(matches!(annulus_energy_shear_factor_of_moduli(1.0, 1.0, 1.0, 1.0), Err(ShearFactorError::InnerRadiusOutOfRange { .. })));
    assert_eq!(
        annulus_energy_shear_factor_of_moduli(0.5, 1.0, 0.0, 1.0),
        Err(ShearFactorError::NonPositiveModulus { name: "elastic_modulus", value: 0.0 })
    );
    assert_eq!(
        annulus_energy_shear_factor_of_moduli(0.5, 1.0, 1.0, -1.0),
        Err(ShearFactorError::NonPositiveModulus { name: "shear_modulus", value: -1.0 })
    );
    // nu = E/(2G) - 1 = 1/2 and above, reported as that binary64 nu.
    assert_eq!(
        annulus_energy_shear_factor_of_moduli(0.5, 1.0, 3.0, 1.0),
        Err(ShearFactorError::PoissonRatioOutOfRange { value: 0.5 })
    );
    // E/(2G) -> 0: t*t overflows below about 7.458e-155, and D is refused.
    assert!(annulus_energy_shear_factor_of_moduli(0.09, 0.1, 2.0 * 7.46e-155, 1.0).is_ok());
    assert!(matches!(
        annulus_energy_shear_factor_of_moduli(0.09, 0.1, 2.0 * 7.45e-155, 1.0),
        Err(ShearFactorError::NumericalRange { name: "kappa_E: (7 + t*t)*(b*b) + 48*m2" })
    ));
}
