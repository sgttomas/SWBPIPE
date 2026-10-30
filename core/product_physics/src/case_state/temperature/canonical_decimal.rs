//! The canonical decimal of an authored binary64 and its exact kelvin value in
//! integer form, as DEL-02-01 Design `model-document-0.9.0.md` §1.5.1 defines
//! them. This is the one implementation: the temperature identity uses it, and
//! C1's lowering reuses it. No step uses a float formatter: Rust's `{:e}` and
//! Python's `repr` disagree on 16- and 17-digit ties, and §1.5.1 takes the
//! closest shortest round-trip decimal with exact ties to the even digit.
//!
//! The domain is |v| <= 10^6 in the authored unit with a canonical decimal
//! exponent e >= -24; anything else is refused as out of range. Within it every
//! kelvin value is N / D with D = 9·10^24 and |N| < 10^31, so equality and order
//! are integer comparisons that cannot overflow.
use open_pipe_stress_units::UnitId;
use std::cmp::Ordering;

/// D of the integer form: every admitted kelvin value is N / D.
pub(crate) const KELVIN_DENOMINATOR: i128 = 9 * 10i128.pow(24);

/// ±`digits`·10^`exponent`, with `digits` not divisible by 10; zero is `0`·10^0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalDecimal {
    pub(crate) negative: bool,
    pub(crate) digits: u64,
    pub(crate) exponent: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CanonicalError {
    NonFinite,
    /// Outside |v| <= 10^6 with e >= -24 (or the 2^-80..2^20 pre-gate).
    RangeUnsupported,
}

/// Unsigned 256-bit integer on the stack, little-endian limbs. §1.5.1 bounds
/// every intermediate below 2^192, so no operation here can overflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct U256([u64; 4]);

impl U256 {
    fn from_u128(value: u128) -> Self {
        Self([value as u64, (value >> 64) as u64, 0, 0])
    }
    fn mul_u64(self, factor: u64) -> Self {
        let mut out = [0u64; 4];
        let mut carry = 0u128;
        for (limb, slot) in self.0.iter().zip(out.iter_mut()) {
            let wide = *limb as u128 * factor as u128 + carry;
            *slot = wide as u64;
            carry = wide >> 64;
        }
        debug_assert_eq!(carry, 0);
        Self(out)
    }
    fn mul_pow10(self, power: u32) -> Self {
        (0..power).fold(self, |acc, _| acc.mul_u64(10))
    }
    fn shl(self, bits: u32) -> Self {
        let (words, bits) = ((bits / 64) as usize, bits % 64);
        let mut out = [0u64; 4];
        for i in (words..4).rev() {
            let low = self.0[i - words];
            let carry = if bits > 0 && i > words {
                self.0[i - words - 1] >> (64 - bits)
            } else {
                0
            };
            out[i] = (low << bits) | carry;
        }
        Self(out)
    }
    fn shr(self, bits: u32) -> Self {
        let (words, bits) = ((bits / 64) as usize, bits % 64);
        let mut out = [0u64; 4];
        for i in 0..4usize.saturating_sub(words) {
            let high = self.0[i + words];
            let carry = if bits > 0 && i + words + 1 < 4 {
                self.0[i + words + 1] << (64 - bits)
            } else {
                0
            };
            out[i] = (high >> bits) | carry;
        }
        Self(out)
    }
    fn add(self, other: Self) -> Self {
        let mut out = [0u64; 4];
        let mut carry = false;
        for i in 0..4 {
            let (sum, c1) = self.0[i].overflowing_add(other.0[i]);
            let (sum, c2) = sum.overflowing_add(carry as u64);
            out[i] = sum;
            carry = c1 || c2;
        }
        Self(out)
    }
    /// `self - other`; callers only subtract a smaller value.
    fn sub(self, other: Self) -> Self {
        let mut out = [0u64; 4];
        let mut borrow = false;
        for i in 0..4 {
            let (diff, b1) = self.0[i].overflowing_sub(other.0[i]);
            let (diff, b2) = diff.overflowing_sub(borrow as u64);
            out[i] = diff;
            borrow = b1 || b2;
        }
        Self(out)
    }
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().rev().cmp(other.0.iter().rev())
    }
}

