# Context: DEL-02-04

**Deliverable ID:** DEL-02-04
**Name:** Plugin and extension domain contracts
**Package ID:** PKG-02
**Package Name:** Domain Model, Units, and Core Schemas
**Type:** API_CONTRACT

## Description
Define extension points and governance constraints for plugins/adapters.

## Anticipated Artifacts
- plugin interface spec
- sandbox/permission model notes

## Scope Coverage
- SOW-038

## Objective Support
- OBJ-009

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Cross-cutting but kept to domain/API definitions.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Cross-cutting but kept to domain/API definitions.

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
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-02-04
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-038
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-02-04


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.6
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; SCA-003 local SQLite project store/index profile with rebuildable SQLite FTS5/BM25 retrieval sidecars; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, migration framework/tooling, DB migration implementation details, binary asset/export packaging, and optional retrieval-cache implementation details remain implementation-level decisions unless resolved under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-003 Storage Profile Injection
- **No-Bypass Rule:** Plugins, extensions, adapters, and imports must not receive direct SQL access to project storage.
- **Boundary Rule:** Plugin/adapter operations must use schema-first application services that preserve validation, unit checks, provenance, diagnostics, private-data controls, and canonical hash behavior.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
