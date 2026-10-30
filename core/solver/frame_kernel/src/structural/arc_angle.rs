//! T4-I29: the binary64 arc angle 2·atan2(y, x) without libm.
//!
//! The curved element (`open_pipe_stress_curved_bend`) forms its included
//! angle φ = 2·atan2(s, c) from the half-angle sine and cosine, and every
//! stiffness and load entry depends on that one binary64 value. The
//! platform's `atan2` is not specified to the last bit, so a libm φ can
//! differ by an ulp between platforms and carry that difference into K and
//! into everything formed from it. This function forms the same angle from
//! K3a's in-tree arithmetic (`retained::wide`), which uses integer
//! operations only, so its result is bitwise identical on every platform.
//!
//! **Method.** For finite y > 0 and x > 0 (both lifted exactly), at
//! p = 128: t = rnd(y/x), a = `atan_positive(t)` (K3a's reduction and
//! series), φ̃ = 2a (exact), and the result is φ̃ rounded once to binary64
//! (`to_binary64`, to nearest, ties to even). The edges are exact:
//! y = 0 gives +0 and x = 0 gives fl(π).
//!
//! **Accuracy.** The quotient's rounding is a relative perturbation
//! |η| ≤ u = 2⁻¹²⁸ of t, which moves atan t by at most λ(η)·atan t ≈ u
//! (K3a's Lemma A); `atan_positive` adds at most 21.54u (its proved bound).
//! So |φ̃ − φ| ≤ 22.6u·φ (+O(u²)), and the binary64 result is the correctly
//! rounded φ except when φ lies within 22.6·2⁻¹²⁸·φ of a binary64 rounding
//! boundary; it is never more than 0.5 ulp + 2⁻¹²⁰ relative from φ.

use super::retained::wide::multi::Binary64Outcome;
use super::retained::wide::{Wide2, WideArith};

/// Precision of the intermediate angle (K3a's maximum).
const PRECISION: u32 = 128;

