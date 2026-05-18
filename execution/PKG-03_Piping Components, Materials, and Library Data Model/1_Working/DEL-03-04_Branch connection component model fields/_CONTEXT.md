# Context: DEL-03-04

**Deliverable ID:** DEL-03-04
**Name:** Branch connection component model fields
**Package ID:** PKG-03
**Package Name:** Piping Components, Materials, and Library Data Model
**Type:** BACKEND_FEATURE_SLICE

## Description
Implement branch connection data fields for geometry, reinforcement, user SIF/flexibility, and source metadata.

## Anticipated Artifacts
- branch component model
- validation tests

## Scope Coverage
- SOW-008

## Objective Support
- OBJ-004

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Component model has more local fields but remains bounded.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Component model has more local fields but remains bounded.

## Package Reference
- **Package:** PKG-03 Piping Components, Materials, and Library Data Model
- **Package Scope:** Defines material/component/section/library models and public/private data governance at the data-object level.
- **Package Assigned Scope Items:** SOW-007, SOW-008, SOW-009, SOW-010, SOW-017, SOW-018, SOW-019, SOW-044, SOW-051
- **Package Exclusions:** Does not implement the rule evaluator or global solver.

## Decomposition Reference
- **Decomposition:** docs/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.4
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-03-04
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-008
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-03-04


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
