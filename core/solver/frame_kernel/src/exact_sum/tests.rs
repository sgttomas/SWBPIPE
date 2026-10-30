//! K1 and K2 (S11 section 9). Expected values are correctly rounded Fraction
//! references: the generated table in `k1_reference.rs` (Python
//! `fractions.Fraction`, generator recorded with the S11-K run records) and
//! hand-derived exact values below. None comes from the code under test.
use super::*;
#[rustfmt::skip]
mod k1_reference;
use k1_reference::*;

fn f(bits: u64) -> f64 {
    f64::from_bits(bits)
}

#[test]
fn k1_seeded_random_sums_match_fraction_reference() {
    assert!(RANDOM_SUMS.len() >= 150);
    let mut differs_from_fold = 0;
    for (values, expected) in RANDOM_SUMS {
        let values: Vec<f64> = values.iter().map(|&b| f(b)).collect();
        let actual = exact_rounded_sum(values.iter().copied()).unwrap();
        assert_eq!(actual.to_bits(), *expected, "{values:?}");
        let fold = values.iter().fold(0.0, |s, v| s + v);
        if fold.to_bits() != *expected {
            differs_from_fold += 1;
        }
    }
    // The table must contain cases a binary64 fold gets wrong.
    assert!(differs_from_fold > 10, "{differs_from_fold}");
}

#[test]
fn k1_seeded_random_dots_match_fraction_reference() {
    assert!(RANDOM_DOTS.len() >= 150);
    let mut differs_from_fold = 0;
    for (pairs, expected) in RANDOM_DOTS {
        let pairs: Vec<(f64, f64)> = pairs.iter().map(|&(a, b)| (f(a), f(b))).collect();
        let actual = exact_rounded_dot(pairs.iter().copied()).unwrap();
        assert_eq!(actual.to_bits(), *expected, "{pairs:?}");
        let fold = pairs.iter().fold(0.0, |s, (a, b)| s + a * b);
        if fold.to_bits() != *expected {
            differs_from_fold += 1;
        }
    }
    assert!(differs_from_fold > 10, "{differs_from_fold}");
}

#[test]
fn k1_probe_d_is_correctly_rounded_where_ascending_naive_sum_is_not() {
    let values = [2.0_f64.powi(-110), 2.0_f64.powi(-53), 1.0];
    // Precondition: the ascending naive sum gives 1.0.
    let mut ascending = values;
    ascending.sort_by(|a, b| a.abs().total_cmp(&b.abs()));
    assert_eq!(ascending.iter().fold(0.0, |s, v| s + v), 1.0);
    assert_eq!(exact_rounded_sum(values).unwrap().to_bits(), PROBE_D);
    assert_eq!(f(PROBE_D), 1.0000000000000002);
}

#[test]
fn k1_cancellations_of_three_or_more_terms() {
    // S11 section 5.3 orders (G, n, -G) and (n, G, -G); exact net is n.
    for (g, n) in [(1e5, 0.3), (1e6, 0.3), (1e7, 0.3), (1e8, 0.3), (1e80, 1e-8)] {
        for order in [[g, n, -g], [n, g, -g], [g, -g, n]] {
            assert_eq!(exact_rounded_sum(order).unwrap().to_bits(), n.to_bits());
        }
    }
    // The realistic thermal pair after a 1.3 N co-axial load.
    let thermal = [1.3, 4.1e7, -4.1e7];
    assert_eq!(thermal.iter().fold(0.0, |s, v| s + v), 1.2999999970197678);
    assert_eq!(exact_rounded_sum(thermal).unwrap().to_bits(), THERMAL);
}

#[test]
fn k1_ties_overflow_adjacent_and_subnormal_nets() {
    let half = 2.0_f64.powi(-53);
    let tiny = f64::from_bits(1);
    // Ties to even and sticky bits.
    assert_eq!(exact_rounded_sum([1.0, half]).unwrap(), 1.0);
    assert_eq!(
        exact_rounded_sum([1.0, half, tiny]).unwrap().to_bits(),
        1.0_f64.to_bits() + 1
    );
    let odd = f64::from_bits(1.0_f64.to_bits() + 1);
    assert_eq!(
        exact_rounded_sum([odd, half]).unwrap().to_bits(),
        1.0_f64.to_bits() + 2
    );
    // Overflow-adjacent: exact intermediates never overflow.
    assert_eq!(
        exact_rounded_sum([f64::MAX, f64::MAX, -f64::MAX]).unwrap(),
        f64::MAX
    );
    let ulp_max = 2.0_f64.powi(971);
    assert_eq!(
        exact_rounded_sum([f64::MAX, ulp_max / 4.0]).unwrap(),
        f64::MAX
    );
    // MAX + half an ulp is a tie; even is 2^1024, outside the range.
    assert_eq!(
        exact_rounded_sum([f64::MAX, ulp_max / 2.0]),
        Err(SumError::NonRepresentable)
    );
    assert_eq!(
        exact_rounded_sum([-f64::MAX, -f64::MAX]),
        Err(SumError::NonRepresentable)
    );
    // Subnormal nets.
    assert_eq!(
        exact_rounded_sum([f64::MIN_POSITIVE, -tiny])
            .unwrap()
            .to_bits(),
        (1u64 << 52) - 1
    );
    assert_eq!(exact_rounded_sum([tiny, tiny]).unwrap().to_bits(), 2);
    assert_eq!(
        exact_rounded_sum([1e-300, 3.0 * tiny, -1e-300])
            .unwrap()
            .to_bits(),
        3
    );
    assert_eq!(exact_rounded_sum([f64::NAN]), Err(SumError::NonFinite));
    assert_eq!(
        exact_rounded_dot([(1.0, f64::INFINITY)]),
        Err(SumError::NonFinite)
    );
}

