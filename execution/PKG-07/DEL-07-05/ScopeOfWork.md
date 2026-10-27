---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-05
package_id: PKG-07
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@00115c71931bcae79909602d653740d3bb72dfa1
project_scope_refs: [SOW-023]
package_objective_refs: [OBJ-006, OBJ-007]
---

# Scope of Work — DEL-07-05

Reconciliation evidence: the retired D-41 current-declaration snapshots are preserved at source `00115c71931bcae79909602d653740d3bb72dfa1` and in `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`. Current scope and decisions are resolved through `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`; dependency context is resolved through `execution/_DAG/_LATEST.md`. Open obligations remain in this Scope of Work and owning decisions, with execution in the selected graph; lifecycle remains in `_STATUS.md`. This text repair establishes no lifecycle, review, validation, release, professional-reliance, or code-compliance closure.


**Scope placement.** Owner ruling, 2026-10-23 ("I approve OD-1 through OD-5 as you recommended."; `docs/COMPLETENESS_MAP.md` §4.2), OD-5: viewport result overlays and reaction arrows (FR-GUI-001) are to be built now. This records the ruling; the clauses below are unchanged.

## Purpose and Objective Traceability

This Scope of Work defines `DEL-07-05` in service of project scope [SOW-023] and package objectives [OBJ-006, OBJ-007].

- **OUT-001** — A results-viewer contract covering unit-aware mechanics and user-rule result categories, tabular and graphical review, warnings and assumptions, result-envelope traceability, ratio availability, and report/export handoff signals is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-07-05 Results viewer

> #### Datasheet: DEL-07-05 Results viewer
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

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
> | Current source basis | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` accepted current basis; `docs/_Registers/Deliverables.csv`; `docs/_Registers/ScopeLedger.csv`; `docs/CONTRACT.md`; `docs/SPEC.md`; `docs/TYPES.md`; `docs/DIRECTIVE.md` |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Setup value | Source |
> |---|---|---|
> | Intended GUI surface | Results viewer for tabular and graphical review | `_CONTEXT.md`; `docs/SPEC.md` section 7 |
> | Result categories in scope | Displacements, rotations, forces, moments, restraint reactions, equipment loads, stresses, and ratios | SOW-023 in `docs/_Registers/ScopeLedger.csv`; `docs/SPEC.md` section 8 |
> | Rotational-deformation current slice | Result envelopes emit `rx`/`ry`/`rz` rows and the viewer exposes result rows. The graphical deformed shape consumes translational `ux`/`uy`/`uz`; rotation glyphs draw each node's emitted `rx`/`ry`/`rz` rows as a double-headed rotation-vector arrow (right-hand rule) at normalized display length (`apps/desktop/src/features/viewport/deformationOverlay.ts`, `viewportRotationGlyph.ts`). Focused browser evidence: `apps/desktop/e2e/rotation-glyph.spec.ts` | DEC-074 O1; PDU-061; `MEMORY.md` 2026-06-12; `_run_records/TASK_RUN_2026-06-12_1110.md` |
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
> | Exact UI component library, state library, and visual layout details | Resolve current dependency and component/state choices through `apps/desktop/package.json` and the implemented consumers. DEC-012 permits choices in a sealed brief or later ruling; preserve durable/transient separation and accepted architecture. Release scope follows DEC-057. |
> | Exact result-envelope schema fields for each displayed category | Resolve the accepted result-envelope fields through `schemas/results.schema.yaml` and `apps/desktop/src/features/results/ResultsPanel.tsx`; preserve category, unit, status, provenance and missing-data contracts. |
> | Exact stress-ratio formulas, thresholds, allowables, or code categories | Out of scope for public defaults; user/rule-pack supplied |
> | Professional acceptance or code compliance status | Out of software authority; human review remains required |
>

### CLM-006 — Construction

> ##### Construction
>
> The results viewer is a review surface over already-produced mechanical, diagnostic, stress, reaction, equipment-load, and user-rule-check result envelopes. Its graphical deformed shape uses translational components; rotation glyphs display the emitted rotational components. Both are display transforms of emitted rows, not physical-scale drawings: the deformed shape is drawn normalized, or for one load case or combination at a stated magnification factor of the physical displacement, and the rotation glyphs are normalized. The factor, the undeformed ghost and the animation are session view settings; they are not model data and change no result value. The viewer does not compute solver mechanics, recover stresses, evaluate rule packs, define protected code criteria, or export final reports directly unless a later sealed implementation brief expands scope.
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
> - `AGENTS.md` - bootstrap boundaries and unknown-value rule.
> - `AGENTS.md` - Type 2 sealed dispatch rule.
> - `docs/CONTRACT.md` - invariants OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-RULE-1/2/3, OPS-K-AUTH-1, OPS-K-IP-1/2/3, OPS-K-PRIV-1/2, OPS-K-AGENT-1..4.
> - `docs/SPEC.md` - architecture, GUI, warnings, reporting, acceptance semantics.
> - `docs/TYPES.md` - analysis-status vocabulary and Result/Report object boundary.
> - `docs/DIRECTIVE.md` - product principles, stop rules, and professional boundary.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` - package, objective, and architecture-basis context.
> - `docs/_Registers/Deliverables.csv` (retired in `48584391de`; see Git history) - DEL-07-05 row.
> - `docs/_Registers/ScopeLedger.csv` (retired in `48584391de`; see Git history) - SOW-023 row.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-07-05 Results viewer

