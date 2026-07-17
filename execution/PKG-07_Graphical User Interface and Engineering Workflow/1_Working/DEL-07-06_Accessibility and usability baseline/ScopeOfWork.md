---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-06
package_id: PKG-07
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-036]
package_objective_refs: [OBJ-006]
---

# Scope of Work — DEL-07-06

## Purpose and Objective Traceability

This Scope of Work defines `DEL-07-06` in service of project scope [SOW-036] and package objectives [OBJ-006].

- **OUT-001** — An accessibility and engineering-review usability baseline covering keyboard access, labels, readability, large-model navigation, unit and diagnostic visibility, warning separation, and report-facing review boundaries is produced without asserting an unapproved conformance level.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-07-06 Accessibility and usability baseline

> #### Datasheet: DEL-07-06 Accessibility and usability baseline
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-06-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-07-06 |
> | Deliverable name | Accessibility and usability baseline |
> | Package ID | PKG-07 |
> | Package name | Graphical User Interface and Engineering Workflow |
> | Type | UX_UI_SLICE |
> | Scope coverage | SOW-036 |
> | Objective support | OBJ-006 |
> | Context envelope | M |
> | Setup state | Draft setup artifact for later implementation review |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Baseline value |
> |---|---|
> | Work surface | GUI workflow and report-facing review surfaces, limited to setup documentation in this run |
> | Baseline topics | Keyboard navigation, labels/tooltips, contrast/readability, search/filter, copy/export, undo/redo discoverability, inline validation, warning separation |
> | Engineering review focus | Model creation, missing data, results, assumptions, diagnostics, provenance, and human-review boundaries remain visible |
> | Architecture basis | Tauri 2 desktop shell, TypeScript/React/Vite GUI, Three.js where viewport-facing, schema-first command/query/job/result envelopes |
> | Accessibility conformance target | TBD by human ruling; this setup artifact does not assert a WCAG level |
> | Report-facing accessibility target | TBD separately for report preview/export and generated report artifacts |
> | Implementation status | No GUI source, tests, schemas, manifests, or report templates changed in this deliverable |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Handling |
> |---|---|
> | Missing solve-required data | Must remain visible as a `SOLVE_BLOCKING` diagnostic rather than being silently defaulted |
> | Missing rule-check data | Must remain visible as a `RULE_CHECK_BLOCKING` diagnostic distinct from solve blocking |
> | Weak or missing provenance | Must remain visible as provenance or assumption review information where engineering reliance may be affected |
> | Result or report review | Must preserve units, warnings, assumptions, provenance, and professional-boundary notices |
> | Accessibility evidence artifacts | Screenshots, fixtures, and exported examples must pass protected-content and private-data review before public use |
> | Protected standards or private project data | Must not be introduced into public examples, screenshots, tests, reports, or setup artifacts |
> | Compliance or certification language | Must not be claimed by the GUI, report, software, or agent output |
>

### CLM-006 — Construction

> ##### Construction
>
> This setup deliverable defines the future baseline checklist and verification hooks for accessibility and engineering-review usability. It does not select component libraries, implement UI behavior, create automated tests, choose final WCAG conformance, or change report templates.
>
> Future implementation should translate the checklist into GUI and report tests only after the relevant GUI framework choices and human accessibility target are accepted. Any implementation work remains outside this setup session.
>

### CLM-007 — References

> ##### References
>
> | Source | Relevant source slice |
> |---|---|
> | `_CONTEXT.md` | DEL-07-06 identity, SOW-036, OBJ-006, architecture basis injection |
> | `docs/PRD.md` | Sections 6.5, 7, 8, 14, 15, 20, and 21 |
> | `docs/SPEC.md` | Sections 1, 7, and 8 |
> | `docs/CONTRACT.md` | OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-AUTH-1, OPS-K-IP-1/2/3, OPS-K-PRIV-1/2, OPS-K-AGENT-1..4 |
> | `docs/TYPES.md` | Analysis-status vocabulary, provenance labels, report/result boundary |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and report boundary |
> | `docs/VALIDATION_STRATEGY.md` | GUI workflow validation and report reproducibility validation |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-07, DEL-07-06, AB-00-03, AB-00-05, AB-00-06, AB-00-08, OI-002 |
> | `docs/_Registers/Deliverables.csv` | DEL-07-06 register row |
> | `docs/_Registers/ScopeLedger.csv` | SOW-036 row |
>

