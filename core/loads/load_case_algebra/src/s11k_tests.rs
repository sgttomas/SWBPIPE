//! S11-K test K6 (S11 section 9): E13, the exact linear combination. Expected
//! values are hand-derived exact nets (the cancelling operands are exact
//! negatives), never produced by the code under test. Each kill case first
//! asserts that today's binary64 fold differs (the precondition rule).
use super::*;

fn operand(id: &str, value: f64) -> AlgebraOperand {
    AlgebraOperand::new(
        id,
        id,
        AlgebraQuantity::new(value, LoadDimension::Force).unwrap(),
        vec![AnalysisStatus::MechanicsSolved],
    )
}

fn combine(values: &[(&str, f64, f64)]) -> AlgebraResult {
    let operands: Vec<_> = values.iter().map(|&(id, v, _)| operand(id, v)).collect();
    let terms: Vec<_> = values
        .iter()
        .map(|&(id, _, c)| CombinationTerm::new(id, c).unwrap())
        .collect();
    evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms })
}

fn fold(values: &[(&str, f64, f64)]) -> f64 {
    values.iter().fold(0.0, |s, &(_, v, c)| s + v * c)
}

#[test]
fn k6_cancelling_combinations_are_exact() {
    for (g, n) in [(1e80, 1e-8), (1e8, 0.3), (1e7, 0.3)] {
        for order in [
            [("A", g, 1.0), ("B", n, 1.0), ("A2", g, -1.0)],
            [("B", n, 1.0), ("A", g, 1.0), ("A2", g, -1.0)],
        ] {
            assert_ne!(
                fold(&order).to_bits(),
                n.to_bits(),
                "precondition {order:?}"
            );
            let result = combine(&order);
            assert!(!result.is_blocked());
            assert_eq!(result.quantity.unwrap().value.to_bits(), n.to_bits());
        }
    }
    // A + B - A2 with (1e80, 1e-8): today's fold gives exactly 0.
    let terms = [("A", 1e80, 1.0), ("B", 1e-8, 1.0), ("A2", 1e80, -1.0)];
    assert_eq!(fold(&terms), 0.0);
    assert_eq!(combine(&terms).quantity.unwrap().value, 1e-8);
}

#[test]
fn k6_exact_products_of_non_unit_factors() {
    let terms = [("X", 1.3, 1.0), ("T", 4.1e7, 1.35), ("T2", 4.1e7, -1.35)];
    assert_eq!(fold(&terms), 1.2999999970197678);
    assert_eq!(
        combine(&terms).quantity.unwrap().value.to_bits(),
        1.3_f64.to_bits()
    );
    // Two-term and same-sign combinations: the exact sum equals today's value.
    let terms = [("sustain", 10.0, 1.0), ("thermal", 2.5, 1.5)];
    assert_eq!(combine(&terms).quantity.unwrap().value, 13.75);
}

#[test]
fn k6_range_and_non_finite_paths() {
    // An exact net outside the binary64 range is the signed infinity (today's
    // fold also overflows); a cancelling overflow-adjacent case is now exact.
    let over = combine(&[("A", 1e300, 1e10)]);
    assert_eq!(over.quantity.unwrap().value, f64::INFINITY);
    let under = combine(&[("A", 1e300, -1e10)]);
    assert_eq!(under.quantity.unwrap().value, f64::NEG_INFINITY);
    let cancel = combine(&[
        ("A", f64::MAX, 1.0),
        ("B", f64::MAX, 1.0),
        ("C", f64::MAX, -1.0),
    ]);
    assert_eq!(cancel.quantity.unwrap().value, f64::MAX);
    // A non-finite operand keeps today's non-finite value.
    let mut bad = operand("N", 1.0);
    bad.quantity.value = f64::NAN;
    let operands = vec![bad, operand("M", 2.0)];
    let terms = vec![
        CombinationTerm::new("N", 1.0).unwrap(),
        CombinationTerm::new("M", 1.0).unwrap(),
    ];
    let result = evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms });
    assert!(result.quantity.unwrap().value.is_nan());
    // An exact zero is +0.0.
    let zero = combine(&[("A", 5.0, -1.0), ("B", 5.0, 1.0)]);
    assert_eq!(zero.quantity.unwrap().value.to_bits(), 0);
}
