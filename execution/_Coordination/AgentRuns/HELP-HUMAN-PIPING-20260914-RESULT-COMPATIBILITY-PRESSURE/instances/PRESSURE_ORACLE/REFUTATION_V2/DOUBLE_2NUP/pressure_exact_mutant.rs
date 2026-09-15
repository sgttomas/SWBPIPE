//! Dormant exact-annulus pressure mechanics.
//!
//! This private module contains only validated scalar mechanics and local
//! element pairs. It does not infer pressure-region topology or participate in
//! product-preview assembly, solve, recovery, or result serialization.

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExactPressureError {
    InvalidGeometry,
    NonRepresentableGeometry,
    InvalidMaterial,
    NonRepresentableMaterial,
    InvalidPressure,
    RadiusOutsideWall,
    InvalidStrain,
    NonRepresentableLoad,
    NonRepresentableStress,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ExactAnnulus {
    ri_m: f64,
    ro_m: f64,
    ai_m2: f64,
    as_m2: f64,
}

impl ExactAnnulus {
    pub(crate) fn from_radii(ri_m: f64, ro_m: f64) -> Result<Self, ExactPressureError> {
        if !ri_m.is_finite() || !ro_m.is_finite() || ri_m <= 0.0 || ri_m >= ro_m {
            return Err(ExactPressureError::InvalidGeometry);
        }

        let ai_m2 = (PI * ri_m) * ri_m;
        let as_m2 = (PI * (ro_m - ri_m)) * (ro_m + ri_m);
        if !is_positive_finite(ai_m2) || !is_positive_finite(as_m2) {
            return Err(ExactPressureError::NonRepresentableGeometry);
        }

        Ok(Self {
            ri_m,
            ro_m,
            ai_m2,
            as_m2,
        })
    }

    pub(crate) fn inner_radius_m(self) -> f64 {
        self.ri_m
    }

    pub(crate) fn outer_radius_m(self) -> f64 {
        self.ro_m
    }

    pub(crate) fn internal_area_m2(self) -> f64 {
        self.ai_m2
    }

    pub(crate) fn wall_area_m2(self) -> f64 {
        self.as_m2
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct IsotropicENu {
    e_pa: f64,
    nu: f64,
    g_pa: f64,
}

impl IsotropicENu {
    pub(crate) fn new(e_pa: f64, nu: f64) -> Result<Self, ExactPressureError> {
        if !e_pa.is_finite() || !nu.is_finite() || e_pa <= 0.0 || nu <= -1.0 || nu >= 0.5 {
            return Err(ExactPressureError::InvalidMaterial);
        }

        let g_pa = e_pa / (2.0 * (1.0 + nu));
        if !is_positive_finite(g_pa) {
            return Err(ExactPressureError::NonRepresentableMaterial);
        }

        Ok(Self { e_pa, nu, g_pa })
    }

    pub(crate) fn elastic_modulus_pa(self) -> f64 {
        self.e_pa
    }

    pub(crate) fn poisson_ratio(self) -> f64 {
        self.nu
    }

    pub(crate) fn shear_modulus_pa(self) -> f64 {
        self.g_pa
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct InternalDifferentialPressure {
    p_pa: f64,
}

impl InternalDifferentialPressure {
    pub(crate) fn new(p_pa: f64) -> Result<Self, ExactPressureError> {
        if !p_pa.is_finite() {
            return Err(ExactPressureError::InvalidPressure);
        }
        Ok(Self { p_pa })
    }

    pub(crate) fn pressure_pa(self) -> f64 {
        self.p_pa
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct RadialHoopStressPa {
    radial_pa: f64,
    hoop_pa: f64,
}

impl RadialHoopStressPa {
    pub(crate) fn radial_pa(self) -> f64 {
        self.radial_pa
    }

    pub(crate) fn hoop_pa(self) -> f64 {
        self.hoop_pa
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct AxialState {
    wall_force_n: f64,
    effective_force_n: f64,
    axial_membrane_pa: f64,
}

impl AxialState {
    pub(crate) fn wall_force_n(self) -> f64 {
        self.wall_force_n
    }

    pub(crate) fn effective_force_n(self) -> f64 {
        self.effective_force_n
    }

    pub(crate) fn axial_membrane_pa(self) -> f64 {
        self.axial_membrane_pa
    }
}

pub(crate) fn lame_at_radius(
    annulus: ExactAnnulus,
    pressure: InternalDifferentialPressure,
    r_m: f64,
) -> Result<RadialHoopStressPa, ExactPressureError> {
    if !r_m.is_finite() || r_m < annulus.ri_m || r_m > annulus.ro_m {
        return Err(ExactPressureError::RadiusOutsideWall);
    }

    let transverse_trace_pa = pressure_trace_pa(annulus, pressure)?;

    // This is algebraically A-B/r^2, factored so both exact wall-boundary
    // tractions remain well resolved even when the wall radii are adjacent
    // binary64 values and the Lamé coefficients are much larger than p.
    let inner_radius_ratio = annulus.ri_m / r_m;
    let outside_wall_fraction = ((annulus.ro_m - r_m) / (annulus.ro_m - annulus.ri_m))
        * ((annulus.ro_m + r_m) / (annulus.ro_m + annulus.ri_m));
    let radial_pa = checked_product3(
        -pressure.p_pa,
        inner_radius_ratio * inner_radius_ratio,
        outside_wall_fraction,
        ExactPressureError::NonRepresentableStress,
    )?;
    let hoop_pa = transverse_trace_pa - radial_pa;
    if !hoop_pa.is_finite() {
        return Err(ExactPressureError::NonRepresentableStress);
    }

    Ok(RadialHoopStressPa { radial_pa, hoop_pa })
}

pub(crate) fn axial_state(
    annulus: ExactAnnulus,
    material: IsotropicENu,
    pressure: InternalDifferentialPressure,
    thermal_strain: f64,
    epsilon_z: f64,
) -> Result<AxialState, ExactPressureError> {
    if !thermal_strain.is_finite() || !epsilon_z.is_finite() {
        return Err(ExactPressureError::InvalidStrain);
    }
    // Form the constitutive sum in stress space before scaling it to force.
    // This preserves a finite cancellation even when both separately scaled
    // force terms would overflow.
    let poisson_stress_pa = (2.0 * material.nu) * pressure_trace_pa(annulus, pressure)?;
    let mechanical_strain = epsilon_z - thermal_strain;
    let axial_membrane_pa = if mechanical_strain.is_finite() {
        material.e_pa.mul_add(mechanical_strain, poisson_stress_pa)
    } else {
        // Opposite finite strains can overflow in their direct subtraction
        // even when multiplication by a small modulus gives a finite stress.
        let strain_scale = epsilon_z.abs().max(thermal_strain.abs());
        let normalized_mechanical_strain =
            (epsilon_z / strain_scale) - (thermal_strain / strain_scale);
        checked_product3(
            material.e_pa,
            strain_scale,
            normalized_mechanical_strain,
            ExactPressureError::NonRepresentableStress,
        )? + poisson_stress_pa
    };
    if !axial_membrane_pa.is_finite() {
        return Err(ExactPressureError::NonRepresentableStress);
    }

    let force_from_stress_n = checked_product3(
        annulus.as_m2,
        axial_membrane_pa,
        1.0,
        ExactPressureError::NonRepresentableLoad,
    )?;
    let wall_force_n = if axial_membrane_pa.abs() < f64::MIN_POSITIVE {
        // A stress can round into the subnormal range before multiplication
        // by a large wall area. Retain the equivalent direct-force identity
        // as a narrow fallback, without allowing its separate terms to turn
        // a valid stress-scaled force into an error.
        direct_axial_force(annulus, material, pressure, thermal_strain, epsilon_z)
            .filter(|value| *value != 0.0)
            .unwrap_or(force_from_stress_n)
    } else {
        force_from_stress_n
    };
    let fluid_force_n = fluid_force(annulus, pressure)?;
    let effective_force_n = wall_force_n - fluid_force_n;
    if !wall_force_n.is_finite() || !effective_force_n.is_finite() {
        return Err(ExactPressureError::NonRepresentableLoad);
    }

    Ok(AxialState {
        wall_force_n,
        effective_force_n,
        axial_membrane_pa,
    })
}

/// Return the applied local external equivalent RHS caused by uniform thermal
/// and pressure-Poisson eigenstrain. This is not a recovered wall-action pair.
pub(crate) fn eigenload_pair(
    annulus: ExactAnnulus,
    material: IsotropicENu,
    pressure: InternalDifferentialPressure,
    thermal_strain: f64,
) -> Result<[f64; 2], ExactPressureError> {
    if !thermal_strain.is_finite() {
        return Err(ExactPressureError::InvalidStrain);
    }

    let poisson_stress_pa = (2.0 * material.nu) * pressure_trace_pa(annulus, pressure)?;
    let eigen_stress_pa = (-material.e_pa).mul_add(thermal_strain, poisson_stress_pa);
    if !eigen_stress_pa.is_finite() {
        return Err(ExactPressureError::NonRepresentableLoad);
    }

    let force_from_stress_n = checked_product3(
        annulus.as_m2,
        eigen_stress_pa,
        1.0,
        ExactPressureError::NonRepresentableLoad,
    )?;
    let i_rhs_n = if eigen_stress_pa.abs() < f64::MIN_POSITIVE {
        direct_eigen_force(annulus, material, pressure, thermal_strain)
            .filter(|value| *value != 0.0)
            .unwrap_or(force_from_stress_n)
    } else {
        force_from_stress_n
    };
    Ok([i_rhs_n, -i_rhs_n])
}

/// Return the pure local mathematical pressure-cap pair. Transfer and support
/// topology remain the caller's responsibility.
pub(crate) fn cap_pair(
    annulus: ExactAnnulus,
    pressure: InternalDifferentialPressure,
) -> Result<[f64; 2], ExactPressureError> {
    let fluid_force_n = fluid_force(annulus, pressure)?;
    Ok([-fluid_force_n, fluid_force_n])
}

fn fluid_force(
    annulus: ExactAnnulus,
    pressure: InternalDifferentialPressure,
) -> Result<f64, ExactPressureError> {
    let value = pressure.p_pa * annulus.ai_m2;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(ExactPressureError::NonRepresentableLoad)
    }
}

fn pressure_trace_pa(
    annulus: ExactAnnulus,
    pressure: InternalDifferentialPressure,
) -> Result<f64, ExactPressureError> {
    // For walls no thicker than the bore radius, normalize by ri. This keeps
    // the factored squared-radius difference resolved for adjacent radii and
    // for positive subnormal areas. For thicker walls, normalize by ro so the
    // radius ratio remains bounded and its denominator is cancellation-free.
    let relative_wall = (annulus.ro_m - annulus.ri_m) / annulus.ri_m;
    if relative_wall <= 1.0 {
        let normalized_span = relative_wall * (2.0 + relative_wall);
        checked_scaled_ratio(
            2.0,
            pressure.p_pa,
            normalized_span,
            ExactPressureError::NonRepresentableStress,
        )
    } else {
        let radius_ratio = annulus.ri_m / annulus.ro_m;
        let normalized_span = (1.0 - radius_ratio) * (1.0 + radius_ratio);
        let scaled_pressure = checked_product3(
            pressure.p_pa,
            radius_ratio,
            radius_ratio,
            ExactPressureError::NonRepresentableStress,
        )?;
        checked_scaled_ratio(
            2.0,
            scaled_pressure,
            normalized_span,
            ExactPressureError::NonRepresentableStress,
        )
    }
}

fn direct_axial_force(
    annulus: ExactAnnulus,
    material: IsotropicENu,
    pressure: InternalDifferentialPressure,
    thermal_strain: f64,
    epsilon_z: f64,
) -> Option<f64> {
    let strain_force_n = checked_product3(
        material.e_pa,
        annulus.as_m2,
        epsilon_z,
        ExactPressureError::NonRepresentableLoad,
    )
    .ok()?;
    let eigen_force_n = direct_eigen_force(annulus, material, pressure, thermal_strain)?;
    let wall_force_n = strain_force_n + eigen_force_n;
    wall_force_n.is_finite().then_some(wall_force_n)
}

fn direct_eigen_force(
    annulus: ExactAnnulus,
    material: IsotropicENu,
    pressure: InternalDifferentialPressure,
    thermal_strain: f64,
) -> Option<f64> {
    let poisson_force_n = direct_poisson_force(annulus, material, pressure)?;
    let thermal_force_n = checked_product3(
        material.e_pa,
        annulus.as_m2,
        thermal_strain,
        ExactPressureError::NonRepresentableLoad,
    )
    .ok()?;
    let eigen_force_n = poisson_force_n - thermal_force_n;
    eigen_force_n.is_finite().then_some(eigen_force_n)
}

fn direct_poisson_force(
    annulus: ExactAnnulus,
    material: IsotropicENu,
    pressure: InternalDifferentialPressure,
) -> Option<f64> {
    checked_product3(
        4.0,
        material.nu,
        fluid_force(annulus, pressure).ok()?,
        ExactPressureError::NonRepresentableLoad,
    )
    .ok()
}

fn checked_product3(
    a: f64,
    b: f64,
    c: f64,
    error: ExactPressureError,
) -> Result<f64, ExactPressureError> {
    if !a.is_finite() || !b.is_finite() || !c.is_finite() {
        return Err(error);
    }
    if a == 0.0 || b == 0.0 || c == 0.0 {
        return Ok((a * b) * c);
    }

    // Choose the association whose first product has an exponent closest to
    // zero. That keeps the intermediate normal whenever the factors permit,
    // instead of selecting an upward-rounded subnormal partial product merely
    // because its final magnitude is larger.
    let exponent_a = binary_exponent(a);
    let exponent_b = binary_exponent(b);
    let exponent_c = binary_exponent(c);
    let mut order = [
        ((exponent_a + exponent_b).abs(), 0usize),
        ((exponent_a + exponent_c).abs(), 1usize),
        ((exponent_b + exponent_c).abs(), 2usize),
    ];
    order.sort_by_key(|candidate| candidate.0);

    let mut zero = None;
    for (_, association) in order {
        let value = match association {
            0 => (a * b) * c,
            1 => (a * c) * b,
            _ => (b * c) * a,
        };
        if value.is_finite() {
            if value != 0.0 {
                return Ok(value);
            }
            zero = Some(value);
        }
    }
    zero.ok_or(error)
}

fn checked_scaled_ratio(
    a: f64,
    b: f64,
    denominator: f64,
    error: ExactPressureError,
) -> Result<f64, ExactPressureError> {
    if !a.is_finite() || !b.is_finite() || !denominator.is_finite() || denominator == 0.0 {
        return Err(error);
    }
    if a == 0.0 || b == 0.0 {
        return Ok((a * b) / denominator);
    }

    let exponent_a = binary_exponent(a);
    let exponent_b = binary_exponent(b);
    let exponent_denominator = binary_exponent(denominator);
    let mut order = [
        ((exponent_a + exponent_b).abs(), 0usize),
        ((exponent_a - exponent_denominator).abs(), 1usize),
        ((exponent_b - exponent_denominator).abs(), 2usize),
    ];
    order.sort_by_key(|candidate| candidate.0);

    let mut zero = None;
    for (_, association) in order {
        let value = match association {
            0 => (a * b) / denominator,
            1 => (a / denominator) * b,
            _ => (b / denominator) * a,
        };
        if value.is_finite() {
            if value != 0.0 {
                return Ok(value);
            }
            zero = Some(value);
        }
    }
    zero.ok_or(error)
}

fn binary_exponent(value: f64) -> i32 {
    let bits = value.abs().to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    if biased != 0 {
        biased - 1023
    } else {
        let fraction = bits & ((1u64 << 52) - 1);
        (63 - fraction.leading_zeros() as i32) - 1074
    }
}

fn is_positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON_MULTIPLIER: f64 = 128.0;
    const SUBNORMAL_ULP_FLOOR: f64 = 8.0 * f64::from_bits(1);

    fn assert_scalar(actual: f64, expected: f64, scale: f64) {
        assert!(actual.is_finite(), "actual value is non-finite: {actual}");
        let limit = EPSILON_MULTIPLIER * f64::EPSILON * scale.abs() + SUBNORMAL_ULP_FLOOR;
        assert!(
            (actual - expected).abs() <= limit,
            "{actual} != {expected}; residual={} limit={limit} scale={scale}",
            (actual - expected).abs()
        );
    }

    fn reference_inputs() -> (ExactAnnulus, IsotropicENu, InternalDifferentialPressure) {
        (
            ExactAnnulus::from_radii(1.0, 2.0).unwrap(),
            IsotropicENu::new(120.0, 0.25).unwrap(),
            InternalDifferentialPressure::new(3.0).unwrap(),
        )
    }

    fn assert_pair(actual: [f64; 2], expected: [f64; 2], scale: f64) {
        assert_scalar(actual[0], expected[0], scale);
        assert_scalar(actual[1], expected[1], scale);
        assert_scalar(actual[0] + actual[1], 0.0, scale);
    }

    #[test]
    fn annulus_accepts_reference_and_adjacent_representable_radii() {
        let reference = ExactAnnulus::from_radii(1.0, 2.0).unwrap();
        assert_eq!(reference.inner_radius_m(), 1.0);
        assert_eq!(reference.outer_radius_m(), 2.0);
        assert_scalar(reference.internal_area_m2(), PI, PI);
        assert_scalar(reference.wall_area_m2(), 3.0 * PI, 3.0 * PI);

        let adjacent_outer = f64::from_bits(1.0f64.to_bits() + 1);
        let adjacent = ExactAnnulus::from_radii(1.0, adjacent_outer).unwrap();
        let expected_wall_area = PI * f64::EPSILON * (2.0 + f64::EPSILON);
        assert!(adjacent.wall_area_m2() > 0.0);
        assert_scalar(
            adjacent.wall_area_m2(),
            expected_wall_area,
            expected_wall_area,
        );
    }

    #[test]
    fn annulus_rejects_invalid_and_nonrepresentable_geometry() {
        for (ri, ro) in [
            (f64::NAN, 2.0),
            (f64::INFINITY, 2.0),
            (f64::NEG_INFINITY, 2.0),
            (1.0, f64::NAN),
            (1.0, f64::INFINITY),
            (1.0, f64::NEG_INFINITY),
            (0.0, 2.0),
            (-1.0, 2.0),
            (2.0, 1.0),
            (1.0, 1.0),
        ] {
            assert_eq!(
                ExactAnnulus::from_radii(ri, ro),
                Err(ExactPressureError::InvalidGeometry)
            );
        }
        assert_eq!(
            ExactAnnulus::from_radii(1.0e-200, 2.0e-200),
            Err(ExactPressureError::NonRepresentableGeometry)
        );
        assert_eq!(
            ExactAnnulus::from_radii(1.0e200, 2.0e200),
            Err(ExactPressureError::NonRepresentableGeometry)
        );
    }

    #[test]
    fn material_derives_single_shear_modulus_authority() {
        let material = IsotropicENu::new(120.0, 0.25).unwrap();
        assert_eq!(material.elastic_modulus_pa(), 120.0);
        assert_eq!(material.poisson_ratio(), 0.25);
        assert_eq!(material.shear_modulus_pa(), 48.0);

        let midpoint = IsotropicENu::new(180.0, 7.0 / 24.0).unwrap();
        assert_scalar(midpoint.shear_modulus_pa(), 2160.0 / 31.0, 2160.0 / 31.0);
        assert!((midpoint.shear_modulus_pa() - 69.0).abs() > 0.5);
    }

    #[test]
    fn material_rejects_invalid_bounds_and_nonrepresentable_g() {
        for (e, nu) in [
            (f64::NAN, 0.25),
            (f64::INFINITY, 0.25),
            (f64::NEG_INFINITY, 0.25),
            (120.0, f64::NAN),
            (120.0, f64::INFINITY),
            (120.0, f64::NEG_INFINITY),
            (0.0, 0.25),
            (-1.0, 0.25),
            (120.0, -1.0),
            (120.0, 0.5),
            (120.0, -2.0),
            (120.0, 1.0),
        ] {
            assert_eq!(
                IsotropicENu::new(e, nu),
                Err(ExactPressureError::InvalidMaterial)
            );
        }
        assert_eq!(
            IsotropicENu::new(f64::MAX, f64::from_bits((-1.0f64).to_bits() - 1)),
            Err(ExactPressureError::NonRepresentableMaterial)
        );
        assert_eq!(
            IsotropicENu::new(f64::from_bits(1), 0.49),
            Err(ExactPressureError::NonRepresentableMaterial)
        );
    }

    #[test]
    fn pressure_accepts_finite_signed_values_only() {
        for pressure in [3.0, 0.0, -3.0, f64::from_bits(1)] {
            assert_eq!(
                InternalDifferentialPressure::new(pressure)
                    .unwrap()
                    .pressure_pa(),
                pressure
            );
        }
        for pressure in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                InternalDifferentialPressure::new(pressure),
                Err(ExactPressureError::InvalidPressure)
            );
        }
    }

    #[test]
    fn lame_matches_boundaries_interior_and_constant_sum() {
        let (annulus, _, pressure) = reference_inputs();
        let inner = lame_at_radius(annulus, pressure, 1.0).unwrap();
        let middle = lame_at_radius(annulus, pressure, 1.5).unwrap();
        let outer = lame_at_radius(annulus, pressure, 2.0).unwrap();

        assert_scalar(inner.radial_pa(), -3.0, 3.0);
        assert_scalar(inner.hoop_pa(), 5.0, 5.0);
        assert_scalar(outer.radial_pa(), 0.0, 3.0);
        assert_scalar(outer.hoop_pa(), 2.0, 2.0);
        assert_scalar(middle.radial_pa(), -7.0 / 9.0, 7.0 / 9.0);
        assert_scalar(middle.hoop_pa(), 25.0 / 9.0, 25.0 / 9.0);
        for state in [inner, middle, outer] {
            assert_scalar(state.radial_pa() + state.hoop_pa(), 2.0, 2.0);
        }
    }

    #[test]
    fn lame_uses_pressure_scale_for_adjacent_radius_boundary_tractions() {
        let ro = f64::from_bits(1.0f64.to_bits() + 1);
        let annulus = ExactAnnulus::from_radii(1.0, ro).unwrap();
        let pressure = InternalDifferentialPressure::new(3.0).unwrap();
        let inner = lame_at_radius(annulus, pressure, 1.0).unwrap();
        let outer = lame_at_radius(annulus, pressure, ro).unwrap();

        assert_scalar(inner.radial_pa(), -3.0, 3.0);
        assert_scalar(outer.radial_pa(), 0.0, 3.0);
        assert!(inner.hoop_pa().is_finite());
        assert!(outer.hoop_pa().is_finite());
        assert!(inner.hoop_pa().abs() > 1.0e15);

        // The factored wall area can be representable even when the
        // unscaled squared-radius difference rounds to zero.
        let tiny_inner = 2.0f64.powi(-537);
        let tiny_outer = tiny_inner * 1.125;
        assert_eq!((tiny_outer - tiny_inner) * (tiny_outer + tiny_inner), 0.0);
        let tiny_annulus = ExactAnnulus::from_radii(tiny_inner, tiny_outer).unwrap();
        assert_eq!(tiny_annulus.wall_area_m2(), f64::from_bits(1));
        let tiny_inner_stress = lame_at_radius(tiny_annulus, pressure, tiny_inner).unwrap();
        let tiny_outer_stress = lame_at_radius(tiny_annulus, pressure, tiny_outer).unwrap();
        assert_scalar(tiny_inner_stress.radial_pa(), -3.0, 3.0);
        assert_scalar(tiny_outer_stress.radial_pa(), 0.0, 3.0);
    }

    #[test]
    fn lame_rejects_radii_outside_closed_wall_interval() {
        let (annulus, _, pressure) = reference_inputs();
        for radius in [
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::from_bits(1.0f64.to_bits() - 1),
            f64::from_bits(2.0f64.to_bits() + 1),
        ] {
            assert_eq!(
                lame_at_radius(annulus, pressure, radius),
                Err(ExactPressureError::RadiusOutsideWall)
            );
        }
    }

    #[test]
    fn lame_zero_and_signed_pressure_scale_linearly() {
        let (annulus, _, positive) = reference_inputs();
        let zero = InternalDifferentialPressure::new(0.0).unwrap();
        let negative = InternalDifferentialPressure::new(-3.0).unwrap();
        let positive_state = lame_at_radius(annulus, positive, 1.5).unwrap();
        let negative_state = lame_at_radius(annulus, negative, 1.5).unwrap();
        let zero_state = lame_at_radius(annulus, zero, 1.5).unwrap();
        assert_scalar(zero_state.radial_pa(), 0.0, 0.0);
        assert_scalar(zero_state.hoop_pa(), 0.0, 0.0);
        assert_scalar(
            negative_state.radial_pa(),
            -positive_state.radial_pa(),
            positive_state.radial_pa(),
        );
        assert_scalar(
            negative_state.hoop_pa(),
            -positive_state.hoop_pa(),
            positive_state.hoop_pa(),
        );
    }

    fn assert_axial_case(
        thermal_strain: f64,
        epsilon_z: f64,
        expected_wall_over_pi: f64,
        expected_effective_over_pi: f64,
        expected_stress: f64,
        expected_eigen_i_over_pi: f64,
    ) {
        let (annulus, material, pressure) = reference_inputs();
        let state = axial_state(annulus, material, pressure, thermal_strain, epsilon_z).unwrap();
        let eigen = eigenload_pair(annulus, material, pressure, thermal_strain).unwrap();
        let force_scale = 20.0 * PI;
        assert_scalar(
            state.wall_force_n(),
            expected_wall_over_pi * PI,
            force_scale,
        );
        assert_scalar(
            state.effective_force_n(),
            expected_effective_over_pi * PI,
            force_scale,
        );
        assert_scalar(state.axial_membrane_pa(), expected_stress, 3.0);
        assert_pair(
            eigen,
            [
                expected_eigen_i_over_pi * PI,
                -expected_eigen_i_over_pi * PI,
            ],
            force_scale,
        );
    }

    #[test]
    fn four_rational_pi_states_match_frozen_oracle() {
        assert_axial_case(0.0, 1.0 / 240.0, 3.0, 0.0, 1.0, 1.5);
        assert_axial_case(0.0, 0.0, 1.5, -1.5, 0.5, 1.5);
        assert_axial_case(0.0, -1.0 / 240.0, 0.0, -3.0, 0.0, 1.5);
        assert_axial_case(1.0 / 1000.0, 31.0 / 6000.0, 3.0, 0.0, 1.0, 57.0 / 50.0);
        assert_axial_case(
            1.0 / 1000.0,
            0.0,
            57.0 / 50.0,
            -93.0 / 50.0,
            19.0 / 50.0,
            57.0 / 50.0,
        );
    }

    #[test]
    fn wall_effective_and_membrane_identities_remain_independent() {
        let (annulus, material, pressure) = reference_inputs();
        let state = axial_state(annulus, material, pressure, 0.0, 0.0).unwrap();
        let fluid_force = pressure.pressure_pa() * annulus.internal_area_m2();
        assert_scalar(
            state.effective_force_n(),
            state.wall_force_n() - fluid_force,
            state.wall_force_n().abs() + fluid_force.abs(),
        );
        assert_scalar(
            state.axial_membrane_pa(),
            state.wall_force_n() / annulus.wall_area_m2(),
            state.axial_membrane_pa(),
        );
        assert!(state.wall_force_n() > 0.0, "positive wall force is tension");
        assert_scalar(state.axial_membrane_pa(), 0.5, 0.5);
        assert!((state.axial_membrane_pa() - 1.5).abs() > 0.5);
    }

    #[test]
    fn cap_and_applied_eigen_rhs_have_frozen_local_signs() {
        let (annulus, material, pressure) = reference_inputs();
        assert_pair(
            cap_pair(annulus, pressure).unwrap(),
            [-3.0 * PI, 3.0 * PI],
            3.0 * PI,
        );
        assert_pair(
            eigenload_pair(annulus, material, pressure, 0.0).unwrap(),
            [1.5 * PI, -1.5 * PI],
            1.5 * PI,
        );
        let reversed = InternalDifferentialPressure::new(-3.0).unwrap();
        assert_pair(
            cap_pair(annulus, reversed).unwrap(),
            [3.0 * PI, -3.0 * PI],
            3.0 * PI,
        );
    }

    #[test]
    fn reductions_auxetic_behavior_and_superposition_hold() {
        let annulus = ExactAnnulus::from_radii(1.0, 2.0).unwrap();
        let pressure = InternalDifferentialPressure::new(3.0).unwrap();
        let zero_pressure = InternalDifferentialPressure::new(0.0).unwrap();
        let nu_zero = IsotropicENu::new(120.0, 0.0).unwrap();
        let auxetic = IsotropicENu::new(120.0, -0.5).unwrap();

        let no_poisson = axial_state(annulus, nu_zero, pressure, 0.0, 0.0).unwrap();
        assert_scalar(no_poisson.wall_force_n(), 0.0, 3.0 * PI);
        assert_pair(
            eigenload_pair(annulus, nu_zero, pressure, 0.0).unwrap(),
            [0.0, 0.0],
            3.0 * PI,
        );

        let thermal_only = axial_state(annulus, nu_zero, zero_pressure, 1.0 / 1024.0, 0.0).unwrap();
        assert!(thermal_only.wall_force_n() < 0.0);
        let auxetic_state = axial_state(annulus, auxetic, pressure, 0.0, 0.0).unwrap();
        assert!(auxetic_state.wall_force_n() < 0.0);

        let combined = axial_state(annulus, nu_zero, pressure, 1.0 / 1024.0, 1.0 / 512.0).unwrap();
        let strain_only = axial_state(annulus, nu_zero, zero_pressure, 0.0, 1.0 / 512.0).unwrap();
        assert_scalar(
            combined.wall_force_n(),
            thermal_only.wall_force_n() + strain_only.wall_force_n(),
            thermal_only.wall_force_n().abs() + strain_only.wall_force_n().abs(),
        );
    }

    #[test]
    fn numeric_edge_cases_preserve_frozen_outputs() {
        for (ri, ro, e, p, expected_wall, expected_axial, expected_inner_hoop) in [
            (
                2.0f64.powi(-200),
                2.0f64.powi(-199),
                2.0f64.powi(200),
                2.0f64.powi(100),
                7.711185983182499e-91,
                2.1127510003803822e29,
                2.1127510003803824e30,
            ),
            (
                2.0f64.powi(200),
                2.0f64.powi(201),
                2.0f64.powi(-200),
                2.0f64.powi(-100),
                3.1997686291752666e90,
                1.314768175368353e-31,
                1.314768175368353e-30,
            ),
        ] {
            let annulus = ExactAnnulus::from_radii(ri, ro).unwrap();
            let material = IsotropicENu::new(e, 0.25).unwrap();
            let pressure = InternalDifferentialPressure::new(p).unwrap();
            let axial = axial_state(annulus, material, pressure, 0.0, 0.0).unwrap();
            let inner = lame_at_radius(annulus, pressure, ri).unwrap();
            assert_scalar(axial.wall_force_n(), expected_wall, expected_wall.abs());
            assert_scalar(
                axial.axial_membrane_pa(),
                expected_axial,
                expected_axial.abs(),
            );
            assert_scalar(inner.radial_pa(), -p, p.abs());
            assert_scalar(
                inner.hoop_pa(),
                expected_inner_hoop,
                expected_inner_hoop.abs(),
            );
        }
    }

    #[test]
    fn subnormal_pressure_rounding_is_accepted_without_blanket_rejection() {
        let annulus = ExactAnnulus::from_radii(1.0, 2.0).unwrap();
        let material = IsotropicENu::new(120.0, 0.25).unwrap();
        let pressure = InternalDifferentialPressure::new(f64::from_bits(1)).unwrap();
        let axial = axial_state(annulus, material, pressure, 0.0, 0.0).unwrap();
        let inner = lame_at_radius(annulus, pressure, 1.0).unwrap();
        let outer = lame_at_radius(annulus, pressure, 2.0).unwrap();
        let cap = cap_pair(annulus, pressure).unwrap();
        let eigen = eigenload_pair(annulus, material, pressure, 0.0).unwrap();
        assert!(axial.wall_force_n().is_finite());
        assert!(axial.axial_membrane_pa().is_finite());
        assert_scalar(inner.radial_pa(), -f64::from_bits(1), f64::from_bits(1));
        assert_scalar(outer.radial_pa(), 0.0, f64::from_bits(1));
        assert!(cap.into_iter().all(f64::is_finite));
        assert!(eigen.into_iter().all(f64::is_finite));
    }

    #[test]
    fn eigen_rhs_avoids_underflowing_eigenstrain_intermediate() {
        let annulus = ExactAnnulus::from_radii(1.0, 2.0).unwrap();
        let material = IsotropicENu::new(1.0e308, 0.25).unwrap();
        let pressure = InternalDifferentialPressure::new(3.0).unwrap();
        let pair = eigenload_pair(annulus, material, pressure, 0.0).unwrap();
        assert_pair(pair, [1.5 * PI, -1.5 * PI], 1.5 * PI);

        let thermal_pair = eigenload_pair(annulus, material, pressure, 1.0e-308).unwrap();
        let expected_i = 1.5 * PI - (1.0e308 * 1.0e-308) * (3.0 * PI);
        assert_pair(thermal_pair, [expected_i, -expected_i], 12.0 * PI);
    }

    #[test]
    fn subnormal_area_intermediates_do_not_erase_representable_stress() {
        let ri = 2.0f64.powi(-537);
        let ro = ri * 1.125;
        let radius = ri + (ro - ri) / 2.0;
        let annulus = ExactAnnulus::from_radii(ri, ro).unwrap();
        let material = IsotropicENu::new(120.0, 0.25).unwrap();
        let pressure = InternalDifferentialPressure::new(3.0).unwrap();
        assert_eq!(annulus.wall_area_m2(), f64::from_bits(1));

        let stress = lame_at_radius(annulus, pressure, radius).unwrap();
        let outer = lame_at_radius(annulus, pressure, ro).unwrap();
        let expected_radial = -6720.0 / 4913.0;
        let expected_trace = 384.0 / 17.0;
        assert_scalar(stress.radial_pa(), expected_radial, expected_radial.abs());
        assert_scalar(
            stress.hoop_pa(),
            expected_trace - expected_radial,
            expected_trace + expected_radial.abs(),
        );
        assert_scalar(outer.hoop_pa(), expected_trace, expected_trace);

        let axial = axial_state(annulus, material, pressure, 0.0, 0.0).unwrap();
        assert_scalar(axial.axial_membrane_pa(), 96.0 / 17.0, 96.0 / 17.0);
    }

    #[test]
    fn underflowed_fluid_force_does_not_erase_representable_stress() {
        let ri = 2.0f64.powi(-400);
        let ro = 2.0f64.powi(-399);
        let pressure_value = 2.0f64.powi(-400);
        let annulus = ExactAnnulus::from_radii(ri, ro).unwrap();
        let material = IsotropicENu::new(120.0, 0.25).unwrap();
        let pressure = InternalDifferentialPressure::new(pressure_value).unwrap();

        assert_eq!(pressure_value * annulus.internal_area_m2(), 0.0);
        let inner = lame_at_radius(annulus, pressure, ri).unwrap();
        assert_scalar(inner.radial_pa(), -pressure_value, pressure_value);
        assert_scalar(
            inner.hoop_pa(),
            5.0 * pressure_value / 3.0,
            5.0 * pressure_value / 3.0,
        );
        let outer = lame_at_radius(annulus, pressure, ro).unwrap();
        assert_scalar(
            outer.hoop_pa(),
            2.0 * pressure_value / 3.0,
            2.0 * pressure_value / 3.0,
        );

        let axial = axial_state(annulus, material, pressure, 0.0, 0.0).unwrap();
        assert_scalar(
            axial.axial_membrane_pa(),
            pressure_value / 6.0,
            pressure_value / 6.0,
        );
    }

    #[test]
    fn balanced_product_preserves_wall_force_from_subnormal_partial_product() {
        let annulus = ExactAnnulus::from_radii(2.0f64.powi(-500), 2.0f64.powi(-499)).unwrap();
        let material = IsotropicENu::new(2.0f64.powi(600), 0.25).unwrap();
        let pressure = InternalDifferentialPressure::new(0.0).unwrap();
        let state = axial_state(annulus, material, pressure, 0.0, 2.0f64.powi(-78)).unwrap();
        let expected_force = 3.0 * PI * 2.0f64.powi(-478);
        assert_scalar(state.wall_force_n(), expected_force, expected_force);
        assert_scalar(
            state.axial_membrane_pa(),
            2.0f64.powi(522),
            2.0f64.powi(522),
        );
    }

    #[test]
    fn balanced_product_preserves_applied_eigen_rhs_from_subnormal_partial_product() {
        let annulus = ExactAnnulus::from_radii(2.0f64.powi(-500), 2.0f64.powi(-499)).unwrap();
        let material = IsotropicENu::new(2.0f64.powi(600), 0.25).unwrap();
        let pressure = InternalDifferentialPressure::new(0.0).unwrap();
        let expected_force = 3.0 * PI * 2.0f64.powi(-478);
        assert_pair(
            eigenload_pair(annulus, material, pressure, 2.0f64.powi(-78)).unwrap(),
            [-expected_force, expected_force],
            expected_force,
        );
    }

    fn overflowing_force_cancellation_inputs() -> (
        ExactAnnulus,
        IsotropicENu,
        InternalDifferentialPressure,
        f64,
    ) {
        (
            ExactAnnulus::from_radii(1.0, 1024.0).unwrap(),
            IsotropicENu::new(2.0f64.powi(1022), -0.75).unwrap(),
            InternalDifferentialPressure::new(2.0f64.powi(1022)).unwrap(),
            -1.5 / (2.0f64.powi(20) - 1.0),
        )
    }

    #[test]
    fn overflowing_force_terms_cancel_to_representable_axial_state() {
        let (annulus, material, pressure, thermal_strain) = overflowing_force_cancellation_inputs();
        let state = axial_state(annulus, material, pressure, thermal_strain, 0.0).unwrap();
        assert_scalar(state.wall_force_n(), -1.836948414308395e290, 1.0e308);
        assert_scalar(state.effective_force_n(), -1.4119048864730642e308, f64::MAX);
        assert_scalar(state.axial_membrane_pa(), -5.576318724783634e283, 1.0e302);
    }

    #[test]
    fn overflowing_force_terms_cancel_to_representable_applied_eigen_rhs() {
        let (annulus, material, pressure, thermal_strain) = overflowing_force_cancellation_inputs();
        assert_pair(
            eigenload_pair(annulus, material, pressure, thermal_strain).unwrap(),
            [-1.836948414308395e290, 1.836948414308395e290],
            1.0e308,
        );
    }

    #[test]
    fn opposed_finite_strains_do_not_overflow_before_stress_scaling() {
        let annulus = ExactAnnulus::from_radii(1.0, 2.0).unwrap();
        let material = IsotropicENu::new(2.0f64.powi(-1000), 0.25).unwrap();
        let pressure = InternalDifferentialPressure::new(0.0).unwrap();
        let epsilon_z = 2.0f64.powi(1023);
        let thermal_strain = -epsilon_z;
        let expected_wall_force = 3.0 * PI * 2.0f64.powi(24);
        let expected_eigen_i = 3.0 * PI * 2.0f64.powi(23);

        let state = axial_state(annulus, material, pressure, thermal_strain, epsilon_z).unwrap();
        assert_scalar(state.axial_membrane_pa(), 2.0f64.powi(24), 2.0f64.powi(24));
        assert_scalar(
            state.wall_force_n(),
            expected_wall_force,
            expected_wall_force,
        );
        assert_scalar(
            state.effective_force_n(),
            expected_wall_force,
            expected_wall_force,
        );
        assert_pair(
            eigenload_pair(annulus, material, pressure, thermal_strain).unwrap(),
            [expected_eigen_i, -expected_eigen_i],
            expected_eigen_i,
        );
    }

    #[test]
    fn nonfinite_strains_and_true_output_overflow_return_errors() {
        let (annulus, material, pressure) = reference_inputs();
        for strain in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                axial_state(annulus, material, pressure, strain, 0.0),
                Err(ExactPressureError::InvalidStrain)
            );
            assert_eq!(
                axial_state(annulus, material, pressure, 0.0, strain),
                Err(ExactPressureError::InvalidStrain)
            );
            assert_eq!(
                eigenload_pair(annulus, material, pressure, strain),
                Err(ExactPressureError::InvalidStrain)
            );
        }

        let adjacent = ExactAnnulus::from_radii(1.0, f64::from_bits(1.0f64.to_bits() + 1)).unwrap();
        let maximum_pressure = InternalDifferentialPressure::new(f64::MAX).unwrap();
        assert!(lame_at_radius(adjacent, maximum_pressure, 1.0).is_err());
        assert!(cap_pair(annulus, maximum_pressure).is_err());

        let large_material = IsotropicENu::new(f64::MAX / 4.0, 0.25).unwrap();
        let zero_pressure = InternalDifferentialPressure::new(0.0).unwrap();
        assert!(axial_state(annulus, large_material, zero_pressure, 0.0, 2.0).is_err());
        assert!(eigenload_pair(annulus, large_material, zero_pressure, 2.0).is_err());
    }
}
