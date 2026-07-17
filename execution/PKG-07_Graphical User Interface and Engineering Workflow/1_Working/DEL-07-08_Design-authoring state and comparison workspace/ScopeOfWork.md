---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-08
package_id: PKG-07
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-076]
package_objective_refs: [OBJ-015, OBJ-016]
---

# Scope of Work — DEL-07-08

## Purpose and Objective Traceability

This Scope of Work defines `DEL-07-08` in service of project scope [SOW-076] and package objectives [OBJ-015, OBJ-016].

- **OUT-001** — A design-authoring and comparison workspace contract covering design knowledge and constraints, operation/diff review, state/run browsing, comparison tables and overlays, upstream-contract use, and application-service mutation boundaries is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-07-08 Design-authoring state and comparison workspace

> #### Datasheet: DEL-07-08 Design-authoring state and comparison workspace
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-08-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | DEL-07-08 | `_CONTEXT.md` / Context header |
> | Name | Design-authoring state and comparison workspace | `_CONTEXT.md` / Context header |
> | Package | PKG-07 Graphical User Interface and Engineering Workflow | `_CONTEXT.md` / Package Reference |
> | Type | UX_UI_SLICE | `_CONTEXT.md` / Type |
> | Scope item | SOW-076 | `_CONTEXT.md` / Scope Coverage |
> | Objectives | OBJ-015, OBJ-016 | `_CONTEXT.md` / Objective Support |
> | Context envelope | L | `_CONTEXT.md` / Context Envelope |
> | Context risk | WATCH; confirm scope and split if it expands | `_CONTEXT.md` / Context Budget QA |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Current value |
> |---|---|
> | Intended GUI surfaces | Design knowledge panel; operation diff review; state/run browser; comparison overlays. Source: `_CONTEXT.md` / Anticipated Artifacts. |
> | Scope statement | The GUI shall support design-authoring and comparison workflows, including design knowledge panels, constraint/warning panels, state/run browsers, comparison tables, and graphical comparison overlays. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-076. |
> | GUI-facing stack basis | Rust core/application services, Tauri 2 desktop shell where GUI-facing, TypeScript/React/Vite GUI where GUI-facing, and Three.js viewport where 3D viewport-facing. Source: `_CONTEXT.md` / Architecture Basis Injection. |
> | Exact dependency versions | TBD. Source: `_CONTEXT.md` / Architecture Basis Injection / Still TBD. |
> | Mutation boundary | GUI mutations route through application-service commands; durable project state remains separated from transient session, viewport, selection, and job-progress state. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-05. |
> | Diagnostic boundary | Diagnostics and result envelopes carry code, class, severity, source, affected object, message, remediation, and provenance; outputs must not claim certification or compliance. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-06. |
> | Data boundary | Missing solve-required or rule-check-required values are explicit findings and not silent defaults. Source: `docs/CONTRACT.md` / Invariant index. |
> | Professional boundary | Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Source: `docs/CONTRACT.md` / Invariant index. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Status |
> |---|---|
> | Upstream dependency mirror | `Dependencies.csv` contains 21 rows; all are ACTIVE approved DAG-006 mirror rows. Source: `Dependencies.csv` and `_DEPENDENCIES.md` / Generated Dependency Register. |
> | Architecture basis dependencies | AB-00-01, AB-00-02, AB-00-03, AB-00-05, AB-00-06, AB-00-07, and AB-00-08 apply as dispatchable constraints. Source: `_CONTEXT.md` / Architecture Basis Injection. |
> | GUI predecessor dependencies | DEL-07-01, DEL-07-02, DEL-07-04, and DEL-07-05 are declared upstream GUI foundations. Source: `Dependencies.csv` rows DAG-002-E0840 through DAG-002-E0843. |
> | Design/constraint dependencies | DEL-13-01, DEL-13-03, and DEL-13-04 are declared upstream design-knowledge, constraint-validation, and transformation inputs. Source: `Dependencies.csv` rows DAG-002-E0844 through DAG-002-E0846. |
> | State/comparison dependencies | DEL-14-01, DEL-14-03, DEL-14-04, and DEL-14-05 are declared upstream state and comparison inputs. Source: `Dependencies.csv` rows DAG-002-E0847 through DAG-002-E0850. |
> | Operation workflow dependencies | DEL-16-01, DEL-16-02, and DEL-16-03 are declared upstream operation schema, diff preview, and audit inputs. Source: `Dependencies.csv` rows DAG-002-E0851 through DAG-002-E0853. |
> | Frontend implementation evidence | TBD; this folder contains setup documents only at this workflow stage. Source: local folder inspection. |
>

