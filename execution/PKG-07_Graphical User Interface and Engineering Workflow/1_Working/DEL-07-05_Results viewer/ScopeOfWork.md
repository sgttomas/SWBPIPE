---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-05
package_id: PKG-07
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-023]
package_objective_refs: [OBJ-006, OBJ-007]
---

# Scope of Work — DEL-07-05

## Purpose and Objective Traceability

This Scope of Work defines `DEL-07-05` in service of project scope [SOW-023] and package objectives [OBJ-006, OBJ-007].

- **OUT-001** — A results-viewer contract covering unit-aware mechanics and user-rule result categories, tabular and graphical review, warnings and assumptions, result-envelope traceability, ratio availability, and report/export handoff signals is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-07-05 Results viewer

> #### Datasheet: DEL-07-05 Results viewer
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-05-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-07-05 |
> | Deliverable name | Results viewer |
> | Package ID | PKG-07 |
> | Package name | Graphical User Interface and Engineering Workflow |
> | Deliverable type | UX_UI_SLICE |
> | Scope item | SOW-023 |
> | Objectives | OBJ-006, OBJ-007 |
> | Context envelope | L |
> | Current source basis | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7; `docs/_Registers/Deliverables.csv`; `docs/_Registers/ScopeLedger.csv`; `docs/CONTRACT.md`; `docs/SPEC.md`; `docs/TYPES.md`; `docs/DIRECTIVE.md` |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Setup value | Source |
> |---|---|---|
> | Intended GUI surface | Results viewer for tabular and graphical review | `_CONTEXT.md`; `docs/SPEC.md` section 7 |
> | Result categories in scope | Displacements, rotations, forces, moments, restraint reactions, equipment loads, stresses, and ratios | SOW-023 in `docs/_Registers/ScopeLedger.csv`; `docs/SPEC.md` section 8 |
> | Rotational-deformation current slice | Result envelopes emit `rx`/`ry`/`rz` rows and the viewer exposes result rows, while the graphical deformation overlay consumes translational `ux`/`uy`/`uz` only; rotational visualization remains explicit residual work | DEC-074 O1; PDU-061; `MEMORY.md` 2026-06-12; `_run_records/TASK_RUN_2026-06-12_1110.md` |
> | Architectural route | GUI reads schema-first command/query/job result envelopes through application services | `_CONTEXT.md` Architecture Basis Injection |
> | Unit behavior | Result values must remain unit-aware and dimensionally checked | OPS-K-UNIT-1 in `docs/CONTRACT.md`; `docs/DIRECTIVE.md` section 3 |
> | Diagnostics behavior | Missing data, assumptions, provenance, nonlinear/convergence issues, and IP boundary warnings must remain visible | `docs/SPEC.md` section 7; OPS-K-DATA-2 and OPS-K-AUTH-1 in `docs/CONTRACT.md` |
> | Rule-pack ratio behavior | Ratios are displayed only when a user-supplied rule pack and required inputs support the check; incomplete rule input remains an explicit finding | SOW-023 note; OPS-K-DATA-1/2 and OPS-K-RULE-1/2/3 in `docs/CONTRACT.md` |
> | Report/export relationship | Viewer content should remain reproducible and handoff-ready for reports and structured result exports | OBJ-007; SOW-046; `docs/SPEC.md` section 8 |
> | Protected-data posture | No protected standards text, protected tables, copied formulas, proprietary thresholds, or certification claims are introduced | OPS-K-IP-1/2/3 and OPS-K-AUTH-1 in `docs/CONTRACT.md`; `docs/DIRECTIVE.md` sections 3-5 |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Status |
> |---|---|
> | Current evidence includes implemented GUI result review and a translational deformation overlay; this R5 repair changes documentation/status records only | FACT |
> | Exact UI component library, state library, and visual layout details | TBD |
> | Exact result-envelope schema fields for each displayed category | TBD, owned by schema/result-envelope implementation deliverables |
> | Exact stress-ratio formulas, thresholds, allowables, or code categories | Out of scope for public defaults; user/rule-pack supplied |
> | Professional acceptance or code compliance status | Out of software authority; human review remains required |
>