> #### Specification: DEL-07-05 Results viewer
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-010 — Scope

> ##### Scope
>
> This deliverable owns the GUI results-viewer slice that presents reviewable analysis outputs for displacements, rotations, forces, moments, restraint reactions, equipment loads, stresses, and user-rule ratios. The current implemented slice includes tabular result review, a translational deformed-shape overlay, rotation glyphs for emitted `rx`/`ry`/`rz` rows (DEC-074 O1 / PDU-061), and result colour on members and support reaction arrows in the viewport (OD-5). It is a UX/UI slice under PKG-07 and supports OBJ-006 and OBJ-007.
>
> The results viewer does not run solver logic, recover stresses, evaluate rule packs, define protected engineering criteria, or grant professional approval. This R5 repair records the current slice and residual only; it does not implement rotational visualization or change GUI source, tests, schemas, or result data.
>
> Current implementation (at `62c1efd75b`): rotational visualization was added after that repair. In `apps/desktop/src/features/viewport/`, `deformationOverlay.ts` derives the translational overlay, the rotation overlay and the glyph anchors, `viewportRotationGlyph.ts` builds the glyph meshes and `PipeViewport.tsx` draws them. The result tables are `apps/desktop/src/features/results/ResultsPanel.tsx`. Focused browser evidence is `apps/desktop/e2e/rotation-glyph.spec.ts` (harness `apps/desktop/e2e/rotation-glyph/`). Per `docs/CI_STRATEGY.md`, it runs in the single Chromium project on relevant pull requests. The reference module `core/gui/results_viewer/engine.py` (`tests/test_results_viewer_contract.py`) is not called by the desktop.
>
> Current implementation (OD-5, FR-GUI-001): the viewport has two result overlays, both off by default and session view state only; they change no model, result or export. **Result colour** colours each member by one value for one load case or combination: the largest absolute value of one emitted per-member row kind over that member's rows (its stations, its two end nodes for displacement magnitude, or the emitted source rows of a component's intensified-bending row). The quantities offered are the catalogued kinds the result carries (`RESULT_COLOUR_QUANTITIES` in `resultColourOverlay.ts`); range-envelope rows are not drawn and rows of one quantity and case in more than one unit are not compared. The scale is the design system's `result.scale.1`–`.7` interpolated in OKLCH from 0 to the largest member value (design system 2.6, 6.7), with no threshold; a member with no colourable value draws `canvas.unsolved` at 60% over the ground. The canvas legend states the quantity, unit, case, five ticks, the members' minimum and maximum and the count of members with no value. A row colours only where the reader's numerical standing admits the result and the results table gives it no badge other than "checks passed" (`overlayMayShowRow` in `numericalHealth.ts`, sharing `rowHealthState` with the table); otherwise nothing colours and the legend gives "No result colour on the current model" with the numerical health header, the case's state and the output sentence. **Reaction arrows** draw each support's emitted `support_reaction_component_v2` rows for the case, only where the rows state `coordinate_system: "global"`, with their sign convention (support on pipe): the force as a single-headed arrow arriving at the support's node, the moment as the rotation glyph's double-headed arrow, each scaled to the largest drawn vector of its kind. The legend counts the producer's withheld attributions ("Ambiguous attribution is withheld"), supports without reaction rows and rows in another frame, and reads out a hovered or selected support's components and magnitudes; the same standing rule applies. Colour adds no draw call (instance colours are rewritten in place when the table changes); the arrows are static instanced meshes rebuilt only when the case or result changes. Sources: `apps/desktop/src/features/viewport/resultColourOverlay.ts`, `reactionOverlay.ts`, `viewportReactionArrows.ts`, `ResultOverlayLegend.tsx`, `viewportResource.ts` (`setResultColourPresentation`) and `PipeViewport.tsx`. Evidence: `resultColourOverlay.test.ts`, `reactionOverlay.test.ts`, `viewportReactionArrows.test.ts`, `viewportResultColour.test.ts`, `viewportPalette.test.ts`, `PipeViewport.overlays.test.tsx` and the browser spec `apps/desktop/e2e/result-overlays.spec.ts` (harness `apps/desktop/e2e/result-overlays/`). This note describes code only; it records no backcheck or acceptance.
>
> Current implementation (survey item A2): the Results section has a per-support load summary. For the chosen support it lists the emitted global reaction components, the emitted force and moment magnitude rows and the support node's translations and rotations for each load case and combination represented by finite emitted rows, then, for each column whose admitted values share one stored unit, the signed maximum and minimum with the governing case, taken over the rows that are one solved state (range envelopes are listed but not included). A case the overlays' standing rule withholds shows no values and says why. Values follow the report display rule. No allowable loads are shown; the support record carries none. Sources: apps/desktop/src/features/results/supportLoadSummary.ts and SupportLoadSummaryPanel.tsx; Design support-load-summary.md. Evidence: supportLoadSummary.test.ts, SupportLoadSummaryPanel.test.tsx and the native smoke's structure checks. This note describes code only; it records no backcheck or acceptance.
>
> Current implementation (survey item A3): the Results section has a session-only stress and ratio table. For each stress category of the current solved result's session rule-check run, it lists the run's outcomes by ratio or by computed magnitude; for each emitted stress kind, it lists admitted finite rows by magnitude. A ratio is computed value divided by limit, formed only for a less-than relation with finite values in the same stored unit and dimension, a positive limit and a finite quotient. Ranked rows are ordered largest first when comparable; stress-mode groups with mixed stored units retain source order and disable the threshold. An applied threshold hides only ranked emitted stress rows and checked outcomes below it. Incomplete outcomes (RULE_INPUTS_INCOMPLETE) are listed first with a reason built from the run's structured fields and are never ranked; incomplete, withheld, failed and unranked outcomes are never hidden by the threshold. Ranked rows are paged, with failed outcomes always listed. A case the overlays' standing rule withholds shows no values or pass/fail status and says why. Values follow the report display rule. Where available, a location button selects a bound result row or selector entity in the session. The status shown is the rule check's own; the product supplies no allowable. Sources: apps/desktop/src/features/results/stressRatioTable.ts and StressRatioTablePanel.tsx; Design stress-ratio-table.md. Evidence: stressRatioTable.test.ts, StressRatioTablePanel.test.tsx, StressRatioTablePanel.derivation.test.tsx and the native smoke's structure checks. This note describes code only; it records no backcheck or acceptance.
>
> Current implementation (survey item A8): the Results section has a displacement extremes table. For each model node, or for the nodes of supports of one stated family or of any hanger as the report counts hangers, it gives the signed maximum and minimum of each emitted global translation and rotation over the load cases and combinations the user chooses, each with its governing case, formed only where the values share one stored unit. Only admitted cases that are one solved state can be chosen; a case the overlays' standing rule withholds and a range envelope are listed, disabled and counted with their reasons. With no admitted case chosen, no table is shown. Nodes can be sorted by the largest magnitude of one component when its ranked extremes share one stored unit; otherwise model order is retained with a units-differ note. The table shows at most 50 nodes per page. Values follow the report display rule. The extremes check no clearance or travel, and no limit is shown. Sources: apps/desktop/src/features/results/displacementExtremes.ts and DisplacementExtremesPanel.tsx; Design displacement-extremes.md. Evidence: displacementExtremes.test.ts, DisplacementExtremesPanel.test.tsx and the native smoke's structure checks. This note describes code only; it records no backcheck or acceptance.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | REQ-07-05-001 | The viewer shall present result categories listed in SOW-023 when those categories are present in validated result envelopes. | SOW-023; `docs/SPEC.md` sections 7-8 |
> | REQ-07-05-002 | The viewer shall preserve the architecture boundary: GUI-facing code consumes application-service result envelopes and shall not bypass domain-core unit checks, provenance checks, diagnostics, or public/private data boundaries. | `_CONTEXT.md` (retired in `497a8d81e4`; see Git history) Architecture Basis Injection; `docs/SPEC.md` section 1 |
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
> No external engineering code or standards text is introduced by this deliverable. The controlling references for this setup are the SWBPIPE governance and technical documents listed in `_REFERENCES.md`.
>
> Any future rule-check wording, stress ratio threshold, allowable, code category, or formula must come from a user-supplied or lawfully imported private rule pack and must preserve provenance and redistribution status.
>

