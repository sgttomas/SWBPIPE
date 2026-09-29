//! W1's retained-precision structural method (T3 D1 §4.1, `FK/structural/retained/`).
//!
//! Slice K3a adds only the arithmetic the D-5 formation check (K-D5) needs:
//! `wide::Wide<2>` with correctly rounded + − × ÷ √ at a runtime precision
//! p ≤ 128, the exact lift from binary64, the exact split into at most three
//! binary64 terms for `ExactAccumulator::add_product`, the included-angle
//! arctangent, and the work counter. Its first caller is K-D5's formation
//! check (`structural/formation_check.rs`).
//!
//! Slice K3 adds, in `wide/multi.rs` beside K3a's unchanged `Wide<2>` code:
//! `Wide<L>` at L = 4, 8 and 16 with correctly rounded + − × ÷ √ at a runtime
//! precision p ≤ 64L (`WideContext<L>`), the rounded conversion to binary64
//! with its outcome at every width (L = 2 included), exact widening, rounded
//! narrowing, TwoSum and TwoProduct, the integer constructor, and per-width
//! work counts (`AttemptWork`). Its first caller is K4.

// K-D5 uses `Wide2`, `WideArith` (add, sub, mul, div, sqrt, included_angle),
// `from_f64`, `split_binary64`/`add_product_to`, `mul_pow2`, `neg`, `abs`,
// `cmp_value`, `is_zero` and `WideError`. The 12 items of K3a's tested surface
// with no non-test caller (13 before K3, whose `multi` constructs
// `NotNormalized`) carry their own `#[allow(dead_code)]` in `wide.rs`,
// each with its reason (test-only, or the slice that will call it). K3's
// `wide/multi.rs` items follow the same rule (K4 API).
pub(crate) mod wide;

// K4 (the W1a kernel method, T3 D1 §4.1; ROOT's K4 rulings): the correctly
// rounded exact multi-term sum, the source, the ledger at p, formation and
// assembly, the factor and its screens, recovery, combinations and the adaptive
// schedule. K4 has no product caller (ROOT's K4 ruling Q1): F2a wires W1. Each
// entry point without a non-test caller carries its own `#[allow(dead_code)]`
// naming its consumer (F2a API or V-K API); the helpers they reach are live.
pub(crate) mod adaptive;
pub(crate) mod assemble;
// D1 revision 5a.3 (R7 §4.1.6.1 item 6a, §4.1.6.2, §4.1.6.3): directed wide
// rounding, the certified inverse-norm bounds and the verification.
pub(crate) mod bound;
pub(crate) mod combine;
pub(crate) mod directed;
pub(crate) mod factor;
pub(crate) mod ledger;
pub(crate) mod recover;
pub(crate) mod source;
pub(crate) mod verify;
pub(crate) mod wide_sum;