### CLM-006 — Construction

> ##### Construction
>
> The results viewer is a review surface over already-produced mechanical, diagnostic, stress, reaction, equipment-load, and user-rule-check result envelopes. Its current graphical deformation overlay uses translational components only. Rotational components are emitted and reviewable as result rows but are not visualized as rotational deformation; that surviving work is owned here and recorded in `_STATUS.md`. The viewer does not compute solver mechanics, recover stresses, evaluate rule packs, define protected code criteria, or export final reports directly unless a later sealed implementation brief expands scope.
>
> The viewer setup must preserve these visible boundaries:
>
> - mechanics solved is distinct from rule-pack checked;
> - rule-pack pass/fail is distinct from professional approval;
> - missing solve-required or rule-check-required data is a displayed finding;
> - every numerical value remains unit-aware;
> - result displays carry warnings, assumptions, and provenance hooks;
> - public examples and templates use no protected standards data or proprietary engineering values.
>

### CLM-007 — References

> ##### References
>
> - `INIT.md` - bootstrap boundaries and unknown-value rule.
> - `AGENTS.md` - Type 2 sealed dispatch rule.
> - `docs/CONTRACT.md` - invariants OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-RULE-1/2/3, OPS-K-AUTH-1, OPS-K-IP-1/2/3, OPS-K-PRIV-1/2, OPS-K-AGENT-1..4.
> - `docs/SPEC.md` - architecture, GUI, warnings, reporting, acceptance semantics.
> - `docs/TYPES.md` - analysis-status vocabulary and Result/Report object boundary.
> - `docs/DIRECTIVE.md` - product principles, stop rules, and professional boundary.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` - package, objective, and architecture-basis context.
> - `docs/_Registers/Deliverables.csv` - DEL-07-05 row.
> - `docs/_Registers/ScopeLedger.csv` - SOW-023 row.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-07-05 Results viewer

> #### Specification: DEL-07-05 Results viewer
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-05-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable owns the GUI results-viewer slice that presents reviewable analysis outputs for displacements, rotations, forces, moments, restraint reactions, equipment loads, stresses, and user-rule ratios. The current implemented slice includes tabular result review and a translational deformed-shape overlay. Emitted `rx`/`ry`/`rz` rows are available in the result surface, but rotational deformation is not graphically visualized and remains explicit DEL-07-05 residual work under DEC-074 O1 / PDU-061. It is a UX/UI slice under PKG-07 and supports OBJ-006 and OBJ-007.
>
> The results viewer does not run solver logic, recover stresses, evaluate rule packs, define protected engineering criteria, or grant professional approval. This R5 repair records the current slice and residual only; it does not implement rotational visualization or change GUI source, tests, schemas, or result data.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | REQ-07-05-001 | The viewer shall present result categories listed in SOW-023 when those categories are present in validated result envelopes. | SOW-023; `docs/SPEC.md` sections 7-8 |
> | REQ-07-05-002 | The viewer shall preserve the architecture boundary: GUI-facing code consumes application-service result envelopes and shall not bypass domain-core unit checks, provenance checks, diagnostics, or public/private data boundaries. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` section 1 |
> | REQ-07-05-003 | Displayed numerical result values shall be unit-aware and dimensionally labeled; missing or incompatible units shall be shown as diagnostics rather than silently converted or defaulted. | OPS-K-UNIT-1 and OPS-K-DATA-2 in `docs/CONTRACT.md`; `docs/DIRECTIVE.md` section 3 |
> | REQ-07-05-004 | The viewer shall distinguish mechanics result status, rule-input incompleteness, user-rule-check status, failed user-rule status, and human-review-required status; it shall not expose automatic `CODE_COMPLIANT` status. | `docs/TYPES.md` section 4; OPS-K-AUTH-1 |
> | REQ-07-05-005 | Rule-pack ratios shall be displayed only when the required user-supplied rule-pack inputs and checksum/provenance status support the calculation; otherwise the ratio surface shall show an unavailable or blocked state with a diagnostic. | SOW-023 note; OPS-K-DATA-1/2; OPS-K-RULE-1/2/3 |
> | REQ-07-05-006 | Result review shall keep warnings, assumptions, missing data, solver diagnostics, nonlinear/convergence state, provenance gaps, and IP-boundary warnings visible with the result context they qualify. | `docs/SPEC.md` section 7; OPS-K-DATA-2; OPS-K-AUTH-1 |
> | REQ-07-05-007 | Result displays intended for reports or exports shall retain traceability to model version/hash, solver version, rule-pack name/version/checksum, units, warnings, assumptions, and limitations when those envelope fields exist. | OBJ-007; SOW-039; SOW-046; `docs/SPEC.md` section 8 |
> | REQ-07-05-008 | Public setup artifacts shall not introduce protected standards text, protected tables, proprietary formulas, code-specific allowables, stress limits, SIF/flexibility values, load-combination defaults, or certification language. | OPS-K-IP-1/2/3; OPS-K-AUTH-1; `docs/DIRECTIVE.md` sections 3-5 |
> | REQ-07-05-009 | The future implementation shall include UI tests or equivalent review evidence for category availability, unit labels, warning/blocked states, status separation, and report/export handoff signals. | DEL-07-05 anticipated artifacts; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` sections 4-5 |
>

### CLM-012 — Standards

> ##### Standards
>
> No external engineering code or standards text is introduced by this deliverable. The controlling references for this setup are the OpenPipeStress governance and technical documents listed in `_REFERENCES.md`.
>
> Any future rule-check wording, stress ratio threshold, allowable, code category, or formula must come from a user-supplied or lawfully imported private rule pack and must preserve provenance and redistribution status.
>

### CLM-013 — Verification

> ##### Verification
>
> | Verification ID | Method | Expected evidence |
> |---|---|---|
> | VER-07-05-001 | Document review | Four-document kit exists and matches DEL-07-05 scope. |
> | VER-07-05-002 | Boundary review | No protected code data, proprietary thresholds, or certification claims appear in setup artifacts. |
> | VER-07-05-003 | Dependency-register validation | `Dependencies.csv` validates against v3.1 schema. |
> | VER-07-05-004 | Semantic setup review | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist with complete matrix/lens coverage. |
> | VER-07-05-005 | Current implementation and residual review | Existing UI evidence covers result categories, units, diagnostics, status separation, and the translational overlay. Confirm that emitted `rx`/`ry`/`rz` rows remain unvisualized and that rotational deformation stays recorded in `_STATUS.md` until separately implemented and verified. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required setup records for this deliverable are:
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

### CLM-015 — Acceptance Notes

> ##### Acceptance Notes
>
> This setup can be marked `SEMANTIC_READY` only after the four-document pass, semantic matrix build, lens register, Pass 3 consistency sweep, dependency extraction, and local validation gates complete. `SEMANTIC_READY` is a development lifecycle state only; it is not product implementation, code compliance, professional approval, or an issued deliverable.

### CLM-016 — D-41 R5 T5 PDU-008 current GUI boundary

> ##### D-41 R5 T5 PDU-008 current GUI boundary
>
> The viewer presents an explicit governing-ratio state: supplied ratio rows are available and filterable; no ratio rows yields an unavailable state without inferred criteria. Rotational deformation visualization remains unimplemented.

- **AC-001** — The contract preserves the implemented result surface and translational overlay, retains rotational visualization as an explicit residual, separates mechanics, user-rule checks, and human review, and invents no thresholds, formulas, allowables, code categories, compliance status, or professional approval.

## Production and Verification Method — Praxeology

### CLM-017 — Procedure: DEL-07-05 Results viewer

> #### Procedure: DEL-07-05 Results viewer
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-018 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-05-DECL-004`.
>

