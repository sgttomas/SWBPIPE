# Piping SCA-009 Gate 3 — exact amendment preview

## State

`CANDIDATE — GATE 3 NOT APPROVED`

This is an exact, derivative amendment candidate. Every postimage below is a
candidate file stored under this snapshot's `postimages/` directory; **no
live surface has been modified**. The live `SOFTWARE_DECOMP.md`, the three
companion registers, the DEL-07-03 metadata files, and both `_LATEST.md`
pointers are byte-identical to the basis. No repository, pointer, snapshot,
notice, product, runtime, lifecycle, release, reliance, dependency,
estimate, schedule, or Git state has changed.

## Basis and authority

- Current basis: `main@89758a32634ee6cedbd1dbadf35e3728fb48d2eb` plus the
  intact ruled-upon branch commit
  `f5112824f055b3b5584a852dd68923530dc6620b` on
  `claude/piping-sca-009-gate2-20260820` (single landing PR #593 per the
  owner's process direction).
- Accepted Gate-2 impact: `Impact_Assessment.md` SHA-256
  `bfa25d898e65b82012b2a93988432a121d5f2b842a5469cf7d53593a1a2ba6d0`,
  approved with two owner modifications (`ACCEPTANCE_RECORD.md` Gate-2
  section): **context envelope L** for `DEL-07-09` (post-change
  distribution `S=9, M=69, L=24, XL=0`) and the **implementation-landing
  column** now carried by `Vocabulary_Annex.md`.
- Decision row: **`DEC-094`** — verified as the next free decision ID at
  execution (live §12 ends at `DEC-093`; the Gate-2 assessment's
  expectation of a next-free row after `DEC-091` was stale because
  `DEC-092`/`DEC-093` landed via other instruments; no collision exists).

## SourceRef verification (SOW-077)

The proposed SourceRef `PRD v0.3 §14; SCA-009 owner request 2026-08-20`
was verified against the live `docs/PRD.md`: the adopted PRD authority is
**version 0.3** (amended 2026-07-16 per D-47 / DEC-080 / SCA-007), and its
**§14 "GUI Requirements"** (§14.1 Main Interface) enumerates the
viewport/tree/inspector/editor/manager surfaces the vocabulary and palette
contract governs. The citation is correct as proposed; no correction was
required.

## Exact working-surface postimages

| Live path | Preimage SHA-256 | Exact postimage | Postimage SHA-256 |
|---|---|---|---|
| `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` | `7d1edd912e2d07a75a5ffb52fd3498a7c54bd86af1ea94784532c7571a29924c` | `postimages/SOFTWARE_DECOMP.md` | `a1bf1148b8b96aed83c8a042437b9714d543cb068070662b97397dc700da48a3` |
| `projects/chirality-piping/docs/_Registers/ScopeLedger.csv` | `77f4d3163a9cdd8b454216286cfd0caae0dedec96a897ccd3654f6a90be49785` | `postimages/ScopeLedger.csv` | `97f5a113739e193f07f6d1dfac36f43e08642e12c983f3a3b597a31abe553238` |
| `projects/chirality-piping/docs/_Registers/Deliverables.csv` | `249bb83544c118a1577bbcc59878366cb49dda024f028c1726bf4aacbc2beaa0` | `postimages/Deliverables.csv` | `f46ea92ab5e5896e86f6b0b8ecbec4e98158886def5d0cf0d7b80d0200f03539` |
| `projects/chirality-piping/docs/_Registers/ContextBudgetQA.csv` | `0da89a2026a1e70c73de4a66c85e4841be2322f7bcd7a282bcb0ea4a422458fd` | `postimages/ContextBudgetQA.csv` | `84497efb81d4b31b085e016d37a285aed3755be7740a632794e355dad259468d` |
| `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/_STATUS.md` | `9791393d283b1deee625754ebb4e2ef51a6d090ec6b9fcfc367d1d1982694771` | `postimages/DEL-07-03_STATUS.md` | `41039f77e6c0d86dcd013e0c17f09bf51889fbc1aef3a9a68e54d834f8e95596` |
| `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/_CONTEXT.md` | `eb7db50a6adecce8521b402223a3cacae0803fdc64d4ba3abf719ad5623034c0` | `postimages/DEL-07-03_CONTEXT.md` | `e69a8d7d832b5af0f15e08c3c242e9d5ace98f26a752ee41fda06855f528ae10` |

The complete exact diffs are embedded below (this SCA keeps them inline in
this preview rather than as separate patch files; the `postimages/` files
are the byte-exact application targets).

## Exact decomposition changes (`SOFTWARE_DECOMP.md`)

1. Frontmatter `revision: 0.11` → `revision: 0.12`.
2. §1: one new revision-history paragraph recording SCA-009 (inserted after
   the v0.11/SCA-008 paragraph).
3. §4 SSOW: +1 register row `SOW-077` (after `SOW-076`).
4. §5 Objectives: `OBJ-006` gains `SOW-077` / `DEL-07-09`; `OBJ-015` gains
   `SOW-077` / `DEL-07-09`.
5. §6 Packages: PKG-07 Assigned Scope Items += `SOW-077`.
6. §7 Deliverables (PKG-07): +1 row `DEL-07-09`, Type `UX_UI_SLICE`,
   Context **L** per the owner's Gate-2 envelope ruling.
7. §9 Scope ledger summary: +1 row `SOW-077` (mapped PKG-07 / DEL-07-09 /
   OBJ-006, OBJ-015 / DecisionRef DEC-094).
8. §10 Coverage and telemetry: `ScopeItemCount` 76 → 77;
   `DeliverableCount` 101 → 102; `ContextEnvelopeCounts`
   `S=9, M=69, L=23, XL=0` → `S=9, M=69, L=24, XL=0`; `Revision`
   `0.11 — 2026-07-27` → `0.12 — 2026-08-20`. (§10 carries no separate
   context-row count metric; the 101 → 102 context-row delta is realized in
   `ContextBudgetQA.csv` and the envelope distribution.)
9. §12 Decision log: +1 row `DEC-094` (after `DEC-093`).
10. §13 Gate posture: revision-basis sentence updated to
    "This v0.12 decomposition is the accepted current decomposition basis
    after SCA-009." (The §13 architecture-basis clause "as amended through
    SCA-008" is deliberately untouched: SCA-009 does not amend the SCA-001
    architecture basis.)

