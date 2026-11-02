//! Section re-derivation in double-double (design §3.4 check 1, "trusted
//! leaves"; §2.1 and §2.2 for the mass bases).
//!
//! From the authored OD, wall t, mill deduction c_m (an absolute length),
//! corrosion allowance c_a (EX) and densities:
//! - the metal wall w is t − c_m on PF (`source_od_effective_wall_areas/v1`)
//!   and the nominal t on EX (`source_od_nominal_wall_pressure_bore_areas/v1`,
//!   the D-5 basis); the bore wall is w on PF and the pressure-bore wall
//!   t_r = (t − c_m) − c_a on EX;
//! - the stiffness section on the metal annulus: A = π·w·(OD − w),
//!   I = π/16·(OD² + ID²)·(OD − w)·w with ID = OD − 2w, and J = 2I;
//! - m̄ = ρ_m·A + ρ_c·π/4·ID_bore² + ρ_ins·π·t_ins·(OD + t_ins);
//! - j̄ = ρ_m·J + ρ_ins·J_ins with J_ins = π/16·(D_o² + OD²)·(D_o + OD)·t_ins,
//!   D_o = OD + 2t_ins, and ī = j̄/2 (contents excluded, D-e).
//!
//! These are the annulus formulas with the difference of squares and of
//! fourth powers factored through the wall, so no subtraction cancels; π is
//! the double-double π, not fl(π).

use std::error::Error;
use std::fmt;

use crate::dd::{max_propagating_nan, Dd};

pub const PF_MASS_METHOD: &str = "source_od_effective_wall_areas/v1";
pub const EX_MASS_METHOD: &str = "source_od_nominal_wall_pressure_bore_areas/v1";

/// The route's mass and stiffness-section basis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassBasis {
    /// PF: metal and contents on the wall t − c_m.
    EffectiveWall,
    /// EX: metal on the nominal wall t, contents on the pressure bore.
    NominalWallPressureBore,
}

impl MassBasis {
    pub fn method(self) -> &'static str {
        match self {
            Self::EffectiveWall => PF_MASS_METHOD,
            Self::NominalWallPressureBore => EX_MASS_METHOD,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Insulation {
    pub thickness: f64,
    pub density: f64,
}

/// The authored section and densities, binary64, absolute lengths in m.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PipeSection {
    pub outside_diameter: f64,
    pub wall_thickness: f64,
    pub mill_tolerance: f64,
    /// Required on EX.
    pub corrosion_allowance: Option<f64>,
    pub material_density: f64,
    /// `None` is empty.
    pub contents_density: Option<f64>,
    pub insulation: Option<Insulation>,
}

/// The re-derived section quantities, each in double-double.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SectionQuantities {
    pub basis: MassBasis,
    pub inner_diameter_metal: Dd,
    pub inner_diameter_bore: Dd,
    pub metal_mass_per_length: Dd,
    pub contents_mass_per_length: Dd,
    pub insulation_mass_per_length: Dd,
    /// m̄, kg/m.
    pub mass_per_length: Dd,
    /// j̄, kg·m.
    pub polar_rotary_inertia_per_length: Dd,
    /// ī = j̄/2, kg·m.
    pub bending_rotary_inertia_per_length: Dd,
    pub area: Dd,
    pub second_moment: Dd,
    pub torsion_constant: Dd,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SectionError {
    NonFinite {
        field: &'static str,
    },
    /// OD, the metal wall and the material density must be positive; the
    /// other inputs nonnegative; a wall may not exceed the radius.
    OutOfDomain {
        field: &'static str,
    },
    CorrosionAllowanceRequired,
}

impl fmt::Display for SectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { field } => write!(f, "section input {field} is not finite"),
            Self::OutOfDomain { field } => write!(f, "section input {field} is out of domain"),
            Self::CorrosionAllowanceRequired => {
                write!(f, "an explicit corrosion allowance is required on EX")
            }
        }
    }
}

impl Error for SectionError {}

/// The binary64 section properties a formation used, for the
/// section-derivation difference.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FormedSection {
    pub area: f64,
    pub second_moment: f64,
    pub torsion_constant: f64,
}