### CLM-019 — Purpose

> ##### Purpose
>
> This procedure records how DEL-07-05 artifacts are maintained against the implemented results-viewer slice and what a future bounded implementation brief must verify before extending that slice.
>

### CLM-020 — Prerequisites

> ##### Prerequisites
>
> - Sealed deliverable context for DEL-07-05.
> - Write scope limited to this deliverable folder.
> - Governing references in `_REFERENCES.md` are available.
> - Applicable invariants from `docs/CONTRACT.md` are preserved.
> - Architecture-basis constraints in `_CONTEXT.md` are treated as dispatch constraints, not as issued implementation content.
>

### CLM-021 — Steps

> ##### Steps
>
> 1. Confirm the deliverable identity, package, scope item, objectives, context envelope, and architecture-basis IDs from `_CONTEXT.md`.
> 2. Read the governing references named in `_REFERENCES.md`, especially `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/DIRECTIVE.md`, and the decomposition/register rows for DEL-07-05.
> 3. Run `four-documents` with `RUN_PASSES=P1_P2` by drafting `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` from accessible sources.
> 4. Run `semantic-matrix-build` by replacing `_SEMANTIC.md` with a deliverable-local semantic lens and marking semantic audit status.
> 5. Run `lens-register` by producing `_SEMANTIC_LENSING.md` with complete matrix-cell coverage and any warranted enrichment items.
> 6. Run `four-documents` with `RUN_PASSES=P3_ONLY` by checking the lensing register against the four-document kit and applying only warranted, source-supported enrichments.
> 7. Run `dependency-extract` by producing `Dependencies.csv` v3.1 and refreshing `_DEPENDENCIES.md` with conservative anchor and execution edges.
> 8. Run local validation gates for four-document presence, dependency schema, status enum, and protected-boundary spot checks.
> 9. Set `_STATUS.md` Current State to `SEMANTIC_READY` only if all setup gates pass.
>

