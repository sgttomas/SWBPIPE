# Context: DEL-02-02

**Deliverable ID:** DEL-02-02
**Name:** Unit system and dimensional-analysis core contract
**Package ID:** PKG-02
**Package Name:** Domain Model, Units, and Core Schemas
**Type:** BACKEND_FEATURE_SLICE

## Description
Specify unit dimensions, conversion rules, storage conventions, and unit test obligations.

## Anticipated Artifacts
- core/units module contract
- unit tests
- docs/SPEC.md section

## Scope Coverage
- SOW-025

## Objective Support
- OBJ-001
- OBJ-012

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Foundation for solver and rule engine.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Foundation for solver and rule engine.

## Package Reference
- **Package:** PKG-02 Domain Model, Units, and Core Schemas
- **Package Scope:** Defines the canonical software entities, unit system, persistence/serialization contracts, and extensibility boundaries.
- **Package Assigned Scope Items:** SOW-002, SOW-025, SOW-038, SOW-041, SOW-050
- **Package Exclusions:** Does not implement numerical solving or GUI views.

## Decomposition Reference
- **Decomposition:** docs/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.4
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-02-02
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-025
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-02-02


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
