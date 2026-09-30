//! K6b (T3 K6b brief; ROOT's rulings on I16's plan): W1 observations on the
//! K6 harness, through FK's `structural::retained_api` (K6b's A0).
//!
//! Observation only: nothing here asserts a time or memory bound, and nothing
//! changes K4's method. The adapter maps a K6 kernel model to K4's
//! `SourceParts`; the counts and the estimate derive W1's storage and its
//! admission estimate; the staged sequence runs one `solve_case` per repeat,
//! accounts K4's work by stage and precision, and forms the budget-truncated
//! prefixes; the rows module keys the published rows and derives R1's member
//! quantities.

pub mod adapter;
pub mod counts;
pub mod rows;
pub mod staged;
