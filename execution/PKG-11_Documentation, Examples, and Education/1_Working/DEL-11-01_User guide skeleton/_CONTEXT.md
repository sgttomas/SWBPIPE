# Context: DEL-11-01

**Deliverable ID:** DEL-11-01
**Name:** User guide skeleton
**Package ID:** PKG-11
**Package Name:** Documentation, Examples, and Education
**Type:** DOC_UPDATE

## Description
Create user guide structure for project setup, modeling, solving, rule checks, reports, and limitations.

## Anticipated Artifacts
- docs/user_guide/index.md

## Scope Coverage
- SOW-033

## Objective Support
- OBJ-001
- OBJ-011

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Documentation only.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Documentation only.

## Package Reference
- **Package:** PKG-11 Documentation, Examples, and Education
- **Package Scope:** Produces user/developer guides, theory notes, tutorials, and invented-data example models.
- **Package Assigned Scope Items:** SOW-033
- **Package Exclusions:** Does not publish protected standards examples or commercial software examples.

## Decomposition Reference
- **Decomposition:** docs/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.4
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-11-01
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-033
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-11-01


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
