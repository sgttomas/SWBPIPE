# Context: DEL-06-01

**Deliverable ID:** DEL-06-01
**Name:** Rule-pack schema
**Package ID:** PKG-06
**Package Name:** Rule Packs and User-Supplied Code Check Engine
**Type:** DATA_MODEL_CHANGE

## Description
Define the rule-pack artifact schema: metadata, required inputs, formulas, allowables, statuses, and redistribution flags.

## Anticipated Artifacts
- schemas/rule_pack.schema.yaml
- docs/SPEC.md update

## Scope Coverage
- SOW-016
- SOW-042

## Objective Support
- OBJ-005

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Schema and examples only.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Schema and examples only.

## Package Reference
- **Package:** PKG-06 Rule Packs and User-Supplied Code Check Engine
- **Package Scope:** Implements the schema, sandboxed evaluator, required-input checks, and private lifecycle for rule packs.
- **Package Assigned Scope Items:** SOW-004, SOW-016, SOW-042, SOW-045
- **Package Exclusions:** Does not ship ASME or other proprietary rule content.

## Decomposition Reference
- **Decomposition:** docs/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.4
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-06-01
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-016,SOW-042
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-06-01


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
