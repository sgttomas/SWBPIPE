//! Active source-OD/wall annulus, separate from the preserved radii-only scalar API.
use super::{
    cap_pair, eigenload_pair, pressure_trace_scaled, scaled_output, ExactAnnulus,
    ExactPressureError, InternalDifferentialPressure, IsotropicENu, RadialHoopStressPa, Scaled,
};
use crate::annulus_geometry::{source_bore_area_scaled, source_wall_area_scaled, validate_wall};

#[derive(Debug, Clone, Copy)]
pub(crate) struct SourceAnnulus {
    od_m: f64,
    wall_m: f64,
    // Reuse existing cap/eigen constitutive arithmetic through its actual areas.
    // This private value is never passed to the radii-only interior Lamé API.
    kernel: ExactAnnulus,
    i_m4: f64,
    j_m4: f64,
    z_m3: f64,
}
fn positive(value: Scaled) -> Result<f64, ExactPressureError> {
    let value = value.to_f64();
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(ExactPressureError::NonRepresentableGeometry)
    }
}
impl SourceAnnulus {
    pub(crate) fn from_od_wall(od_m: f64, wall_m: f64) -> Result<Self, ExactPressureError> {
        validate_wall(od_m, wall_m).map_err(|_| ExactPressureError::InvalidGeometry)?;
        let ro_m = od_m * 0.5;
        let ri_m = ro_m - wall_m;
        // Retained active-frame admission guard, distinct from pure area validity.
        if !ro_m.is_finite() || ri_m <= 0.0 || ri_m >= ro_m {
            return Err(ExactPressureError::InvalidGeometry);
        }
        let as_scaled = source_wall_area_scaled(od_m, wall_m)
            .map_err(|_| ExactPressureError::InvalidGeometry)?;
        let ai_scaled = source_bore_area_scaled(od_m, wall_m)
            .map_err(|_| ExactPressureError::InvalidGeometry)?;
        let ro = Scaled::from_f64(od_m).mul(Scaled::from_f64(0.5));
        let ri = ro.sub(Scaled::from_f64(wall_m));
        let i = as_scaled
            .mul(ro.mul(ro).add(ri.mul(ri)))
            .mul(Scaled::from_f64(0.25));
        let j = i.mul(Scaled::from_f64(2.0));
        let z = i
            .div(ro)
            .ok_or(ExactPressureError::NonRepresentableGeometry)?;
        Ok(Self {
            od_m,
            wall_m,
            kernel: ExactAnnulus {
                ri_m,
                ro_m,
                ai_m2: positive(ai_scaled)?,
                as_m2: positive(as_scaled)?,
                ai_scaled,
                as_scaled,
            },
            i_m4: positive(i)?,
            j_m4: positive(j)?,
            z_m3: positive(z)?,
        })
    }
    pub(crate) fn outside_diameter_m(self) -> f64 {
        self.od_m
    }
    pub(crate) fn effective_wall_thickness_m(self) -> f64 {
        self.wall_m
    }
    pub(crate) fn inner_radius_m(self) -> f64 {
        self.kernel.inner_radius_m()
    }
    pub(crate) fn outer_radius_m(self) -> f64 {
        self.kernel.outer_radius_m()
    }
    pub(crate) fn internal_area_m2(self) -> f64 {
        self.kernel.internal_area_m2()
    }
    pub(crate) fn wall_area_m2(self) -> f64 {
        self.kernel.wall_area_m2()
    }
    pub(crate) fn second_moment_m4(self) -> f64 {
        self.i_m4
    }
    pub(crate) fn polar_moment_m4(self) -> f64 {
        self.j_m4
    }
    pub(crate) fn section_modulus_m3(self) -> f64 {
        self.z_m3
    }
    pub(crate) fn cap_pair(
        self,
        p: InternalDifferentialPressure,
    ) -> Result<[f64; 2], ExactPressureError> {
        cap_pair(self.kernel, p)
    }
    pub(crate) fn eigenload_pair(
        self,
        m: IsotropicENu,
        p: InternalDifferentialPressure,
        thermal: f64,
    ) -> Result<[f64; 2], ExactPressureError> {
        eigenload_pair(self.kernel, m, p, thermal)
    }
    /// Exact two-difference identity of source OD/2-wall, without changing source inputs.
    pub(crate) fn source_bore_key(self) -> (u64, u64) {
        let a = self.od_m * 0.5;
        let b = self.wall_m;
        let x = a - b;
        let b_virtual = a - x;
        let a_virtual = x + b_virtual;
        let y = (a - a_virtual) + (b_virtual - b);
        (x.to_bits(), if y == 0.0 { 0 } else { y.to_bits() })
    }
    pub(crate) fn pressure_group_value(
        self,
        p: InternalDifferentialPressure,
        coefficient: f64,
        direction_magnitude: f64,
    ) -> Result<f64, ExactPressureError> {
        if !coefficient.is_finite() || !direction_magnitude.is_finite() {
            return Err(ExactPressureError::NonRepresentableLoad);
        }
        let value = super::fluid_force_scaled(self.kernel, p)
            .mul(Scaled::from_f64(coefficient))
            .mul(Scaled::from_f64(direction_magnitude));
        let output = scaled_output(value, ExactPressureError::NonRepresentableLoad)?;
        if output == 0.0 && value.mantissa != 0.0 {
            Err(ExactPressureError::NonRepresentableLoad)
        } else {
            Ok(output)
        }
    }
    pub(crate) fn recover_wall_effective_membrane(
        self,
        mechanical_force: f64,
        m: IsotropicENu,
        p: InternalDifferentialPressure,
    ) -> Result<(f64, f64, f64), ExactPressureError> {
        if !mechanical_force.is_finite() {
            return Err(ExactPressureError::NonRepresentableLoad);
        }
        let nm = Scaled::from_f64(mechanical_force);
        let fluid = super::fluid_force_scaled(self.kernel, p);
        let two_nu = 2.0 * m.poisson_ratio();
        let wall = fluid.fused_mul_add(Scaled::from_f64(two_nu), nm);
        // Two-difference retains a possible low term in (2nu-1); near 0.5 the high term itself is exact.
        let high = two_nu - 1.0;
        let bv = two_nu - high;
        let av = high + bv;
        let low = (two_nu - av) + (bv - 1.0);
        let effective = fluid.fused_mul_add(
            Scaled::from_f64(low),
            fluid.fused_mul_add(Scaled::from_f64(high), nm),
        );
        let membrane = wall
            .div(self.kernel.as_scaled)
            .ok_or(ExactPressureError::NonRepresentableStress)?;
        Ok((
            scaled_output(wall, ExactPressureError::NonRepresentableLoad)?,
            scaled_output(effective, ExactPressureError::NonRepresentableLoad)?,
            scaled_output(membrane, ExactPressureError::NonRepresentableStress)?,
        ))
    }
    pub(crate) fn surface_stresses(
        self,
        p: InternalDifferentialPressure,
    ) -> Result<[RadialHoopStressPa; 2], ExactPressureError> {
        let trace = pressure_trace_scaled(self.kernel, p);
        let outer = scaled_output(trace, ExactPressureError::NonRepresentableStress)?;
        let inner = scaled_output(
            trace.add(Scaled::from_f64(p.pressure_pa())),
            ExactPressureError::NonRepresentableStress,
        )?;
        Ok([
            RadialHoopStressPa {
                radial_pa: -p.pressure_pa(),
                hoop_pa: inner,
            },
            RadialHoopStressPa {
                radial_pa: 0.0,
                hoop_pa: outer,
            },
        ])
    }
    /// T4-U2 (H-2): a realized arc's pressure strain
    /// ε_p = (1 − 2ν)·pAi/(E·As), the closed bend's free axial strain under
    /// its wetted-wall load, own caps and the Poisson eigenstrain (the
    /// straight Lamé mean, RV1 S-5). As is the element's own area (the same
    /// binary64 value, RV1 N-3). Formed in `Scaled` arithmetic as the straight
    /// path forms its loads: (1 − 2ν) is split exactly into high + low, so
    /// counted from the source operands (OD, wall, p, E, ν) the chain has 15
    /// relative roundings — Ai 5 (r_i twice, PI, two products), p·Ai 1, the
    /// fused (1 − 2ν) product 2, As 4, E·As 1, the quotient 1 and the output
    /// 1 — which [`ARC_PRESSURE_STRAIN_ROUNDINGS`] states for S11-G. A
    /// subnormal nonzero strain is refused (its error is not relative).
    pub(crate) fn arc_pressure_strain(
        self,
        m: IsotropicENu,
        p: InternalDifferentialPressure,
    ) -> Result<f64, ExactPressureError> {
        let fluid = super::fluid_force_scaled(self.kernel, p);
        let minus_two_nu = -2.0 * m.poisson_ratio();
        // TwoSum: high + low == 1 − 2ν exactly.
        let high = 1.0 + minus_two_nu;
        let virtual_b = high - 1.0;
        let low = (1.0 - (high - virtual_b)) + (minus_two_nu - virtual_b);
        let numerator =
            fluid.fused_mul_add(Scaled::from_f64(low), fluid.mul(Scaled::from_f64(high)));
        let stiffness = Scaled::from_f64(m.elastic_modulus_pa()).mul(self.kernel.as_scaled);
        let strain = numerator
            .div(stiffness)
            .ok_or(ExactPressureError::NonRepresentableLoad)?;
        let output = scaled_output(strain, ExactPressureError::NonRepresentableLoad)?;
        if output != 0.0 && output.abs() < f64::MIN_POSITIVE
            || output == 0.0 && strain.mantissa != 0.0
        {
            return Err(ExactPressureError::NonRepresentableLoad);
        }
        Ok(output)
    }
    /// T4-U2 (H-2): an arc section's wall force, effective force and axial
    /// membrane stress from its elastic axial force N_el:
    /// N_w = N_el + pAi, S = N_el (unchanged) and σ_m = N_w/As, the same
    /// `Scaled` path as the straight recovery. No Poisson term enters: on an
    /// arc it is inside N_el through ε_p.
    pub(crate) fn recover_arc_wall_effective_membrane(
        self,
        elastic_force: f64,
        p: InternalDifferentialPressure,
    ) -> Result<(f64, f64, f64), ExactPressureError> {
        if !elastic_force.is_finite() {
            return Err(ExactPressureError::NonRepresentableLoad);
        }
        let wall = super::fluid_force_scaled(self.kernel, p).add(Scaled::from_f64(elastic_force));
        let membrane = wall
            .div(self.kernel.as_scaled)
            .ok_or(ExactPressureError::NonRepresentableStress)?;
        Ok((
            scaled_output(wall, ExactPressureError::NonRepresentableLoad)?,
            elastic_force,
            scaled_output(membrane, ExactPressureError::NonRepresentableStress)?,
        ))
    }
}

/// S11-G: the relative roundings in [`SourceAnnulus::arc_pressure_strain`].
pub(crate) const ARC_PRESSURE_STRAIN_ROUNDINGS: u32 = 15;
