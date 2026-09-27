//! S11-K test K8 (S11 section 9, section 4.5): the exact context's coverage
//! checks also accept the correctly rounded exact sum of the same terms, and
//! still refuse a corrupted value. For the terms {1, 2^-53, 2^-110} the naive
//! projection and the ordered fold both give 1.0, while the correctly rounded
//! net (hand-derived) is 1 + 2^-52.
use super::*;

const NEXT_UP_ONE: f64 = 1.0000000000000002;

fn terms() -> [f64; 3] {
    [1.0, 2.0_f64.powi(-53), 2.0_f64.powi(-110)]
}

#[test]
fn k8_force_coverage_accepts_the_correctly_rounded_net_and_refuses_corruption() {
    let t = terms();
    // Precondition: both of today's accepted projections give 1.0.
    assert_eq!(t.iter().fold(0.0, |s, v| s + v), 1.0);
    let k = vec![vec![1.0]];
    let cs = [StiffnessContribution {
        row: 0,
        col: 0,
        value: 1.0,
    }];
    let fixed = [(0, 0.0)];
    let force_terms: Vec<ForceContribution> = t
        .iter()
        .enumerate()
        .map(|(i, &value)| ForceContribution {
            source: format!("load:{i}"),
            dof: 0,
            value,
        })
        .collect();
    for (stored, accepted) in [
        (NEXT_UP_ONE, true),
        (1.0, true),
        (1.0000000000000004, false),
    ] {
        let force = [stored];
        let system = StructuralSystem {
            stiffness: &k,
            force: &force,
            free_dofs: &[],
            prescribed: &fixed,
            contributions: Some(&cs),
            symmetry: None,
        };
        let result = Context::new(
            &system,
            "s11k force terms",
            ForceBasis::IdentifiedContributions(&force_terms),
            Limits::default(),
        );
        match (accepted, result) {
            (true, Ok(_)) => {}
            (false, Err(Error::Invalid("force source mismatch"))) => {}
            (expected, other) => panic!(
                "stored {stored}: expected accepted={expected}, got {:?}",
                other.err()
            ),
        }
    }
}

#[test]
fn k8_stiffness_coverage_accepts_the_correctly_rounded_net_and_refuses_corruption() {
    let t = terms();
    let cs: Vec<StiffnessContribution> = t
        .iter()
        .map(|&value| StiffnessContribution {
            row: 0,
            col: 0,
            value,
        })
        .collect();
    let fixed = [(0, 0.0)];
    for (stored, accepted) in [
        (NEXT_UP_ONE, true),
        (1.0, true),
        (1.0000000000000004, false),
    ] {
        let k = vec![vec![stored]];
        let system = StructuralSystem {
            stiffness: &k,
            force: &[1.0],
            free_dofs: &[],
            prescribed: &fixed,
            contributions: Some(&cs),
            symmetry: None,
        };
        let result = Context::new(
            &system,
            "s11k stiffness",
            ForceBasis::DeclaredVector,
            Limits::default(),
        );
        match (accepted, result) {
            (true, Ok(_)) => {}
            (false, Err(Error::Invalid("stiffness source mismatch"))) => {}
            (expected, other) => panic!(
                "stored {stored}: expected accepted={expected}, got {:?}",
                other.err()
            ),
        }
    }
}