### CLM-006 — Construction

> ##### Construction
>
> This deliverable is a GUI workflow slice. The expected construction surface is a workspace that composes design knowledge, warnings, operation/diff review, state/run browsing, comparison tables, and graphical comparison overlays without owning the backend schemas or engines that produce those records.
>
> The workspace must consume upstream contracts rather than invent placeholder semantics. The approved DAG-006 edge review states that DEL-07-08 should consume existing GUI foundations plus initial design, transform, comparison, and operation contracts. Source: `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` / DAG2-RD-015.
>
> Implementation details not supported by local evidence remain TBD, including concrete component hierarchy, state-management library, route names, data-fetching library, visual encoding rules, keyboard shortcuts, and Playwright coverage thresholds.
>

### CLM-007 — References

> ##### References
>
> | Reference | Use in this datasheet |
> |---|---|
> | `_CONTEXT.md` | Deliverable identity, scope, objectives, package, architecture basis, context budget. |
> | `_REFERENCES.md` | Local reference index. |
> | `_DEPENDENCIES.md` and `Dependencies.csv` | Approved local DAG-002 mirror and dependency evidence surface. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | SOW-076, OBJ-015, OBJ-016, PKG-07 deliverable row, architecture-basis constraints. |
> | `docs/CONTRACT.md` | Invariants for data boundary, no silent defaults, professional boundary, and agent drafting limits. |
> | `docs/SPEC.md` | GUI warning classes, application-service boundaries, persistence/hash boundaries, result/export/report boundary constraints. |
> | `docs/TYPES.md` | UX_UI_SLICE type and relevant vocabulary for viewport sessions, diagnostics, states, reports, and professional boundary. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and protected-content boundary. |

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-07-08 Design-authoring state and comparison workspace