### CLM-008 — D-41 R5 T5 PDU-045 current state

> ##### D-41 R5 T5 PDU-045 current state
>
> | Item | State |
> |---|---|
> | Evidence classification | `VERIFIED_NOT_VALIDATED` |
> | Review grain | Deterministic project-owned contract records |
> | Independent usability basis | Not present |
> | Desktop runtime evaluation | Not performed |
> | Contrast/readability target | `TBD_by_human_project_authority` |
> | Conformance claim | False |

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-07-06 Accessibility and usability baseline

> #### Specification: DEL-07-06 Accessibility and usability baseline
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-010 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-06-DECL-001`.
>

### CLM-011 — Scope

> ##### Scope
>
> This deliverable specifies setup evidence for a future accessibility and engineering-review usability baseline for the OpenPipeStress GUI and report-facing review surfaces. It covers baseline keyboard access, labels/tooltips, contrast/readability, large-model navigation, result table copy/export, undo/redo discoverability, inline validation messages, warning separation, and visibility of assumptions needed by engineering reviewers.
>
> This setup pass does not implement UI behavior, edit GUI source, edit tests, edit schemas, edit package manifests, select a final accessibility standard, assert a WCAG conformance target, or alter report templates. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
>

### CLM-012 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Source basis | Verification hook |
> |---|---|---|---|
> | DEL-07-06-RQ-001 | The baseline shall keep model creation, missing data, results, assumptions, provenance, and diagnostics visible enough for engineering review. | OBJ-006; PRD sections 6.5, 8, 14, 15; SPEC sections 7 and 8 | Future GUI workflow validation checks review visibility across create, solve, review, and report-preview surfaces. |
> | DEL-07-06-RQ-002 | Major GUI panels and review workflows shall support keyboard navigation paths. | PRD section 21 | Future keyboard navigation tests for major panels and modal/task flows after UI implementation is authorized. |
> | DEL-07-06-RQ-003 | Icon actions, engineering status indicators, and compact controls shall have clear labels or tooltips. | PRD section 21 | Future UI tests or accessibility-tree checks for accessible names and tooltips. |
> | DEL-07-06-RQ-004 | Result visualization and review surfaces shall include high-contrast/readability options sufficient for engineering review, with the exact measurable target marked `TBD` until human ruling. | PRD section 21; SOW-036; OI-002 | Future visual/a11y tests after the human accessibility target and component stack are selected. |
> | DEL-07-06-RQ-005 | Large model-tree and result-review surfaces shall support search/filter and copy/export paths where PRD section 21 identifies them. | PRD section 21; PRD section 14 | Future tests for searchable/filterable model trees and copy/export behavior from result tables. |
> | DEL-07-06-RQ-006 | Inline validation messages shall distinguish solve-blocking, rule-check-blocking, provenance, assumption, nonlinear, and IP-boundary warning classes where applicable. | SPEC section 7; AB-00-06 | Future tests assert diagnostic class display and no collapse of solve warnings into code-check warnings. |
> | DEL-07-06-RQ-007 | Unit-bearing values, result quantities, and report-facing data shall display units and preserve unit-safety context. | OPS-K-UNIT-1; PRD section 6.6; SPEC section 8 | Future UI/report checks verify units remain visible in fields, tables, exports, and report previews. |
> | DEL-07-06-RQ-008 | Undo/redo affordances shall apply only to reversible model edits and shall preserve diagnostics when solve readiness changes. | PRD section 21; AB-00-05 | Future interaction tests verify undo/redo state and diagnostic refresh after reversible edits. |
> | DEL-07-06-RQ-009 | Public GUI fixtures, screenshots, report examples, and checklist examples shall not include protected standards content, proprietary values, private project data, or copied commercial examples. | OPS-K-IP-1/2/3; OPS-K-PRIV; IP_AND_DATA_BOUNDARY sections 2, 3, and 7 | Protected-content/provenance review for public fixtures and templates. |
> | DEL-07-06-RQ-010 | GUI and report-facing language shall not claim certification, sealing, approval, authentication, or automatic engineering code compliance (PRD §21.2). | OPS-K-AUTH-1; TYPES section 4; PRD sections 8.4 and 15 | Product-claims review and future text snapshot tests for prohibited status/language. |
>

### CLM-013 — Standards

> ##### Standards
>
> No final accessibility conformance target is selected by this setup deliverable. `WCAG target TBD` remains the governing state until the human project authority records a target and its applicability to desktop GUI, report preview/export, and generated report artifacts. The decision record should identify whether the target applies equally to interactive desktop workflows, report preview/export surfaces, and generated report files, or whether each surface has a separate target.
>
> Protected standards text, protected tables, proprietary engineering values, and private project content are not available or needed for this baseline. Clause-level requirements are `TBD` unless later supplied from redistributable sources or a human-approved policy.
>

### CLM-014 — Verification

> ##### Verification
>
> | Verification area | Minimum setup expectation |
> |---|---|
> | Keyboard access | Future tests cover traversal of major panels, dialogs, and review workflows. |
> | Labels/tooltips | Future checks confirm icon controls and status indicators have accessible names or equivalent labels. |
> | Contrast/readability | Future checks use the human-selected target; until then, review notes remain `TBD` rather than claiming conformance. |
> | Warning separation | Future tests confirm `SOLVE_BLOCKING`, `RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, `NONLINEAR_WARNING`, and `IP_BOUNDARY_WARNING` remain distinguishable. |
> | Engineering review workflow | Future validation walks through model creation, missing-data review, result review, assumptions, and report preview/export. |
> | Unit/provenance visibility | Future UI/report checks confirm unit labels, source/provenance notes, and private/public status remain visible where relevant. |
> | Protected content and privacy | Public fixtures, screenshots, reports, and examples pass protected-content and private-data review. |
> | Professional boundary | Text, diagnostics, statuses, and reports do not use automatic `CODE_COMPLIANT` language or professional approval claims. |
>
> Pass 3 lensing source rereads: `_SEMANTIC_LENSING.md` items A-001, A-002, F-001, F-002, F-003, and D-001 were checked against `docs/PRD.md` section 21, `docs/CONTRACT.md` OPS-K-AUTH-1, `docs/TYPES.md` section 4, `docs/_Registers/ScopeLedger.csv` SOW-036, and `execution/_Decomposition/SOFTWARE_DECOMP.md` OBJ-006 before this enrichment.
>

