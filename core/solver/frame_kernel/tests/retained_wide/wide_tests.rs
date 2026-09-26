//! K3a tests for `structural::retained::wide` (T3 D1 §4.11 and §7.4, V1-S9's
//! plan scoped to L = 2; ROOT selection condition C1).
//!
//! This file is compiled as the `tests` module of `wide.rs` (a `#[path]`
//! module), so it runs in `frame_kernel`'s own unit-test suite and in hosted
//! CI. Expected values come from `gen_wide_vectors.py` beside it (standard
//! library `fractions.Fraction` and `decimal`), from hardware binary64 at
//! p = 53, or from exact identities; none comes from the code under test.
use super::*;

const TARGETED: &str = include_str!("targeted.txt");
const SPLIT: &str = include_str!("split.txt");
const ATAN: &str = include_str!("atan.txt");
const DIFF_SAMPLE: &str = include_str!("differential_sample.txt");
const DIFF_MANIFEST: &str = include_str!("differential.txt");
const SHA256SUMS: &str = include_str!("SHA256SUMS");

// ---------------------------------------------------------------------------
// Test-only helpers: SHA-256 (FIPS 180-4), SplitMix64, value tokens
// ---------------------------------------------------------------------------

struct Sha256 {
    state: [u32; 8],
    block: [u8; 64],
    filled: usize,
    length: u64,
}

const SHA_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

impl Sha256 {
    fn new() -> Self {
        Self {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            block: [0; 64],
            filled: 0,
            length: 0,
        }
    }

    fn compress(state: &mut [u32; 8], block: &[u8]) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[4 * i],
                block[4 * i + 1],
                block[4 * i + 2],
                block[4 * i + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut v = *state;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7]
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA_K[i])
                .wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v = [
                t1.wrapping_add(t2),
                v[0],
                v[1],
                v[2],
                v[3].wrapping_add(t1),
                v[4],
                v[5],
                v[6],
            ];
        }
        for (s, x) in state.iter_mut().zip(v) {
            *s = s.wrapping_add(x);
        }
    }

    fn update(&mut self, mut data: &[u8]) {
        self.length = self.length.wrapping_add(data.len() as u64);
        if self.filled > 0 {
            let take = (64 - self.filled).min(data.len());
            self.block[self.filled..self.filled + take].copy_from_slice(&data[..take]);
            self.filled += take;
            data = &data[take..];
            if self.filled < 64 {
                return;
            }
            let block = self.block;
            Self::compress(&mut self.state, &block);
            self.filled = 0;
        }
        while data.len() >= 64 {
            Self::compress(&mut self.state, &data[..64]);
            data = &data[64..];
        }
        self.block[..data.len()].copy_from_slice(data);
        self.filled = data.len();
    }

    fn hex(mut self) -> String {
        let bits = self.length.wrapping_mul(8);
        self.update(&[0x80]);
        while self.filled != 56 {
            self.update(&[0]);
        }
        self.update(&bits.to_be_bytes());
        self.state.iter().map(|w| format!("{w:08x}")).collect()
    }
}

fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    h.hex()
}

struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

const TOP: u128 = 1 << 127;
const FULL: u128 = u128::MAX;

fn ones(w: u64) -> u128 {
    if w >= 128 {
        FULL
    } else {
        (1u128 << w) - 1
    }
}

fn w(negative: bool, exponent: i64, sig: u128) -> Wide2 {
    Wide2::from_parts(negative, exponent, [sig as u64, (sig >> 64) as u64]).unwrap()
}

fn f(x: f64) -> Wide2 {
    Wide2::from_f64(x).unwrap()
}

fn arith(p: u32) -> WideArith {
    WideArith::new(p).unwrap()
}

fn parse_value(token: &str) -> Wide2 {
    match token {
        "Z+" => return Wide2::ZERO,
        "Z-" => return Wide2::ZERO.neg(),
        _ => {}
    }
    let negative = match &token[..1] {
        "+" => false,
        "-" => true,
        other => panic!("bad sign {other} in {token}"),
    };
    let (hex, exponent) = token[1..].split_once('p').expect(token);
    let sig = u128::from_str_radix(hex, 16).expect(token);
    w(negative, exponent.parse().expect(token), sig)
}

fn error_token(e: WideError) -> String {
    match e {
        WideError::DivisionByZero => "E:div0".into(),
        WideError::NegativeSqrt => "E:neg_sqrt".into(),
        WideError::AngleDomain => "E:angle_domain".into(),
        WideError::SplitOverflow => "E:split_overflow".into(),
        other => format!("E:unexpected:{other:?}"),
    }
}

fn token(result: Result<Wide2, WideError>) -> String {
    match result {
        Ok(v) => format!("{v:?}"),
        Err(e) => error_token(e),
    }
}

fn apply(a: &mut WideArith, op: &str, x: &Wide2, y: Option<&Wide2>) -> Result<Wide2, WideError> {
    match op {
        "add" => a.add(x, y.unwrap()),
        "sub" => a.sub(x, y.unwrap()),
        "mul" => a.mul(x, y.unwrap()),
        "div" => a.div(x, y.unwrap()),
        "sqrt" => a.sqrt(x),
        other => panic!("unknown op {other}"),
    }
}

