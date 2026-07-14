---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-08-03
package_id: PKG-08
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-024]
package_objective_refs: [OBJ-007, OBJ-011]
---

# Scope of Work — DEL-08-03

## Purpose and Objective Traceability

This Scope of Work defines `DEL-08-03` in service of project scope [SOW-024] and package objectives [OBJ-007, OBJ-011].

- **OUT-001** — A report-section contract covering warnings, assumptions, missing solve and rule inputs, user-supplied values, provenance, units, reproducibility references, and professional-review notices is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-08-03 Warnings, assumptions, and provenance report section

> #### Datasheet: DEL-08-03 Warnings, assumptions, and provenance report section
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-03-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-08-03 |
> | Deliverable Name | Warnings, assumptions, and provenance report section |
> | Package ID | PKG-08 |
> | Package Name | Reporting, Audit, and Reproducibility |
> | Deliverable Type | BACKEND_FEATURE_SLICE |
> | Scope Item | SOW-024 |
> | Objectives | OBJ-007, OBJ-011 |
> | Context Envelope | M |
> | Lifecycle Location | `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-03_Warnings, assumptions, and provenance report section/` |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Report section subject | Missing data, warnings, assumptions, user-supplied values, and source/provenance notes | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-08-03 |
> | Parent report scope | Auditable calculation reports including inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations | `docs/_Registers/ScopeLedger.csv` row SOW-024 |
> | Professional boundary | Software assists analysis and does not authenticate engineering work | `_CONTEXT.md`; `docs/CONTRACT.md` OPS-K-AUTH-1 |
> | Protected-content boundary | Public templates/examples must not reproduce protected standards text, tables, figures, examples, copied formulas, allowables, SIF/flexibility tables, or proprietary commercial data | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-REPORT-2 |
> | Data boundary | Code-specific values are user-supplied or lawfully imported private data, not bundled defaults | `docs/CONTRACT.md` OPS-K-DATA-1 |
> | Missing data treatment | Missing solve-required or rule-check-required values are explicit findings, never silent defaults | `docs/CONTRACT.md` OPS-K-DATA-2 |
> | Provenance treatment | Materials, components, SIFs, flexibility factors, allowables, and rule-pack values carry provenance fields | `docs/CONTRACT.md` OPS-K-DATA-3 |
> | Diagnostics basis | Diagnostics/result envelopes carry code, class, severity, source, affected object, message, remediation, and provenance | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06 |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Datasheet Value |
> |---|---|
> | Implementation state for this setup run | Documentation/setup only; no report code is implemented in this session |
> | Required upstream inputs | Diagnostic/result-envelope contract, warning classes, report generator integration point, audit manifest/hash metadata, and protected-content guardrails |
> | User/private data handling | Report section may reference private rule-pack or source identifiers but must not expose protected/private content in public templates by default |
> | Engineering values | No engineering values are created here; unknown or absent values remain `TBD` or report findings |
> | Certification/compliance claims | Prohibited for software-generated report language |
> | Unit handling | Any future rendered values must preserve units and dimensional context; this setup artifact does not define value schemas |
>

### CLM-006 — Construction

> ##### Construction
>
> The future report section is expected to be a report-renderer subsection that consumes already-classified diagnostics and provenance metadata rather than creating new engineering judgments. The section should present the following report-facing groups:
>
> | Group | Purpose | Boundary |
> |---|---|---|
> | Missing data | Expose absent solve-required or rule-check-required inputs | Do not infer or backfill engineering values |
> | Warnings | Render diagnostic classes from GUI/core/result envelopes | Do not reclassify solver or rule-engine findings without evidence |
> | Assumptions | Show user/model assumptions requiring review | Do not convert assumptions into accepted facts |
> | User-supplied values | Identify values supplied by users, private libraries, or rule packs where report-facing provenance is needed | Do not reproduce protected source content |
> | Source/provenance notes | Summarize source identity, license/redistribution status, checksum/ref where applicable, and review status where available | Do not claim legal sufficiency or professional acceptance |
> | Professional notice | State the software output is decision support and requires competent human review before reliance | Do not claim certification, sealing, approval, or code compliance |
>

### CLM-007 — Provenance Payload Field Inventory