All row texts are exactly the Gate-2 candidate texts from
`Impact_Assessment.md` with only the ruled L-envelope adjustment (deliverable
row Context column, sizing-note reference to the landing ledger, and the
context-row envelope/notes); the verified PRD citation required no change.

## Exact register changes

- `ScopeLedger.csv`: +1 row `SOW-077` (77 data rows; DecisionRef
  `DEC-094`).
- `Deliverables.csv`: +1 row `DEL-07-09` (102 data rows; envelope L).
- `ContextBudgetQA.csv`: +1 row `DEL-07-09,PKG-07,L,WATCH,...` (102 data
  rows; envelope **L** per owner ruling, WATCH per the register's L-row
  convention, watch note keeps editor implementation in sibling slices).

## Exact DEL-07-03 metadata changes (bounded, lifecycle-neutral)

- `_STATUS.md`: the single R-005/R-006 Remaining bullet is re-pointed to
  the DEL-07-09 contract (before → after in the diff below); one History
  line is added; `Last Updated` advances. Lifecycle remains `IN_PROGRESS`;
  the PDU-049/R-011 item and all other text are untouched.
- `_CONTEXT.md`: one boundary-note section is added; scope coverage,
  envelope, and lifecycle values are untouched.

## Not touched by this amendment

`_Decomposition/_LATEST.md` and `_ScopeChange/_LATEST.md` (pointer
advances are Gate-5 actions, pointer-last), every other deliverable folder
(including all DEL-16-04 surfaces), all implementation trees, the PRD, all
historical snapshots, and every dependency edge of every existing
deliverable. The DEL-07-09 folder scaffold and `Dependencies.csv`
extraction are post-Gate-5 PREPARATION / TASK work, not part of this
amendment.

## Complete exact diffs

### Diff — `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`

