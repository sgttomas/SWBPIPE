//! Element stiffness re-formation from supplied operands (DEL-04-09 design
//! §3.4 check 1, seam (i)).
//!
//! A read-only entry to K-D5's p128 element formation: the same routines
//! `formation_check` uses for its intended stiffness pass (`frame_matrix`,
//! `curved_matrix`, `connector_matrix` and `tie_rod_matrix`), at
//! [`FORMATION_PRECISION`], as pure functions of the operands supplied here.
//! No D5C1 judgment, ledger, load or product type enters, and nothing is
//! cached or mutated.
//!
//! The section properties, moduli and flexibility factors are supplied as
//! wide operands ([`WideOperand`], an unevaluated binary64 pair), so a caller
//! can pass values it derived at higher precision than binary64 (the modal
//! checks re-derive A, I and J from OD and wall, and form the rigid-family
//! products s·E and s·G, in double-double). Each operand enters as
//! hi + lo rounded once to p = 128 bits, which is exact whenever the pair
//! spans at most 128 bits (a normalized double-double with no exponent gap
//! between its words spans at most 107). The geometry (node coordinates, the
//! y reference, the bend radius) is authored binary64 and is lifted exactly.
//! Connectors and tie rods are re-formed from the kernel's own
//! [`ObjectiveConnector`] and [`TieRod`], whose stiffness leaves are their
//! binary64 decode, as K-D5 reads them.
//!
//! Each entry of the 12×12 result is returned as the exact split of the p128
//! value into at most three binary64 terms (bits 127…75, 74…22 and 21…0),
//! in decreasing magnitude, zero-padded. The split is exact unless bits below
//! 2⁻¹⁰⁷⁴ were set, which [`ReformedElement::truncated_below_min_subnormal`]
//! reports.
use super::formation_check::{
    connector_matrix, curved_matrix_with, frame_matrix_with, tie_rod_matrix, CurvedFormation,
    CurvedLeaves, Element, FrameLeaves, FORMATION_PRECISION,
};
use super::retained::wide::{Wide2, WideArith, WideError};
use crate::connector::ObjectiveConnector;
use crate::tie_rod::TieRod;
use crate::ELEMENT_DOF;

/// A wide operand: the exact value hi + lo of two binary64 numbers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WideOperand {
    pub hi: f64,
    pub lo: f64,
}

impl WideOperand {
    pub fn from_f64(value: f64) -> Self {
        Self { hi: value, lo: 0.0 }
    }

    /// hi + lo at p, rounded once (exact when the pair spans at most p bits).
    fn lift(&self, a: &mut WideArith) -> Result<Wide2, WideError> {
        let hi = Wide2::from_f64(self.hi)?;
        if self.lo == 0.0 {
            return Ok(hi);
        }
        a.add(&hi, &Wide2::from_f64(self.lo)?)
    }
}

/// A straight frame member: its chord, y reference and section leaves.
/// `shear_area` is A_s of a Timoshenko leaf, `None` for Euler–Bernoulli.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StraightOperands {
    pub coordinates_i: [f64; 3],
    pub coordinates_j: [f64; 3],
    pub y_reference: [f64; 3],
    pub elastic_modulus: WideOperand,
    pub shear_modulus: WideOperand,
    pub area: WideOperand,
    pub second_moment_y: WideOperand,
    pub second_moment_z: WideOperand,
    pub torsion_constant: WideOperand,
    pub shear_area: Option<WideOperand>,
}

/// A realized curved bend (DEC-070 macro-element): its nodes, the user
/// radius R and the plane reference, and its section leaves and flexibility
/// factors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BendOperands {
    pub coordinates_i: [f64; 3],
    pub coordinates_j: [f64; 3],
    pub radius: f64,
    pub y_reference: [f64; 3],
    pub elastic_modulus: WideOperand,
    pub shear_modulus: WideOperand,
    pub area: WideOperand,
    pub second_moment: WideOperand,
    pub torsion_constant: WideOperand,
    pub in_plane_flexibility_factor: WideOperand,
    pub out_of_plane_flexibility_factor: WideOperand,
}

/// A re-formed 12×12 element matrix, node i's six DOFs then node j's, in
/// global axes.
#[derive(Debug, Clone, PartialEq)]
pub struct ReformedElement {
    entries: [[[f64; 3]; ELEMENT_DOF]; ELEMENT_DOF],
    truncated: bool,
}

impl ReformedElement {
    /// Entry (i, j) as at most three nonoverlapping binary64 terms in
    /// decreasing magnitude, zero-padded, whose exact sum is the p128 value.
    pub fn terms(&self, i: usize, j: usize) -> [f64; 3] {
        self.entries[i][j]
    }

    /// True when some entry had set bits below 2⁻¹⁰⁷⁴, dropped by the split
    /// (then that entry's terms miss less than 2⁻¹⁰⁷⁴ in magnitude).
    pub fn truncated_below_min_subnormal(&self) -> bool {
        self.truncated
    }

    fn from_element(k: &Element) -> Result<Self, WideError> {
        let mut entries = [[[0.0; 3]; ELEMENT_DOF]; ELEMENT_DOF];
        let mut truncated = false;
        for (row, out) in k.iter().zip(entries.iter_mut()) {
            for (value, slot) in row.iter().zip(out.iter_mut()) {
                let split = value.split_binary64()?;
                truncated |= split.truncated_below_min_subnormal();
                slot[..split.terms().len()].copy_from_slice(split.terms());
            }
        }
        Ok(Self { entries, truncated })
    }
}