> ###### Provenance Payload Field Inventory
>
> Exact report-section schema field names are `TBD` until implementation-level schema/API work is available. The field families below are the setup basis only:
>
> | Field Family | Setup Basis | Field Name Status |
> |---|---|---|
> | Diagnostic trace | code, class, severity, source, affected object, message, remediation, provenance | Names constrained by AB-00-06; exact schema names `TBD` |
> | Source record | source name, source location, source license or redistribution basis | Names constrained by IP/data policy; exact schema names `TBD` |
> | Rule-pack reference | rule-pack ID/name, version, checksum, source notice, public/private status | Names constrained by rule-pack/report requirements; exact schema names `TBD` |
> | Review disposition | unknown source, protected suspected, review status, human review required | Names constrained by data-boundary and professional-responsibility requirements; exact schema names `TBD` |
>

### CLM-008 — References

> ##### References
>
> | Reference | Used For |
> |---|---|
> | `_CONTEXT.md` | Deliverable identity, package scope, architecture basis injection |
> | `_REFERENCES.md` | Local reference list for this deliverable |
> | `docs/_Registers/Deliverables.csv` | Deliverable row, anticipated artifacts, context notes |
> | `docs/_Registers/ScopeLedger.csv` | SOW-024 scope statement and protected-content note |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-08 and DEL-08-03 decomposition context; AB-00 architecture basis |
> | `docs/CONTRACT.md` | Applicable invariants for reports, data, IP, rule packs, privacy, units, and agent outputs |
> | `docs/SPEC.md` | Reporting/audit requirements and warning class vocabulary |
> | `docs/TYPES.md` | Analysis-status, epistemic, provenance, and report vocabulary |
> | `docs/DIRECTIVE.md` | Product boundaries, stop rules, and professional responsibility principles |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and report boundary policy |
> | `docs/VALIDATION_STRATEGY.md` | Report reproducibility and protected-content validation expectations |

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-08-03 Warnings, assumptions, and provenance report section

> #### Specification: DEL-08-03 Warnings, assumptions, and provenance report section
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-010 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-03-DECL-001`.
>

### CLM-011 — Scope

> ##### Scope
>
> This deliverable specifies the report section that exposes missing data, warnings, assumptions, user-supplied values, and source/provenance notes for OpenPipeStress calculation reports.
>
> In scope:
>
> - report-facing requirements for warning, assumption, missing-data, and provenance disclosure;
> - integration expectations for diagnostics/result-envelope data supplied by GUI/core/rule/report workflows;
> - protected-content and professional-responsibility guardrails for public report templates;
> - acceptance criteria and evidence expected from future implementation work.
>
> Out of scope:
>
> - implementation of report renderer code in this setup session;
> - report generator implementation owned by DEL-08-01;
> - audit manifest/model hash implementation owned by DEL-08-02;
> - result export format owned by DEL-08-04;
> - protected-content linter implementation owned by DEL-08-05;
> - creation or bundling of code-specific values, protected standards content, proprietary formulas, or professional acceptance records.
>

### CLM-012 — Requirements

> ##### Requirements
>
> | Requirement ID | Requirement | Source | Verification |
> |---|---|---|---|
> | DEL-08-03-REQ-001 | The report section shall expose missing solve-required and rule-check-required data as explicit report findings. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` Section 8 | Report fixture or snapshot includes missing-data findings without default values |
> | DEL-08-03-REQ-002 | The report section shall render warning classes supplied by diagnostics/result envelopes, including solve blocking, rule-check blocking, provenance, assumption, nonlinear, and IP-boundary warnings where present. | `docs/SPEC.md` Section 7; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06 | Fixture covers each supported warning class and preserves class/severity/code |
> | DEL-08-03-REQ-003 | Each rendered warning or assumption shall preserve machine-readable trace data when supplied: code, class, severity, source, affected object, message, remediation, and provenance. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06 | Schema/snapshot test confirms trace fields are retained or explicitly marked `TBD` |
> | DEL-08-03-REQ-004 | User-supplied values and rule-pack references shall be identified as user-supplied or private where applicable, with source/provenance notes and rule-pack identity/checksum references when supplied by upstream artifacts. | `docs/CONTRACT.md` OPS-K-DATA-1, OPS-K-DATA-3, OPS-K-RULE-3; `docs/SPEC.md` Section 8 | Report fixture shows source/provenance and rule-pack ref/checksum without embedding protected content |
> | DEL-08-03-REQ-005 | The report section shall include a professional-responsibility notice stating that professional reliance requires competent human review and that software output is decision support. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` Section 8 | Snapshot or template check finds the notice and rejects certification/approval language |
> | DEL-08-03-REQ-006 | The report section shall not reproduce protected code text, protected standards tables, copyrighted examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data in public templates/examples. | `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-REPORT-2; `docs/IP_AND_DATA_BOUNDARY.md` Section 7 | Protected-content lint and review evidence pass for public templates/examples |
> | DEL-08-03-REQ-007 | The report section shall preserve unit context for any rendered values and shall not display unit-bearing values without their units when units are supplied. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/SPEC.md` Section 8 | Report fixture includes unit-bearing values with units and rejects unitless display where units exist |
> | DEL-08-03-REQ-008 | The report section shall distinguish mechanics-solved, user-rule-checked, incomplete-data, and human-review-required states without presenting software-generated code compliance. | `docs/TYPES.md` Section 4; `docs/CONTRACT.md` OPS-K-MECH-2 and OPS-K-AUTH-1 | Status fixture avoids `CODE_COMPLIANT` and includes human-review-required boundary language |
> | DEL-08-03-REQ-009 | The report section shall support reproducibility by referencing model/report metadata supplied by audit manifest work, including model hash, software/solver version, and rule-pack checksum when present. | `docs/SPEC.md` Section 8; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 | Integration fixture consumes manifest fields by reference or marks unavailable fields `TBD` |
> | DEL-08-03-REQ-010 | Missing source/provenance shall itself be reportable as a provenance warning rather than silently omitted. | `docs/CONTRACT.md` OPS-K-DATA-2, OPS-K-DATA-3; `docs/SPEC.md` Section 7 | Fixture with `UNKNOWN_SOURCE` or missing source emits a provenance warning |
> | DEL-08-03-REQ-011 | If automated protected-content lint is unavailable during early implementation, the report-section change shall remain subject to explicit human review and shall record the unavailable lint gate as an open verification item. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-REPORT-2; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` Review checklist | Review evidence records lint status and human review requirement |
> | DEL-08-03-REQ-012 | Report-section tests or review checks shall reject language that claims software certification, approval, endorsement, sealing, authentication, or automatic code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/DIRECTIVE.md` Section 4.2 | Snapshot/lint/review check covers prohibited claim language |
>

### CLM-013 — Standards

> ##### Standards
>
> No protected standards text is required or authorized for this deliverable. Governing standards-body content is intentionally not embedded in public artifacts. Applicable project standards are the OpenPipeStress governance artifacts listed in `_REFERENCES.md`, especially `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
>

