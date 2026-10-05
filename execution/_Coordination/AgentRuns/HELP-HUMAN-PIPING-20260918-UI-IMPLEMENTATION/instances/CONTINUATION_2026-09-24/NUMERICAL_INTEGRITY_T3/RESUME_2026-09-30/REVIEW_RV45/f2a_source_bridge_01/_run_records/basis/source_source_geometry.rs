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
}