```diff
--- a/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md
+++ b/postimages/SOFTWARE_DECOMP.md
@@ -3,7 +3,7 @@
 package_role: working_surface
 doc_kind: decomposition.software
 status: current_basis
-revision: 0.11
+revision: 0.12
 created: 2026-04-30
 refs:
   - rel: depends_on
@@ -38,6 +38,8 @@
 
 Revision v0.11 records `SCA-008`, reconciling the accepted `D-58` current-effect disposition into forward decomposition truth without rewriting `D-30`, `D-31`, `D-58`, `DEC-041`, or `DEC-063`: current reliance on the App-era `D-APP-48` / `D-30` synchronized-consumption mechanism is retired; the automation-condition mechanism remains unresolved; Piping remains outside the Root-runtime and App-harness client sets; and PRD R7 and its product outcomes remain unchanged. The amendment adds `DEC-091`, corrects one current-gate clause in `DEL-16-04/_STATUS.md`, refreshes source, traceability, context-envelope, and coverage-revision metadata, and requires a complete current SCA snapshot. It changes no package, deliverable, scope item, objective, requirement, dependency edge, lifecycle, release, or professional-reliance state and adopts no successor mechanism or repin.
 
+Revision v0.12 records `SCA-009`, the interactive operation vocabulary and tool-palette ownership amendment ruled through SCA-009 Gates 1–2: new scope item `SOW-077` and new PKG-07 deliverable `DEL-07-09` "Interactive operation vocabulary and tool palette contract" (UX_UI_SLICE, context envelope L per owner ruling), mapped to `OBJ-006` and `OBJ-015`, with the ratified two-class vocabulary (NORMATIVE-NOW / ROADMAP, including the implementation-landing ledger) in the SCA-009 `Vocabulary_Annex.md` as the DEL-07-09 coverage contract bound to the implemented operation taxonomy. The single palette surface routes exclusively through the PKG-16 structured-operation layer; the `DEL-07-03-R-005`/`R-006` ownership landing re-points to DEL-07-09; the DEL-16-04 route/support candidate-generator ownership remains reserved to a separate act. The amendment adds `DEC-094` and changes no package, existing deliverable, requirement, dependency edge, lifecycle, release, or professional-reliance state.
+
 Revision v0.8 records `SCA-005`, which propagates the accepted `D-21` /
 `DEC-056` PRD v0.2 milestone-set adoption into the forward authority pointers.
 `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` is the forward PRD authority;
@@ -180,6 +182,7 @@
 |SOW-074|IN|The product shall generate schema-compliant handoff packages with model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, and unsupported-target flags.|PRD v0.2 §16.1, FR-HAND-001 through FR-HAND-004|Handoff quality is central; SCA-004 makes stable ID maps, export manifests, loss reports, target mapping metadata, unsupported/approximated/delegated behavior reporting, CAEPIPE MBF, conservative PCF, GLB/glTF review geometry, and native JSON export packages explicit scope.|
 |SOW-075|IN|The product shall support external-prover workflow metadata without forcing a formal prover-status lifecycle, automatic professional acceptance record, or comprehensive commercial-tool result ingestion in the MVP.|PRD v0.2 §4.2, §16.2-§16.4, §23|Use flexible names, tags, notes, external references, comparison reports, and optional user-owned external harness metadata instead of hard-coded approval statuses; SCA-004 permits CAEPIPE harness records only as non-authoritative external-run evidence.|
 |SOW-076|IN|The GUI shall support design-authoring and comparison workflows, including design knowledge panels, constraint/warning panels, state/run browsers, comparison tables, and graphical comparison overlays.|PRD v0.2 §11.3, §14.5, §15.3|Expands GUI from stress input editing into design iteration and review.|
+|SOW-077|IN|The GUI shall provide a ratified interactive model-building operation vocabulary and a governing tool-palette contract that organizes the human command surface for building and modifying the piping model, with every palette command routed through structured model operations.|PRD v0.3 §14; SCA-009 owner request 2026-08-20|Vocabulary classes NORMATIVE-NOW / ROADMAP per SCA-009 D3 (Vocabulary_Annex); binds to the implemented operation taxonomy; single palette owner DEL-07-09; no second mutation route.|
 
 ## 5. Objectives
 
@@ -191,7 +194,7 @@
 |OBJ-003|Implement a robust global centerline/frame solver for practical piping flexibility analysis.|SOW-005, SOW-006, SOW-011, SOW-012, SOW-013, SOW-014, SOW-015, SOW-035, SOW-052, SOW-053|DEL-04-01, DEL-04-02, DEL-04-03, DEL-04-04, DEL-04-05, DEL-04-06, DEL-05-01, DEL-05-02, DEL-05-03, DEL-05-05, DEL-11-03|
 |OBJ-004|Support piping-specific components and private libraries without bundling protected data.|SOW-007, SOW-008, SOW-009, SOW-010, SOW-017, SOW-018, SOW-019, SOW-044, SOW-051|DEL-03-01, DEL-03-02, DEL-03-03, DEL-03-04, DEL-03-05, DEL-03-06, DEL-03-07, DEL-03-08|
 |OBJ-005|Provide user-defined rule packs that evaluate solver results against user-owned design bases.|SOW-004, SOW-014, SOW-016, SOW-042, SOW-045, SOW-047|DEL-05-02, DEL-05-04, DEL-06-01, DEL-06-02, DEL-06-03, DEL-06-04, DEL-06-05|
-|OBJ-006|Provide a GUI workflow that makes model creation, missing data, results, and assumptions visible.|SOW-020, SOW-021, SOW-022, SOW-023, SOW-036, SOW-055|DEL-07-01, DEL-07-02, DEL-07-03, DEL-07-04, DEL-07-05, DEL-07-06, DEL-07-07|
+|OBJ-006|Provide a GUI workflow that makes model creation, missing data, results, and assumptions visible.|SOW-020, SOW-021, SOW-022, SOW-023, SOW-036, SOW-055, SOW-077|DEL-07-01, DEL-07-02, DEL-07-03, DEL-07-04, DEL-07-05, DEL-07-06, DEL-07-07, DEL-07-09|
 |OBJ-007|Generate reproducible reports and result exports suitable for professional review.|SOW-024, SOW-039, SOW-043, SOW-046, SOW-055|DEL-07-05, DEL-07-07, DEL-08-01, DEL-08-02, DEL-08-03, DEL-08-04, DEL-08-05, DEL-08-06, DEL-17-05, DEL-17-06|
 |OBJ-008|Maintain rigorous verification, validation, regression testing, and release gates.|SOW-026, SOW-027, SOW-032, SOW-053, SOW-054|DEL-04-05, DEL-04-06, DEL-09-01, DEL-09-02, DEL-09-03, DEL-09-04, DEL-09-05, DEL-10-04, DEL-10-05, DEL-11-04|
 |OBJ-009|Enable interoperability and extensibility while preserving governance boundaries.|SOW-030, SOW-031, SOW-038, SOW-049, SOW-054|DEL-02-04, DEL-08-04, DEL-10-01, DEL-10-02, DEL-10-03, DEL-10-04, DEL-10-05, DEL-17-01, DEL-17-02, DEL-17-03, DEL-17-04, DEL-17-05, DEL-17-07, DEL-17-08, DEL-17-09|
@@ -200,7 +203,7 @@
 |OBJ-012|Ensure unit-safe, deterministic, and reproducible model data flow across persistence, solving, rule evaluation, automation, and reporting.|SOW-025, SOW-041, SOW-050, SOW-051, SOW-052, SOW-053, SOW-054|DEL-02-01, DEL-02-02, DEL-02-05, DEL-03-08, DEL-04-06, DEL-05-05, DEL-08-02, DEL-10-05|
 |OBJ-013|Establish a cohesive software architecture spine for UI, backend services, persistence, jobs, APIs, diagnostics, and software-quality gates before package-level implementation proceeds.|SOW-056, SOW-057, SOW-058, SOW-059, SOW-060, SOW-061, SOW-062, SOW-063|DEL-00-01, DEL-00-02, DEL-00-03, DEL-00-04, DEL-00-05, DEL-00-06, DEL-00-07, DEL-00-08|
 |OBJ-014|Provide a schema-backed piping design model that captures physical design context, design knowledge, constraints, assumptions, and analytical transformation traceability.|SOW-064, SOW-065, SOW-066, SOW-067, SOW-068|DEL-02-01, DEL-13-01, DEL-13-02, DEL-13-03, DEL-13-04|
-|OBJ-015|Support controlled model authoring where GUI and agent changes become validated, reviewable, auditable model operations.|SOW-069, SOW-070, SOW-076|DEL-07-08, DEL-16-01, DEL-16-02, DEL-16-03, DEL-16-04|
+|OBJ-015|Support controlled model authoring where GUI and agent changes become validated, reviewable, auditable model operations.|SOW-069, SOW-070, SOW-076, SOW-077|DEL-07-08, DEL-07-09, DEL-16-01, DEL-16-02, DEL-16-03, DEL-16-04|
 |OBJ-016|Manage immutable model states, analysis runs, and deterministic comparisons as first-class product records for design iteration and review.|SOW-071, SOW-072, SOW-073, SOW-076|DEL-07-08, DEL-08-06, DEL-14-01, DEL-14-02, DEL-14-03, DEL-14-04, DEL-14-05|
 |OBJ-017|Produce traceable handoff packages for downstream modeling and professional validation workflows without creating automatic professional approval states.|SOW-074, SOW-075|DEL-08-06, DEL-15-01, DEL-15-02, DEL-15-03, DEL-15-04, DEL-17-01, DEL-17-02, DEL-17-03, DEL-17-04, DEL-17-05, DEL-17-06, DEL-17-07, DEL-17-08, DEL-17-09|
 |OBJ-018|Preserve professional and IP boundaries across design knowledge, constraints, operations, comparisons, reports, handoff packages, export workflows, and agent rationale.|SOW-064, SOW-067, SOW-069, SOW-073, SOW-074, SOW-075|DEL-01-04, DEL-08-06, DEL-12-02, DEL-15-04, DEL-16-04, DEL-17-01, DEL-17-04, DEL-17-05, DEL-17-06, DEL-17-07, DEL-17-09|
@@ -217,7 +220,7 @@
 |PKG-04|Solver Core and Numerical Methods|Implements global 3D centerline/frame mechanics, straight pipe behavior, supports, nonlinear support logic, diagnostics, and performance harnesses.|SOW-005, SOW-006, SOW-011, SOW-012, SOW-035, SOW-053|Does not decide code compliance; produces mechanical results.|
 |PKG-05|Loads, Load Cases, and Stress Recovery|Implements primitive loads, concentrated/distributed user loads, load-case algebra, mechanical stress recovery, and analysis-status semantics.|SOW-013, SOW-014, SOW-015, SOW-047, SOW-052|Does not contain proprietary code load combinations or allowables.|
 |PKG-06|Rule Packs and User-Supplied Code Check Engine|Implements the schema, sandboxed evaluator, required-input checks, and private lifecycle for rule packs.|SOW-004, SOW-016, SOW-042, SOW-045|Does not ship ASME or other proprietary rule content.|
-|PKG-07|Graphical User Interface and Engineering Workflow|Implements the interactive modeler, editors, warning UX, solve-execution UX, and results views.|SOW-020, SOW-021, SOW-022, SOW-023, SOW-036, SOW-055, SOW-076|Does not silently supply missing code data.|
+|PKG-07|Graphical User Interface and Engineering Workflow|Implements the interactive modeler, editors, warning UX, solve-execution UX, and results views.|SOW-020, SOW-021, SOW-022, SOW-023, SOW-036, SOW-055, SOW-076, SOW-077|Does not silently supply missing code data.|
 |PKG-08|Reporting, Audit, and Reproducibility|Implements calculation reports, audit manifests, hashes, report-content guardrails, and exports.|SOW-024, SOW-039, SOW-043, SOW-046|Does not authenticate or certify engineering work.|
 |PKG-09|Verification, Validation, and Quality Oracles|Implements mechanics benchmarks, regression suites, validation manual structure, and release quality gates.|SOW-026, SOW-027|Does not replace professional review for project-specific reliance.|
 |PKG-10|Build, Packaging, API, and Interoperability|Implements public API/plugin boundaries, import/export adapters, headless execution, local FEA handoff contracts, and release packaging.|SOW-030, SOW-031, SOW-032, SOW-049, SOW-054|Does not embed external proprietary tool behavior.|
@@ -320,6 +323,7 @@
 |DEL-07-06|Accessibility and usability baseline|UX_UI_SLICE|SOW-036|OBJ-006|M|May refine once GUI framework chosen.|
 |DEL-07-07|Solve execution UX: progress, cancellation, and diagnostics|UX_UI_SLICE|SOW-055|OBJ-006,OBJ-007|M|Single GUI workflow slice for running and inspecting solves.|
 |DEL-07-08|Design-authoring state and comparison workspace|UX_UI_SLICE|SOW-076|OBJ-015,OBJ-016|L|Adds design knowledge panels, operation/diff review, state/run browsers, comparison tables, and graphical overlays to the GUI workflow.|
+|DEL-07-09|Interactive operation vocabulary and tool palette contract|UX_UI_SLICE|SOW-077|OBJ-006,OBJ-015|L|Owns the ratified two-class operation-vocabulary coverage contract (SCA-009 Vocabulary_Annex incl. implementation-landing ledger) and the single tool-palette surface organization over the DEL-07-01/DEL-07-02 surfaces; all palette commands route through the PKG-16 operation layer; accepted landing zone for the DEL-07-03-R-005/R-006 editor residuals.|
 
 ### PKG-08 — Reporting, Audit, and Reproducibility
 
@@ -539,22 +543,23 @@
 |SOW-074|IN|The product shall generate schema-compliant handoff packages with model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, and unsupported-target flags.|PRD v0.2 §16.1, FR-HAND-001 through FR-HAND-004|PKG-15, PKG-17|DEL-15-01, DEL-15-02, DEL-15-03, DEL-17-01, DEL-17-02, DEL-17-03, DEL-17-04, DEL-17-06, DEL-17-07, DEL-17-08, DEL-17-09|OBJ-017, OBJ-018|FALSE|Handoff quality is central; SCA-004 makes stable ID maps, export manifests, loss reports, target mapping metadata, unsupported/approximated/delegated behavior reporting, CAEPIPE MBF, conservative PCF, GLB/glTF review geometry, and native JSON export packages explicit scope.|
 |SOW-075|IN|The product shall support external-prover workflow metadata without forcing a formal prover-status lifecycle, automatic professional acceptance record, or comprehensive commercial-tool result ingestion in the MVP.|PRD v0.2 §4.2, §16.2-§16.4, §23|PKG-15, PKG-17|DEL-15-04, DEL-17-01, DEL-17-04, DEL-17-05, DEL-17-09|OBJ-017, OBJ-018|FALSE|Use flexible names, tags, notes, external references, comparison reports, and optional user-owned external harness metadata instead of hard-coded approval statuses; SCA-004 permits CAEPIPE harness records only as non-authoritative external-run evidence.|
 |SOW-076|IN|The GUI shall support design-authoring and comparison workflows, including design knowledge panels, constraint/warning panels, state/run browsers, comparison tables, and graphical comparison overlays.|PRD v0.2 §11.3, §14.5, §15.3|PKG-07|DEL-07-08|OBJ-015, OBJ-016|FALSE|Expands GUI from stress input editing into design iteration and review.|
+|SOW-077|IN|The GUI shall provide a ratified interactive model-building operation vocabulary and a governing tool-palette contract that organizes the human command surface for building and modifying the piping model, with every palette command routed through structured model operations.|PRD v0.3 §14; SCA-009 owner request 2026-08-20|PKG-07|DEL-07-09|OBJ-006, OBJ-015|FALSE|Vocabulary classes NORMATIVE-NOW / ROADMAP per SCA-009 D3; single palette owner DEL-07-09; no second mutation route.|
 
 ## 10. Coverage and telemetry
 
 
 |Metric|Value|
 |---|---|
-|ScopeItemCount|76|
+|ScopeItemCount|77|
 |PackageCount|18|
-|DeliverableCount|101|
+|DeliverableCount|102|
 |ObjectiveCount|18|
 |UnassignedScopeItems|0|
 |ScopeItemsWithoutDeliverableMapping|0|
 |UnmappedObjectives|0|
-|ContextEnvelopeCounts|S=9, M=69, L=23, XL=0|
+|ContextEnvelopeCounts|S=9, M=69, L=24, XL=0|
 |OpenIssues|17|
-|Revision|0.11 — 2026-07-27|
+|Revision|0.12 — 2026-08-20|
 
 ## 11. Open issues
 
@@ -677,9 +682,10 @@
 |DEC-091|D-58 current-effect reconciliation: preserve `D-30`, `D-31`, `D-58`, `DEC-041`, and `DEC-063` as historical acts while retiring current reliance on the App-era `D-APP-48` / `D-30` synchronized-consumption mechanism. That mechanism no longer supplies a current basis for the `D-31` declaration that the `DEC-041` no-manual-toil automation condition is met; the automation-condition mechanism remains unresolved. Piping remains outside the Root-runtime and App-harness client sets; PRD R7 and its product outcomes remain unchanged; and no successor mechanism, repin, client obligation, package, deliverable, scope item, objective, requirement, dependency edge, lifecycle, release, or professional-reliance effect is adopted.|`execution/_Coordination/_DECISIONS/D-58_current_mechanism_retirement.md`; `D-58_RULING_2026-07-27.md`; `D-58_EFFECTIVE_STATE_CLOSEOUT_2026-07-27.md`; D-58 decision-register row; SCA-008 Gate-1 confirmation and accepted Gate-2 impact assessment.|Accepted as the forward current-effect row through SCA-008 Gate 3, effective in decomposition revision 0.11 only after separately approved Gate-5 application and validation. Historical rows remain byte-identical; the exact automation-condition mechanism remains unresolved; no product, runtime, client, lifecycle, release, or professional authority follows.|
 |DEC-092|D-45 ruling — temperature-indexed shear modulus: adopt **Option O-B** from `execution/_Coordination/_DECISIONS/D-45_temperature_indexed_shear_modulus.md`. Add a new explicit user-entered shear-modulus G quantity to material temperature points and apply the `DEC-077` method to G: exact-id basis selection remains available; a declared solve temperature linearly interpolates G only between qualifying adjacent user-entered points; every derived G value carries provenance naming both source points and the declared method; and the solve blocks rather than extrapolates when a qualifying bracket is absent or the temperature is at or beyond stored range edges. Base G remains the no-temperature-basis value; a selected temperature basis must consume explicit point G without hidden base-G fallback. All values remain user-entered; no material curve, catalog, code table, protected value, or default is authorized.|Human project authority ruling on 2026-08-01: `2) D-45 ruling: O-B.` Exact 20 UTF-8 bytes without trailing newline, SHA-256 `f4a43267fc5c7e16ed9de5ee736520eedd518866d66d12d74860b279d1e20557`; durable record `execution/_Coordination/_DECISIONS/D-45_RULING_2026-08-01.md`; pre-ruling proposal SHA-256 `c7e2cb465df438c413e56beaaeb1e13fdbf85bb46e88b400c01ef1697590d371`.|Accepted; D-45 register row is `RULED` with packet, ruling, and this row as pointers. Implementation remains a separately bounded DEL-05-02 tranche behind schema/authoring/operation/solver, independent hand-calculation, benchmark, unit, exact-id, provenance, mutual-selection, missing/invalid-input, and edge/no-extrapolation evidence. At codification, live temperature-point inputs still contain E and alpha but no G, and the solve still clones base G; no implementation or lifecycle, stage, release-readiness, professional, certification, sealing, authentication, or code-compliance claim is made.|
 |DEC-093|D-65 ruling — CI-produced surface-4 evidence accepted with binding. Prospectively amend and succeed only the surface-4 execution alternative of `DEC-025`: a five-surface sweep may satisfy surface 4 with evidence from `.github/workflows/piping-desktop-e2e.yml` when its summary records `execution_capability: ci`, the workflow path, Actions run ID and run attempt, exact run head SHA, run conclusion, and `working_tree_dirty=false`; the bound head SHA must equal the sweep `commit_hash`, the conclusion must be `success`, and the registered e2e specs must have executed across both registered viewport projects. The existing `execution_capability: host` path remains permitted with unchanged semantics and is not deprecated. Exact-SHA CI checkout satisfies the clean-tree intent by construction. An infrastructure-failed run may be replaced by a successful rerun of the same workflow on the same SHA; a spec failure remains a genuine blocking surface-4 failure. Acceptance covers the current registered desktop e2e surface only; a future macOS-specific packaged-desktop surface may be designated host-mandatory when registered. Exploratory and agent-as-user testing are excluded from all sweep surfaces. `DEC-025` remains immutable history and governs except where this row expressly supplies the CI alternative.|Human project authority ruling on 2026-08-19, recorded verbatim in `execution/_Coordination/_DECISIONS/D-65_RULING_2026-08-19.md`.|Accepted as the forward surface-4 execution rule. The separately bounded DEL-10-04 Remaining item must teach `tools/release/run_evidence_sweep.py` and its summary schema/validator to record and validate the CI binding, including rejecting a bound head SHA that differs from `commit_hash`; that engineering is not executed here. Until it lands, sweeps continue on the host path. No workflow, sweep tool, schema, validator, manifest, lifecycle, release, publication, or professional-reliance change occurs in this recording tranche.|
+|DEC-094|SCA-009 ruling — interactive operation vocabulary and tool-palette ownership: adopt Option A from the SCA-009 candidate package. Add SOW-077 and DEL-07-09 "Interactive operation vocabulary and tool palette contract" (PKG-07, UX_UI_SLICE, context envelope L per owner ruling), mapped to OBJ-006 and OBJ-015. The ratified two-class vocabulary (NORMATIVE-NOW / ROADMAP, with the implementation-landing ledger) in `execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md` is the DEL-07-09 coverage contract and binds to the implemented operation taxonomy, not the schema OperationKind enum. Single palette-surface owner DEL-07-09; every palette command routes through the PKG-16 structured-operation layer. DEL-07-03-R-005/R-006 ownership landing re-points to DEL-07-09; the DEL-16-04 route/support candidate-generator ownership remains reserved to a separate act. No package, existing deliverable, requirement, dependency edge, lifecycle, release, or professional-reliance state changes.|Human project authority rulings on 2026-08-20, transcribed in the SCA-009 ACCEPTANCE_RECORD: Gate 1 "D1 is confirmed."; Gate 2 "Yes, add the landing column and rule the envelope L.  On that, Gate 2 is approved and you may proceed."|Accepted through SCA-009 Gates 1–2; exact application governed by Gates 3–5. Downstream DAG rebuild, targeted reconciliation refresh, DEL-07-09 dependency extraction, and PREPARATION scaffold remain explicit post-amendment work.|
 
 ## 13. Gate posture
 
-This v0.11 decomposition is the accepted current decomposition basis after SCA-008.
+This v0.12 decomposition is the accepted current decomposition basis after SCA-009.
 
 PREPARATION may scaffold package and deliverable folders from this decomposition and the companion registers after ORCHESTRATOR plans the accepted downstream refreshes. `PKG-00` is retained architecture-basis reference context per HUMAN-STEER-PKG00-EXCLUSION-001 and `D-43`, not a production package awaiting issuance, and its SCA-001 architecture basis as amended through SCA-008 may be injected into `PKG-01` through `PKG-17` sealed contexts and future TASK briefs. Type 2 execution still requires one sealed deliverable context, explicit write scope, applicable invariants, and acceptance criteria. Existing DEV-001 dispatch, immutable DAG snapshots, lifecycle history, implementation evidence, schemas, docs, and code may predate revision 0.11 until refreshed by their owning workflows; SCA-008 does not itself approve a DAG successor, adopt an automation mechanism or client status, change lifecycle, create implementation authority, or make release/professional claims. `D-30`, `D-31`, `D-58`, `DEC-041`, and `DEC-063` remain historical records; `D-58` and `DEC-091` govern current effect.
```

