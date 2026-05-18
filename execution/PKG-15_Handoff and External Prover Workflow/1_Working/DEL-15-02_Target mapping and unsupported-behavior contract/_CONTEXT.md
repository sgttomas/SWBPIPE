# Context: DEL-15-02

**Deliverable ID:** DEL-15-02
**Name:** Target mapping and unsupported-behavior contract
**Package ID:** PKG-15
**Package Name:** Handoff and External Prover Workflow
**Type:** API_CONTRACT

## Description
Define target mapping metadata and unsupported/approximate behavior flags for handoff exports.

## Anticipated Artifacts
- target mapping schema
- unsupported behavior taxonomy

## Scope Coverage
- SOW-074

## Scope Detail
- SOW-074: The product shall generate schema-compliant handoff packages with model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, and unsupported-target flags.
## Objective Support
- OBJ-017

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Avoids silent loss of critical assumptions or unsupported target behavior.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Avoids silent loss of critical assumptions or unsupported target behavior.

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
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-15-02
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-074
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-15-02

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