// ---------------------------------------------------------------------------
// Integrity of the committed vectors and of the test helpers
// ---------------------------------------------------------------------------

#[test]
fn sha256_known_answers() {
    assert_eq!(
        sha256_hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    let long = vec![b'a'; 1_000_000];
    let mut h = Sha256::new();
    for chunk in long.chunks(997) {
        h.update(chunk);
    }
    assert_eq!(
        h.hex(),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
}

#[test]
fn committed_vectors_match_their_recorded_sha256() {
    let files = [
        ("targeted.txt", TARGETED),
        ("split.txt", SPLIT),
        ("atan.txt", ATAN),
        ("differential_sample.txt", DIFF_SAMPLE),
        ("differential.txt", DIFF_MANIFEST),
    ];
    let mut seen = 0;
    for line in SHA256SUMS.lines() {
        let (digest, name) = line.split_once("  ").expect(line);
        let (_, text) = files
            .iter()
            .find(|(n, _)| *n == name)
            .unwrap_or_else(|| panic!("unlisted file {name}"));
        assert_eq!(sha256_hex(text.as_bytes()), digest, "{name}");
        seen += 1;
    }
    assert_eq!(seen, files.len());
}

// ---------------------------------------------------------------------------
// Construction, lift, precision, signs of zero, work counter
// ---------------------------------------------------------------------------

#[test]
fn lift_from_f64_is_exact_for_normal_subnormal_and_signed_zero() {
    assert_eq!(f(1.0), Wide2::ONE);
    assert_eq!(f(0.0), Wide2::ZERO);
    assert_eq!(f(-0.0), Wide2::ZERO.neg());
    assert!(f(-0.0).is_sign_negative() && f(-0.0).is_zero());
    let min_sub = f64::from_bits(1);
    assert_eq!(f(min_sub).parts(), (false, -1074, [0, 1 << 63]));
    let max_sub = f64::from_bits((1u64 << 52) - 1);
    assert_eq!(f(max_sub).exponent(), -1023);
    assert_eq!(f(f64::MIN_POSITIVE).parts(), (false, -1022, [0, 1 << 63]));
    let max = f(f64::MAX);
    assert_eq!(max.exponent(), 1023);
    assert_eq!(max.parts().2, [0, u64::MAX << 11]);
    assert_eq!(f(-3.0).parts(), (true, 1, [0, 3 << 62]));
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(Wide2::from_f64(bad), Err(WideError::NonFinite));
    }
    // Round trip through the split: one exact term equal to the input.
    let mut rng = SplitMix64(0x11F7);
    for i in 0..20_000 {
        let bits = match i % 4 {
            0 => rng.next() & ((1u64 << 52) - 1), // subnormal (or zero)
            _ => rng.next(),
        };
        let x = f64::from_bits(bits);
        if !x.is_finite() {
            continue;
        }
        let split = f(x).split_binary64().unwrap();
        assert!(!split.truncated_below_min_subnormal());
        if x == 0.0 {
            assert!(split.terms().is_empty());
        } else {
            assert_eq!(split.terms().len(), 1);
            assert_eq!(split.terms()[0].to_bits(), bits);
        }
    }
}

#[test]
fn from_parts_validates_normalization_and_exponent_range() {
    assert_eq!(
        Wide2::from_parts(false, 0, [1, 0]),
        Err(WideError::NotNormalized)
    );
    assert_eq!(
        Wide2::from_parts(false, 5, [0, 0]),
        Err(WideError::NotNormalized)
    );
    for e in [EXPONENT_LIMIT + 1, -EXPONENT_LIMIT - 1, i64::MAX, i64::MIN] {
        assert_eq!(
            Wide2::from_parts(false, e, [0, 1 << 63]),
            Err(WideError::ExponentRange)
        );
    }
    assert!(Wide2::from_parts(true, EXPONENT_LIMIT, [0, 1 << 63]).is_ok());
    assert!(Wide2::from_parts(true, -EXPONENT_LIMIT, [0, 1 << 63]).is_ok());
}

#[test]
fn precision_outside_the_supported_range_is_refused() {
    for p in [0, 1, 129, 256] {
        assert_eq!(
            WideArith::new(p).unwrap_err(),
            WideError::InvalidPrecision(p)
        );
    }
    for p in [2, 53, 64, 128] {
        assert_eq!(arith(p).precision(), p);
    }
    let mut a = arith(52);
    assert_eq!(
        a.included_angle(&f(1.0), &f(0.0)),
        Err(WideError::InvalidPrecision(52))
    );
    assert_eq!(
        a.atan_positive(&f(1.0)),
        Err(WideError::InvalidPrecision(52))
    );
}

#[test]
fn signs_of_zero_follow_round_to_nearest() {
    for p in [53, 128] {
        let mut a = arith(p);
        let (pz, nz) = (Wide2::ZERO, Wide2::ZERO.neg());
        assert_eq!(a.add(&pz, &nz).unwrap(), pz);
        assert_eq!(a.add(&nz, &nz).unwrap(), nz);
        assert_eq!(a.sub(&nz, &pz).unwrap(), nz);
        assert_eq!(a.sub(&pz, &pz).unwrap(), pz);
        let x = w(true, 17, TOP | 12345);
        assert_eq!(a.add(&x, &x.neg()).unwrap(), pz);
        assert_eq!(a.sub(&x, &x).unwrap(), pz);
        assert_eq!(a.mul(&nz, &f(3.0)).unwrap(), nz);
        assert_eq!(a.mul(&nz, &f(-3.0)).unwrap(), pz);
        assert_eq!(a.div(&nz, &f(-2.0)).unwrap(), pz);
        assert_eq!(a.sqrt(&nz).unwrap(), nz);
        assert_eq!(a.div(&f(1.0), &pz), Err(WideError::DivisionByZero));
        assert_eq!(a.div(&pz, &nz), Err(WideError::DivisionByZero));
        assert_eq!(a.sqrt(&f(-1.0)), Err(WideError::NegativeSqrt));
    }
}

#[test]
fn work_counter_counts_every_rounded_operation_and_saturates() {
    let mut a = arith(128);
    let x = f(2.0);
    a.add(&x, &x).unwrap();
    a.sub(&x, &x).unwrap();
    a.mul(&x, &x).unwrap();
    a.mul(&x, &x).unwrap();
    a.div(&x, &x).unwrap();
    let _ = a.div(&x, &Wide2::ZERO);
    a.sqrt(&x).unwrap();
    let c = a.work();
    assert_eq!(
        (c.add, c.sub, c.mul, c.div, c.sqrt, c.atan),
        (1, 1, 2, 2, 1, 0)
    );
    assert_eq!(c.rounded_operations(), 7);
    let before = a.work().rounded_operations();
    a.included_angle(&f(1.0), &f(0.0)).unwrap();
    let c = a.work();
    assert_eq!(c.atan, 1);
    assert!(c.rounded_operations() > before + 10);
    let mut full = WorkCounter {
        add: u64::MAX,
        sub: u64::MAX,
        ..WorkCounter::default()
    };
    assert_eq!(full.rounded_operations(), u64::MAX);
    full.add = full.add.saturating_add(1);
    assert_eq!(full.add, u64::MAX);
}

// ---------------------------------------------------------------------------
// p = 53 against hardware binary64, bitwise
// ---------------------------------------------------------------------------

fn random_normal(rng: &mut SplitMix64, exp_span: u64) -> f64 {
    let r = rng.next();
    let sign = r >> 63;
    let exponent = 1023 + (r >> 20) % (2 * exp_span + 1) - exp_span;
    let fraction = rng.next() & ((1u64 << 52) - 1);
    f64::from_bits((sign << 63) | (exponent << 52) | fraction)
}

fn assert_hardware(a: &mut WideArith, op: &str, x: f64, y: f64, hw: f64) {
    assert!(hw.is_normal() || hw == 0.0, "{op} {x:e} {y:e} -> {hw:e}");
    let got = apply(a, op, &f(x), Some(&f(y))).unwrap();
    assert_eq!(got, f(hw), "{op} {x:e} {y:e}: {got:?}");
}

#[test]
fn p53_matches_hardware_binary64_bitwise() {
    let mut a = arith(53);
    let mut rng = SplitMix64(0x5053_3533);
    for i in 0..100_000u64 {
        let x = random_normal(&mut rng, 400);
        let y = match i % 8 {
            0 => -x,
            1 => f64::from_bits(x.to_bits() ^ 1),
            2 => -f64::from_bits(x.to_bits().wrapping_add(1 + (rng.next() & 7))),
            3 => x * 0.5,
            _ => random_normal(&mut rng, 400),
        };
        assert_hardware(&mut a, "add", x, y, x + y);
        assert_hardware(&mut a, "sub", x, y, x - y);
        assert_hardware(&mut a, "mul", x, y, x * y);
        assert_hardware(&mut a, "div", x, y, x / y);
        let s = x.abs();
        assert_eq!(a.sqrt(&f(s)).unwrap(), f(s.sqrt()), "sqrt {s:e}");
    }
    // Ties at p = 53 from binary64 operands (even stays, odd rounds up).
    let e = f64::EPSILON; // 2^-52
    for (x, y) in [
        (1.0, e / 2.0),
        (1.0 + e, e / 2.0),
        (1.0 + 2.0 * e, e / 2.0),
        (-(1.0 + e), -e / 2.0),
        (2.0 - e, e / 2.0),
        (1.0, e / 2.0 + e * e),
    ] {
        assert_hardware(&mut a, "add", x, y, x + y);
    }
}

// ---------------------------------------------------------------------------
// Targeted hard classes and the error-free transformations (generated)
// ---------------------------------------------------------------------------

fn two_sum(a: &mut WideArith, x: &Wide2, y: &Wide2) -> (Wide2, Wide2) {
    let s = a.add(x, y).unwrap();
    let bv = a.sub(&s, x).unwrap();
    let av = a.sub(&s, &bv).unwrap();
    let ea = a.sub(x, &av).unwrap();
    let eb = a.sub(y, &bv).unwrap();
    (s, a.add(&ea, &eb).unwrap())
}

fn two_product(a: &mut WideArith, x: &Wide2, y: &Wide2) -> (Wide2, Wide2) {
    let p = a.precision();
    let splitter = w(
        false,
        i64::from((p + 1) / 2),
        TOP | (1u128 << (127 - (p + 1) / 2)),
    );
    let split = |a: &mut WideArith, v: &Wide2| {
        let c = a.mul(&splitter, v).unwrap();
        let d = a.sub(&c, v).unwrap();
        let hi = a.sub(&c, &d).unwrap();
        let lo = a.sub(v, &hi).unwrap();
        (hi, lo)
    };
    let prod = a.mul(x, y).unwrap();
    let (xh, xl) = split(a, x);
    let (yh, yl) = split(a, y);
    let t = a.mul(&xh, &yh).unwrap();
    let e1 = a.sub(&prod, &t).unwrap();
    let t = a.mul(&xl, &yh).unwrap();
    let e2 = a.sub(&e1, &t).unwrap();
    let t = a.mul(&xh, &yl).unwrap();
    let e3 = a.sub(&e2, &t).unwrap();
    let t = a.mul(&xl, &yl).unwrap();
    (prod, a.sub(&t, &e3).unwrap())
}

#[test]
fn targeted_hard_class_vectors_match_the_fraction_oracle() {
    use std::collections::BTreeMap;
    let mut counts: BTreeMap<(String, String, u32), usize> = BTreeMap::new();
    for line in TARGETED.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let p: u32 = fields[2].parse().unwrap();
        let mut a = arith(p);
        let x = parse_value(fields[3]);
        if fields[0] == "eft" {
            let y = parse_value(fields[4]);
            let (s, e) = match fields[1] {
                "twosum" => two_sum(&mut a, &x, &y),
                "twoprod" => two_product(&mut a, &x, &y),
                other => panic!("{other}"),
            };
            assert_eq!(format!("{s:?}"), fields[5], "{line}");
            assert_eq!(format!("{e:?}"), fields[6], "{line}");
        } else {
            let y = (fields[4] != "-").then(|| parse_value(fields[4]));
            let got = token(apply(&mut a, fields[1], &x, y.as_ref()));
            assert_eq!(got, fields[5], "{line}");
        }
        *counts
            .entry((fields[0].to_string(), fields[1].to_string(), p))
            .or_default() += 1;
    }
    let has = |tag: &str, op: &str, p: u32| {
        counts
            .get(&(tag.into(), op.into(), p))
            .copied()
            .unwrap_or(0)
    };
    for p in [53, 128] {
        for op in ["add", "sub", "mul", "div", "sqrt"] {
            assert!(has("carry", op, p) > 0, "carry {op} {p}");
            assert!(has("cancel1ulp", op, p) > 0, "cancel {op} {p}");
        }
        for op in ["add", "sub", "mul", "div"] {
            assert!(has("sticky", op, p) > 0, "sticky {op} {p}");
        }
        assert!(has("cancelpprime", "sub", p) > 0);
        assert!(has("exactdiv", "div", p) > 0 && has("nearexactdiv", "div", p) > 0);
        assert!(has("perfectsq", "sqrt", p) > 0 && has("nearsq", "sqrt", p) > 0);
        assert!(has("eft", "twosum", p) > 0 && has("eft", "twoprod", p) > 0);
    }
    // Exact ties at the limb boundaries (p = 64: bits 63/64; p = 128: 127/128)
    // and beside them, for every operation where a tie is representable.
    for p in [53, 63, 64, 65, 127, 128] {
        for op in ["add", "sub", "mul"] {
            assert!(has("tie", op, p) > 0, "tie {op} {p}");
        }
    }
    for p in [53, 63, 64, 65, 127] {
        assert!(has("tie", "div", p) > 0, "tie div {p}");
    }
    for p in [53, 63] {
        assert!(has("tie", "sqrt", p) > 0 && has("sticky", "sqrt", p) > 0);
    }
}

