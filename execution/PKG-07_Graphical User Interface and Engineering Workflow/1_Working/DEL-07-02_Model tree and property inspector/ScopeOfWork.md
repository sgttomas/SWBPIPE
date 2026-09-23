---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-02
package_id: PKG-07
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:c78301c67df5729d65c57963e6a915339049e3ffaa12d13961ee201445a9b984
project_scope_refs: [SOW-020,SOW-021,SOW-077]
package_objective_refs: [OBJ-006]
---

# Scope of Work — DEL-07-02

Reconciliation evidence: the retired D-41 current-declaration snapshots are preserved at source `00115c71931bcae79909602d653740d3bb72dfa1` and in `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`. Current scope and decisions are resolved through `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`; dependency context is resolved through `execution/_DAG/_LATEST.md`. Open obligations remain in this Scope of Work and owning decisions, with execution in the selected graph; lifecycle remains in `_STATUS.md`. This text repair establishes no lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

## Current performance acceptance boundary — D-72

Redesigned-product performance acceptance, settled-frame behavior and owned-resource obligations remain open under D-72 and its final addendum. Apply the five inherited D-68 numeric limits, the accepted reference profile, geometry, workload and observation rules, and the S-1/S-2 run/pass rule; S-3 was not accepted. The decision records govern exact criteria. No present implementation or historical demonstration qualifies a later candidate. Select and run a successor demonstration before asserting acceptance. Sources: `execution/_Coordination/_DECISIONS/D-72_RULING_2026-09-18.md` and `D-72_RULING_ADDENDUM_2026-09-18.md`.

## Purpose and Objective Traceability

This Scope of Work defines `DEL-07-02` in service of project scope [SOW-020, SOW-021] and package objectives [OBJ-006].

- **OUT-001** — A model-tree and property-inspector contract covering stable entity hierarchy and selection, unit-aware editable properties, provenance and validation feedback, command-routed mutations, and viewport coordination is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-07-02 Model tree and property inspector

> #### Datasheet: DEL-07-02 Model tree and property inspector
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

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
> | GUI baseline | Resolve current dependency and component/state choices through `apps/desktop/package.json` and the implemented consumers. DEC-012 permits choices in a sealed brief or later ruling; preserve durable/transient separation and accepted architecture. Release scope follows DEC-057. |
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
> Maintain tree grouping, inspector field inventory, unit display/edit hooks, provenance/redistribution/privacy display and diagnostic affected-object navigation against `schemas/model.schema.yaml`, `schemas/results.schema.yaml`, `ModelTree.tsx` and `PropertyInspector.tsx` in `apps/desktop/src/features/model-tree/`. The field contracts and candidate implementation supply the inventory; do not preserve a blanket setup-era TBD. Fixture/screenshot/UI data must remain synthetic, public-domain or otherwise cleared; actual provenance review remains required.

