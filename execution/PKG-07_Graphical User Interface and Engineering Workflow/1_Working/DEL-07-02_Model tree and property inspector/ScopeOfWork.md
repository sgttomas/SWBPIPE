---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-02
package_id: PKG-07
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-020, SOW-021]
package_objective_refs: [OBJ-006]
---

# Scope of Work — DEL-07-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-07-02` in service of project scope [SOW-020, SOW-021] and package objectives [OBJ-006].

- **OUT-001** — A model-tree and property-inspector contract covering stable entity hierarchy and selection, unit-aware editable properties, provenance and validation feedback, command-routed mutations, and viewport coordination is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-07-02 Model tree and property inspector

> #### Datasheet: DEL-07-02 Model tree and property inspector
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-02-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-07-02 |
> | Package ID | PKG-07 |
> | Package | Graphical User Interface and Engineering Workflow |
> | Type | UX_UI_SLICE |
> | Scope items | SOW-020, SOW-021 |
> | Objective | OBJ-006 |
> | Context envelope | M |
> | Anticipated artifacts | model tree; property inspector; UI tests |
> | Accepted delegation | `DEL-00-05` owns GUI state/interaction architecture; `DEL-07-02` owns model-tree/property-inspector behavior implementation within it (`DEC-074` O2). |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Setup value |
> |---|---|
> | UI surface | Single GUI work surface for tree navigation and selected-entity property inspection. |
> | Primary entities | Project, Model, Node, Element, Component, Material, Section, Support, LoadCase, Combination, RulePackRef, Result, and diagnostics where applicable. |
> | GUI baseline | Tauri 2 desktop shell, TypeScript/React/Vite GUI, and Three.js viewport where viewport-facing. Exact component and state libraries are `TBD`. |
> | State boundary | Durable project/model state is separate from transient session, viewport, selection, and job-progress state. GUI mutations route through application-service commands. |
> | Missing-data posture | Missing solve-required and rule-check-required values are surfaced as explicit findings, not defaulted silently. |
> | Data boundary | Private project, material, component, and rule-pack data remain user-controlled and are not transmitted or committed publicly by default. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Status |
> |---|---|
> | Tree hierarchy and grouping rules | Bounded implementation exists from accepted model identities; broader canvas gestures and full model-tree UX remain residual. |
> | Property editor field inventory | Bounded implementation preserves governed unit-bearing fields and metadata; model-level `modulus_basis_records` entry and broader editor coverage remain residual. |
> | Selection synchronization contract | Bounded tree/viewport/inspector selection is implemented within the `DEL-00-05` state/interaction architecture; transient state is not durable model truth. |
> | Command/query contract | Current structured mutations route through the application-service operation seam; broader contract coverage remains governed residual work. |
> | UI tests | Focused evidence covers bounded tree, selection, and inspector behavior; backfill and factoring for `PropertyInspector` and adjacent surfaces remain residual. |
>

### CLM-006 — Construction

> ##### Construction
>
> This kit records the bounded UI behavior already implemented and the remaining delivery boundary. `DEC-074` O2 assigns model-tree/property-inspector behavior implementation to `DEL-07-02` within the GUI state/interaction architecture owned by `DEL-00-05`; it does not transfer architecture authority, broaden functionality, choose unresolved UI libraries, introduce engineering default values, or move any artifact to `ISSUED`.
>
> The model tree and property inspector consume accepted schema/service contracts for object identity, unit-bearing fields, provenance, diagnostics, rule-pack/private-library status, and command/query/result-envelope behavior to the extent supported by current bounded evidence. Missing or unresolved engineering data remains visible as `TBD` or diagnostic state.
>

### CLM-007 — Ownership Boundary

