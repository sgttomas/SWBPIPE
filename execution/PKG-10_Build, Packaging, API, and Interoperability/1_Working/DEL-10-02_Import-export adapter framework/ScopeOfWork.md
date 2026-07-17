---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-10-02
package_id: PKG-10
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@4d153302c3c4cd42578936db160c2bac1270225a
project_scope_refs: [SOW-030]
package_objective_refs: [OBJ-009]
---

# Scope of Work — DEL-10-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-10-02` in service of project scope [SOW-030] and package objectives [OBJ-009].

- **OUT-001** — An import-export adapter-framework contract for governed external-data translation and round-trip evidence is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-10-02 Import/export adapter framework

> #### Datasheet: DEL-10-02 Import/export adapter framework
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-10-02-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-10-02 |
> | Name | Import/export adapter framework |
> | Package ID | PKG-10 |
> | Package | Build, Packaging, API, and Interoperability |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope item | SOW-030 |
> | Objective | OBJ-009 |
> | Context envelope | L |
> | Current production mode | Format-neutral declaration validation plus a deny-only declaration-to-runtime gate; no loader or execution model |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value |
> |---|---|
> | Deliverable purpose | Define and enforce the current format-neutral adapter admission boundary without selecting concrete formats or runtime execution. |
> | Anticipated artifacts | Adapter interfaces; sample invented adapter. |
> | Runtime seam | `gate_adapter_runtime_dispatch` validates the declaration, rejects/quarantines bypass attempts, and never dispatches; accepted declarations remain blocked pending a separately selected runtime. |
> | Architecture baseline | Schema-first command/query/job result envelopes; JSON Schema 2020-12 contracts; canonical JSON/JCS-compatible hashing where JSON payloads are hashed. |
> | Adapter boundary | Adapters may translate external data but cannot bypass units, provenance, redistribution, diagnostics, private/public data controls, validation, sandboxing, envelopes, or report controls. |
> | External format list | TBD; concrete protected or proprietary external formats are not bundled defaults in this deliverable. |
> | Public API transport | TBD; public transport protocol remains a later human/product decision. |
> | Security evidence boundary | Selected-seam negative evidence only; no whole-product security/privacy/legal sufficiency claim. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Source |
> |---|---|
> | All import/export operations must be unit-aware and deterministically report unit conversions. | docs/PRD.md section 6.6; docs/CONTRACT.md OPS-K-UNIT-1 |
> | Imported data must flag missing required fields, missing or inconsistent units, missing provenance, unclear redistribution status, protected-table risk, and user-defined reasonableness concerns. | docs/PRD.md section 13.5 |
> | Private rule packs, component libraries, material data, project files, and calculation results must not be transmitted or exported unexpectedly. | docs/PRD.md sections 18.2 and 18.3; docs/CONTRACT.md OPS-K-PRIV-1 |
> | Adapter outputs for nontrivial operations use diagnostics/result envelopes and must not make certification or compliance claims. | execution/_Decomposition/SOFTWARE_DECOMP.md AB-00-06; docs/CONTRACT.md OPS-K-AUTH-1 |
> | Adapter interfaces remain code-neutral and do not bundle protected standards data or proprietary tool behavior. | INIT.md; docs/DIRECTIVE.md; docs/IP_AND_DATA_BOUNDARY.md |
>

### CLM-006 — Construction

> ##### Construction
>
> The future framework should be described as a shell around import/export providers, not as concrete bundled adapters. Required conceptual pieces are:
>
> - adapter identity and capability metadata;
> - import and export request envelopes;
> - unit-validation hooks before data enters domain workflows;
> - provenance and redistribution-status validation;
> - diagnostics and warning emission using project warning classes;
> - private/public data boundary checks before writing exported payloads;
> - rule-pack and report hooks that preserve sandboxing and report controls;
> - manifest/hash hooks for reproducible imported and exported artifacts.
>

### CLM-007 — References

> ##### References
>
> - INIT.md
> - AGENTS.md
> - docs/DIRECTIVE.md
> - docs/CONTRACT.md
> - docs/TYPES.md
> - docs/SPEC.md
> - docs/IP_AND_DATA_BOUNDARY.md
> - docs/PRD.md
> - execution/_Decomposition/SOFTWARE_DECOMP.md
> - docs/_Registers/Deliverables.csv
> - docs/_Registers/ScopeLedger.csv
> - docs/_Registers/ContextBudgetQA.csv
> - execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-03_Application service command-query-job model/Specification.md
> - execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-06_Diagnostics, warning, and result-envelope contract/Specification.md
> - execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-07_API boundary and adapter contract map/Specification.md
> - execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/Specification.md

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-10-02 Import/export adapter framework

