---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-10-01
package_id: PKG-10
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@4d153302c3c4cd42578936db160c2bac1270225a
project_scope_refs: [SOW-030]
package_objective_refs: [OBJ-009]
---

# Scope of Work — DEL-10-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-10-01` in service of project scope [SOW-030] and package objectives [OBJ-009].

- **OUT-001** — A public API and plugin-boundary contract for governed model exchange, solver jobs, results, rule-pack hooks, diagnostics, provenance, units, privacy, and report controls is produced.

## Deliverable Definition — Ontology

### CLM-001 — Source preamble

> ---
> doc_id: DEL-10-01-DATASHEET
> doc_kind: deliverable.datasheet
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-10-01
> package_id: PKG-10
> ---
>

### CLM-002 — Datasheet: Public API and Plugin Boundary

> #### Datasheet: Public API and Plugin Boundary
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-003 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-10-01-DECL-002`.
>

### CLM-004 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-10-01 |
> | Package ID | PKG-10 |
> | Package | Build, Packaging, API, and Interoperability |
> | Type | API_CONTRACT |
> | Scope item | SOW-030 |
> | Objective | OBJ-009 |
> | Anticipated artifacts | `api/openapi.yaml` or equivalent; plugin boundary doc |
> | Current artifact form | Deliverable-local contract kit; repository-level `api/openapi.yaml` is outside this write scope |
> | Lifecycle target for setup | `SEMANTIC_READY` after the setup sequence passes |
> | Transport decision | TBD; no final external transport selected here |
>

### CLM-005 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Boundary purpose | Define public API/plugin boundaries for model import/export, solver invocation, results, and rule-pack hooks. | `_CONTEXT.md` Description; `docs/_Registers/Deliverables.csv` row DEL-10-01 |
> | Public API capability families | Model creation/import/export, load-case definition, solve execution, result extraction, rule-pack evaluation, report generation, and validation-test execution. | `docs/PRD.md` section 19.3 |
> | Contract baseline | Schema-first command/query/job/result envelopes; JSON Schema 2020-12 public schema/interchange basis. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
> | Adapter/plugin baseline | Deny-bypass boundary: adapters/plugins may translate or extend, but cannot bypass units, provenance, diagnostics, rule sandboxing, public/private data controls, or report controls. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02, AB-00-07; `docs/SPEC.md` section 1 |
> | Professional boundary | API results can expose mechanics status and user-rule-check status, but not software-declared code compliance or professional approval. | `docs/TYPES.md` sections 4 and 6; `docs/CONTRACT.md` OPS-K-AUTH-1 |
> | Protected-data boundary | Public API/plugin contracts must not bundle protected standards text, tables, code-derived formulas, allowables, SIF/flexibility tables, proprietary catalogs, or private project/rule data. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-3 |
>

### CLM-006 — Conditions

> ##### Conditions
>
> | Condition | Required handling |
> |---|---|
> | Missing units on dimensional data | Reject, block, or emit a diagnostic; do not silently default. |
> | Missing provenance or redistribution status | Emit provenance/data-boundary diagnostics and block public contribution paths when needed. |
> | Protected or proprietary data suspected | Stop ingestion, quarantine/escalate, and do not import into public artifacts. |
> | Rule-pack-facing hook | Preserve sandboxed, deterministic, unit-aware evaluation; do not expose arbitrary code execution. |
> | Result publication/export | Use result/diagnostic envelopes and preserve report controls and professional-responsibility notices. |
> | Private data exposure | Local-first by default; no telemetry or external transmission of private project/rule/component/material data by default. |
>

### CLM-007 — Construction

> ##### Construction
>
> This deliverable records a contract boundary, not an implementation. The boundary is organized around these contract surfaces:
>
> | Surface | Contract role | Setup status |
> |---|---|---|
> | Public API envelope | Common schema-first wrapper for commands, queries, jobs, diagnostics, provenance, and result metadata. | Baseline defined; concrete schema file layout TBD. |
> | Model import/export boundary | Allows external data exchange only through schema, unit, provenance, redistribution, and private/public data checks. | Boundary defined; external formats TBD. |
> | Solver invocation boundary | Routes solve requests through application-service command/job semantics with diagnostics, progress/cancellation, and reproducibility metadata. | Boundary defined; concrete transport TBD. |
> | Results boundary | Exposes mechanical results, diagnostics, warnings, hashes, and rule-pack status without compliance or approval claims. | Boundary defined; result export details shared with DEL-08-04/DEL-10-05. |
> | Rule-pack hook boundary | Allows governed rule-pack evaluation hooks while preserving sandboxing, unit safety, checksums, source notes, and private/public markings. | Boundary defined; expression grammar/library TBD. |
> | Plugin manifest boundary | Candidate manifest concept slots include identity, version, declared extension point, schema/envelope compatibility, capability request, data-boundary declaration, diagnostics compatibility, and review status. | Concept inventory only; exact field names and permission taxonomy TBD. |
>

### CLM-008 — References

> ##### References
>
> - `_CONTEXT.md` for sealed deliverable identity, scope, objective, write scope, and SCA-001 architecture-basis injection.
> - `docs/CONTRACT.md` for applicable invariants: OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-RULE-1/2/3, OPS-K-PRIV-1/2, OPS-K-AUTH-1, and OPS-K-AGENT-1..4.
> - `docs/SPEC.md` sections 1, 6, 7, 8, 10, and 11 for layer responsibilities, adapter no-bypass language, rule-pack sandboxing, diagnostics/reporting, and deliverable acceptance semantics.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for SOW-030, OBJ-009, AB-00-02/03/04/06/07/08, and OI-004.
> - `docs/PRD.md` sections 13.4, 13.5, 18.2, 18.3, and 19.3 for public/private data, import warnings, telemetry/private-data protection, and public API capability families.

## Completion and Reliance Basis — Epistemology

### CLM-009 — Source preamble

> ---
> doc_id: DEL-10-01-SPECIFICATION
> doc_kind: deliverable.specification
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-10-01
> package_id: PKG-10
> ---
>

### CLM-010 — Specification: Public API and Plugin Boundary

> #### Specification: Public API and Plugin Boundary
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-011 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-10-01-DECL-001`.
>

