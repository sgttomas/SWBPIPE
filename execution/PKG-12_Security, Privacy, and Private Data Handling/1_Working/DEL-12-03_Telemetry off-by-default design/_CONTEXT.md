# Context: DEL-12-03

**Deliverable ID:** DEL-12-03
**Name:** Telemetry off-by-default design
**Package ID:** PKG-12
**Package Name:** Security, Privacy, and Private Data Handling
**Type:** SECURITY_CONTROL

## Description
Define and implement telemetry policy with opt-in only behavior and no private engineering data transmission.

## Anticipated Artifacts
- telemetry policy
- config defaults
- tests

## Scope Coverage
- SOW-037

## Objective Support
- OBJ-010

## Context Envelope
- **Envelope:** S
- **Envelope Notes:** May be no-op in MVP.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** May be no-op in MVP.

## Package Reference
- **Package:** PKG-12 Security, Privacy, and Private Data Handling
- **Package Scope:** Implements local-first storage policies, redaction/export controls, telemetry constraints, and threat modeling.
- **Package Assigned Scope Items:** SOW-029, SOW-037, SOW-040
- **Package Exclusions:** Does not operate as a cloud service unless separately authorized.

## Decomposition Reference
- **Decomposition:** docs/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.4
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-12-03
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-037
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-12-03


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
