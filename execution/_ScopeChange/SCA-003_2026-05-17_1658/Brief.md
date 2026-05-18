---
amendment_id: SCA-003
doc_kind: scope_change.brief
package_role: snapshot / handoff artifact
created: 2026-05-17
status: executed
---

# SCA-003 Brief

## Request

Adopt the MVP local storage profile for OpenPipeStress:

- SQLite is the local project store/index substrate for MVP.
- Canonical JSON/JCS-compatible schema-governed payloads remain the domain and interchange truth.
- SQLite FTS5/BM25 retrieval sidecars are rebuildable and non-authoritative.
- NumPy sidecars are optional local caches only if later evidence justifies them.
- Large files are referenced in place by path/URI plus hash and metadata by default.
- MVP remains local-install, free, offline-capable, and without hosted DB, daemon, required network, cloud sync, or telemetry path.

## Parsed Actions

|Seq|ActionType|EntityType|EntityID|Requested Change|
|---:|---|---|---|---|
|1|MODIFY|DECOMPOSITION|SOFTWARE_DECOMP|Revise accepted software decomposition from revision 0.5 to 0.6 for SCA-003.|
|2|MODIFY|ARCHITECTURE_BASIS|AB-00-04|Resolve physical storage profile from TBD to local SQLite-backed project store/index with rebuildable sidecars.|
|3|MODIFY|SCOPE_ITEM|SOW-050, SOW-059|Update persistence notes to record SCA-003 while preserving canonical JSON/JCS and migration TBD boundaries.|
|4|MODIFY|OPEN_ISSUE|OI-011|Mark physical storage profile resolved and migration implementation details still TBD.|
|5|MODIFY|DECISION_LOG|DEC-010, DEC-012, DEC-017|Record DEC-017 and supersede the physical-container portion of earlier TBD decisions.|
|6|MODIFY|CONTEXTS|DEL-00-04, DEL-02-04, DEL-02-05, DEL-10-01, DEL-10-02, DEL-10-04, DEL-12-01|Inject SCA-003 dispatch constraints into directly affected contexts.|

## Human Approval Basis

The human project authority requested: `PLEASE IMPLEMENT THIS PLAN`, supplying the SCA-003 MVP Local Storage Profile plan as the implementation target on 2026-05-17. This run treats that instruction as approval to execute the listed SCOPE_CHANGE gates for this bounded amendment and records the gates in `Decision_Log.md`.

## Boundary

SCA-003 directly amends decomposition truth, companion registers, affected `_CONTEXT.md` dispatch surfaces, and SCA snapshot/latest records only. It does not directly edit schemas, source code, runtime dependencies, storage service implementation, desktop save/open UX, local-first policy docs, persistence contract docs, tests, or downstream production documents. Those are downstream TASK obligations.