#[test]
fn twosum_and_twoproduct_are_error_free_on_binary64_inputs() {
    // Independent of the generated vectors: at p = 53, x + y and s + e (and
    // x * y and p + e) are formed exactly at p = 128 and compared.
    let mut a53 = arith(53);
    let mut a128 = arith(128);
    let mut rng = SplitMix64(0xEF7);
    for _ in 0..20_000 {
        let x = f(random_normal(&mut rng, 30));
        let y = f(random_normal(&mut rng, 30));
        let (s, e) = two_sum(&mut a53, &x, &y);
        assert!(s.fits_precision(53) && e.fits_precision(53));
        assert_eq!(a128.add(&x, &y).unwrap(), a128.add(&s, &e).unwrap());
        let (pr, pe) = two_product(&mut a53, &x, &y);
        assert!(pr.fits_precision(53) && pe.fits_precision(53));
        assert_eq!(a128.mul(&x, &y).unwrap(), a128.add(&pr, &pe).unwrap());
    }
}

// ---------------------------------------------------------------------------
// Exponent extremes: refused, never wrapped
// ---------------------------------------------------------------------------

#[test]
fn exponent_extremes_are_refused_never_wrapped() {
    let big = w(false, EXPONENT_LIMIT, TOP);
    let big_ones = w(false, EXPONENT_LIMIT, FULL);
    let tiny = w(false, -EXPONENT_LIMIT, TOP);
    let range = Err(WideError::ExponentRange);
    for p in [53, 128] {
        let mut a = arith(p);
        assert_eq!(a.mul(&big, &big), range);
        assert_eq!(a.mul(&big, &f(2.0)), range);
        assert_eq!(a.add(&big, &big), range);
        assert_eq!(a.div(&big, &f(0.5)), range);
        assert_eq!(a.mul(&tiny, &tiny), range);
        assert_eq!(a.div(&tiny, &big), range);
        assert_eq!(a.div(&big, &tiny), range);
        assert_eq!(a.div(&tiny, &f(2.0)), range);
        // cancellation below the limit
        let tiny2 = w(false, -EXPONENT_LIMIT, TOP | 1);
        assert_eq!(a.sub(&tiny2, &tiny), range);
        // in range at the limits
        assert_eq!(a.mul(&big, &f(1.0)).unwrap(), big);
        assert_eq!(a.mul(&tiny, &f(-1.0)).unwrap(), tiny.neg());
        assert!(a.sqrt(&big).is_ok() && a.sqrt(&tiny).is_ok());
        // the arctangent's t^2 leaves the range for such arguments
        assert_eq!(a.atan_positive(&big), range);
        assert_eq!(a.atan_positive(&tiny), range);
    }
    // Rounding up across the limit (p < 128) is refused as well.
    assert_eq!(arith(53).add(&big_ones, &Wide2::ZERO), range);
    assert_eq!(arith(128).add(&big_ones, &Wide2::ZERO).unwrap(), big_ones);
    assert_eq!(big.mul_pow2(1), range);
    assert_eq!(tiny.mul_pow2(-1), range);
    assert_eq!(big.mul_pow2(i64::MAX), range);
    assert_eq!(tiny.mul_pow2(i64::MIN), range);
    assert_eq!(Wide2::ZERO.mul_pow2(i64::MAX).unwrap(), Wide2::ZERO);
}

