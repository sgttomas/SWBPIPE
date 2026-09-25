//! Exact finite-binary64 sum followed by one nearest-even binary64 projection.
//!
//! Every finite operand is an integer multiple of 2^-1074. Separate unsigned
//! accumulators retain positive/negative integers until final subtraction. The
//! largest finite operand occupies at most bit2097; at most usize::MAX operands
//! on the supported <=64-bit hosts require at most2162 bits.34 limbs provide2176.
//! This does not recover information rounded before the operands were supplied.
const LIMBS: usize = 34;
type Magnitude = [u64; LIMBS];
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SumError {
    NonFinite,
    AccumulatorOverflow,
    NonRepresentable,
}
fn add_word(value: &mut Magnitude, mut word: usize, mut add: u64) -> Result<(), SumError> {
    while add != 0 {
        let target = value.get_mut(word).ok_or(SumError::AccumulatorOverflow)?;
        let (next, carry) = target.overflowing_add(add);
        *target = next;
        add = u64::from(carry);
        word += 1;
    }
    Ok(())
}
fn add_operand(value: &mut Magnitude, bits: u64) -> Result<(), SumError> {
    let exponent = ((bits >> 52) & 0x7ff) as usize;
    let mantissa = (bits & ((1u64 << 52) - 1)) | if exponent == 0 { 0 } else { 1u64 << 52 };
    let shift = if exponent == 0 { 0 } else { exponent - 1 };
    let word = shift / 64;
    let offset = shift % 64;
    add_word(value, word, mantissa << offset)?;
    if offset != 0 {
        add_word(value, word + 1, mantissa >> (64 - offset))?;
    }
    Ok(())
}
fn greater(a: &Magnitude, b: &Magnitude) -> std::cmp::Ordering {
    for i in (0..LIMBS).rev() {
        let order = a[i].cmp(&b[i]);
        if !order.is_eq() {
            return order;
        }
    }
    std::cmp::Ordering::Equal
}
fn subtract(a: &Magnitude, b: &Magnitude) -> Magnitude {
    let mut out = [0; LIMBS];
    let mut borrow = false;
    for i in 0..LIMBS {
        let (x, b1) = a[i].overflowing_sub(b[i]);
        let (y, b2) = x.overflowing_sub(u64::from(borrow));
        out[i] = y;
        borrow = b1 || b2;
    }
    debug_assert!(!borrow);
    out
}
fn bit(a: &Magnitude, index: usize) -> bool {
    a[index / 64] & (1u64 << (index % 64)) != 0
}
fn any_below(a: &Magnitude, end: usize) -> bool {
    let full = end / 64;
    if a[..full].iter().any(|&x| x != 0) {
        return true;
    }
    let rem = end % 64;
    rem != 0 && a[full] & ((1u64 << rem) - 1) != 0
}
fn shifted_low(a: &Magnitude, shift: usize) -> u64 {
    let word = shift / 64;
    let offset = shift % 64;
    let mut out = a[word] >> offset;
    if offset != 0 && word + 1 < LIMBS {
        out |= a[word + 1] << (64 - offset);
    }
    out
}
fn project(a: &Magnitude, negative: bool) -> Result<f64, SumError> {
    let Some(word) = (0..LIMBS).rev().find(|&i| a[i] != 0) else {
        return Ok(0.0);
    };
    let mut highest = word * 64 + 63 - a[word].leading_zeros() as usize;
    let sign = if negative { 1u64 << 63 } else { 0 };
    if highest < 52 {
        return Ok(f64::from_bits(sign | a[0]));
    }
    let shift = highest - 52;
    let mut mantissa = shifted_low(a, shift) & ((1u64 << 53) - 1);
    if shift > 0 && bit(a, shift - 1) && (any_below(a, shift - 1) || mantissa & 1 != 0) {
        mantissa += 1;
    }
    if mantissa == (1u64 << 53) {
        mantissa >>= 1;
        highest += 1;
    }
    let exponent = highest - 51;
    if exponent >= 0x7ff {
        return Err(SumError::NonRepresentable);
    }
    Ok(f64::from_bits(
        sign | ((exponent as u64) << 52) | (mantissa & ((1u64 << 52) - 1)),
    ))
}
pub(crate) fn exact_sum(values: impl IntoIterator<Item = f64>) -> Result<f64, SumError> {
    let mut positive = [0; LIMBS];
    let mut negative = [0; LIMBS];
    for value in values {
        if !value.is_finite() {
            return Err(SumError::NonFinite);
        }
        let bits = value.to_bits();
        add_operand(
            if bits >> 63 == 0 {
                &mut positive
            } else {
                &mut negative
            },
            bits,
        )?;
    }
    match greater(&positive, &negative) {
        std::cmp::Ordering::Less => project(&subtract(&negative, &positive), true),
        _ => project(&subtract(&positive, &negative), false),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cancellation_retains_source_coefficients_and_is_order_independent() {
        let small = 2.0_f64.powi(-53);
        for values in [[1.0, small, -1.0], [-1.0, 1.0, small], [small, -1.0, 1.0]] {
            assert_eq!(exact_sum(values).unwrap(), small);
        }
        let nu = f64::from_bits(0x3fdfffffffffffff);
        assert_eq!(exact_sum([1.0, -2.0 * nu]).unwrap(), small);
    }
    #[test]
    fn rounding_ties_and_sticky_bits_are_explicit() {
        let half = 2.0_f64.powi(-53);
        let tiny = f64::from_bits(1);
        assert_eq!(exact_sum([1.0, half]).unwrap().to_bits(), 1.0_f64.to_bits());
        assert_eq!(
            exact_sum([1.0, half, tiny]).unwrap().to_bits(),
            1.0_f64.to_bits() + 1
        );
        assert_eq!(
            exact_sum([-1.0, -half, -tiny]).unwrap().to_bits(),
            (-1.0_f64).to_bits() + 1
        );
    }
    #[test]
    fn wide_range_cancellation_never_overflows_an_intermediate_float() {
        assert_eq!(
            exact_sum([f64::MAX, f64::MAX, -f64::MAX]).unwrap(),
            f64::MAX
        );
        assert_eq!(
            exact_sum([f64::MAX, -f64::MAX, f64::from_bits(1)]).unwrap(),
            f64::from_bits(1)
        );
        assert_eq!(
            exact_sum([f64::MAX, f64::MAX]),
            Err(SumError::NonRepresentable)
        );
    }
    #[test]
    fn normal_subnormal_boundary_and_nonfinite_rejection() {
        assert_eq!(
            exact_sum([f64::MIN_POSITIVE, -f64::from_bits(1)])
                .unwrap()
                .to_bits(),
            (1u64 << 52) - 1
        );
        assert_eq!(
            exact_sum([f64::from_bits(1), f64::from_bits(1)])
                .unwrap()
                .to_bits(),
            2
        );
        assert_eq!(exact_sum([f64::NAN]), Err(SumError::NonFinite));
        assert_eq!(exact_sum([f64::INFINITY]), Err(SumError::NonFinite));
    }
}
