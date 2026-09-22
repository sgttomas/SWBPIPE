# Context: DEL-16-01

**Deliverable ID:** DEL-16-01
**Name:** Structured model operation schema
**Package ID:** PKG-16
**Package Name:** Model Operation and Agent Proposal Framework
**Type:** DATA_MODEL_CHANGE

## Description
Define structured add, move, modify, delete, reconnect, constraint, load, support, and design-knowledge operations.

## Anticipated Artifacts
- schemas/model_operation.schema.json
- operation fixtures

## Scope Coverage
- SOW-069

## Scope Detail
- SOW-069: All GUI and agent edits shall be represented as structured model operations that pass schema validation, constraint validation, diff preview, and controlled application through the model engine.
## Objective Support
- OBJ-015

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Operations are the only model mutation route for GUI and agent proposals.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Operations are the only model mutation route for GUI and agent proposals.

## Package Reference
- **Package:** PKG-16 Model Operation and Agent Proposal Framework
- **Package Scope:** Implements structured model operations, validation/diff preview, user acceptance/audit trail, and agent rationale/professional-boundary controls.
- **Package Assigned Scope Items:** SOW-069, SOW-070
- **Package Exclusions:** Does not allow hidden model mutations or autonomous engineering acceptance.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Authority basis:** Resolve `execution/_Decomposition/SOFTWARE_DECOMP.md` and its accepted amendments through `execution/_Coordination/_DECISIONS/_REGISTER.md`; source-state-bound evidence records the evaluated revision.
- **Status:** current_basis_for_control_surface_refresh

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-16-01
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-069
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-16-01

## Architecture Basis Injection
- **Scope Change:** SCA-001 architecture basis, carried into SCA-002 revision 0.7 coordination.
- **Architecture Basis:** Applicable `PKG-00 - Software Architecture Runway` constraints are carried by the consolidated `ArchitectureBasis.md` references under D-43 / SCA-006 and their cited decisions. Use applicable basis IDs as context constraints; this reference makes no lifecycle or issuance assertion.
- **Decomposition Reference:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; accepted decisions and amendments are recorded in `execution/_Coordination/_DECISIONS/_REGISTER.md`.
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, physical project package/container, and package-specific implementation choices remain decision-gated unless this deliverable later resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-002 Control-Surface Refresh Note
- This control surface was created by PREPARATION on 2026-05-03 from the accepted revision 0.7 decomposition and companion registers.
- This pass did not dispatch Type 2 work, produce implementation artifacts, promote candidate edges, refresh the blocker queue, materialize local `Dependencies.csv`, or promote the quarantined Chirality corpus.

## PREPARATION Notes
- Structural scaffold and metadata context only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