### CLM-015 — Documentation

> ##### Documentation
>
> Expected future artifacts, when implementation is separately authorized, are:
>
> - accessibility checklist;
> - UI fixes;
> - tests.
>
> Exact checklist format, target accessibility standard, automated a11y tooling, component library, state library, test filenames, screenshot policy, and report accessibility target are `TBD`.
>

### CLM-016 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | DEL-07-06-CF-001 | SOW-036 requires baseline accessibility/usability, but the detailed WCAG target is explicitly TBD. | `ScopeLedger.csv` SOW-036 notes | `Specification.md#Standards` | Requirements, Standards, Verification | Keep baseline requirements qualitative and defer measurable conformance target to human ruling. | TBD |
>

### CLM-017 — D-41 R5 T5 PDU-045 evidence boundary

> ##### D-41 R5 T5 PDU-045 evidence boundary
>
> The current deterministic contract review remains project-owned verification,
> not independent usability validation. Desktop runtime evaluation is not
> performed, no accessibility conformance claim is emitted, and
> contrast/readability findings remain warnings while the measurable target is
> `TBD_by_human_project_authority`.
>

### CLM-018 — D-41 R5 T6 PDU-046 measurable-target hold

> ##### D-41 R5 T6 PDU-046 measurable-target hold
>
> `DEL-07-06-RQ-004` remains `VERIFIED_NOT_VALIDATED`. The qualitative
> contrast/readability review is project-owned verification only; no independent
> usability basis or human-selected measurable target was supplied. PDU-046
> cannot be upgraded by the existing warning evidence.

- **AC-001** — The contract preserves the accessibility and engineering-review baseline, keeps the measurable contrast/readability target explicitly human-owned and TBD, separates diagnostic classes, and preserves units and provenance. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Production and Verification Method — Praxeology

### CLM-019 — Procedure: DEL-07-06 Accessibility and usability baseline

