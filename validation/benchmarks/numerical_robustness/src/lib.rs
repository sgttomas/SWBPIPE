//! VP-ROBUST (T3 D1 §4.10): R1's frozen references (`T3/REFERENCES`,
//! `references.json` 7b176dbb…) against the W1a kernel method, the kernel lane
//! (slice V-K), and later the product lane (V-P).
//!
//! - `cases`: the committed case files and their kernel models (the adapter's
//!   Rust half; `cases/gen_vk_cases.py` is the other).
//! - `exact`: the predicate and the floor comparison, decided exactly.
//! - `floor`: S\* and the zero-scale floor check.
//! - `compare`: verdicts, tallies and the report.
//! - `lane`: each case through `solve_case`, judged.
//! - `records`: the per-case records for ROOT's W1 limits.
//! - `rcm`: K4's RCM port against `sparse_direct`'s.
//! - `parity`: the binary64 gate's sparse–dense parity on RF-MECH and RF-LARGE.
//! - `invariance`: RF-INVARIANCE's recorded cross-variant observations.
//! - `scale`: the scale runs' counts and admission estimate (checkpoint B).
//!
//! No CI test reads R1's files, which the CI checkout omits (it skips the
//! project's run-evidence tree); the case files are generated from R1 and
//! committed here.
pub mod cases;
pub mod compare;
pub mod envelope;
pub mod exact;
pub mod floor;
pub mod invariance;
pub mod lane;
pub mod parity;
pub mod rcm;
pub mod records;
pub mod scale;
pub mod sha256;
