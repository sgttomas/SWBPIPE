# Agent 2 Return — AUDIT-B-DEL-10-05

**Frozen basis:** `0c066652cd527eb1559f715e914262d2bda42602`

## Per-edge findings

### DEP-10-05-E003 — DEL-08-04

- Recorded DAG/local status: `TBD` / `TBD` (`execution/_DAG/DAG-007/DependencyEdges.csv:871`; DEL-10-05 `Dependencies.csv:10`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: a stable consumable result-export interface, not lifecycle closure or the downstream CLI binding.
- Evidence: DEL-08-04 `_STATUS.md:13-17`; commit `723c95b0f`; `core/runner/headless/Cargo.toml:24-26`; `core/runner/headless/src/result_envelope_binding.rs:369-383,487-514`; `core/runner/headless/src/lib.rs:847-870`.
- Contrary evidence: `export-results` remains a downstream stub (`openpipestress-runner.rs:268-274`), and target-format witness hardening remains at DEL-08-04 `_STATUS.md:6-7`; neither negates the provider interface.
- Repair implication: refresh the edge; retain the CLI binding as separate DEL-10-05 work.

### DEP-10-05-E004 — DEL-10-04

- Recorded DAG/local status: `TBD` / `TBD` (`DependencyEdges.csv:872`; local `Dependencies.csv:11`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: provider-neutral build/automation compatibility, expressly excluding provider, manifest, and release-matrix selection.
- Evidence: DEL-10-04 `_STATUS.md:18-25`; `core/runner/headless/Cargo.toml:11-17`; `docs/BUILD_AND_RELEASE.md:20-41,133-170`; DEL-10-05 `ScopeOfWork.md:140-145`.
- Contrary evidence: hosted/public CI remains owner-gated (`BUILD_AND_RELEASE.md:34-41`; DEL-10-04 `_STATUS.md:6-8`).
- Repair implication: refresh while preserving publication and signing gates.

### DEP-10-05-E005 — DEL-02-02

- Recorded DAG/local status: `TBD` / `TBD` (`DependencyEdges.csv:873`; local `Dependencies.csv:12`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: explicit unit-system identity and no silent unit loss/default across runner input/output, not every project-wide unit-policy residual.
- Evidence: DEL-02-02 `_STATUS.md:19-24`; DEL-10-05 `_REVIEW.md:31`; `core/runner/headless/src/lib.rs:316-327,413-422`; `core/runner/headless/src/result_envelope_binding.rs:487-500`.
- Contrary evidence: aliases, diagnostic namespace, broader wiring, and numeric-normalization validation remain open at DEL-02-02 `_STATUS.md:6-11`.
- Repair implication: refresh the bounded runner-unit edge only.

### DEP-10-05-E006 — DEL-02-05

- Recorded DAG/local status: `TBD` / `TBD` (`DependencyEdges.csv:874`; local `Dependencies.csv:13`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: a settled structured-data and persistence/hash strategy, not the later `.opsproj` container or persisted-project CLI input.
- Evidence: DEL-02-05 `_STATUS.md:14-20`; DEL-10-05 `_REVIEW.md:34`; `schemas/headless_runner.schema.yaml:417-458`; `core/runner/headless/src/lib.rs:917-928`.
- Contrary evidence: migrate, compatibility window, `.opsproj`, partitioning, and wasm migration remain open at DEL-02-05 `_STATUS.md:6-9`; persisted-project input is separate future work (`ScopeOfWork.md:57-64,182-184`).
- Repair implication: refresh only the persistence/hash edge.

### DEP-10-05-E007 — DEL-08-02

- Recorded DAG/local status: `TBD` / `TBD` (`DependencyEdges.csv:875`; local `Dependencies.csv:14`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: deterministic serialization, hashes, audit-manifest identity, and reproducibility references where hashes are emitted; not full report-package assembly.
- Evidence: DEL-08-02 `_STATUS.md:6-19`; `core/runner/headless/src/lib.rs:366-379,917-928`; `core/runner/headless/src/result_envelope_binding.rs:464-514`; `core/reporting/audit_manifest/src/lib.rs:244-266,289-360`.
- Contrary evidence: the runner carries an audit-manifest reference rather than assembling the complete report package.
- Repair implication: refresh without making a report-package closure claim.

### DEP-10-05-E008 — DEL-04-06

- Recorded DAG/local status: `TBD` / `TBD` (`DependencyEdges.csv:876`; local `Dependencies.csv:15`).
- Classification: `SATISFIED_IN_FACT_BUT_STALE` (`HIGH`).
- Required maturity: structured solver diagnostics and fail-closed missing-data behavior, not lifecycle closure.
- Evidence: DEL-04-06 `_STATUS.md:6-16`; `core/solver/diagnostics/src/lib.rs:283-301,544-567`; `core/runner/headless/src/lib.rs:394-447`; `core/runner/headless/src/result_envelope_binding.rs:254-315`.
- Contrary evidence: lifecycle remains `IN_PROGRESS`, and the earlier CHECKING transition explicitly made no dependency-closure claim (`_STATUS.md:16-19`).
- Repair implication: refresh the precise diagnostics edge only.

## Contradictions and deferral boundary

- DAG-007 and the local register agree exactly on six `ACTIVE/TBD` rows, while direct interface and implementation evidence supports satisfaction in fact.
- The 2026-06-07 ruling deferred the rows only for the then-current bounded runner-contract boundary (`DEL-10-05/_REVIEW.md:86-106,114-129`); it did not promote them or authorize later cross-boundary work.
- R12 selected benchmark/regression payloads and expressly excluded `export-results` (`execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-10-05_RUNNER_PAYLOAD_BINDINGS.md:21-25`); lines 107-128 state that tranche neither resolved nor depended on the six rows.
- Consolidated ScopeOfWork setup wording that still calls benchmark/regression verbs stubbed is overtaken by R12 implementation under the document's own currentness declaration (`ScopeOfWork.md:72-79,109-115,167-184`).
