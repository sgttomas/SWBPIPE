---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-01
package_id: PKG-07
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-020]
package_objective_refs: [OBJ-006]
---

# Scope of Work — DEL-07-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-07-01` in service of project scope [SOW-020] and package objectives [OBJ-006].

- **OUT-001** — A 3D viewport and centerline-editor contract covering unit-aware nodes, pipe runs, bends, simple component symbols, stable selection identity, command-routed edits, explicit diagnostics, and bounded interaction evidence is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-07-01 3D viewport and centerline editor

> #### Datasheet: DEL-07-01 3D viewport and centerline editor
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-01-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-07-01 |
> | Deliverable name | 3D viewport and centerline editor |
> | Package ID | PKG-07 |
> | Package name | Graphical User Interface and Engineering Workflow |
> | Deliverable type | UX_UI_SLICE |
> | Context envelope | L |
> | Current execution mode | Setup/document production only |
> | Write boundary | This deliverable folder only |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Primary scope item | SOW-020: GUI shall provide a 3D centerline modeler with model tree and piping component visualization | `docs/_Registers/ScopeLedger.csv` row `SOW-020`; `_CONTEXT.md` |
> | Supported objective | OBJ-006: GUI workflow makes model creation, missing data, results, and assumptions visible | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5; `_CONTEXT.md` |
> | Future artifact class | GUI viewport and interaction tests | `docs/_Registers/Deliverables.csv` row `DEL-07-01`; `_CONTEXT.md` |
> | Runtime/UI baseline | Tauri 2 desktop shell, TypeScript/React/Vite GUI, Three.js viewport | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` `DEC-009` |
> | Unresolved implementation choices | Exact dependency versions, component library, state-management library, and platform release matrix remain `TBD` | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` `DEC-012` |
> | State/editing basis | Durable project state is separate from transient viewport, selection, and session state; mutations route through application-service commands | `execution/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-05` |
> | Diagnostic basis | User-facing warnings and result-envelope diagnostics use explicit classes and do not claim code compliance | `execution/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-06`; `docs/SPEC.md` section 7 |
> | Data boundary posture | Code-specific values, protected standards data, manufacturer/private data, and professional approval remain user/private or human-owned | `docs/CONTRACT.md` `OPS-K-DATA-1`, `OPS-K-IP-1`, `OPS-K-AUTH-1`; `docs/IP_AND_DATA_BOUNDARY.md` |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Required handling |
> |---|---|
> | A viewport interaction would require a physical property, component dimension, SIF, flexibility factor, allowable, or code-specific value | Request user-supplied/provenanced data or mark the value `TBD`; do not introduce public defaults |
> | A future component glyph needs protected dimensional tables or vendor catalog data | Stop and route through the protected-data/provenance review path |
> | The viewport can create geometry but required solve data is missing | Surface an explicit diagnostic or missing-data state; do not hide the gap |
> | A rule-pack or code-check status is implied by viewport color, label, or state | Preserve mechanics-solve, user-rule-check, and human-approval separation |
> | A future implementation needs to choose a component or state-management library | Record the decision through the architecture decision path; this setup deliverable does not finalize it |
> | A change would edit GUI source, package manifests, repo-level docs, or tests during this setup session | Stop; those paths are outside this sealed write scope |
>

### CLM-006 — Construction

> ##### Construction
>
> This setup artifact frames a future GUI slice for a 3D centerline viewport and editor. A conforming future implementation should keep these surfaces distinct:
>
> | Surface | Setup expectation |
> |---|---|
> | Viewport rendering | Render pipe centerlines, bend arcs, branch symbols, and simple piping component symbols using the accepted Three.js viewport baseline |
> | Centerline editing | Support creation and editing of nodes and pipe runs through service-command mutations, not direct uncontrolled durable-state edits |
> | Selection and identity | Preserve stable model-entity identity for coordination with the model tree and property inspector |
> | Units and coordinates | Treat coordinates and editable quantities as unit-aware model data; reject incompatible units through domain/service validation |
> | Diagnostics | Carry missing-data, provenance, assumption, nonlinear, and IP-boundary warnings as explicit diagnostics |
> | Undo/redo | Scope undo/redo to reversible model edits and preserve diagnostics/result-envelope integrity |
> | Test evidence | Use GUI-layer tests appropriate to the accepted Vitest/Playwright baseline once implementation is authorized |
>
> This deliverable does not implement product UI in this setup session. It also does not supply engineering component data, rule-pack values, or professional acceptance records.
>

