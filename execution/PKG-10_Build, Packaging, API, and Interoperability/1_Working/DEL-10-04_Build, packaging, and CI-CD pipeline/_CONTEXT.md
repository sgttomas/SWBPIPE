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

## Accepted implementation/build evidence
- D-41 R4 ruling `DEC-074` option O3 attributes the existing root
  `projects/chirality-piping/package.json` workspace manifest to DEL-10-04.
- The manifest is build evidence for the desktop workspace dev/build/test
  scripts and the `generate:product-preview-mechanics` fixture-generator
  command. This clarification does not move implementation, select a CI
  provider, or expand DEL-10-04 scope.
- D-42 ruling `DEC-076` attributes SURF-011,
  `apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx`, to
  DEL-10-04. The panel's embedded packet identifies `DEL-10-04`, `PKG-10`,
  and `SOW-032` and exposes local build/package preview evidence without
  activating CI, packaging, signing, publication, or release authority.

## Scope Coverage
- SOW-032

## Objective Support
- OBJ-008
- OBJ-009

## Context Envelope
- **Envelope:** L
- **Envelope Notes:** Release matrix follows DEC-057, with separately governed evidence-gated expansion.

## Context Budget QA
- **Risk:** WATCH
- **Recommended Action:** Confirm scope and split if it expands
- **Notes:** Release matrix follows DEC-057, with separately governed evidence-gated expansion.

## Package Reference
- **Package:** PKG-10 Build, Packaging, API, and Interoperability
- **Package Scope:** Implements public API/plugin boundaries, import/export adapters, headless execution, local FEA handoff contracts, and release packaging.
- **Package Assigned Scope Items:** SOW-030, SOW-031, SOW-032, SOW-049, SOW-054
- **Package Exclusions:** Does not embed external proprietary tool behavior.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Authority basis:** Resolve `execution/_Decomposition/SOFTWARE_DECOMP.md` and its accepted amendments through `execution/_Coordination/_DECISIONS/_REGISTER.md`; source-state-bound evidence records the evaluated revision.
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-10-04
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-032
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-10-04


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** Applicable `PKG-00 - Software Architecture Runway` constraints are carried by the consolidated `ArchitectureBasis.md` references under D-43 / SCA-006 and their cited decisions. Use applicable basis IDs as context constraints; this reference makes no lifecycle or issuance assertion.
- **Decomposition Reference:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; accepted decisions and amendments are recorded in `execution/_Coordination/_DECISIONS/_REGISTER.md`.
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; SCA-003 local SQLite project store/index profile with rebuildable SQLite FTS5/BM25 retrieval sidecars; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Implementation choices and holds:** Apply DEC-012 through accepted bounded briefs or later rulings; do not copy the original SCA-001 TBD inventory as current status. The decision log in `execution/_Decomposition/SOFTWARE_DECOMP.md` records the storage/migration basis (DEC-017/019), expression grammar (DEC-022), numerical-library basis (DEC-023), package/container and naming (DEC-028/057), and CI/coverage posture (DEC-025/059/060/093); SCA-004 carries the adopted export scope. Exact dependency versions and local mechanisms follow accepted implementation evidence. Public transport, unadopted interfaces, numeric floor values, private-data/provider choices and any unfulfilled delivery or acceptance obligations remain governed separately; a ruled choice is not proof of implementation or release.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-003 Storage Profile Injection
- **Packaging Rule:** MVP desktop packaging should bundle or link the SQLite/FTS5 capability required by the local storage profile without requiring a hosted service, daemon, or network connection.
- **Dependency Rule:** NumPy is not a required MVP storage dependency; any NumPy retrieval cache remains optional and rebuildable unless later TASK evidence promotes it.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