### Diff — `projects/chirality-piping/docs/_Registers/ScopeLedger.csv`

```diff
--- a/projects/chirality-piping/docs/_Registers/ScopeLedger.csv
+++ b/postimages/ScopeLedger.csv
@@ -75,3 +75,4 @@
 SOW-074,IN,"The product shall generate schema-compliant handoff packages with model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, and unsupported-target flags.","PRD v0.2 §16.1, FR-HAND-001 through FR-HAND-004","PKG-15, PKG-17","DEL-15-01, DEL-15-02, DEL-15-03, DEL-17-01, DEL-17-02, DEL-17-03, DEL-17-04, DEL-17-06, DEL-17-07, DEL-17-08, DEL-17-09","OBJ-017, OBJ-018","DEC-014, DEC-018",FALSE,"Handoff quality is central; SCA-004 makes stable ID maps, export manifests, loss reports, target mapping metadata, unsupported/approximated/delegated behavior reporting, CAEPIPE MBF, conservative PCF, GLB/glTF review geometry, and native JSON export packages explicit scope."
 SOW-075,IN,"The product shall support external-prover workflow metadata without forcing a formal prover-status lifecycle, automatic professional acceptance record, or comprehensive commercial-tool result ingestion in the MVP.","PRD v0.2 §4.2, §16.2-§16.4, §23","PKG-15, PKG-17","DEL-15-04, DEL-17-01, DEL-17-04, DEL-17-05, DEL-17-09","OBJ-017, OBJ-018","DEC-015; DEC-016, DEC-018",FALSE,"Use flexible names, tags, notes, external references, comparison reports, and optional user-owned external harness metadata instead of hard-coded approval statuses; SCA-004 permits CAEPIPE harness records only as non-authoritative external-run evidence."
 SOW-076,IN,"The GUI shall support design-authoring and comparison workflows, including design knowledge panels, constraint/warning panels, state/run browsers, comparison tables, and graphical comparison overlays.","PRD v0.2 §11.3, §14.5, §15.3",PKG-07,DEL-07-08,"OBJ-015, OBJ-016",DEC-014,FALSE,Expands GUI from stress input editing into design iteration and review.
+SOW-077,IN,"The GUI shall provide a ratified interactive model-building operation vocabulary and a governing tool-palette contract that organizes the human command surface for building and modifying the piping model, with every palette command routed through structured model operations.","PRD v0.3 §14; SCA-009 owner request 2026-08-20",PKG-07,DEL-07-09,"OBJ-006, OBJ-015",DEC-094,FALSE,Vocabulary classes NORMATIVE-NOW / ROADMAP per SCA-009 D3; single palette owner DEL-07-09; no second mutation route.
```

