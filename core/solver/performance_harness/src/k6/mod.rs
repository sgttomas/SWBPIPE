//! K6 (T3 D1 revision 5a.2 §4.8): kernel harness observations.
//!
//! Observation only. Nothing here asserts a time or memory bound. The module
//! builds the sealed kernel models (R1's RF-LARGE families and the DEC-053
//! nine), their canonical bytes and deterministic storage counts, the staged
//! kernel sequence of the product's linear entry (`SparseAssemblyEvidence::
//! solve_assembled_with_formation_check`, in both modes), and the two legacy
//! DEC-050/053 observation lanes. The observation binary
//! (`src/bin/k6_observe/`) runs them one model and mode per process under a
//! counting, capped allocator that lives only in that binary.
//!
//! The legacy DEC-023/050/053 harness in `lib.rs` is unchanged; this module
//! reads only its fixture builders and the private DEC-053 specification list.

pub mod canonical;
pub mod counts;
pub mod lanes;
pub mod models;
pub mod parity;
pub mod staged;
pub mod w1;

/// The observation modes (ROOT's K6 rulings Q2 and Q12).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// SA's `SparseInteractive`: the pattern path.
    Sparse,
    /// SA's `DenseScrutiny`: the dense view and today's dense Cholesky.
    Dense,
    /// The legacy identity-order DEC-050/053 lane.
    LaneId,
    /// The legacy dense LU DEC-050/053 lane.
    LaneLu,
    /// K4's W1a kernel method through FK's `retained_api` (K6b; ROOT's
    /// rulings on I16's plan, Q2): one `solve_case` per repeat.
    W1a,
}

impl Mode {
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "sparse" => Some(Self::Sparse),
            "dense" => Some(Self::Dense),
            "lane-id" => Some(Self::LaneId),
            "lane-lu" => Some(Self::LaneLu),
            "w1a" => Some(Self::W1a),
            _ => None,
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sparse => "sparse",
            Self::Dense => "dense",
            Self::LaneId => "lane-id",
            Self::LaneLu => "lane-lu",
            Self::W1a => "w1a",
        }
    }
    /// Whether the mode materializes an n² matrix: every such mode is refused
    /// at 10,000 members or more (host rule; ROOT's ruling on RV16-N4).
    pub fn materializes_n2(self) -> bool {
        matches!(self, Self::Dense | Self::LaneLu)
    }
}

/// The member count at and above which every n² mode is refused.
pub const N2_REFUSAL_MEMBERS: usize = 10_000;

/// FNV-1a, 64 bits: a fixed, stated function for streamed digests of `Debug`
/// output and value bits. It is not a cryptographic hash; sha256 is computed
/// in Python (`hashlib`), never in Rust.
#[derive(Debug, Clone, Copy)]
pub struct Fnv64 {
    state: u64,
    len: u64,
}

impl Default for Fnv64 {
    fn default() -> Self {
        Self::new()
    }
}

impl Fnv64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    pub fn new() -> Self {
        Self {
            state: Self::OFFSET,
            len: 0,
        }
    }
    pub fn update(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state ^= u64::from(byte);
            self.state = self.state.wrapping_mul(Self::PRIME);
        }
        self.len += bytes.len() as u64;
    }
    pub fn update_f64(&mut self, value: f64) {
        self.update(&value.to_bits().to_le_bytes());
    }
    pub fn finish(&self) -> u64 {
        self.state
    }
    pub fn len(&self) -> u64 {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl std::fmt::Write for Fnv64 {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.update(s.as_bytes());
        Ok(())
    }
}

/// The length and FNV-1a digest of `value`'s `Debug` text, streamed, so no
/// string of the report's size is ever built.
pub fn debug_digest<T: std::fmt::Debug + ?Sized>(value: &T) -> (u64, u64) {
    use std::fmt::Write;
    let mut hasher = Fnv64::new();
    let _ = write!(hasher, "{value:?}");
    (hasher.len(), hasher.finish())
}