### CLM-013 — Verification

> ##### Verification
>
> | Verification ID | Method | Expected evidence |
> |---|---|---|
> | VER-07-05-001 | Document review | `ScopeOfWork.md` exists and matches DEL-07-05 scope. |
> | VER-07-05-002 | Boundary review | No protected code data, proprietary thresholds, or certification claims appear in setup artifacts. |
> | VER-07-05-003 | Dependency-register validation | `Dependencies.csv` validates against v3.1 schema. |
> | VER-07-05-004 | Semantic setup review | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist with complete matrix/lens coverage. |
> | VER-07-05-005 | Current implementation and residual review | Existing UI evidence covers result categories, units, diagnostics, status separation, the translational overlay and the rotation glyphs (`deformationOverlay.test.ts`, `viewportRotationGlyph.test.ts`, `PipeViewport.rotation.test.tsx`). Confirm that emitted `rx`/`ry`/`rz` rows are shown only as glyphs with their readout, and that the focused browser evidence `apps/desktop/e2e/rotation-glyph.spec.ts` (CLM-024) is bound to the tested candidate. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required setup records for this deliverable are:
>
> - `ScopeOfWork.md`
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
> This setup can be marked `SEMANTIC_READY` only after the contract review, semantic matrix build, lens register, Pass 3 consistency sweep, dependency extraction, and local validation gates complete. `SEMANTIC_READY` is a development lifecycle state only; it is not product implementation or an issued deliverable. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
### CLM-016 — D-41 R5 T5 PDU-008 current GUI boundary