### CLM-007 — References

> ##### References
>
> | Reference | Used for |
> |---|---|
> | `_CONTEXT.md` | Deliverable identity, package, scope, objectives, accepted decomposition revision, and architecture basis injection |
> | `_REFERENCES.md` | Local source inventory |
> | `INIT.md` | Bootstrap boundaries for protected data, missing values, and professional reliance |
> | `AGENTS.md` | Type 2 dispatch and write-scope rule |
> | `docs/CONTRACT.md` | Invariants for protected content, user-supplied code data, units, diagnostics, privacy, IP, and agent limits |
> | `docs/DIRECTIVE.md` | Founding intent, centerline-first model, no silent defaults, and stop rules |
> | `docs/TYPES.md` | UX_UI_SLICE type, analysis-status vocabulary, centerline model, and domain object vocabulary |
> | `docs/SPEC.md` | GUI requirements, warning classes, architecture layers, and acceptance semantics |
> | `docs/PRD.md` | Functional requirements FR-003, FR-013, and GUI requirements section 14 |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data, provenance, and protected-content policy |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-07 and DEL-07-01 decomposition context plus SCA-001 architecture basis |
> | `docs/_Registers/Deliverables.csv` | Deliverable register row |
> | `docs/_Registers/ScopeLedger.csv` | Scope mapping for SOW-020 |

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-07-01 3D viewport and centerline editor