### CLM-012 — Scope

> ##### Scope
>
> DEL-10-01 defines the public API and plugin boundary for OpenPipeStress interoperability. It covers contract obligations for model import/export, solver invocation, result extraction, rule-pack hooks, plugin manifest concepts, diagnostics, provenance, units, privacy, and report controls. SourcePath: `_CONTEXT.md`; SectionRef: Description and Architecture Basis Injection.
>
> This deliverable does not choose a final external transport, implement plugin runtime behavior, edit repository-level API files, implement import/export adapters, select concrete external formats, implement a headless runner, or authorize bypass of the domain core, unit system, provenance checks, rule sandbox, diagnostics, report controls, or public/private data boundary. SourcePath: `_CONTEXT.md`; SectionRef: Acceptance/risk notes and write scope; SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07 and OI-004.
>

### CLM-013 — Requirements

> ##### Requirements
>
> | ID | Requirement | Evidence |
> |---|---|---|
> | DEL-10-01-REQ-01 | The public API boundary shall be schema-first and shall use command, query, job, diagnostics, and result-envelope concepts rather than raw success/failure responses for nontrivial operations. | SourcePath: `_CONTEXT.md`; SectionRef: Resolved Baseline. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-03 and 8.2. |
> | DEL-10-01-REQ-02 | The contract shall classify exposed operation families for model creation/import/export, load-case definition, solve execution, result extraction, rule-pack evaluation, report generation, and validation-test execution. | SourcePath: `docs/PRD.md`; SectionRef: 19.3 Public API. |
> | DEL-10-01-REQ-03 | Mutating API and plugin operations shall route through governed application-service commands or equivalent service boundaries that preserve schema validation, unit checks, provenance checks, diagnostics/result envelopes, report controls, and audit/reproducibility metadata. | SourcePath: `docs/SPEC.md`; SectionRef: 1. Architectural overview. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-02, AB-00-03, AB-00-07. |
> | DEL-10-01-REQ-04 | Long-running solve, report, export, and validation operations shall be represented as jobs with progress, cancellation, reproducibility metadata, and final result envelopes. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-03. SourcePath: `docs/PRD.md`; SectionRef: 20 Performance Requirements. |
> | DEL-10-01-REQ-05 | Model import/export boundaries shall validate units, dimensional consistency, source/provenance, redistribution/license status, public/private classification, diagnostics, and protected-content risk before data enters core workflows or public artifacts. | SourcePath: `docs/PRD.md`; SectionRef: 13.5 Data Import Warnings. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 2-5. |
> | DEL-10-01-REQ-06 | The API and plugin boundary shall prohibit public repository content that embeds protected standards text, protected tables, protected examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data without rights, or private project/rule data. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1, OPS-K-IP-3. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 3. |
> | DEL-10-01-REQ-07 | Public data contribution paths exposed through import/export or plugin APIs shall require source, source location, redistribution/license status, contributor certification, and review disposition; unknowns remain `TBD` or block public contribution. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-2 and OPS-K-AGENT-1. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 4. |
> | DEL-10-01-REQ-08 | Rule-pack-facing API/plugin hooks shall preserve sandboxed, deterministic, unit-aware evaluation and shall not permit arbitrary code execution or bypass of required-input completeness checks. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-RULE-2. SourcePath: `docs/SPEC.md`; SectionRef: 6. Rule-pack evaluator. |
> | DEL-10-01-REQ-09 | Rule-pack identities, versions, checksums, source notices, redistribution status, and private/public markings shall be carried through API envelopes where rule-pack data or results are referenced. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-RULE-3 and OPS-K-DATA-3. SourcePath: `docs/SPEC.md`; SectionRef: 6 and 8. |
> | DEL-10-01-REQ-10 | Result envelopes shall distinguish invalid input, incomplete model, mechanics solved, rule-inputs incomplete, user-rule checked/failed, human-review-required, and human-approved-for-project states without introducing automatic `CODE_COMPLIANT` status. | SourcePath: `docs/TYPES.md`; SectionRef: 4. Analysis-status vocabulary. |
> | DEL-10-01-REQ-11 | API, plugin, adapter, and report-facing outputs shall not claim to certify, seal, approve, authenticate, endorse, or declare engineering code compliance for professional reliance. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AUTH-1. SourcePath: `docs/DIRECTIVE.md`; SectionRef: 4.2 Out of scope. |
> | DEL-10-01-REQ-12 | The plugin boundary shall be deny-by-default: no plugin capability, private-data access, filesystem/network/process access, report/export control bypass, or rule-evaluator integration is granted unless a later approved permission/sandbox design authorizes it. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07 and OI-006. SourcePath: `docs/PRD.md`; SectionRef: 18.2 Telemetry and 18.3 Private Data Protection. |
> | DEL-10-01-REQ-13 | Public API transport, concrete endpoint syntax, concrete plugin runtime, exact permission taxonomy, exact external import/export formats, exact code-generation tooling, and repository-level `api/openapi.yaml` placement remain TBD until human/architecture approval. | SourcePath: `_CONTEXT.md`; SectionRef: Still TBD and write scope. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: OI-004 and 8.2. |
> | DEL-10-01-REQ-14 | JSON payloads that are hashed for API manifests, jobs, model snapshots, results, or reproducibility records shall use the accepted canonical JSON/JCS-compatible hash basis where applicable; non-JSON/binary assets require manifest hashes. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-04 and 8.2. |
> | DEL-10-01-REQ-15 | Verification for this boundary shall include documentation review plus later schema, unit, provenance, diagnostics, protected-content, privacy, rule-sandbox, report-boundary, and adapter/plugin regression gates when concrete implementation artifacts exist. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-08. SourcePath: `docs/VALIDATION_STRATEGY.md`; SectionRef: 2 and 4. |
>

