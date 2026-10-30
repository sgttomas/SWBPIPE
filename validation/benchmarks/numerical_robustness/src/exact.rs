//! Exact decisions for VP-ROBUST (T3 D1 §4.10): the unchanged predicate
//! `|obs − exp| ≤ 1e-9·max(|exp|, scale)`, the magnitude form, the floor
//! comparison and the binary64-range test, all decided on exact rationals.
//!
//! - An observation is a binary64 value, an exact dyadic rational.
//! - An expected value, a scale and a control value are R1's decimal strings,
//!   exact decimal rationals. `1e-9` is the exact decimal 10⁻⁹.
//! - Every value is `±m·2^a·10^b` with an unsigned big integer `m`; two values
//!   are compared after scaling both by one common `2^A·10^B` to integers.
//!   Nothing is rounded, so no decision depends on a binary64 rounding of a
//!   reference value (plan §6.1).
//! - Standard library only: `Nat` is a little-endian `u32`-limb integer with
//!   the few operations the comparisons need.
use std::cmp::Ordering;

/// An unsigned integer, little-endian `u32` limbs, no high zero limb.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nat {
    limbs: Vec<u32>,
}

impl Nat {
    pub fn zero() -> Self {
        Self { limbs: Vec::new() }
    }

    pub fn from_u64(v: u64) -> Self {
        let mut n = Self {
            limbs: vec![v as u32, (v >> 32) as u32],
        };
        n.trim();
        n
    }

    pub fn from_u128(v: u128) -> Self {
        let mut n = Self {
            limbs: (0..4).map(|k| (v >> (32 * k)) as u32).collect(),
        };
        n.trim();
        n
    }

    /// The value as a u128, if it fits (tests).
    pub fn to_u128(&self) -> Option<u128> {
        if self.limbs.len() > 4 {
            return None;
        }
        Some(
            self.limbs
                .iter()
                .enumerate()
                .fold(0u128, |acc, (k, &l)| acc | (u128::from(l) << (32 * k))),
        )
    }

    fn trim(&mut self) {
        while self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }

    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    /// Parses a nonempty string of ASCII decimal digits.
    pub fn from_decimal(digits: &str) -> Option<Self> {
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let mut n = Self::zero();
        let bytes = digits.as_bytes();
        let head = bytes.len() % 9;
        let mut chunks: Vec<&[u8]> = Vec::new();
        if head > 0 {
            chunks.push(&bytes[..head]);
        }
        chunks.extend(bytes[head..].chunks(9));
        for chunk in chunks {
            let mut v = 0u32;
            let mut scale = 1u32;
            for &b in chunk {
                v = v * 10 + u32::from(b - b'0');
                scale *= 10;
            }
            n.mul_small(scale);
            n.add_small(v);
        }
        Some(n)
    }

    fn mul_small(&mut self, m: u32) {
        let mut carry = 0u64;
        for l in &mut self.limbs {
            let t = u64::from(*l) * u64::from(m) + carry;
            *l = t as u32;
            carry = t >> 32;
        }
        if carry > 0 {
            self.limbs.push(carry as u32);
        }
        self.trim();
    }

    fn add_small(&mut self, a: u32) {
        let mut carry = u64::from(a);
        for l in &mut self.limbs {
            if carry == 0 {
                break;
            }
            let t = u64::from(*l) + carry;
            *l = t as u32;
            carry = t >> 32;
        }
        if carry > 0 {
            self.limbs.push(carry as u32);
        }
    }

    /// self·2^k.
    pub fn shl(&self, k: u64) -> Self {
        if self.is_zero() {
            return Self::zero();
        }
        let words = (k / 32) as usize;
        let bits = (k % 32) as u32;
        let mut limbs = vec![0u32; words];
        if bits == 0 {
            limbs.extend_from_slice(&self.limbs);
        } else {
            let mut carry = 0u32;
            for &l in &self.limbs {
                limbs.push((l << bits) | carry);
                carry = l >> (32 - bits);
            }
            if carry > 0 {
                limbs.push(carry);
            }
        }
        let mut n = Self { limbs };
        n.trim();
        n
    }

