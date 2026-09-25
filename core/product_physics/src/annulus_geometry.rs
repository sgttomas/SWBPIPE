//! Pure circular areas from normalized source diameter and radial thickness.
//!
//! No material constants, pressure, stiffness or rounded-radius admission rule
//! enters this API. The shared scaled arithmetic is the existing pressure scalar
//! kernel's numeric primitive; its arithmetic is unchanged.
use super::pressure_exact::Scaled;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceAreaError {
    InvalidOutsideDiameter,
    InvalidWallThickness,
    InvalidInsulationThickness,
    NonRepresentableArea,
}

fn validate_diameter(od: f64) -> Result<(), SourceAreaError> {
    if !od.is_finite() || od <= 0.0 {
        Err(SourceAreaError::InvalidOutsideDiameter)
    } else {
        Ok(())
    }
}
pub(crate) fn validate_wall(od: f64, wall: f64) -> Result<(), SourceAreaError> {
    validate_diameter(od)?;
    // Compare without first doubling wall or rounding the half-diameter.
    if !wall.is_finite() || wall <= 0.0 || wall > od || wall > od - wall {
        Err(SourceAreaError::InvalidWallThickness)
    } else {
        Ok(())
    }
}

pub(crate) fn source_wall_area_scaled(od: f64, wall: f64) -> Result<Scaled, SourceAreaError> {
    validate_wall(od, wall)?;
    let t = Scaled::from_f64(wall);
    Ok(Scaled::from_f64(PI).mul(t).mul(Scaled::from_f64(od).sub(t)))
}
pub(crate) fn source_bore_area_scaled(od: f64, wall: f64) -> Result<Scaled, SourceAreaError> {
    validate_wall(od, wall)?;
    let ri = Scaled::from_f64(od)
        .mul(Scaled::from_f64(0.5))
        .sub(Scaled::from_f64(wall));
    Ok(Scaled::from_f64(PI).mul(ri).mul(ri))
}
fn represented_area(area: Scaled, zero_allowed: bool) -> Result<f64, SourceAreaError> {
    let value = area.to_f64();
    if !value.is_finite() || value < 0.0 || (value == 0.0 && !zero_allowed) {
        Err(SourceAreaError::NonRepresentableArea)
    } else {
        Ok(value)
    }
}
pub(crate) fn source_wall_area_m2(od: f64, wall: f64) -> Result<f64, SourceAreaError> {
    represented_area(source_wall_area_scaled(od, wall)?, false)
}
pub(crate) fn source_bore_area_m2(od: f64, wall: f64) -> Result<f64, SourceAreaError> {
    let area = source_bore_area_scaled(od, wall)?;
    represented_area(area, od == wall + wall)
}
pub(crate) fn source_insulation_area_m2(od: f64, thickness: f64) -> Result<f64, SourceAreaError> {
    validate_diameter(od)?;
    if !thickness.is_finite() || thickness < 0.0 {
        return Err(SourceAreaError::InvalidInsulationThickness);
    }
    let d = Scaled::from_f64(thickness);
    let area = Scaled::from_f64(PI).mul(d).mul(Scaled::from_f64(od).add(d));
    represented_area(area, thickness == 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= 1e-12 * expected.abs(),
            "{actual:e} != {expected:e}"
        );
    }
    #[test]
    fn ordinary_source_areas_and_solid_core_are_explicit() {
        close(source_wall_area_m2(4.0, 1.0).unwrap(), 3.0 * PI);
        close(source_bore_area_m2(4.0, 1.0).unwrap(), PI);
        close(source_wall_area_m2(4.0, 2.0).unwrap(), 4.0 * PI);
        assert_eq!(source_bore_area_m2(4.0, 2.0).unwrap(), 0.0);
        close(source_insulation_area_m2(4.0, 1.0).unwrap(), 5.0 * PI);
        assert_eq!(source_insulation_area_m2(4.0, 0.0).unwrap(), 0.0);
    }
    #[test]
    fn area_does_not_require_distinct_rounded_radii() {
        let t = 2.0_f64.powi(-55);
        assert_eq!(0.5 - t, 0.5);
        close(source_wall_area_m2(1.0, t).unwrap(), PI * t * (1.0 - t));
        close(
            source_insulation_area_m2(1.0, t).unwrap(),
            PI * t * (1.0 + t),
        );
    }
    #[test]
    fn scaled_source_area_does_not_depend_on_resolving_squared_radius_difference() {
        // Rounded outer/inner radii coincide, but the source product remains PI.
        let od = 1e154;
        let t = 1e-154;
        close(source_wall_area_m2(od, t).unwrap(), PI);
        close(source_insulation_area_m2(od, t).unwrap(), PI);
    }
    #[test]
    fn scaled_area_avoids_intermediate_underflow_before_final_rounding() {
        let od = 2.0_f64.powi(-537);
        let wall = 2.0_f64.powi(-539);
        assert_eq!(wall * (od - wall), 0.0);
        // Exact binary source product is 3*pi*2^-1078 = 0.589... minimum subnormals.
        assert_eq!(source_wall_area_m2(od, wall).unwrap(), f64::from_bits(1));
    }
    #[test]
    fn invalid_and_positive_unrepresentable_areas_fail_closed() {
        assert!(source_wall_area_m2(1.0, 0.6).is_err());
        assert!(source_wall_area_m2(1.0, 0.0).is_err());
        assert!(source_wall_area_m2(f64::INFINITY, 0.1).is_err());
        assert!(source_insulation_area_m2(1.0, -0.1).is_err());
        assert!(source_wall_area_m2(1e-200, 1e-201).is_err());
        assert!(source_bore_area_m2(1e-200, 1e-201).is_err());
        assert!(source_insulation_area_m2(1e-200, 1e-201).is_err());
        assert_eq!(source_bore_area_m2(1e-200, 5e-201).unwrap(), 0.0);
    }
}
