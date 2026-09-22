# Context: DEL-03-02

**Deliverable ID:** DEL-03-02
**Name:** Pipe section and component library schema
**Package ID:** PKG-03
**Package Name:** Piping Components, Materials, and Library Data Model
**Type:** DATA_MODEL_CHANGE

## Description
Define section/component records for user-entered dimensions, weights, COGs, and source/license metadata.

## Anticipated Artifacts
- schemas/component.schema.yaml
- schemas/section.schema.yaml

## Scope Coverage
- SOW-018

## Objective Support
- OBJ-004

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Library schema only; no protected tables.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Library schema only; no protected tables.

## Package Reference
- **Package:** PKG-03 Piping Components, Materials, and Library Data Model
- **Package Scope:** Defines material/component/section/library models and public/private data governance at the data-object level.
- **Package Assigned Scope Items:** SOW-007, SOW-008, SOW-009, SOW-010, SOW-017, SOW-018, SOW-019, SOW-044, SOW-051
- **Package Exclusions:** Does not implement the rule evaluator or global solver.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Authority basis:** Resolve `execution/_Decomposition/SOFTWARE_DECOMP.md` and its accepted amendments through `execution/_Coordination/_DECISIONS/_REGISTER.md`; source-state-bound evidence records the evaluated revision.
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-03-02
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-018
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-03-02


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** Applicable `PKG-00 - Software Architecture Runway` constraints are carried by the consolidated `ArchitectureBasis.md` references under D-43 / SCA-006 and their cited decisions. Use applicable basis IDs as context constraints; this reference makes no lifecycle or issuance assertion.
- **Decomposition Reference:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; accepted decisions and amendments are recorded in `execution/_Coordination/_DECISIONS/_REGISTER.md`.
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Implementation choices and open matters:** DEC-012 permits resolution in a sealed brief or later human ruling. Consult the accepted decisions in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12: solver strategy DEC-023; expression grammar DEC-022; migration/versioning DEC-019/033; package/export baseline DEC-028 and SCA-004; CI/coverage DEC-025/059/060/093. Current mechanisms and versions belong to bound implementation evidence. Public transport, per-format contracts, optional retrieval mechanisms and any unselected choices retain their own scope and decision paths; this reference does not close delivery, privacy, review or validation gaps.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