> #### Specification: DEL-07-08 Design-authoring state and comparison workspace
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-08-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> DEL-07-08 covers the GUI workspace slice for design-authoring and comparison workflows. The scope includes design knowledge panels, constraint/warning panels, state/run browsers, comparison tables, operation/diff review, and graphical comparison overlays. Source: `_CONTEXT.md` / Description and Anticipated Artifacts; `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-076.
>
> This deliverable does not own the backend contracts for design knowledge, constraint validation, physical-to-analytical transformation, immutable model states, analysis runs, comparison engines, structured model operations, operation validation, or operation audit trails. Those are declared upstream dependencies in `Dependencies.csv` and are consumed by this GUI workspace.
>
> This deliverable must not silently supply missing code data, protected engineering values, owner requirements, rule-pack values, or professional acceptance states. Source: `docs/CONTRACT.md` / OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-AUTH-1; `docs/IP_AND_DATA_BOUNDARY.md` / Public repository must not contain.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | REQ-07-08-001 | The workspace shall support design-authoring and comparison workflows, including design knowledge panels, constraint/warning panels, state/run browsers, comparison tables, and graphical comparison overlays. | `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-076 |
> | REQ-07-08-002 | The workspace shall consume existing GUI, design, transform, comparison, and operation contracts rather than inventing placeholder semantics. | `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` / DAG2-RD-015 |
> | REQ-07-08-003 | GUI-originated mutations shall route through application-service command intents and shall not directly mutate persisted project payloads. | `docs/SPEC.md` / GUI requirements; `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-05 |
> | REQ-07-08-004 | The operation/diff review surface shall preserve the distinction between proposed, validated, accepted, and audited model operations; exact UI state labels are TBD pending upstream operation contracts. | `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-069, SOW-070; `Dependencies.csv` rows DAG-002-E0851 through DAG-002-E0853 |
> | REQ-07-08-005 | The state/run browser shall treat immutable model states, analysis runs, and deterministic comparisons as first-class review records; exact data contract shape is inherited from upstream PKG-14 deliverables. | `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-071 through SOW-073; `Dependencies.csv` rows DAG-002-E0847 through DAG-002-E0850 |
> | REQ-07-08-006 | Constraint and warning presentation shall preserve structured diagnostic fields and the project warning classes where available. | `docs/SPEC.md` / GUI requirements; `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-06 |
> | REQ-07-08-007 | Comparison views shall be diagnostic and review-oriented; they shall not present deterministic comparisons as automatic external validation, professional approval, or code compliance (PRD §21.2). | `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-073; `docs/CONTRACT.md` / OPS-K-AUTH-1 |
> | REQ-07-08-008 | Missing solve-required, rule-check-required, provenance, assumption, nonlinear, or IP-boundary information shall remain visible as findings or warnings and shall not be silently defaulted by the GUI. | `docs/SPEC.md` / GUI requirements; `docs/CONTRACT.md` / OPS-K-DATA-2 |
> | REQ-07-08-009 | The GUI-facing implementation shall follow the accepted architecture basis: Tauri 2 where desktop-shell-facing, TypeScript/React/Vite where GUI-facing, and Three.js where 3D viewport-facing. Exact package versions remain TBD. | `_CONTEXT.md` / Architecture Basis Injection |
> | REQ-07-08-010 | Verification shall include GUI-appropriate automated checks where implementation exists, including layered GUI tests and rendering/interaction checks; exact test harness and acceptance thresholds are TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-08; `docs/SPEC.md` / GUI requirements |
>

### CLM-012 — Standards

> ##### Standards
>
> | Standard or governing basis | Applicability |
> |---|---|
> | `docs/CONTRACT.md` | Binding project invariants for data boundary, no silent defaults, professional boundary, agent limits, and review status. |
> | `docs/SPEC.md` | Technical architecture and GUI diagnostic requirements. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content and private-data boundary for public artifacts. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 | Accepted decomposition basis for SOW-076, OBJ-015, OBJ-016, PKG-07, and architecture-basis constraints. |
> | PRD v0.2 references | Mentioned by decomposition for SOW-076, SOW-069 through SOW-073, but the PRD source text was not locally read in this workflow. Clause-level requirements from PRD remain TBD unless source text is supplied. |
>

### CLM-013 — Verification

> ##### Verification
>
> | Requirement | Verification approach |
> |---|---|
> | REQ-07-08-001 | Review implemented workspace surfaces against SOW-076 surface list; automated GUI smoke tests are TBD until implementation exists. |
> | REQ-07-08-002 | Inspect data adapters and UI fixtures for dependency-backed contract use; flag placeholder semantics as TBD or implementation blocker. |
> | REQ-07-08-003 | Test that user edits produce application-service command intents rather than direct project payload mutation. Exact command API is TBD pending upstream contracts. |
> | REQ-07-08-004 | Test operation review states against upstream operation validation and audit records once those contracts are available. |
> | REQ-07-08-005 | Test state/run browser queries and comparison navigation against upstream PKG-14 records once those contracts are available. |
> | REQ-07-08-006 | Verify warning class display for `SOLVE_BLOCKING`, `RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, `NONLINEAR_WARNING`, and `IP_BOUNDARY_WARNING` where such diagnostics are supplied. |
> | REQ-07-08-007 | Review UI copy, status labels, and exported comparison affordances for professional-boundary violations. |
> | REQ-07-08-008 | Inject missing-data and provenance-warning fixtures; confirm the GUI surfaces findings without inventing values. |
> | REQ-07-08-009 | Inspect package/runtime configuration once implementation files exist. |
> | REQ-07-08-010 | Run GUI unit/component tests and Playwright/rendering checks once a frontend scaffold exists. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required deliverable-local setup artifacts:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv` and `_DEPENDENCIES.md` preserved as approved DAG-002 mirror/evidence artifacts
>
> Implementation-stage artifacts anticipated by `_CONTEXT.md`:
>
> - GUI design knowledge panel
> - operation diff review
> - state/run browser
> - comparison overlays
>
> Exact implementation file paths and product code artifacts are TBD because this workflow does not create product code.

- **AC-001** — The contract preserves the declared authoring and comparison surfaces, upstream ownership of backend schemas and engines, proposed/validated/accepted/audited state distinctions, diagnostic visibility, no silent defaults, and no external-validation or professional-approval claim.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-07-08 Design-authoring state and comparison workspace

> #### Procedure: DEL-07-08 Design-authoring state and comparison workspace
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-016 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-08-DECL-004`.
>

### CLM-017 — Purpose

