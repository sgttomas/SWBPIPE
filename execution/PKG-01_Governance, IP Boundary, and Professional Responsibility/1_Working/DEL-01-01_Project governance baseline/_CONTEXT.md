# Context: DEL-01-01

**Deliverable ID:** DEL-01-01
**Name:** Project governance baseline
**Package ID:** PKG-01
**Package Name:** Governance, IP Boundary, and Professional Responsibility
**Type:** DOC_UPDATE

## Description
Maintain the project governance baseline, visible docs index, and maintainer
policy skeleton for the public project while preserving unresolved governance
choices as `TBD`.

## Anticipated Artifacts
- docs/CONTRACT.md
- docs/DIRECTIVE.md
- governance/MAINTAINERS.md

## Scope Coverage
- SOW-001
- SOW-048

## Objective Support
- OBJ-001
- OBJ-002

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Single governance surface; policy language only.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Single governance surface; policy language only.

## Package Reference
- **Package:** PKG-01 Governance, IP Boundary, and Professional Responsibility
- **Package Scope:** Defines the product’s legal, governance, contribution, and professional-boundary rules.
- **Package Assigned Scope Items:** SOW-001, SOW-003, SOW-028, SOW-034, SOW-048
- **Package Exclusions:** Does not implement solver or GUI behavior except through requirements and policies.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.7
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-01-01
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-001,SOW-048
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-01-01


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-06, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## Current-Basis Refresh Notes
- The original PREPARATION scaffold was structural only.
- Later governance artifacts now exist at `docs/README.md` and
  `governance/MAINTAINERS.md`; current-basis refresh work may update those
  surfaces when explicitly authorized.
- This context does not authorize lifecycle transition, release acceptance,
  license selection, legal conclusion, professional approval, or code-compliance
  claim.
