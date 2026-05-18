# Context: DEL-07-08

**Deliverable ID:** DEL-07-08
**Name:** Design-authoring state and comparison workspace
**Package ID:** PKG-07
**Package Name:** Graphical User Interface and Engineering Workflow
**Type:** UX_UI_SLICE

## Description
Implement GUI panels and overlays for design knowledge, operation/diff review, state/run browsing, comparison tables, and graphical comparison review.

## Anticipated Artifacts
- GUI design knowledge panel
- operation diff review
- state/run browser
- comparison overlays

## Scope Coverage
- SOW-076

## Scope Detail
- SOW-076: The GUI shall support design-authoring and comparison workflows, including design knowledge panels, constraint/warning panels, state/run browsers, comparison tables, and graphical comparison overlays.
## Objective Support
- OBJ-015
- OBJ-016

## Context Envelope
- **Envelope:** L
- **Envelope Notes:** Adds design-authoring and comparison workspace to the GUI workflow.

## Context Budget QA
- **Risk:** WATCH
- **Recommended Action:** Confirm scope and split if it expands
- **Notes:** Adds design-authoring and comparison workspace to the GUI workflow.

## Package Reference
- **Package:** PKG-07 Graphical User Interface and Engineering Workflow
- **Package Scope:** Implements the interactive modeler, editors, warning UX, solve-execution UX, and results views.
- **Package Assigned Scope Items:** SOW-020, SOW-021, SOW-022, SOW-023, SOW-036, SOW-055, SOW-076
- **Package Exclusions:** Does not silently supply missing code data.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.5
- **Status:** current_basis_for_control_surface_refresh

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-07-08
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-076
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-07-08

## Architecture Basis Injection
- **Scope Change:** SCA-001 architecture basis, carried into SCA-002 revision 0.5 coordination.
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark `PKG-00` as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.5
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-05, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, physical project package/container, and package-specific implementation choices remain decision-gated unless this deliverable later resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-002 Control-Surface Refresh Note
- This control surface was created by PREPARATION on 2026-05-03 from the accepted revision 0.5 decomposition and companion registers.
- This pass did not dispatch Type 2 work, produce implementation artifacts, promote candidate edges, refresh the blocker queue, materialize local `Dependencies.csv`, or promote the quarantined Chirality corpus.

## PREPARATION Notes
- Structural scaffold and metadata context only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
