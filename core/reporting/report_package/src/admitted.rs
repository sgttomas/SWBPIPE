//! The admitted-set type of the report package's identity boundary (W-11,
//! DEL-08-01 Design `exact-route-report-sections.md` §5.5).
//!
//! An [`AdmittedSet`] names the semantic contracts item 0 admits. Its field is
//! private to this module, so no other code can construct one. [`PRODUCTION`],
//! which wraps [`ADMITTED_SEMANTIC_CONTRACTS`], is the only value there is, in
//! every build, test builds included: since the owner opened exact-route
//! packages (§5.1; owner, 2026-10-26) the production set admits preview-physics-2,
//! pressure-2 and pressure-source-1, so the test-only widened set that the
//! in-crate witness used before opening is removed. Every crate-private `_in`
//! variant therefore receives the production set, whoever calls it. The set
//! reaches item 0 only along the forwarding graph that
//! `admitted_tests::admitted_set_forwarding_graph_is_closed` verifies, which
//! also asserts that no other set exists. Standard claim fence applies (claims
//! taxonomy per DEC-081).

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
    pub(crate) fn ids(&self) -> &'static [&'static str] {
        self.0
    }
}

/// The production set: every build's only admitted set.
pub(crate) const PRODUCTION: AdmittedSet = AdmittedSet(&ADMITTED_SEMANTIC_CONTRACTS);
