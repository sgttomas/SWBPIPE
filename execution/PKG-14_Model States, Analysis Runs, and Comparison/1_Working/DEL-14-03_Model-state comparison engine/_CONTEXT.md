# Context: DEL-14-03

**Deliverable ID:** DEL-14-03
**Name:** Model-state comparison engine
**Package ID:** PKG-14
**Package Name:** Model States, Analysis Runs, and Comparison
**Type:** BACKEND_FEATURE_SLICE

## Description
Implement deterministic state diffs using stable IDs and explicit mapping records.

## Anticipated Artifacts
- state comparison engine
- state diff tests

## Scope Coverage
- SOW-073
- SOW-071

## Scope Detail
- SOW-073: The product shall compare two model states and/or two analysis runs deterministically using stable IDs, manual mappings where required, unit-normalized result deltas, and tolerance profiles.- SOW-071: The product shall save named immutable model states with tags, notes, external references, unresolved assumptions, warnings, and deterministic hashes.
## Objective Support
- OBJ-016

## Context Envelope
- **Envelope:** L
- **Envelope Notes:** Reports added/removed/changed/unchanged model entities.

## Context Budget QA
- **Risk:** WATCH
- **Recommended Action:** Confirm scope and split if it expands
- **Notes:** Reports added/removed/changed/unchanged model entities.

## Package Reference
- **Package:** PKG-14 Model States, Analysis Runs, and Comparison
- **Package Scope:** Implements immutable model-state records, analysis-run records, deterministic state/run comparison, mappings, tolerances, and comparison exports.
- **Package Assigned Scope Items:** SOW-071, SOW-072, SOW-073
- **Package Exclusions:** Does not ingest commercial prover outputs comprehensively or determine external validation.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Authority basis:** Resolve `execution/_Decomposition/SOFTWARE_DECOMP.md` and its accepted amendments through `execution/_Coordination/_DECISIONS/_REGISTER.md`; source-state-bound evidence records the evaluated revision.
- **Status:** current_basis_for_control_surface_refresh

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-14-03
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-073, SOW-071
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-14-03

## Architecture Basis Injection
- **Scope Change Basis:** SCA-001 architecture constraints with accepted amendments recorded in `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`. Historical setup/amendment dates below retain their original reach.
- **Architecture Basis:** Applicable `PKG-00 - Software Architecture Runway` constraints are carried by the consolidated `ArchitectureBasis.md` references under D-43 / SCA-006 and their cited decisions. Use applicable basis IDs as context constraints; this reference makes no lifecycle or issuance assertion.
- **Decomposition Reference:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; accepted decisions and amendments are recorded in `execution/_Coordination/_DECISIONS/_REGISTER.md`.
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Implementation choices and resolved decisions:** Apply DEC-012 at its actual reach: a sealed brief or later human ruling may resolve an implementation choice. DEC-017 fixes the SQLite local-store/canonical-JSON profile; DEC-028 fixes the project interchange container; DEC-022 fixes the rule grammar; SCA-004 establishes the named export scope. DEC-025 and DEC-060 govern their adopted verification/coverage portions. Exact dependencies, numerical library, public transport, format-specific contracts and other unselected details remain subject to their applicable accepted brief or decision; no global claim that these topics are all still undecided is made.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-002 Control-Surface Refresh Note
- This control surface was created by PREPARATION on 2026-05-03 from the accepted revision 0.7 decomposition and companion registers.
- This pass did not dispatch Type 2 work, produce implementation artifacts, promote candidate edges, refresh the blocker queue, materialize local `Dependencies.csv`, or promote the quarantined Chirality corpus.

## PREPARATION Notes
- Structural scaffold and metadata context only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
