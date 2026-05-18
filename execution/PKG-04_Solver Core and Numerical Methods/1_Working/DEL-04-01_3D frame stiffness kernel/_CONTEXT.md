# Context: DEL-04-01

**Deliverable ID:** DEL-04-01
**Name:** 3D frame stiffness kernel
**Package ID:** PKG-04
**Package Name:** Solver Core and Numerical Methods
**Type:** BACKEND_FEATURE_SLICE

## Description
Implement the global 3D frame stiffness assembly, coordinate transforms, boundary conditions, and sparse solve interface.

## Anticipated Artifacts
- core/solver/frame_kernel
- unit tests

## Scope Coverage
- SOW-005
- SOW-035

## Objective Support
- OBJ-003

## Context Envelope
- **Envelope:** L
- **Envelope Notes:** Central solver kernel; large but single domain and foundational.

## Context Budget QA
- **Risk:** WATCH
- **Recommended Action:** Confirm scope and split if it expands
- **Notes:** Central solver kernel; large but single domain and foundational.

## Package Reference
- **Package:** PKG-04 Solver Core and Numerical Methods
- **Package Scope:** Implements global 3D centerline/frame mechanics, straight pipe behavior, supports, nonlinear support logic, diagnostics, and performance harnesses.
- **Package Assigned Scope Items:** SOW-005, SOW-006, SOW-011, SOW-012, SOW-035, SOW-053
- **Package Exclusions:** Does not decide code compliance; produces mechanical results.

## Decomposition Reference
- **Decomposition:** docs/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.4
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-04-01
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-005,SOW-035
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-04-01


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
