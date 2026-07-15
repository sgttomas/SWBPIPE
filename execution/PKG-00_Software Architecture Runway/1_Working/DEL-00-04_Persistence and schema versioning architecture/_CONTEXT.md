# Context: DEL-00-04

> **Preserved setup context — superseded for current consumption.** This
> sealed scaffold records the revision-0.7 setup basis and is retained as
> historical evidence. For current use, read `ArchitectureBasis.md`,
> `_STATUS.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9,
> and project decision D-43. The anticipated artifacts and PKG-01–12 gate
> below describe the setup context, not the current architecture-basis state.

**Deliverable ID:** DEL-00-04
**Name:** Persistence and schema versioning architecture
**Package ID:** PKG-00
**Package Name:** Software Architecture Runway
**Type:** DATA_MODEL_CHANGE

## Description
Define project persistence format, schema versioning, migrations, canonicalization, and deterministic round-trip serialization strategy.

## Anticipated Artifacts
- docs/architecture/persistence_versioning.md
- schema versioning contract

## Scope Coverage
- SOW-059

## Objective Support
- OBJ-013

## Context Envelope
- **Envelope:** L
- **Envelope Notes:** Architecture contract for deterministic project files, schema versions, migrations, and canonicalization.

## Context Budget QA
- **Risk:** WATCH
- **Recommended Action:** Confirm scope and split if it expands
- **Notes:** Architecture contract for deterministic project files, schema versions, migrations, and canonicalization.

## Package Reference
- **Package:** PKG-00 Software Architecture Runway
- **Package Scope:** Defines the implementation spine, architecture decisions, service/API boundaries, persistence/versioning strategy, GUI state model, diagnostics contract, and layered software-quality gates required before package-level implementation proceeds.
- **Package Assigned Scope Items:** SOW-056, SOW-057, SOW-058, SOW-059, SOW-060, SOW-061, SOW-062, SOW-063
- **Package Exclusions:** Does not implement product features, solver mechanics, GUI screens, or protected standards data.

## Architecture Gate Rule
- This deliverable is part of the `PKG-00` architecture runway.
- `PKG-01` through `PKG-12` package-level document drafting and implementation planning should not proceed until `PKG-00` reaches the selected architecture readiness threshold or the human changes the gate.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.7
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-00-04
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-059
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-00-04

## SCA-003 Storage Profile Injection
- **Scope Change:** SCA-003
- **Storage Decision:** MVP physical storage is a local SQLite-backed project store/index with rebuildable SQLite FTS5/BM25 retrieval sidecars.
- **Canonical Boundary:** JSON Schema 2020-12, canonical JSON/JCS-compatible hashes, and deterministic round-trip persistence remain the domain and interchange authority.
- **Large File Policy:** Large external files are referenced in place by path/URI plus hash and metadata by default; portable copy/export behavior is a later explicit workflow.
- **No-Bypass Rule:** Direct plugin/adapter SQL access is prohibited; storage mutation must route through application-service create/open/save/validate/version-check/migrate boundaries.
- **Still TBD:** Migration framework/tooling, DB migration implementation details, binary asset/export packaging, and optional retrieval-cache implementation details.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