> #### Specification: DEL-10-02 Import/export adapter framework
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-10-02-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This specification covers the implemented format-neutral adapter declaration validator and the selected declaration-to-runtime admission gate under `DEL-10-02`. The gate enforces adapter-interface obligations, validation hooks, and governance boundaries without creating a runtime loader or selecting an execution model.
>
> Concrete import/export formats remain `TBD`. Protected standards data, proprietary commercial formats, copied code tables, proprietary examples, and private user libraries are not bundled defaults.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source | Verification |
> |---|---|---|---|
> | REQ-10-02-01 | The adapter framework shall operate through schema-first command/query/job result envelopes and shall not bypass the public API/plugin boundary. | execution/_Decomposition/SOFTWARE_DECOMP.md AB-00-03 and AB-00-07 | Review adapter-interface design before implementation. |
> | REQ-10-02-02 | Every import path shall validate units before accepting external data into domain workflows. | docs/PRD.md section 6.6; docs/CONTRACT.md OPS-K-UNIT-1 | Unit-validation test plan in future implementation. |
> | REQ-10-02-03 | Every import path shall capture source/provenance, license or redistribution status, and review disposition for data records that may be contributed publicly or reused. | docs/CONTRACT.md OPS-K-IP-2; docs/IP_AND_DATA_BOUNDARY.md section 4 | Provenance-field checks in future implementation. |
> | REQ-10-02-04 | Imported records with missing required fields, inconsistent units, missing provenance, unclear redistribution status, suspected protected origin, or out-of-range user checks shall produce diagnostics rather than silent defaults. | docs/PRD.md section 13.5; docs/CONTRACT.md OPS-K-DATA-2 | Diagnostic coverage review. |
> | REQ-10-02-05 | Export paths shall check private/public data boundary status before writing payloads that may leave the local project context. | docs/PRD.md section 18.3; docs/CONTRACT.md OPS-K-PRIV-1 | Export-boundary checklist in future implementation. |
> | REQ-10-02-06 | Adapter diagnostics shall include code, class, severity, source, affected object, message, remediation, and provenance when applicable. | execution/_Decomposition/SOFTWARE_DECOMP.md AB-00-06 | Result-envelope schema review. |
> | REQ-10-02-07 | Adapter hooks for rule packs shall preserve rule-pack sandboxing, unit awareness, versioning, checksums, and public/private status. | docs/CONTRACT.md OPS-K-RULE-1 through OPS-K-RULE-3; execution/_Decomposition/SOFTWARE_DECOMP.md AB-00-07; DEC-074 O7/E5 | Negative runtime-gate tests prove a declaration disabling any selected no-bypass control is rejected before dispatch; even a valid declaration remains blocked because no execution model/plugin runtime is selected. |
> | REQ-10-02-08 | Adapter hooks for reports shall preserve report warnings, assumptions, provenance, limitations, protected-content controls, and professional-responsibility notices. | docs/CONTRACT.md OPS-K-REPORT-1, OPS-K-REPORT-2, OPS-K-AUTH-1 | Report-boundary review. |
> | REQ-10-02-09 | The framework shall not choose protected external formats, proprietary tool behavior, or specific commercial integration defaults without human approval and documented redistribution rights. | docs/IP_AND_DATA_BOUNDARY.md; execution/_Decomposition/SOFTWARE_DECOMP.md OI-004 and DEC-012 | Human decision log required before format selection. |
> | REQ-10-02-10 | The framework shall leave mechanics solve, user-rule check, and professional approval states distinct in all adapter result payloads. | docs/TYPES.md section 4; docs/CONTRACT.md OPS-K-AUTH-1 | Status-field review in future interface artifacts. |
>

### CLM-012 — Standards

