---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-02-04
package_id: PKG-02
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-038]
package_objective_refs: [OBJ-009]
---

# Scope of Work — DEL-02-04

## Purpose and Objective Traceability

This Scope of Work defines `DEL-02-04` in service of project scope [SOW-038] and package objectives [OBJ-009].

- **OUT-001** — Plugin and extension domain contracts defining governed extension points, adapter boundaries, permissions, and schema-first application-service access are produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Source preamble

> ---
> doc_id: DEL-02-04-DATASHEET
> doc_kind: deliverable.datasheet
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-02-04
> package_id: PKG-02
> ---
>

### CLM-002 — Datasheet: Plugin and Extension Domain Contracts

> #### Datasheet: Plugin and Extension Domain Contracts
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-003 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-02-04-DECL-002`.
>

### CLM-004 — Identification

> ##### Identification
>
> | Field | Value | Evidence |
> |---|---|---|
> | Deliverable ID | DEL-02-04 | SourcePath: `_CONTEXT.md`; SectionRef: Context: DEL-02-04 |
> | Name | Plugin and extension domain contracts | SourcePath: `_CONTEXT.md`; SectionRef: Context: DEL-02-04 |
> | Package | PKG-02 - Domain Model, Units, and Core Schemas | SourcePath: `_CONTEXT.md`; SectionRef: Package Reference |
> | Type | API_CONTRACT | SourcePath: `_CONTEXT.md`; SectionRef: Type |
> | Scope item | SOW-038 | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-038 |
> | Objective | OBJ-009 | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: Objective-to-deliverable mapping, OBJ-009 |
> | Anticipated artifacts | plugin interface spec; sandbox/permission model notes | SourcePath: `_CONTEXT.md`; SectionRef: Anticipated Artifacts |
> | Decomposition basis | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 | SourcePath: `_CONTEXT.md`; SectionRef: Decomposition Reference |
> | Discipline/domain | API and domain-contract governance for plugins/adapters | ASSUMPTION from Type=`API_CONTRACT`, package scope, and SOW-038 |
> | Responsible party | TBD | Not stated in accessible sources |
> | Governance/ruling owner | TBD | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AGENT-4. No project authority is assigned in the accessible sources for plugin-contract rulings. |
>

### CLM-005 — Attributes

> ##### Attributes
>
> | Attribute | Value | Evidence |
> |---|---|---|
> | Primary contract subject | Extension points and governance constraints for plugins/adapters | SourcePath: `_CONTEXT.md`; SectionRef: Description |
> | Package boundary | Domain entities, unit system, persistence/serialization contracts, and extensibility boundaries; excludes numerical solving and GUI views | SourcePath: `_CONTEXT.md`; SectionRef: Package Reference |
> | Core governance intent | Extensibility must not bypass governance, unit safety, or data-boundary constraints | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-038 |
> | Architecture basis IDs | AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08 | SourcePath: `_CONTEXT.md`; SectionRef: Architecture Basis Injection |
> | Schema/API baseline | JSON Schema 2020-12 for public schemas/interchange; schema-first command/query/job/result envelopes | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline |
> | Hash basis when JSON payloads are hashed | Canonical JSON with JCS-compatible canonicalization | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline |
> | Mandatory boundary checks | Units, provenance, redistribution status, diagnostics, public/private data boundary, report controls | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.1 Architecture basis register, AB-00-07 |
> | Diagnostic payload basis | Diagnostic/result envelopes carry code, class, severity, source, affected object, message, remediation, and provenance | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.1 Architecture basis register, AB-00-06 |
> | External API transport | TBD | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline |
> | Concrete import/export formats | TBD | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: OI-004 |
> | Detailed permission taxonomy and sandbox implementation | TBD | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-038 notes |
>

### CLM-006 — Conditions

> ##### Conditions
>
> - Public artifacts must not include protected standards text, copied tables, protected examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1.
> - Suspected protected content must be quarantined and escalated rather than paraphrased into public data. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-3.
> - Code-specific values and proprietary data are user-supplied or lawfully imported private data, not bundled public defaults. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-1.
> - All plugin or adapter ingress, egress, and rule-related values must remain unit-aware and dimensionally checked where numerical or dimensional data is involved. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-UNIT-1.
> - Plugin and adapter outputs: Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AUTH-1.
> - Any rule-pack or expression-facing extension must remain sandboxed and unable to execute arbitrary code. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-RULE-2.
> - Telemetry, if ever exposed to plugins, is off by default and must not transmit private engineering/code data. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-PRIV-2.
>

### CLM-007 — Construction

> ##### Construction
>
> The DEL-02-04 contract kit is constructed from the following conservative parts:
>
> | Part | Status | Notes |
> |---|---|---|
> | Plugin interface spec | Draft target | Must define extension point identity, lifecycle status, schema version, capability declarations, diagnostic envelope use, and validation obligations. Exact schema file layout is TBD. |
> | Sandbox/permission model notes | Draft target | Must express deny-bypass constraints for units, provenance, data boundary, diagnostics, report controls, and rule sandboxing. Exact runtime mechanism is TBD. |
> | Extension point registry | ASSUMPTION / TBD approval record | Candidate families are import adapter, export adapter, report/output extension, validation hook, and rule-pack integration hook. Approved registry names, registry ownership, and a decision/open-issue pointer are TBD. Concrete public API transport and format support remain TBD. |
> | Capability/permission declarations | ASSUMPTION / TBD approval record | Candidate classes for later review are governed project mutation, read-only query access, private-library access, filesystem path access, network access, report/export generation, background job execution, and rule-pack evaluator integration. Exact permission names, grant records, enforcement implementation, and approval path remain TBD. |
> | Human-ruling log pointer | TBD | Needed before exact extension registry, permission taxonomy, sandbox mechanism, transport binding, or telemetry/private-data exposure can be treated as approved. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: DEC-012 and OI-004. |
> | Provenance and redistribution metadata | Required concept | Public data records require source, source location, license, contributor certification, redistribution status, and review status. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 4. Required provenance fields. |
> | Verification hooks | Required concept | Future implementation should support schema validation, unit checks, protected-content/provenance gates, and adapter/plugin tests. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.1 Architecture basis register, AB-00-08. |
>

### CLM-008 — References

> ##### References
>
> - `_CONTEXT.md`, revision 0.7 accepted basis for DEL-02-04.
> - `_REFERENCES.md`, local reference index for DEL-02-04.
> - `_DEPENDENCIES.md`, human-owned dependency declarations for DEL-02-04.
> - `docs/_Registers/Deliverables.csv`, row DEL-02-04.
> - `docs/_Registers/ScopeLedger.csv`, row SOW-038.
> - `docs/_Registers/ContextBudgetQA.csv`, row DEL-02-04.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md`, revision 0.7, especially PKG-02, DEL-02-04, OBJ-009, SOW-038, and SCA-001 architecture basis rows.
> - Traceability note: `_CONTEXT.md` identifies `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 as the accepted basis, while `_REFERENCES.md` still describes the decomposition reference as accepted v0.2. `_REFERENCES.md` is outside this run's write scope; see `Guidance.md` Conflict Table.
> - `docs/CONTRACT.md`, invariant catalog.
> - `docs/TYPES.md`, deliverable types, status vocabulary, epistemic labels, and data provenance labels.
> - `docs/SPEC.md`, sections 1, 6, 7, 8, 10, and 11.
> - `docs/IP_AND_DATA_BOUNDARY.md`, public/private data policy.
> - `docs/DIRECTIVE.md`, product boundaries and stop rules.
> - `docs/PRD.md`, sections 6.2, 6.3, 6.6, 12.3, 12.4, 15.2, 15.3, 18, and 19.

## Completion and Reliance Basis — Epistemology

### CLM-009 — Source preamble

> ---
> doc_id: DEL-02-04-SPECIFICATION
> doc_kind: deliverable.specification
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-02-04
> package_id: PKG-02
> ---
>

### CLM-010 — Specification: Plugin and Extension Domain Contracts

> #### Specification: Plugin and Extension Domain Contracts
>

### CLM-011 — Scope

> ##### Scope
>
> DEL-02-04 defines the domain/API contract surface for plugin and adapter extension points in OpenPipeStress. It covers plugin/adapter interface expectations, mandatory governance checks, sandbox/permission-model notes, diagnostics obligations, and data-boundary constraints. SourcePath: `_CONTEXT.md`; SectionRef: Description and Anticipated Artifacts.
>
> This deliverable is bounded to PKG-02 domain/API definitions. It does not implement a plugin loader, public API transport, external import/export formats, GUI views, numerical solving behavior, rule-pack evaluator internals, storage container mechanics, or concrete dependency versions. SourcePath: `_CONTEXT.md`; SectionRef: Package Reference; SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline and DEC-012.
>

### CLM-012 — Requirements

> ##### Requirements
>
> | ID | Requirement | Evidence |
> |---|---|---|
> | DEL-02-04-REQ-01 | The contract shall define extension points for plugins/adapters without allowing those extensions to bypass governance, validation, diagnostics, unit safety, report controls, or public/private data-boundary checks. | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: SOW-038. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-02 and AB-00-07. |
> | DEL-02-04-REQ-02 | Plugin/adapter data ingress and egress shall be unit-aware where dimensional or numerical values are present, and incompatible or missing unit metadata shall be reported rather than silently defaulted. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-UNIT-1 and OPS-K-DATA-2. SourcePath: `docs/PRD.md`; SectionRef: 6.6 Unit Safety. |
> | DEL-02-04-REQ-03 | Plugin/adapter contracts shall preserve source/provenance metadata for material, component, rule, load, allowable, SIF/flexibility, and report-affecting values. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-3. SourcePath: `docs/PRD.md`; SectionRef: 6.3 Data Provenance by Design. |
> | DEL-02-04-REQ-04 | Any public data contribution path exposed through plugins/adapters shall require source, source location, redistribution/license status, contributor certification, and review disposition; missing values shall be `TBD` or rejected by policy. | SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 4. Required provenance fields. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-2 and OPS-K-AGENT-1. |
> | DEL-02-04-REQ-05 | Plugin/adapter contracts shall prohibit public redistribution of protected standards text, protected tables, protected examples, copied code formulas, protected dimensional data, proprietary vendor data without rights, and private rule/project/library data. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 3. Public repository must not contain. |
> | DEL-02-04-REQ-06 | Suspected protected or proprietary content encountered by a plugin/adapter shall be classified, quarantined, and escalated for human/legal review rather than imported into public artifacts. | SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 5. Quarantine rule. SourcePath: `docs/DIRECTIVE.md`; SectionRef: 5. Authority and stop rules. |
> | DEL-02-04-REQ-07 | Plugin/adapter diagnostics shall use the project diagnostic/result-envelope basis, including code, class, severity, source, affected object, message, remediation, and provenance where applicable. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-06. |
> | DEL-02-04-REQ-08 | Plugin/adapter outputs and reports shall not claim certification, sealing, approval, authentication, endorsement, or engineering code compliance for reliance (PRD §21.2). | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AUTH-1. SourcePath: `docs/DIRECTIVE.md`; SectionRef: 4.2 Out of scope. |
> | DEL-02-04-REQ-09 | Mutating plugin/adapter operations shall route through schema-first application-service commands or equivalent governed service boundaries, not direct domain-core, solver, storage, or report-control bypasses. For this contract, an equivalent governed boundary preserves schema validation, unit checks where dimensional data is present, diagnostics/result envelopes, provenance and public/private data-boundary checks, report controls, and audit/reproducibility metadata. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-02, AB-00-03, and AB-00-07. SourcePath: `docs/SPEC.md`; SectionRef: 1. Architectural overview. |
> | DEL-02-04-REQ-10 | Read-only and long-running plugin/adapter operations shall preserve the command/query/job distinction where exposed through application services, including cancellation/progress and reproducibility metadata when applicable. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-03. SourcePath: `docs/PRD.md`; SectionRef: 20. Performance Requirements. |
> | DEL-02-04-REQ-11 | Public plugin manifests and interchange contracts shall align with the accepted JSON Schema 2020-12 and schema-first envelope baseline. Exact schema file layout and code-generation tooling remain TBD. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline and DEC-010. |
> | DEL-02-04-REQ-12 | JSON payload hashes used by plugin/adapter manifests, provenance records, or reproducibility artifacts shall use the accepted canonical JSON/JCS-compatible hash basis when hashing is required. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-04 and 8.2 Resolved architecture baseline. |
> | DEL-02-04-REQ-13 | Rule-pack-facing extension hooks shall remain sandboxed, deterministic, unit-aware, and incapable of arbitrary filesystem or network access. Exact expression grammar/library remains TBD and is not resolved by this deliverable. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-RULE-2. SourcePath: `docs/PRD.md`; SectionRef: 12.3 Rule-Pack Evaluator. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: OI-006. |
> | DEL-02-04-REQ-14 | Plugin/adapter contract verification shall include layered checks for schema conformance, unit safety, provenance, diagnostics, protected-content gates, and relevant adapter/plugin regression behavior. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-08. SourcePath: `docs/VALIDATION_STRATEGY.md`; SectionRef: 2. Benchmark families and 4. Release gate. |
> | DEL-02-04-REQ-15 | The contract shall explicitly record remaining implementation-level TBDs: public API transport, concrete import/export formats, exact sandbox mechanism, detailed permission names, exact dependency versions, and CI thresholds. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline, OI-004, and DEC-012. |
> | DEL-02-04-REQ-16 | Public plugin manifest/interface documentation shall include a placeholder concept inventory before contract issuance. Exact JSON Schema file layout, field names, transport binding, and code-generation tooling remain TBD until a human/architecture review resolves them. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline and DEC-010. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: DEC-012. |
> | DEL-02-04-REQ-17 | Exact extension-point registry entries, permission taxonomy names, sandbox mechanism, and approval path shall remain `TBD` until a documented human/project authority or security/architecture review records a decision; the interim contract shall not grant plugin capabilities by default. | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-038. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07 and DEC-012. |
> | DEL-02-04-REQ-18 | Plugins/adapters shall not receive telemetry-facing data or transmit private project, rule-pack, component, material, or calculation-result data by default. Any later plugin exposure to telemetry-facing data remains TBD and requires explicit privacy/security ruling and verification coverage. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-PRIV-1 and OPS-K-PRIV-2. SourcePath: `docs/PRD.md`; SectionRef: 18.2 Telemetry. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: OI-008. |
>

### CLM-013 — Standards

> ##### Standards
>
> No protected engineering code, standards clauses, tables, or examples are incorporated into this deliverable.
>
> Applicable internal standards and baselines:
>
> - `docs/CONTRACT.md` invariant catalog, especially OPS-K-IP-*, OPS-K-DATA-*, OPS-K-AUTH-*, OPS-K-UNIT-1, OPS-K-RULE-2, OPS-K-REPORT-*, OPS-K-PRIV-*, and OPS-K-AGENT-*.
> - `docs/TYPES.md` for deliverable types, analysis-status vocabulary, epistemic labels, and data provenance labels.
> - `docs/SPEC.md` for layer responsibilities, rule-pack evaluator constraints, diagnostics classes, reporting/audit, and Type 2 acceptance semantics.
> - `docs/IP_AND_DATA_BOUNDARY.md` for public/private data and provenance policy.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for SOW-038, OBJ-009, SCA-001 basis IDs, JSON Schema 2020-12, schema-first envelopes, canonical JSON/JCS-compatible hashing, and remaining-TBD boundaries.
>
> External implementation standards:
>
> - JSON Schema 2020-12 is the accepted public schema/interchange baseline by SCA-001. This document does not reproduce the JSON Schema specification text. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline.
>

### CLM-014 — Verification

> ##### Verification
>
> | Requirement IDs | Verification approach |
> |---|---|
> | REQ-01, REQ-09, REQ-10 | Architecture review confirms plugin/adapter calls enter through governed service/domain contracts and cannot bypass validation, diagnostics, envelopes, or report controls. |
> | REQ-02 | Unit schema tests and dimensional validation tests reject incompatible units and surface missing-unit diagnostics. |
> | REQ-03, REQ-04 | Provenance-field schema validation and contribution-review checklist confirm required source, license, contributor, redistribution, and review metadata. |
> | REQ-05, REQ-06 | Protected-content/provenance gate and quarantine procedure exercise public and private data paths with invented non-code fixtures only. |
> | REQ-07 | Diagnostic envelope tests confirm required fields and warning class propagation for plugin/adapter failures. |
> | REQ-08 | Report/template and message review confirms no certification, approval, endorsement, or code-compliance claims (PRD §21.2). |
> | REQ-11, REQ-12 | Schema validation and canonicalization/hash tests verify manifest and envelope stability once concrete schemas exist. |
> | REQ-13 | Security tests for any rule-pack-facing hook confirm no arbitrary code, filesystem, or network access unless a later human-approved sandbox design allows a bounded capability. |
> | REQ-14 | Layered test plan maps plugin/adapter behavior to schema, unit, provenance, diagnostics, protected-content, and regression gates. |
> | REQ-15, REQ-17 | Review confirms implementation-level TBDs remain visible and are not silently resolved or treated as approved capabilities. |
> | REQ-16 | Manifest/interface documentation review confirms each concept slot is present as an approved field or explicit `TBD` placeholder. |
> | REQ-18 | Privacy/security review confirms plugins/adapters do not receive telemetry-facing or private engineering data by default; any exception has an explicit human ruling and test evidence. |
>

### CLM-015 — Verification acceptance criteria and interim gates

> ###### Verification acceptance criteria and interim gates
>
> | Gate | Applies to | Pass/hold criteria |
> |---|---|---|
> | Schema/manifest review gate | REQ-11, REQ-12, REQ-16 | Hold until concrete schema layout, field names, transport binding, and code-generation tooling are either approved or carried as explicit `TBD`; pass only with schema validation and canonicalization/hash evidence where hashing is required. |
> | Layered plugin/adapter gate | REQ-01 through REQ-14 | Pass only when schema, unit, provenance, diagnostics, protected-content/provenance, and regression evidence is present or the missing evidence is explicitly recorded as `TBD`/open risk for human review. |
> | Rule-pack sandbox gate | REQ-13 | Pass only when tests demonstrate deterministic, unit-aware behavior and no arbitrary code execution or unauthorized filesystem/network access. Exact sandbox technology remains TBD until approved. |
> | Human-ruling gate | REQ-15, REQ-17, REQ-18 | Hold until open decisions for transport, registry, permission taxonomy, sandbox mechanism, import/export formats, telemetry/private-data exposure, and approval owner are recorded or explicitly deferred. |
> | Diagnostic class check | REQ-02, REQ-03, REQ-05, REQ-06, REQ-07, REQ-13 | Pass only when missing units, weak provenance, protected-content risk, assumptions, and rule-blocking outcomes surface through the existing warning/diagnostic classes where applicable; exact severity taxonomy beyond the source-defined classes remains TBD. |
>

### CLM-016 — Documentation

> ##### Documentation
>
> Required documentation artifacts for DEL-02-04:
>
> - Plugin interface specification covering identity, manifest metadata, extension point registration, capability declarations, validation lifecycle, diagnostic envelope behavior, and provenance/data-boundary obligations.
> - Sandbox/permission model notes covering deny-by-default posture, capability grant concepts, private data restrictions, network/filesystem/process boundaries, report/export controls, and explicit TBDs.
> - Requirement-to-verification matrix, as above, suitable for later PKG-10 implementation and PKG-12 security/privacy review.
> - Human-ruling log or future open-issue references for public API transport, exact extension point registry, exact permission taxonomy, import/export format list, and concrete sandbox technology.
>
> Manifest/interface placeholder inventory for later schema work:
>
> | Concept slot | Current status | Evidence |
> |---|---|---|
> | Plugin identity and version | TBD field names/layout | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline. |
> | Extension-point declaration | TBD approved registry names | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-038. |
> | Schema/envelope version | TBD field names/layout | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: DEC-010. |
> | Capability request/declaration | TBD permission names and approval path | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07 and DEC-012. |
> | Provenance/data-boundary declaration | Required concept; exact manifest fields TBD | SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 4. Required provenance fields. |
> | Diagnostics compatibility | Required concept; exact manifest fields TBD | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-06. |
> | Review/status metadata | Required concept; exact manifest fields TBD | SourcePath: `docs/TYPES.md`; SectionRef: 5. Epistemic labels. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AGENT-4. |

- **AC-001** — The contracts preserve the accepted source constraints on validation, units, provenance, diagnostics, private data, canonical hashing, storage access, and professional responsibility without granting direct SQL or protected-content access.

## Production and Verification Method — Praxeology

### CLM-017 — Source preamble

> ---
> doc_id: DEL-02-04-PROCEDURE
> doc_kind: deliverable.procedure
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-02-04
> package_id: PKG-02
> ---
>

### CLM-018 — Procedure: Plugin and Extension Domain Contracts

> #### Procedure: Plugin and Extension Domain Contracts
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-019 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-02-04-DECL-004`.
>

