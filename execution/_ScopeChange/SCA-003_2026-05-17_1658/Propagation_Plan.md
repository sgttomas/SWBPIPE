---
amendment_id: SCA-003
doc_kind: scope_change.propagation_plan
package_role: snapshot / handoff artifact
created: 2026-05-17
status: executed
---

# Propagation Plan

## Direct SCA Writes

|Surface|Action|
|---|---|
|`execution/_Decomposition/SOFTWARE_DECOMP.md`|Update revision to 0.6, add SCA-003 intake/vocabulary/architecture/decision/open-issue changes.|
|`docs/_Registers/ScopeLedger.csv`|Update SOW-050 and SOW-059 decision refs and notes.|
|`docs/_Registers/Deliverables.csv`|Update DEL-00-04, DEL-02-05, and DEL-12-01 storage notes.|
|`docs/_Registers/ContextBudgetQA.csv`|Update DEL-00-04, DEL-02-05, and DEL-12-01 context notes.|
|Affected `_CONTEXT.md` files|Inject SCA-003 constraints into persistence, plugin/API, adapter, packaging, and local-first storage contexts.|
|`execution/_ScopeChange/SCA-003_2026-05-17_1658/*`|Create immutable SCA-003 snapshot.|
|`docs/_ScopeChange/SCA-003_2026-05-17_1658/Authority.md`|Record docs-side authority pointer.|
|Latest pointers|Update execution/docs latest SCA pointers and decomposition latest pointer.|

## Context Injection Map

|Deliverable|Reason|
|---|---|
|DEL-00-04|Owns persistence/schema versioning architecture.|
|DEL-02-05|Owns project persistence and round-trip serialization.|
|DEL-02-04|Owns plugin/extension domain contracts and no-bypass rules.|
|DEL-10-01|Owns public API/plugin boundary.|
|DEL-10-02|Owns import/export adapter framework.|
|DEL-10-04|Owns packaging implications for bundled SQLite/FTS5 capability.|
|DEL-12-01|Owns local-first storage and private data path posture.|

## Deferred Downstream Work

|Owner|Work|
|---|---|
|ORCHESTRATOR|Plan bounded TASK refreshes for schema/docs/code/storage implementation.|
|TASK / domain-schema|Update persistence schema physical-container and external artifact reference contract.|
|TASK / persistence|Implement local SQLite storage profile and tests.|
|TASK / security-privacy|Update local-first storage policy/tests for SCA-003.|
|TASK / interop-build|Ensure app packaging can bundle SQLite/FTS5 without hosted service/network dependency.|
|TASK / GUI workflow|Implement save/open UX behind application services.|
|REVIEW / RECONCILIATION|Check downstream docs/schema/code for stale `physical container remains TBD` wording before storage implementation closes.|
|CHANGE|Handle git staging/commit if requested.|

## Explicit Non-Writes

SCA-003 does not directly update schemas, runtime dependencies, code, tests, persistence service behavior, desktop UX, local-first policy docs, persistence architecture docs, DAGs, lifecycle registers, or implementation evidence. Those surfaces remain downstream-owned.
