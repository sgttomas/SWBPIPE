---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-10-05
package_id: PKG-10
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@00115c71931bcae79909602d653740d3bb72dfa1
project_scope_refs: [SOW-054, SOW-032]
package_objective_refs: [OBJ-008, OBJ-009, OBJ-012]
---

# Scope of Work — DEL-10-05

Reconciliation evidence: the retired D-41 current-declaration snapshots are preserved at source `00115c71931bcae79909602d653740d3bb72dfa1` and in `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`. Current scope and decisions are resolved through `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`; dependency context is resolved through `execution/_DAG/_LATEST.md`. Open work and lifecycle remain governed by `_STATUS.md`. This text repair establishes no lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

## Purpose and Objective Traceability

This Scope of Work defines `DEL-10-05` in service of project scope [SOW-054, SOW-032] and package objectives [OBJ-008, OBJ-009, OBJ-012].

- **OUT-001** — A headless CLI and structured-I/O analysis-runner contract for governed solve, input validation, result export, benchmark, and regression workflows is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-10-05 Headless CLI and structured I/O analysis runner

> #### Datasheet: DEL-10-05 Headless CLI and structured I/O analysis runner
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | DEL-10-05 | `_CONTEXT.md` |
> | Package ID | PKG-10 | `_CONTEXT.md` |
> | Package | Build, Packaging, API, and Interoperability | `_CONTEXT.md` |
> | Type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/TYPES.md` section 3 |
> | Lifecycle target for setup | SEMANTIC_READY | Use the current lifecycle recorded in `_STATUS.md`; preserve its state and history. Contract/evidence checks do not authorize a lifecycle transition or issuance. |
> | Current production posture | The local CLI follows DEC-065. Current solve/validate/export/benchmark/regression bindings are in `core/runner/headless/`; release packaging and public transport remain separately governed. | `DEC-065`; TP-RUNNER-015 |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Primary purpose | Headless or structured I/O solve execution for early solver verification, regression, and automation before full GUI maturity | `docs/_Registers/ScopeLedger.csv` row SOW-054; `docs/_Registers/Deliverables.csv` row DEL-10-05 |
> | Required alignment | Schema-first command/query/job envelopes and result exports | `docs/_Registers/Deliverables.csv` row DEL-10-05; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.1 AB-00-03 |
> | Reproducibility basis | Deterministic, unit-safe model data flow; canonical JSON and JCS-compatible hashing where JSON payload hashes are used | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
> | Diagnostics basis | Result envelopes and diagnostics carry machine-readable warning/error information and must not claim certification or code compliance | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.1 AB-00-06; `docs/CONTRACT.md` invariant OPS-K-AUTH-1 |
> | Build/automation relationship | Must remain compatible with accepted build, packaging and CI policy; evidence actual implementation and unadopted platform/release portions separately | DEC-025 governs the local evidence sweep; DEC-059 conditionally governs public sanitized-export CI, and DEC-093 permits the exact-head-bound surface-4 CI alternative. DEC-060 selects coverage telemetry tooling, not numeric floors. Implementation and release evidence remain separately bound. DEC-057 selects macOS Apple Silicon (`aarch64-apple-darwin`) `.app` distributed as zip with SHA-256 and release records. Windows/Linux require evidence-gated decisions. Current unsigned conditions remain; DEC-089 adopts a policy-only future Developer ID/notarization target without closing its credential, implementation, network, publication or release gates. |
> | Public/private boundary | No protected standards data, proprietary catalog values, private rule packs, or private project data are public defaults | `AGENTS.md`; `docs/DIRECTIVE.md` sections 3-4; `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-PRIV-1 |
>

### CLM-004 — Conditions

