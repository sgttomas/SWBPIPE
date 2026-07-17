---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-10-03
package_id: PKG-10
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@4d153302c3c4cd42578936db160c2bac1270225a
project_scope_refs: [SOW-031, SOW-049]
package_objective_refs: [OBJ-009]
---

# Scope of Work — DEL-10-03

## Purpose and Objective Traceability

This Scope of Work defines `DEL-10-03` in service of project scope [SOW-031, SOW-049] and package objectives [OBJ-009].

- **OUT-001** — A local FEA handoff data-contract for explicit governed export of geometry, properties, loads, constraints, units, provenance, mappings, and result re-association is produced.

## Deliverable Definition — Ontology

### CLM-001 — Source preamble

> ---
> doc_id: DEL-10-03-DATASHEET
> doc_kind: deliverable.datasheet
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-10-03
> package_id: PKG-10
> ---
>

### CLM-002 — Datasheet: Local FEA Handoff Data Contract

> #### Datasheet: Local FEA Handoff Data Contract
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-003 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-10-03-DECL-002`.
>

### CLM-004 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-10-03 |
> | Package ID | PKG-10 |
> | Package | Build, Packaging, API, and Interoperability |
> | Type | API_CONTRACT |
> | Scope items | SOW-031, SOW-049 |
> | Objective | OBJ-009 |
> | Anticipated artifacts | `local FEA handoff schema`; `docs/local-analysis notes` |
> | Current artifact form | Deliverable-local contract kit; repository-level schemas and docs are outside this setup write scope |
> | Lifecycle target for setup | `SEMANTIC_READY` after setup gates pass |
> | External FEA implementation | Out of scope |
> | Final external format selection | TBD |
>

### CLM-005 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Boundary purpose | Define an export package for selected local shell/solid FEA handoff and advisory labels for when handoff is recommended. | `_CONTEXT.md` Description; `docs/_Registers/Deliverables.csv` row DEL-10-03 |
> | Normal global method | OpenPipeStress primary analysis remains a 3D centerline/frame model. | `INIT.md` boundaries; `docs/DIRECTIVE.md` section 3; `docs/CONTRACT.md` OPS-K-MECH-1 |
> | Handoff role | Local FEA handoff is a specialized interoperability path for local-detail problems, not the normal global analysis method. | `docs/_Registers/ScopeLedger.csv` rows SOW-031 and SOW-049 |
> | Contract baseline | Schema-first command/query/job result envelopes; JSON Schema 2020-12 public schema/interchange basis; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
> | Adapter boundary | Handoff exports are governed adapter payloads and cannot bypass unit checks, provenance, diagnostics, privacy, protected-content screening, report controls, or professional-boundary language. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02, AB-00-06, AB-00-07; `docs/SPEC.md` section 1 |
> | Criteria authority | Handoff criteria labels are guidance only and do not certify that global beam analysis is sufficient or that local FEA is code-compliant. | `_CONTEXT.md` Context Budget QA; `docs/_Registers/ScopeLedger.csv` row SOW-049; `docs/CONTRACT.md` OPS-K-AUTH-1 |
> | Protected-data posture | The public contract must not embed protected standards text, protected tables, copied code formulas, allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data, or private project/rule data. | `docs/CONTRACT.md` OPS-K-IP-1/2/3; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6 |
>

### CLM-006 — Conditions

> ##### Conditions
>
> | Condition | Required handling |
> |---|---|
> | Selected local region lacks stable model identity | Block or mark `TBD`; do not emit an unverifiable handoff package. |
> | Units or dimensional basis are missing | Emit `SOLVE_BLOCKING` or export-blocking diagnostics where applicable; do not silently default. |
> | Boundary/load/result provenance is missing | Emit provenance diagnostics and keep the missing evidence visible. |
> | User-supplied code or proprietary values are needed | Reference only user/private inputs with provenance and redistribution status; do not bundle those values in public examples. |
> | Protected or proprietary content is suspected | Stop public-path export/contribution, mark suspected content, and route to human review. |
> | Handoff criteria label is shown to a user | Present as advisory guidance that requires competent human review. |
> | External solver/tool behavior is requested | Mark `TBD` or out of scope unless a later approved adapter deliverable supplies the behavior. |
>

### CLM-007 — Construction

> ##### Construction
>
> The local FEA handoff contract is a conceptual export package. It should be described in schema-ready terms without choosing a final external tool format.
>
> | Contract surface | Minimum concept slots | Setup status |
> |---|---|---|
> | Handoff package identity | Package ID, source project/model IDs, originating OpenPipeStress version, package schema version, creation timestamp, privacy/export posture. | Concept defined; final schema field names TBD. |
> | Selected local-detail scope | Region selection reference, included components/elements/nodes, cut boundary description, selection rationale, advisory criteria label. | Concept defined; selection UX and storage details TBD. |
> | Global model context | Units, coordinate frame, model hash, relevant materials/sections/components by reference, solve status, load case/result basis, diagnostics summary. | Concept defined; exact result reference structure TBD. |
> | Boundary condition transfer | Cut locations, coordinate frames, displacement/force/moment/resultant context, load-case association, sign convention notes, unit metadata. | Concept defined; no external solver mapping chosen. |
> | Local geometry/idealization notes | User-supplied local-detail assumptions, meshing/idealization notes, omitted features, open questions, source/provenance records. | Guidance only; no shell/solid mesh implementation. |
> | Handoff guidance label | Advisory labels such as `GLOBAL_CENTERLINE_EXPECTED_SUFFICIENT`, `LOCAL_DETAIL_REVIEW_RECOMMENDED`, `LOCAL_FEA_HANDOFF_RECOMMENDED`, and `HUMAN_REVIEW_REQUIRED`. | Proposed vocabulary for future review, not certification. |
> | Diagnostics and limitations | Diagnostic code/class/severity/source/affected object/message/remediation/provenance, warnings, assumptions, limitations, unresolved `TBD`s. | Governed by AB-00-06. |
> | Reproducibility manifest | Canonical JSON payload hash where applicable, model/result hashes, source pointers, rule-pack references without exposing private values. | Concept defined; canonicalization edge cases TBD. |
>

### CLM-008 — References

> ##### References
>
> - `_CONTEXT.md` for sealed deliverable identity, acceptance/risk notes, write scope, and architecture-basis injection.
> - `docs/CONTRACT.md` for applicable invariants: OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-PRIV, OPS-K-AUTH-1, OPS-K-AGENT-1..4, and professional-responsibility boundaries.
> - `docs/DIRECTIVE.md` sections 1, 3, 4, and 5 for centerline-first intent, local FEA handoff as specialized path, protected-data limits, and stop rules.
> - `docs/TYPES.md` sections 3, 4, 5, 6, 7, and 8 for deliverable type, analysis status, epistemic labels, local FEA handoff vocabulary, provenance labels, and domain object registry.
> - `docs/SPEC.md` sections 1, 3, 4, 7, 8, 9, 10, and 11 for architecture layering, domain objects, solver/result/report boundaries, diagnostics, verification, and acceptance semantics.
> - `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6 for public/private data boundaries, provenance, quarantine, and private user data.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for SOW-031, SOW-049, OBJ-009, PKG-10, AB-00-02/03/04/06/07/08, and OI-004.

