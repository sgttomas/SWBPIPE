---
amendment_id: SCA-003
doc_kind: scope_change.authority
package_role: snapshot / handoff artifact
created: 2026-05-17
status: accepted
---

# SCA-003 Authority Record

## Authority

SCA-003 accepts the MVP local storage profile for OpenPipeStress:

- SQLite is the MVP local project store/index substrate.
- Canonical JSON/JCS-compatible schema-governed payloads remain the domain and interchange truth.
- SQLite FTS5/BM25 retrieval sidecars are rebuildable and non-authoritative.
- NumPy sidecars are optional non-authoritative caches only if later justified.
- Large files are referenced in place by path/URI plus hash and metadata by default.
- MVP storage remains local-only, free, offline-capable, bundled with the desktop app where needed, and without hosted DB, daemon, required network, cloud sync, or telemetry path.
- Direct plugin/adapter SQL access is prohibited.

## Evidence

|Field|Value|
|---|---|
|Execution Snapshot|`execution/_ScopeChange/SCA-003_2026-05-17_1658/`|
|Acceptance Record|`execution/_ScopeChange/SCA-003_2026-05-17_1658/ACCEPTANCE_RECORD.md`|
|Decomposition Revision|`execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.6`|

## Boundaries

This authority record does not itself implement schemas, storage code, runtime dependencies, desktop save/open UX, tests, or downstream production-document refresh. Those remain downstream TASK obligations.