### CLM-020 — Purpose

> ##### Purpose
>
> This procedure describes how to produce and later maintain the DEL-02-04 plugin/extension domain contract artifacts without expanding beyond the sealed PKG-02 domain/API scope or introducing protected/private data into public artifacts.
>

### CLM-021 — Prerequisites

> ##### Prerequisites
>
> - Sealed deliverable context for DEL-02-04 with explicit write scope. SourcePath: `_CONTEXT.md`; SectionRef: Context: DEL-02-04.
> - Current `_STATUS.md` state that permits drafting or update. For this Pass 3 run, state is `SEMANTIC_READY`; the prior Pass 1+2 drafting history includes `OPEN` as recorded in `_STATUS.md`.
> - Scope, deliverable, and context-budget register rows for DEL-02-04 and SOW-038. SourcePath: `docs/_Registers/Deliverables.csv`; SectionRef: row DEL-02-04. SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-038. SourcePath: `docs/_Registers/ContextBudgetQA.csv`; SectionRef: row DEL-02-04.
> - Applicable contract invariants from `docs/CONTRACT.md`, especially IP/data boundary, unit safety, rule sandboxing, report controls, privacy, and agent epistemic constraints.
> - SCA-001 architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08. SourcePath: `_CONTEXT.md`; SectionRef: Architecture Basis Injection.
> - No human-declared upstream/downstream dependency list is available in `_DEPENDENCIES.md`; dependencies are coordinated externally by humans. ASSUMPTION: implementation work may later need outputs from DEL-02-01 and DEL-02-02, but this document kit does not add that dependency.
>

