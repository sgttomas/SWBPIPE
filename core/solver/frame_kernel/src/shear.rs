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
//!
//! [`annulus_energy_shear_factor_of_moduli`] forms the same kappa_E from E and
//! G when nu = E/(2G) - 1 (the pressure-free route, `DEL-05-03` Design
//! `pf-timoshenko.md` §2), without forming 1 + nu; its order and error bound
//! are in its documentation.
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

/// kappa_E of the annulus ri <= r <= ro for the material whose Poisson ratio
/// is nu = E/(2G) - 1 (`DEL-05-03` Design `pf-timoshenko.md` §2), formed
/// without 1 + nu, in this fixed binary64 order with no fused multiply-add:
///
/// ```text
/// m = ri/ro;  m2 = m*m;  a = 1 + m2;  b = 1 - m2
/// x = (2*G)/E;  t = 1 - x
/// D = (7 + t*t)*(b*b) + 48*m2
/// kappa = (6*(a*a)) / D
/// ```
///
/// **Identity.** c = 8nu^2 + 14nu + 7 = 7(1+nu)^2 + nu^2, so for nu > -1,
/// dividing the module form's numerator and D by (1+nu)^2 with
/// t = nu/(1+nu) gives kappa_E = 6a^2 / ((7 + t^2)b^2 + 48m^2), the same
/// value as SHEAR_REFERENCE's (7+14nu+8nu^2)(1+m^2)^2 + 4m^2(5+10nu+4nu^2)
/// form (c*a^2 = c*b^2 + 4m^2*c, and c + 5 + 10nu + 4nu^2 = 12(1+nu)^2). With
/// 1 + nu = E/(2G) exactly, t = 1 - 2G/E. The value is that of
/// [`annulus_energy_shear_factor`] at the exact nu; the bits are not.
///
/// **Error.** Let u = 2^-53, x* = 2G/E and t* = 1 - x* exactly, and
/// beta = m*^2/(1 - m*^2) for the exact m* = ri/ro (the section's condition,
/// shared with [`annulus_energy_shear_factor`]). The domain nu < 1/2 gives
/// x* > 2/3. To first order in u:
/// - x = x*(1 + e), |e| <= u.
/// - x in [1/2, 2]: 1 - x is exact (Sterbenz), so |t - t*| <= u*x* with
///   |t*| <= 1 and x* <= 2, and 7 + t*t is within 2u of 7 + t*^2 (relative:
///   at most (4u + u)/7 from t, plus its own rounding).
/// - x > 2: |1 - x*| >= x*/2, so x's error is amplified at most twice: t has
///   relative error <= 3u, t*t <= 7u and 7 + t*t <= 8u.
/// - Every later operation adds or multiplies positive terms: b*b carries
///   (3 + 6beta)u, (7 + t*t)*(b*b) at most (12 + 6beta)u, 48*m2 4u, D
///   (13 + 6beta)u, 6*(a*a) 7u, and the quotient
///
///   |kappa/kappa* - 1| <= (21 + 6beta)u,
///
/// for every E/(2G) in (0, 3/2), down to E/(2G) -> 0, where 1 + nu is never
/// formed. (Forming 1 + nu = fl(E/(2G) - 1) + 1 instead loses up to
/// 2^-54/(E/(2G)) relative below E/(2G) = 1/2.)
///
/// **Refusals.** A non-finite input, ro <= 0, ri outside [0, ro), E or G not
/// positive, and fl(E/(2G)) >= 3/2 (nu >= 1/2 in the binary64 nu = fl(E/(2G))
/// - 1 of the pressure-free stress recovery; reported as that nu). Beyond the
/// domain, only binary64 range: D is not normal exactly when t*t (or x)
/// overflows, at |t| >= about 2^512, i.e. E/(2G) <= about 7.458e-155 (b lies
/// in [0, 1] and every other term is finite); with D finite, kappa >= 6/D is
/// normal, so its check is a backstop that does not fire.
pub fn annulus_energy_shear_factor_of_moduli(
    inner_radius: f64,
    outer_radius: f64,
    elastic_modulus: f64,
    shear_modulus: f64,
) -> Result<f64, ShearFactorError> {
    let ri = finite("inner_radius", inner_radius)?;
    let ro = finite("outer_radius", outer_radius)?;
    let e = finite("elastic_modulus", elastic_modulus)?;
    let g = finite("shear_modulus", shear_modulus)?;
    if ro <= 0.0 {
        return Err(ShearFactorError::NonPositiveOuterRadius { value: ro });
    }
    if !(0.0..ro).contains(&ri) {
        return Err(ShearFactorError::InnerRadiusOutOfRange {
            inner_radius: ri,
            outer_radius: ro,
        });
    }
    for (name, value) in [("elastic_modulus", e), ("shear_modulus", g)] {
        if value <= 0.0 {
            return Err(ShearFactorError::NonPositiveModulus { name, value });
        }
    }
    let ratio = e / (2.0 * g);
    if ratio >= 1.5 {
        return Err(ShearFactorError::PoissonRatioOutOfRange { value: ratio - 1.0 });
    }
    let m = ri / ro;
    let m2 = m * m;
    let a = 1.0 + m2;
    let b = 1.0 - m2;
    let x = (2.0 * g) / e;
    let t = 1.0 - x;
    let d = (7.0 + t * t) * (b * b) + 48.0 * m2;
    if !(d.is_normal() && d > 0.0) {
        return Err(ShearFactorError::NumericalRange {
            name: "kappa_E: (7 + t*t)*(b*b) + 48*m2",
        });
    }
    let kappa = (6.0 * (a * a)) / d;
    if !(kappa.is_normal() && kappa > 0.0) {
        return Err(ShearFactorError::NumericalRange {
            name: "kappa_E: (6*(a*a))/D",
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