/// |formed − re-derived| / |re-derived| for A, I and J.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SectionDerivationDifference {
    pub area: f64,
    pub second_moment: f64,
    pub torsion_constant: f64,
}

impl SectionDerivationDifference {
    /// The largest component; NaN if any is NaN.
    pub fn max(&self) -> f64 {
        max_propagating_nan(
            max_propagating_nan(self.area, self.second_moment),
            self.torsion_constant,
        )
    }

    /// The componentwise maximum, propagating NaN.
    pub fn combine(self, other: Self) -> Self {
        Self {
            area: max_propagating_nan(self.area, other.area),
            second_moment: max_propagating_nan(self.second_moment, other.second_moment),
            torsion_constant: max_propagating_nan(self.torsion_constant, other.torsion_constant),
        }
    }
}

fn relative(formed: f64, derived: Dd) -> f64 {
    Dd::from_f64(formed)
        .sub(derived)
        .abs()
        .div(derived.abs())
        .to_f64()
}

pub fn section_derivation_difference(
    derived: &SectionQuantities,
    formed: &FormedSection,
) -> SectionDerivationDifference {
    SectionDerivationDifference {
        area: relative(formed.area, derived.area),
        second_moment: relative(formed.second_moment, derived.second_moment),
        torsion_constant: relative(formed.torsion_constant, derived.torsion_constant),
    }
}

fn finite(value: f64, field: &'static str) -> Result<f64, SectionError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(SectionError::NonFinite { field })
    }
}

fn nonnegative(value: f64, field: &'static str) -> Result<f64, SectionError> {
    if finite(value, field)? >= 0.0 {
        Ok(value)
    } else {
        Err(SectionError::OutOfDomain { field })
    }
}

fn positive(value: f64, field: &'static str) -> Result<f64, SectionError> {
    if finite(value, field)? > 0.0 {
        Ok(value)
    } else {
        Err(SectionError::OutOfDomain { field })
    }
}

/// A wall w with 0 ≤ w (0 < w when `positive`) and 2w ≤ OD.
fn wall(od: Dd, w: Dd, strict: bool, field: &'static str) -> Result<Dd, SectionError> {
    let inside = od.sub(w.mul_pow2(1));
    if w.hi < 0.0 || (strict && w.is_zero()) || inside.hi < 0.0 {
        return Err(SectionError::OutOfDomain { field });
    }
    Ok(w)
}

/// π·w·(OD − w), the annulus OD .. OD − 2w.
fn annulus_area(od: Dd, w: Dd) -> Dd {
    Dd::pi().mul(w).mul(od.sub(w))
}

/// π/32·(OD⁴ − ID⁴) = π/8·(OD² + ID²)·(OD − w)·w, the polar moment.
fn annulus_polar(od: Dd, w: Dd) -> Dd {
    let id = od.sub(w.mul_pow2(1));
    let squares = od.mul(od).add(id.mul(id));
    Dd::pi().mul(squares).mul(od.sub(w)).mul(w).mul_pow2(-3)
}