> ##### Purpose
>
> This procedure describes how to produce and verify the DEL-07-08 GUI workspace deliverable in a later implementation pass. It is based on local decomposition, governing references, and the approved DAG-006 dependency mirror. It does not create product code.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Evidence |
> |---|---|
> | Accepted deliverable scope and objectives | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` / DEL-07-08 row, SOW-076, OBJ-015, OBJ-016 |
> | Approved dependency basis | `_DEPENDENCIES.md` and `Dependencies.csv`; `execution/_DAG/DAG-006/APPROVAL_RECORD.md` / Approval Conditions |
> | Architecture-basis constraints | `_CONTEXT.md` / Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-01 through AB-00-08 as applicable |
> | GUI warning and diagnostic vocabulary | `docs/SPEC.md` / GUI requirements; `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-06 |
> | Data and professional boundaries | `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md` |
> | Upstream contracts for implementation | DEL-07-01, DEL-07-02, DEL-07-04, DEL-07-05, DEL-13-01, DEL-13-03, DEL-13-04, DEL-14-01, DEL-14-03, DEL-14-04, DEL-14-05, DEL-16-01, DEL-16-02, and DEL-16-03 as listed in `Dependencies.csv`. |
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Confirm the implementation brief is still bounded to SOW-076 and DEL-07-08. If the work expands beyond design knowledge panels, warnings, operation/diff review, state/run browsing, comparison tables, and graphical overlays, escalate for scope split before implementation.
> 2. Inventory available upstream contracts from the active dependency set. Do not substitute invented schemas, fake operation states, or placeholder comparison semantics for missing upstream contracts; mark unsupported items as TBD.
> 3. Define the workspace information architecture around the SOW-076 surfaces:
>    - design knowledge panel;
>    - constraint/warning panel;
>    - operation/diff review;
>    - state/run browser;
>    - comparison table;
>    - graphical comparison overlay.
> 4. Route any GUI-originated model changes through application-service command intents or upstream structured model operation contracts. Do not directly mutate durable project payloads from the GUI layer.
> 5. Preserve diagnostic structure in warning UI. Where diagnostics are supplied, keep code, class, severity, source, affected object, message, remediation, and provenance available to the user.
> 6. Implement state/run browsing only against upstream model-state and analysis-run records. Exact query shape, routing, pagination, and storage access are TBD pending implementation context.
> 7. Implement comparison tables and overlays only against upstream comparison records, mapping/tolerance contracts, or deterministic diff output. Exact visual encodings and tolerance display rules are TBD.
> 8. Review all labels, empty states, and status text for professional-boundary risk. The GUI must not claim automatic certification, sealing, approval, authentication, or code compliance (PRD §21.2).
> 9. Add tests appropriate to the implemented surface. Expected classes include component/unit tests, interaction tests, and Playwright/rendering checks where a frontend scaffold exists. Exact commands and thresholds are TBD.
> 10. Record remaining TBDs, assumptions, missing upstream contracts, and warning/provenance limitations in the deliverable review notes for human evaluation.
>

### CLM-020 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Scope check | Implemented surfaces map to SOW-076 and do not absorb unrelated package work. |
> | Dependency check | GUI semantics are backed by approved upstream contracts or explicitly marked TBD. |
> | Mutation-boundary check | Durable project state changes pass through application-service commands or structured operation contracts. |
> | Warning check | Missing data, provenance, assumptions, nonlinear uncertainty, and IP-boundary states remain visible. |
> | Comparison-boundary check | Comparison output is review/audit support only and does not imply professional validation. |
> | Public/private data check | No protected standards text, owner standards, private rule-pack payloads, proprietary values, or real secrets are embedded in public artifacts. |
> | Test check | GUI tests run where implementation exists; missing harness details are recorded as TBD. |
>

### CLM-021 — Records

> ##### Records
>
> Maintain or produce the following records as implementation evidence:
>
> - source-grounded requirements or acceptance notes for SOW-076;
> - UI test output, including interaction and rendering checks where applicable;
> - dependency notes showing which upstream contracts were consumed;
> - screenshots or visual review artifacts if created by the implementation brief;
> - unresolved TBD and ASSUMPTION list;
> - professional-boundary and protected-content review notes.
>
> No implementation records exist in this setup workflow.

- **VER-001** — Validate the contract and review source parity, declared workspace surfaces, upstream dependency boundaries, command-intent mutation routing, operation and state distinctions, diagnostics and missing-data visibility, comparison reliance boundaries, and every retained TBD or governed residual.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-07-08 Design-authoring state and comparison workspace

