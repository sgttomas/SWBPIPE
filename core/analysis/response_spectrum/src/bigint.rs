//! Unsigned big integers for the exact combination arithmetic (unit S4).
//!
//! A crate-local, minimal implementation (no external dependency): little-endian u64 limbs, with
//! addition, subtraction, multiplication, shifts, comparison, division with remainder and the
//! integer square root, each exact. The algorithms are the schoolbook ones; their operands are
//! at most a few thousand bits.
use std::cmp::Ordering;

/// An unsigned integer. The limb vector has no trailing zero limbs; zero is the empty vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct BigUint {
    limbs: Vec<u64>,
}

impl BigUint {
    pub(crate) fn zero() -> Self {
        Self { limbs: Vec::new() }
    }

    pub(crate) fn from_limbs(limbs: &[u64]) -> Self {
        let mut n = Self { limbs: limbs.to_vec() };
        n.trim();
        n
    }

    pub(crate) fn from_u128(v: u128) -> Self {
        let mut n = Self { limbs: vec![v as u64, (v >> 64) as u64] };
        n.trim();
        n
    }

    fn trim(&mut self) {
        while self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    /// The number of significant bits; 0 for zero.
    pub(crate) fn bits(&self) -> u64 {
        match self.limbs.last() {
            None => 0,
            Some(&top) => (self.limbs.len() as u64 - 1) * 64 + (64 - u64::from(top.leading_zeros())),
        }
    }

    /// Bit `i` (0 is the least significant).
    pub(crate) fn bit(&self, i: u64) -> bool {
        let limb = (i / 64) as usize;
        limb < self.limbs.len() && (self.limbs[limb] >> (i % 64)) & 1 == 1
    }

    /// Whether any of the bits below `i` is set.
    pub(crate) fn any_below(&self, i: u64) -> bool {
        let full = (i / 64) as usize;
        if self.limbs.iter().take(full.min(self.limbs.len())).any(|&l| l != 0) {
            return true;
        }
        let rest = i % 64;
        full < self.limbs.len() && rest > 0 && self.limbs[full] & ((1u64 << rest) - 1) != 0
    }

    /// The number of trailing zero bits; 0 for zero.
    pub(crate) fn trailing_zeros(&self) -> u64 {
        for (i, &l) in self.limbs.iter().enumerate() {
            if l != 0 {
                return i as u64 * 64 + u64::from(l.trailing_zeros());
            }
        }
        0
    }

    /// The low 64 bits.
    pub(crate) fn low_u64(&self) -> u64 {
        self.limbs.first().copied().unwrap_or(0)
    }

    pub(crate) fn add(&self, other: &Self) -> Self {
        let (long, short) = if self.limbs.len() >= other.limbs.len() { (self, other) } else { (other, self) };
        let mut limbs = Vec::with_capacity(long.limbs.len() + 1);
        let mut carry = 0u64;
        for (i, &a) in long.limbs.iter().enumerate() {
            let b = short.limbs.get(i).copied().unwrap_or(0);
            let (s1, c1) = a.overflowing_add(b);
            let (s2, c2) = s1.overflowing_add(carry);
            limbs.push(s2);
            carry = u64::from(c1) + u64::from(c2);
        }
        if carry > 0 {
            limbs.push(carry);
        }
        let mut n = Self { limbs };
        n.trim();
        n
    }

    /// `self − other`, for `self ≥ other`.
    pub(crate) fn sub(&self, other: &Self) -> Self {
        debug_assert!(self.cmp(other) != Ordering::Less, "subtraction below zero");
        let mut limbs = Vec::with_capacity(self.limbs.len());
        let mut borrow = 0u64;
        for (i, &a) in self.limbs.iter().enumerate() {
            let b = other.limbs.get(i).copied().unwrap_or(0);
            let (d1, b1) = a.overflowing_sub(b);
            let (d2, b2) = d1.overflowing_sub(borrow);
            limbs.push(d2);
            borrow = u64::from(b1) + u64::from(b2);
        }
        let mut n = Self { limbs };
        n.trim();
        n
    }

    pub(crate) fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        let mut limbs = vec![0u64; self.limbs.len() + other.limbs.len()];
        for (i, &a) in self.limbs.iter().enumerate() {
            let mut carry = 0u128;
            for (j, &b) in other.limbs.iter().enumerate() {
                let t = u128::from(a) * u128::from(b) + u128::from(limbs[i + j]) + carry;
                limbs[i + j] = t as u64;
                carry = t >> 64;
            }
            let mut k = i + other.limbs.len();
            while carry > 0 {
                let t = u128::from(limbs[k]) + carry;
                limbs[k] = t as u64;
                carry = t >> 64;
                k += 1;
            }
        }
        let mut n = Self { limbs };
        n.trim();
        n
    }

