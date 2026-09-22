# Context: DEL-06-05

**Deliverable ID:** DEL-06-05
**Name:** Invented non-code example rule pack
**Package ID:** PKG-06
**Package Name:** Rule Packs and User-Supplied Code Check Engine
**Type:** DOC_UPDATE

## Description
Create a demonstration rule pack using invented values and explicit non-engineering notices.

## Anticipated Artifacts
- examples/rule_packs/invented_demo.yaml
- docs/example notice

## Scope Coverage
- SOW-016

## Objective Support
- OBJ-005
- OBJ-011

## Context Envelope
- **Envelope:** S
- **Envelope Notes:** Example only; no proprietary content.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Example only; no proprietary content.

## Package Reference
- **Package:** PKG-06 Rule Packs and User-Supplied Code Check Engine
- **Package Scope:** Implements the schema, sandboxed evaluator, required-input checks, and private lifecycle for rule packs.
- **Package Assigned Scope Items:** SOW-004, SOW-016, SOW-042, SOW-045
- **Package Exclusions:** Does not ship ASME or other proprietary rule content.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Authority basis:** Resolve `execution/_Decomposition/SOFTWARE_DECOMP.md` and its accepted amendments through `execution/_Coordination/_DECISIONS/_REGISTER.md`; source-state-bound evidence records the evaluated revision.
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-06-05
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-016
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-06-05


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** Applicable `PKG-00 - Software Architecture Runway` constraints are carried by the consolidated `ArchitectureBasis.md` references under D-43 / SCA-006 and their cited decisions. Use applicable basis IDs as context constraints; this reference makes no lifecycle or issuance assertion.
- **Decomposition Reference:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; accepted decisions and amendments are recorded in `execution/_Coordination/_DECISIONS/_REGISTER.md`.
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Implementation choices and open matters:** DEC-012 permits resolution in a sealed brief or later human ruling. Consult the accepted decisions in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12: solver strategy DEC-023; expression grammar DEC-022; migration/versioning DEC-019/033; package/export baseline DEC-028 and SCA-004; CI/coverage DEC-025/059/060/093. Current mechanisms and versions belong to bound implementation evidence. Public transport, per-format contracts, optional retrieval mechanisms and any unselected choices retain their own scope and decision paths; this reference does not close delivery, privacy, review or validation gaps.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
