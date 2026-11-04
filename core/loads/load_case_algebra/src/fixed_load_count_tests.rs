//! κ (DEL-05-02 Work `absolute-sum-srss-combinations.md` §5.1, V-6, V-9 and V-10 at
//! the kernel): the flat decisions are the unit factor-sum rule's, bit for bit; the
//! nested ones follow the exact κ.

use super::fixed_load_count::*;
use super::factor_sum_within_unit_tolerance;
use std::cmp::Ordering;

const T: f64 = 4.9406564584124654e-324; // 2^-1074

fn case() -> FixedLoadCount {
    FixedLoadCount::load_case()
}
fn mech(terms: &[(f64, &FixedLoadCount)]) -> FixedLoadCount {
    FixedLoadCount::mechanics(terms.iter().map(|(f, k)| (*f, *k)))
}
fn dy(value: f64) -> Dyadic {
    Dyadic::from_f64(value).unwrap()
}
/// The flat mechanics gate over load cases with these factors.
fn flat_admits(factors: &[f64]) -> bool {
    let cases: Vec<FixedLoadCount> = factors.iter().map(|_| case()).collect();
    let own = FixedLoadCount::mechanics(factors.iter().copied().zip(cases.iter()));
    mechanics_fixed_load_gate(&own, cases.iter()).is_none()
}

#[test]
fn dyadic_arithmetic_is_exact() {
    assert_eq!(dy(0.0), Dyadic::zero());
    assert_eq!(dy(-0.0), Dyadic::zero());
    assert_eq!(dy(1.0), Dyadic::one());
    assert_eq!(dy(T).significand_bits(), 1);
    assert_eq!(dy(f64::MAX).significand_bits(), 53);
    assert!(Dyadic::from_f64(f64::NAN).is_none() && Dyadic::from_f64(f64::INFINITY).is_none());
    // 0.3 + 0.7 = 1 - 2^-54 exactly.
    let sum = dy(0.3).plus(&dy(0.7)).unwrap();
    assert_eq!(sum, Dyadic::one().minus(&dy(2f64.powi(-54))).unwrap());
    // Cancellation to exact zero, and a sign.
    assert_eq!(dy(2f64.powi(52)).minus(&dy(2f64.powi(52))).unwrap(), Dyadic::zero());
    assert_eq!(dy(1.0).minus(&dy(3.0)).unwrap().signum(), -1);
    assert_eq!(dy(-2.0).times_f64(-0.5).unwrap(), Dyadic::one());
    // 1 + 2^-1074 needs 1,075 bits; MAX + T needs 2,098.
    assert_eq!(dy(1.0).plus(&dy(T)).unwrap().significand_bits(), 1075);
    assert_eq!(dy(f64::MAX).plus(&dy(T)).unwrap().significand_bits(), 2098);
    assert_eq!(dy(3.0).compare_magnitude(&dy(-4.0)), Ordering::Less);
    assert_eq!(dy(-4.0).compare_magnitude(&dy(4.0)), Ordering::Equal);
    assert_eq!(Dyadic::zero().compare_magnitude(&dy(T)), Ordering::Less);
}

#[test]
fn the_representation_refuses_beyond_4096_bits() {
    // 1 + 2^-4095 has 4,096 bits; 1 + 2^-4096 has 4,097.
    let tiny = |e: i32| {
        // 2^e by exact products of binary64 factors.
        let mut value = Dyadic::one();
        let mut left = e;
        while left < 0 {
            let step = left.max(-1000);
            value = value.times_f64(2f64.powi(step)).unwrap();
            left -= step;
        }
        value
    };
    assert_eq!(Dyadic::one().plus(&tiny(-4095)).unwrap().significand_bits(), 4096);
    assert!(Dyadic::one().plus(&tiny(-4096)).is_none());
    assert!(Dyadic::one().plus(&tiny(-9000)).is_none());
    // An exact small result of wide operands is formed (cancellation).
    let wide = Dyadic::one().plus(&tiny(-4095)).unwrap();
    assert_eq!(wide.minus(&tiny(-4095)).unwrap(), Dyadic::one());
}