### CLM-022 — Verification

> ##### Verification
>
> | Check | Command or method | Expected result |
> |---|---|---|
> | Four documents exist | `tools/validation/check_four_documents.sh <deliverable path>` | PASS |
> | Dependency schema valid | `python3 tools/validation/validate_dependencies_schema.py <deliverable path>/Dependencies.csv` | VALID with 29 required columns |
> | Lifecycle enum valid | `python3 tools/validation/validate_enum.py LIFECYCLE_STATE SEMANTIC_READY` | VALID |
> | Protected-data boundary | Text scan for protected standards values, code thresholds, copied formulas, or certification claims | No prohibited content found |
> | Scope boundary | `git status --short -- <deliverable path>` | Only deliverable-local files changed |
>

### CLM-023 — Records

> ##### Records
>
> The setup run must leave these records in the deliverable folder:
>
> - four-document kit;
> - semantic matrix and lensing artifacts;
> - dependency register and dependency summary;
> - run records for all five required setup steps;
> - status history showing `OPEN -> INITIALIZED -> SEMANTIC_READY`.
>

### CLM-024 — Future Implementation Procedure Notes

> ##### Future Implementation Procedure Notes
>
> A future implementation brief must not begin from these setup documents alone. It must receive source-code write scope, schema/result-envelope contracts, test requirements, and any human decisions needed for result category layout, unit handling, ratio terminology, equipment-load semantics, and report/export integration.
>
> For rotational deformation specifically, the brief must consume DEC-074 O1 / PDU-061 and the emitted `rx`/`ry`/`rz` evidence, preserve the current translational-overlay behavior unless deliberately changed, define the intended rotational visual semantics, and add focused UI/browser evidence. Until that work lands and is backchecked, `_STATUS.md` remains the sole work-discovery home for the residual; documenting the residual does not claim implementation.

- **VER-001** — Validate the contract and review source parity, result categories and unit labels, diagnostics and provenance, status separation, ratio blocked/unavailable behavior, translational versus rotational visualization boundary, report/export traceability, and every retained governed residual.

## Governing Values and Decisions — Axiology