### CLM-022 — Steps

> ##### Steps
>
> 1. Confirm identity and boundary.
>    - Verify deliverable ID `DEL-02-04`, package `PKG-02`, type `API_CONTRACT`, scope item `SOW-038`, and objective `OBJ-009`.
>    - Confirm the work is limited to plugin/adapter domain/API contract artifacts and does not implement runtime loading, external formats, solver behavior, or GUI views.
>
> 2. Gather governing source slices.
>    - Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and the DEL-02-04 rows in the registers.
>    - Read `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/DIRECTIVE.md`, `docs/VALIDATION_STRATEGY.md`, and relevant SCA-001 rows in `execution/_Decomposition/SOFTWARE_DECOMP.md`.
>    - Use PRD sections only as supporting context where the decomposition or scope ledger cites them.
>
> 3. Define the plugin interface specification at contract level.
>    - Identify required manifest concepts: plugin identity, version, extension point declarations, schema/envelope version, capability request, provenance/data-boundary declaration, diagnostics compatibility, and review status.
>    - Mark exact schema file layout, field names, code-generation tooling, and transport binding as `TBD` unless later source material resolves them.
>
> 4. Define extension point families conservatively.
>    - Candidate families are import adapter, export adapter, report/output extension, validation hook, and rule-pack integration hook.
>    - Mark the registry as ASSUMPTION until the human project authority or later architecture/API deliverables approve concrete extension point names.
>
> 5. Apply mandatory no-bypass controls.
>    - Require unit-aware validation for dimensional/numerical data.
>    - Require source/provenance, redistribution status, and review disposition for imported or public data.
>    - Require diagnostics/result-envelope compatibility for plugin/adapter failures and warnings.
>    - Require protected-content, public/private data-boundary, and report-control checks before data is committed, exported, or included in public artifacts.
>    - Prohibit certification, approval, endorsement, or automatic code-compliance claims (PRD §21.2).
>
> 6. Draft sandbox and permission model notes.
>    - Start from deny-by-default.
>    - Record capability classes as ASSUMPTION/TBD until implementation design resolves them.
>    - Include at least these candidate classes for later review: governed project mutation, read-only query access, private-library access, filesystem path access, network access, report/export generation, background job execution, and rule-pack evaluator integration.
>    - Do not resolve exact sandbox technology, process isolation, permissions syntax, or storage paths in this deliverable.
>
> 7. Map requirements to verification.
>    - For every normative requirement, define a future verification path: schema validation, unit tests, provenance checks, protected-content gates, diagnostics envelope tests, rule-sandbox tests, or architecture review.
>    - Use invented, public-safe fixtures only.
>
> 8. Perform cross-document consistency review.
>    - Confirm Datasheet attributes appear in Specification requirements where appropriate.
>    - Confirm Specification requirements have Guidance rationale and Procedure verification hooks.
>    - Replace unsupported detail with `TBD` or `ASSUMPTION`.
>    - Add or update the Guidance conflict table if sources disagree.
>
> 9. Stop and escalate when needed.
>    - Stop if protected standards/proprietary data is needed, if permission/sandbox design would require resolving a human security decision, if a requirement would blur user-rule status with professional approval, or if source conflicts affect safety or code-relevant values.
>