> ##### Ownership Boundary
>
> - Architecture owner: `DEL-00-05` for GUI state/interaction architecture.
> - Behavior owner: `DEL-07-02` for bounded model-tree/property-inspector implementation within that architecture.
> - Adjacent scopes unchanged: `DEL-07-03` owns specialized material/component/rule-pack editors; `DEL-07-04` owns missing-data warning/blocking UX.
> - Ruling basis: `DEC-074` option O2, resolving `PDU-009` without scope expansion.
>

### CLM-008 — Setup Slot Checklist

> ##### Setup Slot Checklist
>
> | Slot | Setup status | Owner/source status |
> |---|---|---|
> | Tree grouping and hierarchy rules | `TBD` | Expected from accepted domain/schema and GUI state contracts. |
> | Inspector field inventory | `TBD` | Expected from accepted domain/schema contracts for model entities and library references. |
> | Unit display and edit hooks | `TBD` | Expected from accepted unit and command/query contracts. |
> | Provenance and redistribution/private-status display | `TBD` | Expected from material/component/rule-pack/library contracts. |
> | Diagnostic classes and affected-object display | `TBD` | Expected from diagnostics/result-envelope contract. |
> | Fixture, screenshot, and UI test data policy | `TBD` | Must remain synthetic, public-domain, or otherwise cleared. |
>

### CLM-009 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, scope, artifacts, and architecture-basis injection.
> - `docs/_Registers/Deliverables.csv` row `DEL-07-02`.
> - `docs/_Registers/ScopeLedger.csv` rows `SOW-020` and `SOW-021`.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, `PKG-07`, `OBJ-006`, and architecture basis IDs `AB-00-03`, `AB-00-05`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
> - `docs/CONTRACT.md` invariants `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`, `OPS-K-UNIT-1`, `OPS-K-RULE-1`, `OPS-K-RULE-3`, `OPS-K-PRIV-1`, `OPS-K-PRIV-2`, `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`, and `OPS-K-AGENT-1..4`.
> - `docs/SPEC.md` sections 1, 3, 6, 7, 10, and 11.
> - `docs/TYPES.md` sections 3, 4, 5, 6, 7, 8, and 9.
> - `execution/_Coordination/_DECISIONS/D-41_R4_RULING_2026-07-12.md`, `DEC-074` option O2.
>

### CLM-010 — Open Setup Questions

> ##### Open Setup Questions
>
> | Question | Status |
> |---|---|
> | Which accepted schema version supplies the property inspector field inventory? | `TBD` |
> | Which GUI state library, if any, is accepted for transient selection and inspector state? | `TBD` |
> | Which application-service commands and queries are accepted for model tree edits and property reads? | `TBD` |
> | Which diagnostics contract shape is accepted for inline missing-data and provenance warnings? | `TBD` |
> | Which UI test framework conventions apply to this slice beyond the architecture-basis Playwright/Vitest expectation? | `TBD` |

## Completion and Reliance Basis — Epistemology

### CLM-011 — Specification: DEL-07-02 Model tree and property inspector

