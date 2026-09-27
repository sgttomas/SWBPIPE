//! W1's retained-precision structural method (T3 D1 §4.1, `FK/structural/retained/`).
//!
//! Slice K3a adds only the arithmetic the D-5 formation check (K-D5) needs:
//! `wide::Wide<2>` with correctly rounded + − × ÷ √ at a runtime precision
//! p ≤ 128, the exact lift from binary64, the exact split into at most three
//! binary64 terms for `ExactAccumulator::add_product`, the included-angle
//! arctangent, and the work counter. Its first caller is K-D5's formation
//! check (`structural/formation_check.rs`).

// K-D5 uses `Wide2`, `WideArith` (add, sub, mul, div, sqrt, included_angle),
// `from_f64`, `split_binary64`/`add_product_to`, `mul_pow2`, `neg`, `abs`,
// `cmp_value`, `is_zero` and `WideError`. The 13 items of K3a's tested surface
// with no non-test caller carry their own `#[allow(dead_code)]` in `wide.rs`,
// each with its reason (test-only, or K3 API for slice K3).
pub(crate) mod wide;