#[test]
fn k1_probe_x_cases() {
    // Exact products place the value at the 2^-2148 quantum; rounding is at
    // the binary64 subnormal quantum, never by copying accumulator bits.
    let p = 2.0_f64.powi(-538);
    // 3 * 2^-1076 = (3 * 2^-538) * 2^-538 rounds to 2^-1074.
    assert_eq!(
        exact_rounded_dot([(3.0 * p, p)]).unwrap().to_bits(),
        THREE_2M1076
    );
    // A tie at 2^-1075 rounds to even: +0.0.
    let tie = exact_rounded_dot([(p, 2.0_f64.powi(-537))]).unwrap();
    assert_eq!(tie.to_bits(), TIE_2M1075);
    // (1.7e308)^2 - (1.7e308)^2 is exactly zero.
    assert_eq!(
        exact_rounded_dot([(1.7e308, 1.7e308), (-1.7e308, 1.7e308)])
            .unwrap()
            .to_bits(),
        0
    );
    // -2^-1080 underflows to +0.0 under the stated rule (not IEEE's -0.0).
    let negative_underflow = exact_rounded_dot([(-p, 2.0_f64.powi(-542))]).unwrap();
    assert_eq!(negative_underflow.to_bits(), NEG_2M1080);
    // 1.3 + 1.35*4.1e7 - 1.35*4.1e7 as exact products is 1.3.
    let mut acc = ExactAccumulator::new();
    acc.add(1.3).unwrap();
    acc.add_product(1.35, 4.1e7).unwrap();
    acc.add_product(-1.35, 4.1e7).unwrap();
    assert_eq!(acc.round().unwrap().to_bits(), PROBE_X_PRODUCTS);
    // The smallest exact product, 2^-2148 (the accumulator's quantum), and
    // its negation are far below half the least subnormal: +0.0 both.
    let least = tiny_value();
    assert_eq!(exact_rounded_dot([(least, least)]).unwrap().to_bits(), 0);
    assert_eq!(exact_rounded_dot([(-least, least)]).unwrap().to_bits(), 0);
    // Largest exact products fit the 68-limb accumulator.
    let mut big = ExactAccumulator::new();
    big.add_product(f64::MAX, f64::MAX).unwrap();
    big.add_product(-f64::MAX, f64::MAX).unwrap();
    big.add(tiny_value()).unwrap();
    assert_eq!(big.round().unwrap().to_bits(), 1);
}

fn tiny_value() -> f64 {
    f64::from_bits(1)
}

#[test]
fn k1_scaling_is_applied_before_the_single_rounding() {
    // 3 * 2^-1080 is below the subnormal quantum unscaled; scaled by 2^100
    // it is exactly 3 * 2^-980. Rounding first would give +0.0 or 2^-1074.
    let mut acc = ExactAccumulator::new();
    acc.add_product(3.0 * 2.0_f64.powi(-540), 2.0_f64.powi(-540))
        .unwrap();
    assert_eq!(acc.round().unwrap().to_bits(), 0);
    assert_eq!(acc.round_scaled(100).unwrap(), 3.0 * 2.0_f64.powi(-980));
    // Negative scale into the subnormal range rounds once, at 2^-1074.
    let mut acc = ExactAccumulator::new();
    acc.add(3.0 * 2.0_f64.powi(-1000)).unwrap();
    assert_eq!(acc.round_scaled(-75).unwrap().to_bits(), 2); // 1.5 -> 2 (even)
    assert_eq!(acc.round_scaled(2100), Err(SumError::NonRepresentable));
}

#[test]
fn k2_every_zero_is_positive_zero() {
    for values in [
        vec![1e8, -1e8],
        vec![-0.0],
        vec![-0.0, -0.0],
        vec![],
        vec![-5.0, 2.5, 2.5],
    ] {
        let sum = exact_rounded_sum(values.iter().copied()).unwrap();
        assert_eq!(sum.to_bits(), 0, "{values:?}");
    }
    assert_eq!(exact_rounded_dot([(-0.0, 3.0)]).unwrap().to_bits(), 0);
    assert_eq!(
        exact_rounded_dot([(-2.0, 3.0), (2.0, 3.0)])
            .unwrap()
            .to_bits(),
        0
    );
    let empty = ExactAccumulator::new();
    assert!(empty.is_zero());
    assert_eq!(empty.signum(), 0);
    assert_eq!(empty.round_scaled(-40).unwrap().to_bits(), 0);
}