### Diff — `projects/chirality-piping/docs/_Registers/Deliverables.csv`

```diff
--- a/projects/chirality-piping/docs/_Registers/Deliverables.csv
+++ b/postimages/Deliverables.csv
@@ -100,3 +100,4 @@
 DEL-17-07,PKG-17,Conservative PCF subset exporter,Implement a conservative PCF subset exporter for broader plant-design interoperability with explicit unsupported/approximated behavior reporting and no reliance on hidden translator defaults.,BACKEND_FEATURE_SLICE,PCF subset profile; PCF writer; unsupported behavior report; invented fixtures,"SOW-030,SOW-074","OBJ-009,OBJ-017,OBJ-018",L,PCF is useful for interoperability but not the first validation backbone; translator behavior remains explicitly bounded.
 DEL-17-08,PKG-17,GLB/glTF review geometry export,"Implement GLB/glTF review geometry export for lightweight visual inspection of model geometry, stable identity, and export package review context.",BACKEND_FEATURE_SLICE,glTF/GLB export profile; review geometry writer; ID sidecar; geometry fixtures,"SOW-030,SOW-074","OBJ-009,OBJ-017",M,"Review geometry is visual handoff support only, not solver geometry or professional validation."
 DEL-17-09,PKG-17,Export adapter SDK and additional targets,"Define and implement the export adapter SDK surface for community/additional targets while preserving validation, provenance, diagnostics, stable IDs, and public/private data boundaries.",API_CONTRACT,export adapter SDK contract; adapter template; target registry contract; validation checklist,"SOW-030,SOW-074,SOW-075","OBJ-009,OBJ-017,OBJ-018",L,Adapter extensibility target sequenced after common profile and package contracts.
+DEL-07-09,PKG-07,Interactive operation vocabulary and tool palette contract,"Own the ratified two-class interactive operation vocabulary coverage contract and the single tool-palette surface organization over the viewport and tree/inspector surfaces, routed through the structured model operation layer.",UX_UI_SLICE,vocabulary coverage ledger; palette organization contract; palette-to-operation routing map,SOW-077,"OBJ-006,OBJ-015",L,Owns the SCA-009 ratified vocabulary contract and palette organization; accepted landing zone for the DEL-07-03-R-005/R-006 editor residuals.
```