/// The canonical decimal of a finite binary64 inside the domain (§1.5.1
/// steps 1 to 5): the shortest decimal that parses back to `value`, the one
/// closer to it when two of that length do, and the even one on an exact tie.
pub(crate) fn canonical_decimal(value: f64) -> Result<CanonicalDecimal, CanonicalError> {
    if !value.is_finite() {
        return Err(CanonicalError::NonFinite);
    }
    if value == 0.0 {
        return Ok(CanonicalDecimal {
            negative: false,
            digits: 0,
            exponent: 0,
        });
    }
    // Step 1: the domain's |v| <= 10^6 (exact in binary64) inside the pre-gate's
    // 2^20, and the pre-gate's |v| >= 2^-80 (biased exponent 943).
    let bits = value.abs().to_bits();
    if value.abs() > 1e6 || bits >> 52 < 943 {
        return Err(CanonicalError::RangeUnsupported);
    }
    let b = (bits & ((1u64 << 52) - 1)) | (1u64 << 52);
    // -E, from 32 to 132.
    let shift = 1075 - ((bits >> 52) as u32);
    // Step 2: the largest p with 10^p <= |v|, i.e. 10^max(0,p)·2^-E <= B·10^max(0,-p).
    let p = (-25..=6)
        .rev()
        .find(|&p: &i32| {
            let left = U256::from_u128(1).mul_pow10(p.max(0) as u32).shl(shift);
            let right = U256::from_u128(b as u128).mul_pow10((-p).max(0) as u32);
            left.cmp(&right) != Ordering::Greater
        })
        .ok_or(CanonicalError::RangeUnsupported)?;
    // Steps 3 and 4, on integers scaled by S = 2^(2-E)·10^max(0,-q).
    let even = b % 2 == 0;
    for n in 1..=17 {
        let q = p - n + 1;
        let fine = (-q).max(0) as u32;
        let coarse = q.max(0) as u32;
        let d0 = if q <= 0 {
            U256::from_u128(b as u128).mul_pow10(fine).shr(shift).0[0]
        } else {
            (b >> shift) / 10u64.pow(coarse)
        };
        let unit = U256::from_u128(1).mul_pow10(fine);
        let scaled = U256::from_u128(4 * b as u128).mul_pow10(fine);
        let upper = scaled.add(unit.mul_u64(2));
        let lower = scaled.sub(if b == 1u64 << 52 {
            unit
        } else {
            unit.mul_u64(2)
        });
        let at = |d: u64| U256::from_u128(d as u128).mul_pow10(coarse).shl(2 + shift);
        let round_trips = |x: U256| match (x.cmp(&lower), x.cmp(&upper)) {
            (Ordering::Greater, Ordering::Less) => true,
            (Ordering::Equal, _) | (_, Ordering::Equal) => even,
            _ => false,
        };
        let (x0, x1) = (at(d0), at(d0 + 1));
        let digits = match (round_trips(x0), round_trips(x1)) {
            (false, false) => continue,
            (true, false) => d0,
            (false, true) => d0 + 1,
            (true, true) => match scaled.sub(x0).cmp(&x1.sub(scaled)) {
                Ordering::Less => d0,
                Ordering::Greater => d0 + 1,
                Ordering::Equal if d0 % 2 == 0 => d0,
                Ordering::Equal => d0 + 1,
            },
        };
        // Step 5: strip trailing zeros, restore the sign, apply the domain's e >= -24.
        let (mut digits, mut exponent) = (digits, q);
        while digits % 10 == 0 {
            digits /= 10;
            exponent += 1;
        }
        if exponent < -24 {
            return Err(CanonicalError::RangeUnsupported);
        }
        return Ok(CanonicalDecimal {
            negative: value < 0.0,
            digits,
            exponent,
        });
    }
    // Seventeen significant digits always round-trip a binary64.
    unreachable!("no round-tripping decimal within 17 digits")
}

