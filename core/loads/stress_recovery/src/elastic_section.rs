//! Small-strain elastic normal/torsional quantities for one circular section.
//!
//! Inputs are coherent SI values in a right-handed local frame. Section actions
//! act on the positive-x cut face: N is tensile, T is about +x, and My/Mz follow
//! the right-hand rule. Thus sigma_x(y,z) = N/A + My*z/I - Mz*y/I.
//! The caller owns case/station/frame/section provenance and must supply physical
//! wall N (including any pressure reconstruction) exactly once. These are not
//! principal, equivalent, piping-code, hoop/radial, or transverse-shear stresses.
//! No flexibility or stress-intensification factor is implicit in this mapping.

/// Properties from the same homogeneous circular cross-section.
///
/// Numeric validation does not establish geometric consistency: the caller must
/// establish I_y = I_z = I and circular Saint-Venant J (= 2I for an annulus), as
/// well as the section's material/geometry applicability. Units are m², m⁴, m⁴, m.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ElasticCircularSection {
    pub area: f64,
    pub second_moment: f64,
    pub polar_moment: f64,
    pub outer_radius: f64,
}

/// Signed N (N), My/Mz (N m), and T (N m) at one common section/cut/frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SignedSectionActions {
    pub axial_force: f64,
    pub bending_moment_y: f64,
    pub bending_moment_z: f64,
    pub torsional_moment: f64,
}

/// All quantities are Pa. The normal extrema are over the outer circumference,
/// not over stations along a member; zero bending ties all circumferential fibers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ElasticSectionStress {
    pub axial_normal: f64,
    pub bending_amplitude: f64,
    pub maximum_normal: f64,
    pub minimum_normal: f64,
    pub maximum_absolute_normal: f64,
    /// Signed circumferential shear amplitude: tau_xy=-tau*z/ro,
    /// tau_xz=tau*y/ro. Its magnitude is the maximum outer torsional shear.
    pub torsional_shear: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElasticSectionError {
    NonFiniteInput {
        quantity: &'static str,
    },
    NonPositiveSection {
        quantity: &'static str,
    },
    /// Includes overflow and a nonzero result rounding to zero. Finite subnormal
    /// results are accepted at f64 precision; exact real arithmetic is not claimed.
    UnrepresentableQuantity {
        quantity: &'static str,
    },
}

/// Recover signed axial/torsional quantities and circular normal extrema.
///
/// Bending is hypot(My,Mz)/Z, Z=I/ro. Each moment is scaled before hypot to
/// avoid overflow of a moment norm whose resulting stress is representable.
/// Products/ratios use binary exponent separation to avoid intermediate overflow
/// or underflow. An unrepresentable required quantity returns an error; no
/// partial result, clamped value, or unavailable-as-zero result is returned.
pub fn evaluate_elastic_section(
    section: ElasticCircularSection,
    actions: SignedSectionActions,
) -> Result<ElasticSectionStress, ElasticSectionError> {
    for (quantity, value) in [
        ("area", section.area),
        ("second_moment", section.second_moment),
        ("polar_moment", section.polar_moment),
        ("outer_radius", section.outer_radius),
    ] {
        finite_input(quantity, value)?;
        if value <= 0.0 {
            return Err(ElasticSectionError::NonPositiveSection { quantity });
        }
    }
    for (quantity, value) in [
        ("axial_force", actions.axial_force),
        ("bending_moment_y", actions.bending_moment_y),
        ("bending_moment_z", actions.bending_moment_z),
        ("torsional_moment", actions.torsional_moment),
    ] {
        finite_input(quantity, value)?;
    }
    let axial_normal = scaled_ratio(actions.axial_force, 1.0, section.area, "axial_normal")?;
    let by = scaled_ratio(
        actions.bending_moment_y,
        section.outer_radius,
        section.second_moment,
        "bending_normal_y",
    )?;
    let bz = scaled_ratio(
        actions.bending_moment_z,
        section.outer_radius,
        section.second_moment,
        "bending_normal_z",
    )?;
    let bending_amplitude = finite_output("bending_amplitude", by.hypot(bz))?;
    let torsional_shear = scaled_ratio(
        actions.torsional_moment,
        section.outer_radius,
        section.polar_moment,
        "torsional_shear",
    )?;
    Ok(ElasticSectionStress {
        axial_normal,
        bending_amplitude,
        maximum_normal: finite_output("maximum_normal", axial_normal + bending_amplitude)?,
        minimum_normal: finite_output("minimum_normal", axial_normal - bending_amplitude)?,
        maximum_absolute_normal: finite_output(
            "maximum_absolute_normal",
            axial_normal.abs() + bending_amplitude,
        )?,
        torsional_shear,
    })
}

