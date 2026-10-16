//! Active source-OD/wall annulus over the exact-annulus kernel.
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

/// D-5 (DEL-05-03 Design `section-bases-and-timoshenko.md` §1): a section
/// basis on which an exact-contract stress row is published.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SectionBasis {
    /// The nominal wall t.
    Nominal,
    /// The reduced wall t_r = (t − c_m) − c_a.
    Reduced,
}

impl SectionBasis {
    /// Both bases in publication order: each nominal row is immediately
    /// followed by its reduced twin (§2.5).
    pub(crate) const BOTH: [SectionBasis; 2] = [SectionBasis::Nominal, SectionBasis::Reduced];
    /// The basis name in `metadata.basis`, evidence `section_basis` and
    /// underscore-style ids.
    pub(crate) const fn token(self) -> &'static str {
        match self {
            SectionBasis::Nominal => "nominal_section",
            SectionBasis::Reduced => "reduced_section",
        }
    }
    /// The final segment of a hyphen-style id.
    pub(crate) const fn hyphen(self) -> &'static str {
        match self {
            SectionBasis::Nominal => "nominal-section",
            SectionBasis::Reduced => "reduced-section",
        }
    }
    /// The `; section_basis=…` suffix of a stress row's recovery token.
    pub(crate) const fn basis_suffix(self) -> &'static str {
        match self {
            SectionBasis::Nominal => "; section_basis=nominal_section",
            SectionBasis::Reduced => "; section_basis=reduced_section",
        }
    }
}

/// The wall force N_w and effective force S of one section cut, formed once
/// on the pressure bore; `wall` keeps N_w before its binary64 rounding so
/// each basis divides the same exact value by its own area.
#[derive(Debug, Clone, Copy)]
pub(crate) struct WallForces {
    wall: Scaled,
    pub(crate) wall_n: f64,
    pub(crate) effective_n: f64,
}

/// D-5 (DEL-05-03 Design `section-bases-and-timoshenko.md` §1): the three
/// geometries of one exact-contract pipe section, derived once per pipe.
///
/// - The **nominal section** (wall t) carries stiffness: A, I, J and Z of the
///   frame and of a realized arc, the thermal/fit eigenloads E·A·ε, and As in
///   ε_p = (1 − 2ν)pAi/(E·As).
/// - The **reduced section** (wall t_r = (t − c_m) − c_a, in exactly this
///   binary64 order) is the second stress basis.
/// - The **pressure bore** r_i,r = D/2 − t_r is the reduced section's own
///   bore: caps, Poisson pairs, joint faces, every `PressureOperand` and
///   `source_bore_key` use Ai = π r_i,r².
///
/// With c_a = 0 the reduced section is bit-identical to the former single
/// t − c_m section; with c_m = c_a = 0 both sections are.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ExactSectionBases {
    nominal: SourceAnnulus,
    reduced: SourceAnnulus,
    mill_tolerance_m: f64,
    corrosion_allowance_m: f64,
    /// T4-U8 (Design §3.1): (kappa_E, A_s) of a timoshenko_annulus_energy_v1
    /// straight member, formed from the nominal annulus; `None` for an
    /// Euler-Bernoulli member and every realized arc.
    timoshenko: Option<(f64, f64)>,
}

