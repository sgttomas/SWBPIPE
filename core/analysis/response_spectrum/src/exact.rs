//! Exact comparisons of a period query with a period knot (Agent 0's decision 2, recorded in
//! the DEL-04-10 scope).
//!
//! The operands are represented values: the query frequency f as the binary64 the modal result
//! publishes (or the user enters), and each knot as the binary64 of the authored abscissa. The
//! period of that represented f is the exact rational T = 1/f. Rounding 1/f to binary64 can land
//! on a knot that T is not equal to, or overflow, so the rounded reciprocal never decides where T
//! lies. The exact comparison selects the interval and decides in or out of range, and nothing
//! more: it makes neither operand exact (the modal frequency carries the modal solve's own error,
//! a knot is the binary64 nearest to what the user wrote), and the interpolation after it is
//! binary64 arithmetic with an error bound conditional on the arithmetic and libm assumptions
//! stated in [`crate::table`].
//!
//! For a positive knot P, T < P exactly when P·f > 1, so the comparison is the sign of the
//! exact product P·f − 1. Each positive finite binary64 is m·2^e with integer m < 2^53, so the
//! product is m_P·m_f·2^(e_P+e_f) with m_P·m_f < 2^106: an exact integer comparison over every
//! positive finite operand, subnormal or near the overflow threshold, with no floating-point
//! operation.
use std::cmp::Ordering;

/// `x = m·2^e` exactly, `m ≥ 1`, for a positive finite `x`.
pub(crate) fn decompose(x: f64) -> (u64, i32) {
    debug_assert!(x.is_finite() && x > 0.0);
    let bits = x.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1u64 << 52) - 1);
    if biased == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1u64 << 52), biased - 1075)
    }
}

/// The ordering of the exact product `a·b` against 1, for positive finite `a` and `b`.
pub(crate) fn product_cmp_one(a: f64, b: f64) -> Ordering {
    let (ma, ea) = decompose(a);
    let (mb, eb) = decompose(b);
    let m = u128::from(ma) * u128::from(mb); // 1 ≤ m < 2^106
    let e = ea + eb;
    if e >= 0 {
        // m·2^e ≥ 1, with equality only for m = 1 and e = 0.
        if e == 0 && m == 1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    } else if -e >= 107 {
        Ordering::Less // m < 2^106 < 2^(−e)
    } else {
        m.cmp(&(1u128 << (-e)))
    }
}

/// Where the exact period `T = 1/frequency` of the represented binary64 `frequency` lies
/// relative to the represented binary64 `knot`: `Less` when `T < knot`, `Equal` when `T = knot`
/// exactly, `Greater` when `T > knot`. Both arguments are positive finite binary64 values; the
/// comparison of those represented values is exact (module documentation).
///
/// # Panics
/// If either argument is not positive and finite. The validated table and query never pass one.
pub fn period_query_ordering(frequency: f64, knot: f64) -> Ordering {
    assert!(frequency.is_finite() && frequency > 0.0, "frequency must be positive and finite");
    assert!(knot.is_finite() && knot > 0.0, "knot must be positive and finite");
    // T < P ⇔ P·f > 1.
    product_cmp_one(knot, frequency).reverse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decomposition_is_exact_at_the_ends_of_the_range() {
        assert_eq!(decompose(1.0), (1u64 << 52, -52));
        assert_eq!(decompose(f64::from_bits(1)), (1, -1074));
        assert_eq!(decompose(f64::MIN_POSITIVE), (1u64 << 52, -1074));
        assert_eq!(decompose(f64::MAX), ((1u64 << 53) - 1, 971));
    }

    #[test]
    fn products_against_one() {
        assert_eq!(product_cmp_one(0.5, 2.0), Ordering::Equal);
        assert_eq!(product_cmp_one(f64::MAX, f64::MAX), Ordering::Greater);
        assert_eq!(product_cmp_one(f64::from_bits(1), f64::from_bits(1)), Ordering::Less);
        // 2^-1023 is subnormal; its product with 2^1023 is exactly 1.
        assert_eq!(product_cmp_one(f64::from_bits(1u64 << 51), 2f64.powi(1023)), Ordering::Equal);
    }
}
