---
amendment_id: SCA-002
doc_kind: scope_change.decision_log
package_role: snapshot / handoff artifact
created: 2026-05-02
status: executed
---

# Decision Log

|DecisionID|Decision|Basis|Status|
|---|---|---|---|
|SCA-002-DEC-001|Admit PRD v0.2 as the working product-design basis for revision 0.5.|`docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` and comparison brief.|Accepted.|
|SCA-002-DEC-002|Preserve existing solver, rule-pack, IP/data-boundary, validation, and professional-boundary scope.|PRD v0.2 states the full analytical engine remains serious and useful while non-authoritative.|Accepted.|
|SCA-002-DEC-003|Add new flat packages `PKG-13` through `PKG-16` rather than overloading existing packages with unrelated lifecycle/comparison/operation scope.|SOFTWARE_DECOMP flat package and no-overlap invariants.|Accepted.|
|SCA-002-DEC-004|Keep commercial prover parsers, formal prover lifecycle/status, and automatic professional acceptance out of MVP decomposition scope unless later SCA reprioritizes them.|PRD v0.2 non-goals and MVP exclusions.|Accepted.|
|SCA-002-DEC-005|Record downstream artifact refresh as obligations, not direct SCA writes.|SCOPE_CHANGE no-direct-collateral-write rule and user downstream rule.|Accepted.|
|SCA-002-DEC-006|Apply hindsight-review cleanup so deliverables only claim package-local scope, scope-ledger mappings are reciprocal, package assignments match the ledger, latest pointer is current, and supersession CSVs use the canonical tool schema.|`REVIEW_HANDOFF_TO_NEXT_INSTANCE.md`; SOFTWARE_DECOMP single-domain deliverable invariant; local validation results.|Applied and accepted for downstream refresh planning.|
|SCA-002-DEC-007|Accept corrected SCA-002 revision `0.5` for downstream refresh planning and route planning through `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`.|Human approval on 2026-05-03: `APPROVE: proceed with SCA-002 for acceptance recording and downstream refresh planning`.|Accepted for planning; Type 2 dispatch remains blocked until refreshed coordination surfaces are accepted.|