    pub(crate) fn shl(&self, bits: u64) -> Self {
        if self.is_zero() {
            return Self::zero();
        }
        let whole = (bits / 64) as usize;
        let rest = bits % 64;
        let mut limbs = vec![0u64; whole];
        if rest == 0 {
            limbs.extend_from_slice(&self.limbs);
        } else {
            let mut carry = 0u64;
            for &l in &self.limbs {
                limbs.push((l << rest) | carry);
                carry = l >> (64 - rest);
            }
            limbs.push(carry);
        }
        let mut n = Self { limbs };
        n.trim();
        n
    }

    /// `⌊self / 2^bits⌋`.
    pub(crate) fn shr(&self, bits: u64) -> Self {
        let whole = (bits / 64) as usize;
        if whole >= self.limbs.len() {
            return Self::zero();
        }
        let rest = bits % 64;
        let src = &self.limbs[whole..];
        let mut limbs = Vec::with_capacity(src.len());
        for (i, &l) in src.iter().enumerate() {
            let high = if rest == 0 { 0 } else { src.get(i + 1).map_or(0, |&h| h << (64 - rest)) };
            limbs.push(if rest == 0 { l } else { (l >> rest) | high });
        }
        let mut n = Self { limbs };
        n.trim();
        n
    }

    /// `(⌊self / d⌋, self mod d)`, for `d > 0`: schoolbook long division on u64 limbs (Knuth,
    /// TAOCP vol. 2, §4.3.1, Algorithm D), with a single-limb fast path.
    pub(crate) fn divrem(&self, d: &Self) -> (Self, Self) {
        assert!(!d.is_zero(), "division by zero");
        if self.cmp(d) == Ordering::Less {
            return (Self::zero(), self.clone());
        }
        if d.limbs.len() == 1 {
            let dv = u128::from(d.limbs[0]);
            let mut q = vec![0u64; self.limbs.len()];
            let mut r = 0u128;
            for i in (0..self.limbs.len()).rev() {
                let cur = (r << 64) | u128::from(self.limbs[i]);
                q[i] = (cur / dv) as u64;
                r = cur % dv;
            }
            let mut q = Self { limbs: q };
            q.trim();
            return (q, Self::from_u128(r));
        }
        // Normalize so the divisor's top limb has its high bit set.
        let shift = u64::from(d.limbs.last().unwrap().leading_zeros());
        let v = d.shl(shift).limbs;
        let mut u = self.shl(shift).limbs;
        u.push(0);
        let n = v.len();
        let m = u.len() - n - 1;
        let mut q = vec![0u64; m + 1];
        let b = 1u128 << 64;
        for j in (0..=m).rev() {
            let top = (u128::from(u[j + n]) << 64) | u128::from(u[j + n - 1]);
            let mut qhat = top / u128::from(v[n - 1]);
            let mut rhat = top % u128::from(v[n - 1]);
            while qhat >= b || qhat * u128::from(v[n - 2]) > ((rhat << 64) | u128::from(u[j + n - 2])) {
                qhat -= 1;
                rhat += u128::from(v[n - 1]);
                if rhat >= b {
                    break;
                }
            }
            // u[j..j+n+1] -= qhat·v
            let mut borrow: i128 = 0;
            let mut carry: u128 = 0;
            for i in 0..n {
                let p = qhat * u128::from(v[i]) + carry;
                carry = p >> 64;
                let t = i128::from(u[i + j]) - i128::from(p as u64) + borrow;
                u[i + j] = t as u64;
                borrow = t >> 64;
            }
            let t = i128::from(u[j + n]) - carry as i128 + borrow;
            u[j + n] = t as u64;
            if t < 0 {
                // qhat was one too large: add v back.
                qhat -= 1;
                let mut c = 0u128;
                for i in 0..n {
                    let s2 = u128::from(u[i + j]) + u128::from(v[i]) + c;
                    u[i + j] = s2 as u64;
                    c = s2 >> 64;
                }
                u[j + n] = u[j + n].wrapping_add(c as u64);
            }
            q[j] = qhat as u64;
        }
        let mut q = Self { limbs: q };
        q.trim();
        let mut r = Self { limbs: u[..n].to_vec() };
        r.trim();
        (q, r.shr(shift))
    }

    pub(crate) fn one() -> Self {
        Self { limbs: vec![1] }
    }

    /// `⌊√self⌋`, by Newton's iteration from an overestimate.
    pub(crate) fn isqrt(&self) -> Self {
        if self.is_zero() {
            return Self::zero();
        }
        // An overestimate x0 ≥ √self: from the top 53 bits (n < (t + 1)·2^(2k) with t the top part),
        // √n < √(t + 1)·2^k, rounded up generously; then x ← ⌊(x + ⌊n/x⌋)/2⌋ decreases to ⌊√n⌋.
        let bits = self.bits();
        let k = bits.saturating_sub(106) / 2; // keep about 106 bits in the top part
        let t = self.shr(2 * k);
        let t_f = t.limbs.iter().rev().fold(0.0f64, |acc, &l| acc * 18446744073709551616.0 + l as f64);
        let estimate = ((t_f * (1.0 + 1e-12) + 4.0).sqrt() * (1.0 + 1e-12)).ceil() + 2.0;
        let mut x = if estimate.is_finite() && estimate < 1.8e19 {
            Self::from_u128(estimate as u128).shl(k)
        } else {
            Self::one().shl(bits.div_ceil(2))
        };
        // Guard: the iteration needs a start at or above ⌊√n⌋.
        if x.mul(&x) < *self {
            x = Self::one().shl(bits.div_ceil(2));
        }
        loop {
            let (q, _) = self.divrem(&x);
            let y = x.add(&q).shr(1);
            if y.cmp(&x) != Ordering::Less {
                return x;
            }
            x = y;
        }
    }
}