### CLM-014 — Standards

> ##### Standards
>
> No protected engineering code, standards clauses, tables, formulas, examples, material allowables, SIF/flexibility content, protected dimensional data, or proprietary vendor data are incorporated into this deliverable.
>
> Applicable internal baselines:
>
> - `docs/CONTRACT.md` invariant catalog, especially OPS-K-IP-*, OPS-K-DATA-*, OPS-K-UNIT-1, OPS-K-RULE-*, OPS-K-PRIV-*, OPS-K-AUTH-1, and OPS-K-AGENT-*.
> - `docs/TYPES.md` for API contract deliverable type, analysis-status vocabulary, epistemic labels, provenance labels, and canonical domain object registry.
> - `docs/SPEC.md` for layer responsibilities, adapter no-bypass rule, rule-pack evaluator constraints, diagnostics classes, reporting/audit content, and Type 2 acceptance semantics.
> - `docs/IP_AND_DATA_BOUNDARY.md` for public/private data, provenance, quarantine, private user data, and report boundary policy.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for SOW-030, OBJ-009, AB-00-02/03/04/06/07/08, and OI-004.
>
> External implementation standard baseline:
>
> - JSON Schema 2020-12 is the accepted public schema/interchange baseline by SCA-001. This deliverable does not reproduce the JSON Schema specification text and does not choose concrete schema file layout.
>

