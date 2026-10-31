//! The element re-formation seam against K-D5's own re-formation: with the
//! binary64 leaves lifted as wide operands, every family gives K-D5's p128
//! matrices bit for bit; a wide operand reaches the formation unrounded.
//! Invented inputs only.
use super::super::formation_check::{
    connector_matrix, curved_matrix, frame_matrix, tie_rod_matrix, CurvedFormation,
};
use super::super::retained::wide::WideArith;
use super::*;
use crate::connector::{ConnectorAttachment, ScaledWorkMatrix};
use crate::tie_rod::TieRodInput;
use crate::{FrameElement, FrameNode, FrameSection};

/// K-D5's p128 element split as the seam returns it.
fn split(k: &Element) -> ReformedElement {
    ReformedElement::from_element(k).unwrap()
}

fn skew_section() -> FrameSection {
    let pi = std::f64::consts::PI;
    let (od, id) = (0.2191_f64, 0.2027_f64);
    let i4 = pi * ((od * od) * (od * od) - (id * id) * (id * id)) / 64.0;
    FrameSection::new(200e9, 80e9, pi * (od * od - id * id) / 4.0, i4, i4, 2.0 * i4).unwrap()
}

fn straight_operands(e: &FrameElement) -> StraightOperands {
    let s = &e.section;
    StraightOperands {
        coordinates_i: e.node_i.coordinates,
        coordinates_j: e.node_j.coordinates,
        y_reference: e.y_reference,
        elastic_modulus: WideOperand::from_f64(s.elastic_modulus),
        shear_modulus: WideOperand::from_f64(s.shear_modulus),
        area: WideOperand::from_f64(s.area),
        second_moment_y: WideOperand::from_f64(s.second_moment_y),
        second_moment_z: WideOperand::from_f64(s.second_moment_z),
        torsion_constant: WideOperand::from_f64(s.torsion_constant),
        shear_area: s.beam_theory.shear_area().map(WideOperand::from_f64),
    }
}

fn skew_frame(section: FrameSection) -> FrameElement {
    FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [1.0, 2.0, 2.0]).unwrap(),
        section,
        [1.0, 0.0, 0.0],
    )
    .unwrap()
}

fn skew_bend() -> CurvedFormation {
    CurvedFormation {
        node_i: 3,
        node_j: 4,
        coordinates_i: [1.0, 0.25, -0.5],
        coordinates_j: [0.125, 1.0, 0.0],
        radius: 0.6875,
        y_reference: [1.0, 1.0, 0.25],
        elastic_modulus: 2.03e11,
        shear_modulus: 7.8e10,
        area: 5.3e-3,
        second_moment: 2.9e-5,
        torsion_constant: 5.8e-5,
        in_plane_flexibility_factor: 3.7,
        out_of_plane_flexibility_factor: 2.9,
    }
}

fn bend_operands(e: &CurvedFormation) -> BendOperands {
    BendOperands {
        coordinates_i: e.coordinates_i,
        coordinates_j: e.coordinates_j,
        radius: e.radius,
        y_reference: e.y_reference,
        elastic_modulus: WideOperand::from_f64(e.elastic_modulus),
        shear_modulus: WideOperand::from_f64(e.shear_modulus),
        area: WideOperand::from_f64(e.area),
        second_moment: WideOperand::from_f64(e.second_moment),
        torsion_constant: WideOperand::from_f64(e.torsion_constant),
        in_plane_flexibility_factor: WideOperand::from_f64(e.in_plane_flexibility_factor),
        out_of_plane_flexibility_factor: WideOperand::from_f64(e.out_of_plane_flexibility_factor),
    }
}

#[test]
fn straight_matches_kd5_bit_for_bit() {
    for section in [
        skew_section(),
        FrameSection::timoshenko(200e9, 80e9, 5.3e-3, 2.9e-5, 2.9e-5, 5.8e-5, 2.7e-3).unwrap(),
    ] {
        let e = skew_frame(section);
        let mut a = WideArith::new(FORMATION_PRECISION).unwrap();
        let kd5 = split(&frame_matrix(&mut a, &e).unwrap());
        let seam = reform_straight(&straight_operands(&e)).unwrap();
        assert_eq!(seam, kd5);
        assert!(!seam.truncated_below_min_subnormal());
    }
}