> #### Guidance: DEL-07-08 Design-authoring state and comparison workspace
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-023 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-08-DECL-003`.
>

### CLM-024 — Purpose

> ##### Purpose
>
> This deliverable adds a GUI workspace for design iteration and comparison review. It is the GUI-facing integration point for design knowledge, warnings, model operations, immutable states, analysis runs, and comparison records. Source: `_CONTEXT.md` / Description; `execution/_Decomposition/SOFTWARE_DECOMP.md` / DEL-07-08 row.
>

### CLM-025 — Principles

> ##### Principles
>
> 1. Consume upstream contracts, do not invent them. DEL-07-08 depends on GUI foundations plus design, transform, state/comparison, and operation contracts. The workspace should represent those contracts faithfully and mark unsupported details as TBD. Source: `Dependencies.csv`; `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` / DAG2-RD-015.
> 2. Keep authoring controlled. GUI-originated edits should become application-service command intents and controlled operations, not direct writes to durable project state. Source: `docs/SPEC.md` / GUI requirements; `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-05.
> 3. Keep missing information visible. Missing solve-required data, rule-check data, provenance, assumptions, nonlinear uncertainty, and IP-boundary risks are findings or warnings, not values to hide or default. Source: `docs/SPEC.md` / GUI requirements; `docs/CONTRACT.md` / OPS-K-DATA-2.
> 4. Keep comparisons review-oriented. State/run comparisons support design review and audit; acceptance and professional judgment remain with the responsible engineer. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-073; `docs/CONTRACT.md` / OPS-K-AUTH-1.
> 5. Preserve public/private and protected-content boundaries. Design knowledge and project metadata may include user-owned or private information; public artifacts must not copy protected standards, owner requirements, private rule-pack payloads, or proprietary data. Source: `docs/IP_AND_DATA_BOUNDARY.md` / Public repository must not contain.
>

### CLM-026 — Considerations

> ##### Considerations
>
> The workspace is context-envelope L with WATCH risk. Its scope includes several surface families: design knowledge, warnings, operations, state/run browsing, tables, and graphical overlays. If implementation expands beyond one bounded GUI workflow, the decomposition guidance says to confirm scope and split if needed. Source: `_CONTEXT.md` / Context Budget QA.
>
> The GUI stack basis is established at an architectural level, but exact dependency versions, state library, package structure, transport API, and final app workflow behavior remain TBD. Source: `_CONTEXT.md` / Architecture Basis Injection; `docs/SPEC.md` / GUI requirements.
>
> The warning panel should avoid flattening diagnostic meaning into plain prose. The local sources identify warning classes and diagnostic fields; display choices should preserve code, class, severity, source, affected object, message, remediation, and provenance when supplied. Source: `docs/SPEC.md` / GUI requirements; `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-06.
>
> The design knowledge panel should not bundle owner standards, protected code data, proprietary catalog values, or invented engineering defaults. When values are unavailable, the UI should preserve TBD, missing, or provenance-warning state. Source: `docs/IP_AND_DATA_BOUNDARY.md`; `docs/CONTRACT.md` / OPS-K-DATA-1 and OPS-K-DATA-2.
>

### CLM-027 — Trade-offs

> ##### Trade-offs
>
> | Decision pressure | Conservative guidance |
> |---|---|
> | Rich workspace versus bounded scope | Build around the SOW-076 surfaces and upstream contracts first; defer unrelated editors or report rendering to their owning deliverables. |
> | Convenience editing versus auditability | Prefer structured operations and diff review over direct mutation because OBJ-015 requires controlled, reviewable, auditable model operations. |
> | Visual overlays versus traceability | Overlays should be backed by state/comparison/diagnostic records where available; unsupported visual semantics remain TBD. |
> | Smooth comparison UX versus professional boundary | Use neutral review wording. Do not label a comparison as approved, validated, certified, sealed, or code compliant unless a separate human-owned record is explicitly in scope (PRD §21.2). |
> | Public examples versus realistic data | Use invented or cleared data only. Real owner/project/code/vendor data belongs in private user-controlled paths unless contribution review clears it. |
>

### CLM-028 — Examples

> ##### Examples
>
> No product UI screenshots, component implementations, or locally accessible PRD examples were available in this workflow. Example layout, copy, fixtures, and visual encoding remain TBD until an implementation brief supplies or creates product evidence.
>

### CLM-029 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None observed in production-document drafting | No cross-document content conflict was found during four-doc Pass 2. | TBD | TBD | TBD | N/A | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-076 OBJ-015 OBJ-016 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