### CLM-015 — Verification

> ##### Verification
>
> | Requirement IDs | Verification approach |
> |---|---|
> | REQ-01, REQ-03, REQ-04 | Architecture/API review confirms commands, queries, jobs, progress/cancellation, diagnostics, result envelopes, and governed service boundaries are represented without selecting final transport. |
> | REQ-02 | Public API capability-family review confirms the PRD section 19.3 families are represented and that non-DEL-10-01 implementation work remains deferred. |
> | REQ-05, REQ-06, REQ-07 | Protected-content/provenance review confirms import/export and plugin contribution paths require units, source, redistribution status, private/public classification, and quarantine behavior. |
> | REQ-08, REQ-09, REQ-12 | Rule-pack/security review confirms no hook bypasses sandboxing, unit awareness, checksum/version/source metadata, private markings, or required-input blocking. |
> | REQ-10, REQ-11 | Status/professional-boundary review confirms no automatic code-compliance state or certification/approval claim appears in API, plugin, result, or report outputs. |
> | REQ-13 | TBD review confirms transport, endpoint syntax, plugin runtime, permission taxonomy, format list, code generation, and repository-level API artifact placement are not silently resolved. |
> | REQ-14 | Reproducibility review confirms canonical JSON/JCS-compatible hash basis is referenced where hashes are required and non-JSON assets remain manifest-hash governed. |
> | REQ-15 | Layered gate review confirms later concrete implementation will need schema, unit, provenance, diagnostics, protected-content, privacy, sandbox, report, and regression evidence. |
>

### CLM-016 — Interim Setup Acceptance Criteria

> ###### Interim Setup Acceptance Criteria
>
> | Gate | Pass condition |
> |---|---|
> | Scope gate | Only DEL-10-01 deliverable-local files are edited. |
> | Document gate | Datasheet, Specification, Guidance, and Procedure exist and include default sections. |
> | Semantic gate | `_SEMANTIC.md` exists, contains matrices A, B, C, F, D, K, G, X, T, E, and passes the local result-cell audit. |
> | Lensing gate | `_SEMANTIC_LENSING.md` exists, covers every cell in matrices A, B, C, F, D, X, E, and records warranted items without resolving human decisions. |
> | Dependency gate | `Dependencies.csv` exists, validates against v3.1 required columns, uses canonical write-form enums, and `_DEPENDENCIES.md` summarizes the active rows. |
> | Boundary gate | No transport, plugin runtime, repository-level `api/openapi.yaml`, source code, package manifests, protected data, private engineering data, or compliance/certification claims are introduced. |
>

### CLM-017 — Documentation

