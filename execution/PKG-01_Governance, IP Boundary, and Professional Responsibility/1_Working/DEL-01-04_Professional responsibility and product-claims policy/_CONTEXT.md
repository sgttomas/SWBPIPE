# Context: DEL-01-04

**Deliverable ID:** DEL-01-04
**Name:** Professional responsibility and product-claims policy
**Package ID:** PKG-01
**Package Name:** Governance, IP Boundary, and Professional Responsibility
**Type:** DOC_UPDATE

## Description
Define permitted/prohibited claims, report disclaimers, and human approval boundaries.

## Anticipated Artifacts
- docs/PROFESSIONAL_BOUNDARY.md
- report notice template

## Scope Coverage
- SOW-034

## Objective Support
- OBJ-011

## Context Envelope
- **Envelope:** S
- **Envelope Notes:** Policy wording; no code.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Policy wording; no code.

## Package Reference
- **Package:** PKG-01 Governance, IP Boundary, and Professional Responsibility
- **Package Scope:** Defines the product’s legal, governance, contribution, and professional-boundary rules.
- **Package Assigned Scope Items:** SOW-001, SOW-003, SOW-028, SOW-034, SOW-048
- **Package Exclusions:** Does not implement solver or GUI behavior except through requirements and policies.

## Decomposition Reference
- **Decomposition:** docs/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.4
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-01-04
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-034
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-01-04


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-06, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
