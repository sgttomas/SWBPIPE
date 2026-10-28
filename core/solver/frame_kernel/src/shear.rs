//! T4-U8 (D-6): the energy-matched shear factor of a homogeneous isotropic
//! circular annulus, `timoshenko_annulus_energy_v1` (SHEAR_REFERENCE §1;
//! `DEL-05-03` Design `section-bases-and-timoshenko.md` §3.1).
//!
//! kappa_E matches the Saint-Venant shear strain energy per unit length,
//! V^2/(2 Ks) with Ks = kappa_E G A. It is neither Cowper's
//! displacement-average coefficient nor 5/6. The binary64 operation order is
//! fixed, with no fused multiply-add, so the Rust and Python readers reproduce
//! kappa and A_s bit for bit from the evidence:
//!
//! ```text
//! m  = ri/ro;  m2 = m*m;  a = 1 + m2;  b = 1 - m2;  n1 = 1 + nu
//! c  = (8*nu + 14)*nu + 7
//! D  = c*(b*b) + 48*(m2*(n1*n1))
//! kappa = (6*(n1*n1))*(a*a) / D
//! A_s = kappa * A
//! ```
//!
//! D is the cancellation-free form (7+14nu+8nu^2)(1-m^2)^2 + 48m^2(1+nu)^2 of
//! (7+14nu+8nu^2)(1+m^2)^2 + 4m^2(5+10nu+4nu^2). ri and ro are the section's
//! own radii (the nominal section under D-5) and nu the member's selected
//! Poisson ratio (the E/nu basis that gives G).
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShearFactorError {
    /// An input is NaN or infinite.
    NonFiniteInput { name: &'static str, value: f64 },
    /// ro is not positive.
    NonPositiveOuterRadius { value: f64 },
    /// ri is outside [0, ro).
    InnerRadiusOutOfRange { inner_radius: f64, outer_radius: f64 },
    /// nu is outside the open interval (-1, 1/2).
    PoissonRatioOutOfRange { value: f64 },
    /// A modulus (`name`) is not positive.
    NonPositiveModulus { name: &'static str, value: f64 },
    /// The area is not positive.
    NonPositiveArea { value: f64 },
    /// A formed value is zero, subnormal or non-finite (`name` names it).
    NumericalRange { name: &'static str },
}

impl fmt::Display for ShearFactorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => write!(f, "{name} must be finite, got {value}"),
            Self::NonPositiveOuterRadius { value } => {
                write!(f, "outer radius must be positive, got {value}")
            }
            Self::InnerRadiusOutOfRange {
                inner_radius,
                outer_radius,
            } => write!(
                f,
                "inner radius must satisfy 0 <= ri < ro, got ri = {inner_radius}, ro = {outer_radius}"
            ),
            Self::PoissonRatioOutOfRange { value } => {
                write!(f, "Poisson ratio must satisfy -1 < nu < 1/2, got {value}")
            }
            Self::NonPositiveModulus { name, value } => {
                write!(f, "{name} must be positive, got {value}")
            }
            Self::NonPositiveArea { value } => write!(f, "area must be positive, got {value}"),
            Self::NumericalRange { name } => write!(
                f,
                "range: annulus shear factor outside the binary64 normal range at {name}"
            ),
        }
    }
}

impl Error for ShearFactorError {}

fn finite(name: &'static str, value: f64) -> Result<f64, ShearFactorError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(ShearFactorError::NonFiniteInput { name, value })
    }
}