#[test]
fn bend_matches_kd5_bit_for_bit() {
    let e = skew_bend();
    let mut a = WideArith::new(FORMATION_PRECISION).unwrap();
    let kd5 = split(&curved_matrix(&mut a, &e).unwrap());
    let seam = reform_bend(&bend_operands(&e)).unwrap();
    assert_eq!(seam, kd5);
}

#[test]
fn connector_and_tie_rod_match_kd5_bit_for_bit() {
    let mut upper = [0.0; 21];
    let diagonal: [f64; 6] = [5e6, 4e6, 3e6, 2e3, 1e3, 1.5e3];
    let mut at = 0;
    for r in 0..6 {
        for c in r..6 {
            if r == c {
                upper[at] = diagonal[r];
            } else if c == r + 1 {
                upper[at] = 0.125 * diagonal[r].min(diagonal[c]);
            }
            at += 1;
        }
    }
    let (t, u) = (1.0 / 3.0, 2.0 / 3.0);
    let connector = ObjectiveConnector::new(
        FrameNode::new(0, [0.25, -0.5, 1.0]).unwrap(),
        FrameNode::new(1, [0.5, 0.0, 1.5]).unwrap(),
        ConnectorAttachment::global([0.0; 3]),
        ConnectorAttachment::global([0.0; 3]),
        [[t, u, -u], [u, t, u], [u, -u, -t]],
        ScaledWorkMatrix {
            upper_triangle: upper,
            translation_scale: 1.0,
        },
        [0.0; 6],
    )
    .unwrap()
    .with_tie_rods(&[TieRodInput {
        offset_i: [-0.05, 0.13, -0.07],
        offset_j: [0.06, -0.11, 0.09],
        stiffness: 2.5e7,
        reference_extension: -1.25e-3,
    }])
    .unwrap();
    let mut a = WideArith::new(FORMATION_PRECISION).unwrap();
    assert_eq!(
        reform_connector(&connector).unwrap(),
        split(&connector_matrix(&mut a, &connector).unwrap())
    );
    let rod = &connector.tie_rods()[0];
    assert_eq!(
        reform_tie_rod(rod).unwrap(),
        split(&tie_rod_matrix(&mut a, rod).unwrap())
    );
}

/// A wide area A = 1 + 2⁻⁸⁰ (not a binary64 number) reaches the formation
/// unrounded: with E = L = 1 along x, the axial entry is exactly A, split
/// into the terms 1 and 2⁻⁸⁰, while the binary64 neighbour 1 gives 1 alone.
#[test]
fn a_wide_operand_enters_unrounded() {
    let tiny = 2.0_f64.powi(-80);
    let section = FrameSection::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let e = FrameElement::new(
        FrameNode::new(0, [0.0; 3]).unwrap(),
        FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap(),
        section,
        [0.0, 1.0, 0.0],
    )
    .unwrap();
    let mut operands = straight_operands(&e);
    let rounded = reform_straight(&operands).unwrap();
    assert_eq!(rounded.terms(0, 0), [1.0, 0.0, 0.0]);
    operands.area = WideOperand { hi: 1.0, lo: tiny };
    let wide = reform_straight(&operands).unwrap();
    assert_eq!(wide.terms(0, 0), [1.0, tiny, 0.0]);
    assert_eq!(wide.terms(0, 6), [-1.0, -tiny, 0.0]);
    // Only the axial entries depend on A.
    for r in 0..12 {
        for c in 0..12 {
            if ![0, 6].contains(&r) || ![0, 6].contains(&c) {
                assert_eq!(wide.terms(r, c), rounded.terms(r, c), "[{r}][{c}]");
            }
        }
    }
}

#[test]
fn refusals_are_typed_and_repeat_runs_are_identical() {
    let e = skew_frame(skew_section());
    let mut operands = straight_operands(&e);
    let first = reform_straight(&operands).unwrap();
    assert_eq!(reform_straight(&operands).unwrap(), first);
    operands.coordinates_j = operands.coordinates_i;
    assert_eq!(reform_straight(&operands), Err(WideError::DivisionByZero));
    let mut operands = straight_operands(&e);
    operands.area = WideOperand {
        hi: f64::NAN,
        lo: 0.0,
    };
    assert_eq!(reform_straight(&operands), Err(WideError::NonFinite));
    // A bend whose chord exceeds its diameter has no angle.
    let mut bend = bend_operands(&skew_bend());
    bend.radius = 0.25;
    assert_eq!(reform_bend(&bend), Err(WideError::AngleDomain));
}
