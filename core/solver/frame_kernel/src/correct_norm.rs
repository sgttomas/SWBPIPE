//! Correctly rounded Euclidean norms: `norm2(a, b)` is RN(√(a² + b²)) and
//! `norm3(a, b, c)` is RN(√(a² + b² + c²)), rounded to nearest, ties to even.
//!
//! `f64::hypot` calls the platform's libm, which is not correctly rounded:
//! macOS and glibc differ in the last bit, so a published magnitude formed with
//! it depends on the machine. These functions use only IEEE 754 basic operations,
//! `sqrt` and `mul_add`, a correctly rounded fma (hardware, or the platform's
//! `fma` where there is none), so every platform gives the same bits. A
//! three-component magnitude is one rounding of the exact norm, not the chain
//! `a.hypot(b).hypot(c)`, so it does not depend on the order of its components.
//!
//! Special values follow C Annex F's `hypot`: an infinite argument gives +∞
//! even when another is NaN; otherwise a NaN gives NaN; zeros of either sign
//! give +0; a result beyond the finite range gives +∞.
//!
//! Method (x₀ ≥ x₁ ≥ x₂ are the absolute values, e the exponent of x₀ and
//! yᵢ = xᵢ·2⁻ᵉ, so y₀ ∈ [1, 2) exactly):
//! 1. If y₁ < 2⁻⁶⁰, the result is x₀: the exact norm lies in [x₀, x₀ + 2^(e−120)),
//!    below the midpoint x₀ + 2^(e−53) to the next double (subnormal
//!    gaps are wider still).
//! 2. Otherwise each kept yᵢ (y₀, y₁, and y₂ when y₂ ≥ 2⁻¹²⁰) is squared exactly
//!    as p + q with `mul_add`; every q is zero or a normal number, so S = Σ(p + q)
//!    is the exact scaled sum of squares. A smaller y₂ ≠ 0 is a sticky bit:
//!    y₂² < 2⁻²⁴⁰, while S − m² for every candidate midpoint m is a multiple of
//!    2⁻²²⁴, so y₂ can only break an exact tie (upward).
//! 3. A candidate from a double-double square root is moved, one ulp at a time
//!    on the destination's grid (subnormal and overflowing results included),
//!    until the exact sign of S − m² at its two neighbouring midpoints m, found
//!    by an error-free expansion sum, says it is the nearest (ties to even).

const MANTISSA: u64 = (1 << 52) - 1;

/// RN(√(a² + b²)): `hypot` with correct rounding on every platform.
pub fn norm2(a: f64, b: f64) -> f64 {
    norm3(a, b, 0.0)
}

/// RN(√(a² + b² + c²)), one rounding of the exact 3-norm.
pub fn norm3(a: f64, b: f64, c: f64) -> f64 {
    if a.is_infinite() || b.is_infinite() || c.is_infinite() {
        return f64::INFINITY;
    }
    if a.is_nan() || b.is_nan() || c.is_nan() {
        return f64::NAN;
    }
    let (mut x0, mut x1, mut x2) = (a.abs(), b.abs(), c.abs());
    if x0 < x1 {
        std::mem::swap(&mut x0, &mut x1);
    }
    if x1 < x2 {
        std::mem::swap(&mut x1, &mut x2);
    }
    if x0 < x1 {
        std::mem::swap(&mut x0, &mut x1);
    }
    if x0 == 0.0 {
        return 0.0;
    }
    let e = exponent(x0);
    let y0 = scale(x0, -e);
    let y1 = scale(x1, -e);
    if y1 < pow2(-60) {
        return x0;
    }
    let y2 = scale(x2, -e);
    let (y2, sticky) = if y2 < pow2(-120) {
        (0.0, x2 != 0.0)
    } else {
        (y2, false)
    };
    let (p0, q0) = square(y0);
    let (p1, q1) = square(y1);
    let (p2, q2) = square(y2);
    // Candidate: a double-double square root of S (about 2⁻¹⁰⁰ relative).
    let (s, t0) = two_sum(p0, p1);
    let (s, t1) = two_sum(s, p2);
    let low = ((t0 + t1) + q0) + (q1 + q2);
    let root = s.sqrt();
    let root = root + (root.mul_add(-root, s) + low) / (2.0 * root);
    let mut r = scale(root, e);
    if r > f64::MAX {
        r = f64::MAX;
    }
    let squares = [p0, q0, p1, q1, p2, q2];
    loop {
        let bits = r.to_bits();
        let odd = bits & 1 == 1;
        let rs = scale(r, -e);
        // Half the gap to the next double up, scaled by 2⁻ᵉ (MAX's gap is 2^971).
        let up = pow2(ulp_exponent(bits) - 1 - e);
        let above = midpoint_sign(&squares, rs, up);
        if above > 0 || (above == 0 && (sticky || odd)) {
            if r == f64::MAX {
                return f64::INFINITY;
            }
            r = f64::from_bits(bits + 1);
            continue;
        }
        // Below a normal power of two the gap halves.
        let normal_power_of_two = bits & MANTISSA == 0 && bits >> 52 > 1;
        let down = if normal_power_of_two { up * 0.5 } else { up };
        let below = midpoint_sign(&squares, rs, -down);
        if below < 0 || (below == 0 && !sticky && odd) {
            r = f64::from_bits(bits - 1);
            continue;
        }
        return r;
    }
}