> ##### Conditions
>
> | Condition | Status |
> |---|---|
> | Exact CLI command names | Settled for the local runner surface: `openpipestress-runner` with verbs `solve`, `validate-input`, `export-results`, `run-benchmark`, and `run-regression` |
> | Exact structured input schema fields | Use `schemas/headless_runner.schema.yaml` and the current CLI wrapper in `core/runner/headless/src/bin/openpipestress-runner.rs`; preserve source/result envelopes and explicit unsupported/missing-payload diagnostics. |
> | Public API transport | TBD per SCA-001 remaining-TBD boundary |
> | External import/export format list | SCA-004 and the accepted decomposition OI-004/PKG-17 allocation govern named export scope. Per-format interface/provenance guarantees and integration remain separately governed; this does not select a new public API transport or plugin runtime. |
> | CI provider and coverage thresholds | DEC-025 governs the local evidence sweep; DEC-059 conditionally governs public sanitized-export CI, and DEC-093 permits the exact-head-bound surface-4 CI alternative. DEC-060 selects coverage telemetry tooling, not numeric floors. Implementation and release evidence remain separately bound. |
> | Process/network/filesystem policy | Settled by `DEC-065`: single foreground local process; no network, daemon, telemetry, hidden filesystem mutation, repository-default private-data write, user-home scanning, or direct SQL/SQLite bypass; stdout JSON default with explicit `--output` path allowed |
> | Protected-data status | No protected standards text, tables, examples, material allowables, SIF/flexibility tables, proprietary values, or certification claims introduced here |
>

### CLM-005 — Construction

> ##### Construction
>
> The local runner shall consume schema-first requests and produce deterministic structured results through the governed application-service/kernel bindings. Current implementation and payload routing are recorded in `core/runner/headless/src/bin/openpipestress-runner.rs`, `benchmark_binding.rs`, `result_envelope_binding.rs` and `redaction_binding.rs`; export, benchmark and regression are not merely stub vocabulary. Missing or unsupported payloads must remain explicit blocking diagnostics.
>
> Preserve unit/provenance validation, result envelopes, rule sandboxing, private-data/export controls and human professional review. Public transport, external adapters, packaging, signing, publication and lifecycle issuance retain their owning scope and gates.

### CLM-006 — References

> ##### References
>
> - `_CONTEXT.md` for sealed deliverable identity, SOW/OBJ coverage, acceptance/risk notes, and architecture-basis injection.
> - `docs/CONTRACT.md` for invariants OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-RULE-1/2/3, OPS-K-PRIV-1/2, OPS-K-AUTH-1, and OPS-K-AGENT-1..4.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` sections 8, 9, 11, and 12 for architecture basis, scope ledger, open issues, and accepted decisions.
> - `docs/_Registers/Deliverables.csv` row DEL-10-05.
> - `docs/_Registers/ScopeLedger.csv` rows SOW-054 and SOW-032.
> - `docs/SPEC.md` sections 1, 7, 8, 9, and 10 for layer boundaries, diagnostics/reporting, V&V, and deliverable mechanics.
> - `docs/TYPES.md` sections 3, 4, 6, 8, and 9 for deliverable type, analysis-status vocabulary, data boundaries, object registry, and lifecycle semantics.

## Completion and Reliance Basis — Epistemology

### CLM-007 — Specification: DEL-10-05 Headless CLI and structured I/O analysis runner

> #### Specification: DEL-10-05 Headless CLI and structured I/O analysis runner
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — Scope

> ##### Scope
>
> Current deliverable obligation: implement a CLI or equivalent headless interface for schema-driven solve execution, benchmark automation, and report/result export before full GUI maturity. Implementation and record changes require an active bounded brief under `AGENTS.md`; `_STATUS.md` records lifecycle and remaining work. References below to the original setup write boundary apply only to that historical run. They do not exclude later authorized delivery, waive the retained requirements, or authorize issuance.
>
>
> This deliverable defines setup-stage requirements and evidence for a future headless CLI or equivalent structured I/O analysis runner. The future runner exists to support early solver verification, regression automation, and non-GUI execution paths while the full GUI matures.
>
> `DEC-065` settles the local CLI/process policy for this deliverable. The
> current bounded implementation provides the stable `openpipestress-runner`
> binary with `solve`, `validate-input`, `export-results`, `run-benchmark`, and
> `run-regression` verbs. It does not implement release packaging/signing,
> hosted/public transport, external adapters, CI workflows, public publication,
> or lifecycle issuance.
>
### CLM-010 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source | Verification |
> |---|---|---|---|
> | R-01 | The future runner must execute through schema-first command/query/job/result-envelope boundaries rather than direct solver bypasses. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 | Confirm future implementation calls application-service contracts and preserves command/job separation. |
> | R-02 | Structured input and output must remain unit-aware, deterministic, and reproducible. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/_Registers/ScopeLedger.csv` SOW-054 and OBJ-012 | Confirm future fixtures include unit-bearing values and deterministic rerun expectations. |
> | R-03 | Missing solve-required or rule-check-required data must surface as diagnostics, not silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` section 3 | Confirm future runner exits/results include blocking diagnostics for missing required inputs. |
> | R-04 | Result output must align with schema-first result exports and diagnostic/result-envelope contracts. | `docs/_Registers/ScopeLedger.csv` SOW-046 and SOW-061; `docs/_Registers/Deliverables.csv` DEL-10-05 notes | Confirm future outputs can be consumed by regression comparison and report/result export workflows without private-data leakage. |
> | R-05 | Diagnostics must preserve code/class/severity/source/object/remediation/provenance semantics and no compliance/certification claim. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/CONTRACT.md` OPS-K-AUTH-1 | Confirm future error/warning examples include professional-boundary notices where relevant. |
> | R-06 | The local runner must expose stable development/test CLI verbs while keeping CI provider, release matrix, signing/publication, and hosted/public transport outside this implementation. | `docs/_Registers/ScopeLedger.csv` SOW-032; `DEC-065`; `DEC-025`; `DEC-057`; `DEC-059` | Confirm `openpipestress-runner` verbs are stable, package scripts are not release claims, and repo-level CI/release workflow files are not introduced by this tranche. |
> | R-07 | Public examples and fixtures must use original/invented or otherwise permitted data and must not embed protected standards content. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-RULE-1; `docs/DIRECTIVE.md` sections 3-5 | Future fixture review must check protected-content risk and provenance. |
> | R-08 | The runner must not transmit private project, material, component, or rule-pack data by default. | `docs/CONTRACT.md` OPS-K-PRIV-1/2; `docs/DIRECTIVE.md` section 6 | Future runner configuration review must confirm local-first/private-data defaults. |
> | R-09 | Runner results may indicate mechanics solve and user-rule-check statuses but must not use automatic code-compliance status. | `docs/TYPES.md` section 4; `docs/CONTRACT.md` OPS-K-MECH-2 and OPS-K-AUTH-1 | Future status tests must reject misleading compliance/certification wording. |
>