> ##### Standards
>
> No external engineering code or proprietary format standard is incorporated by this setup artifact. Applicable project standards are internal governance invariants:
>
> - OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3
> - OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3
> - OPS-K-UNIT-1
> - OPS-K-RULE-1, OPS-K-RULE-2, OPS-K-RULE-3
> - OPS-K-PRIV-1, OPS-K-PRIV-2
> - OPS-K-AUTH-1
> - OPS-K-AGENT-1 through OPS-K-AGENT-4
>

### CLM-013 — Verification

> ##### Verification
>
> Current bounded verification includes:
>
> - schema validation for adapter request/result envelopes;
> - unit conversion and dimensional-consistency tests;
> - provenance and redistribution-status rejection/flagging tests;
> - private-data export-warning tests;
> - protected-content/provenance gates for public examples and fixtures;
> - diagnostics envelope tests for import, export, and failed validation paths;
> - rule-pack hook tests proving adapters cannot execute arbitrary rule code or bypass evaluator sandboxing;
> - report-boundary tests proving exports do not suppress warnings, limitations, or professional-responsibility notices.
> - a selected-seam dispatch gate with no executor/callback path;
> - matrix tests disabling unit, provenance, privacy, protected-content, rule-pack sandbox, checksum, report, and private-transmission controls;
> - quarantine evidence for suspected protected content; and
> - a positive declaration that still returns `BLOCKED_RUNTIME_NOT_SELECTED`.
>
> These checks prove only the selected declaration-to-runtime seam. They do not establish whole-product security/privacy sufficiency, choose the runtime/plugin model, or settle non-JSON/binary partitioning.
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required future artifacts remain:
>
> - adapter interfaces;
> - sample invented adapter using invented, non-code, non-proprietary data only.
>
> This setup run does not create those implementation artifacts.

- **AC-001** — The contract preserves schema-first adapter interfaces, unit/dimensional/provenance and redistribution checks, public/private data separation, protected-content quarantine, diagnostics and loss reporting, deterministic round-trip behavior, format-specific extension boundaries, downstream FEA-handoff compatibility, and unresolved external-format decisions without inventing implementation authority.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-10-02 Import/export adapter framework

> #### Procedure: DEL-10-02 Import/export adapter framework
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-016 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-10-02-DECL-004`.
>

### CLM-017 — Purpose

> ##### Purpose
>
> Define how to maintain the current format-neutral validation and deny-only runtime gate while preserving the sealed scope and governance boundaries recorded for `DEL-10-02`.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> - Confirm the sealed brief authorizes implementation work beyond this setup/document production run.
> - Read `_CONTEXT.md`, `Specification.md`, and the applicable `AB-00-*` architecture basis rows.
> - Confirm the write scope before creating adapter source, tests, sample adapters, package manifests, or repo-level artifacts.
> - Confirm whether the target work is framework-only or a concrete external format. Concrete external format selection requires human approval if not already recorded.
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Identify the adapter operation types needed by the authorized implementation slice: import, export, validation, diagnostics, manifest/hash, rule-pack hook, report hook, and private-data boundary check.
> 2. Define adapter metadata fields for identity, capability, supported direction, supported payload class, source/provenance requirements, redistribution posture, and privacy behavior.
> 3. Route nontrivial adapter operations through schema-first command/query/job result envelopes.
> 4. Add unit-validation hooks before imported values can become domain objects.
> 5. Add provenance and redistribution-status validation before imported data can be accepted for reuse or public contribution.
> 6. Add diagnostics for missing fields, invalid units, missing provenance, unclear redistribution, protected-content suspicion, private-data export risk, and failed reasonableness checks.
> 7. Add rule-pack hooks only through the sandboxed, unit-aware rule-pack boundary; do not allow adapters to execute arbitrary rule code.
> 8. Add report/export hooks that preserve warnings, assumptions, limitations, provenance, and professional-responsibility notices.
> 9. Keep specific external formats, commercial tool behavior, public API transport, and package/container details as `TBD` unless a human ruling is cited.
> 10. Use invented data only for public sample adapters or fixtures.
> 11. Route every current declaration-to-runtime request through `gate_adapter_runtime_dispatch`; do not add an executor/callback or treat declaration acceptance as dispatch authority.
> 12. Preserve `adapter_execution_model`, `plugin_runtime`, permission grants, concrete formats, and redaction workflow as `TBD` until separately selected.
>

### CLM-020 — Verification

> ##### Verification
>
> - Verify no protected standards text, copied tables, proprietary examples, or private project data are introduced.
> - Verify missing required data and missing provenance produce diagnostics rather than defaults.
> - Verify unit-bearing imported/exported values are dimensionally checked.
> - Verify private-boundary checks run before shared exports.
> - Verify result envelopes do not claim certification, approval, sealing, or automatic code compliance (PRD §21.2).
> - Verify tests and examples use invented or otherwise redistributable data with provenance.
> - Verify each selected no-bypass control fails closed before dispatch, protected-suspected content quarantines, and a valid declaration remains blocked with `ADAPTER_RUNTIME_NOT_SELECTED`.
>

### CLM-021 — Records

> ##### Records
>
> Future implementation work should record:
>
> - interface files and schema contracts created;
> - adapter validation tests and diagnostics tests;
> - provenance/protected-content gate results;
> - external format decisions and human approval references when applicable;
> - open `TBD` decisions that remain after implementation.

- **VER-001** — Validate the contract and review source parity, adapter lifecycle and format-extension coverage, unit/provenance/private-data controls, loss and diagnostic visibility, round-trip and downstream-handoff evidence, retained format TBDs, and no-bypass or professional-authority limits.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-10-02 Import/export adapter framework

> #### Guidance: DEL-10-02 Import/export adapter framework
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-023 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-10-02-DECL-003`.
>