/// 2·atan2(y, x) ∈ [0, π] for finite y ≥ 0 and x ≥ 0, not both zero, as the
/// binary64 rounding of K3a's p = 128 arctangent (module documentation:
/// method and accuracy). `None` for any other input, or if the wide
/// arithmetic refuses (unreachable for binary64 operands: the exponents of
/// t and of its powers stay far inside K3a's ±2⁶² range).
pub fn twice_atan2_nonnegative(y: f64, x: f64) -> Option<f64> {
    if !(y.is_finite() && x.is_finite()) || y < 0.0 || x < 0.0 {
        return None;
    }
    match (y == 0.0, x == 0.0) {
        (true, true) => return None,
        (true, false) => return Some(0.0),
        (false, true) => return Some(std::f64::consts::PI),
        (false, false) => {}
    }
    let mut arith = WideArith::new(PRECISION).ok()?;
    let t = arith
        .div(&Wide2::from_f64(y).ok()?, &Wide2::from_f64(x).ok()?)
        .ok()?;
    let angle = arith.atan_positive(&t).ok()?.mul_pow2(1).ok()?;
    match angle.to_binary64() {
        Binary64Outcome::Normal(value) | Binary64Outcome::Subnormal { value, .. } => Some(value),
        // A positive angle below half the least subnormal rounds to +0.
        Binary64Outcome::Underflow { .. } => Some(0.0),
        Binary64Outcome::Overflow { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // (y, x, 2·atan2(y, x)) as binary64 bits, the last correctly rounded
    // from a 90-digit decimal arctangent (exact rational quotient, series
    // after half-angle reduction; standard library only). Row 1 is the s, c
    // of T0R's quarter arc as the curved element forms them (R = 0.2, chord
    // (1.2 − 1.0, 0.2, 0)), whose correctly rounded φ is one ulp above the
    // value one platform libm returned; then 1e-9 rad, near-π, s = 1/2 and
    // extreme-ratio cases, 30 random s with c = √(1 − s²), and 10 random
    // (y, x) over ±20 decades. Bitwise equality with the correctly rounded
    // value is expected on every row (no row lies within 2⁻¹²⁰ of a rounding
    // boundary), and the comparison is platform-independent.
    #[test]
    fn arc_angle_bits_are_frozen() {
        for (y, x, bits) in FROZEN {
            let got = twice_atan2_nonnegative(f64::from_bits(*y), f64::from_bits(*x)).unwrap();
            assert_eq!(
                got.to_bits(),
                *bits,
                "y={:e} x={:e} got {got:e}",
                f64::from_bits(*y),
                f64::from_bits(*x)
            );
        }
    }

    const FROZEN: &[(u64, u64, u64)] = &[
        (0x3fe6a09e667f3bcc, 0x3fe6a09e667f3bcd, 0x3ff921fb54442d18),
        (0x3e012e0be826d695, 0x3ff0000000000000, 0x3e112e0be826d695),
        (0x3ff0000000000000, 0x3e112e0be826d695, 0x400921fb53ff74e9),
        (0x3fe0000000000000, 0x3febb67ae8584caa, 0x3ff0c152382d7366),
        (0x01a56e1fc2f8f359, 0x4008000000000000, 0x019c92d503f699cc),
        (0x4008000000000000, 0x01a56e1fc2f8f359, 0x400921fb54442d18),
        (0x3fe18a30f84df15a, 0x3feac3ca9d04f6c3, 0x3ff2904aa4a802ab),
        (0x3fd6221bccc9caac, 0x3fee068669b51b22, 0x3fe6999a4b3efc2f),
        (0x3feb0905266c46fc, 0x3fe11ebb87db6735, 0x400019c0274f602c),
        (0x3fd2786160b1a082, 0x3feea36f4c0b21c3, 0x3fe2bc9b3a509a75),
        (0x3fe054bf17b583cd, 0x3feb84df42bba6c0, 0x3ff123852572fe36),
        (0x3fd6010d9b24b0e0, 0x3fee0c98b9197f45, 0x3fe676632e1badda),
        (0x3fda9773d826331a, 0x3fed1b68f8438594, 0x3feb6c4f83f49d50),
        (0x3fef29d9ec211ecd, 0x3fcd1363cd9e9658, 0x4005775d64e1b955),
        (0x3fba8e39bb1e1138, 0x3fefd3ce4fa543f1, 0x3fca9a7a192ad595),
        (0x3fdc7604c537b312, 0x3feca964cd1fd143, 0x3fed7e7378b81ef2),
        (0x3fccc67e8cd9645c, 0x3fef2e4f94254224, 0x3fdd05ff79e67c0d),
        (0x3fd65f4c70af257e, 0x3fedfb2de41beb99, 0x3fe6dadd5e1c84e8),
        (0x3feff5add1fdd6df, 0x3fa9b166a135f53f, 0x4008545a0341510f),
        (0x3fd5133bf756f242, 0x3fee3718d61f448f, 0x3fe579d4fd6a4912),
        (0x3fe372ea662b485d, 0x3fe9695364f5a214, 0x3ff4e77fd8402f64),
        (0x3fdadb00b6b04aba, 0x3fed0be3a27deb29, 0x3febb6a6e5d87c0c),
        (0x3fe75fe044cc203c, 0x3fe5dad3c383da39, 0x3ffa3521efc2107b),
        (0x3fdc9f0c47d11392, 0x3fec9f2c2538183f, 0x3fedac4a852de26f),
        (0x3fc5d0d70382db68, 0x3fef88241e316657, 0x3fd5ec3cec014380),
        (0x3fdbc539be45e6de, 0x3fecd49f6bb0f195, 0x3fecb9a605ff35ff),
        (0x3fda1da83564e994, 0x3fed36ed85e7105f, 0x3feae6a853c893de),
        (0x3fe95df146bc3252, 0x3fe381c0ebc6633b, 0x3ffd49c33d653a18),
        (0x3fe200df3e181022, 0x3fea7487ada0e2fc, 0x3ff31f021505f959),
        (0x3fc923ad58b2b590, 0x3fef607309543d56, 0x3fd94dc9614d8e13),
        (0x3fe59023d5b091a4, 0x3fe7a4d806342ca3, 0x3ff7a92b9df2baee),
        (0x3fd318953239e854, 0x3fee8ad7c5346a6a, 0x3fe36430824d2373),
        (0x3fe06168d7007fcc, 0x3feb7d578462eee2, 0x3ff13240c465bdaf),
        (0x3fcc243ffd1999ec, 0x3fef378f778a0e3e, 0x3fdc5f969b806463),
        (0x3fda49cd37ab4430, 0x3fed2d05ebc9be29, 0x3feb170b03bee5ff),
        (0x3fe927d6bde3d622, 0x3fe3c75537247ba2, 0x3ffcf19f8d12682f),
        (0x3faa676ee751ce08, 0x4370cd90f837745f, 0x3c3924689d3a5234),
        (0x41fa5fb10c01e5e7, 0x43cb913af7792cf8, 0x3e2e9d550113b036),
        (0x3d031af2d6a631ab, 0x4086d87d9b59a140, 0x3c7ac2c487c55b27),
        (0x3f167a4fa16dbb76, 0x3ff800024dc57747, 0x3f1df8674b89f1a8),
        (0x4306cafb10a6ee06, 0x411848903c481886, 0x400921fb54221560),
        (0x4157d94cc52203c8, 0x43d03021d3d9593a, 0x3d879263c3ce1d15),
        (0x3fe51999589f6e8c, 0x44102a3f0fe3749b, 0x3bd4e274962cf6dc),
        (0x3e73b65eba9001ca, 0x3bbfb792a05fa89c, 0x400921fb54442b7c),
        (0x3bfcb28b90e1a1ff, 0x411852f406a835c7, 0x3ae2e073913d803e),
        (0x3d700413347c6bf9, 0x3f71e0581a5f3918, 0x3dfcab7144bac2cd),
    ];

    #[test]
    fn edges_and_refusals() {
        assert_eq!(twice_atan2_nonnegative(0.0, 1.0), Some(0.0));
        assert_eq!(
            twice_atan2_nonnegative(1.0, 0.0),
            Some(std::f64::consts::PI)
        );
        assert_eq!(
            twice_atan2_nonnegative(1.0, 1.0),
            Some(std::f64::consts::FRAC_PI_2)
        );
        assert_eq!(twice_atan2_nonnegative(0.0, 0.0), None);
        assert_eq!(twice_atan2_nonnegative(-1.0, 1.0), None);
        assert_eq!(twice_atan2_nonnegative(1.0, -1.0), None);
        assert_eq!(twice_atan2_nonnegative(f64::NAN, 1.0), None);
        assert_eq!(twice_atan2_nonnegative(1.0, f64::INFINITY), None);
        // The smallest and largest ratios binary64 operands can form.
        let tiny = twice_atan2_nonnegative(f64::from_bits(1), f64::MAX).unwrap();
        assert_eq!(tiny, 0.0);
        let huge = twice_atan2_nonnegative(f64::MAX, f64::from_bits(1)).unwrap();
        assert_eq!(huge, std::f64::consts::PI);
    }
}