> #### Procedure: DEL-07-06 Accessibility and usability baseline
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-020 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-06-DECL-004`.
>

### CLM-021 — Purpose

> ##### Purpose
>
> Define the setup procedure for producing and later using the accessibility and usability baseline for engineering-review GUI and report-facing workflows.
>

### CLM-022 — Prerequisites

> ##### Prerequisites
>
> - Sealed DEL-07-06 context with write scope limited to this deliverable folder.
> - Register scope: SOW-036 and OBJ-006.
> - Applicable invariants: OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-AUTH-1, OPS-K-IP-1/2/3, OPS-K-PRIV, OPS-K-AGENT-1..4.
> - Architecture basis: AB-00-03, AB-00-05, AB-00-06, AB-00-08 where relevant to GUI commands, state, diagnostics, result envelopes, and tests.
> - Human ruling remains required for the final accessibility conformance target.
>

### CLM-023 — Steps

> ##### Steps
>
> 1. Confirm the work is setup/document production only and does not edit GUI source, tests, schemas, manifests, package files, or repo-level docs.
> 2. Read the deliverable context, registers, contract, decomposition, and relevant product/specification source slices.
> 3. Record the accessibility/usability baseline topics from PRD section 21: keyboard navigation, icon labels/tooltips, high-contrast result visualization, search/filter, copy/export, undo/redo, project templates, inline validation, and solve-vs-code-check warning separation.
> 4. Map baseline topics to engineering review needs from OBJ-006: model creation, missing data, results, and assumptions must be visible.
> 5. Preserve diagnostic class boundaries from SPEC section 7 and AB-00-06.
> 6. Mark exact WCAG or equivalent conformance target as `TBD` pending human decision, including whether the target applies to desktop GUI, report preview/export, generated report files, or separate surfaces.
> 7. Define verification hooks for future GUI workflow validation, report reproducibility validation, protected-content review, private-data review, and product-claims review.
> 8. Generate semantic and dependency setup artifacts without treating semantic matrices as engineering authority.
> 9. Set `_STATUS.md` to `SEMANTIC_READY` only if the four documents, semantic matrix, lensing register, dependency register, and local validation checks pass.
>

### CLM-024 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Four-document kit | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist with default sections. |
> | Scope control | No files outside the DEL-07-06 working folder are modified by this task. |
> | Accessibility target | Exact WCAG or equivalent target remains `TBD`; no final conformance claim is made. |
> | Data boundary | No protected standards content, proprietary values, or private project data are introduced. |
> | Warning boundary | Solve-blocking and rule-check-blocking warnings remain distinct in future verification hooks. |
> | Fixture boundary | Future screenshots, public examples, and exported report fixtures are checked for protected standards content and private project data. |
> | Professional boundary | Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). |
> | Dependency register | `Dependencies.csv` validates against v3.1 schema. |
>
> Pass 3 lensing source rereads: `_SEMANTIC_LENSING.md` items X-001 and X-002 were checked against `docs/SPEC.md` section 7, `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06, `docs/IP_AND_DATA_BOUNDARY.md` section 3, and this procedure's verification table before this enrichment.
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
> - `_run_records/*`
> - `_STATUS.md`
>

### CLM-026 — D-41 R5 T5 PDU-045 hold check

> ##### D-41 R5 T5 PDU-045 hold check
>
> 1. Build the deterministic accessibility/usability baseline from invented GUI
>    contract records.
> 2. Confirm desktop runtime evaluation is `not_performed`.
> 3. Confirm the accessibility target remains
>    `TBD_by_human_project_authority`.
> 4. Confirm contrast/readability findings remain warnings and no accessibility
>    conformance claim is emitted.
> 5. Preserve `VERIFIED_NOT_VALIDATED`; do not infer independent usability
>    validation.

- **VER-001** — Validate the contract and review source parity, keyboard and labeling expectations, diagnostic and warning separation, unit/provenance visibility, protected-content boundaries, professional-boundary language, and every retained TBD or governed residual.

## Governing Values and Decisions — Axiology

### CLM-027 — Guidance: DEL-07-06 Accessibility and usability baseline

> #### Guidance: DEL-07-06 Accessibility and usability baseline
>

### CLM-028 — Purpose

> ##### Purpose
>
> This deliverable frames accessibility and usability as engineering-review support, not as a standalone visual polish task. The baseline should help a user navigate major GUI panels, find missing inputs, understand warnings, review results, and inspect assumptions without hiding the project boundaries defined by OpenPipeStress governance.
>

### CLM-029 — Principles

