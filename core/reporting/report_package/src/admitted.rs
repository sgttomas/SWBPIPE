//! The admitted-set type of the report package's identity boundary (W-11,
//! DEL-08-01 Design `exact-route-report-sections.md` §5.5).
//!
//! An [`AdmittedSet`] names the semantic contracts item 0 admits. Its field is
//! private to this module, so no other code can construct one. In a non-test
//! build [`PRODUCTION`], which wraps [`ADMITTED_SEMANTIC_CONTRACTS`], is the
//! only value there is, so every crate-private `_in` variant receives the
//! production set there, whoever calls it. The widened test set,
//! `EXACT_WITNESS_ADMITTED`, exists only in this crate's own test build, for
//! the in-crate witness module. The set reaches item 0 only along the
//! forwarding graph that `admitted_tests::admitted_set_forwarding_graph_is_closed`
//! verifies. Standard claim fence applies (claims taxonomy per DEC-081).

use crate::document_form::ADMITTED_SEMANTIC_CONTRACTS;

/// The semantic contracts an item-0 check admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AdmittedSet(&'static [&'static str]);

impl AdmittedSet {
    /// Whether the set admits `id`.
    pub(crate) fn contains(&self, id: &str) -> bool {
        self.0.contains(&id)
    }

    /// The admitted identities.
    #[allow(dead_code)]
    pub(crate) fn ids(&self) -> &'static [&'static str] {
        self.0
    }
}

/// The production set: every build's only admitted set outside this crate's
/// tests.
pub(crate) const PRODUCTION: AdmittedSet = AdmittedSet(&ADMITTED_SEMANTIC_CONTRACTS);

/// The three identities the in-crate witness admits (test build only).
#[cfg(test)]
use open_pipe_stress_result_export::semantic_contract::{PRESSURE_ID, PRESSURE_SOURCE_ID, PREVIEW_PHYSICS_ID};
// Used by the in-crate witness module (slice (b) of the design).
#[allow(dead_code)]
#[cfg(test)]
pub(crate) const EXACT_WITNESS_ADMITTED: AdmittedSet = AdmittedSet(&[PREVIEW_PHYSICS_ID, PRESSURE_ID, PRESSURE_SOURCE_ID]);