pub fn derive_section(
    section: &PipeSection,
    basis: MassBasis,
) -> Result<SectionQuantities, SectionError> {
    let od = Dd::from_f64(positive(section.outside_diameter, "outside_diameter")?);
    let t = Dd::from_f64(positive(section.wall_thickness, "wall_thickness")?);
    let mill = Dd::from_f64(nonnegative(section.mill_tolerance, "mill_tolerance")?);
    let rho_m = positive(section.material_density, "material_density")?;
    let reduced = t.sub(mill);
    let (metal, bore) = match basis {
        MassBasis::EffectiveWall => (reduced, reduced),
        MassBasis::NominalWallPressureBore => {
            let allowance = section
                .corrosion_allowance
                .ok_or(SectionError::CorrosionAllowanceRequired)?;
            let allowance = Dd::from_f64(nonnegative(allowance, "corrosion_allowance")?);
            (t, reduced.sub(allowance))
        }
    };
    let metal = wall(od, metal, true, "metal_wall")?;
    let bore = wall(od, bore, false, "bore_wall")?;
    let area = annulus_area(od, metal);
    let torsion_constant = annulus_polar(od, metal);
    let second_moment = torsion_constant.mul_pow2(-1);
    let inner_diameter_metal = od.sub(metal.mul_pow2(1));
    let inner_diameter_bore = od.sub(bore.mul_pow2(1));
    let metal_mass = area.mul_f64(rho_m);
    let contents_mass = match section.contents_density {
        None => Dd::ZERO,
        Some(rho) => {
            let rho = nonnegative(rho, "contents_density")?;
            Dd::pi()
                .mul(inner_diameter_bore)
                .mul(inner_diameter_bore)
                .mul_pow2(-2)
                .mul_f64(rho)
        }
    };
    let (insulation_mass, insulation_polar) = match section.insulation {
        None => (Dd::ZERO, Dd::ZERO),
        Some(Insulation { thickness, density }) => {
            let ti = Dd::from_f64(nonnegative(thickness, "insulation_thickness")?);
            let rho = nonnegative(density, "insulation_density")?;
            let outer = od.add(ti.mul_pow2(1));
            let a = Dd::pi().mul(ti).mul(od.add(ti));
            let squares = outer.mul(outer).add(od.mul(od));
            let j = Dd::pi()
                .mul(squares)
                .mul(outer.add(od))
                .mul(ti)
                .mul_pow2(-4);
            (a.mul_f64(rho), j.mul_f64(rho))
        }
    };
    let polar = torsion_constant.mul_f64(rho_m).add(insulation_polar);
    Ok(SectionQuantities {
        basis,
        inner_diameter_metal,
        inner_diameter_bore,
        metal_mass_per_length: metal_mass,
        contents_mass_per_length: contents_mass,
        insulation_mass_per_length: insulation_mass,
        mass_per_length: metal_mass.add(contents_mass).add(insulation_mass),
        polar_rotary_inertia_per_length: polar,
        bending_rotary_inertia_per_length: polar.mul_pow2(-1),
        area,
        second_moment,
        torsion_constant,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn section() -> PipeSection {
        PipeSection {
            outside_diameter: 0.25,
            wall_thickness: 0.0625,
            mill_tolerance: 0.0,
            corrosion_allowance: Some(0.0),
            material_density: 8.0,
            contents_density: None,
            insulation: None,
        }
    }

    /// OD = 1/4, w = 1/16: A = π·(1/16)(3/16) = 3π/256 and
    /// J = π/32·(1/256 − 1/4096) = 15π/131072, exactly in units of π.
    #[test]
    fn exact_annulus() {
        let q = derive_section(&section(), MassBasis::EffectiveWall).unwrap();
        let pi = Dd::pi();
        assert_eq!(q.area, pi.mul_f64(3.0 / 256.0));
        assert_eq!(q.torsion_constant, pi.mul_f64(15.0 / 131072.0));
        assert_eq!(q.second_moment, pi.mul_f64(15.0 / 262144.0));
        assert_eq!(q.mass_per_length, pi.mul_f64(3.0 / 32.0));
        assert_eq!(
            q.polar_rotary_inertia_per_length,
            pi.mul_f64(15.0 / 16384.0)
        );
    }

    #[test]
    fn refusals() {
        let mut s = section();
        s.corrosion_allowance = None;
        assert_eq!(
            derive_section(&s, MassBasis::NominalWallPressureBore),
            Err(SectionError::CorrosionAllowanceRequired)
        );
        assert!(derive_section(&s, MassBasis::EffectiveWall).is_ok());
        s.mill_tolerance = 0.0625;
        assert_eq!(
            derive_section(&s, MassBasis::EffectiveWall),
            Err(SectionError::OutOfDomain {
                field: "metal_wall"
            })
        );
        let mut s = section();
        s.wall_thickness = 0.25;
        assert_eq!(
            derive_section(&s, MassBasis::EffectiveWall),
            Err(SectionError::OutOfDomain {
                field: "metal_wall"
            })
        );
        let mut s = section();
        s.outside_diameter = f64::NAN;
        assert_eq!(
            derive_section(&s, MassBasis::EffectiveWall),
            Err(SectionError::NonFinite {
                field: "outside_diameter"
            })
        );
    }
}