/// §5.1: the flat decisions equal the unit factor-sum rule's (option C).
#[test]
fn flat_mechanics_decisions_are_the_unit_factor_sum_rule() {
    let bound = 2f64.powi(-46);
    for factors in [&[0.3, 0.7][..], &[1.2, -0.2], &[1.0 / 3.0, 2.0 / 3.0], &[1.0 + bound], &[1.0 - bound], &[1.0, bound], &[1.0]] {
        assert!(flat_admits(factors), "admitted {factors:?}");
        assert!(factor_sum_within_unit_tolerance(factors.iter().copied()).unwrap());
    }
    let p52 = 2f64.powi(52);
    for factors in [&[2.0][..], &[1.0 + 2f64.powi(-45)], &[p52, -p52], &[2f64.powi(53), -2f64.powi(53)], &[f64::MAX, f64::MAX], &[], &[1.0, bound, T], &[0.0]] {
        assert!(!flat_admits(factors), "withheld {factors:?}");
        assert!(!factor_sum_within_unit_tolerance(factors.iter().copied()).unwrap());
    }
    // A sample of factor pairs and triples near and away from a unit sum agrees.
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    for _ in 0..4000 {
        let a = (next() % 2001) as f64 / 1000.0 - 0.5;
        let nudge = [0.0, bound, -bound, 2f64.powi(-47), 2f64.powi(-45), T][(next() % 6) as usize];
        let factors = if next() % 2 == 0 { vec![a, 1.0 - a + nudge] } else { vec![a, 0.5 - a, 0.5 + nudge] };
        assert_eq!(flat_admits(&factors), factor_sum_within_unit_tolerance(factors.iter().copied()).unwrap(), "{factors:?}");
    }
}

#[test]
fn the_tree_counts_the_fixed_load() {
    let (a, b) = (case(), case());
    // (base + EQ) − base has κ = 0 exactly.
    let difference = FixedLoadCount::subtraction(&a, &b);
    assert!(difference.is_exact_zero());
    let srss = FixedLoadCount::unsigned([&difference, &difference.clone()]);
    assert!(srss.is_exact_zero());
    // srss over load cases is undefined; so is an envelope of a case and a difference.
    assert_eq!(FixedLoadCount::unsigned([&a]), FixedLoadCount::Undefined(Undefined::UnsignedNonzero));
    assert_eq!(FixedLoadCount::envelope([&a, &difference]), FixedLoadCount::Undefined(Undefined::EnvelopeMixed));
    assert_eq!(FixedLoadCount::envelope([&a, &b]), case());
    // An undefined operand makes every dependant undefined.
    let mixed = FixedLoadCount::envelope([&a, &difference]);
    for dependant in [
        mech(&[(1.0, &mixed)]),
        FixedLoadCount::subtraction(&mixed, &a),
        FixedLoadCount::envelope([&mixed]),
        FixedLoadCount::unsigned([&difference, &mixed]),
    ] {
        assert_eq!(dependant, FixedLoadCount::Undefined(Undefined::Operand));
    }
    // sustained + SRSS of differences: κ = 1.
    let occasional = mech(&[(1.0, &a), (1.0, &srss)]);
    assert_eq!(occasional, case());
    assert!(mechanics_fixed_load_gate(&occasional, [&a, &srss]).is_none());
}

/// V-6 at the kernel: the gates over nested operands.
#[test]
fn v6_the_gates_over_nested_operands() {
    let (a, b) = (case(), case());
    let withheld_unsigned = Some(FixedLoadWithholding::UnsignedOperand);
    // SRSS over load cases: withheld; over differences: admitted.
    assert_eq!(unsigned_fixed_load_gate([&a, &b]), withheld_unsigned);
    let difference = FixedLoadCount::subtraction(&a, &b);
    assert_eq!(unsigned_fixed_load_gate([&difference]), None);
    // A mixed envelope operand: withheld (unsigned) and named (mechanics).
    let mixed = FixedLoadCount::envelope([&a, &difference]);
    assert_eq!(unsigned_fixed_load_gate([&mixed]), withheld_unsigned);
    let over_mixed = mech(&[(1.0, &mixed)]);
    assert_eq!(mechanics_fixed_load_gate(&over_mixed, [&mixed]), Some(FixedLoadWithholding::OperandUndefined));
    // mechanics(1·(A − B)), κ = 0: withheld under the unit rule.
    let nested = mech(&[(1.0, &difference)]);
    assert_eq!(mechanics_fixed_load_gate(&nested, [&difference]), Some(FixedLoadWithholding::UnitFactorSum));
    // r1 F1: D = (1 + 2^-46)A − B has κ = 2^-46; 2^46·D carries the fixed load once
    // but is no reset to 0, so absolute_sum(2^46·D) is withheld.
    let d = mech(&[(1.0 + 2f64.powi(-46), &a), (-1.0, &b)]);
    assert_eq!(d.defined().unwrap(), &dy(2f64.powi(-46)));
    assert_eq!(unsigned_fixed_load_gate([&d]), withheld_unsigned);
    let amplified = mech(&[(2f64.powi(46), &d)]);
    assert_eq!(amplified, case());
    assert_eq!(unsigned_fixed_load_gate([&amplified]), withheld_unsigned);
    assert_eq!(FixedLoadCount::unsigned([&d]), FixedLoadCount::Undefined(Undefined::UnsignedNonzero));
    // B plus it would publish the fixed load twice under κ = 1: its κ is undefined.
    let twice = mech(&[(1.0, &b), (1.0, &FixedLoadCount::unsigned([&amplified]))]);
    assert_eq!(twice, FixedLoadCount::Undefined(Undefined::Operand));
}