> #### Specification: DEL-07-02 Model tree and property inspector
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-012 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-02-DECL-001`.
>

### CLM-013 — Scope

> ##### Scope
>
> This deliverable owns the bounded model-tree and property-inspector behavior implementation within the GUI state/interaction architecture owned by `DEL-00-05`. It covers tree navigation, selected-entity property presentation/editing, selection synchronization with the 3D centerline workflow, missing-data visibility, provenance/private-data presentation, and UI test expectations.
>
> Current evidence includes bounded implementation and focused tests. Under `DEC-074` option O2, this ownership statement does not transfer GUI architecture authority from `DEL-00-05`, broaden functionality, select unresolved component/state libraries, introduce engineering defaults, embed protected standards content, or claim professional approval/code compliance. Broader model-level modulus entry, unit-entry coverage, test/factoring hardening, and full canvas/tree/editor UX remain residual work.
>

### CLM-014 — Accepted Ownership Delegation

> ##### Accepted Ownership Delegation
>
> - `DEL-00-05` owns GUI state/interaction architecture, including the architectural selection-model role.
> - `DEL-07-02` owns model-tree/property-inspector behavior implementation within that architecture.
> - `DEC-074` option O2 resolves `PDU-009` and the corresponding ownership clarification without transferring architecture or expanding functionality.
> - `DEL-07-03` retains specialized material/component/rule-pack editor scope, and `DEL-07-04` retains missing-data warning/blocking UX scope.
>

### CLM-015 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Source basis | Verification hook |
> |---|---|---|---|
> | DEL-07-02-RQ-001 | The model tree shall expose navigation for centerline model entities and piping component visualization without duplicating durable model truth in transient UI state. | SOW-020; AB-00-05; docs/SPEC.md section 7 | Future UI tests for tree rendering, selection, and model identity consistency. |
> | DEL-07-02-RQ-002 | The property inspector shall present selected-entity fields for materials, sections, components, load cases, supports, rule-pack references, and private-library references where this slice owns the inspector surface. | SOW-021; docs/SPEC.md sections 3 and 7; DEC-074 O2 | Current implementation/run evidence and focused UI tests for entity-specific inspector panels and read-only/editable state. |
> | DEL-07-02-RQ-003 | Unit-bearing values shown or edited through the inspector shall preserve unit awareness and dimensional validation hooks. | OPS-K-UNIT-1; docs/TYPES.md object registry | Unit/display/edit validation tests once schema and service contracts are accepted. |
> | DEL-07-02-RQ-004 | Missing solve-required or rule-check-required values shall be visible as findings and shall not be silently supplied by the tree, inspector, or UI defaults. | OPS-K-DATA-2; OBJ-006; docs/SPEC.md section 7 | Negative UI tests for missing physical inputs and missing rule-pack inputs. |
> | DEL-07-02-RQ-005 | Provenance and redistribution/private status shall remain visible for materials, sections, components, and rule-pack references where inspector fields expose them. | OPS-K-DATA-3; OPS-K-RULE-3; OPS-K-PRIV-1; docs/TYPES.md sections 7 and 8 | UI tests for provenance/status display and private/public boundary indicators. |
> | DEL-07-02-RQ-006 | GUI mutations from the property inspector shall route through application-service commands; tree/inspector reads shall use governed query or result-envelope boundaries. | AB-00-03; AB-00-05 | Service-boundary review and command/query interaction tests. |
> | DEL-07-02-RQ-007 | Diagnostics shown in the inspector shall use governed diagnostic/result-envelope concepts and shall not claim certification, sealing, professional approval, or automatic code compliance. | AB-00-06; OPS-K-AGENT-4; docs/TYPES.md analysis statuses | Diagnostic presentation tests and protected/professional-claim review. |
> | DEL-07-02-RQ-008 | Public examples, screenshots, fixtures, and test data for the UI slice shall not contain protected standards text, protected tables, proprietary commercial data, or private project data. | OPS-K-IP-1; OPS-K-IP-3; OPS-K-RULE-1; OPS-K-PRIV-1 | Protected-content and fixture provenance review. |
>

### CLM-016 — Standards

> ##### Standards
>
> No protected standard text, protected tables, protected examples, material allowables, SIF/flexibility tables, proprietary component values, or proprietary project data are available in this deliverable-local setup context. Any future standards or owner-code basis must remain a private/user-supplied input or a non-protected pointer with provenance. Clause-level requirements are `TBD`.
>

### CLM-017 — Verification

> ##### Verification
>
> | Verification area | Minimum setup expectation |
> |---|---|
> | Tree navigation | Tests should confirm tree nodes represent accepted model entities and preserve stable identity through selection. |
> | Property inspector | Tests should confirm selected entity type controls visible field groups and editability. |
> | Missing data | Tests should show missing solve-required and rule-check-required values remain visible and classified. |
> | Unit safety | Tests should cover unit-bearing value display/edit pathways and dimensional validation failures once contracts exist. |
> | Provenance/privacy | Tests should show provenance and private/public redistribution status are visible where relevant. |
> | Command/query boundary | Tests or review evidence should show inspector edits do not bypass application-service commands. |
> | Professional boundary | UI text, diagnostics, and fixtures must not claim code compliance, certification, approval, or sealing. |
>

### CLM-018 — Verification Coverage Slots

> ##### Verification Coverage Slots
>
> | Coverage slot | Required future evidence |
> |---|---|
> | Tree navigation and stable identity | UI test or review evidence that tree entries map to accepted model identities. |
> | Selection synchronization | UI test or review evidence that tree, viewport, and inspector selection remain aligned without durable-state drift. |
> | Inspector field groups | UI test or review evidence that selected entity type controls visible field groups and editability. |
> | Transient/durable state split | Review evidence that selection, expansion, filters, and panel focus remain transient unless a persistence contract authorizes otherwise. |
> | Command-backed edits | UI/service test or review evidence that inspector mutations route through application-service commands. |
> | Missing-data visibility | Negative UI tests for solve-required and rule-check-required gaps. |
> | Provenance/privacy visibility | UI tests or review evidence for provenance, checksum/source status, and private/public redistribution indicators. |
> | Protected/professional-boundary review | Evidence that fixtures and UI text avoid protected data and compliance/certification claims. |
>

### CLM-019 — Documentation

> ##### Documentation
>
> Current owned implementation artifacts include:
>
> - `apps/desktop/src/features/model-tree/ModelTree.tsx`;
> - `apps/desktop/src/features/model-tree/PropertyInspector.tsx`;
> - focused UI tests and deliverable-local run evidence for bounded tree, selection, and inspector behavior.
>
> GUI component/state-library architecture remains owned by `DEL-00-05`. Broader model-level modulus entry, unit-entry coverage, test/factoring hardening, and full canvas/tree/editor UX remain open; unresolved schema versions, command/query names, and screenshot fixture policy remain `TBD` where current accepted evidence does not settle them.
>

### CLM-020 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Issue | Contenders | Human ruling |
> |---|---|---|---|
> | D41-PDU-009 | Whether GUI selection/inspector ownership remains wholly in `DEL-00-05` or behavior implementation belongs in `DEL-07-02`. | `DEL-00-05`; `DEL-07-02` | Resolved by `DEC-074` O2: `DEL-00-05` owns GUI state/interaction architecture; `DEL-07-02` owns model-tree/property-inspector behavior implementation within it. |

- **AC-001** — The contract preserves the current implemented inspection/editing boundary, explicit read-only and missing-data states, durable-versus-transient state separation, and protected/private data constraints without inventing engineering defaults, component data, authority, or hidden mutations.

## Production and Verification Method — Praxeology

### CLM-021 — Procedure: DEL-07-02 Model tree and property inspector

> #### Procedure: DEL-07-02 Model tree and property inspector
>

### CLM-022 — Purpose

> ##### Purpose
>
> Define the bounded procedure for maintaining and extending model-tree/property-inspector behavior within the GUI state/interaction architecture owned by `DEL-00-05`, without transferring architecture authority or expanding functionality.
>

### CLM-023 — Prerequisites

> ##### Prerequisites
>
> - Confirm the sealed brief names `DEL-07-02` and the approved implementation write scope before any GUI source or test file is edited.
> - Confirm the accepted `DEC-074` O2 delegation: `DEL-00-05` owns GUI state/interaction architecture and `DEL-07-02` owns model-tree/property-inspector behavior implementation within it.
> - Confirm upstream architecture basis constraints from `AB-00-03`, `AB-00-05`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
> - Confirm accepted domain/schema contracts for entity identity, unit-bearing values, provenance, diagnostics, rule-pack references, and private-library references.
> - Confirm accepted application-service command/query contracts for inspector edits and tree/selection reads.
> - Confirm all UI fixtures, screenshots, and examples are synthetic, public-domain, or otherwise cleared for repository use.
>

### CLM-024 — Steps

> ##### Steps
>
> 1. Re-read `_CONTEXT.md`, `Specification.md`, `_DEPENDENCIES.md`, and any accepted upstream schema/service contracts.
> 2. Identify the entity types and field groups that `DEL-07-02` owns for tree navigation and selected-entity inspection without redefining `DEL-00-05` state/interaction architecture.
> 3. Define tree-to-viewport-to-inspector selection behavior using transient GUI state and stable model identities.
> 4. Define property inspector read-only/editable states and command-backed mutation paths.
> 5. Preserve unit display/edit hooks, dimensional validation, provenance, private/public status, and rule-pack checksum/source status in the inspector where relevant.
> 6. Add visible findings for missing solve-required inputs, missing rule-check inputs, provenance warnings, assumptions, and IP-boundary warnings without creating defaults.
> 7. Require a bounded implementation brief for any new behavior. Preserve the open residuals for model-level `modulus_basis_records` entry, broader unit entry/pickers, test/factoring hardening, and broader canvas/tree/editor UX until separately executed.
> 8. Maintain UI tests for tree navigation, selection synchronization, inspector field groups, missing-data visibility, provenance/private status, and command/query boundary behavior.
>

### CLM-025 — Verification

> ##### Verification
>
> | Check | Expected evidence |
> |---|---|
> | Scope boundary | Changes are limited to the approved implementation scope for DEL-07-02. |
> | Tree/selection behavior | UI tests confirm model tree selection updates the inspector and stays aligned with stable model identity. |
> | Inspector behavior | UI tests confirm selected entity type controls visible field groups and command-backed edit behavior. |
> | Missing-data behavior | UI tests confirm missing solve-required and rule-check-required values are explicit findings. |
> | Unit/provenance behavior | Tests or review evidence confirm unit, provenance, checksum, and private/public status are not dropped. |
> | IP/privacy boundary | Fixtures and UI text contain no protected standards content, proprietary values, private project data, or compliance claims. |
>

### CLM-026 — Records

> ##### Records
>
> - Implementation notes or pull request summary when code work is authorized.
> - UI test results.
> - Command/query boundary review evidence.
> - Fixture and screenshot provenance notes.
> - Protected-content review evidence where applicable.
> - Missing-data, provenance, and private-status UI evidence.
> - Any human rulings for `TBD` items.
> - `DEC-074` O2 ownership-delegation evidence and the corresponding deliverable-local R5 run record.
>

### CLM-027 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The model tree, property inspector, selection flow, and command-backed edits are implemented in the bounded GUI slice. Broader product integration and ownership gaps remain only where explicitly recorded; lifecycle remains `IN_PROGRESS`.

- **VER-001** — Validate the contract and review source parity, tree/selection identity, property categories and edit routing, unit and provenance handling, viewport synchronization, diagnostics and blocked states, current residuals, and professional-boundary language.

## Governing Values and Decisions — Axiology

### CLM-028 — Guidance: DEL-07-02 Model tree and property inspector

> #### Guidance: DEL-07-02 Model tree and property inspector
>

### CLM-029 — Purpose

> ##### Purpose
>
> This deliverable owns the bounded model-tree/property-inspector behavior implementation within the GUI state/interaction architecture owned by `DEL-00-05`. The implemented and residual work helps users see model structure, selected-entity data, missing inputs, assumptions, provenance, and private/rule-pack status early in the workflow.
>

### CLM-030 — Accepted Ownership Boundary

> ##### Accepted Ownership Boundary
>
> - `DEL-00-05` retains GUI state/interaction architecture ownership.
> - `DEL-07-02` owns model-tree/property-inspector behavior implementation within that architecture.
> - `DEC-074` option O2 resolves `PDU-009`; it does not transfer architecture or broaden functionality.
> - `DEL-07-03` and `DEL-07-04` retain their specialized-editor and missing-data warning/blocking scopes respectively.
>

### CLM-031 — Principles

> ##### Principles
>
> - Treat the model tree as navigation and visibility, not a second durable model store.
> - Route edits through application-service commands and keep selection, expansion, filtering, and panel focus as transient GUI state.
> - Show missing solve-required and rule-check-required values as explicit findings instead of silently filling defaults.
> - Preserve unit and provenance context for editable engineering fields.
> - Keep rule-pack checks, private-library values, and professional acceptance distinct from the open mechanics model.
> - Keep implementation choices that remain unresolved, including component/state libraries and exact command/query names, as `TBD`.
>

### CLM-032 — Considerations

> ##### Considerations
>
> The bounded tree and inspector implementation consumes upstream domain schema, persistence, command/query/job envelope, diagnostics, rule-pack, material/component/library, unit, and GUI state contracts. Current evidence settles bounded behavior only; unresolved contracts and broader UX remain governed residuals rather than authority for this deliverable to redefine `DEL-00-05` architecture.
>
> The inspector should support workflows that make engineering gaps visible before solve execution or rule checking. It should not turn missing code-specific, material, component, SIF/flexibility, allowable, or rule-pack values into public defaults. Private project and library data should remain local/user-controlled unless a later export or contribution path is explicitly authorized with documented rights.
>

### CLM-033 — Trade-offs

> ##### Trade-offs
>
> | Topic | Guidance |
> |---|---|
> | Tree richness vs scope control | Include the navigation and inspection concepts needed by DEL-07-02, but leave specialized editors to DEL-07-03 and missing-data blocking UX to DEL-07-04 unless an accepted brief expands scope. |
> | Convenience vs data boundary | Do not prefill protected/code-specific values for convenience; use visible `TBD`, diagnostics, or incomplete-state UI. |
> | Selection state vs durable data | Keep current selection, tree expansion, filters, and panel focus out of durable project state unless a persistence contract later authorizes a view-state record. |
> | Editable inspector vs command boundary | Inspector edits should be command-backed and validation-aware; direct mutation of durable model objects from UI state should be avoided. |
> | Rule status vs professional judgment | Show user-rule/check readiness or diagnostics only as software findings; do not present them as professional code compliance. |
>

### CLM-034 — Boundary Rationale

> ##### Boundary Rationale
>
> The inspector is a convenience surface over governed model and service contracts. Command-backed edits keep validation, unit checks, diagnostics, provenance handling, undo/redo scope, and result-envelope behavior in the application-service boundary instead of allowing UI state to become an alternate authority.
>
> The split with adjacent GUI deliverables remains explicit: DEL-07-02 owns navigation and selected-entity inspection setup, DEL-07-03 owns specialized material/component/rule-pack editor implementation, and DEL-07-04 owns missing-data warning and blocking UX unless a later sealed brief or human ruling changes that boundary.
>

### CLM-035 — Examples

> ##### Examples
>
> Current focused tests and run evidence use governed fixtures for bounded behavior. Any future screenshots, examples, or additional fixtures must use synthetic, public-domain, or otherwise cleared model data and must not reproduce protected standards content, proprietary project data, or protected code examples.
>

### CLM-036 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Issue | Contenders | Human ruling |
> |---|---|---|---|
> | D41-PDU-009 | Selection/inspector architecture versus behavior ownership. | `DEL-00-05`; `DEL-07-02` | `DEC-074` O2: `DEL-00-05` owns GUI state/interaction architecture; `DEL-07-02` owns model-tree/property-inspector behavior implementation within it. |
>

### CLM-037 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The model tree, property inspector, selection flow, and command-backed edits are implemented in the bounded GUI slice. Broader product integration and ownership gaps remain only where explicitly recorded; lifecycle remains `IN_PROGRESS`.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-020 SOW-021 OBJ-006 | CLM-011 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
