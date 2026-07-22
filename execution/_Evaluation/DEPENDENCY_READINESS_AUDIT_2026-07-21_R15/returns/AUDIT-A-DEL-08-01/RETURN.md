# Agent 2 Return — AUDIT-A-DEL-08-01

**Frozen basis:** `0c066652cd527eb1559f715e914262d2bda42602`

## Per-edge findings

### DAG-002-E0522 — DEL-02-05

- Recorded DAG/local status: `TBD` / `TBD` (`execution/_DAG/DAG-007/DependencyEdges.csv:629`; DEL-08-01 `Dependencies.csv:12`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: a stable persistence/round-trip contract usable by the report boundary, not every future container and migration feature.
- Evidence: DEL-02-05 `_STATUS.md:16-20`; `core/project_persistence/service.py:93-170,252-330,464-477`; `core/reporting/report_generator/src/lib.rs:233-261,536-541`; commit `101dcb420b5d63e597e87c7574e876b7bffedc56`.
- Contrary evidence: DEL-02-05 retains explicit migrate, compatibility-window, `.opsproj`, and wasm residuals at `_STATUS.md:7-9`; the report crate intentionally does not parse project files (`report_generator/src/lib.rs:3-8`). These do not defeat the semantic predecessor.
- Repair implication: refresh satisfaction while preserving DEL-02-05 lifecycle and residuals.

### DAG-002-E0523 — DEL-05-03

- Recorded DAG/local status: `TBD` / `TBD` (`DependencyEdges.csv:630`; local `Dependencies.csv:13`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: defined mechanics-stress result semantics suitable for report result-envelope consumption, not rule allowables or professional validation.
- Evidence: DEL-05-03 `_STATUS.md:11-16`; `core/loads/stress_recovery/src/lib.rs:1-7,59-109,327-362,490-540`; `core/reporting/report_generator/src/lib.rs:12-20,484-499,555-561`; commit `c6430cbd5c9b222260fc53af27212807b09bd84f`.
- Contrary evidence: application-service ownership, code/rule stress mapping, production tolerances, and professional reliance remain outside the slice (`MEMORY.md:257-260,299-303`).
- Repair implication: record only the bounded mechanics-stress predecessor as satisfied.

### DAG-002-E0524 — DEL-05-04

- Recorded DAG/local status: `TBD` / `TBD` (`DependencyEdges.csv:631`; local `Dependencies.csv:14`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: canonical separation of mechanics, rule-check, incomplete-data, and human-review/acceptance states.
- Evidence: DEL-05-04 `_STATUS.md:13-17`; `schemas/analysis_status.schema.yaml:94-117,160-195`; `core/reporting/report_generator/src/lib.rs:23-33,484-496,610-625`; DAG-007 analogous satisfied rows `DAG-002-E0453` and `DEL-12-03-E002` at `DependencyEdges.csv:322,1011`.
- Contrary evidence: a future stale-hash acceptance-reuse negative remains at `_STATUS.md:6-7`; it does not unset the report vocabulary.
- Repair implication: refresh without claiming acceptance-runtime completion.

### DAG-002-E0525 — DEL-06-04

- Recorded DAG/local status: `TBD` / `TBD` (`DependencyEdges.csv:632`; local `Dependencies.csv:15`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: stable identity, version, privacy, provenance, checksum, redaction-marker, and manifest-reference semantics for report use.
- Evidence: DEL-06-04 `_STATUS.md:17-22`; `core/rules/rule_pack_lifecycle/src/lib.rs:204-252,372-449`; `core/reporting/report_generator/src/lib.rs:284-304,577-597`; analogous satisfied DAG rows at `DependencyEdges.csv:994,1026`.
- Contrary evidence: result-envelope binding, future adapter dispatch, binary partitioning, and numeric-unit ownership remain open at `_STATUS.md:7-9`.
- Repair implication: record satisfaction only for report-safe rule-pack metadata/checksum semantics.

### DAG-002-E0526 — DEL-08-02

- Recorded DAG/local status: `TBD` / `TBD` (`DependencyEdges.csv:633`; local `Dependencies.csv:16`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: defined hashes, solver-version evidence, unit-system reference, rule-pack checksum capture, and validation.
- Evidence: DEL-08-02 `_STATUS.md:12-19`; `core/reporting/audit_manifest/src/lib.rs:245-254,289-368,416-468`; `core/reporting/report_generator/src/lib.rs:12-20,484-490,562-568`; commit `0b7a24c93e2c68ad18883b7d9d660b823ba15d67`.
- Contrary evidence: lifecycle remains `IN_PROGRESS`, but `Remaining` is empty; transport/container concerns are outside the precise edge.
- Repair implication: refresh without promoting lifecycle or container completion.

### DAG-002-E0527 — DEL-08-03

- Recorded DAG/local status: `TBD` / `TBD` (`DependencyEdges.csv:634`; local `Dependencies.csv:17`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: stable disclosure structures and validation for diagnostics, status, provenance, user values, assumptions, limitations, and review-needed TBDs.
- Evidence: DEL-08-03 `_STATUS.md:12-17`; `core/reporting/report_sections/src/lib.rs:259-330,346-388,404-476`; `core/reporting/report_generator/src/lib.rs:12-20,484-490,569-575,765-778`; `core/reporting/report_renderer/src/lib.rs:841-880`; analogous satisfied DAG row at `DependencyEdges.csv:911`.
- Contrary evidence: lifecycle remains `IN_PROGRESS`; the contract is nevertheless implemented, validated, and consumed.
- Repair implication: record contract-grain satisfaction only.

### DAG-002-E0528 — DEL-01-04

- Recorded DAG/local status: `TBD` / `TBD` (`DependencyEdges.csv:635`; local `Dependencies.csv:18`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: a defined report claims boundary and separation between software results and human acceptance.
- Evidence: DEL-01-04 `_STATUS.md:11-15`; `docs/PROFESSIONAL_BOUNDARY.md:49-58,81-114,132-150`; `docs/report_notice_template.md:35-59`; `core/reporting/report_generator/src/lib.rs:320-350,610-625,1011-1030`; `core/reporting/report_renderer/src/lib.rs:704-739`.
- Contrary evidence: owner-class final acceptance, legal authority, release-label, and acceptance-record choices remain open (`PROFESSIONAL_BOUNDARY.md:168-176`).
- Repair implication: record only satisfaction of the report-claims constraint.

## Contradictions

- DAG-007 and the DEL-08-01 register are field-equivalent for all seven rows; both retain `TBD`.
- The contradiction is temporal: `LastSeen=2026-06-16` versus committed semantic contracts, implementation, report-consumer bindings, and later currentness evidence.
- DAG-007 records analogous target/maturity edges as `SATISFIED` for other consumers while preserving the same broader residuals.
- No exact-scope deferral for these seven DEL-08-01 dependencies was found.
- All conclusions are advisory; recorded `TBD` controls until a governed successor is accepted.