    /// self·10^k.
    pub fn mul_pow10(&self, mut k: u64) -> Self {
        let mut n = self.clone();
        while k >= 9 {
            n.mul_small(1_000_000_000);
            k -= 9;
        }
        if k > 0 {
            n.mul_small(10u32.pow(k as u32));
        }
        n
    }

    pub fn add(&self, o: &Self) -> Self {
        let len = self.limbs.len().max(o.limbs.len());
        let mut limbs = Vec::with_capacity(len + 1);
        let mut carry = 0u64;
        for k in 0..len {
            let t = u64::from(*self.limbs.get(k).unwrap_or(&0))
                + u64::from(*o.limbs.get(k).unwrap_or(&0))
                + carry;
            limbs.push(t as u32);
            carry = t >> 32;
        }
        if carry > 0 {
            limbs.push(carry as u32);
        }
        let mut n = Self { limbs };
        n.trim();
        n
    }

    /// self − o; requires self ≥ o.
    pub fn sub(&self, o: &Self) -> Self {
        assert!(self.cmp_nat(o) != Ordering::Less, "Nat::sub underflow");
        let mut limbs = Vec::with_capacity(self.limbs.len());
        let mut borrow = 0i64;
        for k in 0..self.limbs.len() {
            let mut t =
                i64::from(self.limbs[k]) - i64::from(*o.limbs.get(k).unwrap_or(&0)) - borrow;
            if t < 0 {
                t += 1 << 32;
                borrow = 1;
            } else {
                borrow = 0;
            }
            limbs.push(t as u32);
        }
        let mut n = Self { limbs };
        n.trim();
        n
    }

    pub fn mul(&self, o: &Self) -> Self {
        if self.is_zero() || o.is_zero() {
            return Self::zero();
        }
        let mut acc = vec![0u64; self.limbs.len() + o.limbs.len() + 1];
        for (i, &a) in self.limbs.iter().enumerate() {
            let mut carry = 0u64;
            for (j, &b) in o.limbs.iter().enumerate() {
                let t = acc[i + j] + u64::from(a) * u64::from(b) + carry;
                acc[i + j] = t & 0xffff_ffff;
                carry = t >> 32;
            }
            let mut k = i + o.limbs.len();
            while carry > 0 {
                let t = acc[k] + carry;
                acc[k] = t & 0xffff_ffff;
                carry = t >> 32;
                k += 1;
            }
        }
        let mut n = Self {
            limbs: acc.into_iter().map(|v| v as u32).collect(),
        };
        n.trim();
        n
    }

    pub fn cmp_nat(&self, o: &Self) -> Ordering {
        self.limbs
            .len()
            .cmp(&o.limbs.len())
            .then_with(|| self.limbs.iter().rev().cmp(o.limbs.iter().rev()))
    }
}

/// An exact rational `±mant·2^p2·10^p10`. Zero has `mant = 0` and no sign.
#[derive(Clone, Debug)]
pub struct Exact {
    negative: bool,
    mant: Nat,
    p2: i64,
    p10: i64,
}

impl Exact {
    pub fn zero() -> Self {
        Self {
            negative: false,
            mant: Nat::zero(),
            p2: 0,
            p10: 0,
        }
    }

    fn new(negative: bool, mant: Nat, p2: i64, p10: i64) -> Self {
        let negative = negative && !mant.is_zero();
        Self {
            negative,
            mant,
            p2,
            p10,
        }
    }