### CLM-011 — Standards

> ##### Standards
>
> | Standard or basis | Applicability | Status |
> |---|---|---|
> | JSON Schema 2020-12 baseline | Governs public schema/interchange posture for future structured I/O | The current headless contract is `schemas/headless_runner.schema.yaml`; preserve schema-first JSON and the exact accepted input/output semantics. |
> | Canonical JSON/JCS-compatible hashing | Applies where JSON payload hashes are used for reproducibility | DEC-017 retains canonical JSON domain truth and a rebuildable SQLite projection; DEC-028 selects a multi-member zip/directory package with member inventory and per-member JCS hashes. DEC-057 names the `.opsproj` package. Storage roots, integration and unmet controls remain separately governed. |
> | Command/query/job/result-envelope baseline | Governs application-service separation for GUI/headless execution | Resolve current Rust input/output records in `core/runner/headless/src/lib.rs` and `schemas/headless_runner.schema.yaml`; preserve the command/query/job/result and GUI/headless separation obligations. |
> | Professional responsibility boundary | Prohibits software or agent claims of certification, sealing, approval, or automatic code compliance (PRD §21.2) | Binding invariant |
>

### CLM-012 — Verification

> ##### Verification
>
> Verify the DEC-065 local command families against their actual accepted payload bindings: solve, validate-input, export-results, run-benchmark and run-regression. The current CLI source and test definitions include named/whole-suite execution, rejection of missing/unsupported cases, controlled result output and blocking diagnostics. Bind any claimed result to the exercised candidate; the TP-RUNNER-015 stub observations remain historical evidence only.
>
> Maintain unit/provenance/result-envelope/privacy checks, deterministic regression evidence and explicit unsupported-state behavior. Missing end-to-end/native/packaging/external-adapter witnesses remain separate delivery obligations; this records no fresh suite result.

### CLM-013 — Documentation

> ##### Documentation
>
> Required setup artifacts for this run:
>
> - `ScopeOfWork.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/*`
> - `_STATUS.md`
>
### CLM-014 — Pass 3 Lensing Addendum

> ##### Pass 3 Lensing Addendum
>
> Semantic lensing identified implementation-sensitive TBDs. `DEC-065` resolves
> the local CLI command names, local process policy, network prohibition,
> filesystem posture, and package-script posture for TP-RUNNER-015. Public API transport, external adapter integration and unresolved storage/automation matters retain their owning decisions. CI and release-matrix policy follow DEC-025/057/059/089/093; policy adoption does not complete the associated release acts.

