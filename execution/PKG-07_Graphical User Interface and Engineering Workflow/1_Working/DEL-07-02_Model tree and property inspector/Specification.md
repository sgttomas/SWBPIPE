# Specification: DEL-07-02 Model tree and property inspector

## Scope

This deliverable specifies setup evidence for the future model tree and property inspector GUI slice. It covers tree navigation, selected-entity property presentation/editing, selection synchronization with the 3D centerline workflow, missing-data visibility, provenance/private-data presentation, and UI test expectations at a setup level.

This setup pass does not implement GUI source, edit tests, select unresolved component/state libraries, introduce engineering defaults, embed protected standards content, or claim professional approval/code compliance.

## Requirements

| Req ID | Requirement | Source basis | Verification hook |
|---|---|---|---|
| DEL-07-02-RQ-001 | The model tree shall expose navigation for centerline model entities and piping component visualization without duplicating durable model truth in transient UI state. | SOW-020; AB-00-05; docs/SPEC.md section 7 | Future UI tests for tree rendering, selection, and model identity consistency. |
| DEL-07-02-RQ-002 | The property inspector shall present selected-entity fields for materials, sections, components, load cases, supports, rule-pack references, and private-library references where this slice owns the inspector surface. | SOW-021; docs/SPEC.md sections 3 and 7 | Future UI tests for entity-specific inspector panels and read-only/editable state. |
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

Expected future artifacts, when implementation is separately authorized, are:

- model tree;
- property inspector;
- UI tests.

Exact source paths, component library, state library, schema version, command/query names, test filenames, and screenshot fixture policy are `TBD`.

## Conflict Table (for human ruling)

| Conflict ID | Issue | Contenders | Human ruling |
|---|---|---|---|
| None | No source conflict identified in setup evidence. | N/A | N/A |