// ---------------------------------------------------------------------------
// The seeded Fraction differential (regenerated from the recorded seeds)
// ---------------------------------------------------------------------------

fn rand_sig(rng: &mut SplitMix64) -> u128 {
    let hi = rng.next();
    let lo = rng.next();
    let sel = rng.next();
    let mut sig = ((u128::from(hi) << 64) | u128::from(lo)) | TOP;
    match sel % 8 {
        0 => {
            let nbits = 1 + (sel >> 8) % 128;
            sig = (sig >> (128 - nbits)) << (128 - nbits);
        }
        1 => {
            let w = 1 + (sel >> 8) % 128;
            sig |= ones(w) << (128 - w);
        }
        2 => {
            let nbits = 1 + (sel >> 8) % 127;
            let low = (sel >> 16) % (128 - nbits);
            sig = ((sig >> (128 - nbits)) << (128 - nbits)) | (1u128 << low);
        }
        _ => {}
    }
    sig
}

/// Mirrors `gen_operands` in gen_wide_vectors.py call for call.
fn gen_operands(rng: &mut SplitMix64) -> (u8, Wide2, Wide2) {
    let r = rng.next();
    let op = (r % 5) as u8;
    let mode = (r >> 3) % 8;
    let mut a_neg = (r >> 6) & 1 == 1;
    let b_neg = (r >> 7) & 1 == 1;
    let ea = ((r >> 8) % 4001) as i64 - 2000;
    let zero_a = (r >> 20) % 64 == 0;
    let zero_b = (r >> 26) % 64 == 0;
    let mut a_sig = rand_sig(rng);
    let (b_sig, eb) = match mode {
        0..=3 => {
            let eb = ea + (rng.next() % 301) as i64 - 150;
            (rand_sig(rng), eb)
        }
        4 => {
            let delta = u128::from(rng.next() % 16);
            let b = if a_sig <= FULL - delta {
                a_sig + delta
            } else {
                a_sig - delta
            };
            (b, ea)
        }
        5 => {
            let width = rng.next() % 129;
            let x_hi = rng.next();
            let x_lo = rng.next();
            let x = (u128::from(x_hi) << 64) | u128::from(x_lo);
            ((a_sig ^ (x & ones(width))) | TOP, ea)
        }
        6 => {
            let width = rng.next() % 128;
            let x = rng.next();
            a_sig = FULL ^ (u128::from(x) & ones(width));
            let width2 = rng.next() % 128;
            let y = rng.next();
            (TOP | (u128::from(y) & ones(width2)), ea + 1)
        }
        _ => {
            let eb = ea + (rng.next() % 3001) as i64 - 1500;
            (rand_sig(rng), eb)
        }
    };
    if op == 4 {
        a_neg = false;
    }
    let a = if zero_a {
        if a_neg {
            Wide2::ZERO.neg()
        } else {
            Wide2::ZERO
        }
    } else {
        w(a_neg, ea, a_sig)
    };
    let b = if zero_b {
        if b_neg {
            Wide2::ZERO.neg()
        } else {
            Wide2::ZERO
        }
    } else {
        w(b_neg, eb, b_sig)
    };
    (op, a, b)
}

