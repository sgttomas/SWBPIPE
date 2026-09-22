# Context: DEL-17-06

**Deliverable ID:** DEL-17-06
**Name:** Stress-neutral CSV/JSON package
**Package ID:** PKG-17
**Package Name:** Export Format Interoperability
**Type:** BACKEND_FEATURE_SLICE

## Description
Define and implement stress-neutral CSV/JSON exports for review, regression comparison, and downstream tooling without representing code-compliance or professional-approval semantics.

## Anticipated Artifacts
- stress-neutral CSV schema
- stress-neutral JSON schema
- export writer
- comparison fixtures

## Scope Coverage
- SOW-046
- SOW-074

## Objective Support
- OBJ-007
- OBJ-017
- OBJ-018

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Result/export assurance target sequenced after export package contracts and run/result records.

## Package Reference
- **Package:** PKG-17 Export Format Interoperability
- **Package Scope:** Implements deterministic export-format contracts, source-evidence basis, target profiles, stable ID maps, loss reports, external validation harness boundaries, review geometry exports, and adapter SDK surfaces for downstream interoperability.
- **Package Assigned Scope Items:** SOW-030, SOW-046, SOW-074, SOW-075
- **Package Exclusions:** Does not bundle commercial solvers, embed proprietary examples, reverse engineer protected formats, bypass licenses, or claim code compliance, professional acceptance, or formal solver validation.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Authority basis:** Resolve `execution/_Decomposition/SOFTWARE_DECOMP.md` and its accepted amendments through `execution/_Coordination/_DECISIONS/_REGISTER.md`; source-state-bound evidence records the evaluated revision.
- **Status:** current_basis_after_SCA-004

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-17-06
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-046,SOW-074
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-17-06

## Architecture Basis Injection
- **Scope Change Basis:** SCA-001 architecture constraints with accepted amendments recorded in `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`. Historical setup/amendment dates below retain their original reach.
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08.
- **Resolved Baseline:** Rust core/application services where implementation-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; explicit deterministic JSON hash byte-basis metadata (`deterministic_sorted_compact_json_payload_hash` for the retained 0.1 Python profile, explicitly non-JCS, per D-41 R5 T2A; the D-67 0.2 profile is `openpipestress_jcs_ijson_v1`); no-bypass adapter/plugin boundaries; protected-content validation gates as applicable.
- **Dispatch Rule:** Future TASK execution must apply only applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-004 Control-Surface Note
- This control surface was created by PREPARATION-style scaffolding on 2026-05-18 from SOFTWARE_DECOMP revision 0.7 and companion registers.
- This pass does not produce production four-document content, implementation artifacts, lifecycle promotion beyond OPEN, release claims, professional claims, or DAG-005 approval.

## D-67 bounded versioned compatibility adoption — 2026-09-14

The existing Python sorted-compact JSON requirement above remains the 0.1 profile. The owner-approved 0.2 successor uses the checked shared Rust `openpipestress_jcs_ijson_v1` profile, preserves received checksum evidence separately, and retains the separately labeled CSV contract. See `ScopeOfWork.md` CLM-042 and `execution/_Coordination/_DECISIONS/D-67_RULING_2026-09-14.md`. This bounded contract amendment does not change decomposition, lifecycle or dependency satisfaction.
