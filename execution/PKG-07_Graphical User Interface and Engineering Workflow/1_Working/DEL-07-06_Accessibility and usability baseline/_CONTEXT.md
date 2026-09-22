# Context: DEL-07-06

**Deliverable ID:** DEL-07-06
**Name:** Accessibility and usability baseline
**Package ID:** PKG-07
**Package Name:** Graphical User Interface and Engineering Workflow
**Type:** UX_UI_SLICE

## Description
Define and implement baseline keyboard, contrast, readability, and review-workflow usability requirements.

## Anticipated Artifacts
- accessibility checklist
- UI fixes
- tests

## Scope Coverage
- SOW-036

## Objective Support
- OBJ-006

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** May refine once GUI framework chosen.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** May refine once GUI framework chosen.

## Package Reference
- **Package:** PKG-07 Graphical User Interface and Engineering Workflow
- **Package Scope:** Implements the interactive modeler, editors, warning UX, solve-execution UX, and results views.
- **Package Assigned Scope Items:** SOW-020, SOW-021, SOW-022, SOW-023, SOW-036, SOW-055
- **Package Exclusions:** Does not silently supply missing code data.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Authority basis:** Resolve `execution/_Decomposition/SOFTWARE_DECOMP.md` and its accepted amendments through `execution/_Coordination/_DECISIONS/_REGISTER.md`; source-state-bound evidence records the evaluated revision.
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-07-06
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-036
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-07-06


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** Applicable `PKG-00 - Software Architecture Runway` constraints are carried by the consolidated `ArchitectureBasis.md` references under D-43 / SCA-006 and their cited decisions. Use applicable basis IDs as context constraints; this reference makes no lifecycle or issuance assertion.
- **Decomposition Reference:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; accepted decisions and amendments are recorded in `execution/_Coordination/_DECISIONS/_REGISTER.md`.
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-05, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Implementation choices and holds:** Apply DEC-012 through accepted bounded briefs or later rulings; do not copy the original SCA-001 TBD inventory as current status. The decision log in `execution/_Decomposition/SOFTWARE_DECOMP.md` records the storage/migration basis (DEC-017/019), expression grammar (DEC-022), numerical-library basis (DEC-023), package/container and naming (DEC-028/057), and CI/coverage posture (DEC-025/059/060/093); SCA-004 carries the adopted export scope. Exact dependency versions and local mechanisms follow accepted implementation evidence. Public transport, unadopted interfaces, numeric floor values, private-data/provider choices and any unfulfilled delivery or acceptance obligations remain governed separately; a ruled choice is not proof of implementation or release.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