> ##### D-41 R5 T5 PDU-008 current GUI boundary
>
> The viewer presents an explicit governing-ratio state: supplied ratio rows are available and filterable; no ratio rows yields an unavailable state without inferred criteria. Rotation glyphs draw emitted `rx`/`ry`/`rz` rows; with mixed rotation units across nodes, or no complete component triple, the rotation readout is unavailable rather than comparing or inferring values.

Current reading and output boundary (B7b, T4-U6). The viewer reads every result through DEL-05-02's Rust product reader and re-derives no verdict; a result the reader does not admit is shown as unsupported, historical values only. The standing line states each route's published limitations, the retained routes included. An exact-contract result also shows one stress headline per D-5 section basis (nominal and reduced), and no governing basis is chosen. A reopened saved run is shown as historical evidence: a 0.3.0 record is checked against its saved source by DEL-14-02's record validator, and a refusal is a display-only finding. Viewing grants no output. Since epoch 8 (owner, 2026-10-21: "A accept your recommendations for the entire set."; DEL-04-07 Design `desktop-exact-outputs.md`), the result JSON export, the stress-neutral export and the rule check admit pressure-2 and pressure-source-1 results at the Rust reader's numerically eligible standing. An eligible pressure-2 load/reference-state result is admitted with them, and each of those panels discloses its resolved load/reference states; a pressure-source-1 load/reference-state result is never eligible. Every other desktop output surface refuses both identities by name (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`, `apps/desktop/src/features/results/outputPolicy.ts`). Only the result JSON and stress-neutral exports admit the retained successors, at numerically eligible standing with the live native capture, which no product caller delivers before B8.

- **AC-001** — The contract preserves the implemented result surface, the translational overlay and the rotation glyphs for emitted `rx`/`ry`/`rz` rows, separates mechanics, user-rule checks, and human review, and invents no thresholds, formulas, allowables, code categories, compliance status, or professional approval.

## Production and Verification Method — Praxeology

### CLM-017 — Procedure: DEL-07-05 Results viewer

> #### Procedure: DEL-07-05 Results viewer
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

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
> 3. Maintain `ScopeOfWork.md` by drafting `ScopeOfWork.md` from accessible sources.
> 4. Run `semantic-matrix-build` by replacing `_SEMANTIC.md` with a deliverable-local semantic lens and marking semantic audit status.
> 5. Run `lens-register` by producing `_SEMANTIC_LENSING.md` with complete matrix-cell coverage and any warranted enrichment items.
> 6. Review `ScopeOfWork.md` against source evidence by checking the lensing register against the Scope of Work contract and applying only warranted, source-supported enrichments.
> 7. Run `dependency-extract` by producing `Dependencies.csv` v3.1 and refreshing `_DEPENDENCIES.md` with conservative anchor and execution edges.
> 8. Run local validation gates for Scope of Work contract, dependency schema, status enum, and protected-boundary spot checks.
> 9. Set `_STATUS.md` Current State to `SEMANTIC_READY` only if all setup gates pass.
>
### CLM-022 — Verification

> ##### Verification
>
> | Check | Command or method | Expected result |
> |---|---|---|
> | the Scope of Work contract exist | `python3 tools/scope_of_work/validate_scope_of_work.py <DELIVERABLE_PATH>/ScopeOfWork.md` | PASS |
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
> - `ScopeOfWork.md`;
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
> For rotational deformation specifically, the brief must consume DEC-074 O1 / PDU-061 and the emitted `rx`/`ry`/`rz` evidence, preserve the current translational-overlay behavior unless deliberately changed, define the intended rotational visual semantics, and add focused UI/browser evidence. Until that work lands and is backchecked, the owner-steered work graph selected through `loop/LOOP_INIT.md` selects work; `_STATUS.md` retains the residual; documenting the residual does not claim implementation.
>
> Current implementation (at `62c1efd75b`): the rotational visual semantics and focused browser evidence named above now exist in code. The rotation overlay and glyph anchors are derived in `apps/desktop/src/features/viewport/deformationOverlay.ts`, the glyph meshes are built in `viewportRotationGlyph.ts` and drawn by `PipeViewport.tsx`, with unit tests `deformationOverlay.test.ts`, `viewportRotationGlyph.test.ts` and `PipeViewport.rotation.test.tsx` and browser spec `apps/desktop/e2e/rotation-glyph.spec.ts`; DEL-07-05:1 records their behaviour. This note describes code only; it records no backcheck or residual closure.

- **VER-001** — Validate the contract and review source parity, result categories and unit labels, diagnostics and provenance, status separation, ratio blocked/unavailable behavior, translational versus rotational visualization boundary, report/export traceability, and every retained governed residual.

## Governing Values and Decisions — Axiology

### CLM-025 — Guidance: DEL-07-05 Results viewer

> #### Guidance: DEL-07-05 Results viewer
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

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
> - Keep professional-boundary notices close to result status; acceptance and professional judgment remain with the responsible engineer.
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
> - rotational-deformation visualization for emitted `rx`/`ry`/`rz` rows, now drawn as rotation glyphs, with focused browser evidence in `apps/desktop/e2e/rotation-glyph.spec.ts`;
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
> | Ratio display vs. code-compliance claims | Ratios may show user-rule-pack calculations when inputs are complete. The UI must not turn those ratios into automatic professional approval or public code compliance claims (PRD §21.2). |
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
> | Exact result-envelope schema fields consumed by the viewer | Resolve the accepted result-envelope fields through `schemas/results.schema.yaml` and `apps/desktop/src/features/results/ResultsPanel.tsx`; preserve category, unit, status, provenance and missing-data contracts. |
> | Exact UI layout, component library, state library, and broader overlay behavior | Layout/component/state choices are local implementation details under DEC-012 and current desktop sources. Rotational-deformation visualization (DEC-074 O1/PDU-061) is implemented as rotation glyphs, with focused browser evidence in `apps/desktop/e2e/rotation-glyph.spec.ts`. |
> | Exact rule-ratio terminology when private rule packs differ by user design basis | TBD |
> | Exact equipment-load aggregation/display semantics | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-023 OBJ-006 OBJ-007 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |

## Delivery commitments and evidence limits

These clauses retain open delivery duties, owner decisions, and bounded evidence limits. Their keys link to the finite source-retirement account. They do not assert completion, lift a hold, change an accepted scope boundary, or select execution work. The governing requirements and cited decisions control future implementation and acceptance.

- **DEL-07-05:1** — Implement rotational-deformation visualization for emitted `rx`/`ry`/`rz` result rows (source: DEC-074 O1 / PDU-061). The viewport now draws, per node, a double-headed arrow along the rotation vector of its emitted `global_nodal_rotation_x/y/z` rows: right-hand rule, at the deformed position when the deformed shape is drawn, on the basis that governs the node's deformed shape (no glyph if that basis lacks a complete triple; only a node without a governing displacement row uses its longest rotation), never from range-envelope rows, with length normalized to the longest drawn vector. The deformation readout reports the largest absolute emitted component with its unit. Mixed units across nodes, or no complete triple, leave the rotation overlay unavailable. The translational overlay is unchanged. Unit and readout evidence (jsdom has no WebGL, so the drawn meshes are covered by unit tests of their geometry and placement): `deformationOverlay.test.ts`, `viewportRotationGlyph.test.ts`, `PipeViewport.rotation.test.tsx`. Focused browser evidence (CLM-024): `apps/desktop/e2e/rotation-glyph.spec.ts` renders the production viewport over the bundled invented reference in Chromium. With the emitted rotation rows, the spec shows the readout, adds exactly the glyph draw calls and triangles, and changes the pixels at each glyph's projected shaft midpoint on the deformed shape. With only those rows dropped, none of this appears, the camera projection is unchanged and a canvas-corner control patch is identical. The spec runs in the required pull-request browser lane (chromium-desktop) and in the broad browser matrix.
- **DEL-07-05:2** — Preserve ratio producer/rule-pack sufficiency as upstream responsibilities and never synthesize missing criteria. The current result-semantic contract provides no eligible governing-ratio family, so the viewer reports governing ratio unavailable rather than treating a supplied unrecognized row as an eligible ratio. Evidence: the current semantic tables `fixtures/results/semantic_contract_v0_3_*.json` (no row is `governing_ratio_eligible`; B7a removed the 0.2 table), `apps/desktop/src/features/results/ResultsPanel.test.tsx` and `apps/desktop/src/App.test.tsx` (PR #787 result-integrity basis). A future eligible producer/semantic basis remains separately governed.
- **DEL-07-05:3** — Deflected-shape controls, element axes and selection-following results (FR-GUI-001, FR-SOL-007; adopted under the owner's 2026-10-27 delegation, #1662). The deformation popover selects one load case or combination and offers a magnification factor of physical displacement in model length units when the required components, convertible units and a nonzero displacement are available. When a finite default drawing can be formed, the default is the 1–2–5 factor nearest on a logarithmic scale to drawing the largest displacement at one tenth of the model's largest bounding-box side. Without a positive finite model extent or a finite default, the popover states that there is no default and still accepts a factor. A factor is drawn only while every drawn offset stays within 1e15 model length units and every drawn position is finite at the viewport's 32-bit precision; otherwise the default, or the normalized drawing, is shown with the reason. The canvas, the popover summary and its explanation state the magnification drawn, the factor times the phase. The canvas and explanation name both factors when the phase slider draws less than the full deflection. The popover also offers a dimmed undeformed model and continuous animation, disabled under a reduced-motion preference. The governing-row normalized drawing remains the default. The toolbar separately offers local triads using the straight-member frame convention (x from From to To, y from the projected `y_reference`, z = x × y), formed on coordinates converted to metres with the frame kernel's normalization sequence and its unit-length, orthogonality and right-handedness checks at 1e-12, or global axes; a table produced by the compiled kernel shows the view accepting, refusing and forming axes as the kernel does. Local triads are withheld, with the reason, for missing end nodes or `y_reference`, frames the kernel refuses, and length units that do not convert to metres in the view, and on spans marked as realized curved bends. The results table independently states and filters each row's published frame and can filter rows by the primary selected entity; with that filter on and no entity selected, it shows no rows and says so. Turning that filter off removes the selection restriction; the other filters still apply. No result value is re-projected. All controls are session view state. Evidence: `deflectedShape.test.ts`, `PipeViewport.deflection.test.tsx`, `elementAxes.test.ts`, `PipeViewport.elementAxes.test.tsx`, `selectionReveal.test.tsx`; Design `Design/deflected-shape-and-element-axes.md`. Existing ui-foundation and C3 browser layout checks pass; focused browser verification of the new deformation and axis controls remains open.