### CLM-023 — Verification

> ##### Verification
>
> Completion checks for this deliverable:
>
> - `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist and retain the default required sections.
> - Scope remains DEL-02-04 and PKG-02; no files outside the allowed write targets are modified.
> - Requirements cite accessible source slices or are labeled ASSUMPTION/TBD.
> - No protected standards/code text, copied tables, copied code formulas, proprietary commercial data, or private project/rule/library data is introduced.
> - No statement claims certification, sealing, approval, endorsement, or automatic engineering code compliance (PRD §21.2).
> - Terms are used consistently: plugin/adapter, extension point, schema-first envelope, unit-aware validation, provenance, diagnostics/result envelope, public/private data boundary, and report controls.
> - Remaining implementation details are visible as TBD, especially public API transport, import/export format list, exact permission taxonomy, exact sandbox mechanism, exact dependency versions, CI thresholds, and concrete schema layout.
>

### CLM-024 — Records

> ##### Records
>
> - `Datasheet.md` records deliverable identity, attributes, conditions, construction assumptions, and source references.
> - `Specification.md` records normative requirements, standards/baselines, verification mapping, and documentation obligations.
> - `Guidance.md` records rationale, principles, considerations, trade-offs, invented safe examples, and conflict status.
> - `Procedure.md` records the production and maintenance procedure for the contract kit.
> - Future manifest/schema review record should identify the approved fields or explicit `TBD` placeholders for plugin identity/version, extension declarations, schema/envelope version, capability declarations, provenance/data-boundary declarations, diagnostics compatibility, and review/status metadata.
> - Future canonicalization/hash verification record should identify the manifest or payload schema version, canonicalization basis, sample invented fixtures, expected hashes, and unresolved `TBD` items when JSON hashing is required.
> - Future sandbox/privacy verification record should document rule-pack-facing sandbox tests, denied arbitrary-code/filesystem/network paths, capability-grant review evidence, and any approved exception for telemetry-facing or private-data exposure.
> - `_run_records/TASK_RUN_*.md` records the TASK execution, sources consulted, outputs, QA checks, and warnings.
> - `_STATUS.md` records the safe lifecycle update when the current state permits it.

- **VER-001** — Validate the contracts and review source parity, extension-point and permission boundaries, no-bypass behavior, structured diagnostics, private-data controls, and absence of protected content or authority expansion.

## Governing Values and Decisions — Axiology

### CLM-025 — Source preamble

> ---
> doc_id: DEL-02-04-GUIDANCE
> doc_kind: deliverable.guidance
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-02-04
> package_id: PKG-02
> ---
>

### CLM-026 — Guidance: Plugin and Extension Domain Contracts

> #### Guidance: Plugin and Extension Domain Contracts
>

### CLM-027 — Purpose

> ##### Purpose
>
> DEL-02-04 exists so OpenPipeStress can be extensible without turning plugins or adapters into an escape route around the domain model, unit system, provenance model, diagnostics, report controls, or public/private data boundary. This is the PKG-02 domain/API contract for that boundary, not a plugin-loader implementation. SourcePath: `_CONTEXT.md`; SectionRef: Description. SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: SOW-038.
>

### CLM-028 — Principles

> ##### Principles
>
> 1. Deny-bypass is the governing principle. Plugins/adapters may extend behavior only through governed interfaces that preserve validation, diagnostics, envelopes, and report controls. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-02 and AB-00-07.
> 2. Schema-first boundaries should be preferred for public manifests and interchange surfaces because SCA-001 selected JSON Schema 2020-12 and schema-first command/query/job/result envelopes. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline.
> 3. Unit and provenance checks are not optional plugin features. They are part of the core data contract and must be preserved across imports, exports, reports, and rule-facing hooks. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-UNIT-1 and OPS-K-DATA-3.
> 4. Private or protected data must remain under user control. Public artifacts can provide schemas, empty templates, invented examples, and import mechanisms, but must not embed protected standards or proprietary data. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 2. Public repository may contain and 3. Public repository must not contain.
> 5. Diagnostics should be machine-readable as well as user-visible. Plugin/adapter failures need enough source, affected-object, remediation, and provenance detail to support audit and review. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-06.
> 6. Software may show computed states and user-rule outcomes; acceptance and professional judgment remain with the responsible engineer. SourcePath: `docs/TYPES.md`; SectionRef: 4. Analysis-status vocabulary. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AUTH-1.
>

### CLM-029 — Considerations

> ##### Considerations
>
> - Keep DEL-02-04 at the domain/API level. Public API transport, concrete import/export formats, concrete plugin loader behavior, and packaging/distribution are later implementation decisions unless separately authorized. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: OI-004 and DEC-012.
> - Treat plugin capability grants as explicit and minimal. ASSUMPTION: a future permission model will need to distinguish read-only project access, governed project mutation, private-library access, filesystem path access, network access, report/export generation, background job execution, and rule-pack evaluator integration. Exact permission names and enforcement technology are TBD.
> - Treat candidate extension point families as rationale-backed assumptions, not an approved registry. Import/export adapter candidates trace to the public API/import-export scope; report/output extension candidates trace to report-control obligations; validation-hook candidates trace to layered testing and protected-content/provenance gates; rule-pack integration hook candidates trace to sandboxed, unit-aware rule evaluation. Exact registry names, registry owner, and approval record are TBD.
> - Prefer private-by-default behavior for user rule packs, component libraries, material data, project files, and calculation results. SourcePath: `docs/PRD.md`; SectionRef: 18.1 Local-First Design and 18.2 Telemetry.
> - When a plugin imports external values, the contract should require provenance and redistribution metadata before those values can become public or reusable. Missing provenance should produce `PROVENANCE_WARNING` or a blocking result when required for reliance. SourcePath: `docs/SPEC.md`; SectionRef: 7. GUI requirements warning classes; SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 4. Required provenance fields.
> - If a plugin touches rule-pack evaluation, it should not run arbitrary executable code. Keep that work declarative, sandboxed, deterministic, unit-aware, and aligned with the rule-pack engine scope. SourcePath: `docs/PRD.md`; SectionRef: 12.3 Rule-Pack Evaluator.
> - If a plugin hashes a JSON manifest or JSON data payload for reproducibility, use the accepted canonical JSON/JCS-compatible hash basis. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline.
>

### CLM-030 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | Extensibility vs. governance | Prefer stricter validation and explicit diagnostics over broad plugin authority. SOW-038 requires extensibility without bypass. |
> | Schema-first contracts vs. idiomatic in-process APIs | Keep the public/domain contract schema-first where possible; implementation bindings can be generated or wrapped later. Exact code-generation tooling is TBD. |
> | Sandboxing strength vs. plugin ergonomics | Start from least privilege and grant capabilities only when evidence, user intent, and review controls are clear. Exact sandbox mechanism is TBD. |
> | Import/export flexibility vs. data boundary protection | Support adapters, but require provenance, redistribution status, unit checks, and protected-content gates before data leaves private control. |
> | Report customization vs. protected content risk | Allow private report templates under user responsibility, but public templates and exports must not reproduce protected standards or proprietary formulas. |
> | Plugin diagnostics volume vs. auditability | Favor structured diagnostics with source and remediation over terse failures; diagnostics are part of reproducibility and review. |
>

### CLM-031 — Open Decision Table (for human ruling)

> ##### Open Decision Table (for human ruling)
>
> | Decision ID | Decision needed | Current bounded position | Source basis | Human ruling |
> |---|---|---|---|---|
> | OD-02-04-001 | Public API transport for plugin/adapter interfaces | TBD; keep schema-first envelope baseline only | `execution/_Decomposition/SOFTWARE_DECOMP.md` 8.2 Resolved architecture baseline; DEC-012 | TBD |
> | OD-02-04-002 | Approved extension-point registry names and owner | TBD; candidate families remain ASSUMPTION | `docs/_Registers/ScopeLedger.csv` row SOW-038; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-07 | TBD |
> | OD-02-04-003 | Permission taxonomy, sandbox mechanism, and approval path | TBD; deny-by-default and no-bypass controls remain governing | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-07 and DEC-012; `docs/CONTRACT.md` OPS-K-RULE-2 | TBD |
> | OD-02-04-004 | Supported import/export formats | TBD; do not imply concrete format support | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-004 and DEC-012 | TBD |
> | OD-02-04-005 | Whether plugins can ever receive telemetry-facing or private engineering data | TBD; default is no telemetry/private-data exposure | `docs/CONTRACT.md` OPS-K-PRIV-1 and OPS-K-PRIV-2; `docs/PRD.md` 18.2 Telemetry | TBD |
>

### CLM-032 — Examples

> ##### Examples
>
> The examples below are invented contract scenarios only. They do not define concrete schema syntax or provide engineering values.
>
> | Scenario | Expected contract behavior |
> |---|---|
> | Invented CSV import adapter reads user-supplied component metadata from a private path. | Adapter declares private data access, maps fields to canonical schema, requires units for dimensional values, records source/provenance, and emits diagnostics for missing source/license fields. |
> | Invented result exporter writes a machine-readable result package. | Exporter receives data through a governed query/result envelope, preserves units, includes warnings/assumptions, and does not claim code compliance or professional approval (PRD §21.2). |
> | Invented rule-pack helper provides a custom interpolation table using user-owned values. | Helper remains sandboxed and deterministic, records redistribution status, rejects missing units, and does not access filesystem or network except through a granted, reviewed capability. |
> | Invented report extension tries to include copied standards text. | Protected-content gate blocks the public artifact, classifies the issue for quarantine/human review, and emits an `IP_BOUNDARY_WARNING`. |
>

### CLM-033 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> No technical source conflict was identified for plugin-contract content during Pass 3. One metadata traceability conflict remains outside this run's write scope.
>
> | Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
> |---|---|---|---|---|---|---|
> | CONF-02-04-001 | Decomposition revision wording differs between current context and local reference index. | `_CONTEXT.md` - Decomposition Reference identifies accepted revision 0.7/current_basis. | `_REFERENCES.md` - Decomposition and Registers describes the decomposition as accepted v0.2. | Datasheet References; future metadata/reference cleanup. | Treat `_CONTEXT.md` and current `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 as the active basis for this run; route `_REFERENCES.md` cleanup to a later authorized metadata/reference pass. | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-038 OBJ-009 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