### Diff — `projects/chirality-piping/docs/_Registers/ContextBudgetQA.csv`

```diff
--- a/projects/chirality-piping/docs/_Registers/ContextBudgetQA.csv
+++ b/postimages/ContextBudgetQA.csv
@@ -100,3 +100,4 @@
 DEL-17-07,PKG-17,L,WATCH,Confirm scope and split if it expands,PCF is useful for interoperability but not the first validation backbone; translator behavior remains explicitly bounded.
 DEL-17-08,PKG-17,M,OK,Proceed with bounded Type 2 brief,"Review geometry is visual handoff support only, not solver geometry or professional validation."
 DEL-17-09,PKG-17,L,WATCH,Confirm scope and split if it expands,Adapter extensibility target sequenced after common profile and package contracts.
+DEL-07-09,PKG-07,L,WATCH,Confirm scope and split if it expands,Contract/coverage slice owning the SCA-009 ratified operation vocabulary and single palette surface; envelope L per owner ruling; watch scope if editor implementation (R-005/R-006 successors) lands here rather than in sibling slices.
```

### Diff — `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/_STATUS.md`

```diff
--- a/projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/_STATUS.md
+++ b/postimages/DEL-07-03_STATUS.md
@@ -1,13 +1,14 @@
 # Status: DEL-07-03 Material, component, and rule-pack editors
 
 **Current State:** IN_PROGRESS
-**Last Updated:** 2026-07-12
+**Last Updated:** 2026-08-20
 
 ## Remaining
-- Preserve PDU-041 / `DEL-07-03-R-005` and `R-006` as documented GUI absences until an accepted ownership/scope binding authorizes load-case and support/restraint editors in this deliverable; adjacent DEL-07-02/other GUI authoring does not silently close them.
+- `DEL-07-03-R-005` and `R-006` (load-case, support/restraint editors) are re-pointed by SCA-009 (decomposition revision 0.12, DEC-094): their accepted ownership landing is the `DEL-07-09` interactive operation vocabulary and tool palette contract. They remain documented GUI absences on this deliverable's surfaces until DEL-07-09-governed coverage work closes them through the owning implementation deliverables; adjacent DEL-07-02/other GUI authoring still does not silently close them. PDU-041 evidence is preserved unchanged.
 - Obtain a separately authorized independent usability/security validation basis before upgrading PDU-049 / `DEL-07-03-R-011` beyond `VERIFIED_NOT_VALIDATED`; project-owned verification does not supply that basis (source: DEC-074 O7-before-E5; PDU-049, 2026-07-12)
 
 ## History
+- 2026-08-20 - SCA-009 (decomposition revision 0.12, DEC-094) re-pointed the R-005/R-006 ownership landing to DEL-07-09 per the owner's D5 ruling; documented absences and PDU-041 evidence preserved; no lifecycle change (state remains IN_PROGRESS).
 - 2026-07-12 - D-41 R5 T5 PDU-008 recorded current cross-surface GUI interaction evidence while preserving PDU-041's DEL-07-03 load-case/support editor ownership absences; no silent closure or lifecycle change.
 - 2026-07-12 - D-41 R5 T5/PDU-041 added focused blocking evidence for unsupported DEL-07-03 load-case/support editor kinds and preserved the absence without scope creation; lifecycle remains IN_PROGRESS.
 - 2026-04-30 - State set to OPEN (PREPARATION)
```

### Diff — `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/_CONTEXT.md`

```diff
--- a/projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/_CONTEXT.md
+++ b/postimages/DEL-07-03_CONTEXT.md
@@ -54,6 +54,9 @@
 - **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
 - **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.
 
+## Boundary Note (SCA-009)
+- SCA-009 (decomposition revision 0.12, DEC-094) assigns the ownership landing for `DEL-07-03-R-005`/`R-006` (load-case, support/restraint editors) to `DEL-07-09` — the PKG-07 interactive operation vocabulary and tool palette contract. This deliverable's scope coverage, envelope, and lifecycle are unchanged; the WATCH advice to split rather than expand stands. Boundary metadata only.
+
 ## PREPARATION Notes
 - Structural scaffold only.
 - No Type 2 implementation artifacts are drafted in this folder by PREPARATION.
```

## Gate-3 decision

The owner is asked to approve exactly this preimage/postimage set (cite
this `Amendment_Preview.md` by its SHA-256). Approval opens Gate 4
(propagation plan) only; nothing is applied at Gate 3.
