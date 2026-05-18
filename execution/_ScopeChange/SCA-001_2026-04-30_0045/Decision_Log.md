---
amendment_id: SCA-001
doc_kind: scope_change.decision_log
created: 2026-04-30
status: executed
---

# Decision Log

|Gate|Decision|Disposition|
|---|---|---|
|Gate 1|Validate SCA-001 as a SOFTWARE decomposition/context propagation amendment.|Accepted by human.|
|Gate 2|Accept impact assessment covering decomposition, registers, downstream contexts, coordination state, and future TASK dispatch.|Accepted by human.|
|Gate 3|Approve formal decomposition/register amendment preview for SCA-001.|Approved by human.|
|Gate 4|Approve propagation plan for SCA-001.|Approved by human.|
|Gate 5|Execute propagation and validation.|Executed in this snapshot.|

## Architecture Decisions Added

- `DEC-008`: Use `PKG-00` `SEMANTIC_READY` content as SCA-001 architecture-basis candidate for sealed brief injection, without treating it as `ISSUED`.
- `DEC-009`: Adopt Rust core/application services, Tauri 2, TypeScript/React/Vite, and Three.js as the runtime/UI baseline.
- `DEC-010`: Adopt JSON Schema 2020-12, schema-first envelopes, and canonical JSON/JCS-compatible hashing as schema/API/persistence baseline.
- `DEC-011`: Adopt Cargo tests, Vitest, Playwright, validation gates, protected-content/provenance gates, and deterministic solver/rule gates as test baseline.
- `DEC-012`: Preserve exact versions, solver library, rule grammar, public API transport, import/export format list, CI details, thresholds, and physical project package/container as implementation-level TBDs.
