# Specification: DEL-07-06 Accessibility and usability baseline

## Scope

This deliverable specifies setup evidence for a future accessibility and engineering-review usability baseline for the OpenPipeStress GUI and report-facing review surfaces. It covers baseline keyboard access, labels/tooltips, contrast/readability, large-model navigation, result table copy/export, undo/redo discoverability, inline validation messages, warning separation, and visibility of assumptions needed by engineering reviewers.

This setup pass does not implement UI behavior, edit GUI source, edit tests, edit schemas, edit package manifests, select a final accessibility standard, assert a WCAG conformance target, alter report templates, or claim professional engineering approval or code compliance.

## Requirements

| Req ID | Requirement | Source basis | Verification hook |
|---|---|---|---|
| DEL-07-06-RQ-001 | The baseline shall keep model creation, missing data, results, assumptions, provenance, and diagnostics visible enough for engineering review. | OBJ-006; PRD sections 6.5, 8, 14, 15; SPEC sections 7 and 8 | Future GUI workflow validation checks review visibility across create, solve, review, and report-preview surfaces. |
| DEL-07-06-RQ-002 | Major GUI panels and review workflows shall support keyboard navigation paths. | PRD section 21 | Future keyboard navigation tests for major panels and modal/task flows after UI implementation is authorized. |
| DEL-07-06-RQ-003 | Icon actions, engineering status indicators, and compact controls shall have clear labels or tooltips. | PRD section 21 | Future UI tests or accessibility-tree checks for accessible names and tooltips. |
| DEL-07-06-RQ-004 | Result visualization and review surfaces shall include high-contrast/readability options sufficient for engineering review, with the exact measurable target marked `TBD` until human ruling. | PRD section 21; SOW-036; OI-002 | Future visual/a11y tests after the human accessibility target and component stack are selected. |
| DEL-07-06-RQ-005 | Large model-tree and result-review surfaces shall support search/filter and copy/export paths where PRD section 21 identifies them. | PRD section 21; PRD section 14 | Future tests for searchable/filterable model trees and copy/export behavior from result tables. |
| DEL-07-06-RQ-006 | Inline validation messages shall distinguish solve-blocking, rule-check-blocking, provenance, assumption, nonlinear, and IP-boundary warning classes where applicable. | SPEC section 7; AB-00-06 | Future tests assert diagnostic class display and no collapse of solve warnings into code-check warnings. |
| DEL-07-06-RQ-007 | Unit-bearing values, result quantities, and report-facing data shall display units and preserve unit-safety context. | OPS-K-UNIT-1; PRD section 6.6; SPEC section 8 | Future UI/report checks verify units remain visible in fields, tables, exports, and report previews. |
| DEL-07-06-RQ-008 | Undo/redo affordances shall apply only to reversible model edits and shall preserve diagnostics when solve readiness changes. | PRD section 21; AB-00-05 | Future interaction tests verify undo/redo state and diagnostic refresh after reversible edits. |
| DEL-07-06-RQ-009 | Public GUI fixtures, screenshots, report examples, and checklist examples shall not include protected standards content, proprietary values, private project data, or copied commercial examples. | OPS-K-IP-1/2/3; OPS-K-PRIV; IP_AND_DATA_BOUNDARY sections 2, 3, and 7 | Protected-content/provenance review for public fixtures and templates. |
| DEL-07-06-RQ-010 | GUI and report-facing language shall not claim certification, sealing, approval, authentication, or automatic engineering code compliance. | OPS-K-AUTH-1; TYPES section 4; PRD sections 8.4 and 15 | Product-claims review and future text snapshot tests for prohibited status/language. |

## Standards

No final accessibility conformance target is selected by this setup deliverable. `WCAG target TBD` remains the governing state until the human project authority records a target and its applicability to desktop GUI, report preview/export, and generated report artifacts. The decision record should identify whether the target applies equally to interactive desktop workflows, report preview/export surfaces, and generated report files, or whether each surface has a separate target.

Protected standards text, protected tables, proprietary engineering values, and private project content are not available or needed for this baseline. Clause-level requirements are `TBD` unless later supplied from redistributable sources or a human-approved policy.

## Verification

| Verification area | Minimum setup expectation |
|---|---|
| Keyboard access | Future tests cover traversal of major panels, dialogs, and review workflows. |
| Labels/tooltips | Future checks confirm icon controls and status indicators have accessible names or equivalent labels. |
| Contrast/readability | Future checks use the human-selected target; until then, review notes remain `TBD` rather than claiming conformance. |
| Warning separation | Future tests confirm `SOLVE_BLOCKING`, `RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, `NONLINEAR_WARNING`, and `IP_BOUNDARY_WARNING` remain distinguishable. |
| Engineering review workflow | Future validation walks through model creation, missing-data review, result review, assumptions, and report preview/export. |
| Unit/provenance visibility | Future UI/report checks confirm unit labels, source/provenance notes, and private/public status remain visible where relevant. |
| Protected content and privacy | Public fixtures, screenshots, reports, and examples pass protected-content and private-data review. |
| Professional boundary | Text, diagnostics, statuses, and reports do not use automatic `CODE_COMPLIANT` language or professional approval claims. |

Pass 3 lensing source rereads: `_SEMANTIC_LENSING.md` items A-001, A-002, F-001, F-002, F-003, and D-001 were checked against `docs/PRD.md` section 21, `docs/CONTRACT.md` OPS-K-AUTH-1, `docs/TYPES.md` section 4, `docs/_Registers/ScopeLedger.csv` SOW-036, and `docs/_Decomposition/SOFTWARE_DECOMP.md` OBJ-006 before this enrichment.

## Documentation

Expected future artifacts, when implementation is separately authorized, are:

- accessibility checklist;
- UI fixes;
- tests.

Exact checklist format, target accessibility standard, automated a11y tooling, component library, state library, test filenames, screenshot policy, and report accessibility target are `TBD`.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| DEL-07-06-CF-001 | SOW-036 requires baseline accessibility/usability, but the detailed WCAG target is explicitly TBD. | `ScopeLedger.csv` SOW-036 notes | `Specification.md#Standards` | Requirements, Standards, Verification | Keep baseline requirements qualitative and defer measurable conformance target to human ruling. | TBD |
