# Context: DEL-14-05

**Deliverable ID:** DEL-14-05
**Name:** Comparison mapping, tolerance, and export contracts
**Package ID:** PKG-14
**Package Name:** Model States, Analysis Runs, and Comparison
**Type:** API_CONTRACT

## Description
Define manual mappings, unmatched classifications, tolerance profiles, and CSV/JSON/report-section exports.

## Anticipated Artifacts
- comparison mapping schema
- tolerance profile schema
- comparison exporters

## Scope Coverage
- SOW-073

## Scope Detail
- SOW-073: The product shall compare two model states and/or two analysis runs deterministically using stable IDs, manual mappings where required, unit-normalized result deltas, and tolerance profiles.
## Objective Support
- OBJ-016

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Future external result states may be represented without changing core compare logic.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Future external result states may be represented without changing core compare logic.

## Package Reference
- **Package:** PKG-14 Model States, Analysis Runs, and Comparison
- **Package Scope:** Implements immutable model-state records, analysis-run records, deterministic state/run comparison, mappings, tolerances, and comparison exports.
- **Package Assigned Scope Items:** SOW-071, SOW-072, SOW-073
- **Package Exclusions:** Does not ingest commercial prover outputs comprehensively or determine external validation.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.5
- **Status:** current_basis_for_control_surface_refresh

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-14-05
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-073
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-14-05

## Architecture Basis Injection
- **Scope Change:** SCA-001 architecture basis, carried into SCA-002 revision 0.5 coordination.
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark `PKG-00` as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.5
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, physical project package/container, and package-specific implementation choices remain decision-gated unless this deliverable later resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-002 Control-Surface Refresh Note
- This control surface was created by PREPARATION on 2026-05-03 from the accepted revision 0.5 decomposition and companion registers.
- This pass did not dispatch Type 2 work, produce implementation artifacts, promote candidate edges, refresh the blocker queue, materialize local `Dependencies.csv`, or promote the quarantined Chirality corpus.

## PREPARATION Notes
- Structural scaffold and metadata context only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
