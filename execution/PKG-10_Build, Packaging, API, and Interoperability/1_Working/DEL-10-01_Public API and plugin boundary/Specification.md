---
doc_id: DEL-10-01-SPECIFICATION
doc_kind: deliverable.specification
status: draft
created: 2026-04-30
deliverable_id: DEL-10-01
package_id: PKG-10
---

# Specification: Public API and Plugin Boundary

## Scope

DEL-10-01 defines the public API and plugin boundary for OpenPipeStress interoperability. It covers contract obligations for model import/export, solver invocation, result extraction, rule-pack hooks, plugin manifest concepts, diagnostics, provenance, units, privacy, and report controls. SourcePath: `_CONTEXT.md`; SectionRef: Description and Architecture Basis Injection.

This deliverable does not choose a final external transport, implement plugin runtime behavior, edit repository-level API files, implement import/export adapters, select concrete external formats, implement a headless runner, or authorize bypass of the domain core, unit system, provenance checks, rule sandbox, diagnostics, report controls, or public/private data boundary. SourcePath: `_CONTEXT.md`; SectionRef: Acceptance/risk notes and write scope; SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07 and OI-004.

## Requirements

| ID | Requirement | Evidence |
|---|---|---|
| DEL-10-01-REQ-01 | The public API boundary shall be schema-first and shall use command, query, job, diagnostics, and result-envelope concepts rather than raw success/failure responses for nontrivial operations. | SourcePath: `_CONTEXT.md`; SectionRef: Resolved Baseline. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-03 and 8.2. |
| DEL-10-01-REQ-02 | The contract shall classify exposed operation families for model creation/import/export, load-case definition, solve execution, result extraction, rule-pack evaluation, report generation, and validation-test execution. | SourcePath: `docs/PRD.md`; SectionRef: 19.3 Public API. |
| DEL-10-01-REQ-03 | Mutating API and plugin operations shall route through governed application-service commands or equivalent service boundaries that preserve schema validation, unit checks, provenance checks, diagnostics/result envelopes, report controls, and audit/reproducibility metadata. | SourcePath: `docs/SPEC.md`; SectionRef: 1. Architectural overview. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-02, AB-00-03, AB-00-07. |
| DEL-10-01-REQ-04 | Long-running solve, report, export, and validation operations shall be represented as jobs with progress, cancellation, reproducibility metadata, and final result envelopes. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-03. SourcePath: `docs/PRD.md`; SectionRef: 20 Performance Requirements. |
| DEL-10-01-REQ-05 | Model import/export boundaries shall validate units, dimensional consistency, source/provenance, redistribution/license status, public/private classification, diagnostics, and protected-content risk before data enters core workflows or public artifacts. | SourcePath: `docs/PRD.md`; SectionRef: 13.5 Data Import Warnings. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 2-5. |
| DEL-10-01-REQ-06 | The API and plugin boundary shall prohibit public repository content that embeds protected standards text, protected tables, protected examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data without rights, or private project/rule data. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1, OPS-K-IP-3. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 3. |
| DEL-10-01-REQ-07 | Public data contribution paths exposed through import/export or plugin APIs shall require source, source location, redistribution/license status, contributor certification, and review disposition; unknowns remain `TBD` or block public contribution. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-2 and OPS-K-AGENT-1. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 4. |
| DEL-10-01-REQ-08 | Rule-pack-facing API/plugin hooks shall preserve sandboxed, deterministic, unit-aware evaluation and shall not permit arbitrary code execution or bypass of required-input completeness checks. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-RULE-2. SourcePath: `docs/SPEC.md`; SectionRef: 6. Rule-pack evaluator. |
| DEL-10-01-REQ-09 | Rule-pack identities, versions, checksums, source notices, redistribution status, and private/public markings shall be carried through API envelopes where rule-pack data or results are referenced. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-RULE-3 and OPS-K-DATA-3. SourcePath: `docs/SPEC.md`; SectionRef: 6 and 8. |
| DEL-10-01-REQ-10 | Result envelopes shall distinguish invalid input, incomplete model, mechanics solved, rule-inputs incomplete, user-rule checked/failed, human-review-required, and human-approved-for-project states without introducing automatic `CODE_COMPLIANT` status. | SourcePath: `docs/TYPES.md`; SectionRef: 4. Analysis-status vocabulary. |
| DEL-10-01-REQ-11 | API, plugin, adapter, and report-facing outputs shall not claim to certify, seal, approve, authenticate, endorse, or declare engineering code compliance for professional reliance. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AUTH-1. SourcePath: `docs/DIRECTIVE.md`; SectionRef: 4.2 Out of scope. |
| DEL-10-01-REQ-12 | The plugin boundary shall be deny-by-default: no plugin capability, private-data access, filesystem/network/process access, report/export control bypass, or rule-evaluator integration is granted unless a later approved permission/sandbox design authorizes it. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07 and OI-006. SourcePath: `docs/PRD.md`; SectionRef: 18.2 Telemetry and 18.3 Private Data Protection. |
| DEL-10-01-REQ-13 | Public API transport, concrete endpoint syntax, concrete plugin runtime, exact permission taxonomy, exact external import/export formats, exact code-generation tooling, and repository-level `api/openapi.yaml` placement remain TBD until human/architecture approval. | SourcePath: `_CONTEXT.md`; SectionRef: Still TBD and write scope. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: OI-004 and 8.2. |
| DEL-10-01-REQ-14 | JSON payloads that are hashed for API manifests, jobs, model snapshots, results, or reproducibility records shall use the accepted canonical JSON/JCS-compatible hash basis where applicable; non-JSON/binary assets require manifest hashes. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-04 and 8.2. |
| DEL-10-01-REQ-15 | Verification for this boundary shall include documentation review plus later schema, unit, provenance, diagnostics, protected-content, privacy, rule-sandbox, report-boundary, and adapter/plugin regression gates when concrete implementation artifacts exist. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-08. SourcePath: `docs/VALIDATION_STRATEGY.md`; SectionRef: 2 and 4. |