## Completion and Reliance Basis — Epistemology

### CLM-009 — Source preamble

> ---
> doc_id: DEL-10-03-SPECIFICATION
> doc_kind: deliverable.specification
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-10-03
> package_id: PKG-10
> ---
>

### CLM-010 — Specification: Local FEA Handoff Data Contract

> #### Specification: Local FEA Handoff Data Contract
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-011 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-10-03-DECL-001`.
>

### CLM-012 — Scope

> ##### Scope
>
> DEL-10-03 defines a local FEA handoff data contract for selected local-detail problems that may require external shell/solid analysis. It covers the conceptual export package, advisory handoff criteria labels, diagnostics, units, provenance, privacy, protected-content controls, reproducibility metadata, and professional-boundary language. SourcePath: `_CONTEXT.md`; SectionRef: Description, Scope Coverage, Objective Support, Architecture Basis Injection.
>
> This deliverable does not implement shell/solid FEA, generate meshes, choose an external solver, choose final external exchange formats, encode proprietary tool behavior, provide code-specific local-stress acceptance rules, declare global beam analysis sufficient for a project, or certify local FEA results. SourcePath: `_CONTEXT.md`; SectionRef: Context Envelope and Acceptance/risk notes. SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: rows SOW-031 and SOW-049.
>

### CLM-013 — Requirements

> ##### Requirements
>
> | ID | Requirement | Evidence |
> |---|---|---|
> | DEL-10-03-REQ-01 | The local FEA handoff contract shall treat global centerline/frame analysis as the normal global method and local shell/solid FEA as an optional specialized handoff path. | SourcePath: `docs/DIRECTIVE.md`; SectionRef: section 3 principle 3 and section 4.2. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-MECH-1. |
> | DEL-10-03-REQ-02 | The contract shall define schema-ready concept slots for handoff package identity, selected local region, global model context, boundary condition transfer, local-detail notes, diagnostics, limitations, provenance, privacy, and reproducibility metadata. | SourcePath: `_CONTEXT.md`; SectionRef: Anticipated Artifacts. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-03, AB-00-04, AB-00-06, AB-00-07. |
> | DEL-10-03-REQ-03 | Handoff payload concepts shall remain unit-aware and dimensionally explicit for coordinates, forces, moments, displacements, rotations, stresses, load/result references, and any user-supplied local-detail assumptions. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-UNIT-1. SourcePath: `docs/SPEC.md`; SectionRef: sections 1, 3, 4, and 8. |
> | DEL-10-03-REQ-04 | Handoff payload concepts shall carry source/provenance, redistribution status, privacy classification, and review status where engineering reliance or public contribution risk may be affected. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-2, OPS-K-DATA-3, OPS-K-PRIV. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: sections 4-6. |
> | DEL-10-03-REQ-05 | The contract shall surface missing solve-required values, missing rule-check values, missing provenance, unresolved local-detail assumptions, and unresolved format/tool decisions as explicit diagnostics or `TBD`s, never as silent defaults. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-2 and OPS-K-AGENT-1. SourcePath: `docs/SPEC.md`; SectionRef: section 7 warning classes. |
> | DEL-10-03-REQ-06 | Handoff guidance labels shall be advisory only and shall not assert code compliance, professional approval, certification, sealing, endorsement, or project-specific acceptability (PRD §21.2). | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AUTH-1. SourcePath: `docs/TYPES.md`; SectionRef: sections 4 and 6. |
> | DEL-10-03-REQ-07 | The contract shall prohibit public artifacts from embedding protected standards text, protected tables, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data without rights, private project data, or private rule-pack values. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1 and OPS-K-IP-3. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: sections 2-3. |
> | DEL-10-03-REQ-08 | The handoff contract shall align with the schema-first command/query/job/result-envelope boundary and shall not allow an export adapter or plugin to bypass domain validation, unit checks, diagnostics, provenance checks, privacy controls, protected-content screening, or report controls. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-02, AB-00-03, AB-00-06, AB-00-07. SourcePath: `docs/architecture/plugin_boundary.md`; SectionRef: Boundary Rules and No-Bypass Constraints. |
> | DEL-10-03-REQ-09 | Final public API transport, concrete external FEA format list, concrete adapter implementation, external solver invocation semantics, and schema file placement shall remain `TBD` unless later approved in a separate implementation deliverable. | SourcePath: `_CONTEXT.md`; SectionRef: Still TBD. SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: OI-004 and section 8.2. |
> | DEL-10-03-REQ-10 | The contract shall include advisory criteria labels that distinguish expected global-centerline sufficiency from recommended local-detail review or local FEA handoff, while keeping the final engineering decision with a competent human. | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: rows SOW-031 and SOW-049. SourcePath: `docs/DIRECTIVE.md`; SectionRef: sections 2.2 and 3. |
> | DEL-10-03-REQ-11 | The contract shall require diagnostics/result envelopes to carry code, class, severity, source, affected object, message, remediation, and provenance for handoff-blocking, assumption, provenance, privacy, and IP-boundary findings. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-06. SourcePath: `docs/SPEC.md`; SectionRef: section 7. |
> | DEL-10-03-REQ-12 | Verification for this setup deliverable shall include document review, semantic/lensing completeness checks, dependency register validation, protected-content/professional-boundary scans, and preservation of unresolved implementation decisions as `TBD`. | SourcePath: `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`; SectionRef: sections 4-5. SourcePath: `docs/VALIDATION_STRATEGY.md`; SectionRef: sections 1, 2, and 4. |
>

### CLM-014 — Standards

> ##### Standards
>
> No protected engineering code, standards clauses, tables, formulas, examples, material allowables, SIF/flexibility content, protected dimensional data, proprietary external-tool behavior, or proprietary vendor data are incorporated into this deliverable.
>
> Applicable internal baselines:
>
> - `docs/CONTRACT.md` invariant catalog, especially OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-PRIV, OPS-K-AUTH-1, OPS-K-AGENT-1..4, and OPS-K-MECH-1/2.
> - `docs/DIRECTIVE.md` for centerline global model first, local FEA as specialized handoff, stop rules, and professional-responsibility boundaries.
> - `docs/TYPES.md` for API contract type, analysis-status vocabulary, epistemic labels, provenance labels, centerline model, and local FEA handoff vocabulary.
> - `docs/SPEC.md` for layer responsibilities, domain objects, solver/result/report boundaries, diagnostics classes, verification strategy, and Type 2 acceptance semantics.
> - `docs/IP_AND_DATA_BOUNDARY.md` for public/private data, provenance, quarantine, private user data, and report boundary policy.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for SOW-031, SOW-049, OBJ-009, PKG-10, AB-00-02/03/04/06/07/08, and OI-004.
>
> External implementation baseline:
>
> - JSON Schema 2020-12 is the accepted public schema/interchange baseline by SCA-001. This setup deliverable does not create a repository-level schema file and does not choose an external FEA exchange format.
>

### CLM-015 — Verification

> ##### Verification
>
> | Requirement IDs | Verification approach |
> |---|---|
> | REQ-01, REQ-06, REQ-10 | Boundary review confirms local FEA is framed as optional guidance and that human review remains required. |
> | REQ-02, REQ-03, REQ-04, REQ-11 | Contract review confirms package identity, selected local region, global context, boundary-condition concepts, units, provenance, diagnostics, privacy, and reproducibility slots are represented at concept level. |
> | REQ-05, REQ-09 | TBD review confirms missing values, external format/tool decisions, schema placement, and implementation details are visible rather than silently resolved. |
> | REQ-07 | Protected-content review confirms no protected standards data, proprietary external-tool behavior, private project data, or private rule-pack values are introduced. |
> | REQ-08 | Adapter/API review confirms no-bypass constraints are carried into the handoff boundary. |
> | REQ-12 | Setup evidence review confirms four docs, semantic/lensing artifacts, dependency artifacts, run records, and status updates exist and pass local gates. |
> | Future implementation gate | Later source-code or schema work must add tests for handoff export schema validation, unit/provenance/privacy checks, protected-content screening, diagnostics, reproducibility metadata, and no-certification language before release use. |
>

### CLM-016 — Interim Setup Acceptance Criteria

> ###### Interim Setup Acceptance Criteria
>
> | Gate | Pass condition |
> |---|---|
> | Scope gate | Only DEL-10-03 deliverable-local files are edited. |
> | Document gate | Datasheet, Specification, Guidance, and Procedure exist and include default sections. |
> | Semantic gate | `_SEMANTIC.md` exists, contains matrices A, B, C, F, D, K, G, X, T, E, and passes the local result-cell audit. |
> | Lensing gate | `_SEMANTIC_LENSING.md` exists, covers every cell in matrices A, B, C, F, D, X, E, and preserves lens-not-authority separation. |
> | Dependency gate | `Dependencies.csv` exists, validates against v3.1 required columns, uses canonical write-form enums, and `_DEPENDENCIES.md` summarizes active rows. |
> | Boundary gate | No external FEA implementation, mesh generation, external tool behavior, final format choice, source code, package manifest, protected data, private engineering data, or certification/compliance claim is introduced. |
>

### CLM-017 — Documentation

> ##### Documentation
>
> Required setup artifacts for DEL-10-03:
>
> - Deliverable-local local FEA handoff contract kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`.
> - Semantic setup artifacts: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and `_run_records/*`.
> - Dependency artifacts: `Dependencies.csv` and `_DEPENDENCIES.md`.
>
> Concept inventory for later schema work:
>
> | Concept slot | Current status | Evidence |
> |---|---|---|
> | Handoff package schema version | Required concept; exact field name/layout TBD. | `_CONTEXT.md` Anticipated Artifacts; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04. |
> | Source model/result references | Required concept; exact object reference layout TBD. | `docs/SPEC.md` sections 3-5 and 8. |
> | Selected local region | Required concept; selection UX and persistence details TBD. | `docs/_Registers/ScopeLedger.csv` row SOW-031. |
> | Boundary-condition transfer | Required concept; no external solver mapping chosen. | `docs/_Registers/ScopeLedger.csv` row SOW-031. |
> | Advisory criteria label | Required concept; guidance only and human-reviewed. | `docs/_Registers/ScopeLedger.csv` row SOW-049. |
> | Units/provenance/privacy/diagnostics | Required concepts; exact schema fields TBD. | `docs/CONTRACT.md` OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-PRIV; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06. |
> | Reproducibility/hash basis | Required where payload hashes are used; canonicalization details TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and section 8.2. |