> #### Specification: DEL-07-01 3D viewport and centerline editor
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-01-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable defines setup documentation for the future 3D viewport and centerline editor slice. It is limited to the local `DEL-07-01` working folder and does not create or modify GUI source files, interaction tests, package manifests, dependency manifests, or repo-level documentation.
>
> The deliverable covers:
>
> - the future viewport/editor responsibilities for nodes, pipe runs, bends, and simple component symbols;
> - the accepted runtime/UI and architecture-basis constraints applicable to this GUI slice;
> - data-boundary, unit-safety, diagnostic, privacy, and professional-responsibility constraints;
> - setup evidence required before future implementation work begins.
>
> The deliverable excludes:
>
> - implementation of React, Tauri, Three.js, or state-management code;
> - final selection of unresolved component/state libraries or exact dependency versions;
> - model tree, property inspector, editor panels, solve execution UX, or results viewer behavior owned by adjacent PKG-07 deliverables;
> - any protected standards data, code-specific defaults, proprietary component dimensions, or professional approval workflow.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | Requirement ID | Requirement | Source |
> |---|---|---|
> | DEL-07-01-REQ-01 | The future implementation shall provide a 3D centerline viewport/editor for creating and editing nodes, pipe runs, bends, and simple piping component symbols. | `docs/_Registers/Deliverables.csv` row `DEL-07-01`; `docs/PRD.md` FR-003 and FR-013 |
> | DEL-07-01-REQ-02 | The viewport shall represent the visual categories listed for the 3D viewport where applicable to this slice: pipe centerlines, bend arcs, branch symbols, valves, flanges, reducers, expansion joints, supports, labels, load vectors, deformed shapes, reaction arrows, and stress-ratio color maps. Items beyond initial centerline editing may remain deferred to adjacent result/component slices. | `docs/PRD.md` section 14.2; `_CONTEXT.md` description |
> | DEL-07-01-REQ-03 | The setup deliverable shall preserve the accepted runtime baseline of Tauri 2, TypeScript/React/Vite, and Three.js while keeping exact dependency versions and component/state libraries `TBD`. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` `DEC-009`, `DEC-012` |
> | DEL-07-01-REQ-04 | Viewport model mutations shall route through application-service commands and shall keep durable project state separate from transient viewport, selection, camera, and interaction state. | `execution/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-03`, `AB-00-05` |
> | DEL-07-01-REQ-05 | Coordinates, dimensions, component references, loads, and editable quantities exposed through the viewport shall be unit-aware and validated by domain/service contracts rather than silently coerced. | `docs/CONTRACT.md` `OPS-K-UNIT-1`; `docs/SPEC.md` sections 1 and 3 |
> | DEL-07-01-REQ-06 | The viewport/editor shall not supply protected standards data, code-specific defaults, component dimensional tables, SIF/flexibility values, allowables, or proprietary manufacturer data. Missing values shall remain explicit findings or `TBD`. | `docs/CONTRACT.md` `OPS-K-IP-1`, `OPS-K-DATA-1`, `OPS-K-DATA-2`; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6 |
> | DEL-07-01-REQ-07 | Viewport diagnostics shall preserve warning classes and result-envelope boundaries, including solve-blocking, rule-check-blocking, provenance, assumption, nonlinear, and IP-boundary warnings. | `docs/SPEC.md` section 7; `execution/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-06` |
> | DEL-07-01-REQ-08 | Visual status, labels, colors, and interaction states shall not claim certification, sealing, approval, authentication, or engineering code compliance for reliance. | `docs/CONTRACT.md` `OPS-K-AUTH-1`; `docs/TYPES.md` sections 4 and 6 |
> | DEL-07-01-REQ-09 | Future tests for the implemented slice shall use the accepted GUI test baseline and verify viewport rendering, selection/editing commands, missing-data visibility, and no-protected-data defaults. Exact test harness details beyond the baseline remain `TBD`. | `execution/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-08`; `docs/PRD.md` section 21 |
> | DEL-07-01-REQ-10 | This setup run shall write only deliverable-local setup artifacts and shall not move any artifact to `ISSUED`. | `AGENTS.md` dispatch rule; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 4 |
>

### CLM-012 — Standards

> ##### Standards
>
> No external engineering code, standards-body text, dimensional table, component catalog, allowable table, SIF/flexibility table, or proprietary commercial data is used as an authority for this setup deliverable.
>
> | Standard or governing source | Status |
> |---|---|
> | OpenPipeStress governance and invariant documents | Accessible local governing source |
> | PRD GUI requirements | Accessible local product source |
> | SCA-001 architecture basis | Accepted downstream dispatch basis, not `ISSUED` product implementation |
> | Exact GUI dependency versions | `TBD`; future implementation decision |
> | Component and state-management libraries | `TBD`; future implementation decision |
> | Engineering code values and component catalogs | User/private or legally imported only; not supplied by this deliverable |
>

### CLM-013 — Verification

> ##### Verification
>
> | Verification ID | Verifies | Method |
> |---|---|---|
> | DEL-07-01-VER-01 | Four-document kit exists locally | Run `tools/validation/check_four_documents.sh` on this deliverable folder |
> | DEL-07-01-VER-02 | Semantic setup artifacts exist | Confirm `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and `_run_records/*` exist locally |
> | DEL-07-01-VER-03 | Dependency register is schema-valid | Run `python3 tools/validation/validate_dependencies_schema.py` on local `Dependencies.csv` |
> | DEL-07-01-VER-04 | Dependency enums are canonical | Run `python3 tools/validation/validate_enum.py` for emitted enum values |
> | DEL-07-01-VER-05 | Setup stayed inside the assigned write scope | Check git status and changed paths for this deliverable folder only |
> | DEL-07-01-VER-06 | Protected-data and professional boundaries are visible | Review the setup documents for no-protected-content, no-silent-default, and no-certification language |
> | DEL-07-01-VER-07 | Unresolved implementation choices remain unresolved | Confirm exact dependency versions and component/state libraries are marked `TBD` rather than finalized |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required setup artifacts for this deliverable are:
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
> Future implementation artifacts such as GUI source and interaction tests remain outside this session's write scope.

### CLM-015 — D-41 R5 T5 PDU-008 current GUI boundary

> ##### D-41 R5 T5 PDU-008 current GUI boundary
>
> Current PDU-008 evidence covers structured node and straight-pipe authoring, support handoff, and reference-only component insertion. Dedicated bend authoring and full component-symbol geometry authoring remain absent; this is not full REQ-01 closure.

- **AC-001** — The contract preserves the current implemented viewport slice and named residuals, separates durable model state from transient interaction state, keeps missing or protected engineering data explicit, and invents no component dimensions, code values, defaults, compliance status, or professional approval.

## Production and Verification Method — Praxeology

### CLM-016 — Procedure: DEL-07-01 3D viewport and centerline editor

