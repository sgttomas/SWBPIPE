// U4 G5 (D-6 = (a); R/I65/u4_g3_01/G2_AMENDMENTS.md §1): the one encoder of the
// retained build identity. It has three users and no dependencies:
// - build.rs, through `include!("src/build_identity.rs")`, which emits the value
//   as one `cargo:rustc-env=OPS_RETAINED_BUILD_IDENTITY=<encoded>` line;
// - this crate, as a private module of retained_memory.rs, which compares the
//   compiled value with the registered builds;
// - G6's registration texts, printed by a G5 test from the qualified build.
// Plain `//` comments only: the file is `include!`d at item position.

/// The identity format version, the first token of every identity text.
#[allow(dead_code)]
pub(crate) const IDENTITY_VERSION: &str = "v1";
/// The whole identity text when any key cannot be read (a read failure, never an
/// empty value: an empty variable such as `target.env` on Apple is a value).
#[allow(dead_code)]
pub(crate) const IDENTITY_UNAVAILABLE: &str = "v1;unavailable";
/// The environment variable build.rs sets and the crate reads with `option_env!`.
#[allow(dead_code)]
pub(crate) const IDENTITY_VARIABLE: &str = "OPS_RETAINED_BUILD_IDENTITY";
/// The keys, in their fixed order (BUILD.md §2.1's table).
#[allow(dead_code)]
pub(crate) const IDENTITY_KEYS: [&str; 16] = [
    "rustc.release",
    "rustc.commit",
    "rustc.host",
    "rustc.llvm",
    "target",
    "target.arch",
    "target.pointer_width",
    "target.endian",
    "target.os",
    "target.env",
    "panic",
    "profile",
    "opt_level",
    "debug_assertions",
    "rustflags",
    "pkg",
];

/// Whether a value byte passes through unescaped: printable ASCII other than
/// space, `%`, `;` and `=`.
#[allow(dead_code)]
pub(crate) const fn identity_byte_is_plain(byte: u8) -> bool {
    byte >= 0x21 && byte < 0x7F && byte != b'%' && byte != b';' && byte != b'='
}

/// `%`-encode one value's bytes into `out`: `%`, `;`, `=`, every byte below 0x21
/// and every byte at or above 0x7F become `%XX` with uppercase hex.
#[allow(dead_code)]
pub(crate) fn encode_identity_value(value: &[u8], out: &mut String) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for &byte in value {
        if identity_byte_is_plain(byte) {
            out.push(byte as char);
        } else {
            out.push('%');
            out.push(HEX[usize::from(byte >> 4)] as char);
            out.push(HEX[usize::from(byte & 0x0F)] as char);
        }
    }
}

/// The identity text: `v1;key=value;…` in `IDENTITY_KEYS` order, one line.
#[allow(dead_code)]
pub(crate) fn encode_identity(values: &[&[u8]; 16]) -> String {
    let mut out = String::from(IDENTITY_VERSION);
    for (key, value) in IDENTITY_KEYS.iter().zip(values.iter()) {
        out.push(';');
        out.push_str(key);
        out.push('=');
        encode_identity_value(value, &mut out);
    }
    out
}

/// Decode one `%`-encoded value; `None` if it is not in the encoder's image
/// (a plain byte that should be escaped, a lowercase or short escape, or an
/// escape of a byte the encoder passes through).
#[allow(dead_code)]
pub(crate) fn decode_identity_value(text: &str) -> Option<Vec<u8>> {
    fn hex(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        if byte == b'%' {
            let (hi, lo) = (*bytes.get(i + 1)?, *bytes.get(i + 2)?);
            let decoded = hex(hi)? << 4 | hex(lo)?;
            if identity_byte_is_plain(decoded) {
                return None;
            }
            out.push(decoded);
            i += 3;
        } else if identity_byte_is_plain(byte) {
            out.push(byte);
            i += 1;
        } else {
            return None;
        }
    }
    Some(out)
}

/// Decode a whole identity text into its values, in key order; `None` unless
/// the version is `v1` and the keys are exactly `IDENTITY_KEYS`, in order.
#[allow(dead_code)]
pub(crate) fn decode_identity(text: &str) -> Option<Vec<Vec<u8>>> {
    let mut tokens = text.split(';');
    if tokens.next()? != IDENTITY_VERSION {
        return None;
    }
    let mut values = Vec::with_capacity(IDENTITY_KEYS.len());
    for key in IDENTITY_KEYS {
        let (k, v) = tokens.next()?.split_once('=')?;
        if k != key {
            return None;
        }
        values.push(decode_identity_value(v)?);
    }
    if tokens.next().is_some() {
        return None;
    }
    Some(values)
}