/// V-9 at the kernel: decision OQ-1.
#[test]
fn v9_solved_state_plus_increments() {
    let (a, b) = (case(), case());
    let difference = FixedLoadCount::subtraction(&a, &b);
    let srss = FixedLoadCount::unsigned([&difference]);
    let admitted = |terms: &[(bool, f64, &FixedLoadCount)]| solved_state_plus_increments(terms.iter().copied());
    assert!(admitted(&[(true, 1.0, &a), (false, 1.0, &srss)]));
    assert!(admitted(&[(false, -1.0, &srss), (true, 1.0, &a), (false, 0.5, &difference)]));
    // Factor not exactly 1; two load cases; κ ≠ 0; undefined κ (B + srss(A)); singleton; flat.
    assert!(!admitted(&[(true, 1.0 + f64::EPSILON, &a), (false, 1.0, &srss)]));
    assert!(!admitted(&[(true, 0.9999999999999999, &a), (false, 1.0, &srss)]));
    assert!(!admitted(&[(true, 1.0, &a), (true, 1.0, &b), (false, 1.0, &srss)]));
    let one = mech(&[(1.0, &a)]);
    assert!(!admitted(&[(true, 1.0, &a), (false, 1.0, &one)]));
    let srss_of_case = FixedLoadCount::unsigned([&a]);
    assert!(!admitted(&[(true, 1.0, &b), (false, 1.0, &srss_of_case)]));
    assert!(!admitted(&[(true, 1.0, &a)]));
    assert!(!admitted(&[(true, 1.0, &a), (true, 1.0, &b)]));
    assert!(!admitted(&[(false, 1.0, &srss)]));
    // κ = 2^-46 is not exactly 0.
    let near = mech(&[(1.0 + 2f64.powi(-46), &a), (-1.0, &b)]);
    assert!(!admitted(&[(true, 1.0, &a), (false, 1.0, &near)]));
}

/// V-10: the r2 chain M₀ = A + tB, Mₙ₊₁ = A + t(Mₙ − A), t = 2⁻¹⁰⁷⁴.
#[test]
fn v10_the_representation_chain() {
    let (a, b) = (case(), case());
    let mut m = mech(&[(1.0, &a), (T, &b)]);
    let bits = [1075, 2149, 3223];
    for (level, expected) in bits.iter().enumerate() {
        assert_eq!(m.defined().unwrap().significand_bits(), *expected, "M{level}");
        if level == 2 {
            break;
        }
        let increment = FixedLoadCount::subtraction(&m, &a);
        m = mech(&[(1.0, &a), (T, &increment)]);
    }
    // κ(M₂ − A) = t³ exactly, not 0, so an unsigned combination of it is withheld.
    let m2 = m.clone();
    let increment = FixedLoadCount::subtraction(&m2, &a);
    let t3 = dy(T).times_f64(T).unwrap().times_f64(T).unwrap();
    assert_eq!(increment.defined().unwrap(), &t3);
    assert!(!increment.is_exact_zero());
    assert_eq!(unsigned_fixed_load_gate([&increment]), Some(FixedLoadWithholding::UnsignedOperand));
    // M₃ = A + t(M₂ − A) needs 4,297 bits: undefined and named.
    let m3 = mech(&[(1.0, &a), (T, &increment)]);
    assert!(m3.is_unrepresentable());
    // Q = mechanics(2A + t(M₂ − A)), κ = 2 + t⁴ (4,298 bits): undefined although its
    // operands are defined, so the unit rule withholds it.
    let q = mech(&[(2.0, &a), (T, &increment)]);
    assert!(q.is_unrepresentable());
    assert_eq!(mechanics_fixed_load_gate(&q, [&a, &increment]), Some(FixedLoadWithholding::UnitFactorSum));
    // A dependant of M₃ is undefined by its operand, not itself unrepresentable.
    let dependant = FixedLoadCount::subtraction(&m3, &a);
    assert_eq!(dependant, FixedLoadCount::Undefined(Undefined::Operand));
    assert!(!dependant.is_unrepresentable());
}

#[test]
fn the_unit_rule_is_decided_without_forming_an_unrepresentable_offset() {
    // κ = (1 + 2^-1074)·2^-4296: κ and 1 are more than 4,097 binades apart, so κ − 1
    // is not representable; κ < 1/2.
    let mut kappa = dy(1.0).plus(&dy(T)).unwrap(); // 1 + 2^-1074
    for _ in 0..4 {
        kappa = kappa.times_f64(T).unwrap();
    }
    assert!(kappa.minus(&Dyadic::one()).is_none());
    assert!(!within_unit_factor_sum(&kappa));
    assert!(within_unit_factor_sum(&Dyadic::one()));
    assert!(!within_unit_factor_sum(&Dyadic::zero()));
    assert!(!within_unit_factor_sum(&dy(-1.0)));
}
