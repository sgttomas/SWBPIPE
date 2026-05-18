---
amendment_id: SCA-003
doc_kind: scope_change.decision_log
package_role: snapshot / handoff artifact
created: 2026-05-17
status: executed
---

# SCA-003 Decision Log

## Gate Decisions

|Gate|Decision|Basis|Status|
|---|---|---|---|
|Gate 1|Validate SCA-003 as a SOFTWARE decomposition amendment modifying architecture-basis, persistence notes, open issue, decision log, registers, and affected contexts.|Human request to implement the SCA-003 MVP Local Storage Profile plan on 2026-05-17.|Accepted and executed.|
|Gate 2|Accept impact assessment: no package/deliverable/objective cardinality changes; downstream schema/docs/code/storage implementation remains stale and deferred to TASK.|Human-supplied plan identified direct truth surfaces and downstream stale surfaces.|Accepted and executed.|
|Gate 3|Approve amendment preview: revision 0.6, AB-00-04 update, SOW-050/SOW-059 updates, OI-011 update, DEC-017 addition, companion register synchronization.|Human-supplied plan explicitly requested these amendments.|Accepted and executed.|
|Gate 4|Approve propagation plan limited to decomposition, companion registers, affected `_CONTEXT.md`, SCA snapshot, docs-side authority, and latest pointers.|Human-supplied plan required handoff of docs/schema/code propagation to bounded downstream TASK work.|Accepted and executed.|
|Gate 5|Execute and validate SCA-003 controlled amendment.|Current run implementation evidence and validation checks.|Executed; downstream closure remains required.|

## Architecture Decisions

|DecisionID|Decision|Basis|Status|
|---|---|---|---|
|SCA-003-DEC-001|Accept SQLite as the MVP local project store/index substrate while preserving canonical JSON/JCS as domain truth.|User storage proposal and accepted SCA-003 plan.|Accepted.|
|SCA-003-DEC-002|Treat SQLite FTS5/BM25 retrieval sidecars as rebuildable and non-authoritative.|User proposal and local-first storage constraints.|Accepted.|
|SCA-003-DEC-003|Do not lock NumPy into MVP storage; allow only optional local rebuildable caches if later evidence justifies them.|Packaging and offline/local-only constraint.|Accepted.|
|SCA-003-DEC-004|Reference large files in place by path/URI plus hash and metadata by default; do not copy them into the project package by default.|Local-first/private-data and portability tradeoff.|Accepted.|
|SCA-003-DEC-005|Prohibit hosted DB, daemon, required network, cloud sync, telemetry path, and direct plugin/adapter SQL access for MVP storage.|Local-install/offline/no-bypass requirements.|Accepted.|
|SCA-003-DEC-006|Keep schema, docs, code, tests, and desktop UX propagation as downstream TASK work, not direct SCOPE_CHANGE collateral edits.|SCOPE_CHANGE no-direct-collateral-write rule and accepted SCA-003 plan.|Accepted.|