impl ExactSectionBases {
    /// The reduced wall t_r = (t − c_m) − c_a. The order is fixed: the
    /// algebraically equal t − (c_m + c_a) rounds differently.
    pub(crate) fn reduced_wall(nominal_wall_m: f64, mill_tolerance_m: f64, corrosion_allowance_m: f64) -> f64 {
        (nominal_wall_m - mill_tolerance_m) - corrosion_allowance_m
    }
    /// The three geometries from the authored OD, nominal wall t, mill
    /// tolerance c_m (absent = 0) and corrosion allowance c_a, each section's
    /// A, I, J and Z from `SourceAnnulus::from_od_wall` with its own wall.
    pub(crate) fn derive(
        od_m: f64,
        nominal_wall_m: f64,
        mill_tolerance_m: f64,
        corrosion_allowance_m: f64,
    ) -> Result<Self, ExactPressureError> {
        let reduced_wall = Self::reduced_wall(nominal_wall_m, mill_tolerance_m, corrosion_allowance_m);
        Ok(Self {
            nominal: SourceAnnulus::from_od_wall(od_m, nominal_wall_m)?,
            reduced: SourceAnnulus::from_od_wall(od_m, reduced_wall)?,
            mill_tolerance_m,
            corrosion_allowance_m,
            timoshenko: None,
        })
    }
    /// The same section bases of a timoshenko_annulus_energy_v1 straight
    /// member, with its energy-matched factor kappa_E and shear area A_s
    /// (FK `shear::annulus_energy_shear_factor` on the nominal annulus and
    /// `shear::shear_area` on its area), for the stiffness leaf and the
    /// evidence's `beam_theory`.
    pub(crate) fn with_timoshenko(self, shear_factor: f64, shear_area_m2: f64) -> Self {
        Self { timoshenko: Some((shear_factor, shear_area_m2)), ..self }
    }
    /// (kappa_E, A_s) of a Timoshenko straight member; `None` otherwise.
    pub(crate) fn timoshenko(self) -> Option<(f64, f64)> {
        self.timoshenko
    }
    pub(crate) fn nominal(self) -> SourceAnnulus {
        self.nominal
    }
    pub(crate) fn reduced(self) -> SourceAnnulus {
        self.reduced
    }
    pub(crate) fn basis(self, basis: SectionBasis) -> SourceAnnulus {
        match basis {
            SectionBasis::Nominal => self.nominal,
            SectionBasis::Reduced => self.reduced,
        }
    }
    pub(crate) fn outside_diameter_m(self) -> f64 {
        self.nominal.outside_diameter_m()
    }
    pub(crate) fn nominal_wall_m(self) -> f64 {
        self.nominal.effective_wall_thickness_m()
    }
    pub(crate) fn mill_tolerance_m(self) -> f64 {
        self.mill_tolerance_m
    }
    pub(crate) fn corrosion_allowance_m(self) -> f64 {
        self.corrosion_allowance_m
    }
    /// The wall whose bore is the pressure bore (t_r): the `wall_thickness`
    /// operand of every pressure-area formula (`PressureOperand`,
    /// `PressureFreeStrainCurved`, `certified_pressure_area_product`).
    pub(crate) fn pressure_wall_m(self) -> f64 {
        self.reduced.effective_wall_thickness_m()
    }
    /// r_i,r = D/2 − t_r.
    pub(crate) fn pressure_bore_radius_m(self) -> f64 {
        self.reduced.inner_radius_m()
    }
    /// Ai = π r_i,r².
    pub(crate) fn pressure_bore_area_m2(self) -> f64 {
        self.reduced.internal_area_m2()
    }
    /// The exact stored pressure-bore Ai and the basis's As, before any
    /// binary64 publication.
    pub(crate) fn area_dyadic_parts(self, basis: SectionBasis) -> ((f64, i32), (f64, i32)) {
        (self.reduced.kernel.ai_scaled.dyadic_parts(), self.basis(basis).kernel.as_scaled.dyadic_parts())
    }
    /// The terminal/joint-face cap pair ±pAi on the pressure bore.
    pub(crate) fn cap_pair(self, p: InternalDifferentialPressure) -> Result<[f64; 2], ExactPressureError> {
        self.reduced.cap_pair(p)
    }
    /// The straight Poisson eigen pair ±2νpAi on the pressure bore. Formed at
    /// zero thermal strain, where the wall area does not enter (the thermal
    /// term E·As·ε of `eigenload_pair` is exactly zero).
    pub(crate) fn poisson_eigenload_pair(
        self,
        m: IsotropicENu,
        p: InternalDifferentialPressure,
    ) -> Result<[f64; 2], ExactPressureError> {
        self.reduced.eigenload_pair(m, p, 0.0)
    }
    /// Each basis's own closed-annulus surface stresses (§2.1): the Lamé
    /// solution of that section with internal pressure p at its bore r_i,b.
    pub(crate) fn surface_stresses(
        self,
        basis: SectionBasis,
        p: InternalDifferentialPressure,
    ) -> Result<[RadialHoopStressPa; 2], ExactPressureError> {
        self.basis(basis).surface_stresses(p)
    }
    /// The pressure bore's two-difference identity (`source_bore_key`).
    pub(crate) fn source_bore_key(self) -> (u64, u64) {
        self.reduced.source_bore_key()
    }
    /// One pressure source group's formed value on the pressure bore.
    pub(crate) fn pressure_group_value(
        self,
        p: InternalDifferentialPressure,
        coefficient: f64,
        direction_magnitude: f64,
    ) -> Result<f64, ExactPressureError> {
        self.reduced.pressure_group_value(p, coefficient, direction_magnitude)
    }
    /// A straight section's wall force N_w = N_m + 2νpAi and effective force
    /// S = N_m + (2ν − 1)pAi, formed once with Ai on the pressure bore (§1,
    /// the split of the former single-annulus recovery). The membrane stress
    /// of each basis is [`Self::membrane`].
    pub(crate) fn recover_wall_effective(
        self,
        mechanical_force: f64,
        m: IsotropicENu,
        p: InternalDifferentialPressure,
    ) -> Result<WallForces, ExactPressureError> {
        if !mechanical_force.is_finite() {
            return Err(ExactPressureError::NonRepresentableLoad);
        }
        let nm = Scaled::from_f64(mechanical_force);
        let fluid = super::fluid_force_scaled(self.reduced.kernel, p);
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
        Ok(WallForces {
            wall,
            wall_n: scaled_output(wall, ExactPressureError::NonRepresentableLoad)?,
            effective_n: scaled_output(effective, ExactPressureError::NonRepresentableLoad)?,
        })
    }
    /// T4-U2 (H-2) on a realized arc: N_w = N_el + pAi with Ai on the
    /// pressure bore and S = N_el (unchanged). No Poisson term enters: on an
    /// arc it is inside N_el through ε_p.
    pub(crate) fn recover_arc_wall_effective(
        self,
        elastic_force: f64,
        p: InternalDifferentialPressure,
    ) -> Result<WallForces, ExactPressureError> {
        if !elastic_force.is_finite() {
            return Err(ExactPressureError::NonRepresentableLoad);
        }
        let wall = super::fluid_force_scaled(self.reduced.kernel, p).add(Scaled::from_f64(elastic_force));
        Ok(WallForces {
            wall,
            wall_n: scaled_output(wall, ExactPressureError::NonRepresentableLoad)?,
            effective_n: elastic_force,
        })
    }
    /// σ_m = N_w/A_b: the one wall force divided by that basis's exact
    /// stored area (the same `Scaled` path as the former single section).
    pub(crate) fn membrane(self, basis: SectionBasis, forces: WallForces) -> Result<f64, ExactPressureError> {
        let membrane = forces
            .wall
            .div(self.basis(basis).kernel.as_scaled)
            .ok_or(ExactPressureError::NonRepresentableStress)?;
        scaled_output(membrane, ExactPressureError::NonRepresentableStress)
    }
    /// T4-U2 (H-2): a realized arc's pressure strain
    /// ε_p = (1 − 2ν)·pAi/(E·As), the closed bend's free axial strain under
    /// its wetted-wall load, own caps and the Poisson eigenstrain (the
    /// straight Lamé mean, RV1 S-5). Under D-5 Ai is the pressure bore's and
    /// As is the arc element's own (nominal, stiffness) area, the same
    /// binary64 value as the element's (RV1 N-3). Formed in `Scaled`
    /// arithmetic as the straight path forms its loads, with (1 − 2ν) split
    /// exactly into high + low.
    ///
    /// S11-G's rounding count for two walls. The source operands are OD, the
    /// pressure wall t_r, the nominal wall t, p, E and ν; t_r is the
    /// contract's binary64 leaf (t − c_m) − c_a, exactly as the former single
    /// wall was the binary64 leaf t − c_m, so its own two subtractions are part
    /// of the operand's definition and are not counted. Each wall enters only
    /// its own area: Ai = PI·r_i·r_i with r_i = OD·0.5 − t_r costs 5 (the r_i
    /// rounding entering twice, PI, two products) and As = PI·t·(OD − t)
    /// costs 4 (PI, the difference, two products). The rest of the chain is
    /// unchanged: p·Ai 1, the fused (1 − 2ν) product 2, E·As 1, the quotient
    /// 1 and the output 1. Splitting one wall into two therefore leaves the
    /// count at 15 = 5 + 1 + 2 + 4 + 1 + 1 + 1, which
    /// [`ARC_PRESSURE_STRAIN_ROUNDINGS`] states. With c_m = c_a = 0 both walls
    /// are the same leaf and the chain is the former one bit for bit. A
    /// subnormal nonzero strain is refused (its error is not relative).
    pub(crate) fn arc_pressure_strain(
        self,
        m: IsotropicENu,
        p: InternalDifferentialPressure,
    ) -> Result<f64, ExactPressureError> {
        let fluid = super::fluid_force_scaled(self.reduced.kernel, p);
        let minus_two_nu = -2.0 * m.poisson_ratio();
        // TwoSum: high + low == 1 − 2ν exactly.
        let high = 1.0 + minus_two_nu;
        let virtual_b = high - 1.0;
        let low = (1.0 - (high - virtual_b)) + (minus_two_nu - virtual_b);
        let numerator =
            fluid.fused_mul_add(Scaled::from_f64(low), fluid.mul(Scaled::from_f64(high)));
        let stiffness = Scaled::from_f64(m.elastic_modulus_pa()).mul(self.nominal.kernel.as_scaled);
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
}

/// S11-G: the relative roundings in [`ExactSectionBases::arc_pressure_strain`]
/// (5 + 1 + 2 + 4 + 1 + 1 + 1 for the pressure wall and the nominal wall; the
/// argument is on that function).
pub(crate) const ARC_PRESSURE_STRAIN_ROUNDINGS: u32 = 15;

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn bits(s: SourceAnnulus) -> [u64; 9] {
        [s.outside_diameter_m(), s.effective_wall_thickness_m(), s.inner_radius_m(), s.outer_radius_m(), s.internal_area_m2(),
            s.wall_area_m2(), s.second_moment_m4(), s.polar_moment_m4(), s.section_modulus_m3()]
        .map(f64::to_bits)
    }
    fn close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() <= 1e-14 * expected.abs(), "{actual:e} vs {expected:e}");
    }

    /// D-5 (DEL-05-03 Design §1): the three geometries of one section. The
    /// I8 MILLTOL section (D = 0.2 m, t = 0.01 m, c_m = 0.00125 m, c_a =
    /// 0.002 m) is one where the binary64 order matters: (t − c_m) − c_a and
    /// t − (c_m + c_a) differ in their last bit, and the reduced wall is the
    /// former.
    #[test]
    fn d5_three_geometries_in_binary64_order() {
        let (od, t, cm, ca): (f64, f64, f64, f64) = (0.2, 0.01, 0.00125, 0.002);
        let reduced_wall = (t - cm) - ca;
        assert_ne!(reduced_wall.to_bits(), (t - (cm + ca)).to_bits(), "precondition: the order is observable");
        let g = ExactSectionBases::derive(od, t, cm, ca).unwrap();
        // The nominal section: wall t, the stiffness section.
        assert_eq!(bits(g.nominal()), bits(SourceAnnulus::from_od_wall(od, t).unwrap()));
        assert_eq!(g.nominal_wall_m().to_bits(), t.to_bits());
        assert_eq!(g.nominal().inner_radius_m().to_bits(), (od * 0.5 - t).to_bits());
        // The reduced section: wall (t − c_m) − c_a, every property its own.
        assert_eq!(bits(g.reduced()), bits(SourceAnnulus::from_od_wall(od, reduced_wall).unwrap()));
        assert_eq!(g.pressure_wall_m().to_bits(), reduced_wall.to_bits());
        assert_ne!(bits(g.reduced()), bits(SourceAnnulus::from_od_wall(od, t - (cm + ca)).unwrap()));
        // The pressure bore: r_i,r = D/2 − t_r and Ai = π r_i,r², the reduced section's own bore.
        assert_eq!(g.pressure_bore_radius_m().to_bits(), (od * 0.5 - reduced_wall).to_bits());
        assert_eq!(g.pressure_bore_area_m2().to_bits(), g.reduced().internal_area_m2().to_bits());
        close(g.pressure_bore_area_m2(), PI * (od * 0.5 - reduced_wall).powi(2));
        assert_eq!(g.source_bore_key(), g.reduced().source_bore_key());
        assert_ne!(g.source_bore_key(), g.nominal().source_bore_key());
        assert_eq!((g.mill_tolerance_m(), g.corrosion_allowance_m(), g.outside_diameter_m()), (cm, ca, od));
        for basis in SectionBasis::BOTH {
            assert_eq!(bits(g.basis(basis)), bits(match basis { SectionBasis::Nominal => g.nominal(), SectionBasis::Reduced => g.reduced() }));
        }
        let (m, p) = (IsotropicENu::new(2.0e11, 0.3).unwrap(), InternalDifferentialPressure::new(1.0e7).unwrap());
        // Caps and Poisson pairs on the pressure bore.
        let cap = g.cap_pair(p).unwrap();
        assert_eq!(cap.map(f64::to_bits), g.reduced().cap_pair(p).unwrap().map(f64::to_bits));
        assert_ne!(cap.map(f64::to_bits), g.nominal().cap_pair(p).unwrap().map(f64::to_bits));
        assert_eq!(g.poisson_eigenload_pair(m, p).unwrap().map(f64::to_bits),
            g.reduced().eigenload_pair(m, p, 0.0).unwrap().map(f64::to_bits));
        // One wall force on the pressure bore, divided by each basis's own area.
        let forces = g.recover_wall_effective(1.0e5, m, p).unwrap();
        close(forces.wall_n, 1.0e5 + 0.6 * 1.0e7 * g.pressure_bore_area_m2());
        close(forces.effective_n, 1.0e5 - 0.4 * 1.0e7 * g.pressure_bore_area_m2());
        let (nominal, reduced) = (g.membrane(SectionBasis::Nominal, forces).unwrap(), g.membrane(SectionBasis::Reduced, forces).unwrap());
        close(nominal, forces.wall_n / g.nominal().wall_area_m2());
        close(reduced, forces.wall_n / g.reduced().wall_area_m2());
        assert!(reduced > nominal);
        // Each basis's Lamé stresses are its own annulus's.
        for basis in SectionBasis::BOTH {
            let [inner, outer] = g.surface_stresses(basis, p).unwrap();
            let [i, o] = g.basis(basis).surface_stresses(p).unwrap();
            assert_eq!((inner, outer), (i, o));
        }
        assert_ne!(g.surface_stresses(SectionBasis::Nominal, p).unwrap(), g.surface_stresses(SectionBasis::Reduced, p).unwrap());
        // The arc pressure strain: Ai on the pressure bore over E·As of the nominal (stiffness) wall.
        close(g.arc_pressure_strain(m, p).unwrap(), 0.4 * 1.0e7 * g.pressure_bore_area_m2() / (2.0e11 * g.nominal().wall_area_m2()));
        assert_eq!(ARC_PRESSURE_STRAIN_ROUNDINGS, 15);
    }

    /// With c_a = 0 the reduced section is the former single t − c_m section
    /// bit for bit (so is every pressure quantity); with c_m = c_a = 0 the two
    /// sections and the former section coincide.
    #[test]
    fn d5_zero_allowance_reproduces_the_former_section() {
        let (od, t, cm): (f64, f64, f64) = (0.2, 0.01, 0.00125);
        let former = SourceAnnulus::from_od_wall(od, t - cm).unwrap();
        let g = ExactSectionBases::derive(od, t, cm, 0.0).unwrap();
        assert_eq!(bits(g.reduced()), bits(former));
        assert_eq!(g.source_bore_key(), former.source_bore_key());
        let (m, p) = (IsotropicENu::new(2.0e11, 0.3).unwrap(), InternalDifferentialPressure::new(-3.0e6).unwrap());
        assert_eq!(g.cap_pair(p).unwrap().map(f64::to_bits), former.cap_pair(p).unwrap().map(f64::to_bits));
        assert_eq!(g.pressure_group_value(p, 0.5, 1.0).unwrap().to_bits(), former.pressure_group_value(p, 0.5, 1.0).unwrap().to_bits());
        assert_eq!(bits(g.nominal()), bits(SourceAnnulus::from_od_wall(od, t).unwrap()));
        let both = ExactSectionBases::derive(od, t, 0.0, 0.0).unwrap();
        assert_eq!(bits(both.nominal()), bits(both.reduced()));
        assert_eq!(bits(both.nominal()), bits(SourceAnnulus::from_od_wall(od, t).unwrap()));
        let forces = both.recover_wall_effective(-2.5e4, m, p).unwrap();
        assert_eq!(both.membrane(SectionBasis::Nominal, forces).unwrap().to_bits(), both.membrane(SectionBasis::Reduced, forces).unwrap().to_bits());
    }

    /// The section-sum check stays: a reduced wall that is not positive is refused.
    #[test]
    fn d5_reduced_wall_must_remain_a_wall() {
        assert!(ExactSectionBases::derive(0.2, 0.01, 0.005, 0.005).is_err());
        assert!(ExactSectionBases::derive(0.2, 0.01, 0.0, 0.0101).is_err());
    }
}