- **AC-001** — The contract preserves the DEC-065 local command surface, schema-first application-service routing, unit-aware deterministic I/O, blocking diagnostics, result-envelope and reproducibility evidence, invented/permitted fixtures, local-first private-data behavior, current bounded implementation evidence, and the current payload-binding evidence and separately unresolved persisted-project, external-adapter, public-transport and release obligations under the ruled CI/release policies without compliance claims.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-10-05 Headless CLI and structured I/O analysis runner

> #### Procedure: DEL-10-05 Headless CLI and structured I/O analysis runner
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-017 — Purpose

> ##### Purpose
>
> Current deliverable obligation: implement a CLI or equivalent headless interface for schema-driven solve execution, benchmark automation, and report/result export before full GUI maturity. Implementation and record changes require an active bounded brief under `AGENTS.md`; `_STATUS.md` records lifecycle and remaining work. References below to the original setup write boundary apply only to that historical run. They do not exclude later authorized delivery, waive the retained requirements, or authorize issuance.
>
>
> Define the setup and future execution procedure for a governed headless runner deliverable without implementing the runner in this session.
>
### CLM-018 — Prerequisites

> ##### Prerequisites
>
> Current deliverable obligation: implement a CLI or equivalent headless interface for schema-driven solve execution, benchmark automation, and report/result export before full GUI maturity. Implementation and record changes require an active bounded brief under `AGENTS.md`; `_STATUS.md` records lifecycle and remaining work. References below to the original setup write boundary apply only to that historical run. They do not exclude later authorized delivery, waive the retained requirements, or authorize issuance.
>
>
> - Sealed context for DEL-10-05 with write scope limited to this deliverable folder.
> - Governing references listed in `_REFERENCES.md`.
> - Architecture-basis constraints injected in `_CONTEXT.md`.
> - No protected standards data, proprietary values, private project data, or certification/compliance claims in the artifacts.
> - Future implementation must receive a separate sealed brief before changing CLI/source files, fixtures, package manifests, CI files, or repo-level automation.
>
### CLM-019 — Steps

> ##### Steps
>
> 1. Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, decomposition and register rows for DEL-10-05, SOW-054, and SOW-032.
> 2. Draft the `ScopeOfWork.md` from accessible governance, decomposition, and register sources.
> 3. Keep exact CLI commands, schema fields, public API transport, CI provider, release matrix, and package/container details as `TBD` unless a cited source or human ruling resolves them.
> 4. Generate `_SEMANTIC.md` from the deliverable-local perspective and production documents.
> 5. Generate `_SEMANTIC_LENSING.md` from `_SEMANTIC.md` and the four production documents.
> 6. Apply warranted semantic-lensing items conservatively, with source reread evidence and no new implementation particulars.
> 7. Generate `Dependencies.csv` and `_DEPENDENCIES.md` from local source documents and the decomposition basis.
> 8. Validate the `ScopeOfWork.md`, dependency schema, semantic coverage, lens coverage, and status state.
> 9. Record run evidence under `_run_records/`.
>
### CLM-020 — Verification

> ##### Verification
>
> Setup is acceptable for `SEMANTIC_READY` only when:
>
> - `ScopeOfWork.md` exists.
> - `_SEMANTIC.md` is populated and has no failed final-cell semantic gate.
> - `_SEMANTIC_LENSING.md` includes all required lens coverage rows.
> - `Dependencies.csv` validates against v3.1 schema.
> - `_DEPENDENCIES.md` counts align with `Dependencies.csv`.
> - `_STATUS.md` records `SEMANTIC_READY` only, not `ISSUED`.
> - `git status` shows changes only inside this deliverable folder for this run.
>
### CLM-021 — Records