> ##### Principles
>
> - Accessibility supports auditability: keyboard paths, readable statuses, copy/export, and clear labels help reviewers inspect the model and evidence.
> - Missing data remains visible. The GUI must not hide or silently repair missing solve-required values, rule-check inputs, weak provenance, or assumptions.
> - Warning classes should stay distinct. Solve readiness, rule-check readiness, provenance, assumptions, nonlinear uncertainty, and IP boundary warnings have different engineering meanings.
> - Units and provenance are part of readability. A readable table that omits units or source status is not adequate for engineering review.
> - Accessibility conformance is not finalized here. The exact WCAG or other target remains `TBD` until human ruling.
> - Public fixtures and report examples must remain protected-data-free and private-data-free.
> - The software may support review; acceptance and professional judgment remain with the responsible engineer.
>

### CLM-030 — Considerations

> ##### Considerations
>
> Keyboard navigation should cover primary work surfaces before detailed shortcuts are optimized. Major panels include the model tree, property inspector, editor surfaces, solver/diagnostic surfaces, results browser, and report preview/export paths named in PRD section 14 and PRD section 21.
>
> Contrast/readability should be evaluated against engineering review content, not only decorative UI elements. Result color maps, warning badges, units, provenance labels, and status text are high-risk because they affect interpretation of the model and results.
>
> Search/filter and copy/export matter because engineering models can grow large. These functions should preserve context such as object identity, units, warning class, and source/provenance notes.
>
> Undo/redo needs a review-aware boundary. It should help users recover reversible modeling edits while preserving diagnostics and not implying that solved or checked states remain valid after content changes.
>
> Reports are in scope only as report-facing review surfaces and output expectations. This deliverable does not edit report templates, but it preserves the need for readable warnings, assumptions, units, provenance, and professional-boundary notices.
>
> The target decision is a human project-authority decision, not a TASK inference. Until that decision exists, implementation work should use the qualitative baseline in PRD section 21 and mark measurable conformance claims as `TBD`.
>

### CLM-031 — Trade-offs

> ##### Trade-offs
>
> | Decision area | Setup guidance |
> |---|---|
> | Qualitative baseline vs. formal target | Use the qualitative PRD baseline now; record the formal target as `TBD` until a human decision. |
> | Visual density vs. readability | Engineering review surfaces may be dense, but units, warning classes, and object identity must remain legible. |
> | Color maps vs. status clarity | Color can help but must not be the only signal for result state, warning class, or rule-check status. |
> | Keyboard workflow vs. shortcut breadth | Establish reliable navigation and focus order before broad shortcut catalogs. |
> | Report preview vs. report template design | Preserve report accessibility requirements without changing report templates in this setup deliverable. |
>

### CLM-032 — Examples

> ##### Examples
>
> - A keyboard user can move from the model tree to the property inspector, inspect a selected component, see that a required rule-pack input is missing, and reach the diagnostic details without using a mouse.
> - A result table copy/export action preserves the quantity name, object identifier, unit, result status, and relevant warning class.
> - A stress-ratio visualization uses color plus a textual/status cue so review does not depend on color alone.
> - A public screenshot fixture uses invented or neutral data and does not expose protected standards content, proprietary values, or private project data.
>

### CLM-033 — Pass 3 Source Rereads

> ##### Pass 3 Source Rereads
>
> The semantic lensing items were treated as candidate improvements only. Enrichments above were checked against `_SEMANTIC_LENSING.md` items B-001 and E-001, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 1, `docs/DIRECTIVE.md` section 2.2, and `docs/TYPES.md` section 4.
>

### CLM-034 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | DEL-07-06-CF-001 | Baseline accessibility is in scope, but the detailed WCAG target is unresolved. | `docs/_Registers/ScopeLedger.csv` SOW-036 | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-002 and DEL-07-06 note | Specification Standards and Verification; Guidance Principles | Keep conformance target as `TBD`; allow only qualitative setup requirements until human ruling. | TBD |
>

### CLM-035 — D-41 R5 T5 PDU-045 guidance

> ##### D-41 R5 T5 PDU-045 guidance
>
> Keep project-authored accessibility/usability checks classified as
> verification. Do not upgrade them to usability validation or measurable
> contrast/readability conformance without a separately authorized independent
> basis and a selected target.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-036 OBJ-006 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
