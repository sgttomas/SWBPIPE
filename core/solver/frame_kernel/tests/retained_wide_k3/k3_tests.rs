//! K3 tests for `structural::retained::wide::multi` (T3 D1 §4.11 and §7.4;
//! the brief's classes A–F).
//!
//! This file is compiled as the `tests` module of `wide/multi.rs` (a `#[path]`
//! module), so it runs in `frame_kernel`'s own unit-test suite and in hosted
//! CI. As a descendant of both `wide` and `multi`, it reaches K3a's
//! `WideArith` and the new generic core, which it instantiates at L = 2 (the
//! only place the core runs at L = 2) to check it bitwise against K3a.
//!
//! Expected values come from `gen_wide_k3_vectors.py` beside it (standard
//! library `fractions.Fraction`), from K3a's committed vectors and digests, from
//! hardware binary64 at p = 53, from `ExactAccumulator`, or from exact
//! identities checked at a wider width; none comes from the code under test.
use super::super::{Binary64Split, Wide2, WideArith, WorkCounter};
use super::*;
use crate::exact_sum::{ExactAccumulator, SumError};
use std::time::Instant;

const TARGETED_L4: &str = include_str!("targeted_l4.txt");
const TARGETED_L8: &str = include_str!("targeted_l8.txt");
const TARGETED_L16: &str = include_str!("targeted_l16.txt");
const CONVERSION: &str = include_str!("conversion.txt");
const EFT: &str = include_str!("eft.txt");
const DIFF_SAMPLE: &str = include_str!("differential_sample.txt");
const DIFF_MANIFEST: &str = include_str!("differential.txt");
const SHA256SUMS: &str = include_str!("SHA256SUMS");
// K3a's committed vectors and digests (read, never written; K3a's own tests
// keep checking them against K3a's path).
const K3A_TARGETED: &str = include_str!("../retained_wide/targeted.txt");
const K3A_DIFF_MANIFEST: &str = include_str!("../retained_wide/differential.txt");

const CLASS_PRECISIONS: [u32; 8] = [53, 128, 192, 256, 320, 512, 576, 1024];

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

fn top<const L: usize>() -> [u64; L] {
    let mut s = [0u64; L];
    s[L - 1] = 1 << 63;
    s
}

fn full<const L: usize>() -> [u64; L] {
    [u64::MAX; L]
}

/// Bits [lo, hi) set.
fn set_range(a: &mut [u64], lo: usize, hi: usize) {
    for i in lo..hi {
        set_bit(a, i);
    }
}

/// A value from raw parts, checked for the value model.
fn raw<const L: usize>(negative: bool, exponent: i64, significand: [u64; L]) -> Wide<L> {
    assert!(
        limbs_zero(&significand) || significand[L - 1] >> 63 == 1,
        "not normalized"
    );
    if limbs_zero(&significand) {
        return signed_zero(negative);
    }
    Wide {
        negative,
        exponent,
        significand,
    }
}

/// Parses a compact token (`Z+`, `Z-`, or `<sign><hex>p<e>` with the hex
/// digits left-aligned to 64L bits).
fn parse<const L: usize>(token: &str) -> Wide<L> {
    match token {
        "Z+" => return signed_zero(false),
        "Z-" => return signed_zero(true),
        _ => {}
    }
    let negative = match &token[..1] {
        "+" => false,
        "-" => true,
        other => panic!("bad sign {other} in {token}"),
    };
    let (hex, exponent) = token[1..].split_once('p').expect(token);
    assert!(!hex.is_empty() && hex.len() <= 16 * L, "{token}");
    let mut significand = [0u64; L];
    for k in 0..L {
        let start = (16 * k).min(hex.len());
        let end = (16 * k + 16).min(hex.len());
        let mut digits = hex[start..end].to_string();
        while digits.len() < 16 {
            digits.push('0');
        }
        significand[L - 1 - k] = u64::from_str_radix(&digits, 16).expect(token);
    }
    raw(negative, exponent.parse().expect(token), significand)
}

/// The compact token of a value (the generator's `W.token`).
fn tok<const L: usize>(w: &Wide<L>) -> String {
    if limbs_zero(&w.significand) {
        return if w.negative { "Z-" } else { "Z+" }.to_string();
    }
    let mut hex = String::new();
    for limb in w.significand.iter().rev() {
        hex.push_str(&format!("{limb:016x}"));
    }
    let sign = if w.negative { '-' } else { '+' };
    format!("{sign}{}p{}", hex.trim_end_matches('0'), w.exponent)
}

fn res_tok<const L: usize>(r: Result<Wide<L>, WideError>) -> String {
    match r {
        Ok(v) => tok(&v),
        Err(WideError::DivisionByZero) => "E:div0".into(),
        Err(WideError::NegativeSqrt) => "E:neg_sqrt".into(),
        Err(other) => format!("E:unexpected:{other:?}"),
    }
}

fn ctx<const L: usize>(p: u32) -> WideContext<L>
where
    Wide<L>: SupportedWidth,
{
    WideContext::<L>::new(p).unwrap()
}

fn lift<const L: usize>(x: f64) -> Wide<L>
where
    Wide<L>: SupportedWidth,
{
    Wide::<L>::from_f64(x).unwrap()
}

fn apply_ctx<const L: usize>(
    c: &mut WideContext<L>,
    op: &str,
    x: &Wide<L>,
    y: Option<&Wide<L>>,
) -> Result<Wide<L>, WideError>
where
    Wide<L>: SupportedWidth,
{
    match op {
        "add" => c.add(x, y.unwrap()),
        "sub" => c.sub(x, y.unwrap()),
        "mul" => c.mul(x, y.unwrap()),
        "div" => c.div(x, y.unwrap()),
        "sqrt" => c.sqrt(x),
        other => panic!("unknown op {other}"),
    }
}

/// The generic core at any core width (L = 2 here only).
fn apply_core<const L: usize>(
    p: u32,
    op: &str,
    x: &Wide<L>,
    y: Option<&Wide<L>>,
) -> Result<Wide<L>, WideError>
where
    Wide<L>: CoreWidth,
{
    match op {
        "add" => add_rounded(x, y.unwrap(), p),
        "sub" => add_rounded(x, &negated(y.unwrap()), p),
        "mul" => mul_rounded(x, y.unwrap(), p),
        "div" => div_rounded(x, y.unwrap(), p),
        "sqrt" => sqrt_rounded(x, p),
        other => panic!("unknown op {other}"),
    }
}

fn apply_k3a(
    a: &mut WideArith,
    op: &str,
    x: &Wide2,
    y: Option<&Wide2>,
) -> Result<Wide2, WideError> {
    match op {
        "add" => a.add(x, y.unwrap()),
        "sub" => a.sub(x, y.unwrap()),
        "mul" => a.mul(x, y.unwrap()),
        "div" => a.div(x, y.unwrap()),
        "sqrt" => a.sqrt(x),
        other => panic!("unknown op {other}"),
    }
}

/// (code, bits, relative precision bits) of a conversion outcome, as the
/// generator encodes it.
fn outcome_record(o: Binary64Outcome) -> (u8, u64, u64) {
    match o {
        Binary64Outcome::Normal(v) => (0, v.to_bits(), 0),
        Binary64Outcome::Subnormal {
            value,
            relative_precision,
        } => (1, value.to_bits(), relative_precision.to_bits()),
        Binary64Outcome::Underflow { negative } => (2, u64::from(negative) << 63, 0),
        Binary64Outcome::Overflow { negative } => (3, u64::from(negative) << 63, 0),
    }
}

