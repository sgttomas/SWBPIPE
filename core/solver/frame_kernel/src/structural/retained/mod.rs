//! W1's retained-precision structural method (T3 D1 §4.1, `FK/structural/retained/`).
//!
//! Slice K3a adds only the arithmetic the D-5 formation check (K-D5) needs:
//! `wide::Wide<2>` with correctly rounded + − × ÷ √ at a runtime precision
//! p ≤ 128, the exact lift from binary64, the exact split into at most three
//! binary64 terms for `ExactAccumulator::add_product`, the included-angle
//! arctangent, and the work counter. Nothing in the product calls it yet, so
//! no published value changes; K-D5 is its first caller.

// K3a lands before its first caller (K-D5); until then the items are unused
// outside the tests. K-D5 removes this allowance.
#![allow(dead_code)]

pub(crate) mod wide;