    /// A decimal string: an optional sign, digits with an optional point, and an
    /// optional exponent (`e`/`E`, optional sign). R1's `0`, `0.00000001`,
    /// `1e+80` and `-3.2913…e-2864` all parse exactly.
    pub fn parse_decimal(text: &str) -> Result<Self, String> {
        let err = || format!("not a decimal: {text:?}");
        let (negative, body) = match text.as_bytes().first() {
            Some(b'-') => (true, &text[1..]),
            Some(b'+') => (false, &text[1..]),
            _ => (false, text),
        };
        let (mantissa, exp10) = match body.find(['e', 'E']) {
            Some(k) => (&body[..k], body[k + 1..].parse::<i64>().map_err(|_| err())?),
            None => (body, 0),
        };
        let (int, frac) = match mantissa.find('.') {
            Some(k) => (&mantissa[..k], &mantissa[k + 1..]),
            None => (mantissa, ""),
        };
        if int.is_empty() && frac.is_empty() {
            return Err(err());
        }
        let digits = format!("{int}{frac}");
        let mant = Nat::from_decimal(&digits).ok_or_else(err)?;
        Ok(Self::new(negative, mant, 0, exp10 - frac.len() as i64))
    }

    /// A finite binary64 value, exactly (±0 is zero).
    pub fn from_f64(x: f64) -> Self {
        assert!(x.is_finite(), "Exact::from_f64 of a non-finite value");
        let bits = x.to_bits();
        let biased = ((bits >> 52) & 0x7ff) as i64;
        let fraction = bits & ((1u64 << 52) - 1);
        let (m, e) = if biased == 0 {
            (fraction, -1074)
        } else {
            (fraction | (1u64 << 52), biased - 1075)
        };
        Self::new(x.is_sign_negative(), Nat::from_u64(m), e, 0)
    }

    /// 2^k exactly.
    pub fn pow2(k: i64) -> Self {
        Self::new(false, Nat::from_u64(1), k, 0)
    }

    pub fn is_zero(&self) -> bool {
        self.mant.is_zero()
    }

    pub fn is_negative(&self) -> bool {
        self.negative
    }

    pub fn abs(&self) -> Self {
        Self::new(false, self.mant.clone(), self.p2, self.p10)
    }

    pub fn negated(&self) -> Self {
        Self::new(!self.negative, self.mant.clone(), self.p2, self.p10)
    }

    /// self·2^k·10^j exactly.
    pub fn scaled(&self, k: i64, j: i64) -> Self {
        Self::new(self.negative, self.mant.clone(), self.p2 + k, self.p10 + j)
    }

    /// Both operands as signed integers over one common factor 2^A·10^B.
    fn align(a: &Self, b: &Self) -> ((bool, Nat), (bool, Nat), i64, i64) {
        let p2 = if a.is_zero() {
            b.p2
        } else if b.is_zero() {
            a.p2
        } else {
            a.p2.min(b.p2)
        };
        let p10 = if a.is_zero() {
            b.p10
        } else if b.is_zero() {
            a.p10
        } else {
            a.p10.min(b.p10)
        };
        let lift = |x: &Self| -> (bool, Nat) {
            if x.is_zero() {
                return (false, Nat::zero());
            }
            let m = x
                .mant
                .shl((x.p2 - p2) as u64)
                .mul_pow10((x.p10 - p10) as u64);
            (x.negative, m)
        };
        (lift(a), lift(b), p2, p10)
    }

