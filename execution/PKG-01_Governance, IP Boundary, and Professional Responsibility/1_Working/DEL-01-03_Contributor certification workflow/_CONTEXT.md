# Context: DEL-01-03

**Deliverable ID:** DEL-01-03
**Name:** Contributor certification workflow
**Package ID:** PKG-01
**Package Name:** Governance, IP Boundary, and Professional Responsibility
**Type:** DOC_UPDATE

## Description
Define contributor attestations, provenance fields, review routing, and rejection rules for public data contributions.

## Anticipated Artifacts
- CONTRIBUTING.md section
- contributor certification template

## Scope Coverage
- SOW-028
- SOW-048

## Objective Support
- OBJ-002

## Context Envelope
- **Envelope:** S
- **Envelope Notes:** Bounded policy artifact.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Bounded policy artifact.

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
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-01-03
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-028,SOW-048
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-01-03


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-06, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, concrete adapter/export target behavior, CI provider/coverage thresholds, and final license/contributor legal mechanism/reviewer authority/release governance remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## Current-Basis Notes
- The original PREPARATION scaffold was structural only.
- `CONTRIBUTING.md`, `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`, and
  `governance/CONTRIBUTION_REVIEW_CHECKLIST.md` now exist as draft repo-level
  governance artifacts connected to this deliverable.
- This context does not authorize lifecycle transition, final contributor legal
  mechanism selection, license selection, legal conclusion, maintainer or
  reviewer authority assignment, professional approval, code-compliance claim,
  or public acceptance of any specific contributed data.
