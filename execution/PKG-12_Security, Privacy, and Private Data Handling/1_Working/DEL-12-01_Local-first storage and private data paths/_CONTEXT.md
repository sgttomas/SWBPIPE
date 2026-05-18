# Context: DEL-12-01

**Deliverable ID:** DEL-12-01
**Name:** Local-first storage and private data paths
**Package ID:** PKG-12
**Package Name:** Security, Privacy, and Private Data Handling
**Type:** SECURITY_CONTROL

## Description
Define and implement private data storage conventions outside public repository paths.

## Anticipated Artifacts
- storage policy
- private path conventions
- tests

## Scope Coverage
- SOW-029

## Objective Support
- OBJ-010

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Local-first policy must align with the SCA-003 local SQLite storage profile and no-network/no-cloud MVP posture.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Local-first policy must align with the SCA-003 local SQLite storage profile and no-network/no-cloud MVP posture.

## Package Reference
- **Package:** PKG-12 Security, Privacy, and Private Data Handling
- **Package Scope:** Implements local-first storage policies, redaction/export controls, telemetry constraints, and threat modeling.
- **Package Assigned Scope Items:** SOW-029, SOW-037, SOW-040
- **Package Exclusions:** Does not operate as a cloud service unless separately authorized.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.6
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-12-01
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-029
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-12-01


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.6
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; SCA-003 local SQLite project store/index profile with rebuildable SQLite FTS5/BM25 retrieval sidecars; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, migration framework/tooling, DB migration implementation details, binary asset/export packaging, and optional retrieval-cache implementation details remain implementation-level decisions unless resolved under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-003 Storage Profile Injection
- **Storage Decision:** MVP storage is local-only SQLite bundled with the desktop app where needed; no hosted DB, daemon, required network, cloud sync, or telemetry path is authorized.
- **Privacy Rule:** Private project, rule-pack, library, report, diagnostic, and generated-output data remain user-controlled local data.
- **External File Rule:** Large files are referenced in place by path/URI plus hash and privacy/provenance metadata by default; copying into export packages requires a later explicit user-reviewed workflow.
- **Sidecar Rule:** Retrieval sidecars are rebuildable local caches and must not become canonical storage or private-data transmission paths.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
