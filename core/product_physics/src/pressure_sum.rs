//! Exact finite-binary64 sum followed by one nearest-even binary64 projection.
//!
//! S11 (section 4.1.1): this is a thin wrapper over the frame kernel's one
//! correctly rounded accumulator, `exact_sum::exact_rounded_sum` (quantum
//! 2^-2148, 68 limbs). Every finite operand is an integer multiple of 2^-1074,
//! so the result is the same nearest-even projection as before; an exact zero
//! is +0.0. This does not recover information rounded before the operands were
//! supplied.
use open_pipe_stress_frame_kernel::exact_sum;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SumError {
    NonFinite,
    AccumulatorOverflow,
    NonRepresentable,
}
pub(crate) fn exact_sum(values: impl IntoIterator<Item = f64>) -> Result<f64, SumError> {
    exact_sum::exact_rounded_sum(values).map_err(|error| match error {
        exact_sum::SumError::NonFinite => SumError::NonFinite,
        exact_sum::SumError::AccumulatorOverflow => SumError::AccumulatorOverflow,
        exact_sum::SumError::NonRepresentable => SumError::NonRepresentable,
    })
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