const OP_NAMES: [&str; 5] = ["add", "sub", "mul", "div", "sqrt"];

struct StreamSpec {
    seed: u64,
    count: usize,
    chunk: usize,
    sha256: String,
    chunks: Vec<String>,
}

fn stream_spec(name: &str) -> StreamSpec {
    let mut spec = None;
    let mut chunks = Vec::new();
    for line in DIFF_MANIFEST.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields[0] == "stream" && fields[1] == name {
            spec = Some((
                u64::from_str_radix(fields[3], 16).unwrap(),
                fields[5].parse().unwrap(),
                fields[7].parse().unwrap(),
                fields[9].to_string(),
            ));
        }
        if fields[0] == "chunk" && fields[1] == name {
            assert_eq!(fields[2].parse::<usize>().unwrap(), chunks.len());
            chunks.push(fields[3].to_string());
        }
    }
    let (seed, count, chunk, sha256) = spec.unwrap();
    StreamSpec {
        seed,
        count,
        chunk,
        sha256,
        chunks,
    }
}

fn run_differential(name: &str, mixed: bool, min_count: usize) {
    let spec = stream_spec(name);
    assert!(spec.count >= min_count, "{name}: {} operations", spec.count);
    assert_eq!(spec.chunks.len() * spec.chunk, spec.count);
    let samples: Vec<Vec<&str>> = DIFF_SAMPLE
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .filter(|f| f[0] == name)
        .collect();
    assert!(samples.len() >= 1000);
    let mut rng = SplitMix64(spec.seed);
    let mut total = Sha256::new();
    let mut chunk = Sha256::new();
    let mut arith_by_p: Vec<Option<WideArith>> = vec![None; 129];
    let mut record = Vec::with_capacity(28);
    let mut per_op = [0usize; 5];
    for i in 0..spec.count {
        let p = if mixed {
            2 + (rng.next() % 127) as u32
        } else {
            128
        };
        let (op, a, b) = gen_operands(&mut rng);
        per_op[op as usize] += 1;
        let arith = arith_by_p[p as usize].get_or_insert_with(|| WideArith::new(p).unwrap());
        let result = apply(arith, OP_NAMES[op as usize], &a, Some(&b));
        if let Some(sample) = samples.get(i) {
            assert_eq!(sample[1].parse::<usize>().unwrap(), i);
            assert_eq!(
                (
                    sample[2],
                    sample[3].parse::<u32>().unwrap(),
                    sample[4],
                    sample[5]
                ),
                (
                    OP_NAMES[op as usize],
                    p,
                    format!("{a:?}").as_str(),
                    format!("{b:?}").as_str()
                ),
                "{name} operands {i} (generator port)"
            );
            assert_eq!(token(result), sample[6], "{name} record {i}");
        }
        record.clear();
        record.push(op);
        if mixed {
            record.push(p as u8);
        }
        match result {
            Ok(v) => {
                let (negative, exponent, sig) = v.parts();
                record.push(0);
                record.push(u8::from(negative));
                record.extend_from_slice(&exponent.to_le_bytes());
                record.extend_from_slice(&sig[0].to_le_bytes());
                record.extend_from_slice(&sig[1].to_le_bytes());
            }
            Err(WideError::DivisionByZero) => {
                record.push(1);
                record.extend_from_slice(&[0u8; 25]);
            }
            Err(e) => panic!("{name} record {i}: unexpected {e:?}"),
        }
        total.update(&record);
        chunk.update(&record);
        if (i + 1) % spec.chunk == 0 {
            let done = std::mem::replace(&mut chunk, Sha256::new());
            let k = i / spec.chunk;
            assert_eq!(done.hex(), spec.chunks[k], "{name} chunk {k}");
        }
    }
    assert_eq!(total.hex(), spec.sha256, "{name} stream");
    for (op, n) in per_op.iter().enumerate() {
        assert!(*n > spec.count / 6, "{name}: {} {}", OP_NAMES[op], n);
    }
}