> #### Procedure: DEL-07-01 3D viewport and centerline editor
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-017 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-01-DECL-004`.
>

### CLM-018 — Purpose

> ##### Purpose
>
> This procedure records how to produce and verify the setup artifacts for `DEL-07-01`, and it gives future implementation work a bounded execution path for the 3D viewport and centerline editor.
>

### CLM-019 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Status |
> |---|---|
> | Sealed deliverable context for `DEL-07-01` | Present in `_CONTEXT.md` and the user brief |
> | Write scope limited to this deliverable folder | Required for this setup session |
> | Governing references available locally | Present through `_REFERENCES.md` and repo docs |
> | SCA-001 architecture basis injected | Present in `_CONTEXT.md` |
> | Protected standards or proprietary data needed | Not needed for setup; must be excluded |
> | GUI source/package/test write authorization | Not present in this setup session |
>

### CLM-020 — Steps

> ##### Steps
>
> 1. Confirm the active deliverable path is `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor`.
> 2. Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, governing docs, decomposition, registers, and the four requested skill files.
> 3. Produce the four-document setup kit:
>    - `Datasheet.md` for identity, attributes, conditions, construction, and references;
>    - `Specification.md` for scope, requirements, standards, verification, and documentation;
>    - `Guidance.md` for rationale, principles, considerations, trade-offs, and examples;
>    - `Procedure.md` for prerequisites, steps, verification, and records.
> 4. Mark unknown or unresolved implementation choices as `TBD`; do not finalize exact GUI dependency versions or component/state libraries.
> 5. Run `semantic-matrix-build` for this deliverable and write `_SEMANTIC.md` with the semantic lens, audit result, and status update if the audit passes.
> 6. Run `lens-register` for this deliverable and write `_SEMANTIC_LENSING.md` without modifying production documents.
> 7. Run `four-documents` with `RUN_PASSES=P3_ONLY` by treating `_SEMANTIC_LENSING.md` as a candidate worklist only; incorporate only source-supported changes.
> 8. Run `dependency-extract` for this deliverable and write `Dependencies.csv` plus `_DEPENDENCIES.md` with anchor and execution dependencies.
> 9. Validate local artifacts:
>    - four-document kit exists;
>    - dependency schema is valid;
>    - dependency enum values are canonical;
>    - semantic artifacts and run records exist;
>    - changed paths remain inside the assigned deliverable folder.
> 10. Leave the deliverable at `SEMANTIC_READY` only if setup gates pass. Do not move anything to `ISSUED`.
>

### CLM-021 — Future Implementation Procedure

> ##### Future Implementation Procedure
>
> When a later implementation brief authorizes GUI source and tests, the future worker should:
>
> 1. Re-read the sealed brief, SCA-001 architecture basis, and these setup artifacts.
> 2. Keep implementation within the future authorized source/test paths, not this setup folder unless instructed.
> 3. Define viewport responsibilities for rendering, camera behavior, selection, snapping/drag handles, entity creation, and command dispatch.
> 4. Keep durable model edits behind application-service commands and validation envelopes.
> 5. Keep transient viewport/session state separate from model persistence.
> 6. Use unit-aware domain data for coordinates and editable quantities.
> 7. Display missing-data and provenance diagnostics without supplying code-specific or proprietary defaults.
> 8. Add GUI tests for rendering, selection, edit commands, undo/redo eligibility, and diagnostic visibility.
> 9. Run the accepted GUI validation gates before review.
>

### CLM-022 — Verification

> ##### Verification
>
> | Check | Command or method | Expected result |
> |---|---|---|
> | Four-document kit | `tools/validation/check_four_documents.sh <DELIVERABLE_PATH>` | PASS |
> | Dependency schema | `python3 tools/validation/validate_dependencies_schema.py <DELIVERABLE_PATH>/Dependencies.csv` | VALID |
> | Enum values | `python3 tools/validation/validate_enum.py <ENUM_NAME> <value>` | VALID for emitted enum values |
> | Semantic/lensing artifacts | File existence check | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` present |
> | Status | Review `_STATUS.md` | `Current State: SEMANTIC_READY` only after all setup gates pass |
> | Scope | `git status --short -- <DELIVERABLE_PATH>` | Changed paths are deliverable-local only |
>

### CLM-023 — Records

> ##### Records
>
> The setup run should leave:
>
> - four production setup documents;
> - `_SEMANTIC.md`;
> - `_SEMANTIC_LENSING.md`;
> - `Dependencies.csv`;
> - refreshed `_DEPENDENCIES.md`;
> - `_run_records/*` entries for each setup step;
> - `_STATUS.md` history showing initialization and semantic readiness.

- **VER-001** — Validate the contract and review source parity, centerline and symbol boundaries, command/service mutation routing, stable identity, units, diagnostics, current implementation declarations and residuals, protected-data controls, and every retained governed TBD.

## Governing Values and Decisions — Axiology

### CLM-024 — Guidance: DEL-07-01 3D viewport and centerline editor