// ---- D-6's reviewed inputs (RR "U4 G4: the margin rule trips") ----------------
// The reviewed-lock record binds 17 inputs by SHA-256: the PP lock, the precommit
// reader's 13 `include_str!` statics, and J1's three appended statics (DEF-C, DEF-E,
// XTABLE; I93 REVISION_01 §1.4). build.rs hashes them (no build-dependency:
// the digest below is self-contained) and emits one line,
// `OPS_RETAINED_REVIEWED_INPUTS=v1;<path>=<hex>;…`, in `REVIEWED_INPUTS` order,
// with `unavailable` for any input it cannot read. A registered profile records
// the exact text; any difference is Stale, never a compile error.

/// The environment variable build.rs sets for the reviewed inputs.
#[allow(dead_code)]
pub(crate) const REVIEWED_INPUTS_VARIABLE: &str = "OPS_RETAINED_REVIEWED_INPUTS";
/// The reviewed inputs, relative to this package's manifest directory: the PP
/// lock, then the reader's statics in G4's ORIGINS.json order.
#[allow(dead_code)]
pub(crate) const REVIEWED_INPUTS: [&str; 17] = [
    "Cargo.lock",
    "../../schemas/physics_source_recovery.schema.json",
    "../../schemas/retained_precision_mp_v2.schema.json",
    "../../fixtures/results/retained_precision_prepared_ordinary_v1.json",
    "../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json",
    "../../fixtures/results/semantic_contract_v0_2.json",
    "../../fixtures/results/semantic_contract_v0_3_precision_1.json",
    "../../fixtures/results/semantic_contract_v0_3_physics_1.json",
    "../../fixtures/results/semantic_contract_v0_3_load_reference_1.json",
    "../../fixtures/results/semantic_contract_v0_3_load_reference_source_1.json",
    "../../fixtures/results/semantic_contract_v0_3_preview_physics_1.json",
    "../../fixtures/results/semantic_contract_v0_3_physics_source_1.json",
    "../../fixtures/results/semantic_contract_v0_3_source_blocks_1.json",
    "../../schemas/source_block_recovery.schema.json",
    // B2/B3 J1 (I93 REVISION_01 §1.4; B3-D decision B3D-18: appended, so existing
    // positions keep their meaning): the combination and exact formation definitions
    // and the physics-retained-1 table.
    "../../fixtures/results/retained_precision_prepared_combination_v1.json",
    "../../fixtures/results/retained_precision_prepared_exact_v1.json",
    "../../fixtures/results/semantic_contract_v0_3_physics_retained_1.json",
];

/// The reviewed-input text for the given digests (`None`: unreadable).
#[allow(dead_code)]
pub(crate) fn encode_reviewed_inputs(digests: &[Option<[u8; 32]>; 17]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::from(IDENTITY_VERSION);
    for (path, digest) in REVIEWED_INPUTS.iter().zip(digests.iter()) {
        out.push(';');
        encode_identity_value(path.as_bytes(), &mut out);
        out.push('=');
        match digest {
            Some(bytes) => {
                for byte in bytes {
                    out.push(HEX[usize::from(byte >> 4)] as char);
                    out.push(HEX[usize::from(byte & 0x0F)] as char);
                }
            }
            None => out.push_str("unavailable"),
        }
    }
    out
}

/// SHA-256 (FIPS 180-4), self-contained so build.rs needs no build-dependency.
/// A G5 test checks it against the crate's `sha2` dependency.
#[allow(dead_code)]
pub(crate) fn sha256(data: &[u8]) -> [u8; 32] {
    let mut state: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    let mut chunks = data.chunks_exact(64);
    for chunk in &mut chunks {
        let mut block = [0u8; 64];
        block.copy_from_slice(chunk);
        sha256_compress(&mut state, &block);
    }
    let rest = chunks.remainder();
    let mut block = [0u8; 64];
    block[..rest.len()].copy_from_slice(rest);
    block[rest.len()] = 0x80;
    if rest.len() >= 56 {
        sha256_compress(&mut state, &block);
        block = [0u8; 64];
    }
    let bits = (data.len() as u64).wrapping_mul(8);
    block[56..].copy_from_slice(&bits.to_be_bytes());
    sha256_compress(&mut state, &block);
    let mut out = [0u8; 32];
    for (i, word) in state.iter().enumerate() {
        out[4 * i..4 * i + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}
#[allow(dead_code)]
fn sha256_compress(state: &mut [u32; 8], block: &[u8; 64]) {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut w = [0u32; 64];
    for i in 0..16 {
        w[i] = u32::from_be_bytes([block[4 * i], block[4 * i + 1], block[4 * i + 2], block[4 * i + 3]]);
    }
    for i in 16..64 {
        let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for i in 0..64 {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ (!e & g);
        let t1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = s0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }
    for (word, add) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
        *word = word.wrapping_add(add);
    }
}