    pub fn cmp(&self, o: &Self) -> Ordering {
        let ((na, a), (nb, b), _, _) = Self::align(self, o);
        match (na, nb) {
            (false, true) => {
                if a.is_zero() && b.is_zero() {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }
            (true, false) => {
                if a.is_zero() && b.is_zero() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
            (false, false) => a.cmp_nat(&b),
            (true, true) => b.cmp_nat(&a),
        }
    }

    pub fn add(&self, o: &Self) -> Self {
        let ((na, a), (nb, b), p2, p10) = Self::align(self, o);
        if na == nb {
            return Self::new(na, a.add(&b), p2, p10);
        }
        match a.cmp_nat(&b) {
            Ordering::Less => Self::new(nb, b.sub(&a), p2, p10),
            _ => Self::new(na, a.sub(&b), p2, p10),
        }
    }

    pub fn sub(&self, o: &Self) -> Self {
        self.add(&o.negated())
    }

    pub fn mul(&self, o: &Self) -> Self {
        Self::new(
            self.negative != o.negative,
            self.mant.mul(&o.mant),
            self.p2 + o.p2,
            self.p10 + o.p10,
        )
    }

    pub fn max(a: &Self, b: &Self) -> Self {
        if a.cmp(b) == Ordering::Less {
            b.clone()
        } else {
            a.clone()
        }
    }

    /// A binary64 approximation for display only (never used in a decision).
    pub fn approx(&self) -> f64 {
        if self.is_zero() {
            return 0.0;
        }
        let digits = decimal_digits(&self.mant);
        let text = format!(
            "{}{}e{}",
            if self.negative { "-" } else { "" },
            digits,
            self.p10
        );
        let mut v: f64 = text.parse().unwrap_or(0.0);
        // Exact power-of-two steps (display only).
        let mut k = self.p2;
        while k > 0 {
            let s = k.min(512);
            v *= f64::from_bits(((1023 + s) as u64) << 52);
            k -= s;
        }
        while k < 0 {
            let s = (-k).min(512);
            v *= f64::from_bits(((1023 - s) as u64) << 52);
            k += s;
        }
        v
    }
}

fn decimal_digits(n: &Nat) -> String {
    if n.is_zero() {
        return "0".into();
    }
    let mut limbs = n.limbs.clone();
    let mut parts = Vec::new();
    while !limbs.is_empty() {
        let mut rem = 0u64;
        for l in limbs.iter_mut().rev() {
            let t = (rem << 32) | u64::from(*l);
            *l = (t / 1_000_000_000) as u32;
            rem = t % 1_000_000_000;
        }
        while limbs.last() == Some(&0) {
            limbs.pop();
        }
        parts.push(rem);
    }
    let mut out = parts.pop().unwrap().to_string();
    for p in parts.iter().rev() {
        out.push_str(&format!("{p:09}"));
    }
    out
}

/// 10⁻⁹·max(|exp|, scale).
pub fn tolerance(exp: &Exact, scale: &Exact) -> Exact {
    Exact::max(&exp.abs(), scale).scaled(0, -9)
}

/// The unchanged predicate `|obs − exp| ≤ 1e-9·max(|exp|, scale)`, exactly.
pub fn predicate(obs: &Exact, exp: &Exact, scale: &Exact) -> bool {
    obs.sub(exp).abs().cmp(&tolerance(exp, scale)) != Ordering::Greater
}

/// The predicate on a magnitude `√(my² + mz²)`, exactly, without forming the
/// square root (plan §6.2): with t = 10⁻⁹·max(|exp|, scale), it holds iff
/// `my² + mz² ≤ (exp + t)²` (and exp + t ≥ 0) and, where exp − t > 0,
/// `my² + mz² ≥ (exp − t)²`.
pub fn magnitude_predicate(my: f64, mz: f64, exp: &Exact, scale: &Exact) -> bool {
    let (y, z) = (Exact::from_f64(my), Exact::from_f64(mz));
    let s = y.mul(&y).add(&z.mul(&z));
    let t = tolerance(exp, scale);
    let upper = exp.add(&t);
    if upper.is_negative() || s.cmp(&upper.mul(&upper)) == Ordering::Greater {
        return false;
    }
    let lower = exp.sub(&t);
    if lower.is_negative() || lower.is_zero() {
        return true;
    }
    s.cmp(&lower.mul(&lower)) != Ordering::Less
}

/// True when a nonzero exp has no binary64 value: its rounding to nearest
/// (ties to even) is ±0 (|exp| ≤ 2^-1075) or ±∞ (|exp| ≥ 2^1024 − 2^970).
pub fn outside_binary64(exp: &Exact) -> bool {
    if exp.is_zero() {
        return false;
    }
    let a = exp.abs();
    let tiny = Exact::pow2(-1075);
    let huge = Exact::pow2(1024).sub(&Exact::pow2(970));
    a.cmp(&tiny) != Ordering::Greater || a.cmp(&huge) != Ordering::Less
}

/// The floor comparison (D1 §4.10): the comparison scale max(|exp|, scale) is
/// at least R·S*, R = 2^-34, decided exactly (no rounding of R·S*).
pub fn covered(exp: &Exact, scale: &Exact, s_star: f64) -> bool {
    let comparison = Exact::max(&exp.abs(), scale);
    let floor = Exact::from_f64(s_star).scaled(-34, 0);
    comparison.cmp(&floor) != Ordering::Less
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A seeded 64-bit generator (SplitMix64) for the differential tests.
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }
    }