/// The exact sign of S − (rs + h)², S the sum of `squares` and h a signed power
/// of two: rs² splits exactly, and 2·rs·h and h² are exact.
fn midpoint_sign(squares: &[f64; 6], rs: f64, h: f64) -> i32 {
    let (rr, rq) = square(rs);
    let [p0, q0, p1, q1, p2, q2] = *squares;
    exact_sign(&[p0, q0, p1, q1, p2, q2, -rr, -rq, -2.0 * rs * h, -h * h])
}

/// floor(log₂ x) for finite x > 0, subnormals included.
fn exponent(x: f64) -> i32 {
    let bits = x.to_bits();
    let biased = (bits >> 52) as i32;
    if biased == 0 {
        63 - bits.leading_zeros() as i32 - 1074
    } else {
        biased - 1023
    }
}

/// The exponent of the gap above the positive finite double with these bits.
fn ulp_exponent(bits: u64) -> i32 {
    let biased = (bits >> 52) as i32;
    if biased == 0 {
        -1074
    } else {
        biased - 1023 - 52
    }
}

/// 2^k for k in [−1022, 1023].
fn pow2(k: i32) -> f64 {
    debug_assert!((-1022..=1023).contains(&k));
    f64::from_bits(((k + 1023) as u64) << 52)
}

/// x·2^k; exact whenever the result is a normal number or x is scaled up.
fn scale(mut x: f64, mut k: i32) -> f64 {
    while k > 1000 {
        x *= pow2(1000);
        k -= 1000;
    }
    while k < -1000 {
        x *= pow2(-1000);
        k += 1000;
    }
    x * pow2(k)
}

/// a² = p + q exactly (q is exact when it is a normal number, as here).
fn square(a: f64) -> (f64, f64) {
    let p = a * a;
    (p, a.mul_add(a, -p))
}

/// a + b = s + t exactly (Knuth's TwoSum; no overflow occurs here).
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let v = s - a;
    (s, (a - (s - v)) + (b - v))
}