/// N of the exact kelvin value N / `KELVIN_DENOMINATOR` of a canonical decimal
/// in an absolute-temperature unit's exact affine definition; `None` for a unit
/// without one.
pub(crate) fn kelvin_numerator(decimal: CanonicalDecimal, unit: UnitId) -> Option<i128> {
    // M = m·10^(e+24); the domain keeps e + 24 >= 0 and |M| <= 10^30.
    let magnitude = decimal.digits as i128 * 10i128.pow((decimal.exponent + 24) as u32);
    let m = if decimal.negative {
        -magnitude
    } else {
        magnitude
    };
    let offset = 10i128.pow(22);
    match unit {
        UnitId::Kelvin => Some(9 * m),
        UnitId::DegreeCelsius => Some(9 * (m + 27315 * offset)),
        UnitId::DegreeFahrenheit => Some(5 * (m + 45967 * offset)),
        UnitId::DegreeRankine => Some(5 * m),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// (negative, digits, exponent) of a decimal text, trailing zeros stripped.
    fn parse(text: &str) -> CanonicalDecimal {
        let (negative, text) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        let (mantissa, exponent) = text.split_once('e').unwrap_or((text, "0"));
        let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let mut digits: u64 = format!("{whole}{fraction}").parse().unwrap();
        let mut exponent = exponent.parse::<i32>().unwrap() - fraction.len() as i32;
        if digits == 0 {
            return CanonicalDecimal {
                negative: false,
                digits: 0,
                exponent: 0,
            };
        }
        while digits % 10 == 0 {
            digits /= 10;
            exponent += 1;
        }
        CanonicalDecimal {
            negative,
            digits,
            exponent,
        }
    }

    #[test]
    fn w17_ties_and_the_power_of_two_lower_end_take_the_canonical_decimal() {
        // Rust's `{:e}` prints ...813e-3, ...1563e-3 and ...7313e-8 for the first three.
        for (bits, canonical) in [
            (0x3f80010000000000u64, "0.007814407348632812"),
            (0x3f50640000000000, "0.0010004043579101562"),
            (0x3e60000000000000, "2.9802322387695312e-08"),
            // 2^-24: B = 2^52, so the lower half gap is half the upper.
            (0x3e70000000000000, "5.960464477539063e-08"),
        ] {
            let value = f64::from_bits(bits);
            assert_eq!(
                canonical_decimal(value),
                Ok(parse(canonical)),
                "{bits:016x}"
            );
            assert_eq!(
                canonical_decimal(-value),
                Ok(CanonicalDecimal {
                    negative: true,
                    ..parse(canonical)
                })
            );
        }
    }

    #[test]
    fn the_domain_is_refused_by_name_at_both_edges() {
        let range = Err(CanonicalError::RangeUnsupported);
        let above = f64::from_bits(1e6f64.to_bits() + 1);
        assert_eq!(canonical_decimal(1e6), Ok(parse("1e6")));
        assert_eq!(canonical_decimal(-1e6), Ok(parse("-1e6")));
        assert_eq!(canonical_decimal(above), range);
        assert_eq!(canonical_decimal(-above), range);
        assert_eq!(canonical_decimal(2f64.powi(20)), range);
        assert_eq!(canonical_decimal(1e300), range);
        // e = -24 is inside; e = -25 is outside, at any magnitude.
        assert_eq!(canonical_decimal(1e-24), Ok(parse("1e-24")));
        assert_eq!(canonical_decimal(9.99e-22), Ok(parse("9.99e-22")));
        assert_eq!(canonical_decimal(1e-25), range);
        assert_eq!(canonical_decimal(1.5e-24), range);
        assert_eq!(
            canonical_decimal(1.23456789012345e-10),
            Ok(parse("1.23456789012345e-10"))
        );
        assert_eq!(canonical_decimal(1.234567890123456e-10), range);
        assert_eq!(canonical_decimal(2f64.powi(-80)), range);
        assert_eq!(canonical_decimal(f64::MIN_POSITIVE), range);
        assert_eq!(canonical_decimal(f64::NAN), Err(CanonicalError::NonFinite));
        assert_eq!(
            canonical_decimal(f64::INFINITY),
            Err(CanonicalError::NonFinite)
        );
        assert_eq!(canonical_decimal(-0.0), Ok(parse("0")));
    }

    #[test]
    fn integer_form_follows_each_unit_affine_definition() {
        let n = |text: &str, unit| kelvin_numerator(parse(text), unit).unwrap();
        let d = KELVIN_DENOMINATOR;
        assert_eq!(n("293.15", UnitId::Kelvin), 29315 * d / 100);
        assert_eq!(n("20", UnitId::DegreeCelsius), 29315 * d / 100);
        assert_eq!(n("527.67", UnitId::DegreeRankine), 29315 * d / 100);
        assert_eq!(n("68", UnitId::DegreeFahrenheit), 29315 * d / 100);
        assert_eq!(n("1e-24", UnitId::Kelvin), 9);
        assert_eq!(
            n("-1e6", UnitId::DegreeFahrenheit),
            5 * (-10i128.pow(30) + 45967 * 10i128.pow(22))
        );
        assert_eq!(kelvin_numerator(parse("1"), UnitId::Meter), None);
    }

    /// Python `repr` of tie binaries and random binary64 values in the domain,
    /// written by `tools/fixtures/temperature_canonical_decimal_corpus.py`.
    #[test]
    fn corpus_matches_python_repr() {
        let corpus = include_str!(
            "../../../tests/fixtures/temperature_identity/canonical_decimal_repr_corpus.txt"
        );
        let mut rows = 0;
        for line in corpus.lines().filter(|line| !line.starts_with('#')) {
            let (bits, repr) = line.split_once(' ').unwrap();
            let value = f64::from_bits(u64::from_str_radix(bits, 16).unwrap());
            assert_eq!(canonical_decimal(value), Ok(parse(repr)), "{bits} {repr}");
            rows += 1;
        }
        assert!(rows >= 4000, "{rows}");
    }
}