### CLM-024 — Purpose

> ##### Purpose
>
> This deliverable provides a format-neutral adapter declaration validator and a deny-only runtime admission gate so later implementation cannot treat declaration acceptance as runtime authority. The framework is an extension surface, not a route around domain validation, unit safety, provenance, rule-pack sandboxing, reporting controls, or professional-responsibility limits.
>

### CLM-025 — Principles

> ##### Principles
>
> - Treat adapters as translators at the edge of the application service boundary.
> - Require schema-first envelopes for nontrivial import/export operations.
> - Validate units and dimensional meaning before external data becomes domain data.
> - Preserve source/provenance, redistribution status, and private/public data markings.
> - Emit diagnostics for missing fields, missing provenance, unclear redistribution, suspected protected content, unit inconsistencies, and private-data export risk.
> - Keep concrete format support `TBD` until a human decision records the external format, license posture, redistribution status, and test obligations.
> - Use invented data only for public samples.
> - Treat `ACCEPTED_FORMAT_NEUTRAL_DECLARATION` as declaration evidence only. Runtime dispatch remains blocked until a separately governed execution/plugin model is selected.
>

### CLM-026 — Considerations

> ##### Considerations
>
> An adapter may be technically able to parse many files, but parsing does not establish redistribution rights, engineering suitability, code compliance, or professional acceptance. The framework should therefore separate:
>
> - syntactic parse success;
> - schema and unit validation;
> - provenance and redistribution review;
> - mechanics-readiness;
> - rule-check-readiness;
> - human-review-needed state.
>
> Adapters that import private material libraries, component records, rule-pack references, or project data should default to local/private handling. Export operations should warn before writing private or protected-suspected values to shared payloads.
>
> The current O7/E5 seam is deliberately narrow: a declaration disabling unit,
> provenance, privacy, protected-content, rule-pack sandbox, checksum, report, or
> private-transmission controls is rejected; suspected protected content is
> quarantined; a valid declaration is still not dispatched. This does not prove
> other adapters, plugins, reports, exports, telemetry, or storage seams.
>

### CLM-027 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | Flexible adapter ecosystem vs. governance control | Favor a narrow framework with mandatory validation hooks; add format-specific adapters later. |
> | User convenience vs. data provenance | Missing source or redistribution status is a finding, not an auto-filled default. |
> | Broad public formats vs. IP safety | Do not bundle protected or proprietary defaults without documented rights and human approval. |
> | Fast export vs. auditability | Exports should carry diagnostics, hashes/manifests where applicable, and warning state. |
>

### CLM-028 — Examples

> ##### Examples
>
> - Acceptable public sample: an invented adapter that imports a small invented component record with invented dimensions, invented source metadata, and permissive redistribution marked as invented/original.
> - Not acceptable public sample: a bundled adapter fixture copied from a standards table, vendor catalog, commercial software example, or private project library without documented redistribution rights.
>

### CLM-029 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No current conflicts detected in setup sources. | NA | NA | NA | NA | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 OBJ-009 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