- **AC-001** — The contract preserves solver-neutral schema and manifest boundaries, stable identifiers, coordinate and unit conventions, provenance and protected/private-data controls, completeness and diagnostic checks, deterministic hashes, local-tool separation, declared omissions and unsupported features, and visible solver/export-format TBDs without claiming external-solver validation or professional approval.

## Production and Verification Method — Praxeology

### CLM-018 — Source preamble

> ---
> doc_id: DEL-10-03-PROCEDURE
> doc_kind: deliverable.procedure
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-10-03
> package_id: PKG-10
> ---
>

### CLM-019 — Procedure: Local FEA Handoff Data Contract

> #### Procedure: Local FEA Handoff Data Contract
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-020 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-10-03-DECL-004`.
>

### CLM-021 — Purpose

> ##### Purpose
>
> Use this procedure to produce, review, and later refine the DEL-10-03 local FEA handoff data contract without implementing external FEA, choosing final external formats, embedding proprietary tool behavior, bypassing adapter/API governance, or making compliance/certification claims.
>

### CLM-022 — Prerequisites

> ##### Prerequisites
>
> - The sealed DEL-10-03 context is available in `_CONTEXT.md`.
> - Governing documents and registers named in `_REFERENCES.md` have been read.
> - Applicable architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08 are treated as dispatch constraints, not copied as full PKG-00 authority.
> - Current setup write scope is restricted to this deliverable folder.
> - External FEA implementation, final exchange formats, concrete adapter code, shell/solid meshing, solver-specific boundary-condition mapping, and schema file placement remain TBD.
>

### CLM-023 — Steps

> ##### Steps
>
> 1. Confirm identity and scope:
>    - Verify the deliverable is `DEL-10-03` under `PKG-10`.
>    - Confirm scope items SOW-031 and SOW-049 and objective OBJ-009.
>    - Confirm no edit target is outside this deliverable folder.
> 2. Establish the boundary:
>    - Record that global centerline/frame analysis is the normal global method.
>    - Record that local shell/solid FEA is an optional specialized handoff path.
>    - Record that the contract is guidance/data-contract work only.
> 3. Inventory handoff package concept slots:
>    - Package identity and schema version.
>    - Source project/model/result references and hashes.
>    - Selected local region and selection rationale.
>    - Unit system and coordinate frames.
>    - Load-case/result basis and diagnostics.
>    - Boundary-condition transfer concepts.
>    - Local-detail assumptions and omitted-feature notes.
>    - Provenance, redistribution status, privacy classification, and review status.
>    - Limitations, advisory criteria label, and human-review notice.
> 4. Define advisory criteria labels:
>    - Use labels to communicate screening posture.
>    - Keep labels advisory and human-reviewed.
>    - Do not convert labels into compliance, certification, approval, or automatic acceptability status.
> 5. Preserve no-bypass controls:
>    - Require future handoff export behavior to pass through schema, unit, provenance, privacy, protected-content, diagnostics, hash, and report-boundary checks.
>    - Do not authorize direct solver, storage, report, private-library, or external-tool bypass paths.
> 6. Preserve unresolved decisions:
>    - Mark final schema filename/location, exchange format list, external FEA tool behavior, adapter implementation, boundary-condition mapping, and validation fixtures as `TBD`.
>    - Do not choose defaults without human/project authority evidence.
> 7. Check protected-data and authority boundaries:
>    - Confirm no protected standards text, tables, copied formulas, proprietary vendor data, private project data, private rule-pack values, or commercial software examples are embedded.
>    - Confirm no wording claims certification, sealing, approval, endorsement, or code compliance (PRD §21.2).
> 8. Produce setup artifacts:
>    - Four-document kit.
>    - `_SEMANTIC.md`.
>    - `_SEMANTIC_LENSING.md`.
>    - `Dependencies.csv`.
>    - `_DEPENDENCIES.md`.
>    - `_run_records/*`.
>    - `_STATUS.md` with `SEMANTIC_READY` only after setup gates pass.
>

### CLM-024 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Write-scope check | Only files in the DEL-10-03 folder changed. |
> | Four-document check | Datasheet, Specification, Guidance, and Procedure exist with default sections. |
> | Handoff boundary check | Local FEA is optional/specialized; global centerline/frame analysis remains the normal global method. |
> | Implementation boundary check | No external FEA implementation, mesh generation, external solver behavior, final exchange format, source code, package manifest, or repository-level schema file is introduced. |
> | Protected-data check | No protected standards content, proprietary commercial data, private project/rule/component/material data, or copied commercial software examples are introduced. |
> | Professional-boundary check | No certification, sealing, approval, endorsement, or code-compliance claim appears (PRD §21.2). |
> | Semantic check | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and preserve lens-not-authority separation. |
> | Dependency check | `Dependencies.csv` validates against v3.1 schema and canonical enum values. |
>

### CLM-025 — Records

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

### CLM-026 — Completion Condition

> ##### Completion Condition
>
> The setup sequence is complete when the required setup artifacts exist, dependency validation passes, semantic/lensing artifacts are internally consistent, unresolved decisions remain visible as `TBD`, protected-data/professional-boundary checks are clean, and `_STATUS.md` records `SEMANTIC_READY` without any `ISSUED` transition.

- **VER-001** — Validate the contract and review source parity, geometry/property/load/constraint and mapping coverage, coordinate/unit/provenance requirements, manifest/hash and diagnostic evidence, local FEA boundary and re-association behavior, retained tool/format TBDs, and prohibited validation or approval claims.

## Governing Values and Decisions — Axiology

### CLM-027 — Source preamble

> ---
> doc_id: DEL-10-03-GUIDANCE
> doc_kind: deliverable.guidance
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-10-03
> package_id: PKG-10
> ---
>

### CLM-028 — Guidance: Local FEA Handoff Data Contract

> #### Guidance: Local FEA Handoff Data Contract
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-029 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-10-03-DECL-003`.
>

### CLM-030 — Purpose

> ##### Purpose
>
> This deliverable gives future adapter, API, report, validation, and documentation work a governed data-contract boundary for local shell/solid FEA handoff. It exists to make selected local-detail handoff possible without changing the normal global analysis method, bypassing unit/provenance/privacy controls, importing protected data, or overstating what software can decide.
>

### CLM-031 — Principles

> ##### Principles
>
> - Treat local FEA handoff as an optional interoperability path. The OpenPipeStress global model remains a 3D centerline/frame model unless later scope explicitly changes that boundary.
> - Keep the handoff package descriptive and reproducible. It should identify the source model, selected local region, units, load/result basis, diagnostics, assumptions, provenance, and hashes.
> - Keep criteria labels advisory. They can help a user decide where further local review may be warranted. Acceptance, professional judgment, and any certification, sealing, or code-compliance determination remain with the responsible engineer and project authority.
> - Prefer schema-ready concepts and `TBD` placeholders over invented external solver fields, mesh settings, or proprietary exchange behavior.
> - Require every dimensional or engineering value in the handoff package to carry units and source/provenance where reliance may be affected.
> - Preserve public/private and protected-content boundaries. User-private values may be referenced as private inputs; protected or proprietary data must not become public project content.
> - Route future export behavior through governed adapter/API envelopes so diagnostics, privacy controls, provenance checks, and report controls remain active.
>

### CLM-032 — Considerations

> ##### Considerations
>
> The local FEA handoff boundary has several audiences:
>
> | Audience | What it needs | Boundary implication |
> |---|---|---|
> | Global piping analyst | A clear advisory reason why local-detail review is being considered. | Provide guidance labels and limitations, not certification. |
> | External FEA specialist | Enough context to reconstruct a local problem responsibly. | Include selected region, units, coordinate frame, load/result basis, boundary-condition concepts, diagnostics, and assumptions. |
> | Adapter/API implementer | Stable schema-ready contract surfaces. | Keep final tool formats and export mechanics TBD until a later implementation brief. |
> | Reviewer/auditor | Evidence that handoff did not bypass governance. | Preserve provenance, privacy classification, hashes, diagnostics, protected-content checks, and professional-boundary notices. |
>
> Advisory label examples for future schema review:
>
> | Label | Intended meaning | Boundary note |
> |---|---|---|
> | `GLOBAL_CENTERLINE_EXPECTED_SUFFICIENT` | Current evidence suggests the centerline model is the appropriate analysis level for the stated purpose. | Guidance only; not certification. |
> | `LOCAL_DETAIL_REVIEW_RECOMMENDED` | A local feature, assumption, or uncertainty should be reviewed before relying on global results alone. | Requires human interpretation. |
> | `LOCAL_FEA_HANDOFF_RECOMMENDED` | A selected local-detail problem may need shell/solid treatment outside the global solver. | Does not implement or validate external FEA. |
> | `HUMAN_REVIEW_REQUIRED` | Professional review is required before project reliance. | Always true for professional use. |
>
> General factors that may support local-detail review include local geometry or restraint behavior that is not represented by the centerline model, local load introduction, attachment/nozzle/equipment interface concerns, localized discontinuity behavior, or unresolved assumptions. These are screening prompts, not automatic rules.
>
> General factors that may support staying with global centerline analysis include a question limited to system flexibility, displacements, reactions, member forces/moments, and open-mechanics stress recovery within the model scope, with complete solve-required data, unit consistency, and visible assumptions. This is still subject to competent review.
>

### CLM-033 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | More local-detail information vs. private-data exposure | Export only what is needed for the selected handoff and preserve privacy/provenance classifications. |
> | Early format selection vs. interoperability flexibility | Keep final external formats TBD until adapter work can evaluate unit, provenance, privacy, and protected-content gates. |
> | Advisory criteria vs. false certainty | Use labels and rationale, not pass/fail certification. |
> | Boundary condition detail vs. solver-specific behavior | Describe source model results, cut context, units, and sign conventions without choosing external solver mapping. |
> | Public examples vs. protected data risk | Use invented examples only in future documentation and validation. |
>

### CLM-034 — Examples

> ##### Examples
>
> No engineering numeric example, protected standards example, proprietary vendor example, copied commercial-software example, or tool-specific instruction is included here.
>
> Acceptable invented examples for later work:
>
> - A handoff package manifest that references a source model hash, selected region ID, unit system, local coordinate frame, load-case/result basis, diagnostics summary, and privacy classification.
> - A local-review note that records an advisory label, human-readable rationale, open assumptions, and required reviewer follow-up.
> - A result-envelope concept showing that a handoff export was blocked because units, provenance, or selected-region identity were incomplete.
>

### CLM-035 — Human-Ruling Queue

> ##### Human-Ruling Queue
>
> | Topic | Current disposition |
> |---|---|
> | Final local FEA handoff schema filename and repository location | TBD; outside this setup write scope |
> | Final external FEA format list | TBD |
> | Exact adapter implementation and external solver invocation behavior | TBD |
> | Exact handoff package field names and JSON Schema layout | TBD |
> | Exact advisory criteria label vocabulary | PROPOSAL in this deliverable; requires later human/API review |
> | Mapping from global model results to external shell/solid boundary conditions | TBD; no solver-specific behavior selected |
> | Validation fixtures for handoff export | TBD; future invented/public examples only |
>

### CLM-036 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict detected during setup. Remaining issues are explicit TBDs or advisory vocabulary proposals rather than contradictory source claims. | NA | NA | NA | Keep TBDs visible until human/project authority records decisions. | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-031 SOW-049 OBJ-009 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
