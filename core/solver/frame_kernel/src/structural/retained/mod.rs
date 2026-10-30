//! W1's retained-precision structural method (T3 D1 §4.1, `FK/structural/retained/`).
//!
//! `wide::Wide<2>` is the arithmetic of the D-5 formation check (K-D5,
//! `structural/formation_check.rs`): correctly rounded + − × ÷ √ at a runtime
//! precision p ≤ 128, the exact lift from binary64, the exact split into at most
//! three binary64 terms for `ExactAccumulator::add_product`, the included-angle
//! arctangent, and the work counter.
//!
//! `wide/multi.rs` adds `Wide<L>` at L = 4, 8 and 16 with correctly rounded
//! + − × ÷ √ at a runtime precision p ≤ 64L (`WideContext<L>`), the rounded
//! conversion to binary64 with its outcome at every width (L = 2 included), exact
//! widening, rounded narrowing, TwoSum and TwoProduct, the integer constructor, and
//! per-width work counts (`AttemptWork`), for the retained method below.

// Items with no caller outside tests (accessors and test entries the tests use to
// reach the arithmetic's internals) are `#[cfg(test)]`.
pub(crate) mod wide;

// The retained method (the W1a kernel method, T3 D1 §4.1): the correctly rounded
// exact multi-term sum, the source, the ledger at p, formation and assembly, the
// factor and its screens, recovery, combinations and the adaptive schedule. The
// product reaches it through `structural::retained_api`.
pub(crate) mod adaptive;
pub(crate) mod assemble;
// D1 revision 5a.3 (R7 §4.1.6.1 item 6a, §4.1.6.2, §4.1.6.3): directed wide
// rounding, the certified inverse-norm bounds and the verification.
pub(crate) mod bound;
pub(crate) mod combine;
pub(crate) mod directed;
pub(crate) mod factor;
pub(crate) mod ledger;
pub(crate) mod declared;
pub(crate) mod recover;
mod product_certificate;
// V-K's seeded faults (T3 D1 §4.10, §7.3): compiled only for tests and under
// the `mutation-controls` feature, which only numerical_robustness's mutation
// run enables; inactive unless `FK_SEEDED_FAULT` names a fault.
#[cfg(any(test, feature = "mutation-controls"))]
pub(crate) mod seeded;
pub(crate) mod source;
pub(crate) mod verify;
pub(crate) mod wide_sum;

pub(crate) mod work;

pub(crate) mod origins;
