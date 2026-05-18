# Context: DEL-02-05

**Deliverable ID:** DEL-02-05
**Name:** Project persistence and round-trip serialization
**Package ID:** PKG-02
**Package Name:** Domain Model, Units, and Core Schemas
**Type:** DATA_MODEL_CHANGE

## Description
Implement create/open/save/versioned project persistence and deterministic model round-trip behavior for units, loads, rule-pack refs, and provenance metadata.

## Anticipated Artifacts
- project file schema
- round-trip tests
- persistence service contract

## Scope Coverage
- SOW-050
- SOW-041

## Objective Support
- OBJ-001
- OBJ-012

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Single data persistence surface; implement canonical JSON/JCS payloads over the SCA-003 local SQLite storage profile.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Single data persistence surface; implement canonical JSON/JCS payloads over the SCA-003 local SQLite storage profile.

## Package Reference
- **Package:** PKG-02 Domain Model, Units, and Core Schemas
- **Package Scope:** Defines the canonical software entities, unit system, persistence/serialization contracts, and extensibility boundaries.
- **Package Assigned Scope Items:** SOW-002, SOW-025, SOW-038, SOW-041, SOW-050
- **Package Exclusions:** Does not implement numerical solving or GUI views.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.6
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-02-05
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-050,SOW-041
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-02-05


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.6
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; SCA-003 local SQLite project store/index profile with rebuildable SQLite FTS5/BM25 retrieval sidecars; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, migration framework/tooling, DB migration implementation details, binary asset/export packaging, and optional retrieval-cache implementation details remain implementation-level decisions unless resolved under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-003 Storage Profile Injection
- **Storage Decision:** SQLite is the MVP local project store/index substrate, not the canonical domain truth.
- **Persistence Rule:** Store schema-governed canonical project/model/state/run JSON payloads with JCS-compatible hashes; SQLite tables may hold canonical bytes and indexed projections.
- **Sidecar Rule:** Retrieval sidecars are rebuildable and non-authoritative; NumPy sidecars are optional only if later benchmark-backed retrieval work justifies them.
- **External File Rule:** Large files are referenced in place by path/URI plus hash, size/classification, and verification metadata by default.
- **No-Bypass Rule:** No direct SQL access is exposed to plugins, adapters, or imports; all mutation routes through application services and validation gates.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
