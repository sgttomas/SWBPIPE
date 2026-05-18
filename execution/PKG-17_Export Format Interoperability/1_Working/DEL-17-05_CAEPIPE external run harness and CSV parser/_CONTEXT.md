# Context: DEL-17-05

**Deliverable ID:** DEL-17-05
**Name:** CAEPIPE external run harness and CSV parser
**Package ID:** PKG-17
**Package Name:** Export Format Interoperability
**Type:** BACKEND_FEATURE_SLICE

## Description
Implement an optional external harness that invokes a user-provided licensed CAEPIPE executable and parses documented CSV outputs for regression and handoff validation evidence.

## Anticipated Artifacts
- external CAEPIPE harness
- run directory contract
- CSV parser
- skip-without-executable tests

## Scope Coverage
- SOW-030
- SOW-046
- SOW-075

## Objective Support
- OBJ-007
- OBJ-009
- OBJ-017
- OBJ-018

## Context Envelope
- **Envelope:** L
- **Envelope Notes:** Harness remains optional and user-owned; no solver binary, license bypass, or professional acceptance claim.

## Package Reference
- **Package:** PKG-17 Export Format Interoperability
- **Package Scope:** Implements deterministic export-format contracts, source-evidence basis, target profiles, stable ID maps, loss reports, external validation harness boundaries, review geometry exports, and adapter SDK surfaces for downstream interoperability.
- **Package Assigned Scope Items:** SOW-030, SOW-046, SOW-074, SOW-075
- **Package Exclusions:** Does not bundle commercial solvers, embed proprietary examples, reverse engineer protected formats, bypass licenses, or claim code compliance, professional acceptance, or formal solver validation.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.7
- **Status:** current_basis_after_SCA-004

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-17-05
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-030,SOW-046,SOW-075
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-17-05

## Architecture Basis Injection
- **Scope Changes:** SCA-001 architecture basis as amended by SCA-003 and SCA-004.
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08.
- **Resolved Baseline:** Rust core/application services where implementation-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis for JSON payloads; no-bypass adapter/plugin boundaries; protected-content validation gates as applicable.
- **Dispatch Rule:** Future TASK execution must apply only applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-004 Control-Surface Note
- This control surface was created by PREPARATION-style scaffolding on 2026-05-18 from SOFTWARE_DECOMP revision 0.7 and companion registers.
- This pass does not produce production four-document content, implementation artifacts, lifecycle promotion beyond OPEN, release claims, professional claims, or DAG-005 approval.
