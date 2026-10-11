//! K1 x K2a (T3 I8 brief, "Interactions with merged and parallel slices":
//! checked formation feeds sparse assembly). Drafted by I8 against K2a's
//! branch (`IMPLEMENTATION/K1/wip/k1_k2a_interaction.rs.txt`) and landed by
//! I8R after K2a merged (main `f12e06876`), against K2a's merged code.
//!
//! The sparse assembly forms each element through the same call as the dense
//! assembly (`global_stiffness`, which K2a checks), in the same order, before
//! any value is accumulated. So a formation refusal is the same named
//! `FrameKernelError::NumericalRange` for the same first element in both
//! representations, and a normal formation gives bit-identical values.
//!
//! Invented inputs only:
//! - R1's frozen RF-RANGE members, as K2a's generated `tests/k2a/rf_range_models.rs`
//!   carries them;
//! - K2a's product-reach operands (`product_physics/tests/k2a_formation_range_runtime.rs`,
//!   `EXACT_ZERO` and `LEAST_SUBNORMAL`), restated below.
//!
//! The section is formed as PP `derive_pipe_section` forms it, as K2a's tests do.
use open_pipe_stress_frame_kernel::structural::{assemble_sparse_stiffness, SparseAssemblyOptions};
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness_with_connectors, FrameElement, FrameKernelError, FrameNode,
    FrameProperties, FrameSection,
};
use std::f64::consts::PI;

// K2a's generated models, shared with its own test; this file uses only the
// RF-RANGE members.
#[path = "k2a/rf_range_models.rs"]
#[allow(dead_code)]
mod models;
use models::*;

/// FK's `AXIS_TOLERANCE` (private in `lib.rs`): an element whose length is
/// at or below it is refused by `FrameElement::new` as `DegenerateAxis`.
const AXIS_TOLERANCE: f64 = 1.0e-12;

/// The section as PP `derive_pipe_section` forms it (as K2a's tests do).
fn product_section(e: f64, g: f64, od: f64, t: f64) -> FrameSection {
    let id = od - 2.0 * t;
    let area = PI * (od.powi(2) - id.powi(2)) / 4.0;
    let second_moment = PI * (od.powi(4) - id.powi(4)) / 64.0;
    FrameSection::new(
        e,
        g,
        area,
        second_moment,
        second_moment,
        2.0 * second_moment,
    )
    .unwrap()
}

fn member_properties(m: &RangeMember) -> FrameProperties {
    FrameProperties::new(
        product_section(
            f64::from_bits(m.e),
            f64::from_bits(m.g),
            f64::from_bits(m.od),
            f64::from_bits(m.t),
        ),
        f64::from_bits(m.length),
    )
    .unwrap()
}

/// A member from `start` along global x, y reference along global y.
fn member_at(
    start: [f64; 3],
    index: usize,
    p: &FrameProperties,
) -> Result<FrameElement, FrameKernelError> {
    FrameElement::new(
        FrameNode::new(index, start).unwrap(),
        FrameNode::new(index + 1, [start[0] + p.length, start[1], start[2]]).unwrap(),
        p.section,
        [0.0, 1.0, 0.0],
    )
}

/// Both assemblies of `frames` on `node_count` nodes: the same error, or
/// bit-identical values.
fn assert_same(node_count: usize, frames: &[FrameElement], ctx: &str) -> Option<FrameKernelError> {
    let dense = assemble_global_stiffness_with_connectors(node_count, frames, &[]);
    let sparse = assemble_sparse_stiffness(
        node_count,
        frames,
        &[],
        &[],
        &[],
        &SparseAssemblyOptions::new(),
    );
    match (dense, sparse) {
        (Ok(d), Ok(s)) => {
            let view = s.to_dense();
            for (a, b) in d.iter().flatten().zip(view.iter().flatten()) {
                assert_eq!(a.to_bits(), b.to_bits(), "{ctx}");
            }
            None
        }
        (Err(d), Err(s)) => {
            assert_eq!(d, s, "{ctx}");
            Some(d)
        }
        (d, s) => panic!("{ctx}: dense {d:?}, sparse {s:?}"),
    }
}