fn outcome_token(o: Binary64Outcome) -> String {
    match o {
        Binary64Outcome::Normal(v) => format!("N:{:016x}", v.to_bits()),
        Binary64Outcome::Subnormal {
            value,
            relative_precision,
        } => format!(
            "S:{:016x}:{:016x}",
            value.to_bits(),
            relative_precision.to_bits()
        ),
        Binary64Outcome::Underflow { negative } => format!("U{}", if negative { '-' } else { '+' }),
        Binary64Outcome::Overflow { negative } => format!("O{}", if negative { '-' } else { '+' }),
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
}

#[test]
fn committed_vectors_match_their_recorded_sha256() {
    let files = [
        ("targeted_l4.txt", TARGETED_L4),
        ("targeted_l8.txt", TARGETED_L8),
        ("targeted_l16.txt", TARGETED_L16),
        ("conversion.txt", CONVERSION),
        ("eft.txt", EFT),
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

#[test]
fn token_round_trip_and_debug_scheme() {
    let w: Wide<4> = parse("+8p0");
    assert_eq!(w, Wide::<4>::ONE);
    assert_eq!(tok(&w), "+8p0");
    assert_eq!(
        format!("{w:?}"),
        format!("+8{}p0", "0".repeat(63)),
        "Debug of Wide<4>: 64 hex digits"
    );
    let v: Wide<16> = parse("-c0000000000000001p-3");
    assert_eq!(v.significand[15], 0xc000_0000_0000_0000);
    assert_eq!(v.significand[14], 0x1000_0000_0000_0000);
    assert_eq!(format!("{:?}", Wide::<8>::ZERO.neg()), "Z-");
    assert_eq!(format!("{:?}", lift::<16>(-3.0)).len(), 1 + 256 + 2);
}

/// FK's `[profile.test]` (T3 ROOT ruling on Q7) optimizes FK's own test
/// builds and keeps overflow checks and debug assertions on explicitly, so test
/// semantics do not change. Both are observable here (ROOT's ruling on the
/// checkpoint-C profile mutants P1 and P2).
#[test]
fn test_profile_keeps_overflow_checks_and_debug_assertions_on() {
    let overflowed = std::panic::catch_unwind(|| {
        let a = std::hint::black_box(u64::MAX);
        let b = std::hint::black_box(1u64);
        a + b
    });
    assert!(
        overflowed.is_err(),
        "overflow checks are off in FK's test build (u64::MAX + 1 wrapped)"
    );
    assert!(
        cfg!(debug_assertions),
        "debug assertions are off in FK's test build"
    );
}

// ---------------------------------------------------------------------------
// A. Nothing existing moves
// ---------------------------------------------------------------------------

#[test]
fn existing_wide_error_display_strings_are_unchanged() {
    // Published as `detail=retained arithmetic: {e}` through K-D5
    // (`formation_check.rs`); every existing string is pinned byte for byte.
    let cases = [
        (
            WideError::InvalidPrecision(7),
            "retained precision 7 is not supported",
        ),
        (
            WideError::NonFinite,
            "retained lift of a non-finite binary64 value",
        ),
        (
            WideError::ExponentRange,
            "retained exponent outside the supported range",
        ),
        (WideError::DivisionByZero, "retained division by zero"),
        (
            WideError::NegativeSqrt,
            "retained square root of a negative value",
        ),
        (WideError::NotNormalized, "retained value is not normalized"),
        (
            WideError::AngleDomain,
            "retained arctangent argument outside its domain",
        ),
        (
            WideError::ArctangentLimit,
            "retained arctangent internal limit reached",
        ),
        (
            WideError::SplitOverflow,
            "retained value too large for a binary64 split",
        ),
        (
            WideError::Accumulator(SumError::NonFinite),
            "retained split accumulation: exact sum operand is not finite",
        ),
        (
            WideError::Accumulator(SumError::AccumulatorOverflow),
            "retained split accumulation: exact sum accumulator overflow",
        ),
        (
            WideError::Accumulator(SumError::NonRepresentable),
            "retained split accumulation: exact sum is outside the binary64 range",
        ),
        // K3's one new variant.
        (
            WideError::OperandPrecision,
            "retained operand exceeds the working precision",
        ),
    ];
    for (e, text) in cases {
        assert_eq!(e.to_string(), text, "{e:?}");
    }
    assert_eq!(
        format!("{:?}", WideError::OperandPrecision),
        "OperandPrecision"
    );
}

#[test]
fn wide2_debug_tokens_and_work_counter_are_unchanged() {
    assert_eq!(
        format!("{:?}", Wide2::ONE),
        "+80000000000000000000000000000000p0"
    );
    assert_eq!(format!("{:?}", Wide2::ZERO), "Z+");
    assert_eq!(format!("{:?}", Wide2::ZERO.neg()), "Z-");
    assert_eq!(
        format!("{:?}", Wide2::from_f64(-3.0).unwrap()),
        "-c0000000000000000000000000000000p1"
    );
    let mut a = WideArith::new(128).unwrap();
    let x = Wide2::from_f64(2.0).unwrap();
    a.add(&x, &x).unwrap();
    a.mul(&x, &x).unwrap();
    let c: WorkCounter = a.work();
    assert_eq!((c.add, c.mul, c.rounded_operations()), (1, 1, 2));
    // The L = 2 conversion does not touch K3a's split.
    let split: Binary64Split = x.split_binary64().unwrap();
    assert_eq!(split.terms(), &[2.0]);
}

#[test]
fn l2_core_matches_k3a_on_k3a_targeted_vectors() {
    let mut checked = 0;
    for line in K3A_TARGETED.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let p: u32 = fields[2].parse().unwrap();
        let x: Wide2 = parse(fields[3]);
        if fields[0] == "eft" {
            let y: Wide2 = parse(fields[4]);
            let (s, e) = match fields[1] {
                "twosum" => two_sum_rounded(&x, &y, p),
                "twoprod" => two_product_rounded(&x, &y, p),
                other => panic!("{other}"),
            }
            .unwrap();
            // K3a's vectors hold s = fl_p and the exact error e; the new core's
            // (s, e) is the same pair (e = +0 when exact).
            assert_eq!(format!("{s:?}"), fields[5], "{line}");
            let expected_e: Wide2 = parse(fields[6]);
            if expected_e.is_zero() {
                assert!(e.is_zero() && !e.is_sign_negative(), "{line}: {e:?}");
            } else {
                assert_eq!(format!("{e:?}"), fields[6], "{line}");
            }
        } else {
            let y = (fields[4] != "-").then(|| parse::<2>(fields[4]));
            let core = apply_core(p, fields[1], &x, y.as_ref());
            let mut arith = WideArith::new(p).unwrap();
            let k3a = apply_k3a(&mut arith, fields[1], &x, y.as_ref());
            assert_eq!(core, k3a, "{line}");
            let expected = match &k3a {
                Ok(v) => format!("{v:?}"),
                Err(WideError::DivisionByZero) => "E:div0".into(),
                Err(WideError::NegativeSqrt) => "E:neg_sqrt".into(),
                Err(e) => panic!("{e:?}"),
            };
            assert_eq!(expected, fields[5], "{line}");
        }
        checked += 1;
    }
    assert!(checked >= 2000, "{checked}");
}

// K3a's differential operand rules, ported from K3a's test for this check.
fn k3a_ones(w: u64) -> u128 {
    if w >= 128 {
        u128::MAX
    } else {
        (1u128 << w) - 1
    }
}

fn k3a_rand_sig(rng: &mut SplitMix64) -> u128 {
    let hi = rng.next();
    let lo = rng.next();
    let sel = rng.next();
    let t = 1u128 << 127;
    let mut sig = ((u128::from(hi) << 64) | u128::from(lo)) | t;
    match sel % 8 {
        0 => {
            let nbits = 1 + (sel >> 8) % 128;
            sig = (sig >> (128 - nbits)) << (128 - nbits);
        }
        1 => {
            let w = 1 + (sel >> 8) % 128;
            sig |= k3a_ones(w) << (128 - w);
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

fn wide2(negative: bool, exponent: i64, sig: u128) -> Wide2 {
    Wide2::from_parts(negative, exponent, [sig as u64, (sig >> 64) as u64]).unwrap()
}

fn k3a_gen_operands(rng: &mut SplitMix64) -> (u8, Wide2, Wide2) {
    let t = 1u128 << 127;
    let r = rng.next();
    let op = (r % 5) as u8;
    let mode = (r >> 3) % 8;
    let mut a_neg = (r >> 6) & 1 == 1;
    let b_neg = (r >> 7) & 1 == 1;
    let ea = ((r >> 8) % 4001) as i64 - 2000;
    let zero_a = (r >> 20) % 64 == 0;
    let zero_b = (r >> 26) % 64 == 0;
    let mut a_sig = k3a_rand_sig(rng);
    let (b_sig, eb) = match mode {
        0..=3 => {
            let eb = ea + (rng.next() % 301) as i64 - 150;
            (k3a_rand_sig(rng), eb)
        }
        4 => {
            let delta = u128::from(rng.next() % 16);
            let b = if a_sig <= u128::MAX - delta {
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
            ((a_sig ^ (x & k3a_ones(width))) | t, ea)
        }
        6 => {
            let width = rng.next() % 128;
            let x = rng.next();
            a_sig = u128::MAX ^ (u128::from(x) & k3a_ones(width));
            let width2 = rng.next() % 128;
            let y = rng.next();
            (t | (u128::from(y) & k3a_ones(width2)), ea + 1)
        }
        _ => {
            let eb = ea + (rng.next() % 3001) as i64 - 1500;
            (k3a_rand_sig(rng), eb)
        }
    };
    if op == 4 {
        a_neg = false;
    }
    let a = if zero_a {
        signed_zero(a_neg)
    } else {
        wide2(a_neg, ea, a_sig)
    };
    let b = if zero_b {
        signed_zero(b_neg)
    } else {
        wide2(b_neg, eb, b_sig)
    };
    (op, a, b)
}

const OP_NAMES: [&str; 5] = ["add", "sub", "mul", "div", "sqrt"];

/// (seed, count, chunk, sha256, chunk digests) of a named stream in a manifest.
fn manifest_stream(
    manifest: &str,
    kind: &str,
    name: &str,
    seed_field: usize,
) -> (u64, usize, usize, String, Vec<String>) {
    let mut spec = None;
    let mut chunks = Vec::new();
    for line in manifest.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields[0] == kind && fields[1] == name {
            let at = |key: &str| {
                let i = fields.iter().position(|f| *f == key).unwrap();
                fields[i + 1]
            };
            assert_eq!(fields[seed_field - 1], "seed");
            spec = Some((
                u64::from_str_radix(fields[seed_field], 16).unwrap(),
                at("count").parse().unwrap(),
                at("chunk").parse().unwrap(),
                at("sha256").to_string(),
            ));
        }
        if fields[0] == "chunk" && fields[1] == name {
            assert_eq!(fields[2].parse::<usize>().unwrap(), chunks.len());
            chunks.push(fields[3].to_string());
        }
    }
    let (seed, count, chunk, sha) = spec.unwrap_or_else(|| panic!("no stream {name}"));
    assert_eq!(chunks.len() * chunk, count, "{name}");
    (seed, count, chunk, sha, chunks)
}

fn k3a_stream_through_the_l2_core(name: &str, mixed: bool) {
    let (seed, count, chunk_len, sha, chunks) =
        manifest_stream(K3A_DIFF_MANIFEST, "stream", name, 3);
    let mut rng = SplitMix64(seed);
    let mut total = Sha256::new();
    let mut chunk = Sha256::new();
    let mut k3a: Vec<Option<WideArith>> = vec![None; 129];
    let mut record = Vec::with_capacity(28);
    for i in 0..count {
        let p = if mixed {
            2 + (rng.next() % 127) as u32
        } else {
            128
        };
        let (op, a, b) = k3a_gen_operands(&mut rng);
        let arith = k3a[p as usize].get_or_insert_with(|| WideArith::new(p).unwrap());
        let reference = apply_k3a(arith, OP_NAMES[op as usize], &a, Some(&b));
        let core = apply_core(p, OP_NAMES[op as usize], &a, Some(&b));
        assert_eq!(core, reference, "{name} record {i}: {a:?} {b:?}");
        record.clear();
        record.push(op);
        if mixed {
            record.push(p as u8);
        }
        match core {
            Ok(v) => {
                record.push(0);
                record.push(u8::from(v.negative));
                record.extend_from_slice(&v.exponent.to_le_bytes());
                record.extend_from_slice(&v.significand[0].to_le_bytes());
                record.extend_from_slice(&v.significand[1].to_le_bytes());
            }
            Err(WideError::DivisionByZero) => {
                record.push(1);
                record.extend_from_slice(&[0u8; 25]);
            }
            Err(e) => panic!("{name} record {i}: unexpected {e:?}"),
        }
        total.update(&record);
        chunk.update(&record);
        if (i + 1) % chunk_len == 0 {
            let done = std::mem::replace(&mut chunk, Sha256::new());
            assert_eq!(done.hex(), chunks[i / chunk_len], "{name} chunk");
        }
    }
    assert_eq!(total.hex(), sha, "{name} stream digest from the L = 2 core");
}

#[test]
fn l2_core_matches_k3a_on_k3a_p128_differential_and_its_digests() {
    k3a_stream_through_the_l2_core("p128", false);
}

#[test]
fn l2_core_matches_k3a_on_k3a_mixed_differential_and_its_digests() {
    k3a_stream_through_the_l2_core("mixed", true);
}

// ---------------------------------------------------------------------------
// B. The arithmetic at L = 4, 8 and 16
// ---------------------------------------------------------------------------

fn random_normal(rng: &mut SplitMix64, exp_span: u64) -> f64 {
    let r = rng.next();
    let sign = r >> 63;
    let exponent = 1023 + (r >> 20) % (2 * exp_span + 1) - exp_span;
    let fraction = rng.next() & ((1u64 << 52) - 1);
    f64::from_bits((sign << 63) | (exponent << 52) | fraction)
}

fn p53_hardware<const L: usize>()
where
    Wide<L>: SupportedWidth,
{
    let mut c = ctx::<L>(53);
    let mut rng = SplitMix64(0x5053_3533 ^ L as u64);
    let check = |c: &mut WideContext<L>, op: &str, x: f64, y: f64, hw: f64| {
        assert!(hw.is_normal() || hw == 0.0, "{op} {x:e} {y:e} -> {hw:e}");
        let got = apply_ctx(c, op, &lift(x), Some(&lift(y))).unwrap();
        assert_eq!(got, lift(hw), "L = {L}: {op} {x:e} {y:e}: {got:?}");
    };
    for i in 0..100_000u64 {
        let x = random_normal(&mut rng, 400);
        let y = match i % 8 {
            0 => -x,
            1 => f64::from_bits(x.to_bits() ^ 1),
            2 => -f64::from_bits(x.to_bits().wrapping_add(1 + (rng.next() & 7))),
            3 => x * 0.5,
            _ => random_normal(&mut rng, 400),
        };
        check(&mut c, "add", x, y, x + y);
        check(&mut c, "sub", x, y, x - y);
        check(&mut c, "mul", x, y, x * y);
        check(&mut c, "div", x, y, x / y);
        let s = x.abs();
        assert_eq!(
            c.sqrt(&lift(s)).unwrap(),
            lift(s.sqrt()),
            "L = {L}: sqrt {s:e}"
        );
    }
    let e = f64::EPSILON;
    for (x, y) in [
        (1.0, e / 2.0),
        (1.0 + e, e / 2.0),
        (1.0 + 2.0 * e, e / 2.0),
        (-(1.0 + e), -e / 2.0),
        (2.0 - e, e / 2.0),
        (1.0, e / 2.0 + e * e),
    ] {
        check(&mut c, "add", x, y, x + y);
    }
}

#[test]
fn p53_matches_hardware_binary64_bitwise_at_l4() {
    p53_hardware::<4>();
}

#[test]
fn p53_matches_hardware_binary64_bitwise_at_l8() {
    p53_hardware::<8>();
}

#[test]
fn p53_matches_hardware_binary64_bitwise_at_l16() {
    p53_hardware::<16>();
}

/// The targeted precisions of width L (the generator's `precisions`).
fn targeted_precisions(limbs: usize) -> Vec<u32> {
    let nb = 64 * limbs as u32;
    let mut ps: Vec<u32> = CLASS_PRECISIONS
        .iter()
        .copied()
        .filter(|&p| p <= nb)
        .collect();
    for k in 1..=limbs as u32 {
        for d in [-1i64, 0, 1] {
            let p = (64 * k) as i64 + d;
            if 2 <= p && p <= nb as i64 {
                ps.push(p as u32);
            }
        }
    }
    ps.sort_unstable();
    ps.dedup();
    ps
}

fn targeted_classes<const L: usize>(text: &str)
where
    Wide<L>: SupportedWidth,
{
    use std::collections::BTreeMap;
    let mut counts: BTreeMap<(String, String, u32), usize> = BTreeMap::new();
    for line in text.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let p: u32 = fields[2].parse().unwrap();
        let mut c = ctx::<L>(p);
        let x = parse::<L>(fields[3]);
        let y = (fields[4] != "-").then(|| parse::<L>(fields[4]));
        let got = res_tok(apply_ctx(&mut c, fields[1], &x, y.as_ref()));
        assert_eq!(got, fields[5], "L = {L}: {line}");
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
    let nb = 64 * L as u32;
    let ps = targeted_precisions(L);
    // every limb boundary 64k ± 1, 64k and every class precision ≤ 64L
    assert_eq!(ps.len(), {
        let mut n = 3 * L - 1;
        n += [53u32, 320, 576]
            .iter()
            .filter(|&&p| p <= nb && p % 64 != 0)
            .count();
        n
    });
    for &p in &ps {
        for op in ["add", "sub", "mul"] {
            assert!(has("tie", op, p) > 0, "L = {L}: tie {op} {p}");
        }
        if p + 1 < nb {
            assert!(has("tie", "div", p) > 0, "L = {L}: tie div {p}");
        }
        if 2 * (p + 1) <= nb {
            assert!(has("tie", "sqrt", p) > 0, "L = {L}: tie sqrt {p}");
        }
        for op in ["add", "sub", "mul", "div", "sqrt"] {
            assert!(has("carry", op, p) > 0, "L = {L}: carry {op} {p}");
        }
        for op in ["add", "sub", "mul", "div", "sqrt"] {
            assert!(has("cancel", op, p) > 0, "L = {L}: cancel {op} {p}");
        }
        assert!(has("exactdiv", "div", p) > 0 && has("nearexactdiv", "div", p) > 0);
        assert!(has("perfectsq", "sqrt", p) > 0 && has("nearsq", "sqrt", p) > 0);
        for op in ["add", "sub", "mul", "div", "sqrt"] {
            assert!(has("sticky", op, p) > 0, "L = {L}: sticky {op} {p}");
        }
    }
}

#[test]
fn targeted_hard_classes_match_the_fraction_oracle_at_l4() {
    targeted_classes::<4>(TARGETED_L4);
}

#[test]
fn targeted_hard_classes_match_the_fraction_oracle_at_l8() {
    targeted_classes::<8>(TARGETED_L8);
}

#[test]
fn targeted_hard_classes_match_the_fraction_oracle_at_l16() {
    targeted_classes::<16>(TARGETED_L16);
}

fn carry_through_every_limb<const L: usize>()
where
    Wide<L>: SupportedWidth,
{
    let nb = 64 * L as u32;
    let mut c = ctx::<L>(nb);
    let ones = raw(false, 0, full::<L>());
    // All ones plus one unit of the last place: an exact carry through every
    // limb, to 2^1.
    let unit = raw(false, 1 - nb as i64, top::<L>());
    assert_eq!(c.add(&ones, &unit).unwrap(), raw(false, 1, top::<L>()));
    // All ones plus half an ulp at p = 64L: a tie on an odd kept part, which
    // rounds up through every limb.
    let half = raw(false, -(nb as i64), top::<L>());
    assert_eq!(c.add(&ones, &half).unwrap(), raw(false, 1, top::<L>()));
    // The same, one limb short of the full width: rounds up at p = 64L − 64.
    let mut c2 = ctx::<L>(nb - 64);
    assert_eq!(
        c2.add(&ones, &Wide::<L>::ZERO).unwrap(),
        raw(false, 1, top::<L>())
    );
    // Borrow through every limb: 2^1 − ulp = all ones.
    assert_eq!(c.sub(&raw(false, 1, top::<L>()), &unit).unwrap(), ones);
}

#[test]
fn carry_and_borrow_run_through_every_limb_at_every_width() {
    carry_through_every_limb::<4>();
    carry_through_every_limb::<8>();
    carry_through_every_limb::<16>();
}

fn exponent_extremes<const L: usize>()
where
    Wide<L>: SupportedWidth,
{
    let big = raw(false, EXPONENT_LIMIT, top::<L>());
    let big_ones = raw(false, EXPONENT_LIMIT, full::<L>());
    let tiny = raw(false, -EXPONENT_LIMIT, top::<L>());
    let range = Err(WideError::ExponentRange);
    let nb = 64 * L as u32;
    for p in [53, nb] {
        let mut c = ctx::<L>(p);
        let two = lift::<L>(2.0);
        assert_eq!(c.mul(&big, &big), range);
        assert_eq!(c.mul(&big, &two), range);
        assert_eq!(c.add(&big, &big), range);
        assert_eq!(c.div(&big, &lift(0.5)), range);
        assert_eq!(c.mul(&tiny, &tiny), range);
        assert_eq!(c.div(&tiny, &big), range);
        assert_eq!(c.div(&big, &tiny), range);
        assert_eq!(c.div(&tiny, &two), range);
        let mut tiny2 = top::<L>();
        tiny2[0] |= 1;
        assert_eq!(c.sub(&raw(false, -EXPONENT_LIMIT, tiny2), &tiny), range);
        assert_eq!(c.mul(&big, &lift(1.0)).unwrap(), big);
        assert_eq!(c.mul(&tiny, &lift(-1.0)).unwrap(), tiny.neg());
        assert!(c.sqrt(&big).is_ok() && c.sqrt(&tiny).is_ok());
        assert_eq!(c.from_integer(false, &[1], i64::MAX), range);
        assert_eq!(c.from_integer(false, &[1], i64::MIN), range);
        assert_eq!(c.from_integer(false, &[3], EXPONENT_LIMIT), range);
        assert!(c.from_integer(false, &[3], EXPONENT_LIMIT - 1).is_ok());
        assert!(c.from_integer(false, &[1], EXPONENT_LIMIT).is_ok());
    }
    // Rounding up across the limit (p < 64L) is refused as well.
    assert_eq!(ctx::<L>(53).add(&big_ones, &Wide::<L>::ZERO), range);
    assert_eq!(ctx::<L>(53).round(&big_ones), range);
    assert_eq!(
        ctx::<L>(nb).add(&big_ones, &Wide::<L>::ZERO).unwrap(),
        big_ones
    );
    assert_eq!(big.mul_pow2(1), range);
    assert_eq!(tiny.mul_pow2(-1), range);
    assert_eq!(big.mul_pow2(i64::MAX), range);
    assert_eq!(tiny.mul_pow2(i64::MIN), range);
    assert_eq!(Wide::<L>::ZERO.mul_pow2(i64::MAX).unwrap(), Wide::<L>::ZERO);
    for e in [EXPONENT_LIMIT + 1, -EXPONENT_LIMIT - 1, i64::MAX, i64::MIN] {
        assert_eq!(Wide::<L>::from_parts(false, e, top::<L>()), range);
    }
    // The conversion never wraps: overflow and underflow at the limits.
    assert_eq!(
        big.to_binary64(),
        Binary64Outcome::Overflow { negative: false }
    );
    assert_eq!(
        tiny.neg().to_binary64(),
        Binary64Outcome::Underflow { negative: true }
    );
}

#[test]
fn exponent_extremes_are_refused_never_wrapped_at_every_width() {
    exponent_extremes::<4>();
    exponent_extremes::<8>();
    exponent_extremes::<16>();
}

fn zeros_lift_parts_and_precision<const L: usize>()
where
    Wide<L>: SupportedWidth,
{
    let nb = 64 * L as u32;
    for p in [0, 1, nb + 1, u32::MAX] {
        assert_eq!(
            WideContext::<L>::new(p).unwrap_err(),
            WideError::InvalidPrecision(p)
        );
    }
    for p in [2, 53, nb - 1, nb] {
        assert_eq!(ctx::<L>(p).precision(), p);
    }
    for p in [53, nb] {
        let mut c = ctx::<L>(p);
        let (pz, nz) = (Wide::<L>::ZERO, Wide::<L>::ZERO.neg());
        assert_eq!(c.add(&pz, &nz).unwrap(), pz);
        assert_eq!(c.add(&nz, &nz).unwrap(), nz);
        assert_eq!(c.sub(&nz, &pz).unwrap(), nz);
        assert_eq!(c.sub(&pz, &pz).unwrap(), pz);
        let mut s = top::<L>();
        s[0] |= 12345;
        let x = raw(true, 17, s);
        assert_eq!(c.add(&x, &x.neg()).unwrap(), pz);
        assert_eq!(c.sub(&x, &x).unwrap(), pz);
        assert_eq!(c.mul(&nz, &lift(3.0)).unwrap(), nz);
        assert_eq!(c.mul(&nz, &lift(-3.0)).unwrap(), pz);
        assert_eq!(c.div(&nz, &lift(-2.0)).unwrap(), pz);
        assert_eq!(c.sqrt(&nz).unwrap(), nz);
        assert_eq!(c.div(&lift(1.0), &pz), Err(WideError::DivisionByZero));
        assert_eq!(c.div(&pz, &nz), Err(WideError::DivisionByZero));
        assert_eq!(c.sqrt(&lift(-1.0)), Err(WideError::NegativeSqrt));
        assert_eq!(c.round(&nz).unwrap(), nz);
        assert_eq!(c.from_integer(true, &[0, 0, 0], -2148).unwrap(), pz);
        assert!(!c
            .from_integer(true, &[0; 68], 5)
            .unwrap()
            .is_sign_negative());
    }
    // The lift is exact for normal, subnormal and signed zero.
    assert_eq!(lift::<L>(1.0), Wide::<L>::ONE);
    assert_eq!(lift::<L>(0.0), Wide::<L>::ZERO);
    assert_eq!(lift::<L>(-0.0), Wide::<L>::ZERO.neg());
    assert_eq!(
        lift::<L>(f64::from_bits(1)).parts(),
        (false, -1074, top::<L>())
    );
    assert_eq!(lift::<L>(f64::MIN_POSITIVE).exponent(), -1022);
    let mut max = [0u64; L];
    max[L - 1] = u64::MAX << 11;
    assert_eq!(lift::<L>(f64::MAX).parts(), (false, 1023, max));
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(Wide::<L>::from_f64(bad), Err(WideError::NonFinite));
    }
    // from_parts validates normalization and the zero exponent.
    let mut low = [0u64; L];
    low[0] = 1;
    assert_eq!(
        Wide::<L>::from_parts(false, 0, low),
        Err(WideError::NotNormalized)
    );
    assert_eq!(
        Wide::<L>::from_parts(false, 5, [0; L]),
        Err(WideError::NotNormalized)
    );
    assert!(Wide::<L>::from_parts(true, EXPONENT_LIMIT, top::<L>()).is_ok());
    // Comparisons and the precision test.
    let (a, b) = (lift::<L>(-2.0), lift::<L>(3.0));
    assert_eq!(a.cmp_value(&b), Ordering::Less);
    assert_eq!(b.cmp_value(&a), Ordering::Greater);
    assert_eq!(
        Wide::<L>::ZERO.cmp_value(&Wide::<L>::ZERO.neg()),
        Ordering::Equal
    );
    assert_eq!(a.abs().cmp_value(&lift(2.0)), Ordering::Equal);
    assert!(lift::<L>(3.0).fits_precision(2) && !lift::<L>(5.0).fits_precision(2));
    assert!(raw::<L>(false, 0, full::<L>()).fits_precision(nb));
    assert!(!raw::<L>(false, 0, full::<L>()).fits_precision(nb - 1));
}

#[test]
fn signs_of_zero_the_lift_parts_and_the_precision_range_at_every_width() {
    zeros_lift_parts_and_precision::<4>();
    zeros_lift_parts_and_precision::<8>();
    zeros_lift_parts_and_precision::<16>();
}

// ---------------------------------------------------------------------------
// D. The seeded differentials (regenerated from the recorded seeds)
// ---------------------------------------------------------------------------

fn rand_bits<const L: usize>(rng: &mut SplitMix64) -> [u64; L] {
    let mut v = [0u64; L];
    for k in 0..L {
        v[L - 1 - k] = rng.next();
    }
    v
}

/// Mirrors `rand_sig` in gen_wide_k3_vectors.py call for call.
fn rand_sig<const L: usize>(rng: &mut SplitMix64) -> [u64; L] {
    let nb = 64 * L as u64;
    let mut sig = rand_bits::<L>(rng);
    sig[L - 1] |= 1 << 63;
    let sel = rng.next();
    match sel % 8 {
        0 => {
            let nbits = 1 + (sel >> 8) % nb;
            clear_below(&mut sig, (nb - nbits) as usize);
        }
        1 => {
            let w = 1 + (sel >> 8) % nb;
            set_range(&mut sig, (nb - w) as usize, nb as usize);
        }
        2 => {
            let nbits = 1 + (sel >> 8) % (nb - 1);
            let low = (sel >> 24) % (nb - nbits);
            clear_below(&mut sig, (nb - nbits) as usize);
            set_bit(&mut sig, low as usize);
        }
        3 => {
            let k = 1 + (sel >> 8) % L as u64;
            let w = (64 * k as i64 + ((sel >> 16) % 3) as i64 - 1).clamp(1, nb as i64) as usize;
            sig = [0; L];
            set_range(&mut sig, nb as usize - w, nb as usize);
        }
        _ => {}
    }
    sig
}

/// Mirrors `gen_operands` in gen_wide_k3_vectors.py call for call.
fn gen_operands<const L: usize>(rng: &mut SplitMix64) -> (u8, Wide<L>, Wide<L>) {
    let nb = 64 * L as u64;
    let r = rng.next();
    let op = (r % 5) as u8;
    let mode = (r >> 3) % 8;
    let mut a_neg = (r >> 6) & 1 == 1;
    let b_neg = (r >> 7) & 1 == 1;
    let ea = ((r >> 8) % 4001) as i64 - 2000;
    let zero_a = (r >> 20) % 64 == 0;
    let zero_b = (r >> 26) % 64 == 0;
    let mut a_sig = rand_sig::<L>(rng);
    let (b_sig, eb) = match mode {
        0..=3 => {
            let eb = ea + (rng.next() % 301) as i64 - 150;
            (rand_sig::<L>(rng), eb)
        }
        4 => {
            let delta = rng.next() % 16;
            let mut d = [0u64; L];
            d[0] = delta;
            let mut b = a_sig;
            if add_in_place(&mut b, &d) {
                b = a_sig;
                sub_in_place(&mut b, &d);
            }
            (b, ea)
        }
        5 => {
            let w = rng.next() % (nb + 1);
            let mut x = rand_bits::<L>(rng);
            clear_from(&mut x, w as usize);
            let mut b = a_sig;
            for (bi, xi) in b.iter_mut().zip(x) {
                *bi ^= xi;
            }
            b[L - 1] |= 1 << 63;
            (b, ea)
        }
        6 => {
            let w = rng.next() % nb;
            let mut x = rand_bits::<L>(rng);
            clear_from(&mut x, w as usize);
            for (ai, xi) in a_sig.iter_mut().zip(x) {
                *ai = u64::MAX ^ xi;
            }
            let w2 = rng.next() % nb;
            let mut y = rand_bits::<L>(rng);
            clear_from(&mut y, w2 as usize);
            y[L - 1] |= 1 << 63;
            (y, ea + 1)
        }
        _ => {
            let span = nb + 400;
            let eb = ea + (rng.next() % (2 * span + 1)) as i64 - span as i64;
            (rand_sig::<L>(rng), eb)
        }
    };
    if op == 4 {
        a_neg = false;
    }
    let a = if zero_a {
        signed_zero(a_neg)
    } else {
        raw(a_neg, ea, a_sig)
    };
    let b = if zero_b {
        signed_zero(b_neg)
    } else {
        raw(b_neg, eb, b_sig)
    };
    (op, a, b)
}

fn mixed_choices(limbs: usize) -> Vec<u32> {
    let nb = 64 * limbs as u32;
    let mut out: Vec<u32> = CLASS_PRECISIONS
        .iter()
        .copied()
        .filter(|&p| p <= nb)
        .collect();
    for k in 1..=limbs as u32 {
        for d in [-1i64, 0, 1] {
            let p = (64 * k) as i64 + d;
            if 2 <= p && p <= nb as i64 {
                out.push(p as u32);
            }
        }
    }
    out
}

fn mixed_p(rng: &mut SplitMix64, limbs: usize, choices: &[u32]) -> u32 {
    let r = rng.next();
    if r % 2 == 0 {
        2 + ((r >> 1) % (64 * limbs as u64 - 1)) as u32
    } else {
        choices[((r >> 1) % choices.len() as u64) as usize]
    }
}

fn run_differential<const L: usize>(name: &str, fixed_p: Option<u32>, min_count: usize)
where
    Wide<L>: SupportedWidth,
{
    let started = Instant::now();
    let (seed, count, chunk_len, sha, chunks) = manifest_stream(DIFF_MANIFEST, "stream", name, 7);
    assert!(count >= min_count, "{name}: {count} operations");
    let samples: Vec<Vec<&str>> = DIFF_SAMPLE
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .filter(|f| f[0] == name)
        .collect();
    assert_eq!(samples.len(), 1000, "{name}");
    let nb = 64 * L as u32;
    let choices = mixed_choices(L);
    let mut rng = SplitMix64(seed);
    let mut total = Sha256::new();
    let mut chunk = Sha256::new();
    let mut contexts: Vec<Option<WideContext<L>>> = vec![None; nb as usize + 1];
    let mut record = Vec::with_capacity(12 + 8 * L);
    let mut per_op = [0usize; 5];
    let mut precisions_seen = std::collections::BTreeSet::new();
    for i in 0..count {
        let p = match fixed_p {
            Some(p) => p,
            None => mixed_p(&mut rng, L, &choices),
        };
        let (op, a, b) = gen_operands::<L>(&mut rng);
        per_op[op as usize] += 1;
        precisions_seen.insert(p);
        let c = contexts[p as usize].get_or_insert_with(|| ctx::<L>(p));
        let result = apply_ctx(c, OP_NAMES[op as usize], &a, Some(&b));
        if let Some(sample) = samples.get(i) {
            assert_eq!(sample[1].parse::<usize>().unwrap(), i);
            assert_eq!(
                (
                    sample[2],
                    sample[3].parse::<u32>().unwrap(),
                    sample[4],
                    sample[5]
                ),
                (OP_NAMES[op as usize], p, tok(&a).as_str(), tok(&b).as_str()),
                "{name} operands {i} (generator port)"
            );
            assert_eq!(res_tok(result), sample[6], "{name} record {i}");
        }
        record.clear();
        record.push(op);
        if fixed_p.is_none() {
            record.extend_from_slice(&(p as u16).to_le_bytes());
        }
        match result {
            Ok(v) => {
                record.push(0);
                record.push(u8::from(v.negative));
                record.extend_from_slice(&v.exponent.to_le_bytes());
                for limb in v.significand {
                    record.extend_from_slice(&limb.to_le_bytes());
                }
            }
            Err(WideError::DivisionByZero) => {
                record.push(1);
                record.extend(std::iter::repeat_n(0u8, 9 + 8 * L));
            }
            Err(e) => panic!("{name} record {i}: unexpected {e:?}"),
        }
        total.update(&record);
        chunk.update(&record);
        if (i + 1) % chunk_len == 0 {
            let done = std::mem::replace(&mut chunk, Sha256::new());
            let k = i / chunk_len;
            assert_eq!(done.hex(), chunks[k], "{name} chunk {k}");
        }
    }
    assert_eq!(total.hex(), sha, "{name} stream");
    for (op, n) in per_op.iter().enumerate() {
        assert!(*n > count / 6, "{name}: {} {}", OP_NAMES[op], n);
    }
    if fixed_p.is_none() {
        for p in &choices {
            assert!(precisions_seen.contains(p), "{name}: precision {p} unused");
        }
    }
    println!(
        "{name}: {count} operations at L = {L}, debug wall time {:.1} s",
        started.elapsed().as_secs_f64()
    );
}

#[test]
fn seeded_fraction_differential_at_p256_l4() {
    run_differential::<4>("p256", Some(256), 1_000_000);
}

#[test]
fn seeded_fraction_differential_at_p512_l8() {
    run_differential::<8>("p512", Some(512), 1_000_000);
}

#[test]
fn seeded_fraction_differential_at_p1024_l16() {
    run_differential::<16>("p1024", Some(1024), 1_000_000);
}

#[test]
fn seeded_fraction_differential_at_mixed_precision_l4() {
    run_differential::<4>("mixed4", None, 200_000);
}

#[test]
fn seeded_fraction_differential_at_mixed_precision_l8() {
    run_differential::<8>("mixed8", None, 200_000);
}

#[test]
fn seeded_fraction_differential_at_mixed_precision_l16() {
    run_differential::<16>("mixed16", None, 200_000);
}

// ---------------------------------------------------------------------------
// C. The conversion to binary64, at every width including L = 2
// ---------------------------------------------------------------------------

fn check_conversion_line<const L: usize>(line: &str, fields: &[&str]) {
    let value = parse::<L>(fields[3]);
    assert_eq!(outcome_token(value.to_binary64()), fields[4], "{line}");
}

#[test]
fn conversion_boundary_vectors_match_the_fraction_oracle() {
    use std::collections::BTreeMap;
    let mut tags: BTreeMap<(usize, String), usize> = BTreeMap::new();
    for line in CONVERSION.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(fields[0], "conv");
        let limbs: usize = fields[1].parse().unwrap();
        match limbs {
            2 => check_conversion_line::<2>(line, &fields),
            4 => check_conversion_line::<4>(line, &fields),
            8 => check_conversion_line::<8>(line, &fields),
            16 => check_conversion_line::<16>(line, &fields),
            other => panic!("width {other}"),
        }
        *tags.entry((limbs, fields[2].to_string())).or_default() += 1;
    }
    for limbs in [2, 4, 8, 16] {
        for tag in [
            "zero",
            "min_subnormal",
            "max_subnormal",
            "min_normal",
            "max",
            "up_to_normal",
            "below_up_to_normal",
            "half_min_subnormal",
            "half_min_plus_tail",
            "half_min_minus_tail",
            "subnormal_tie",
            "subnormal_tie_far_sticky",
            "subnormal_tie_far_below",
            "double_rounding_trap",
            "below_midpoint",
            "midpoint",
            "above_midpoint",
            "overflow",
            "exponent_limit",
            "normal_tie",
            "normal_tie_far_sticky",
            "normal_carry",
        ] {
            assert!(
                tags.contains_key(&(limbs, tag.to_string())),
                "L = {limbs}: {tag}"
            );
        }
    }
}

fn boundary_outcomes<const L: usize>() {
    let nb = 64 * L as i64;
    let v = |negative: bool, exponent: i64, sig: [u64; L]| raw::<L>(negative, exponent, sig);
    let t = top::<L>();
    let mut t1 = t;
    t1[0] |= 1;
    let f = full::<L>();
    let normal = |x: f64| Binary64Outcome::Normal(x);
    // The largest subnormal and the smallest normal.
    let mut max_sub = [0u64; L];
    max_sub[L - 1] = ((1u64 << 52) - 1) << 12;
    let max_sub_value = f64::from_bits((1u64 << 52) - 1);
    // 2^-1075 / max_sub = 1/(2^53 − 2), rounded upward: just above 2^-53.
    let quanta = ((1u64 << 52) - 1) as f64;
    let nearest = 0.5 / quanta;
    assert!(nearest.mul_add(quanta, -0.5) < 0.0);
    assert_eq!(
        v(false, -1023, max_sub).to_binary64(),
        Binary64Outcome::Subnormal {
            value: max_sub_value,
            relative_precision: nearest.next_up(),
        }
    );
    assert_eq!(v(false, -1022, t).to_binary64(), normal(f64::MIN_POSITIVE));
    // 2^-1022 − 2^-1075 (the tie) rounds up into the normal range.
    let mut below = [0u64; L];
    below[L - 1] = u64::MAX << 10;
    assert_eq!(
        v(false, -1023, below).to_binary64(),
        normal(f64::MIN_POSITIVE)
    );
    // Exactly 2^-1075: a tie that rounds to zero, so it underflows.
    assert_eq!(
        v(true, -1075, t).to_binary64(),
        Binary64Outcome::Underflow { negative: true }
    );
    // 2^-1075 plus the smallest tail this width carries: the least subnormal.
    let min_sub = Binary64Outcome::Subnormal {
        value: f64::from_bits(1),
        relative_precision: 0.5,
    };
    assert_eq!(v(false, -1075, t1).to_binary64(), min_sub);
    // Just below 2^-1075: underflow.
    assert_eq!(
        v(false, -1076, f).to_binary64(),
        Binary64Outcome::Underflow { negative: false }
    );
    // MAX; just below the overflow midpoint 2^1024 − 2^970; the midpoint.
    assert_eq!(lift_any::<L>(f64::MAX).to_binary64(), normal(f64::MAX));
    let mut mid = [0u64; L];
    mid[L - 1] = u64::MAX << 10; // 54 ones: 2^1024 − 2^970 at exponent 1023
    assert_eq!(
        v(false, 1023, mid).to_binary64(),
        Binary64Outcome::Overflow { negative: false }
    );
    let mut just_below = mid;
    decrement(&mut just_below);
    assert_eq!(v(true, 1023, just_below).to_binary64(), normal(-f64::MAX));
    assert!(nb >= 128);
}

/// The exact lift at any width (K3a's lift at L = 2).
fn lift_any<const L: usize>(x: f64) -> Wide<L> {
    let bits = x.to_bits();
    let negative = bits >> 63 != 0;
    let biased = ((bits >> 52) & 0x7ff) as i64;
    let fraction = bits & ((1u64 << 52) - 1);
    let (integer, lsb) = match (biased, fraction) {
        (0, 0) => return signed_zero(negative),
        (0, f) => (f, -1074),
        (b, f) => (f | (1u64 << 52), b - 1075),
    };
    let lz = integer.leading_zeros();
    let mut significand = [0u64; L];
    significand[L - 1] = integer << lz;
    raw(negative, lsb + 63 - i64::from(lz), significand)
}

#[test]
fn conversion_boundary_outcomes_at_every_width() {
    boundary_outcomes::<2>();
    boundary_outcomes::<4>();
    boundary_outcomes::<8>();
    boundary_outcomes::<16>();
}

#[test]
fn conversion_outcomes_follow_the_zero_convention() {
    // An exact ±0 is normal and keeps its sign.
    assert_eq!(
        outcome_record(Wide2::ZERO.to_binary64()),
        (0, 0, 0),
        "+0 at L = 2"
    );
    assert_eq!(
        outcome_record(Wide::<16>::ZERO.neg().to_binary64()),
        (0, 1 << 63, 0),
        "-0 at L = 16"
    );
    assert_eq!(
        Wide2::ZERO.neg().to_binary64().value().unwrap().to_bits(),
        1 << 63
    );
    // Underflow and overflow keep their sign and never return a value.
    let tiny = raw::<8>(true, -2000, top::<8>());
    assert_eq!(
        tiny.to_binary64(),
        Binary64Outcome::Underflow { negative: true }
    );
    assert_eq!(tiny.to_binary64().value(), None);
    assert_eq!(
        tiny.neg().to_binary64(),
        Binary64Outcome::Underflow { negative: false }
    );
    let huge = raw::<4>(true, 2000, top::<4>());
    assert_eq!(
        huge.to_binary64(),
        Binary64Outcome::Overflow { negative: true }
    );
    assert_eq!(huge.to_binary64().value(), None);
    // A subnormal keeps its sign.
    let sub = lift::<4>(-f64::from_bits(3)).to_binary64();
    assert_eq!(sub.value().unwrap().to_bits(), (1 << 63) | 3);
}

/// Mirrors `gen_conv_value` in gen_wide_k3_vectors.py call for call.
fn gen_conv_value<const L: usize>(rng: &mut SplitMix64) -> Wide<L> {
    let nb = 64 * L as u64;
    let r = rng.next();
    let region = r % 16;
    let negative = (r >> 4) & 1 == 1;
    let zero = (r >> 5) % 128 == 0;
    let x = rng.next();
    let e: i64 = if region < 5 {
        -1076 + (x % 57) as i64
    } else if region < 8 {
        1018 + (x % 7) as i64
    } else if region < 10 {
        -1076 - (x % (nb + 64)) as i64
    } else if region == 10 {
        if x & 1 == 1 {
            (1i64 << 62) - (x % 1000) as i64
        } else {
            -(1i64 << 62) + ((x >> 1) % 1000) as i64
        }
    } else {
        -1100 + (x % 2201) as i64
    };
    // conv_sig
    let mut sig = rand_sig::<L>(rng);
    let sel = rng.next();
    let kind = sel % 6;
    let kept = 53.min(e + 1075);
    let rb = nb as i64 - 1 - kept;
    if 0 <= rb && rb <= nb as i64 - 1 {
        let rb = rb as usize;
        match kind {
            0 => {
                clear_below(&mut sig, rb + 1);
                set_bit(&mut sig, rb);
            }
            1 if rb > 0 => {
                clear_below(&mut sig, rb + 1);
                set_bit(&mut sig, rb);
                set_bit(&mut sig, ((sel >> 8) % rb as u64) as usize);
            }
            2 if rb < nb as usize - 1 => {
                clear_below(&mut sig, rb + 1);
                set_range(&mut sig, 0, rb);
            }
            3 => sig = full::<L>(),
            _ => {}
        }
    }
    if zero {
        signed_zero(negative)
    } else {
        raw(negative, e, sig)
    }
}

fn run_conversion_differential<const L: usize>(name: &str) {
    let started = Instant::now();
    let (seed, count, chunk_len, sha, chunks) =
        manifest_stream(DIFF_MANIFEST, "conversion", name, 5);
    assert!(count >= 1_000_000, "{name}: {count}");
    let samples: Vec<Vec<&str>> = DIFF_SAMPLE
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .filter(|f| f[0] == name)
        .collect();
    assert_eq!(samples.len(), 1000, "{name}");
    let mut rng = SplitMix64(seed);
    let mut total = Sha256::new();
    let mut chunk = Sha256::new();
    let mut outcomes = [0usize; 4];
    for i in 0..count {
        let value = gen_conv_value::<L>(&mut rng);
        let outcome = value.to_binary64();
        let (code, bits, relative) = outcome_record(outcome);
        outcomes[code as usize] += 1;
        if let Some(sample) = samples.get(i) {
            assert_eq!(sample[1].parse::<usize>().unwrap(), i);
            assert_eq!(sample[2], tok(&value), "{name} value {i} (generator port)");
            assert_eq!(outcome_token(outcome), sample[3], "{name} record {i}");
        }
        let mut record = [0u8; 17];
        record[0] = code;
        record[1..9].copy_from_slice(&bits.to_le_bytes());
        record[9..17].copy_from_slice(&relative.to_le_bytes());
        total.update(&record);
        chunk.update(&record);
        if (i + 1) % chunk_len == 0 {
            let done = std::mem::replace(&mut chunk, Sha256::new());
            let k = i / chunk_len;
            assert_eq!(done.hex(), chunks[k], "{name} chunk {k}");
        }
    }
    assert_eq!(total.hex(), sha, "{name} stream");
    // Concentrated at the boundaries: every outcome well represented.
    for n in outcomes {
        assert!(n > count / 20, "{name}: {outcomes:?}");
    }
    println!(
        "{name}: {count} conversions at L = {L} {outcomes:?}, debug wall time {:.1} s",
        started.elapsed().as_secs_f64()
    );
}

#[test]
fn seeded_conversion_differential_at_l2() {
    run_conversion_differential::<2>("conv2");
}

#[test]
fn seeded_conversion_differential_at_l4() {
    run_conversion_differential::<4>("conv4");
}

#[test]
fn seeded_conversion_differential_at_l8() {
    run_conversion_differential::<8>("conv8");
}

#[test]
fn seeded_conversion_differential_at_l16() {
    run_conversion_differential::<16>("conv16");
}

fn lift_round_trip<const L: usize>() {
    let mut rng = SplitMix64(0x11F7 ^ L as u64);
    let mut edges = vec![
        0.0,
        -0.0,
        f64::from_bits(1),
        f64::from_bits((1u64 << 52) - 1),
        f64::MIN_POSITIVE,
        f64::MAX,
        -f64::MAX,
        1.0,
        -3.5,
    ];
    for i in 0..100_000 {
        let bits = match i % 4 {
            0 => rng.next() & ((1u64 << 52) - 1),
            _ => rng.next(),
        };
        edges.push(f64::from_bits(bits));
    }
    for x in edges {
        if !x.is_finite() {
            continue;
        }
        let outcome = lift_any::<L>(x).to_binary64();
        let got = outcome.value().expect("representable");
        assert_eq!(got.to_bits(), x.to_bits(), "L = {L}: {x:e}");
        match outcome {
            Binary64Outcome::Normal(_) => assert!(!x.is_subnormal()),
            Binary64Outcome::Subnormal { .. } => assert!(x.is_subnormal()),
            other => panic!("{other:?}"),
        }
    }
}

#[test]
fn lifted_binary64_values_convert_back_to_the_same_bits_at_every_width() {
    // L = 2 through K3a's own lift.
    for x in [1.0, -0.0, f64::from_bits(5), f64::MAX, -1e-310] {
        let back = Wide2::from_f64(x).unwrap().to_binary64().value().unwrap();
        assert_eq!(back.to_bits(), x.to_bits());
    }
    lift_round_trip::<2>();
    lift_round_trip::<4>();
    lift_round_trip::<8>();
    lift_round_trip::<16>();
}

#[test]
fn conversion_at_l2_matches_exact_accumulator_round_where_both_are_defined() {
    // A value whose split is exact (no set bit below 2^-1074) fed to the
    // accumulator and rounded once must give the conversion's bits. The only
    // allowed differences: overflow (the accumulator's NonRepresentable), an
    // exact −0 (the accumulator's +0.0), and underflow (+0.0; unreachable here,
    // since a nonzero split-exact value is at least 2^-1074).
    let mut rng = SplitMix64(0xACC2);
    let mut counts = [0usize; 4];
    for i in 0..200_000u64 {
        let e: i64 = match i % 8 {
            0 | 1 => -1100 + (rng.next() % 110) as i64, // subnormal band
            2 => 1000 + (rng.next() % 24) as i64,       // near the top
            3 => 1023,                                  // the overflow edge
            _ => -1074 + (rng.next() % 2098) as i64,
        };
        let mut sig = rand_sig::<2>(&mut rng);
        if i % 8 == 3 && rng.next() & 1 == 1 {
            // At or above the midpoint 2^1024 − 2^970: overflow.
            sig[1] |= u64::MAX << 10;
        }
        // Clear the bits below 2^-1074 (bit index 127 − e − 1074 upward).
        let lowest_kept = -1074 - (e - 127);
        if lowest_kept > 0 {
            clear_below(&mut sig, lowest_kept.min(128) as usize);
        }
        if limbs_zero(&sig) {
            continue;
        }
        let negative = rng.next() & 1 == 1;
        let value = raw::<2>(negative, e, sig);
        let split = value.split_binary64().unwrap();
        assert!(!split.truncated_below_min_subnormal());
        let mut acc = ExactAccumulator::new();
        for &t in split.terms() {
            acc.add(t).unwrap();
        }
        match value.to_binary64() {
            Binary64Outcome::Normal(x) => {
                counts[0] += 1;
                assert_eq!(acc.round().unwrap().to_bits(), x.to_bits(), "{value:?}");
            }
            Binary64Outcome::Subnormal { value: x, .. } => {
                counts[1] += 1;
                assert_eq!(acc.round().unwrap().to_bits(), x.to_bits(), "{value:?}");
            }
            Binary64Outcome::Underflow { .. } => panic!("unreachable: {value:?}"),
            Binary64Outcome::Overflow { .. } => {
                counts[3] += 1;
                assert_eq!(acc.round(), Err(SumError::NonRepresentable), "{value:?}");
            }
        }
    }
    assert!(
        counts[0] > 1000 && counts[1] > 1000 && counts[3] > 1000,
        "{counts:?}"
    );
    // The exact −0: the conversion keeps the sign, the accumulator gives +0.0.
    assert_eq!(
        Wide2::ZERO.neg().to_binary64(),
        Binary64Outcome::Normal(-0.0)
    );
    assert_eq!(ExactAccumulator::new().round().unwrap().to_bits(), 0);
}

#[test]
fn subnormal_relative_precision_is_rounded_upward() {
    let mut ks: Vec<u64> = (1..=2000).collect();
    for j in 1..52 {
        let p = 1u64 << j;
        ks.extend([p - 1, p, p + 1]);
    }
    ks.push((1u64 << 52) - 1);
    let mut rng = SplitMix64(0x5B);
    for _ in 0..100_000 {
        ks.push(1 + rng.next() % ((1u64 << 52) - 1));
    }
    for k in ks {
        if k == 0 || k >= 1 << 52 {
            continue;
        }
        let r = half_quantum_over(k);
        // r = m · 2^q exactly; r ≥ 1/(2k) ⇔ m · 2k ≥ 2^-q, and the next value
        // below is < 1/(2k).
        let bits = r.to_bits();
        let q = ((bits >> 52) & 0x7ff) as i64 - 1075;
        let m = u128::from((bits & ((1u64 << 52) - 1)) | (1u64 << 52));
        assert!(q < 0 && -q < 120);
        let scale = 1u128 << (-q);
        let two_k = 2 * u128::from(k);
        assert!(m * two_k >= scale, "k = {k}: below 1/(2k)");
        assert!(
            (m - 1) * two_k < scale,
            "k = {k}: not the least upper value"
        );
        // The same value by K2b's hardware formula (0.5/k, next up when the
        // fused residual is negative).
        let quanta = k as f64;
        let nearest = 0.5 / quanta;
        let k2b = if nearest.mul_add(quanta, -0.5) < 0.0 {
            nearest.next_up()
        } else {
            nearest
        };
        assert_eq!(r.to_bits(), k2b.to_bits(), "k = {k}");
    }
}

// ---------------------------------------------------------------------------
// E. K4's arithmetic: TwoSum, TwoProduct, widening, narrowing, the integer
// constructor, and the §7.3-13/16 analogues
// ---------------------------------------------------------------------------

/// |e| ≤ ulp_p(s)/2 for a nonzero e: e's exponent is at most e_s − p, with
/// equality only for a power of two.
fn half_ulp_bound<const L: usize>(s: &Wide<L>, e: &Wide<L>, p: u32) -> bool {
    if limbs_zero(&e.significand) {
        return true;
    }
    let limit = s.exponent - i64::from(p);
    e.exponent < limit || (e.exponent == limit && e.significand == top::<L>())
}

fn check_eft_line<const L: usize>(fields: &[&str], line: &str)
where
    Wide<L>: SupportedWidth,
{
    let p: u32 = fields[2].parse().unwrap();
    let mut c = ctx::<L>(p);
    let x = parse::<L>(fields[3]);
    let y = parse::<L>(fields[4]);
    let (s, e) = match fields[0] {
        "twosum" => c.two_sum(&x, &y),
        "twoprod" => c.two_product(&x, &y),
        other => panic!("{other}"),
    }
    .unwrap();
    assert_eq!(tok(&s), fields[5], "{line}");
    assert_eq!(tok(&e), fields[6], "{line}");
    assert!(s.fits_precision(p) && e.fits_precision(p), "{line}");
    assert!(half_ulp_bound(&s, &e, p), "{line}");
}

fn check_narrow_line<const S: usize, const D: usize>(fields: &[&str], line: &str)
where
    Wide<D>: SupportedWidth,
{
    let p: u32 = fields[3].parse().unwrap();
    let x = parse::<S>(fields[4]);
    let got = ctx::<D>(p).round(&x).unwrap();
    assert_eq!(tok(&got), fields[5], "{line}");
}

fn check_int_line<const L: usize>(fields: &[&str], line: &str)
where
    Wide<L>: SupportedWidth,
{
    let p: u32 = fields[2].parse().unwrap();
    let negative = fields[3] == "1";
    let exponent: i64 = fields[4].parse().unwrap();
    let limbs: usize = fields[5].parse().unwrap();
    let hex = fields[6];
    let mut magnitude = vec![0u64; limbs];
    for (k, limb) in magnitude.iter_mut().enumerate() {
        let end = hex.len().saturating_sub(16 * k);
        let start = hex.len().saturating_sub(16 * (k + 1));
        if end > start {
            *limb = u64::from_str_radix(&hex[start..end], 16).unwrap();
        }
    }
    let got = ctx::<L>(p)
        .from_integer(negative, &magnitude, exponent)
        .unwrap();
    assert_eq!(tok(&got), fields[7], "{line}");
}

#[test]
fn two_sum_two_product_narrowing_and_the_integer_constructor_match_the_fraction_oracle() {
    use std::collections::BTreeMap;
    let mut counts: BTreeMap<(String, usize), usize> = BTreeMap::new();
    let mut accumulator_shaped = 0;
    for line in EFT.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let limbs: usize = fields[1].parse().unwrap();
        match fields[0] {
            "twosum" | "twoprod" => match limbs {
                4 => check_eft_line::<4>(&fields, line),
                8 => check_eft_line::<8>(&fields, line),
                16 => check_eft_line::<16>(&fields, line),
                other => panic!("{other}"),
            },
            "narrow" => {
                let target: usize = fields[2].parse().unwrap();
                match (limbs, target) {
                    (2, 4) => check_narrow_line::<2, 4>(&fields, line),
                    (2, 8) => check_narrow_line::<2, 8>(&fields, line),
                    (2, 16) => check_narrow_line::<2, 16>(&fields, line),
                    (4, 4) => check_narrow_line::<4, 4>(&fields, line),
                    (4, 8) => check_narrow_line::<4, 8>(&fields, line),
                    (4, 16) => check_narrow_line::<4, 16>(&fields, line),
                    (8, 4) => check_narrow_line::<8, 4>(&fields, line),
                    (8, 8) => check_narrow_line::<8, 8>(&fields, line),
                    (8, 16) => check_narrow_line::<8, 16>(&fields, line),
                    (16, 4) => check_narrow_line::<16, 4>(&fields, line),
                    (16, 8) => check_narrow_line::<16, 8>(&fields, line),
                    (16, 16) => check_narrow_line::<16, 16>(&fields, line),
                    other => panic!("{other:?}"),
                }
            }
            "int" => {
                if fields[5] == "68" && fields[4] == "-2148" {
                    accumulator_shaped += 1;
                }
                match limbs {
                    4 => check_int_line::<4>(&fields, line),
                    8 => check_int_line::<8>(&fields, line),
                    16 => check_int_line::<16>(&fields, line),
                    other => panic!("{other}"),
                }
            }
            other => panic!("{other}"),
        }
        *counts.entry((fields[0].to_string(), limbs)).or_default() += 1;
    }
    for limbs in [4, 8, 16] {
        for kind in ["twosum", "twoprod", "int"] {
            assert!(counts.get(&(kind.to_string(), limbs)).copied().unwrap_or(0) >= 20);
        }
    }
    for limbs in [2, 4, 8, 16] {
        assert!(
            counts
                .get(&("narrow".to_string(), limbs))
                .copied()
                .unwrap_or(0)
                >= 50
        );
    }
    assert!(accumulator_shaped >= 100, "{accumulator_shaped}");
}

fn eft_random<const L: usize>()
where
    Wide<L>: SupportedWidth,
{
    // s + e = x ∘ y exactly, checked at L = 16 and p = 1024, where both sides
    // are exact (operands of at most 512 bits, exponent gaps within 400).
    let mut rng = SplitMix64(0xEF7 ^ L as u64);
    let mut exact = ctx::<16>(1024);
    for &p in CLASS_PRECISIONS
        .iter()
        .filter(|&&p| p <= 512.min(64 * L as u32))
    {
        let mut c = ctx::<L>(p);
        for _ in 0..500 {
            let x = c
                .round(&raw::<L>(
                    rng.next() & 1 == 1,
                    (rng.next() % 401) as i64 - 200,
                    rand_sig::<L>(&mut rng),
                ))
                .unwrap();
            let y = c
                .round(&raw::<L>(
                    rng.next() & 1 == 1,
                    (rng.next() % 401) as i64 - 200,
                    rand_sig::<L>(&mut rng),
                ))
                .unwrap();
            let (s, e) = c.two_sum(&x, &y).unwrap();
            assert_eq!(s, c.add(&x, &y).unwrap());
            assert!(half_ulp_bound(&s, &e, p));
            let (x16, y16, s16, e16) = (
                x.widen::<16>(),
                y.widen::<16>(),
                s.widen::<16>(),
                e.widen::<16>(),
            );
            assert_eq!(
                exact.add(&x16, &y16).unwrap(),
                exact.add(&s16, &e16).unwrap(),
                "L = {L}, p = {p}: TwoSum {x:?} {y:?}"
            );
            let (s, e) = c.two_product(&x, &y).unwrap();
            assert_eq!(s, c.mul(&x, &y).unwrap());
            assert!(half_ulp_bound(&s, &e, p));
            assert_eq!(
                exact.mul(&x16, &y16).unwrap(),
                exact.add(&s.widen::<16>(), &e.widen::<16>()).unwrap(),
                "L = {L}, p = {p}: TwoProduct {x:?} {y:?}"
            );
        }
        // Operands wider than p are refused.
        let wide = raw::<L>(false, 0, full::<L>());
        let one = Wide::<L>::ONE;
        if p < 64 * L as u32 {
            assert_eq!(c.two_sum(&wide, &one), Err(WideError::OperandPrecision));
            assert_eq!(c.two_product(&one, &wide), Err(WideError::OperandPrecision));
        }
        // An exact result gives e = +0.
        let (s, e) = c.two_sum(&one, &one).unwrap();
        assert_eq!((s, e), (lift(2.0), Wide::<L>::ZERO));
        let (s, e) = c.two_product(&lift(-3.0), &lift(0.5)).unwrap();
        assert_eq!((s, e), (lift(-1.5), Wide::<L>::ZERO));
    }
}

#[test]
fn two_sum_and_two_product_are_error_free_on_random_operands() {
    eft_random::<4>();
    eft_random::<8>();
    eft_random::<16>();
}

#[test]
fn widening_is_exact_and_round_trips() {
    let mut rng = SplitMix64(0x71DE);
    for _ in 0..5_000 {
        let e = (rng.next() % 4001) as i64 - 2000;
        let negative = rng.next() & 1 == 1;
        let x2 = raw::<2>(negative, e, rand_sig::<2>(&mut rng));
        let x4 = x2.widen::<4>();
        let x8 = x4.widen::<8>();
        let x16 = x8.widen::<16>();
        assert_eq!(x16, x2.widen::<16>());
        // The top limbs carry the value; the value and its conversion agree.
        assert_eq!(&x16.significand[14..], &x2.significand[..]);
        assert!(limbs_zero(&x16.significand[..14]));
        assert_eq!((x16.negative, x16.exponent), (negative, e));
        assert_eq!(x16.to_binary64(), x2.to_binary64());
        // Narrowing back at the full precision of the narrower width is exact.
        assert_eq!(ctx::<4>(128).round(&x16).unwrap(), x4);
        assert_eq!(ctx::<8>(512).round(&x16).unwrap(), x8);
        let y8 = raw::<8>(negative, e, rand_sig::<8>(&mut rng));
        assert_eq!(ctx::<8>(512).round(&y8.widen::<16>()).unwrap(), y8);
        assert_eq!(
            ctx::<4>(256).round(&y8.widen::<16>()).unwrap(),
            ctx::<4>(256).round(&y8).unwrap()
        );
    }
    assert_eq!(Wide2::ZERO.neg().widen::<8>(), Wide::<8>::ZERO.neg());
}

#[test]
fn narrowing_rounds_ties_and_far_sticky_bits_correctly() {
    // A value of 1024 bits whose top 256 bits end in a tie: even stays, and a
    // single bit in limb 0 (768 bits below) rounds it up.
    let mut sig = top::<16>();
    set_bit(&mut sig, 1024 - 257); // the round bit of p = 256
    let tie = raw::<16>(false, 0, sig);
    let mut c = ctx::<4>(256);
    assert_eq!(c.round(&tie).unwrap(), Wide::<4>::ONE);
    let mut sig1 = sig;
    set_bit(&mut sig1, 0);
    let mut up = top::<4>();
    up[0] = 1;
    assert_eq!(
        c.round(&raw::<16>(false, 0, sig1)).unwrap(),
        raw::<4>(false, 0, up)
    );
    // Truncation would keep ONE here as well; the carry case separates it.
    assert_eq!(
        c.round(&raw::<16>(true, 5, full::<16>())).unwrap(),
        raw::<4>(true, 6, top::<4>())
    );
}

#[test]
fn integer_constructor_matches_exact_accumulator_at_p53() {
    // The accumulator's exact sum, mirrored as a 68-limb magnitude at quantum
    // 2^-2148, rounded once by the constructor at p = 53 and converted, equals
    // the accumulator's own rounding. The terms keep the sum normal, where a
    // 53-bit rounding is the binary64 rounding.
    let mut rng = SplitMix64(0x1A7);
    let add_term = |mag: &mut [u64; 68], x: f64| {
        let bits = x.to_bits();
        let biased = ((bits >> 52) & 0x7ff) as i64;
        let integer = (bits & ((1u64 << 52) - 1)) | (1u64 << 52);
        let position = (biased - 1075 + 2148) as usize;
        let mut term = [0u64; 68];
        term[position / 64] = integer << (position % 64);
        if position % 64 > 11 {
            term[position / 64 + 1] = integer >> (64 - position % 64);
        }
        assert!(!add_in_place(mag, &term));
    };
    for _ in 0..5_000 {
        let mut acc = ExactAccumulator::new();
        let mut positive = [0u64; 68];
        let mut negative = [0u64; 68];
        for _ in 0..(2 + rng.next() % 6) {
            let x = random_normal(&mut rng, 60);
            acc.add(x).unwrap();
            if x < 0.0 {
                add_term(&mut negative, -x);
            } else {
                add_term(&mut positive, x);
            }
        }
        let (neg, mut net) = if cmp_limbs(&positive, &negative) == Ordering::Less {
            (true, negative)
        } else {
            (false, positive)
        };
        sub_in_place(&mut net, if neg { &positive } else { &negative });
        let rounded = acc.round().unwrap();
        let got = ctx::<4>(53).from_integer(neg, &net, -2148).unwrap();
        if rounded != 0.0 && !rounded.is_normal() {
            continue;
        }
        assert_eq!(
            got.to_binary64(),
            Binary64Outcome::Normal(rounded),
            "{rounded:e}"
        );
    }
}

/// Grow-Expansion with TwoSum: the expansion's components are nonoverlapping
/// and sum exactly to the terms.
fn grow<const L: usize>(c: &mut WideContext<L>, expansion: &mut Vec<Wide<L>>, x: &Wide<L>)
where
    Wide<L>: SupportedWidth,
{
    let mut q = *x;
    for h in expansion.iter_mut() {
        let (s, e) = c.two_sum(&q, h).unwrap();
        *h = e;
        q = s;
    }
    expansion.push(q);
}

fn nonzero<const L: usize>(expansion: &[Wide<L>]) -> Vec<Wide<L>> {
    expansion
        .iter()
        .copied()
        .filter(|w| !limbs_zero(&w.significand))
        .collect()
}

fn cancellation_analogues<const L: usize>()
where
    Wide<L>: SupportedWidth,
{
    let nb = 64 * L as u32;
    for p in CLASS_PRECISIONS.iter().copied().filter(|&p| p <= nb) {
        let mut c = ctx::<L>(p);
        // (x, eps, −x, whether the fold at p must lose eps entirely)
        let mut cases: Vec<(Wide<L>, Wide<L>, Wide<L>, bool)> = Vec::new();
        // V1's check L: (1e80, 1e-8, −1e80). The fold loses 1e-8 entirely for
        // p ≤ 256 (1e-8 is below half an ulp of 1e80); above, it loses part.
        let (big, small) = (lift::<L>(1e80), lift::<L>(1e-8));
        cases.push((big, small, big.neg(), p <= 256));
        // A duplicate-operand cancellation: two bit-identical computed terms
        // A and −A, with a third term 2^-300 as large (entirely lost for
        // p ≤ 256; partly above).
        let a = c.mul(&lift(1.0 / 3.0), &lift(7.0)).unwrap();
        let a = c.div(&a, &lift(3.0)).unwrap();
        cases.push((a, a.mul_pow2(-300).unwrap(), a.neg(), p <= 256));
        // The scaled variants at every p: a small term 2^-(p+16) as large,
        // which the fold always loses.
        let large = lift::<L>(1.5).mul_pow2(i64::from(p) + 16).unwrap();
        cases.push((large, lift(1.0), large.neg(), true));
        cases.push((a, a.mul_pow2(-(i64::from(p) + 16)).unwrap(), a.neg(), true));
        let mut exact = ctx::<16>(1024);
        for (x, eps, minus_x, lost) in cases {
            // Exact through an expansion: its components sum exactly to eps
            // (summed at L = 16, p = 1024, exact for these spans).
            let mut expansion = Vec::new();
            for t in [&x, &eps, &minus_x] {
                grow(&mut c, &mut expansion, t);
            }
            let components = nonzero(&expansion);
            assert!(!components.is_empty() && components.len() <= 2);
            let mut sum = Wide::<16>::ZERO;
            for w in &components {
                sum = exact.add(&sum, &w.widen::<16>()).unwrap();
            }
            assert_eq!(
                sum,
                eps.widen::<16>(),
                "L = {L}, p = {p}: expansion of {x:?} {eps:?}: {components:?}"
            );
            // Folded at p, the small term is lost entirely where `lost`. (The
            // literal sums are partly or wholly kept above p = 256: at 512 and
            // 1024 the fold of V1's check L is exact.)
            let partial = c.add(&x, &eps).unwrap();
            let folded = c.add(&partial, &minus_x).unwrap();
            if lost {
                assert!(folded.is_zero(), "L = {L}, p = {p}: fold kept {folded:?}");
            }
        }
    }
}

#[test]
fn check_l_and_duplicate_cancellation_are_exact_through_an_expansion_and_lost_when_folded() {
    cancellation_analogues::<4>();
    cancellation_analogues::<8>();
    cancellation_analogues::<16>();
}

// ---------------------------------------------------------------------------
// F. The work counter
// ---------------------------------------------------------------------------

fn exercise<const L: usize>(c: &mut WideContext<L>, n: u64)
where
    Wide<L>: SupportedWidth,
{
    let x = lift::<L>(2.0);
    for _ in 0..n {
        c.add(&x, &x).unwrap();
    }
    c.sub(&x, &x).unwrap();
    c.sub(&x, &x).unwrap();
    c.mul(&x, &x).unwrap();
    c.div(&x, &x).unwrap();
    let _ = c.div(&x, &Wide::<L>::ZERO);
    c.sqrt(&x).unwrap();
    c.round(&Wide2::ONE).unwrap();
    c.from_integer(false, &[3], 0).unwrap();
    c.two_sum(&x, &x).unwrap();
    c.two_product(&x, &x).unwrap();
    let _ = c.two_product(&raw::<L>(false, 0, full::<L>()), &x);
}

#[test]
fn work_is_counted_by_kind_and_width_and_saturates() {
    let mut c4 = ctx::<4>(128);
    let mut c8 = ctx::<8>(320);
    let mut c16 = ctx::<16>(1024);
    exercise(&mut c4, 1);
    exercise(&mut c8, 2);
    exercise(&mut c16, 3);
    let w4 = c4.work();
    assert_eq!(
        (
            w4.add,
            w4.sub,
            w4.mul,
            w4.div,
            w4.sqrt,
            w4.round,
            w4.two_sum,
            w4.two_product
        ),
        (1, 2, 1, 2, 1, 2, 1, 2)
    );
    assert_eq!(w4.operations(), 12);
    assert_eq!(c8.work().add, 2);
    assert_eq!(c16.work().add, 3);
    let mut attempt = AttemptWork::default();
    attempt.record(&c4);
    attempt.record(&c8);
    attempt.record(&c16);
    assert_eq!(attempt.width::<4>(), c4.work());
    assert_eq!(attempt.width::<8>(), c8.work());
    assert_eq!(attempt.width::<16>(), c16.work());
    // Saturation: counts never wrap.
    let saturated = WidthWork {
        add: u64::MAX,
        ..WidthWork::default()
    };
    let mut twice = saturated;
    twice.merge(&saturated);
    assert_eq!(twice.add, u64::MAX);
    assert_eq!(saturated.limb_multiply_equivalents(16), u64::MAX);
    assert_eq!(saturated.operations(), u64::MAX);
}

#[test]
fn limb_multiply_cost_table_is_pinned() {
    let expected: [(OpKind, [u64; 3]); 8] = [
        (OpKind::Add, [8, 16, 32]),
        (OpKind::Sub, [8, 16, 32]),
        (OpKind::Mul, [16, 64, 256]),
        (OpKind::Div, [1290, 4626, 17442]),
        (OpKind::Sqrt, [1548, 5140, 18468]),
        (OpKind::Round, [8, 16, 32]),
        (OpKind::TwoSum, [48, 96, 192]),
        (OpKind::TwoProduct, [32, 96, 320]),
    ];
    for (kind, costs) in expected {
        for (limbs, cost) in [4usize, 8, 16].into_iter().zip(costs) {
            assert_eq!(
                limb_multiply_cost(kind, limbs),
                cost,
                "{kind:?} at L = {limbs}"
            );
        }
    }
    // One operation of every kind at L = 8 costs the sum of its column.
    let one_each = WidthWork {
        add: 1,
        sub: 1,
        mul: 1,
        div: 1,
        sqrt: 1,
        round: 1,
        two_sum: 1,
        two_product: 1,
    };
    assert_eq!(
        one_each.limb_multiply_equivalents(8),
        16 + 16 + 64 + 4626 + 5140 + 16 + 96 + 96
    );
}

#[test]
fn attempt_work_merges_across_widths() {
    let mut c4 = ctx::<4>(128);
    let mut c8 = ctx::<8>(512);
    let x4 = lift::<4>(3.0);
    let x8 = lift::<8>(3.0);
    c4.mul(&x4, &x4).unwrap();
    c4.div(&x4, &x4).unwrap();
    c8.sqrt(&x8).unwrap();
    let mut first = AttemptWork::default();
    first.record(&c4);
    let mut second = AttemptWork::default();
    second.record(&c8);
    second.record(&c4);
    first.merge(&second);
    assert_eq!(first.width::<4>().mul, 2);
    assert_eq!(first.width::<4>().div, 2);
    assert_eq!(first.width::<8>().sqrt, 1);
    assert_eq!(first.width::<16>(), WidthWork::default());
    assert_eq!(first.limb_multiply_equivalents(), 2 * 16 + 2 * 1290 + 5140);
}