### CLM-014 — Verification

> ##### Verification
>
> Future implementation work should provide at least these checks:
>
> | Check | Purpose |
> |---|---|
> | Warning fixture coverage | Verifies rendering for each supported warning class without reclassification |
> | Missing-data fixture | Verifies absent solve/rule data appears as a finding and not a default |
> | Provenance fixture | Verifies source/provenance fields, private/public status, and `UNKNOWN_SOURCE` handling |
> | Professional-boundary snapshot | Verifies the notice is present and prohibited certification/compliance language is absent |
> | Protected-content lint | Verifies public templates/examples do not embed protected standards content |
> | Lint fallback review | Verifies unavailable protected-content lint is recorded as an open verification item and does not advance silently |
> | Prohibited-claim check | Verifies report text avoids certification, approval, endorsement, sealing, authentication, and automatic code-compliance claims |
> | Unit display check | Verifies unit-bearing values are rendered with units |
> | Reproducibility reference check | Verifies manifest/hash/rule-pack checksum fields are referenced when supplied |
>

### CLM-015 — Documentation

> ##### Documentation
>
> Required setup artifacts for this run:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/*`
> - `_STATUS.md`
>
> Future production artifacts anticipated by the register:
>
> - report provenance section;
> - tests.

- **AC-001** — The contract preserves upstream diagnostic class, severity, source, affected object, message, remediation and provenance when supplied; exposes missingness rather than defaulting it; protects private/protected content; and makes no certification, approval, sealing, authentication, or automatic-compliance claim.

## Production and Verification Method — Praxeology

### CLM-016 — Procedure: DEL-08-03 Warnings, assumptions, and provenance report section

> #### Procedure: DEL-08-03 Warnings, assumptions, and provenance report section
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-017 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-03-DECL-004`.
>

### CLM-018 — Purpose

> ##### Purpose
>
> This procedure describes how a future TASK implementation should produce and verify the warnings, assumptions, and provenance report section while preserving the sealed scope, protected-data boundary, diagnostics/result-envelope boundary, and professional-responsibility boundary.
>

### CLM-019 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Expected Source | Status |
> |---|---|---|
> | Sealed DEL-08-03 brief and explicit write scope | `_CONTEXT.md` and human dispatch | Available for setup |
> | Applicable invariants and architecture basis | `docs/CONTRACT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08 | Available for setup |
> | Diagnostics/result-envelope contract | AB-00-06 and future schema/interface artifacts | Partly available; exact implementation `TBD` |
> | Report generator integration point | DEL-08-01 | Upstream/integration dependency |
> | Audit manifest/hash metadata | DEL-08-02 | Upstream/integration dependency |
> | Warning data from GUI/core | DEL-07-04, DEL-07-07, solver/rule diagnostics deliverables | Upstream/integration dependency |
> | Protected-content guardrail | DEL-08-05 and IP/data policy | Upstream/integration dependency for linter implementation |
>

### CLM-020 — Steps

> ##### Steps
>
> 1. Confirm scope.
>    - Verify the work remains limited to DEL-08-03 report-section behavior.
>    - Do not implement unrelated report generator, manifest, export, linter, GUI, solver, or rule-engine code.
>
> 2. Identify input payloads.
>    - Use diagnostics/result envelopes as the source for warning class, code, severity, source, affected object, message, remediation, and provenance.
>    - Use report/audit manifest inputs for software version, solver version, model hash, rule-pack name/version/checksum, and source notes where supplied.
>    - Mark unavailable fields `TBD` instead of inventing values.
>
> 3. Map report subsections.
>    - Missing data findings.
>    - Warnings grouped or filterable by class/severity/source.
>    - Assumptions requiring review.
>    - User-supplied values and private/public source status.
>    - Source/provenance summary.
>    - Professional-responsibility and protected-content notices.
>
> 4. Preserve boundaries.
>    - Do not reproduce protected standards text, copied standards tables, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary vendor data in public templates/examples.
>    - Do not claim certification, sealing, approval, authentication, endorsement, or automatic code compliance.
>    - Do not transmit or publish private project/rule-pack/library data by default.
>    - Bind final professional-boundary wording to a canonical product-claims policy or approved report notice when available; until then, keep wording conservative and mark exact final text `TBD`.
>
> 5. Implement or refresh report fixtures.
>    - Include at least one fixture for each warning class listed in the diagnostics basis where upstream fixtures exist.
>    - Include fixtures for missing solve data, missing rule-check data, missing provenance, assumptions, private rule-pack reference, and human-review-required status.
>    - Use invented/non-code data only for public examples.
>
> 6. Verify output.
>    - Run report snapshot or equivalent tests for expected section content.
>    - Run protected-content lint once DEL-08-05 or equivalent tooling is available.
>    - Confirm unit-bearing values retain units.
>    - Confirm status language distinguishes mechanics solve, user rule check, and human review.
>
> 7. Record evidence.
>    - List changed report-section files and tests.
>    - Record validation commands/results.
>    - Record warnings, `TBD` fields, and open dependencies.
>

### CLM-021 — Verification

> ##### Verification
>
> | Verification Item | Acceptance Signal |
> |---|---|
> | Four-document setup | `tools/validation/check_four_documents.sh <deliverable_path>` passes |
> | Semantic setup | `_SEMANTIC.md` exists and semantic audit records PASS |
> | Lensing setup | `_SEMANTIC_LENSING.md` exists with complete matrix coverage |
> | Dependency setup | `Dependencies.csv` validates against v3.1 schema |
> | Dependency schema command | `python3 tools/validation/validate_dependencies_schema.py <deliverable_path>/Dependencies.csv` passes |
> | Scope guard | No files outside the DEL-08-03 working folder are modified by this setup run |
> | Protected-data guard | Public setup artifacts contain no protected standards text/tables/formulas or proprietary data |
> | Professional-boundary guard | Setup artifacts do not claim certification, approval, sealing, endorsement, or software-authenticated code compliance |
> | Lint fallback guard | If protected-content lint tooling is unavailable, record that status as an open verification item requiring human review |
>

### CLM-022 — Records

> ##### Records
>
> This setup run should leave these records in the deliverable folder:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/`
> - `_STATUS.md`

- **VER-001** — Validate the contract and review source parity, all warning and missing-data classes, trace-field preservation, unit and provenance visibility, manifest references, protected-content controls and fallback review, prohibited-claim language, and every retained TBD or governed residual.

## Governing Values and Decisions — Axiology

### CLM-023 — Guidance: DEL-08-03 Warnings, assumptions, and provenance report section

> #### Guidance: DEL-08-03 Warnings, assumptions, and provenance report section
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-024 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-03-DECL-003`.
>

### CLM-025 — Purpose

> ##### Purpose
>
> This deliverable exists so calculation reports make uncertainty, missing inputs, assumptions, and provenance visible to professional reviewers. The section should help a reviewer see what the software computed, what the user supplied, what remains incomplete, and what must not be treated as software-certified engineering approval.
>

### CLM-026 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Findings over defaults | Missing solve-required or rule-check-required values should appear as explicit findings, not be filled by convenience defaults. |
> | Provenance over opacity | Values affecting engineering reliance should carry source/provenance notes or a visible `TBD`/unknown-source finding. |
> | Warnings preserve origin | Report rendering should carry diagnostic class, severity, source, affected object, and remediation from the producing layer when available. |
> | Decision support only | Report language may support professional review but must not claim certification, sealing, approval, authentication, or automatic code compliance. |
> | Protected-content restraint | Public templates/examples may identify rule-pack references and checksums but must not reproduce protected standards text, tables, copied formulas, or proprietary content. |
> | Unit visibility | Values shown in the section should preserve unit context wherever units exist in upstream data. |
>

### CLM-027 — Considerations

> ##### Considerations
>
> The report section should be a consumer of upstream diagnostics and provenance, not a source of new engineering meaning. If upstream data says a rule-pack input is missing, the report should expose that state. If a source note is missing, the report should expose that as a provenance warning. If a human has not accepted the output for project use, the report should not imply that acceptance.
>
> The section should remain useful even when upstream components are incomplete. `TBD`, `UNKNOWN_SOURCE`, or missing manifest fields are acceptable setup outputs when the evidence is not available. They should be made visible rather than normalized away.
>
> When upstream diagnostic or provenance fields are missing, the safer report behavior is to preserve the missingness as `TBD` or an explicit warning. Inferring a message, source, severity, or professional disposition inside the report renderer would create unaudited meaning and could blur the boundary between software output and engineering judgment.
>

### CLM-028 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Preferred Direction |
> |---|---|
> | Readability vs audit detail | Keep the section readable, but retain machine-traceable warning/provenance fields in the report source data. |
> | Private detail vs public safety | Public templates should reference private rule-pack/library identity, version, checksum, and source notes without exposing protected/private content by default. |
> | Concise warnings vs remediation | Include enough remediation/source context for review; avoid turning warnings into instructions that imply professional acceptance. |
> | Early implementation vs dependency maturity | Future code may begin with schema-backed fixtures and `TBD` integration points while upstream diagnostics, manifest, and linter deliverables mature. |
>

### CLM-029 — Examples

> ##### Examples
>
> The following examples are invented reporting patterns only. They do not encode code-specific requirements or protected standards content.
>
> | Scenario | Report Section Behavior |
> |---|---|
> | Rule-pack required input is absent | Show a rule-check blocking finding, affected object/reference if supplied, and remediation such as "provide required user/rule-pack input"; do not report a pass/fail code result. |
> | Component value has no source note | Show the value only with its unit context if otherwise permitted, mark source/provenance as `UNKNOWN_SOURCE` or `TBD`, and emit a provenance warning. |
> | User entered an assumption for a support condition | Show the assumption as user/model-supplied and requiring review; do not treat it as verified project acceptance. |
> | Private rule pack is used | Show rule-pack name or ID, version, checksum, private/public status, and source notice if available; do not quote protected formulas or standards clauses in public templates. |
> | Solver emitted nonlinear uncertainty | Show nonlinear warning class, severity, source, affected object, and remediation if supplied by the result envelope. |
>

### CLM-030 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No unresolved source conflict found during setup. | N/A | N/A | N/A | N/A | N/A |
>

### CLM-031 — Open Questions

> ##### Open Questions
>
> | Question ID | Question | Current Disposition |
> |---|---|---|
> | DEL-08-03-Q-001 | Exact report renderer API and template format | `TBD`; owned by future implementation and DEL-08-01 integration |
> | DEL-08-03-Q-002 | Exact data contract for warning/provenance payloads | `TBD`; constrained by AB-00-06 diagnostics/result-envelope basis |
> | DEL-08-03-Q-003 | Exact protected-content linter interface | `TBD`; likely dependent on DEL-08-05 |
> | DEL-08-03-Q-004 | Exact audit manifest field names | `TBD`; likely dependent on DEL-08-02 |
> | DEL-08-03-Q-005 | Canonical professional notice wording for final templates | `TBD`; align with product-claims policy or approved report notice before implementation release |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-024 OBJ-007 OBJ-011 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
