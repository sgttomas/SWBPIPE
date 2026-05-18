# Context: DEL-15-04

**Deliverable ID:** DEL-15-04
**Name:** External prover boundary metadata
**Package ID:** PKG-15
**Package Name:** Handoff and External Prover Workflow
**Type:** DATA_MODEL_CHANGE

## Description
Implement flexible external-reference metadata without hard-coded approval, certification, or code-compliance statuses.

## Anticipated Artifacts
- external reference fields
- boundary validation tests

## Scope Coverage
- SOW-075

## Scope Detail
- SOW-075: The product shall support external-prover workflow metadata without forcing a formal prover-status lifecycle, automatic professional acceptance record, or comprehensive commercial-tool result ingestion in the MVP.
## Objective Support
- OBJ-017
- OBJ-018

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Preserves non-authoritative professional reliance boundary.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Preserves non-authoritative professional reliance boundary.

## Package Reference
- **Package:** PKG-15 Handoff and External Prover Workflow
- **Package Scope:** Implements canonical handoff package structure, manifests, target mapping metadata, unsupported-target flags, and non-authoritative external-prover metadata boundaries.
- **Package Assigned Scope Items:** SOW-074, SOW-075
- **Package Exclusions:** Does not force a prover-status lifecycle or generate professional approval records.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.5
- **Status:** current_basis_for_control_surface_refresh

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-15-04
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-075
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-15-04

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