#[test]
fn seeded_fraction_differential_at_p128() {
    run_differential("p128", false, 1_000_000);
}

#[test]
fn seeded_fraction_differential_at_mixed_precision() {
    run_differential("mixed", true, 200_000);
}

// ---------------------------------------------------------------------------
// The exact binary64 split and ExactAccumulator::add_product
// ---------------------------------------------------------------------------

/// value − Σ terms, formed exactly at p = 128.
fn split_remainder(value: &Wide2, split: &Binary64Split) -> Wide2 {
    let mut a = arith(128);
    let mut sum = Wide2::ZERO;
    for &t in split.terms() {
        sum = a.add(&sum, &f(t)).unwrap();
    }
    a.sub(value, &sum).unwrap()
}

fn check_split_invariants(value: &Wide2, split: &Binary64Split) {
    let terms = split.terms();
    assert!(terms.len() <= 3);
    for pair in terms.windows(2) {
        assert!(pair[0].abs() > pair[1].abs());
    }
    for t in terms {
        assert!(t.is_finite() && *t != 0.0);
        assert_eq!(t.is_sign_negative(), value.is_sign_negative());
    }
    let rem = split_remainder(value, split);
    if split.truncated_below_min_subnormal() {
        assert!(!rem.is_zero(), "{value:?}");
        assert_eq!(rem.is_sign_negative(), value.is_sign_negative());
        assert!(rem.exponent() < -1074, "{value:?}: {rem:?}");
    } else {
        assert!(rem.is_zero(), "{value:?}: {rem:?}");
    }
}