### CLM-025 — Guidance: DEL-07-05 Results viewer

> #### Guidance: DEL-07-05 Results viewer
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-026 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-05-DECL-003`.
>

### CLM-027 — Purpose

> ##### Purpose
>
> The results viewer should make analysis outputs reviewable without hiding the conditions that make those outputs trustworthy, incomplete, or unsuitable for professional reliance. Its job is to help users inspect mechanics results, user-rule-check outputs, warnings, assumptions, and report/export readiness; it is not a solver, code interpreter, or professional approval mechanism.
>

### CLM-028 — Principles

> ##### Principles
>
> - Keep result categories visible and separable: displacements, rotations, forces, moments, reactions, equipment loads, stresses, and ratios are related but not interchangeable.
> - Treat result envelopes and diagnostics as the boundary of authority for what the GUI may show.
> - Preserve unit and dimensional context at every numerical display.
> - Show missing solve-required data and rule-check-required data as findings, not as empty success states.
> - Treat stress ratios as user-rule-pack outputs. Do not invent thresholds, allowables, categories, or pass/fail meanings.
> - Preserve export/report traceability: visible result review should align with reproducibility metadata instead of becoming an isolated screen state.
> - Keep professional-boundary notices close enough to result status that software output is not mistaken for certification, sealing, approval, or code compliance.
>

### CLM-029 — Considerations

> ##### Considerations
>
> The current result surface already provides bounded filtering, grouping, tabular review, and a translational deformation overlay. Surviving implementation work should extend that slice without changing the boundaries in this document, including decisions about:
>
> - result category navigation;
> - load case and combination selection;
> - envelope/range selection;
> - node, element, support, and equipment-load targeting;
> - tabular values versus graphical overlays;
> - rotational-deformation visualization for emitted `rx`/`ry`/`rz` rows, which is owned by DEL-07-05 but not implemented in the current overlay;
> - unit display and conversion controls;
> - diagnostic badges, warning panels, and blocked states;
> - report/export readiness indicators.
>

### CLM-030 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | Dense tables vs. graphical overlays | Use each where it supports review. Tables support exact inspection; overlays support spatial pattern recognition. Both must remain unit-aware and diagnostics-qualified. |
> | Ratio display vs. code-compliance claims | Ratios may show user-rule-pack calculations when inputs are complete. The UI must not turn those ratios into automatic professional approval or public code compliance claims. |
> | Convenience filters vs. hidden warnings | Filters should not suppress blocking diagnostics or professional-boundary status without a deliberate, visible review state. |
> | Export readiness vs. report generation | The viewer may expose report/export readiness signals, but report generation and structured result exports remain PKG-08 surfaces unless a later sealed brief says otherwise. |
>

### CLM-031 — Examples

> ##### Examples
>
> The following are structural examples only and include no engineering values:
>
> - A load-combination selection reveals displacements and rotations at selected nodes, with unit labels and any solver diagnostics attached.
> - A support selection reveals restraint reactions and active-state warnings when nonlinear support state is uncertain.
> - A stress result view shows mechanical stress values when available and shows `RULE_INPUTS_INCOMPLETE` if a requested ratio depends on missing user rule-pack inputs.
> - A report/export readiness indicator shows whether result hashes, solver version, rule-pack checksum, warnings, and assumptions are available for downstream reporting.
>

### CLM-032 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict identified during setup. | N/A | N/A | N/A | N/A | N/A |
>

### CLM-033 — Open Issues

> ##### Open Issues
>
> | Issue | Status |
> |---|---|
> | Exact result-envelope schema fields consumed by the viewer | TBD |
> | Exact UI layout, component library, state library, and broader overlay behavior | TBD; rotational-deformation visualization is a named DEL-07-05 residual under DEC-074 O1 / PDU-061 |
> | Exact rule-ratio terminology when private rule packs differ by user design basis | TBD |
> | Exact equipment-load aggregation/display semantics | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-023 OBJ-006 OBJ-007 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