> #### Guidance: DEL-07-01 3D viewport and centerline editor
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-025 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-01-DECL-003`.
>

### CLM-026 — Purpose

> ##### Purpose
>
> This setup deliverable gives future GUI implementation work a bounded viewport/editor contract. The intent is to make centerline model creation visible and reviewable while preserving the project's code-neutral, unit-aware, provenance-aware, and human-review boundaries.
>
> The viewport is a user-facing editor surface, not an authority for engineering values. It can show missing data, provenance gaps, assumptions, and diagnostics, but it must not hide those states or convert them into silent defaults.
>

### CLM-027 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Centerline first | Treat the default global model as a 3D line-element centerline representation. Local shell/solid FEA remains a specialized handoff path. |
> | Command-mediated edits | Future edit gestures should produce application-service commands so durable model state, undo/redo, diagnostics, and persistence remain coherent. |
> | Transient state separation | Camera, hover, selection, drag handles, snapping previews, and job progress are transient GUI state, not durable project data. |
> | Unit-safe editing | Coordinates and editable values need unit-bearing model contracts. Avoid viewport-only numeric assumptions. |
> | Visible uncertainty | Missing solve data, missing rule-check data, weak provenance, user assumptions, and nonlinear uncertainty should be visible diagnostics. |
> | No protected defaults | Component symbols and edit tools must not embed protected code tables, proprietary catalog values, or copied commercial examples. |
> | No professional claim | A rendered model, visual status, or rule-check color is decision support only until a competent human accepts it for project use. |
>

### CLM-028 — Considerations

> ##### Considerations
>
> | Topic | Consideration |
> |---|---|
> | Initial slice size | `DEL-07-01` has an L context envelope. Keep the future implementation bounded to viewport/editor basics and split if it starts absorbing model tree, property inspector, results viewer, or solve-execution work. |
> | Component visualization | Simple glyphs can support recognition and selection. Detailed component properties, private library data, and editor behavior belong to PKG-03 and adjacent PKG-07 deliverables. |
> | State library choice | The exact state-management library remains `TBD`. Future work should preserve the durable/transient split regardless of library choice. |
> | Viewport library baseline | Three.js is the accepted viewport baseline. Exact versions and wrappers remain implementation-level decisions. |
> | Accessibility | The viewport should not be the only path to edit or inspect model data. Keyboard navigation, tooltips, high-contrast options, and structured panels are part of the wider GUI baseline. |
> | Diagnostics | Warnings should be attached to affected model objects where possible so users can navigate from the viewport to the underlying issue. |
> | Testing | Future tests should verify both visual presence and behavior: entity creation, selection, command dispatch, undo/redo eligibility, missing-data rendering, and no hidden defaults. |
>

### CLM-029 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Setup position |
> |---|---|
> | Rich interactive modeling vs. broad GUI scope | Favor a narrow, testable editor slice over absorbing tree, property, solve, or results workflows. |
> | Visual convenience vs. engineering warrant | Favor explicit `TBD` and diagnostics over convenience defaults for code-specific or proprietary data. |
> | Direct canvas mutation vs. service commands | Favor service-command mutation so validation, diagnostics, undo/redo, persistence, and audit records remain coherent. |
> | Detailed component graphics vs. protected data boundary | Favor simple symbolic visualization unless component geometry and metadata have lawful provenance. |
> | Visual status colors vs. compliance language | Favor statuses that distinguish model completeness, mechanics, rule checks, and human review without implying compliance or approval. |
>

### CLM-030 — Examples

> ##### Examples
>
> Acceptable future examples for this slice are limited to invented, non-code, non-proprietary interaction examples, such as:
>
> - creating a node with unit-aware coordinates supplied by the user;
> - connecting two nodes with a pipe-run element that references `TBD` section/material data until supplied;
> - rendering a bend arc as a symbolic component with user-supplied geometry placeholders;
> - surfacing a diagnostic that a component's source/provenance is missing.
>
> Do not use copied standards examples, protected tables, commercial software benchmark models, proprietary component catalogs, code-specific allowables, or real project data in public setup or future test fixtures unless explicit redistribution rights and review disposition exist.
>

### CLM-031 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict identified during setup drafting. | N/A | N/A | N/A | N/A | N/A |
>

### CLM-032 — Review Notes

> ##### Review Notes
>
> - This setup document intentionally does not choose unresolved component/state libraries.
> - This setup document intentionally does not implement product UI.
> - Future work should escalate if the viewport/editor slice needs to exceed `DEL-07-01` scope or introduce protected/private data.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-020 OBJ-006 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