> ##### Documentation
>
> Required setup artifacts for DEL-10-01:
>
> - Deliverable-local API/plugin boundary document kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`.
> - Semantic setup artifacts: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and `_run_records/*`.
> - Dependency artifacts: `Dependencies.csv` and `_DEPENDENCIES.md`.
>
> Plugin/public API manifest concept inventory for later schema work:
>
> | Concept slot | Current status | Evidence |
> |---|---|---|
> | API envelope version | Required concept; exact field name/layout TBD. | `_CONTEXT.md` Resolved Baseline; `execution/_Decomposition/SOFTWARE_DECOMP.md` 8.2. |
> | Operation family | Required concept; allowable families listed by contract, exact endpoint names TBD. | `docs/PRD.md` 19.3. |
> | Command/query/job classification | Required concept; exact schema layout TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03. |
> | Diagnostics envelope | Required concept; exact field layout TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06. |
> | Unit/provenance/data-boundary declarations | Required concepts; exact fields TBD. | `docs/CONTRACT.md` OPS-K-UNIT-1, OPS-K-DATA-3, OPS-K-IP-2. |
> | Rule-pack reference/checksum/source notice | Required where rule-pack data or results are referenced; exact layout TBD. | `docs/SPEC.md` section 6; `docs/CONTRACT.md` OPS-K-RULE-3. |
> | Plugin identity/version/capability request | Candidate concept; exact field names, permission taxonomy, and approval path TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-07 and OI-004. |
> | Privacy/telemetry posture | Required deny-by-default declaration for private data exposure; exact policy hook TBD. | `docs/PRD.md` 18.2 and 18.3. |

- **AC-001** — The contract preserves schema-first command/query/job/result envelopes, deny-bypass service boundaries, unit and provenance validation, protected/private-data controls, deterministic plugin declarations, current implemented-slice evidence, visible transport and permission-taxonomy TBDs, and the prohibition on compliance, certification, or professional-approval claims.

## Production and Verification Method — Praxeology

### CLM-018 — Source preamble

> ---
> doc_id: DEL-10-01-PROCEDURE
> doc_kind: deliverable.procedure
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-10-01
> package_id: PKG-10
> ---
>

### CLM-019 — Procedure: Public API and Plugin Boundary

> #### Procedure: Public API and Plugin Boundary
>

### CLM-020 — Purpose

> ##### Purpose
>
> Use this procedure to produce, review, and later refine the DEL-10-01 public API/plugin boundary without crossing into product implementation, external transport selection, plugin runtime implementation, repository-level API files, source code, package manifests, or protected/private data.
>

### CLM-021 — Prerequisites

> ##### Prerequisites
>
> - The sealed DEL-10-01 context is available in `_CONTEXT.md`.
> - Governing documents and registers named in `_REFERENCES.md` have been read.
> - Applicable architecture basis IDs AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08 are treated as dispatch constraints, not copied as full PKG-00 authority.
> - Current setup write scope is restricted to this deliverable folder.
> - Public transport, plugin runtime, external format list, rule expression grammar/library, code-generation tooling, and repository-level `api/openapi.yaml` placement remain TBD.
>

### CLM-022 — Steps

> ##### Steps
>
> 1. Confirm identity and scope:
>    - Verify the deliverable is `DEL-10-01` under `PKG-10`.
>    - Confirm scope item SOW-030 and objective OBJ-009.
>    - Confirm no edit target is outside this deliverable folder.
> 2. Inventory public API operation families:
>    - Model creation/import/export.
>    - Load-case definition.
>    - Solve execution.
>    - Result extraction.
>    - Rule-pack evaluation.
>    - Report generation.
>    - Validation-test execution.
> 3. Classify operation semantics:
>    - Mutating operations become commands.
>    - Read-only operations become queries.
>    - Long-running solve/report/export/validation operations become jobs with progress, cancellation, reproducibility metadata, and final result envelopes.
> 4. Define mandatory envelope slots at concept level:
>    - Schema/envelope version.
>    - Operation family and command/query/job classification.
>    - Units and dimensional metadata where applicable.
>    - Provenance and redistribution/private-public metadata where applicable.
>    - Diagnostics with code/class/severity/source/affected object/message/remediation/provenance where applicable.
>    - Result status using project analysis-status vocabulary.
>    - Reproducibility metadata such as payload hash, model hash, solver version, rule-pack checksum, and input manifest references where applicable.
> 5. Define no-bypass rules:
>    - Adapters/plugins cannot bypass schema validation, unit checks, provenance checks, public/private data controls, diagnostics, rule sandboxing, result envelopes, report controls, or protected-content gates.
>    - Direct writes into domain core, solver state, storage internals, private libraries, or reports are not authorized by this boundary.
> 6. Define plugin manifest concepts without approving concrete fields:
>    - Plugin identity/version.
>    - Declared extension point.
>    - Compatible schema/envelope version.
>    - Requested capabilities.
>    - Private-data access posture.
>    - Diagnostics compatibility.
>    - Review/status metadata.
>    - Rule-pack hook posture where applicable.
> 7. Preserve unresolved decisions:
>    - Mark public transport, endpoint syntax, plugin runtime, permission taxonomy, import/export formats, code generation, and repository-level API file layout as `TBD`.
>    - Do not choose defaults without human/project authority evidence.
> 8. Check protected-data and authority boundaries:
>    - Confirm no protected standards text, tables, copied formulas, proprietary vendor data, private project data, or private rule-pack content is embedded.
>    - Confirm no wording claims certification, sealing, approval, endorsement, or code compliance.
> 9. Produce setup artifacts:
>    - Four-document kit.
>    - `_SEMANTIC.md`.
>    - `_SEMANTIC_LENSING.md`.
>    - `Dependencies.csv`.
>    - `_DEPENDENCIES.md`.
>    - `_run_records/*`.
>    - `_STATUS.md` with `SEMANTIC_READY` only after setup gates pass.
>

### CLM-023 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Write-scope check | Only files in the DEL-10-01 folder changed. |
> | Four-document check | Datasheet, Specification, Guidance, and Procedure exist with default sections. |
> | Boundary check | No transport, plugin runtime, source code, package manifest, repository-level API file, or external format implementation is introduced. |
> | Protected-data check | No protected standards content, proprietary commercial data, or private project/rule/component/material data is introduced. |
> | Professional-boundary check | No certification, sealing, approval, endorsement, or code-compliance claim appears. |
> | Semantic check | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and preserve lens-not-authority separation. |
> | Dependency check | `Dependencies.csv` validates against v3.1 schema and canonical enum values. |
>

### CLM-024 — Records

> ##### Records
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/TASK_RUN_*.md`
> - `_STATUS.md`
>

### CLM-025 — Completion Condition

> ##### Completion Condition
>
> The setup sequence is complete when the required setup artifacts exist, dependency validation passes, semantic/lensing artifacts are internally consistent, unresolved decisions remain visible as `TBD`, protected-data/professional-boundary checks are clean, and `_STATUS.md` records `SEMANTIC_READY` without any `ISSUED` transition.

- **VER-001** — Validate the contract and review source parity, operation-family and plugin-boundary coverage, current-versus-historical declarations, units/provenance/privacy and deny-bypass controls, diagnostic/result-envelope requirements, retained transport and permission TBDs, and professional-authority limits.

## Governing Values and Decisions — Axiology

### CLM-026 — Source preamble

> ---
> doc_id: DEL-10-01-GUIDANCE
> doc_kind: deliverable.guidance
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-10-01
> package_id: PKG-10
> ---
>

### CLM-027 — Guidance: Public API and Plugin Boundary

> #### Guidance: Public API and Plugin Boundary
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-028 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-10-01-DECL-003`.
>

### CLM-029 — Purpose

> ##### Purpose
>
> This deliverable gives later API, plugin, adapter, headless runner, result export, report, and security work a common boundary contract. It exists to keep interoperability extensible without creating a shortcut around units, provenance, diagnostics, public/private data controls, rule-pack sandboxing, report controls, or professional-responsibility limits.
>

### CLM-030 — Principles

> ##### Principles
>
> - Treat the public API as a governed service boundary, not a direct tunnel into solver internals, storage internals, private libraries, rule-pack evaluator internals, or report-generation controls.
> - Keep transport binding separate from envelope semantics. The envelope can be specified now; HTTP, IPC, CLI JSON, in-process library calls, or other transport choices remain TBD until approved.
> - Prefer schema-first contracts and explicit `TBD` placeholders over invented endpoint syntax or plugin manifest fields.
> - Let adapters translate external data, but require imported data to pass schema, unit, provenance, redistribution, diagnostics, and data-boundary checks before core use.
> - Treat rule-pack hooks as security-sensitive. The hook may expose governed evaluation capabilities later, but it must never authorize arbitrary code execution by implication.
> - Preserve the three-state authority boundary: mechanics solved, user-rule checked, and professional approval are distinct.
>

### CLM-031 — Considerations

> ##### Considerations
>
> The API/plugin boundary has two different audiences:
>
> | Audience | What it needs | Boundary implication |
> |---|---|---|
> | Application services and GUI/headless clients | Stable command/query/job/result envelopes | Do not expose direct mutable domain-core internals. |
> | Adapter and plugin authors | Extension points and failure modes | Deny by default; require manifest, capability, diagnostics, provenance, and review status concepts. |
> | Project reviewers | Evidence that extension cannot bypass governance | Trace every capability to unit, provenance, rule, privacy, report, and professional-boundary controls. |
> | Future implementers | Concrete enough contract to build against | Carry unresolved transport/runtime/schema-layout decisions as visible `TBD`, not hidden assumptions. |
>
> Public API and plugin docs should avoid marketing language such as "certified", "approved", "code compliant", or "professionally accepted". They should describe computed states and human-review obligations using the project vocabulary in `docs/TYPES.md`.
>

### CLM-032 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | More public surface vs. safer core boundary | Start with fewer governed operation families and expand only when validation and privacy controls are defined. |
> | Early OpenAPI file vs. schema-first envelope clarity | Because repository-level `api/openapi.yaml` is outside this write scope and public transport is TBD, keep the current artifact as an equivalent deliverable-local contract. |
> | Plugin flexibility vs. no-bypass controls | Flexibility is acceptable only after capability declarations, sandbox behavior, private-data restrictions, and diagnostics are explicit. |
> | External format support vs. protected data risk | External formats remain TBD until each adapter can enforce provenance, redistribution, protected-content, unit, and diagnostics gates. |
> | Rule-pack integration vs. arbitrary execution risk | Rule hooks should expose declarative, sandboxed, unit-aware evaluation semantics only; exact grammar/library remains TBD. |
>

### CLM-033 — Examples

> ##### Examples
>
> No engineering numeric example, protected standards example, proprietary vendor example, or copied commercial-software example is included here.
>
> Acceptable invented boundary examples for later work are conceptual only. They are not approved schema fields, endpoint definitions, transport bindings, or plugin permissions:
>
> - A model-import command envelope that contains schema version, unit system, provenance summary, private/public classification, payload hash, and diagnostic output slots.
> - A solve-job request envelope that records model hash, solver version request, cancellation token concept, progress diagnostic stream concept, and final mechanics-result envelope concept.
> - A rule-pack-evaluation hook that references rule-pack ID/version/checksum/source notice and returns user-rule status plus missing-input diagnostics without claiming compliance.
>

### CLM-034 — Human-Ruling Queue

> ##### Human-Ruling Queue
>
> | Topic | Current disposition |
> |---|---|
> | Final public API transport | TBD |
> | Repository-level `api/openapi.yaml` or equivalent schema file layout | TBD; outside this setup write scope |
> | Concrete plugin runtime and loading mechanism | TBD |
> | Permission taxonomy and capability names | TBD |
> | Exact import/export format list and priorities | TBD |
> | Rule expression grammar/library | TBD; PKG-06/security decision path |
> | CI/provider thresholds for API/plugin validation gates | TBD |
>

### CLM-035 — Semantic Lensing Hold Points

> ##### Semantic Lensing Hold Points
>
> The setup lensing pass identified these review hold-points for later work:
>
> - Future repository-level API artifacts need an approved path and schema/transport ruling before endpoint-level details are treated as implementable.
> - Plugin capability grants need an approval owner, permission taxonomy, and sandbox/privacy review before any non-default capability is available.
> - Conceptual examples remain examples only; exact fields, message shapes, and capability names stay TBD.
> - Any exception to private-data deny-by-default behavior requires PKG-12/security rationale and explicit human approval.
>

### CLM-036 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict detected during setup. Remaining issues are explicit TBDs rather than contradictory sources. | NA | NA | NA | Keep TBDs visible until human/project authority records decisions. | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 OBJ-009 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