    #[test]
    fn nat_arithmetic_equals_u128_arithmetic() {
        let mut rng = Rng(0x5649_4B31_375F_4E41);
        for _ in 0..20_000 {
            let a = u128::from(rng.next() >> (rng.next() % 64));
            let b = u128::from(rng.next() >> (rng.next() % 64));
            let (na, nb) = (Nat::from_u128(a), Nat::from_u128(b));
            assert_eq!(na.add(&nb).to_u128(), Some(a + b));
            assert_eq!(na.mul(&nb).to_u128(), Some(a * b));
            assert_eq!(na.cmp_nat(&nb), a.cmp(&b));
            if a >= b {
                assert_eq!(na.sub(&nb).to_u128(), Some(a - b));
            }
            let k = rng.next() % 60;
            assert_eq!(na.shl(k).to_u128(), (a < 1 << 64).then(|| a << k));
            let j = rng.next() % 19;
            if a < 1 << 64 {
                assert_eq!(na.mul_pow10(j).to_u128(), Some(a * 10u128.pow(j as u32)));
            }
            assert_eq!(
                Nat::from_decimal(&a.to_string()).unwrap().to_u128(),
                Some(a)
            );
            assert_eq!(decimal_digits(&na), a.to_string());
        }
    }

    #[test]
    fn decimals_and_binary64_values_parse_exactly() {
        let one = Exact::from_f64(1.0);
        for text in ["1", "1.0", "1e0", "10e-1", "0.1e1", "+1", "1.000e+0"] {
            assert_eq!(
                Exact::parse_decimal(text).unwrap().cmp(&one),
                Ordering::Equal,
                "{text}"
            );
        }
        let z = Exact::parse_decimal("0").unwrap();
        assert!(z.is_zero() && !z.is_negative());
        assert!(Exact::parse_decimal("-0").unwrap().is_zero());
        assert!(Exact::from_f64(-0.0).is_zero());
        // 0.1 is not binary64: the decimal and fl(0.1) differ, in the known direction.
        let tenth = Exact::parse_decimal("0.1").unwrap();
        assert_eq!(Exact::from_f64(0.1).cmp(&tenth), Ordering::Greater);
        assert_eq!(
            Exact::parse_decimal("1e+80").unwrap().cmp(&Exact::parse_decimal("100000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap()),
            Ordering::Equal
        );
        // The smallest subnormal and 2^-1075.
        assert_eq!(
            Exact::from_f64(f64::from_bits(1)).cmp(&Exact::pow2(-1074)),
            Ordering::Equal
        );
        for bad in ["", "e5", "1..0", "1e", "--1", "0x10"] {
            assert!(Exact::parse_decimal(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn the_predicate_holds_at_its_boundary_and_not_one_ulp_beyond() {
        // exp = 0, scale = 1e9: t = 1 exactly.
        let (exp, scale) = (Exact::zero(), Exact::parse_decimal("1e9").unwrap());
        for (obs, ok) in [
            (1.0, true),
            (-1.0, true),
            (f64::from_bits(1.0f64.to_bits() + 1), false),
            (f64::from_bits(1.0f64.to_bits() - 1), true),
            (-f64::from_bits(1.0f64.to_bits() + 1), false),
        ] {
            assert_eq!(
                predicate(&Exact::from_f64(obs), &exp, &scale),
                ok,
                "{obs:e}"
            );
        }
        // exp = 2^30, scale 0: t = 2^30·10^-9 = 1.073741824, not dyadic.
        // |Δ| = 1.0625 passes, 1.078125 fails.
        let exp = Exact::from_f64(1073741824.0);
        let zero = Exact::zero();
        assert!(predicate(&Exact::from_f64(1073741825.0625), &exp, &zero));
        assert!(!predicate(&Exact::from_f64(1073741825.078125), &exp, &zero));
        assert!(predicate(&Exact::from_f64(1073741822.9375), &exp, &zero));
        assert!(!predicate(&Exact::from_f64(1073741822.921875), &exp, &zero));
        // max(|exp|, scale) uses |exp|, never |obs|.
        let exp = Exact::parse_decimal("-2e-3").unwrap();
        assert!(predicate(&Exact::from_f64(-0.002), &exp, &zero));
        assert!(!predicate(&Exact::from_f64(0.002), &exp, &zero));
    }

    #[test]
    fn the_magnitude_predicate_holds_at_both_bounds_and_not_one_ulp_beyond() {
        // exp = 5, scale = 3e9: t = 3, so the magnitude must lie in [2, 8].
        let (exp, scale) = (Exact::from_f64(5.0), Exact::parse_decimal("3e9").unwrap());
        let up = |x: f64| f64::from_bits(x.to_bits() + 1);
        let down = |x: f64| f64::from_bits(x.to_bits() - 1);
        for (my, mz, ok) in [
            (0.0, 8.0, true),
            (0.0, up(8.0), false),
            (0.0, 2.0, true),
            (0.0, down(2.0), false),
            (4.8, 6.4, false), // fl(4.8), fl(6.4): the magnitude lies just above 8
            (3.0, 4.0, true),
            (6.0, 8.0, false),
            (0.0, 0.0, false),
        ] {
            assert_eq!(magnitude_predicate(my, mz, &exp, &scale), ok, "{my} {mz}");
        }
        // exp − t ≤ 0: any magnitude up to exp + t passes, 0 included.
        let scale = Exact::parse_decimal("1e10").unwrap();
        assert!(magnitude_predicate(0.0, 0.0, &exp, &scale));
    }

    #[test]
    fn the_binary64_range_is_decided_exactly() {
        let half_min = Exact::pow2(-1075);
        assert!(outside_binary64(&half_min));
        assert!(outside_binary64(&half_min.negated()));
        let just_above = Exact::pow2(-1075).add(&Exact::pow2(-1200));
        assert!(!outside_binary64(&just_above));
        assert!(!outside_binary64(&Exact::from_f64(f64::MAX)));
        let tie = Exact::pow2(1024).sub(&Exact::pow2(970));
        assert!(outside_binary64(&tie));
        assert!(!outside_binary64(&tie.sub(&Exact::pow2(900))));
        assert!(!outside_binary64(&Exact::zero()));
        assert!(outside_binary64(
            &Exact::parse_decimal("6.3976764473949915709056557253e-714").unwrap()
        ));
    }

    #[test]
    fn the_floor_is_r_times_s_star_exactly() {
        // R·S* = 2^-34 exactly at S* = 1; a scale of 2^-34 is covered, one ulp less is not.
        let r = Exact::pow2(-34);
        assert!(covered(&Exact::zero(), &r, 1.0));
        assert!(!covered(
            &Exact::zero(),
            &Exact::from_f64(f64::from_bits(0x3DD0_0000_0000_0000 - 1)),
            1.0
        ));
        // A subnormal R·S* is not rounded: S* = 2^-1000.
        let s = f64::from_bits(0x0170_0000_0000_0000); // 2^-1000
        assert!(covered(&Exact::pow2(-1034), &Exact::zero(), s));
        assert!(!covered(&Exact::pow2(-1035), &Exact::zero(), s));
        // S* = 0: everything is covered.
        assert!(covered(&Exact::zero(), &Exact::zero(), 0.0));
    }
}