## Standards

No protected engineering code, standards clauses, tables, formulas, examples, material allowables, SIF/flexibility content, protected dimensional data, or proprietary vendor data are incorporated into this deliverable.

Applicable internal baselines:

- `docs/CONTRACT.md` invariant catalog, especially OPS-K-IP-*, OPS-K-DATA-*, OPS-K-UNIT-1, OPS-K-RULE-*, OPS-K-PRIV-*, OPS-K-AUTH-1, and OPS-K-AGENT-*.
- `docs/TYPES.md` for API contract deliverable type, analysis-status vocabulary, epistemic labels, provenance labels, and canonical domain object registry.
- `docs/SPEC.md` for layer responsibilities, adapter no-bypass rule, rule-pack evaluator constraints, diagnostics classes, reporting/audit content, and Type 2 acceptance semantics.
- `docs/IP_AND_DATA_BOUNDARY.md` for public/private data, provenance, quarantine, private user data, and report boundary policy.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 for SOW-030, OBJ-009, AB-00-02/03/04/06/07/08, and OI-004.

External implementation standard baseline:

- JSON Schema 2020-12 is the accepted public schema/interchange baseline by SCA-001. This deliverable does not reproduce the JSON Schema specification text and does not choose concrete schema file layout.

## Verification

| Requirement IDs | Verification approach |
|---|---|
| REQ-01, REQ-03, REQ-04 | Architecture/API review confirms commands, queries, jobs, progress/cancellation, diagnostics, result envelopes, and governed service boundaries are represented without selecting final transport. |
| REQ-02 | Public API capability-family review confirms the PRD section 19.3 families are represented and that non-DEL-10-01 implementation work remains deferred. |
| REQ-05, REQ-06, REQ-07 | Protected-content/provenance review confirms import/export and plugin contribution paths require units, source, redistribution status, private/public classification, and quarantine behavior. |
| REQ-08, REQ-09, REQ-12 | Rule-pack/security review confirms no hook bypasses sandboxing, unit awareness, checksum/version/source metadata, private markings, or required-input blocking. |
| REQ-10, REQ-11 | Status/professional-boundary review confirms no automatic code-compliance state or certification/approval claim appears in API, plugin, result, or report outputs. |
| REQ-13 | TBD review confirms transport, endpoint syntax, plugin runtime, permission taxonomy, format list, code generation, and repository-level API artifact placement are not silently resolved. |
| REQ-14 | Reproducibility review confirms canonical JSON/JCS-compatible hash basis is referenced where hashes are required and non-JSON assets remain manifest-hash governed. |
| REQ-15 | Layered gate review confirms later concrete implementation will need schema, unit, provenance, diagnostics, protected-content, privacy, sandbox, report, and regression evidence. |

### Interim Setup Acceptance Criteria

| Gate | Pass condition |
|---|---|
| Scope gate | Only DEL-10-01 deliverable-local files are edited. |
| Document gate | Datasheet, Specification, Guidance, and Procedure exist and include default sections. |
| Semantic gate | `_SEMANTIC.md` exists, contains matrices A, B, C, F, D, K, G, X, T, E, and passes the local result-cell audit. |
| Lensing gate | `_SEMANTIC_LENSING.md` exists, covers every cell in matrices A, B, C, F, D, X, E, and records warranted items without resolving human decisions. |
| Dependency gate | `Dependencies.csv` exists, validates against v3.1 required columns, uses canonical write-form enums, and `_DEPENDENCIES.md` summarizes the active rows. |
| Boundary gate | No transport, plugin runtime, repository-level `api/openapi.yaml`, source code, package manifests, protected data, private engineering data, or compliance/certification claims are introduced. |

## Documentation

Required setup artifacts for DEL-10-01:

- Deliverable-local API/plugin boundary document kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`.
- Semantic setup artifacts: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and `_run_records/*`.
- Dependency artifacts: `Dependencies.csv` and `_DEPENDENCIES.md`.

Plugin/public API manifest concept inventory for later schema work:

| Concept slot | Current status | Evidence |
|---|---|---|
| API envelope version | Required concept; exact field name/layout TBD. | `_CONTEXT.md` Resolved Baseline; `docs/_Decomposition/SOFTWARE_DECOMP.md` 8.2. |
| Operation family | Required concept; allowable families listed by contract, exact endpoint names TBD. | `docs/PRD.md` 19.3. |
| Command/query/job classification | Required concept; exact schema layout TBD. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03. |
| Diagnostics envelope | Required concept; exact field layout TBD. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06. |
| Unit/provenance/data-boundary declarations | Required concepts; exact fields TBD. | `docs/CONTRACT.md` OPS-K-UNIT-1, OPS-K-DATA-3, OPS-K-IP-2. |
| Rule-pack reference/checksum/source notice | Required where rule-pack data or results are referenced; exact layout TBD. | `docs/SPEC.md` section 6; `docs/CONTRACT.md` OPS-K-RULE-3. |
| Plugin identity/version/capability request | Candidate concept; exact field names, permission taxonomy, and approval path TBD. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-07 and OI-004. |
| Privacy/telemetry posture | Required deny-by-default declaration for private data exposure; exact policy hook TBD. | `docs/PRD.md` 18.2 and 18.3. |