/// kappa_E of the annulus ri <= r <= ro with Poisson ratio nu, in the fixed
/// binary64 order of the module documentation. Refuses a non-finite input,
/// ro <= 0, ri outside [0, ro), nu outside (-1, 1/2), and a kappa that is not
/// a positive normal number. ri = 0 is the solid circle,
/// 6(1+nu)^2/(7+14nu+8nu^2).
pub fn annulus_energy_shear_factor(
    inner_radius: f64,
    outer_radius: f64,
    poisson_ratio: f64,
) -> Result<f64, ShearFactorError> {
    let ri = finite("inner_radius", inner_radius)?;
    let ro = finite("outer_radius", outer_radius)?;
    let nu = finite("poisson_ratio", poisson_ratio)?;
    if ro <= 0.0 {
        return Err(ShearFactorError::NonPositiveOuterRadius { value: ro });
    }
    if !(0.0..ro).contains(&ri) {
        return Err(ShearFactorError::InnerRadiusOutOfRange {
            inner_radius: ri,
            outer_radius: ro,
        });
    }
    if !(nu > -1.0 && nu < 0.5) {
        return Err(ShearFactorError::PoissonRatioOutOfRange { value: nu });
    }
    let m = ri / ro;
    let m2 = m * m;
    let a = 1.0 + m2;
    let b = 1.0 - m2;
    let n1 = 1.0 + nu;
    let c = (8.0 * nu + 14.0) * nu + 7.0;
    let d = c * (b * b) + 48.0 * (m2 * (n1 * n1));
    if !(d.is_normal() && d > 0.0) {
        return Err(ShearFactorError::NumericalRange {
            name: "kappa_E: c*(b*b) + 48*(m2*(n1*n1))",
        });
    }
    let kappa = (6.0 * (n1 * n1)) * (a * a) / d;
    if !(kappa.is_normal() && kappa > 0.0) {
        return Err(ShearFactorError::NumericalRange {
            name: "kappa_E: (6*(n1*n1))*(a*a)/D",
        });
    }
    Ok(kappa)
}

/// The shear area A_s = kappa * A, one binary64 product, for
/// `FrameSection::timoshenko`. `area` is the section's own metal area (the
/// nominal area A_n under D-5). Refuses a non-finite or non-positive input
/// and a product that is not a positive normal number.
pub fn shear_area(kappa: f64, area: f64) -> Result<f64, ShearFactorError> {
    let kappa = finite("kappa", kappa)?;
    let area = finite("area", area)?;
    if !(kappa.is_normal() && kappa > 0.0) {
        return Err(ShearFactorError::NumericalRange { name: "kappa" });
    }
    if area <= 0.0 {
        return Err(ShearFactorError::NonPositiveArea { value: area });
    }
    let shear_area = kappa * area;
    if !(shear_area.is_normal() && shear_area > 0.0) {
        return Err(ShearFactorError::NumericalRange {
            name: "A_s: kappa*A",
        });
    }
    Ok(shear_area)
}

/// The moduli terms of an isotropic material whose Poisson ratio is the exact
/// nu* = E/(2G) - 1, formed from E and G without any nu, in this fixed
/// binary64 order:
///
/// ```text
/// x = (2*G)/E   (rounds 2G/E = 1/(1 + nu*))
/// t = 1 - x     (rounds 1 - 2G/E = nu*/(1 + nu*))
/// ```
///
/// In exact arithmetic a ratio (a + b*nu*)/(1 + nu*) is a*x* + b*t* with
/// x* = 2G/E and t* = 1 - x*. x and t are not functions of a rounded nu:
/// at E/(2G) = 1/(3*2^52), fl(fl(E/(2G)) - 1) = -1 + 2^-53, whose
/// 1/(1 + nu) is 2^53, while x = 3*2^52.
///
/// **Exactness.** Let u = 2^-53 and x* = 2G/E. 2*G is exact (a power-of-two
/// scaling) and x = x*(1 + e) with |e| <= u. For x in [1/2, 2], 1 - x is
/// exact (Sterbenz), so t carries only x's rounding: |t - t*| <= u*x*. For
/// x > 2, |1 - x| >= x/2 and 1 - x rounds once, so t's relative error is at
/// most 3u. Forming 1 + nu = fl(fl(E/(2G)) - 1) + 1 instead loses up to
/// 2^-54/(E/(2G)) relative below E/(2G) = 1/2 (x > 2).
///
/// **Refusals.** A non-finite input, E or G not positive, and an x that is
/// not a positive normal number (2*G overflowing, or x out of range). The
/// Poisson domain is the caller's.
pub fn moduli_ratio_terms(
    elastic_modulus: f64,
    shear_modulus: f64,
) -> Result<(f64, f64), ShearFactorError> {
    let e = finite("elastic_modulus", elastic_modulus)?;
    let g = finite("shear_modulus", shear_modulus)?;
    for (name, value) in [("elastic_modulus", e), ("shear_modulus", g)] {
        if value <= 0.0 {
            return Err(ShearFactorError::NonPositiveModulus { name, value });
        }
    }
    let x = (2.0 * g) / e;
    if !(x.is_normal() && x > 0.0) {
        return Err(ShearFactorError::NumericalRange { name: "x: (2*G)/E" });
    }
    Ok((x, 1.0 - x))
}