/// The sign of the exact sum: Shewchuk's grow-expansion with zero elimination
/// keeps a nonoverlapping expansion in increasing magnitude, whose largest
/// component carries the sign of the whole.
fn exact_sign(terms: &[f64; 10]) -> i32 {
    let mut expansion = [0.0_f64; 10];
    let mut length = 0;
    for &term in terms {
        let mut q = term;
        let mut kept = 0;
        for i in 0..length {
            let (s, t) = two_sum(q, expansion[i]);
            if t != 0.0 {
                expansion[kept] = t;
                kept += 1;
            }
            q = s;
        }
        if q != 0.0 {
            expansion[kept] = q;
            kept += 1;
        }
        length = kept;
    }
    match length {
        0 => 0,
        n if expansion[n - 1] > 0.0 => 1,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits(x: f64) -> u64 {
        x.to_bits()
    }

    #[test]
    fn annex_f_special_values() {
        let (inf, nan) = (f64::INFINITY, f64::NAN);
        for (a, b, c) in [
            (inf, nan, 0.0),
            (nan, -inf, 1.0),
            (1.0, 2.0, -inf),
            (nan, nan, inf),
        ] {
            assert_eq!(bits(norm3(a, b, c)), bits(inf));
        }
        assert_eq!(bits(norm2(-inf, nan)), bits(inf));
        assert!(norm3(nan, 1.0, 2.0).is_nan() && norm2(0.0, nan).is_nan());
        for (a, b, c) in [(0.0, -0.0, 0.0), (-0.0, -0.0, -0.0)] {
            assert_eq!(bits(norm3(a, b, c)), bits(0.0));
            assert_eq!(bits(norm2(a, b)), bits(0.0));
        }
        for x in [3.5, -f64::MAX, f64::MIN_POSITIVE, -5e-324, 1e300] {
            assert_eq!(bits(norm2(x, -0.0)), bits(x.abs()));
            assert_eq!(bits(norm3(0.0, x, -0.0)), bits(x.abs()));
        }
    }

    #[test]
    fn exact_cases_order_and_overflow() {
        assert_eq!(norm2(3.0, 4.0), 5.0);
        assert_eq!(norm3(2.0, 3.0, 6.0), 7.0);
        assert_eq!(norm3(-6.0, 2.0, -3.0), 7.0);
        assert_eq!(norm3(3e-320, 4e-320, 0.0), 5e-320);
        assert_eq!(norm2(5e-324, 5e-324), 5e-324);
        assert_eq!(norm2(f64::MAX, 1.0), f64::MAX);
        assert_eq!(norm2(f64::MAX, f64::MAX), f64::INFINITY);
        assert_eq!(
            norm2(f64::MAX * 0.5, f64::MAX * 0.5),
            (f64::MAX * 0.5) * 2f64.sqrt()
        );
        assert_eq!(norm2(1.0, 1.0), 2f64.sqrt());
        let v = [1.0e-3, -7.25, 3.0e2];
        let n = norm3(v[0], v[1], v[2]);
        for p in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            assert_eq!(bits(norm3(v[p[0]], v[p[1]], v[p[2]])), bits(n));
        }
    }

    /// Exact 2-D midpoints: k(2n + 1, 2n(n + 1), 2n² + 2n + 1) has an odd
    /// hypotenuse h in [2^53, 2^54), halfway between the doubles h − 1 and
    /// h + 1. The tie goes to the even significand (down when h ≡ 1 mod 4, as
    /// for k = 1; up when h ≡ 3 mod 4, as for k = 3), and any third component,
    /// kept exactly or as a sticky bit, breaks it upward.
    #[test]
    fn exact_midpoint_ties_and_third_components() {
        let mut seen = [false; 2];
        for (k, n) in [
            (1u128, (1u128 << 26) + 5),
            (1, 70_000_001),
            (3, 40_000_000),
            (3, 40_000_001),
        ] {
            let (a, b, h) = (
                k * (2 * n + 1),
                k * 2 * n * (n + 1),
                k * (2 * n * n + 2 * n + 1),
            );
            assert!(h >> 53 == 1 && h & 1 == 1 && a < (1 << 53) && b < (1 << 54));
            let (af, bf) = (a as f64, b as f64);
            assert!(af as u128 == a && bf as u128 == b);
            let (low, high) = ((h - 1) as f64, (h + 1) as f64);
            assert!(low as u128 == h - 1 && high as u128 == h + 1);
            let even = if (h - 1) % 4 == 0 { low } else { high };
            seen[usize::from(even == high)] = true;
            assert_eq!(bits(norm2(af, bf)), bits(even));
            assert_eq!(bits(norm2(-bf, af)), bits(even));
            for c in [1.0, 1e-300, 5e-324] {
                assert_eq!(bits(norm3(af, c, bf)), bits(high));
            }
        }
        assert_eq!(seen, [true, true]);
    }
}
