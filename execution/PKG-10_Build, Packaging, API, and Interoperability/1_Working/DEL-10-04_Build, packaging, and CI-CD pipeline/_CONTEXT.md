# Context: DEL-10-04

**Deliverable ID:** DEL-10-04
**Name:** Build, packaging, and CI/CD pipeline
**Package ID:** PKG-10
**Package Name:** Build, Packaging, API, and Interoperability
**Type:** CI_CD_CHANGE

## Description
Implement reproducible builds, tests, packaging skeleton, and release artifacts for supported platforms.

## Anticipated Artifacts
- CI workflows
- packaging scripts
- release notes template

## Scope Coverage
- SOW-032

## Objective Support
- OBJ-008
- OBJ-009

## Context Envelope
- **Envelope:** L
- **Envelope Notes:** Platform list TBD.

## Context Budget QA
- **Risk:** WATCH
- **Recommended Action:** Confirm scope and split if it expands
- **Notes:** Platform list TBD.

## Package Reference
- **Package:** PKG-10 Build, Packaging, API, and Interoperability
- **Package Scope:** Implements public API/plugin boundaries, import/export adapters, headless execution, local FEA handoff contracts, and release packaging.
- **Package Assigned Scope Items:** SOW-030, SOW-031, SOW-032, SOW-049, SOW-054
- **Package Exclusions:** Does not embed external proprietary tool behavior.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.6
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-10-04
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-032
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-10-04


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.6
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; SCA-003 local SQLite project store/index profile with rebuildable SQLite FTS5/BM25 retrieval sidecars; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, migration framework/tooling, DB migration implementation details, binary asset/export packaging, and optional retrieval-cache implementation details remain implementation-level decisions unless resolved under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-003 Storage Profile Injection
- **Packaging Rule:** MVP desktop packaging should bundle or link the SQLite/FTS5 capability required by the local storage profile without requiring a hosted service, daemon, or network connection.
- **Dependency Rule:** NumPy is not a required MVP storage dependency; any NumPy retrieval cache remains optional and rebuildable unless later TASK evidence promotes it.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