> ##### Records
>
> Required records:
>
> - Four production documents.
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_STATUS.md`
> - `_run_records/TASK_RUN_2026-04-30_1105_four-documents-p1-p2.md`
> - `_run_records/TASK_RUN_2026-04-30_1105_semantic-matrix-build.md`
> - `_run_records/TASK_RUN_2026-04-30_1105_lens-register.md`
> - `_run_records/TASK_RUN_2026-04-30_1105_four-documents-p3.md`
> - `_run_records/TASK_RUN_2026-04-30_1105_dependency-extract.md`
- **VER-001** — Validate the contract and review source parity, five stable verb families, service-boundary and unit-aware I/O behavior, blocking diagnostics and result envelopes, deterministic fixture evidence, retained downstream/deferred integrations, privacy/protected-content controls, and professional-authority limits.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-10-05 Headless CLI and structured I/O analysis runner

> #### Guidance: DEL-10-05 Headless CLI and structured I/O analysis runner
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-024 — Purpose

> ##### Purpose
>
> This deliverable keeps the early automation path coherent while GUI maturity, packaging details, and result export schemas are still evolving. The headless runner is useful only if it exercises the same governed service boundaries that GUI and downstream automation will rely on.
>

### CLM-025 — Principles

> ##### Principles
>
> - Treat the runner as an application-service client, not a solver shortcut.
> - Keep command/query/job separation visible so progress, cancellation, diagnostics, and result envelopes stay testable.
> - Preserve unit safety and deterministic serialization across input, solve, and output.
> - Prefer explicit `TBD` over invented command names, schema fields, CI provider decisions, or platform matrices.
> - Keep protected standards data, private project data, private rule packs, and proprietary component/material values out of public fixtures.
> - Use result statuses as software findings only; professional reliance still requires competent human review.
>

### CLM-026 — Considerations

> ##### Considerations
>
> The future runner should be useful for R0/R1 verification and regression work before a complete GUI exists. That usefulness depends on stable structured I/O, reproducible runs, and machine-readable diagnostics. It does not require this setup pass to pick final commands, package scripts, or file formats.
>
> The runner is also adjacent to several other deliverables. It must align with the command/query/job model, diagnostics/result envelopes, result export format, build/CI pipeline, unit system, persistence/hash strategy, and solver diagnostics. Those relationships are dependencies and constraints, not permission to edit those deliverables here.
>

### CLM-027 — Trade-offs

> ##### Trade-offs
>
> | Choice | Benefit | Risk | Setup posture |
> |---|---|---|---|
> | Early headless path before full GUI | Enables solver verification and automation earlier | Can bypass GUI warnings if service boundaries are ignored | Require command/job/result-envelope alignment |
> | Schema-first I/O | Supports tests, regression comparison, and downstream tooling Historical setup constraint only; current work follows the accepted deliverable scope and active bounded brief, with lifecycle/issuance separately governed. | Final fields can be over-specified too early | Keep exact schema fields TBD |
> | JSON/JCS-compatible reproducibility | Supports stable hashes and audit trails | Accepted container profile applies; actual storage integration and unmet controls require evidence | DEC-017 retains canonical JSON domain truth and a rebuildable SQLite projection; DEC-028 selects a multi-member zip/directory package with member inventory and per-member JCS hashes. DEC-057 names the `.opsproj` package. Storage roots, integration and unmet controls remain separately governed. |
> | CLI command surface | Familiar for CI and developer workflows | Command names may ossify before architecture is ready | DEC-065 governs the selected headless CLI syntax and process/exit policy. Use `docs/validation_manual/headless_runner_reproduction.md` and `core/runner/headless/` as the current implementation/reproduction references; do not infer a new public API or release claim. |
>

### CLM-028 — Examples

> ##### Examples
>
> Current deliverable obligation: implement a CLI or equivalent headless interface for schema-driven solve execution, benchmark automation, and report/result export before full GUI maturity. Implementation and record changes require an active bounded brief under `AGENTS.md`; `_STATUS.md` records lifecycle and remaining work. References below to the original setup write boundary apply only to that historical run. They do not exclude later authorized delivery, waive the retained requirements, or authorize issuance.
>
>
> No executable CLI examples are provided in this setup run. Future examples must use invented/public-permissive data only and include visible diagnostics for missing data, provenance warnings, and professional-boundary notices where applicable.
>
### CLM-029 — Pass 3 Semantic Lensing Notes

> ##### Pass 3 Semantic Lensing Notes
>
> The semantic lensing register reinforced three points for downstream work:
>
> - DEC-065 supplies the command policy; current field names and payload bindings are recorded in `schemas/headless_runner.schema.yaml` and the CLI wrapper. Preserve explicitly unsupported inputs rather than a blanket field-name TBD.
> - Future implementation must include a verification hook showing that no CLI/source, fixture, manifest, or repo-level automation file is modified outside its sealed write scope.
> - Result export compatibility must be tested against the schema-first result-envelope baseline instead of an ad hoc output shape.
>

### CLM-030 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No conflicting source statements identified during setup. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-054 SOW-032 OBJ-008 OBJ-009 OBJ-012 | CLM-007 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
