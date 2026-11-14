//! The reader's statement of a published result's numerical standing, for a
//! report (DEL-05-02 Standing; DEL-08-01 REQ-024).
//!
//! - [`reader_readability`]: whether the reader can read a result at all, a
//!   second verdict beside `result_export::reader_verdict::standing`'s
//!   unchanged `eligible`.
//! - [`historical_health`]: the per-case health projection of a published
//!   document, from its integrity codes and fixed text.
//! - [`report_standing`]: the standing record a report states, in the closed
//!   numerical vocabulary.
//!
//! # Why this is its own crate, and what must not be added to it
//!
//! `result_export` is a priced source: tools/retained_pricing pins its
//! `src/lib.rs`. Its Cargo.lock, and that of `product_physics`, which depends
//! on it, are reviewed inputs of the registered retained-memory profile. This
//! crate depends on `result_export` and is never a dependency of it or of
//! `product_physics`. A report-standing change therefore leaves the priced
//! reader, and the locks the registration binds, unchanged.
//!
//! - Do not add anything the retained route, `result_export` or
//!   `product_physics` would need to depend on.
//! - Do not add a reader of numerical evidence that `result_export` already
//!   reads. Call its public readers (`reader_verdict`) instead.
//! - The one table copied here, `reader_readability::standing_quality`, is held
//!   equal to the priced reader's own table by `tests/standing_quality_mirror.rs`.
//!   Change the two together.
pub mod historical_health;
pub mod reader_readability;
pub mod report_standing;
