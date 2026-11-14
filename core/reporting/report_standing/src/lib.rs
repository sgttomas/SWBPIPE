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
pub mod historical_health;
pub mod reader_readability;
pub mod report_standing;