#[test]
fn k1_k2a_rf_range_members_form_or_refuse_identically_in_both_representations() {
    let (mut refused, mut formed, mut degenerate) = (0, 0, 0);
    for m in RF_RANGE_MEMBERS {
        let ctx = format!("{} {}", m.case, m.member);
        let p = member_properties(m);
        match member_at([0.0; 3], 0, &p) {
            // A member at or below the axis tolerance (LEF-small, and the
            // L-240 and SIM-a cases) is refused by the shared element
            // constructor before either assembly (K2a: DegenerateAxis on the
            // product's route), identically for both representations.
            Err(error) => {
                assert!(p.length <= AXIS_TOLERANCE, "{ctx}: {error:?}");
                assert_eq!(
                    error,
                    FrameKernelError::DegenerateAxis {
                        detail: "element length"
                    },
                    "{ctx}"
                );
                degenerate += 1;
            }
            Ok(element) => {
                assert!(p.length > AXIS_TOLERANCE, "{ctx}");
                match assert_same(2, &[element], &ctx) {
                    Some(FrameKernelError::NumericalRange { name }) => {
                        assert_eq!(Some(name), m.refusal, "{ctx}");
                        refused += 1;
                    }
                    Some(other) => panic!("{ctx}: {other:?}"),
                    None => {
                        assert_eq!(m.refusal, None, "{ctx}");
                        formed += 1;
                    }
                }
            }
        }
        if m.case.ends_with("LEF-small") {
            assert!(
                p.length <= AXIS_TOLERANCE,
                "{ctx}: LEF-small never reaches assembly"
            );
        }
    }
    // Not vacuous: LEF-large is refused by name in both, the short members
    // are degenerate, and every other vector forms bit-identically.
    eprintln!("k1 x k2a RF-RANGE: refused {refused}, formed {formed}, degenerate {degenerate}");
    assert!(
        refused > 0 && formed > 0 && degenerate > 0,
        "{refused} {formed} {degenerate}"
    );
}

/// K2a's product-reach operands: a 2^-39 m member, OD 1e-11 m, wall 1e-12 m,
/// G 1e-100 Pa, with E 6.4e-280 Pa (reach_zero: (12*E)*I rounds to 0) or
/// 9.6e-280 Pa (reach_lef: the least subnormal).
fn reach(e: f64) -> FrameProperties {
    FrameProperties::new(
        product_section(e, 1.0e-100, 1.0e-11, 1.0e-12),
        1.8189894035458565e-12,
    )
    .unwrap()
}

#[test]
fn k1_k2a_reach_zero_and_reach_lef_are_refused_with_the_same_named_error() {
    for (label, e) in [("reach_zero", 6.4e-280), ("reach_lef", 9.6e-280)] {
        let element = member_at([0.0; 3], 0, &reach(e)).unwrap();
        let error = assert_same(2, &[element], label).expect("refused");
        assert!(
            matches!(error, FrameKernelError::NumericalRange { .. }),
            "{label}: {error:?}"
        );
        eprintln!("k1 x k2a {label}: {error:?}");
    }
}

#[test]
fn k1_k2a_the_first_failing_element_is_the_same_in_both_representations() {
    // Element 0 forms; element 1 (reach_zero) and element 2 (an RF-RANGE
    // LEF-large member) are both refused: each representation reports
    // element 1's error, before accumulating any value.
    let normal = FrameProperties::new(product_section(200e9, 80e9, 0.2, 0.01), 2.0).unwrap();
    let zero = reach(6.4e-280);
    let large = RF_RANGE_MEMBERS
        .iter()
        .find(|m| m.case.ends_with("LEF-large"))
        .map(member_properties)
        .unwrap();
    let e0 = member_at([0.0; 3], 0, &normal).unwrap();
    let e1 = member_at([2.0, 0.0, 0.0], 1, &zero).unwrap();
    let x2 = 2.0 + zero.length;
    let e2 = member_at([x2, 0.0, 0.0], 2, &large).unwrap();
    let first = assert_same(4, &[e0, e1, e2], "ordered").expect("refused");
    let alone = assert_same(2, &[member_at([0.0; 3], 0, &zero).unwrap()], "alone").unwrap();
    assert_eq!(first, alone);
    let reversed = assert_same(4, &[e2, e1, e0], "reversed").expect("refused");
    assert_ne!(first, reversed, "the order decides which refusal is first");
}