fn finite_input(quantity: &'static str, value: f64) -> Result<(), ElasticSectionError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(ElasticSectionError::NonFiniteInput { quantity })
    }
}

fn finite_output(quantity: &'static str, value: f64) -> Result<f64, ElasticSectionError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(ElasticSectionError::UnrepresentableQuantity { quantity })
    }
}

// Positive finite x = mantissa * 2^exponent with mantissa in [1,2).
fn binary_parts(x: f64) -> (f64, i32) {
    let (normal, adjustment) = if x < f64::MIN_POSITIVE {
        (x * 4_503_599_627_370_496.0, -52)
    } else {
        (x, 0)
    };
    let bits = normal.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32 - 1023 + adjustment;
    let mantissa = f64::from_bits((bits & ((1_u64 << 52) - 1)) | (1023_u64 << 52));
    (mantissa, exponent)
}

// value*numerator/denominator without materializing a possibly unrepresentable Z,
// radius ratio, moment norm, or product. Inputs have already been validated.
fn scaled_ratio(
    value: f64,
    numerator: f64,
    denominator: f64,
    quantity: &'static str,
) -> Result<f64, ElasticSectionError> {
    if value == 0.0 {
        return Ok(value);
    }
    let (mv, ev) = binary_parts(value.abs());
    let (mn, en) = binary_parts(numerator);
    let (md, ed) = binary_parts(denominator);
    let mut mantissa = mv * mn / md;
    let mut exponent = ev + en - ed;
    if mantissa >= 2.0 {
        mantissa *= 0.5;
        exponent += 1;
    }
    if mantissa < 1.0 {
        mantissa *= 2.0;
        exponent -= 1;
    }
    let magnitude = if (-1022..=1023).contains(&exponent) {
        mantissa * f64::from_bits(((exponent + 1023) as u64) << 52)
    } else if (-1074..=-1023).contains(&exponent) {
        mantissa * f64::from_bits(1_u64 << (exponent + 1074))
    } else if exponent == -1075 {
        (mantissa * 0.5) * f64::from_bits(1)
    } else {
        return Err(ElasticSectionError::UnrepresentableQuantity { quantity });
    };
    if !magnitude.is_finite() || magnitude == 0.0 {
        return Err(ElasticSectionError::UnrepresentableQuantity { quantity });
    }
    Ok(magnitude.copysign(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn section() -> ElasticCircularSection {
        ElasticCircularSection {
            area: 0.0009 * PI,
            second_moment: 0.0000009225 * PI,
            polar_moment: 0.000001845 * PI,
            outer_radius: 0.05,
        }
    }
    fn s1() -> SignedSectionActions {
        SignedSectionActions {
            axial_force: 1800.0 * PI,
            bending_moment_y: 55.35 * PI,
            bending_moment_z: 73.8 * PI,
            torsional_moment: 221.4 * PI,
        }
    }
    fn zero() -> SignedSectionActions {
        SignedSectionActions {
            axial_force: 0.0,
            bending_moment_y: 0.0,
            bending_moment_z: 0.0,
            torsional_moment: 0.0,
        }
    }
    fn close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= 2e-14 * expected.abs().max(1.0),
            "{actual:.17e} != {expected:.17e}"
        );
    }

    #[test]
    fn s1_exact_si_annulus_recovers_circular_extrema_and_torsion() {
        let s = evaluate_elastic_section(section(), s1()).unwrap();
        close(s.axial_normal, 2e6);
        close(s.bending_amplitude, 5e6);
        close(s.maximum_normal, 7e6);
        close(s.minimum_normal, -3e6);
        close(s.maximum_absolute_normal, 7e6);
        close(s.torsional_shear, 6e6);
        // Independent stress field at tensile fiber y=-.04,z=.03 gives +7 MPa.
        let a = s1();
        let p = section();
        close(
            a.axial_force / p.area
                + a.bending_moment_y * 0.03 / p.second_moment
                + a.bending_moment_z * 0.04 / p.second_moment,
            s.maximum_normal,
        );
    }

    #[test]
    fn s2_pure_torsion_has_no_normal_stress() {
        let s = evaluate_elastic_section(
            section(),
            SignedSectionActions {
                torsional_moment: s1().torsional_moment,
                ..zero()
            },
        )
        .unwrap();
        assert_eq!(s.axial_normal, 0.0);
        assert_eq!(s.bending_amplitude, 0.0);
        assert_eq!(s.maximum_normal, 0.0);
        assert_eq!(s.minimum_normal, 0.0);
        assert_eq!(s.maximum_absolute_normal, 0.0);
        close(s.torsional_shear, 6e6);
    }

    #[test]
    fn arbitrary_section_axis_rotation_preserves_normal_extrema() {
        let base = evaluate_elastic_section(section(), s1()).unwrap();
        for angle in [0.37_f64, -1.2, 2.8, PI / 2.0] {
            let a = s1();
            let (sin, cos) = angle.sin_cos();
            let rotated = SignedSectionActions {
                bending_moment_y: cos * a.bending_moment_y + sin * a.bending_moment_z,
                bending_moment_z: -sin * a.bending_moment_y + cos * a.bending_moment_z,
                ..a
            };
            let s = evaluate_elastic_section(section(), rotated).unwrap();
            close(s.bending_amplitude, base.bending_amplitude);
            close(s.maximum_normal, base.maximum_normal);
            close(s.minimum_normal, base.minimum_normal);
            close(s.torsional_shear, base.torsional_shear);
        }
    }

    #[test]
    fn action_reversal_retains_absolute_normal_and_reverses_signed_outputs() {
        let a = s1();
        let base = evaluate_elastic_section(section(), a).unwrap();
        let reversed = evaluate_elastic_section(
            section(),
            SignedSectionActions {
                axial_force: -a.axial_force,
                bending_moment_y: -a.bending_moment_y,
                bending_moment_z: -a.bending_moment_z,
                torsional_moment: -a.torsional_moment,
            },
        )
        .unwrap();
        close(reversed.axial_normal, -base.axial_normal);
        close(reversed.maximum_normal, -base.minimum_normal);
        close(reversed.minimum_normal, -base.maximum_normal);
        close(
            reversed.maximum_absolute_normal,
            base.maximum_absolute_normal,
        );
        close(reversed.torsional_shear, -base.torsional_shear);
    }

    #[test]
    fn pressure_wall_force_is_counted_once_and_effective_force_is_not_an_input() {
        let p = ElasticCircularSection {
            area: 0.0011 * PI,
            second_moment: 0.0000016775 * PI,
            polar_moment: 0.000003355 * PI,
            outer_radius: 0.06,
        };
        let closed = evaluate_elastic_section(
            p,
            SignedSectionActions {
                axial_force: 5000.0 * PI,
                ..zero()
            },
        )
        .unwrap();
        close(closed.axial_normal, 50e6 / 11.0);
        close(closed.maximum_absolute_normal, 50e6 / 11.0);
        let separate_closures = evaluate_elastic_section(p, zero()).unwrap();
        assert_eq!(separate_closures.maximum_absolute_normal, 0.0);
    }

    #[test]
    fn compression_and_zero_actions_preserve_signed_normal_extrema() {
        let s = evaluate_elastic_section(
            section(),
            SignedSectionActions {
                axial_force: -s1().axial_force,
                ..zero()
            },
        )
        .unwrap();
        close(s.maximum_normal, -2e6);
        close(s.minimum_normal, -2e6);
        close(s.maximum_absolute_normal, 2e6);
        assert_eq!(
            evaluate_elastic_section(section(), zero()).unwrap(),
            ElasticSectionStress {
                axial_normal: 0.0,
                bending_amplitude: 0.0,
                maximum_normal: 0.0,
                minimum_normal: 0.0,
                maximum_absolute_normal: 0.0,
                torsional_shear: 0.0
            }
        );
    }

    #[test]
    fn rejects_every_invalid_section_and_nonfinite_action_slot() {
        for slot in 0..4 {
            for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut p = section();
                match slot {
                    0 => p.area = invalid,
                    1 => p.second_moment = invalid,
                    2 => p.polar_moment = invalid,
                    _ => p.outer_radius = invalid,
                }
                assert!(evaluate_elastic_section(p, zero()).is_err());
            }
            for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut a = s1();
                match slot {
                    0 => a.axial_force = invalid,
                    1 => a.bending_moment_y = invalid,
                    2 => a.bending_moment_z = invalid,
                    _ => a.torsional_moment = invalid,
                }
                assert!(matches!(
                    evaluate_elastic_section(section(), a),
                    Err(ElasticSectionError::NonFiniteInput { .. })
                ));
            }
        }
    }

    #[test]
    fn scaled_arithmetic_avoids_intermediate_overflow_and_underflow() {
        // Product overflows, but cancelling scales yield exactly representable 2^900.
        let huge = 2_f64.powi(900);
        let small = 2_f64.powi(-900);
        close(scaled_ratio(huge, huge, huge, "test").unwrap() / huge, 1.0);
        close(
            scaled_ratio(small, small, small, "test").unwrap() / small,
            1.0,
        );
        assert_eq!(scaled_ratio(f64::MAX, 1.0, f64::MAX, "test").unwrap(), 1.0);
        assert_eq!(
            scaled_ratio(f64::from_bits(1), 1.0, f64::from_bits(1), "test").unwrap(),
            1.0
        );
        // Unscaled hypot would overflow; scaled components are 1 Pa each.
        let p = ElasticCircularSection {
            area: 1.0,
            second_moment: f64::MAX,
            polar_moment: f64::MAX,
            outer_radius: 1.0,
        };
        let s = evaluate_elastic_section(
            p,
            SignedSectionActions {
                bending_moment_y: f64::MAX,
                bending_moment_z: f64::MAX,
                ..zero()
            },
        )
        .unwrap();
        close(s.bending_amplitude, 2_f64.sqrt());
    }

    #[test]
    fn finite_subnormal_results_are_retained_but_nonzero_underflow_is_rejected() {
        let least = f64::from_bits(1);
        assert_eq!(scaled_ratio(least, 1.0, 1.0, "test").unwrap(), least);
        assert_eq!(scaled_ratio(-least, 1.0, 1.0, "test").unwrap(), -least);
        assert_eq!(scaled_ratio(least, 0.75, 1.0, "test").unwrap(), least);
        assert!(scaled_ratio(least, 0.5, 1.0, "test").is_err());
        assert!(scaled_ratio(least, least, 1.0, "test").is_err());
    }

    #[test]
    fn nonrepresentable_component_norm_and_extrema_are_rejected() {
        assert!(scaled_ratio(f64::MAX, 2.0, 1.0, "test").is_err());
        let p = ElasticCircularSection {
            area: 1.0,
            second_moment: 1.0,
            polar_moment: 2.0,
            outer_radius: 1.0,
        };
        assert!(evaluate_elastic_section(
            p,
            SignedSectionActions {
                bending_moment_y: f64::MAX,
                bending_moment_z: f64::MAX,
                ..zero()
            }
        )
        .is_err());
        assert!(evaluate_elastic_section(
            p,
            SignedSectionActions {
                axial_force: f64::MAX,
                bending_moment_y: f64::MAX,
                ..zero()
            }
        )
        .is_err());
    }
}