#[test]
fn split_vectors_match_and_are_exact_above_min_subnormal() {
    let mut truncated = 0;
    let mut overflow = 0;
    let mut three = 0;
    for line in SPLIT.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let value = parse_value(fields[1]);
        let result = value.split_binary64();
        if fields[2].starts_with("E:") {
            assert_eq!(token(result.map(|_| Wide2::ZERO)), fields[2], "{line}");
            overflow += 1;
            continue;
        }
        let split = result.unwrap();
        let n: usize = fields[2].parse().unwrap();
        assert_eq!(split.terms().len(), n, "{line}");
        let bits: Vec<String> = split
            .terms()
            .iter()
            .map(|t| format!("{:016x}", t.to_bits()))
            .collect();
        let expected: Vec<String> = if n == 0 {
            vec![]
        } else {
            fields[3..3 + n].iter().map(|s| s.to_string()).collect()
        };
        assert_eq!(bits, expected, "{line}");
        let flag = if n == 0 { fields[4] } else { fields[3 + n] };
        assert_eq!(split.truncated_below_min_subnormal(), flag == "1", "{line}");
        check_split_invariants(&value, &split);
        truncated += usize::from(split.truncated_below_min_subnormal());
        three += usize::from(n == 3);
    }
    assert!(
        truncated >= 50 && overflow >= 10 && three >= 100,
        "{truncated} {overflow} {three}"
    );
}

#[test]
fn split_is_exact_on_random_and_adversarial_values() {
    let mut rng = SplitMix64(0x5B17);
    for i in 0..50_000 {
        let e = match i % 3 {
            0 => (rng.next() % 2047) as i64 - 1023,
            1 => (rng.next() % 200) as i64 - 1150, // around 2^-1074
            _ => (rng.next() % 40) as i64 + 990,   // near the top
        };
        let sig = rand_sig(&mut rng);
        let value = w(rng.next() & 1 == 1, e, sig);
        match value.split_binary64() {
            Ok(split) => check_split_invariants(&value, &split),
            Err(err) => {
                assert_eq!(err, WideError::SplitOverflow);
                assert!(value.exponent() > 1023);
            }
        }
    }
}

#[test]
fn split_feeds_exact_accumulator_add_product() {
    // K u formed as Σ add_product(term, u) and rounded once equals the
    // correctly rounded product at p = 53 (normal range).
    let mut a53 = arith(53);
    let mut rng = SplitMix64(0xACC);
    for _ in 0..20_000 {
        let k = w(
            rng.next() & 1 == 1,
            (rng.next() % 1601) as i64 - 800,
            rand_sig(&mut rng),
        );
        let u = random_normal(&mut rng, 100);
        let mut acc = ExactAccumulator::new();
        assert!(!k.add_product_to(&mut acc, u).unwrap());
        let got = acc.round().unwrap();
        assert_eq!(f(got), a53.mul(&k, &f(u)).unwrap(), "{k:?} * {u:e}");
    }
    // Below 2^-1074 the split is truncated and the miss is below 2^-1074 |u|.
    let k = w(false, -1060, FULL);
    let mut acc = ExactAccumulator::new();
    assert!(k.add_product_to(&mut acc, 1.0).unwrap());
    let kept = acc.round().unwrap();
    let exact = k;
    let mut a = arith(128);
    let miss = a.sub(&exact, &f(kept)).unwrap();
    assert!(!miss.is_zero() && miss.exponent() < -1074);
}

// ---------------------------------------------------------------------------
// The included-angle arctangent (C1)
// ---------------------------------------------------------------------------

/// |got − (hi + lo)| in ulp of p, compared exactly against `limit_tenths`/10
/// ulp; also returns the ratio as f64 for reporting.
fn atan_error(got: &Wide2, hi: &Wide2, lo: &Wide2, p: u32, limit_tenths: u32) -> (bool, f64) {
    let mut a = arith(128);
    let d1 = a.sub(got, hi).unwrap();
    let err = a.sub(&d1, lo).unwrap().abs();
    let ulp_exponent = got.exponent() - i64::from(p) + 1;
    let limit = f(f64::from(limit_tenths) / 10.0)
        .mul_pow2(ulp_exponent)
        .unwrap();
    let within = err.cmp_value(&limit) != Ordering::Greater;
    let ulp = Wide2::ONE.mul_pow2(ulp_exponent).unwrap();
    let ratio = arith(53).div(&err, &ulp).unwrap();
    let ratio = ratio
        .split_binary64()
        .unwrap()
        .terms()
        .first()
        .copied()
        .unwrap_or(0.0);
    (within, ratio)
}

