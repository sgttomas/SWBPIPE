# Context: DEL-07-03

**Deliverable ID:** DEL-07-03
**Name:** Material, component, and rule-pack editors
**Package ID:** PKG-07
**Package Name:** Graphical User Interface and Engineering Workflow
**Type:** UX_UI_SLICE

## Description

Implement material, section, component, private-library and rule-pack-reference editors plus load-case/combination and self-weight-plan interaction through the PKG-16 operation seam.

## Anticipated Artifacts
- editor panels
- validation UI tests

## Scope Coverage

- SOW-021
- SOW-077

## Objective Support
- OBJ-006

## Context Envelope
- **Envelope:** L
- **Envelope Notes:** Finite editor surfaces only; support/restraint implementation is DEL-07-02. Generation mathematics, schema and provenance contracts stay with their owners.

## Context Budget QA
- **Risk:** WATCH
- **Recommended Action:** Confirm scope and split if it expands
- **Notes:** Multiple editors but same GUI domain; may split later.

## Package Reference
- **Package:** PKG-07 Graphical User Interface and Engineering Workflow
- **Package Scope:** Implements the interactive modeler, editors, warning UX, solve-execution UX, and results views.
- **Package Assigned Scope Items:** SOW-020, SOW-021, SOW-022, SOW-023, SOW-036, SOW-055, SOW-076, SOW-077, SOW-078, SOW-079
- **Package Exclusions:** Does not silently supply missing code data.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Authority basis:** Resolve `execution/_Decomposition/SOFTWARE_DECOMP.md` and its accepted amendments through `execution/_Coordination/_DECISIONS/_REGISTER.md`; source-state-bound evidence records the evaluated revision.
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-07-03
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-021
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-07-03


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** Applicable `PKG-00 - Software Architecture Runway` constraints are carried by the consolidated `ArchitectureBasis.md` references under D-43 / SCA-006 and their cited decisions. Use applicable basis IDs as context constraints; this reference makes no lifecycle or issuance assertion.
- **Decomposition Reference:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; accepted decisions and amendments are recorded in `execution/_Coordination/_DECISIONS/_REGISTER.md`.
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-05, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Implementation choices and holds:** Apply DEC-012 through accepted bounded briefs or later rulings; do not copy the original SCA-001 TBD inventory as current status. The decision log in `execution/_Decomposition/SOFTWARE_DECOMP.md` records the storage/migration basis (DEC-017/019), expression grammar (DEC-022), numerical-library basis (DEC-023), package/container and naming (DEC-028/057), and CI/coverage posture (DEC-025/059/060/093); SCA-004 carries the adopted export scope. Exact dependency versions and local mechanisms follow accepted implementation evidence. Public transport, unadopted interfaces, numeric floor values, private-data/provider choices and any unfulfilled delivery or acceptance obligations remain governed separately; a ruled choice is not proof of implementation or release.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## Boundary Note (SCA-009)
- SCA-009 (decomposition revision 0.12, DEC-094) assigns the ownership landing for `DEL-07-03-R-005`/`R-006` (load-case, support/restraint editors) to `DEL-07-09` — the PKG-07 interactive operation vocabulary and tool palette contract. This deliverable's scope coverage, envelope, and lifecycle are unchanged; the WATCH advice to split rather than expand stands. Boundary metadata only.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.

## SCA-011 responsibility

Applied under the SCA-011 Group 2 decision; audited poststate acceptance is recorded in the SCA-011 Group 3 closure. Finite editor surfaces only; support/restraint implementation is DEL-07-02. Generation mathematics, schema and provenance contracts stay with their owners.
The amended Deliverables and ContextBudgetQA rows govern this allocation; existing state and implementation evidence remain separately bound.