impl PartialOrd for BigUint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigUint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.limbs.len().cmp(&other.limbs.len()).then_with(|| {
            for (a, b) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {
                match a.cmp(b) {
                    Ordering::Equal => continue,
                    o => return o,
                }
            }
            Ordering::Equal
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(v: u128) -> BigUint {
        BigUint::from_u128(v)
    }

    #[test]
    fn arithmetic_agrees_with_u128() {
        let values = [0u128, 1, 2, 3, 0xffff_ffff_ffff_ffff, 1 << 64, (1 << 100) + 12345, u128::MAX / 3];
        for &a in &values {
            for &b in &values {
                if let Some(s) = a.checked_add(b) {
                    assert_eq!(n(a).add(&n(b)), n(s));
                }
                if a >= b {
                    assert_eq!(n(a).sub(&n(b)), n(a - b));
                }
                if let Some(p) = a.checked_mul(b) {
                    assert_eq!(n(a).mul(&n(b)), n(p));
                }
                if b > 0 {
                    let (q, r) = n(a).divrem(&n(b));
                    assert_eq!((q, r), (n(a / b), n(a % b)), "{a} / {b}");
                }
                // Multi-limb divisors: (a·2^200 + b) / (b·2^70 + 3) checked by q·d + r = n, r < d.
                let big = n(a).shl(200).add(&n(b));
                let d = n(b).shl(70).add(&n(3));
                let (q, r) = big.divrem(&d);
                assert_eq!(q.mul(&d).add(&r), big, "{a}, {b}");
                assert!(r < d);
                assert_eq!(n(a).cmp(&n(b)), a.cmp(&b));
            }
            for s in [0u64, 1, 63, 64, 65, 127] {
                if a.leading_zeros() as u64 >= s {
                    assert_eq!(n(a).shl(s), n(a << s));
                }
                assert_eq!(n(a).shr(s), n(a >> s));
            }
        }
    }

    #[test]
    fn integer_square_roots_are_floors() {
        for v in [0u128, 1, 2, 3, 4, 15, 16, 17, 1 << 100, (1 << 100) - 1, u128::MAX] {
            let r = n(v).isqrt();
            let r2 = r.mul(&r);
            let r1 = r.add(&BigUint::one());
            assert!(r2 <= n(v) && r1.mul(&r1) > n(v), "{v}");
        }
        // A large perfect square: (2^300 + 7)^2.
        let a = BigUint::one().shl(300).add(&n(7));
        assert_eq!(a.mul(&a).isqrt(), a);
    }

    /// Division and square roots of pseudo-random numbers up to 40 limbs satisfy their defining
    /// identities: n = q·d + r with r < d, and r² ≤ n < (r + 1)².
    #[test]
    fn random_division_and_roots_satisfy_their_identities() {
        let mut state = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for round in 0..400 {
            let len_n = 1 + (next() % 40) as usize;
            let len_d = 1 + (next() % len_n as u64) as usize;
            let mut nl: Vec<u64> = (0..len_n).map(|_| next()).collect();
            let mut dl: Vec<u64> = (0..len_d).map(|_| next()).collect();
            if round % 5 == 0 {
                // Divisors with small top limbs and runs of all-ones limbs stress the qhat correction.
                *dl.last_mut().unwrap() = 1 + next() % 3;
                nl.iter_mut().take(len_n / 2).for_each(|l| *l = u64::MAX);
            }
            let n = BigUint::from_limbs(&nl);
            let d = BigUint::from_limbs(&dl);
            if d.is_zero() {
                continue;
            }
            let (q, r) = n.divrem(&d);
            assert_eq!(q.mul(&d).add(&r), n, "round {round}");
            assert!(r < d, "round {round}");
            let s = n.isqrt();
            let s1 = s.add(&BigUint::one());
            assert!(s.mul(&s) <= n && s1.mul(&s1) > n, "round {round}");
        }
    }

    #[test]
    fn bit_queries() {
        let a = BigUint::one().shl(130).add(&n(5));
        assert_eq!(a.bits(), 131);
        assert!(a.bit(130) && a.bit(0) && !a.bit(1) && a.bit(2));
        assert!(a.any_below(1) && a.any_below(3) && !BigUint::one().shl(130).any_below(130));
    }
}
