//! Test-only helpers shared by K4's test modules: SHA-256 (FIPS 180-4, as in
//! K3's tests), SplitMix64, the value tokens of K3's generator, limb helpers,
//! and ports of the generators' operand rules (mirrored call for call).
use super::super::super::wide::multi::{SupportedWidth, WideContext};
use super::super::super::wide::{Wide, WideError};

// ---------------------------------------------------------------- SHA-256

pub(crate) struct Sha256 {
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
    pub(crate) fn new() -> Self {
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

    pub(crate) fn update(&mut self, mut data: &[u8]) {
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

    pub(crate) fn hex(mut self) -> String {
        let bits = self.length.wrapping_mul(8);
        self.update(&[0x80]);
        while self.filled != 56 {
            self.update(&[0]);
        }
        self.update(&bits.to_be_bytes());
        self.state.iter().map(|w| format!("{w:08x}")).collect()
    }
}

pub(crate) fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    h.hex()
}

// ---------------------------------------------------------------- SplitMix64

pub(crate) struct SplitMix64(pub(crate) u64);

impl SplitMix64 {
    pub(crate) fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

pub(crate) fn seed_of(tag: &str) -> u64 {
    let b = tag.as_bytes();
    assert_eq!(b.len(), 8);
    u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

// ---------------------------------------------------------------- limbs (little-endian)

pub(crate) fn bit_length(a: &[u64]) -> usize {
    match a.iter().rposition(|&l| l != 0) {
        None => 0,
        Some(top) => top * 64 + 64 - a[top].leading_zeros() as usize,
    }
}

pub(crate) fn shl(a: &[u64], n: usize, out_len: usize) -> Vec<u64> {
    let mut out = vec![0u64; out_len];
    let (w, b) = (n / 64, n % 64);
    for (i, &limb) in a.iter().enumerate() {
        if i + w < out_len {
            out[i + w] |= limb << b;
        }
        if b != 0 && i + w + 1 < out_len {
            out[i + w + 1] |= limb >> (64 - b);
        }
    }
    out
}

pub(crate) fn shr(a: &[u64], n: usize) -> Vec<u64> {
    let (w, b) = (n / 64, n % 64);
    let mut out = vec![0u64; a.len()];
    for i in 0..a.len() {
        if i + w < a.len() {
            out[i] = a[i + w] >> b;
            if b != 0 && i + w + 1 < a.len() {
                out[i] |= a[i + w + 1] << (64 - b);
            }
        }
    }
    out
}

/// a·b, schoolbook (little-endian limbs).
pub(crate) fn big_mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0u64; a.len() + b.len() + 1];
    for (i, &x) in a.iter().enumerate() {
        let mut carry = 0u128;
        for (j, &y) in b.iter().enumerate() {
            let t = u128::from(out[i + j]) + u128::from(x) * u128::from(y) + carry;
            out[i + j] = t as u64;
            carry = t >> 64;
        }
        let mut k = i + b.len();
        while carry != 0 {
            let t = u128::from(out[k]) + carry;
            out[k] = t as u64;
            carry = t >> 64;
            k += 1;
        }
    }
    out
}

/// Numerical order of two little-endian magnitudes.
pub(crate) fn big_cmp(a: &[u64], b: &[u64]) -> std::cmp::Ordering {
    let n = a.len().max(b.len());
    for i in (0..n).rev() {
        let (x, y) = (
            a.get(i).copied().unwrap_or(0),
            b.get(i).copied().unwrap_or(0),
        );
        if x != y {
            return x.cmp(&y);
        }
    }
    std::cmp::Ordering::Equal
}

/// A nonnegative value ≥ num/den (big-endian hex integers, den > 0), decided
/// exactly: sig·den·2^k ≥ num with value = sig·2^k.
pub(crate) fn wide_at_least<const L: usize>(value: &Wide<L>, num_hex: &str, den_hex: &str) -> bool
where
    Wide<L>: SupportedWidth,
{
    assert!(!value.is_sign_negative() || value.is_zero());
    let (num, den) = (hex_limbs(num_hex), hex_limbs(den_hex));
    if value.is_zero() {
        return bit_length(&num) == 0;
    }
    let (_, exponent, sig) = value.parts();
    let k = exponent - (64 * L as i64 - 1);
    let lhs = big_mul(&sig, &den);
    let (lhs, rhs) = if k >= 0 {
        let n = k as usize;
        (shl(&lhs, n, lhs.len() + n / 64 + 1), num)
    } else {
        let n = (-k) as usize;
        (lhs, shl(&num, n, num.len() + n / 64 + 1))
    };
    big_cmp(&lhs, &rhs) != std::cmp::Ordering::Less
}

pub(crate) fn set_bit(a: &mut [u64], i: usize) {
    a[i / 64] |= 1 << (i % 64);
}

pub(crate) fn clear_below(a: &mut [u64], n: usize) {
    for i in 0..a.len() * 64 {
        if i < n {
            a[i / 64] &= !(1 << (i % 64));
        }
    }
}

/// Parses big-endian hex into little-endian limbs.
pub(crate) fn hex_limbs(hex: &str) -> Vec<u64> {
    let digits: Vec<u8> = hex.bytes().collect();
    let mut out = Vec::new();
    let mut end = digits.len();
    while end > 0 {
        let start = end.saturating_sub(16);
        let s = std::str::from_utf8(&digits[start..end]).unwrap();
        out.push(u64::from_str_radix(s, 16).unwrap());
        end = start;
    }
    if out.is_empty() {
        out.push(0);
    }
    out
}

// ---------------------------------------------------------------- tokens

/// `Z+`, `Z-`, or `<sign><hex>p<e>`: the significand's hex digits from the
/// top, left-aligned to 64L bits, and the exponent of the leading bit.
pub(crate) fn parse<const L: usize>(token: &str) -> Wide<L>
where
    Wide<L>: SupportedWidth,
{
    match token {
        "Z+" => return Wide::<L>::ZERO,
        "Z-" => return Wide::<L>::ZERO.neg(),
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
    Wide::<L>::from_parts(negative, exponent.parse().expect(token), significand).expect(token)
}

pub(crate) fn tok<const L: usize>(w: &Wide<L>) -> String
where
    Wide<L>: SupportedWidth,
{
    let (negative, exponent, significand) = w.parts();
    if w.is_zero() {
        return if negative { "Z-" } else { "Z+" }.to_string();
    }
    let mut hex = String::new();
    for limb in significand.iter().rev() {
        hex.push_str(&format!("{limb:016x}"));
    }
    let sign = if negative { '-' } else { '+' };
    format!("{sign}{}p{exponent}", hex.trim_end_matches('0'))
}

/// K3's record encoding of a value: sign, exponent (LE), limbs (LE).
pub(crate) fn enc<const L: usize>(w: &Wide<L>, out: &mut Vec<u8>)
where
    Wide<L>: SupportedWidth,
{
    let (negative, exponent, significand) = w.parts();
    out.push(u8::from(negative));
    out.extend_from_slice(&exponent.to_le_bytes());
    for limb in significand {
        out.extend_from_slice(&limb.to_le_bytes());
    }
}

/// The value mag·2^(e − bits + 1) with leading-bit exponent e (the
/// generator's `w_value`).
pub(crate) fn w_value<const L: usize>(negative: bool, e: i64, mag: &[u64]) -> Wide<L>
where
    Wide<L>: SupportedWidth,
{
    let bits = bit_length(mag);
    assert!(bits > 0 && bits <= 64 * L);
    let shifted = shl(mag, 64 * L - bits, L);
    let mut significand = [0u64; L];
    significand.copy_from_slice(&shifted);
    Wide::<L>::from_parts(negative, e, significand).unwrap()
}

pub(crate) fn ctx<const L: usize>(p: u32) -> WideContext<L>
where
    Wide<L>: SupportedWidth,
{
    WideContext::<L>::new(p).unwrap()
}

pub(crate) fn lift<const L: usize>(x: f64) -> Wide<L>
where
    Wide<L>: SupportedWidth,
{
    Wide::<L>::from_f64(x).unwrap()
}

pub(crate) fn f64_bits(hex: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(hex, 16).unwrap())
}

/// The generator's `rand_bits(rng, n)`: 64n random bits, the first draw the
/// top limb (little-endian result).
pub(crate) fn rand_bits(rng: &mut SplitMix64, n: usize) -> Vec<u64> {
    let mut v = vec![0u64; n];
    for k in 0..n {
        v[n - 1 - k] = rng.next();
    }
    v
}

// ---------------------------------------------------------------- K3's stream operand rules

/// K3's `rand_sig` (gen_wide_k3_vectors.py), call for call.
pub(crate) fn k3_rand_sig<const L: usize>(rng: &mut SplitMix64) -> Vec<u64> {
    let nb = 64 * L as u64;
    let mut sig = rand_bits(rng, L);
    sig[L - 1] |= 1 << 63;
    let sel = rng.next();
    match sel % 8 {
        0 => {
            let nbits = 1 + (sel >> 8) % nb;
            clear_below(&mut sig, (nb - nbits) as usize);
        }
        1 => {
            let w = 1 + (sel >> 8) % nb;
            for i in (nb - w) as usize..nb as usize {
                set_bit(&mut sig, i);
            }
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
            sig = vec![0; L];
            for i in nb as usize - w..nb as usize {
                set_bit(&mut sig, i);
            }
        }
        _ => {}
    }
    sig
}

fn raw<const L: usize>(negative: bool, exponent: i64, sig: &[u64]) -> Wide<L>
where
    Wide<L>: SupportedWidth,
{
    if sig.iter().all(|&l| l == 0) {
        return if negative {
            Wide::<L>::ZERO.neg()
        } else {
            Wide::<L>::ZERO
        };
    }
    let mut s = [0u64; L];
    s.copy_from_slice(sig);
    Wide::<L>::from_parts(negative, exponent, s).unwrap()
}

fn add_small(a: &[u64], d: u64) -> (Vec<u64>, bool) {
    let mut out = a.to_vec();
    let mut carry = d;
    for limb in out.iter_mut() {
        let (s, c) = limb.overflowing_add(carry);
        *limb = s;
        carry = u64::from(c);
        if carry == 0 {
            break;
        }
    }
    (out, carry != 0)
}

fn sub_small(a: &[u64], d: u64) -> Vec<u64> {
    let mut out = a.to_vec();
    let mut borrow = d;
    for limb in out.iter_mut() {
        let (s, b) = limb.overflowing_sub(borrow);
        *limb = s;
        borrow = u64::from(b);
        if borrow == 0 {
            break;
        }
    }
    out
}

/// K3's `gen_operands` (gen_wide_k3_vectors.py), call for call.
pub(crate) fn k3_gen_operands<const L: usize>(rng: &mut SplitMix64) -> (u8, Wide<L>, Wide<L>)
where
    Wide<L>: SupportedWidth,
{
    let nb = 64 * L as u64;
    let r = rng.next();
    let op = (r % 5) as u8;
    let mode = (r >> 3) % 8;
    let mut a_neg = (r >> 6) & 1 == 1;
    let b_neg = (r >> 7) & 1 == 1;
    let ea = ((r >> 8) % 4001) as i64 - 2000;
    let zero_a = (r >> 20) % 64 == 0;
    let zero_b = (r >> 26) % 64 == 0;
    let mut a_sig = k3_rand_sig::<L>(rng);
    let (b_sig, eb) = match mode {
        0..=3 => {
            let eb = ea + (rng.next() % 301) as i64 - 150;
            (k3_rand_sig::<L>(rng), eb)
        }
        4 => {
            let delta = rng.next() % 16;
            let (b, overflow) = add_small(&a_sig, delta);
            if overflow {
                (sub_small(&a_sig, delta), ea)
            } else {
                (b, ea)
            }
        }
        5 => {
            let w = rng.next() % (nb + 1);
            let mut x = rand_bits(rng, L);
            for i in w as usize..nb as usize {
                x[i / 64] &= !(1 << (i % 64));
            }
            let mut b: Vec<u64> = a_sig.iter().zip(&x).map(|(a, x)| a ^ x).collect();
            b[L - 1] |= 1 << 63;
            (b, ea)
        }
        6 => {
            let w = rng.next() % nb;
            let mut x = rand_bits(rng, L);
            for i in w as usize..nb as usize {
                x[i / 64] &= !(1 << (i % 64));
            }
            a_sig = x.iter().map(|x| u64::MAX ^ x).collect();
            let w2 = rng.next() % nb;
            let mut y = rand_bits(rng, L);
            for i in w2 as usize..nb as usize {
                y[i / 64] &= !(1 << (i % 64));
            }
            y[L - 1] |= 1 << 63;
            (y, ea + 1)
        }
        _ => {
            let span = nb + 400;
            let eb = ea + (rng.next() % (2 * span + 1)) as i64 - span as i64;
            (k3_rand_sig::<L>(rng), eb)
        }
    };
    if op == 4 {
        a_neg = false;
    }
    let a = if zero_a {
        raw::<L>(a_neg, 0, &vec![0; L])
    } else {
        raw::<L>(a_neg, ea, &a_sig)
    };
    let b = if zero_b {
        raw::<L>(b_neg, 0, &vec![0; L])
    } else {
        raw::<L>(b_neg, eb, &b_sig)
    };
    (op, a, b)
}

pub(crate) const OP_NAMES: [&str; 5] = ["add", "sub", "mul", "div", "sqrt"];

pub(crate) fn apply<const L: usize>(
    c: &mut WideContext<L>,
    op: u8,
    x: &Wide<L>,
    y: &Wide<L>,
) -> Result<Wide<L>, WideError>
where
    Wide<L>: SupportedWidth,
{
    match op {
        0 => c.add(x, y),
        1 => c.sub(x, y),
        2 => c.mul(x, y),
        3 => c.div(x, y),
        _ => c.sqrt(x),
    }
}

/// (seed, count, chunk, sha256, chunk digests) of a manifest stream.
pub(crate) fn manifest_stream(
    manifest: &str,
    kind: &str,
    name: &str,
) -> (u64, usize, usize, String, Vec<String>) {
    let head = manifest
        .lines()
        .find(|l| {
            let f: Vec<&str> = l.split_whitespace().collect();
            f.len() > 1 && f[0] == kind && f[1] == name
        })
        .expect(name);
    let f: Vec<&str> = head.split_whitespace().collect();
    let field = |key: &str| f[f.iter().position(|x| *x == key).unwrap() + 1];
    let seed = u64::from_str_radix(field("seed"), 16).unwrap();
    let count = field("count").parse().unwrap();
    let chunk = field("chunk").parse().unwrap();
    let sha = field("sha256").to_string();
    let chunks = manifest
        .lines()
        .filter(|l| l.starts_with(&format!("chunk {name} ")))
        .map(|l| l.split_whitespace().nth(3).unwrap().to_string())
        .collect();
    (seed, count, chunk, sha, chunks)
}

/// 2^e exactly, built from its bits (normal or subnormal): the tests' powers
/// of two, with no `powi` (RV19-N6).
pub(crate) fn pow2(e: i32) -> f64 {
    assert!((-1074..=1023).contains(&e), "2^{e}");
    if e >= -1022 {
        f64::from_bits(((e + 1023) as u64) << 52)
    } else {
        f64::from_bits(1u64 << (e + 1074))
    }
}