### CLM-009 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, scope, artifacts, and architecture-basis injection.
> - `docs/_Registers/Deliverables.csv` row `DEL-07-02`.
> - `docs/_Registers/ScopeLedger.csv` rows `SOW-020` and `SOW-021`.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` accepted current basis, `PKG-07`, `OBJ-006`, and architecture basis IDs `AB-00-03`, `AB-00-05`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
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
> | Which accepted schema version supplies the property inspector field inventory? | Resolve fields through `schemas/model.schema.yaml` and the current property-inspector consumer; bind verification to their candidate revisions. |
> | Which GUI state library, if any, is accepted for transient selection and inspector state? | Resolve current dependency and component/state choices through `apps/desktop/package.json` and the implemented consumers. DEC-012 permits choices in a sealed brief or later ruling; preserve durable/transient separation and accepted architecture. Release scope follows DEC-057. |
> | Which application-service commands and queries are accepted for model tree edits and property reads? | Use the accepted PKG-16 operation seam and `apps/desktop/src/services/operationService.ts`; no direct durable-state mutation or alternative validator is authorized. |
> | Which diagnostics contract shape is accepted for inline missing-data and provenance warnings? | Resolve diagnostics through the accepted result/operation envelopes and the inspector consumer; missing-data and provenance warnings remain required, and interface deficiencies remain delivery work. |
> | Which UI test framework conventions apply to this slice beyond the architecture-basis Playwright/Vitest expectation? | Use the candidate-bound desktop package scripts, Vitest and Playwright configuration; existing test definitions are not a fresh execution result. |

## Completion and Reliance Basis — Epistemology

### CLM-011 — Specification: DEL-07-02 Model tree and property inspector

> #### Specification: DEL-07-02 Model tree and property inspector
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-013 — Scope

> ##### Scope
>
> This deliverable owns the bounded model-tree and property-inspector behavior implementation within the GUI state/interaction architecture owned by `DEL-00-05`. It covers tree navigation, selected-entity property presentation/editing, selection synchronization with the 3D centerline workflow, missing-data visibility, provenance/private-data presentation, and UI test expectations.
>
> Current evidence includes bounded implementation and focused tests. Under `DEC-074` option O2, this ownership statement does not transfer GUI architecture authority from `DEL-00-05`, broaden functionality, select unresolved component/state libraries, introduce engineering defaults, or embed protected standards content. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Broader model-level modulus entry, unit-entry coverage, test/factoring hardening, and full canvas/tree/editor UX remain residual work.
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
> | DEL-07-02-RQ-007 | Diagnostics shown in the inspector shall use governed diagnostic/result-envelope concepts and shall not claim certification, sealing, professional approval, or automatic code compliance (PRD §21.2). | AB-00-06; OPS-K-AGENT-4; docs/TYPES.md analysis statuses | Diagnostic presentation tests and protected/professional-claim review. |
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
> | Professional boundary | UI text, diagnostics, and fixtures must not claim code compliance, certification, approval, or sealing (PRD §21.2). |
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
> GUI component/state-library architecture remains owned by `DEL-00-05`. Resolve editable field inventory, unit entry and command/query names through `schemas/model.schema.yaml`, PropertyInspector and workspaceSession; DEC-111 assigns creation/generic editing to DEL-07-02 and specialized editors to DEL-07-03. Preserve missing interaction witnesses, test/factoring gaps and actual fixture/screenshot provenance review separately.
>

### CLM-020 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Issue | Contenders | Human ruling |
> |---|---|---|---|
> | D41-PDU-009 | Whether GUI selection/inspector ownership remains wholly in `DEL-00-05` or behavior implementation belongs in `DEL-07-02`. | `DEL-00-05`; `DEL-07-02` | Resolved by `DEC-074` O2: `DEL-00-05` owns GUI state/interaction architecture; `DEL-07-02` owns model-tree/property-inspector behavior implementation within it. |

- **AC-001** — The contract preserves the current implemented inspection/editing boundary, explicit read-only and missing-data states, durable-versus-transient state separation, and protected/private data constraints without inventing engineering defaults, component data, authority, or hidden mutations. Current source references are `PipeViewport.tsx`, `ModelTree.tsx`, `PropertyInspector.tsx` and `workspaceSession.ts`; preserve DEC-111 creation/generic-edit versus specialized-editor ownership. Source presence does not discharge independent interaction/parity review.


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
> 1. Re-read `_CONTEXT.md`, `ScopeOfWork.md`, `_DEPENDENCIES.md`, and any accepted upstream schema/service contracts.
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
> The split with adjacent GUI deliverables remains explicit: DEL-07-02 owns navigation, selected-entity inspection, creation and generic editing under SCA-011/DEC-111, DEL-07-03 owns specialized material/component/rule-pack editor implementation, and DEL-07-04 owns missing-data warning and blocking UX unless a later sealed brief or human ruling changes that boundary.
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

### CLM-038 — SCA-011 Generic and support editing

**SCA-011 ownership amendment:** this keyed allocation supersedes inconsistent forward ownership wording above under the recorded Group 2 application decision; original observations and all other requirements retain their source meaning.

Own generic tree/table/inspector create/edit/delete interaction with unit/provenance preservation, including support/restraint creation and editing and generic selected-load inspection. DEL-07-03 owns load-manager editing. DEL-07-09 remains R-006 coverage-accountability owner; no dedicated support-editor slice is activated.

- **OUT-002** — This responsibility has an explicit owner and claim-bound verification.
- **AC-002** — The named boundary is honored, its witness is bound to the tested candidate, and missing or held results remain explicit. Ownership assignment alone is not a pass.
- **VER-002** — Exercise generic and support routes through validation/preview/explicit acceptance and DEL-16-06 application; retain nonconvergence/unknown-state visibility and all native-only/unsupported limitations.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-020 SOW-021 OBJ-006 | CLM-011 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
| OUT-002 | OBJ-006 | CLM-038 | AC-002 | VER-002 | Source-bound boundary review and named contract witness; missing evidence remains open |

## Delivery commitments and evidence limits

These clauses retain open delivery duties, owner decisions, and bounded evidence limits. Their keys link to the finite source-retirement account. They do not assert completion, lift a hold, change an accepted scope boundary, or select execution work. The governing requirements and cited decisions control future implementation and acceptance.

- **DEL-07-02:2** — Performance acceptance, settled-frame and owned-resource obligations remain open on the redesigned product under [D-72](../../../_Coordination/_DECISIONS/D-72_RULING_2026-09-18.md) and its [final addendum](../../../_Coordination/_DECISIONS/D-72_RULING_ADDENDUM_2026-09-18.md). All six criteria are ruled and frozen: item 5 uses S-1/S-2, with S-3 not accepted. The addendum removes the fresh former-interface baseline cohort; the original D-70 failures and successor demonstration retain their distinct attribution. Independent-usability holds remain. Qualification requires the ruled candidate evidence; no performance acceptance or lifecycle promotion follows from the criteria ruling.
- **DEL-07-02:4** — Short panels can require scrolling between a label and its fully visible control. The original transient inspector AX omission remains unexplained although fresh processes expose controls; no independent macOS audit or whole-product WCAG conformance is claimed. Independent practitioner usability/security remain separate held work, not extra publication gates.
- **DEL-07-02:5** — Add GUI entry/emission of the model-level modulus_basis_records table (schema model.schema.yaml; per-load-case modulus_basis_ref selection plus bend_pipe_ref/mill-tolerance/equivalent-static entry landed via TP-PMM-GUIEMIT-001 PR #156 and applier acceptance TP-APP-R5-FIELDRULES-001 PR #162) (source: Receipt 10 delta / model.schema.yaml)
- **DEL-07-02:6** — Broaden app unit entry/pickers beyond the covered B-tail surfaces (residual hardening, select when it de-risks current-stage work) (source: PRD plan §3 Phase B-tail row / FR-002)
- **DEL-07-02:7** — Backfill unit tests and factoring for LoadCaseManagerPanel, PipeViewport, PropertyInspector (see also DEL-07-01) (source: PRD plan §3 hardening row H3 / seam plan §9.4)
- **DEL-07-02:8** — Complete broader canvas/model-tree/property-editor UX beyond the landed D-68 selection/virtualized-workspace foundation and table-editing slices. Evidence references: `apps/desktop/e2e/ui-foundation.spec.ts`, `apps/desktop/src/features/model-tree/ModelTree.test.tsx` and `apps/desktop/src/features/workspace/table/ModelTree.table.test.tsx`. PRD FR-003/FR-013/FR-014 residuals still need bounded work-graph assignments and claim-specific acceptance evidence; the landed slices do not establish full UX closure.
- **DEL-07-02:9** — Current toolkit adds rich support/hanger/nonlinear/family configuration, reference-guarded removal, explicit equipment/nozzle DOFs and real Rust-backed display readouts. Unsupported display dimensions retain entered values; drafts/model/evidence stay unchanged. N7 V2 integrated review and parent fan-in were accepted; these implementation records do not close broader UX, modulus-basis or independent validation residuals. See DEL-07-09 current coverage and routing records.
- **DEL-07-02:10** — N7 F1/F3 repairs reject conflicting duplicate stiffness and noncanonical explicit family tokens; canonical UI family values and legacy omitted/null inference are preserved. F2 support/boundary readouts derive linear versus rotational dimension from the effective DOF; unsupported targets retain entered values. Postrepair desktop build / 43 files / 732 tests and host dist 1 test PASS; N7 V2 final rereview subsequently returned PASS and parent fan-in was accepted. See DEL-07-09 Palette_Operation_Routing.md N7 repair amendment; no lifecycle closure.
- **DEL-07-02:11** — Historical review evidence is available at `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N7_FINAL_REVIEW/V2_BACKCHECK/REVIEW_RETURN_V2.md` (PASS) and `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/PRECOMMIT_PARENT_FAN_IN_V1.md` (bounded parent fan-in). The formerly cited `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json` is unavailable in the inspected project tree; existence elsewhere and its asserted v11 revalidation remain unknown. Locate the intake record or obtain a bounded replacement verification before relying on that revalidation. Historical review snapshots, coverage cells and lifecycle state are unchanged; these records do not accept a later UI candidate.
