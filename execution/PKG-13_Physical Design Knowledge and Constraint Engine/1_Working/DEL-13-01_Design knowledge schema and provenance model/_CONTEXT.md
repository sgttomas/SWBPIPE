# Context: DEL-13-01

**Deliverable ID:** DEL-13-01
**Name:** Design knowledge schema and provenance model
**Package ID:** PKG-13
**Package Name:** Physical Design Knowledge and Constraint Engine
**Type:** DATA_MODEL_CHANGE

## Description
Define design knowledge records for endpoints, line data, routing corridors, zones, equipment interfaces, access/slope requirements, source notes, and assumptions.

## Anticipated Artifacts
- schemas/design_knowledge.schema.json
- design knowledge provenance model

## Scope Coverage
- SOW-067

## Scope Detail
- SOW-067: The product shall support user-supplied design knowledge such as endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, and owner/project metadata.
## Objective Support
- OBJ-014

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** No protected owner standards or project data in public examples.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** No protected owner standards or project data in public examples.

## Package Reference
- **Package:** PKG-13 Physical Design Knowledge and Constraint Engine
- **Package Scope:** Implements schema-backed design knowledge, constraint provenance, constraint validation, and physical-to-analytical transformation contracts.
- **Package Assigned Scope Items:** SOW-066, SOW-067, SOW-068
- **Package Exclusions:** Does not bundle owner standards, protected code data, or final engineering acceptance logic.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Authority basis:** Resolve `execution/_Decomposition/SOFTWARE_DECOMP.md` and its accepted amendments through `execution/_Coordination/_DECISIONS/_REGISTER.md`; source-state-bound evidence records the evaluated revision.
- **Status:** current_basis_for_control_surface_refresh

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-13-01
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-067
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-13-01

## Architecture Basis Injection
- **Scope Change Basis:** SCA-001 architecture constraints with accepted amendments recorded in `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`. Historical setup/amendment dates below retain their original reach.
- **Architecture Basis:** Applicable `PKG-00 - Software Architecture Runway` constraints are carried by the consolidated `ArchitectureBasis.md` references under D-43 / SCA-006 and their cited decisions. Use applicable basis IDs as context constraints; this reference makes no lifecycle or issuance assertion.
- **Decomposition Reference:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; accepted decisions and amendments are recorded in `execution/_Coordination/_DECISIONS/_REGISTER.md`.
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Implementation choices and resolved decisions:** Apply DEC-012 at its actual reach: a sealed brief or later human ruling may resolve an implementation choice. DEC-017 fixes the SQLite local-store/canonical-JSON profile; DEC-028 fixes the project interchange container; DEC-022 fixes the rule grammar; SCA-004 establishes the named export scope. DEC-025 and DEC-060 govern their adopted verification/coverage portions. Exact dependencies, numerical library, public transport, format-specific contracts and other unselected details remain subject to their applicable accepted brief or decision; no global claim that these topics are all still undecided is made.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-002 Control-Surface Refresh Note
- This control surface was created by PREPARATION on 2026-05-03 from the accepted revision 0.7 decomposition and companion registers.
- This pass did not dispatch Type 2 work, produce implementation artifacts, promote candidate edges, refresh the blocker queue, materialize local `Dependencies.csv`, or promote the quarantined Chirality corpus.

## PREPARATION Notes
- Structural scaffold and metadata context only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