struct LiftedFrame {
    elastic_modulus: Wide2,
    shear_modulus: Wide2,
    area: Wide2,
    torsion_constant: Wide2,
    second_moment_y: Wide2,
    second_moment_z: Wide2,
    shear_area: Option<Wide2>,
}
impl FrameLeaves for LiftedFrame {
    fn elastic_modulus(&self) -> Result<Wide2, WideError> {
        Ok(self.elastic_modulus)
    }
    fn shear_modulus(&self) -> Result<Wide2, WideError> {
        Ok(self.shear_modulus)
    }
    fn area(&self) -> Result<Wide2, WideError> {
        Ok(self.area)
    }
    fn torsion_constant(&self) -> Result<Wide2, WideError> {
        Ok(self.torsion_constant)
    }
    fn second_moment_y(&self) -> Result<Wide2, WideError> {
        Ok(self.second_moment_y)
    }
    fn second_moment_z(&self) -> Result<Wide2, WideError> {
        Ok(self.second_moment_z)
    }
    fn shear_area(&self) -> Option<Result<Wide2, WideError>> {
        self.shear_area.map(Ok)
    }
}

struct LiftedCurved {
    elastic_modulus: Wide2,
    shear_modulus: Wide2,
    area: Wide2,
    second_moment: Wide2,
    torsion_constant: Wide2,
    in_plane: Wide2,
    out_of_plane: Wide2,
}
impl CurvedLeaves for LiftedCurved {
    fn elastic_modulus(&self) -> Result<Wide2, WideError> {
        Ok(self.elastic_modulus)
    }
    fn shear_modulus(&self) -> Result<Wide2, WideError> {
        Ok(self.shear_modulus)
    }
    fn area(&self) -> Result<Wide2, WideError> {
        Ok(self.area)
    }
    fn second_moment(&self) -> Result<Wide2, WideError> {
        Ok(self.second_moment)
    }
    fn torsion_constant(&self) -> Result<Wide2, WideError> {
        Ok(self.torsion_constant)
    }
    fn in_plane_flexibility_factor(&self) -> Result<Wide2, WideError> {
        Ok(self.in_plane)
    }
    fn out_of_plane_flexibility_factor(&self) -> Result<Wide2, WideError> {
        Ok(self.out_of_plane)
    }
}

/// A straight member's TᵀKT at p (`frame_matrix`).
pub fn reform_straight(operands: &StraightOperands) -> Result<ReformedElement, WideError> {
    let mut a = WideArith::new(FORMATION_PRECISION)?;
    let leaves = LiftedFrame {
        elastic_modulus: operands.elastic_modulus.lift(&mut a)?,
        shear_modulus: operands.shear_modulus.lift(&mut a)?,
        area: operands.area.lift(&mut a)?,
        torsion_constant: operands.torsion_constant.lift(&mut a)?,
        second_moment_y: operands.second_moment_y.lift(&mut a)?,
        second_moment_z: operands.second_moment_z.lift(&mut a)?,
        shear_area: match operands.shear_area {
            Some(value) => Some(value.lift(&mut a)?),
            None => None,
        },
    };
    let k = frame_matrix_with(
        &mut a,
        operands.coordinates_i,
        operands.coordinates_j,
        operands.y_reference,
        &leaves,
    )?;
    ReformedElement::from_element(&k)
}

/// A realized bend's objective 12×12 at p (`curved_matrix`).
pub fn reform_bend(operands: &BendOperands) -> Result<ReformedElement, WideError> {
    let mut a = WideArith::new(FORMATION_PRECISION)?;
    let leaves = LiftedCurved {
        elastic_modulus: operands.elastic_modulus.lift(&mut a)?,
        shear_modulus: operands.shear_modulus.lift(&mut a)?,
        area: operands.area.lift(&mut a)?,
        second_moment: operands.second_moment.lift(&mut a)?,
        torsion_constant: operands.torsion_constant.lift(&mut a)?,
        in_plane: operands.in_plane_flexibility_factor.lift(&mut a)?,
        out_of_plane: operands.out_of_plane_flexibility_factor.lift(&mut a)?,
    };
    // The geometry carrier: only the nodes, R and y are read from it; the
    // section fields are never read (the leaves above supply them).
    let geometry = CurvedFormation {
        node_i: 0,
        node_j: 1,
        coordinates_i: operands.coordinates_i,
        coordinates_j: operands.coordinates_j,
        radius: operands.radius,
        y_reference: operands.y_reference,
        elastic_modulus: 0.0,
        shear_modulus: 0.0,
        area: 0.0,
        second_moment: 0.0,
        torsion_constant: 0.0,
        in_plane_flexibility_factor: 0.0,
        out_of_plane_flexibility_factor: 0.0,
    };
    let k = curved_matrix_with(&mut a, &geometry, &leaves)?;
    ReformedElement::from_element(&k)
}

/// An objective connector's BᵀKB at p (`connector_matrix`).
pub fn reform_connector(connector: &ObjectiveConnector) -> Result<ReformedElement, WideError> {
    let mut a = WideArith::new(FORMATION_PRECISION)?;
    ReformedElement::from_element(&connector_matrix(&mut a, connector)?)
}

/// A tie rod's Cᵀ k C at p (`tie_rod_matrix`).
pub fn reform_tie_rod(rod: &TieRod) -> Result<ReformedElement, WideError> {
    let mut a = WideArith::new(FORMATION_PRECISION)?;
    ReformedElement::from_element(&tie_rod_matrix(&mut a, rod)?)
}

#[cfg(test)]
#[path = "element_reformation_tests.rs"]
mod tests;
