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
- **Package Assigned Scope Items:** SOW-002, SOW-025, SOW-038, SOW-041, SOW-050, SOW-065
- **Package Exclusions:** Does not implement numerical solving or GUI views.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Authority basis:** Resolve `execution/_Decomposition/SOFTWARE_DECOMP.md` and its accepted amendments through `execution/_Coordination/_DECISIONS/_REGISTER.md`; source-state-bound evidence records the evaluated revision.
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-02-02
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-025
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-02-02


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** Applicable `PKG-00 - Software Architecture Runway` constraints are carried by the consolidated `ArchitectureBasis.md` references under D-43 / SCA-006 and their cited decisions. Use applicable basis IDs as context constraints; this reference makes no lifecycle or issuance assertion.
- **Decomposition Reference:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; accepted decisions and amendments are recorded in `execution/_Coordination/_DECISIONS/_REGISTER.md`.
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; an explicit deterministic JSON hash byte basis where JSON is hashed (currently `SORTED_COMPACT_JSON` for Python persistence, explicitly non-JCS, per D-41 R5 T2A); Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Implementation choices and open matters:** DEC-012 permits resolution in a sealed brief or later human ruling. Consult the accepted decisions in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12: solver strategy DEC-023; expression grammar DEC-022; migration/versioning DEC-019/033; package/export baseline DEC-028 and SCA-004; CI/coverage DEC-025/059/060/093. Current mechanisms and versions belong to bound implementation evidence. Public transport, per-format contracts, optional retrieval mechanisms and any unselected choices retain their own scope and decision paths; this reference does not close delivery, privacy, review or validation gaps.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