#[test]
fn arctangent_vectors_within_tolerance_and_bitwise_equal_to_the_emulation() {
    let tol_tenths = ATAN_TOLERANCE_ULPS * 10;
    assert!(tol_tenths <= ATAN_PROVED_BOUND_TENTH_ULPS);
    let mut worst: std::collections::BTreeMap<u32, (f64, String)> = Default::default();
    let count = |prefix: &str| {
        ATAN.lines()
            .filter(|l| {
                l.split_whitespace()
                    .nth(1)
                    .is_some_and(|t| t.starts_with(prefix))
            })
            .count()
    };
    assert_eq!(count("v1:"), 13);
    assert!(count("zero53:") >= 300 && count("zero128:") >= 40 && count("logsmall:") >= 200);
    assert!(count("pi53:") >= 8 && count("pi128:") >= 19 && count("lognearpi:") >= 200);
    assert!(count("farpi:") >= 6 && count("frac:1/2pi") >= 2 && count("rand128:") >= 1500);
    let mut referenced = 0;
    let mut refused = 0;
    for line in ATAN.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let tag = fields[1];
        let p: u32 = fields[2].parse().unwrap();
        let mut a = arith(p);
        let (result, rest) = match fields[0] {
            "angle" => {
                let s = parse_value(fields[3]);
                let c = parse_value(fields[4]);
                (a.included_angle(&s, &c), &fields[5..])
            }
            "atanpos" => {
                let t = parse_value(fields[3]);
                (a.atan_positive(&t), &fields[4..])
            }
            other => panic!("{other}"),
        };
        assert_eq!(token(result), rest[0], "{line}");
        let work = a.work();
        assert!(work.sqrt <= u64::from(ATAN_MAX_REDUCTIONS), "{line}");
        assert!(
            work.div <= 1 + u64::from(ATAN_MAX_REDUCTIONS + ATAN_MAX_TERMS),
            "{line}"
        );
        let Ok(got) = result else {
            refused += 1;
            continue;
        };
        assert!(got.fits_precision(p) && !got.is_sign_negative());
        let hi = parse_value(rest[1]);
        let lo = parse_value(rest[2]);
        let (within_tol, ratio) = atan_error(&got, &hi, &lo, p, tol_tenths);
        let (within_proof, _) = atan_error(&got, &hi, &lo, p, ATAN_PROVED_BOUND_TENTH_ULPS);
        assert!(within_proof, "{line}: {ratio} ulp beyond the proved bound");
        assert!(
            within_tol,
            "{line}: {ratio} ulp beyond the specified tolerance"
        );
        let entry = worst.entry(p).or_insert((0.0, String::new()));
        if ratio > entry.0 {
            *entry = (ratio, tag.to_string());
        }
        referenced += 1;
    }
    assert!(
        referenced >= 3000 && refused >= 10,
        "{referenced} {refused}"
    );
    println!("arctangent: {referenced} referenced vectors, {refused} refusals");
    for (p, (ratio, tag)) in &worst {
        println!("arctangent worst error at p = {p}: {ratio:.4} ulp ({tag})");
    }
}

#[test]
fn arctangent_domain_is_the_open_interval() {
    let mut a = arith(128);
    let one = f(1.0);
    assert_eq!(
        a.included_angle(&Wide2::ZERO, &one),
        Err(WideError::AngleDomain)
    );
    assert_eq!(
        a.included_angle(&Wide2::ZERO.neg(), &one),
        Err(WideError::AngleDomain)
    );
    assert_eq!(
        a.included_angle(&f(-1e-3), &one),
        Err(WideError::AngleDomain)
    );
    assert_eq!(
        a.included_angle(&f(1e-3), &f(-1.0)),
        Err(WideError::AngleDomain)
    );
    assert_eq!(
        a.included_angle(&f(1e-3), &f(-2.0)),
        Err(WideError::AngleDomain)
    );
    assert_eq!(a.atan_positive(&Wide2::ZERO), Err(WideError::AngleDomain));
    assert_eq!(a.atan_positive(&f(-2.0)), Err(WideError::AngleDomain));
    // φ(1, 0) = π/2 and φ(0+, 1) → 0+: both inside the interval.
    let half_pi = a.included_angle(&one, &Wide2::ZERO).unwrap();
    assert_eq!(half_pi.exponent(), 0);
    let tiny = a.included_angle(&f(f64::from_bits(1)), &one).unwrap();
    assert_eq!(tiny, f(f64::from_bits(1)));
}

#[test]
fn one_twentieth_threshold_is_decided_exactly() {
    // 1/20 = 2^-5 * 1.1001 1001 ... (binary) is not dyadic. Its 128-bit
    // rounding is 0xCCCC...CCCD (rounded up), so ...CCCC lies below 1/20.
    let pattern = 0xCCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCCu128;
    let above = w(false, -5, pattern + 1);
    let below = w(false, -5, pattern);
    assert_eq!(arith(128).div(&f(1.0), &f(20.0)).unwrap(), above);
    assert!(at_least_one_twentieth(&above));
    assert!(!at_least_one_twentieth(&below));
    assert!(
        at_least_one_twentieth(&f(0.05)),
        "binary64 0.05 is above 1/20"
    );
    assert!(!at_least_one_twentieth(&f(0.049_999_999_999_999_996)));
    assert!(at_least_one_twentieth(&f(1e300)) && !at_least_one_twentieth(&f(1e-300)));
}
