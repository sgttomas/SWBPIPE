# Specification: DEL-07-02 Model tree and property inspector

## Scope

This deliverable owns the bounded model-tree and property-inspector behavior implementation within the GUI state/interaction architecture owned by `DEL-00-05`. It covers tree navigation, selected-entity property presentation/editing, selection synchronization with the 3D centerline workflow, missing-data visibility, provenance/private-data presentation, and UI test expectations.

Current evidence includes bounded implementation and focused tests. Under `DEC-074` option O2, this ownership statement does not transfer GUI architecture authority from `DEL-00-05`, broaden functionality, select unresolved component/state libraries, introduce engineering defaults, embed protected standards content, or claim professional approval/code compliance. Broader model-level modulus entry, unit-entry coverage, test/factoring hardening, and full canvas/tree/editor UX remain residual work.

## Accepted Ownership Delegation

- `DEL-00-05` owns GUI state/interaction architecture, including the architectural selection-model role.
- `DEL-07-02` owns model-tree/property-inspector behavior implementation within that architecture.
- `DEC-074` option O2 resolves `PDU-009` and the corresponding ownership clarification without transferring architecture or expanding functionality.
- `DEL-07-03` retains specialized material/component/rule-pack editor scope, and `DEL-07-04` retains missing-data warning/blocking UX scope.

## Requirements

| Req ID | Requirement | Source basis | Verification hook |
|---|---|---|---|
| DEL-07-02-RQ-001 | The model tree shall expose navigation for centerline model entities and piping component visualization without duplicating durable model truth in transient UI state. | SOW-020; AB-00-05; docs/SPEC.md section 7 | Future UI tests for tree rendering, selection, and model identity consistency. |
| DEL-07-02-RQ-002 | The property inspector shall present selected-entity fields for materials, sections, components, load cases, supports, rule-pack references, and private-library references where this slice owns the inspector surface. | SOW-021; docs/SPEC.md sections 3 and 7; DEC-074 O2 | Current implementation/run evidence and focused UI tests for entity-specific inspector panels and read-only/editable state. |
| DEL-07-02-RQ-003 | Unit-bearing values shown or edited through the inspector shall preserve unit awareness and dimensional validation hooks. | OPS-K-UNIT-1; docs/TYPES.md object registry | Unit/display/edit validation tests once schema and service contracts are accepted. |
| DEL-07-02-RQ-004 | Missing solve-required or rule-check-required values shall be visible as findings and shall not be silently supplied by the tree, inspector, or UI defaults. | OPS-K-DATA-2; OBJ-006; docs/SPEC.md section 7 | Negative UI tests for missing physical inputs and missing rule-pack inputs. |
| DEL-07-02-RQ-005 | Provenance and redistribution/private status shall remain visible for materials, sections, components, and rule-pack references where inspector fields expose them. | OPS-K-DATA-3; OPS-K-RULE-3; OPS-K-PRIV-1; docs/TYPES.md sections 7 and 8 | UI tests for provenance/status display and private/public boundary indicators. |
| DEL-07-02-RQ-006 | GUI mutations from the property inspector shall route through application-service commands; tree/inspector reads shall use governed query or result-envelope boundaries. | AB-00-03; AB-00-05 | Service-boundary review and command/query interaction tests. |
| DEL-07-02-RQ-007 | Diagnostics shown in the inspector shall use governed diagnostic/result-envelope concepts and shall not claim certification, sealing, professional approval, or automatic code compliance. | AB-00-06; OPS-K-AGENT-4; docs/TYPES.md analysis statuses | Diagnostic presentation tests and protected/professional-claim review. |
| DEL-07-02-RQ-008 | Public examples, screenshots, fixtures, and test data for the UI slice shall not contain protected standards text, protected tables, proprietary commercial data, or private project data. | OPS-K-IP-1; OPS-K-IP-3; OPS-K-RULE-1; OPS-K-PRIV-1 | Protected-content and fixture provenance review. |

## Standards

No protected standard text, protected tables, protected examples, material allowables, SIF/flexibility tables, proprietary component values, or proprietary project data are available in this deliverable-local setup context. Any future standards or owner-code basis must remain a private/user-supplied input or a non-protected pointer with provenance. Clause-level requirements are `TBD`.

## Verification

| Verification area | Minimum setup expectation |
|---|---|
| Tree navigation | Tests should confirm tree nodes represent accepted model entities and preserve stable identity through selection. |
| Property inspector | Tests should confirm selected entity type controls visible field groups and editability. |
| Missing data | Tests should show missing solve-required and rule-check-required values remain visible and classified. |
| Unit safety | Tests should cover unit-bearing value display/edit pathways and dimensional validation failures once contracts exist. |
| Provenance/privacy | Tests should show provenance and private/public redistribution status are visible where relevant. |
| Command/query boundary | Tests or review evidence should show inspector edits do not bypass application-service commands. |
| Professional boundary | UI text, diagnostics, and fixtures must not claim code compliance, certification, approval, or sealing. |

## Verification Coverage Slots

| Coverage slot | Required future evidence |
|---|---|
| Tree navigation and stable identity | UI test or review evidence that tree entries map to accepted model identities. |
| Selection synchronization | UI test or review evidence that tree, viewport, and inspector selection remain aligned without durable-state drift. |
| Inspector field groups | UI test or review evidence that selected entity type controls visible field groups and editability. |
| Transient/durable state split | Review evidence that selection, expansion, filters, and panel focus remain transient unless a persistence contract authorizes otherwise. |
| Command-backed edits | UI/service test or review evidence that inspector mutations route through application-service commands. |
| Missing-data visibility | Negative UI tests for solve-required and rule-check-required gaps. |
| Provenance/privacy visibility | UI tests or review evidence for provenance, checksum/source status, and private/public redistribution indicators. |
| Protected/professional-boundary review | Evidence that fixtures and UI text avoid protected data and compliance/certification claims. |

## Documentation

Current owned implementation artifacts include:

- `apps/desktop/src/features/model-tree/ModelTree.tsx`;
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`;
- focused UI tests and deliverable-local run evidence for bounded tree, selection, and inspector behavior.

GUI component/state-library architecture remains owned by `DEL-00-05`. Broader model-level modulus entry, unit-entry coverage, test/factoring hardening, and full canvas/tree/editor UX remain open; unresolved schema versions, command/query names, and screenshot fixture policy remain `TBD` where current accepted evidence does not settle them.

## Conflict Table (for human ruling)

| Conflict ID | Issue | Contenders | Human ruling |
|---|---|---|---|
| D41-PDU-009 | Whether GUI selection/inspector ownership remains wholly in `DEL-00-05` or behavior implementation belongs in `DEL-07-02`. | `DEL-00-05`; `DEL-07-02` | Resolved by `DEC-074` O2: `DEL-00-05` owns GUI state/interaction architecture; `DEL-07-02` owns model-tree/property-inspector behavior implementation within it. |
