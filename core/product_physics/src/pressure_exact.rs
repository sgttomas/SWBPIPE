//! Exact-annulus pressure scalar mechanics.
//!
//! This private module contains only validated scalar mechanics and local
//! element pairs. The pressure runtime composes these with explicit region
//! topology, assembly, wall-force recovery and profile-qualified publication.

#[cfg(test)]
use std::f64::consts::PI;

mod source_geometry;
pub(crate) use source_geometry::{
    ExactSectionBases, SectionBasis, SourceAnnulus, ARC_PRESSURE_STRAIN_ROUNDINGS,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Scaled {
    mantissa: f64,
    exponent: i32,
}

impl Scaled {
    pub(super) fn from_f64(value: f64) -> Self {
        debug_assert!(value.is_finite());
        if value == 0.0 {
            return Self {
                mantissa: value,
                exponent: 0,
            };
        }

        let bits = value.to_bits();
        let sign = bits & (1u64 << 63);
        let biased = ((bits >> 52) & 0x7ff) as i32;
        let fraction = bits & ((1u64 << 52) - 1);
        if biased != 0 {
            Self {
                mantissa: f64::from_bits(sign | (1022u64 << 52) | fraction),
                exponent: biased - 1022,
            }
        } else {
            let highest_bit = 63 - fraction.leading_zeros() as i32;
            let magnitude = (fraction as f64) / power_of_two(highest_bit + 1);
            Self {
                mantissa: magnitude.copysign(value),
                exponent: highest_bit + 1 - 1074,
            }
        }
    }

    fn from_parts(mantissa: f64, exponent: i32) -> Self {
        let normalized = Self::from_f64(mantissa);
        if normalized.mantissa == 0.0 {
            normalized
        } else {
            Self {
                mantissa: normalized.mantissa,
                exponent: normalized.exponent + exponent,
            }
        }
    }

    fn neg(self) -> Self {
        Self {
            mantissa: -self.mantissa,
            exponent: self.exponent,
        }
    }

    pub(super) fn add(self, other: Self) -> Self {
        if self.mantissa == 0.0 {
            return other;
        }
        if other.mantissa == 0.0 {
            return self;
        }

        let exponent = self.exponent.max(other.exponent);
        let left = align_mantissa(self.mantissa, self.exponent - exponent);
        let right = align_mantissa(other.mantissa, other.exponent - exponent);
        Self::from_parts(left + right, exponent)
    }

    pub(super) fn sub(self, other: Self) -> Self {
        self.add(other.neg())
    }

    pub(super) fn mul(self, other: Self) -> Self {
        if self.mantissa == 0.0 || other.mantissa == 0.0 {
            return Self::from_f64(self.mantissa * other.mantissa);
        }
        Self::from_parts(
            self.mantissa * other.mantissa,
            self.exponent + other.exponent,
        )
    }

    pub(super) fn div(self, other: Self) -> Option<Self> {
        if other.mantissa == 0.0 {
            return None;
        }
        if self.mantissa == 0.0 {
            return Some(Self::from_f64(self.mantissa / other.mantissa));
        }
        Some(Self::from_parts(
            self.mantissa / other.mantissa,
            self.exponent - other.exponent,
        ))
    }

    fn fused_mul_add(self, factor: Self, addend: Self) -> Self {
        if self.mantissa == 0.0 || factor.mantissa == 0.0 {
            return addend.add(self.mul(factor));
        }
        if addend.mantissa == 0.0 {
            return self.mul(factor);
        }

        let product_exponent = self.exponent + factor.exponent;
        let exponent = product_exponent.max(addend.exponent);
        let scaled_factor = align_mantissa(factor.mantissa, product_exponent - exponent);
        let scaled_addend = align_mantissa(addend.mantissa, addend.exponent - exponent);
        Self::from_parts(
            self.mantissa.mul_add(scaled_factor, scaled_addend),
            exponent,
        )
    }

    /// Readonly exact dyadic representation used by numerical enclosures.
    pub(crate) fn dyadic_parts(self)->(f64,i32) {(self.mantissa,self.exponent)}

    pub(super) fn to_f64(self) -> f64 {
        if self.mantissa == 0.0 {
            return self.mantissa;
        }
        if self.exponent > 1024 {
            return f64::INFINITY.copysign(self.mantissa);
        }
        if self.exponent == 1024 {
            return (self.mantissa * power_of_two(1023)) * 2.0;
        }
        if self.exponent < -1074 {
            return 0.0f64.copysign(self.mantissa);
        }
        self.mantissa * power_of_two(self.exponent)
    }
}

fn align_mantissa(mantissa: f64, exponent_shift: i32) -> f64 {
    debug_assert!(exponent_shift <= 0);
    if exponent_shift < -1074 {
        0.0f64.copysign(mantissa)
    } else {
        mantissa * power_of_two(exponent_shift)
    }
}

fn power_of_two(exponent: i32) -> f64 {
    debug_assert!((-1074..=1023).contains(&exponent));
    if exponent >= -1022 {
        f64::from_bits(((exponent + 1023) as u64) << 52)
    } else {
        f64::from_bits(1u64 << (exponent + 1074))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExactPressureError {
    InvalidGeometry,
    NonRepresentableGeometry,
    InvalidMaterial,
    NonRepresentableMaterial,
    InvalidPressure,
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
    ai_scaled: Scaled,
    as_scaled: Scaled,
}

impl ExactAnnulus {
    /// The kernel tests' constructor from radii; the product builds the kernel
    /// from a section (`source_geometry`).
    #[cfg(test)]
    pub(crate) fn from_radii(ri_m: f64, ro_m: f64) -> Result<Self, ExactPressureError> {
        if !ri_m.is_finite() || !ro_m.is_finite() || ri_m <= 0.0 || ri_m >= ro_m {
            return Err(ExactPressureError::InvalidGeometry);
        }

        let ri_scaled = Scaled::from_f64(ri_m);
        let ro_scaled = Scaled::from_f64(ro_m);
        let pi_scaled = Scaled::from_f64(PI);
        let ai_scaled = pi_scaled.mul(ri_scaled).mul(ri_scaled);
        let as_scaled = pi_scaled
            .mul(ro_scaled.sub(ri_scaled))
            .mul(ro_scaled.add(ri_scaled));
        let ai_m2 = ai_scaled.to_f64();
        let as_m2 = as_scaled.to_f64();
        if !is_positive_finite(ai_m2) || !is_positive_finite(as_m2) {
            return Err(ExactPressureError::NonRepresentableGeometry);
        }

        Ok(Self {
            ri_m,
            ro_m,
            ai_m2,
            as_m2,
            ai_scaled,
            as_scaled,
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

        let denominator =
            Scaled::from_f64(2.0).mul(Scaled::from_f64(1.0).add(Scaled::from_f64(nu)));
        let g_pa = Scaled::from_f64(e_pa)
            .div(denominator)
            .expect("validated Poisson ratio gives positive denominator")
            .to_f64();
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

    let fluid_force = fluid_force_scaled(annulus, pressure);
    let poisson_force = Scaled::from_f64(2.0)
        .mul(Scaled::from_f64(material.nu))
        .mul(fluid_force);
    let elastic_wall_scale = Scaled::from_f64(material.e_pa).mul(annulus.as_scaled);
    let i_rhs =
        elastic_wall_scale.fused_mul_add(Scaled::from_f64(thermal_strain).neg(), poisson_force);
    let i_rhs_n = scaled_output(i_rhs, ExactPressureError::NonRepresentableLoad)?;
    Ok([i_rhs_n, -i_rhs_n])
}

/// Return the pure local mathematical pressure-cap pair. Transfer and support
/// topology remain the caller's responsibility.
pub(crate) fn cap_pair(
    annulus: ExactAnnulus,
    pressure: InternalDifferentialPressure,
) -> Result<[f64; 2], ExactPressureError> {
    let fluid_force_n = scaled_output(
        fluid_force_scaled(annulus, pressure),
        ExactPressureError::NonRepresentableLoad,
    )?;
    Ok([-fluid_force_n, fluid_force_n])
}

fn fluid_force_scaled(annulus: ExactAnnulus, pressure: InternalDifferentialPressure) -> Scaled {
    Scaled::from_f64(pressure.p_pa).mul(annulus.ai_scaled)
}

fn pressure_trace_scaled(annulus: ExactAnnulus, pressure: InternalDifferentialPressure) -> Scaled {
    Scaled::from_f64(2.0)
        .mul(fluid_force_scaled(annulus, pressure))
        .div(annulus.as_scaled)
        .expect("validated annulus has positive wall area")
}

fn scaled_output(value: Scaled, error: ExactPressureError) -> Result<f64, ExactPressureError> {
    let output = value.to_f64();
    if output.is_finite() {
        Ok(output)
    } else {
        Err(error)
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

    /// The applied eigen RHS of the reference annulus at a thermal strain:
    /// [i, j] = [k, -k] * pi.
    fn assert_eigen_case(thermal_strain: f64, expected_eigen_i_over_pi: f64) {
        let (annulus, material, pressure) = reference_inputs();
        let eigen = eigenload_pair(annulus, material, pressure, thermal_strain).unwrap();
        assert_pair(
            eigen,
            [
                expected_eigen_i_over_pi * PI,
                -expected_eigen_i_over_pi * PI,
            ],
            20.0 * PI,
        );
    }

    #[test]
    fn rational_pi_eigen_states_match_frozen_oracle() {
        assert_eigen_case(0.0, 1.5);
        assert_eigen_case(1.0 / 1000.0, 57.0 / 50.0);
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
    fn eigen_rhs_without_poisson_coupling_is_zero_under_pressure() {
        let annulus = ExactAnnulus::from_radii(1.0, 2.0).unwrap();
        let pressure = InternalDifferentialPressure::new(3.0).unwrap();
        let nu_zero = IsotropicENu::new(120.0, 0.0).unwrap();
        assert_pair(
            eigenload_pair(annulus, nu_zero, pressure, 0.0).unwrap(),
            [0.0, 0.0],
            3.0 * PI,
        );
    }

    #[test]
    fn subnormal_pressure_rounding_is_accepted_without_blanket_rejection() {
        let annulus = ExactAnnulus::from_radii(1.0, 2.0).unwrap();
        let material = IsotropicENu::new(120.0, 0.25).unwrap();
        let pressure = InternalDifferentialPressure::new(f64::from_bits(1)).unwrap();
        let cap = cap_pair(annulus, pressure).unwrap();
        let eigen = eigenload_pair(annulus, material, pressure, 0.0).unwrap();
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
        let expected_eigen_i = 3.0 * PI * 2.0f64.powi(23);
        assert_pair(
            eigenload_pair(annulus, material, pressure, thermal_strain).unwrap(),
            [expected_eigen_i, -expected_eigen_i],
            expected_eigen_i,
        );
    }

    #[test]
    fn rounded_subnormal_area_does_not_control_normal_force_outputs() {
        let ri = 2.0f64.powi(-537);
        let ro = 2.0f64.powi(-536);
        let annulus = ExactAnnulus::from_radii(ri, ro).unwrap();
        let material = IsotropicENu::new(120.0, 0.25).unwrap();

        for pressure_exponent in [199, 200, 201] {
            for sign in [1.0, -1.0] {
                let pressure_value = sign * 2.0f64.powi(pressure_exponent);
                let pressure = InternalDifferentialPressure::new(pressure_value).unwrap();
                let expected_fluid_force = sign * PI * 2.0f64.powi(pressure_exponent - 1074);
                let expected_wall_force = 0.5 * expected_fluid_force;

                assert_pair(
                    cap_pair(annulus, pressure).unwrap(),
                    [-expected_fluid_force, expected_fluid_force],
                    expected_fluid_force.abs(),
                );
                assert_pair(
                    eigenload_pair(annulus, material, pressure, 0.0).unwrap(),
                    [expected_wall_force, -expected_wall_force],
                    expected_wall_force.abs(),
                );
            }
        }
    }

    #[test]
    fn final_force_conversion_preserves_gradual_underflow_and_signed_zero() {
        let pressure = InternalDifferentialPressure::new(f64::from_bits(1)).unwrap();

        let rounds_up = ExactAnnulus::from_radii(0.5, 1.0).unwrap();
        let rounded_pair = cap_pair(rounds_up, pressure).unwrap();
        assert_eq!(rounded_pair[0].to_bits(), (-f64::from_bits(1)).to_bits());
        assert_eq!(rounded_pair[1].to_bits(), f64::from_bits(1).to_bits());

        let rounds_to_zero = ExactAnnulus::from_radii(0.25, 1.0).unwrap();
        let zero_pair = cap_pair(rounds_to_zero, pressure).unwrap();
        assert_eq!(zero_pair[0].to_bits(), (-0.0f64).to_bits());
        assert_eq!(zero_pair[1].to_bits(), 0.0f64.to_bits());
    }

    #[test]
    fn nonfinite_strains_and_true_output_overflow_return_errors() {
        let (annulus, material, pressure) = reference_inputs();
        for strain in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                eigenload_pair(annulus, material, pressure, strain),
                Err(ExactPressureError::InvalidStrain)
            );
        }

        let maximum_pressure = InternalDifferentialPressure::new(f64::MAX).unwrap();
        assert!(cap_pair(annulus, maximum_pressure).is_err());

        let large_material = IsotropicENu::new(f64::MAX / 4.0, 0.25).unwrap();
        let zero_pressure = InternalDifferentialPressure::new(0.0).unwrap();
        assert!(eigenload_pair(annulus, large_material, zero_pressure, 2.0).is_err());
    }
}
