# SWBPIPE completeness and reconciliation map

**Basis:** `origin/main` at `7a780551f0` (2026-10-23). This is a replaceable
current-state record for the owner and the work plan. Replace it when the
basis moves materially; do not append to it.

It answers three questions. Which intended outcomes does the product deliver
today? Which deliverables' documents have drifted from the code? What must come
before development testing, the owner's hands-on use and validation? It is not a
release assessment. Classifications are facts read from code, tests and one
hands-on run (§5). They are not acceptance: acceptance, issuance and release
remain the owner's (F-PIP-3).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Classes**
- **D:** delivered, with evidence (code a user reaches, and tests).
- **R:** delivered, but the deliverable's ScopeOfWork, Design or
  `deliverable.yaml` does not match the code.
- **P:** partial or missing.
- **O:** out of scope for v0.1 by a cited owner ruling.

A Must requirement met only on one solve route, only headless, or only as a
schema is P. Where PRD §23 (MVP) or a Must requirement includes something and no
ruling removes it, the row says **scope unclear**.

The "scope unclear" notes and the "no ruling" statements in §1 and §2 are
the assessment before the owner's rulings of 2026-10-23. §4.2 records those
rulings (OD-1 to OD-5), and they take precedence over the earlier notes.

**Two solve routes** recur throughout:
- **PF:** the pressure-free route, a model with no `pressure_contract`.
- **EX:** the exact pressure route, `3.0.0/exact_pressure_v3`.

## 1. What a user can do today

Working in the macOS desktop app on invented data, a user can do the following.
The native smoke and the probe in §5 exercised this path in the real app.

- **Projects:** open a model document, or start a blank or local project.
- **Authoring:** place nodes and pipes in the 3D viewport, assign materials and
  sections, and add bends, expansion joints, supports, load cases and
  combinations. Every edit is a validated, diffed and undoable operation.
- **Solving:** solve on either route. PF covers linear and nonlinear supports,
  wind, seismic and combinations. EX covers pressure, load/reference states and
  objective-connector joints.
- **Results:** read displacements, element-local forces, reactions and
  stresses, each with a numerical-integrity standing, and view the deformed
  shape.
- **Saving:** save the project to the local store and reopen it.
- **Rule packs:** author a private rule pack and run it against results. Missing
  inputs block pass/fail.
- **Libraries:** import private material, section and component libraries.
- **Exports:** export result JSON and stress-neutral JSON on both routes. PF
  results also export the PCF and MBF package documents (redacted), a native
  package review manifest and a preview handoff JSON. The byte-exact
  downstream outputs (stress-neutral CSV, glTF review geometry, PCF text and
  MBF text) are withheld by the desktop: their routes require lossless
  materialization and session values are private
  (`ControlledExportLink.tsx:36-48`).

The biggest gaps:

1. **No report a user can obtain.** For a pressure-free session, Render gives
   `blocked=true` (18,031 redaction decisions), with no preview and no hash
   (§5). Report inputs are classified private, and the render runs under the
   public-report redaction context with no local-private intent control.
   - Exact-route results are refused for every report (owner ruling 10A,
     2026-10-21).
   - Save Report Package refuses every current result.
   - Affected: FR-REP-001 (Must) and MVP "basic report generation".
2. **No model states, run history or comparison between states or runs.**
   FR-CMP-001..004 are Must and §23 MVP items. They exist as schemas and
   headless Python engines only. No ruling says explicitly whether software
   0.1.0 includes them. Two texts point in different directions:
   - The owner-adopted issuance-wave ruling D-11 (DEC-062, 2026-07-04) places
     PKG-14 in "W7 — Forward horizon v0.2" (Phase G). DEL-14-01:3 marks the
     program "stage-gated: v0.2 R3".
   - PRD §23 still lists these items in the MVP. The "v0.2" in D-11 names the
     PRD v0.2 milestone set, not a software version.

   The "Not in v0.1" text in the release-notes draft
   (`docs/RELEASE_NOTES_v0.1.0_DRAFT.md:118-121`, commit 91eb15e09e) is an
   agent's reading, not a ruling. The same W7 placement covers PKG-13 (design
   knowledge), PKG-15 (handoff), PKG-16 (agent) and PKG-17 (exports).
3. **No rigid elements.** Valves, flanges, reducers and rigid or specialty
   components are analysed as plain pipe on PF and refused on EX.
   - DEC-045 (owner, 2026-06-20) chose geometry plus user stiffness scaling,
     but the scaling is never applied.
   - Affected: FR-SOL-004 (Must) and the MVP/R1 "rigid elements" item.
4. **No canonical model export.** The desktop has no save-model-to-file command.
   The handoff download is a preview document that does not conform to
   `schemas/handoff_package.schema.json`. Nothing is exported for pressure
   models. Affected: FR-HAND-001 (Must) and the MVP item.
5. **One model format, missing slots.** Since #1554, `schemas/model.schema.yaml`
   describes the desktop's model document (0.6.0), and the bundled demo validates
   against it. The former shape moved to
   `schemas/physical_transform_model.schema.yaml`. Tags, notes, unresolved
   assumptions and rule-pack references have no slot. Affected:
   FR-MOD-001/004/006 (Must).
6. **No design-knowledge entry or constraint validation in the desktop.** The
   Design Knowledge panel shows a compiled-in document, and constraint
   validation is Python-only. Affected: FR-KNOW-001 (Must) and R6.
7. **No end-to-end proof of R2.** No single automated or human run takes a
   model authored in the GUI through native solve, review, save and reopen
   (DEC-047 bypassed the human packaged pass).
8. **Documents trail the code.** Most PKG-07, PKG-08, PKG-06 and PKG-17
   `deliverable.yaml` pointers name Python or Rust reference modules the
   desktop never calls, and omit the desktop code that delivers the outcome.
   The validation manual's runnable reproduction procedure is out of date (Part 1 is a frozen procedure whose dated notes say its commands now need a flag or are refused). Several
   ScopeOfWork files still say "not implemented in this setup" for code that
   ships (§3).

## 2. Outcome table

Each row gives the PRD requirement (`docs/PRD.md` line), its priority and
class, the evidence, and the deliverables.

### 2.1 Model, schema, design knowledge (PRD §11.1–11.2, §13)

| ID (PRD line) | Pri | Class | Evidence | Deliverables | Notes |
|---|---|---|---|---|---|
| FR-MOD-001 (476) | Must | P | **Store:** `apps/desktop/src-tauri/src/lib.rs:2452` save, `:1189` load. **Tests:** `lib.rs:6270,6325,7552`; `e2e/b3b-project-persistence.spec.ts`. **Desktop:** File › New/Save/Open Local Project, Open Model Document | DEL-02-05, DEL-02-01, DEL-00-04 | Model, units and provenance strings round-trip. Tags, notes, assumptions and rule-pack refs have no slot. No save-to-file. The product document fails the canonical schema. |
| FR-MOD-002 (477) | Must | P | Typed entities in `core/product_physics/src/lib.rs:366,448,577` | DEL-02-01 | Lines, constraints and equipment interfaces are not entities in the product document. |
| FR-MOD-003 (478) | Must | D | Identity changes refused: `core/model_operations/operation_applier/src/rich_authoring.rs:1092`; test `:864`; `atomic_batch.rs:690` | DEL-02-01, DEL-16-06 | No all-operations ID-survival test. |
| FR-MOD-004 (479) | Must | P | Only `project.name` and entity `label` | DEL-02-01 | No tags or notes. |
| FR-MOD-005 (480) | Must | P | Library records carry structured provenance: `core/library_import/library_import_document/src/lib.rs:339-402` | DEL-03-01, DEL-03-07 | Model entities have one free provenance string. |
| FR-MOD-006 (481) | Must | P | `unresolved_assumptions` exists only in `schemas/physical_transform_model.schema.yaml` (the headless transform's format) | DEL-02-01, DEL-13-01 | No slot in the desktop document, and no entry path. |
| FR-MOD-007 (482) | Must | P | **Python transform:** `core/model_transform/physical_to_analytical/contract.py:113`, used by tests only. **Desktop:** solves the document directly in Rust (`preview_physics.rs`) | DEL-13-04 | No transformation-warning record on the solve path; holds DEL-13-04:2/3 apply. |
| FR-KNOW-001 (488) | Must | P | `lib.rs:1773` returns a compiled-in fixture; `KnowledgePanel.tsx` is read-only | DEL-13-01 | **Scope unclear:** Must requirement and R6 deliverable, while §23 omits it and D-11 places PKG-13 in W7 ("Forward horizon v0.2"). |
| FR-KNOW-002 (489) | Must | P | Unknown references give `MODEL_INCOMPLETE` (`lib.rs:19079`). An unrestrained body is caught at solve (`lib.rs:15409`). Exact pressure regions reject branching, cycles and disconnected traversal (`pressure_runtime.rs:1535,1594`) | DEL-13-03 | No general topology check: a disconnected but separately anchored route is not flagged. |
| FR-KNOW-003 (490) | Should | P | `core/constraints/validation/engine.py`; `tests/test_constraint_validation.py` | DEL-13-03, DEL-13-02 | Python only; record checks, no geometry. |
| FR-KNOW-004 (491) | Should | P | `schemas/constraint.schema.json` | DEL-13-02 | Schema only. |
| FR-KNOW-005 (492) | Could | P | Absent (PRD §13.3 says "In future releases") | (R7) | |
| §10.1 step 1, select units | — | P | `projectService.ts:377-402` fixes SI for a blank project | DEL-02-02 | Display units switchable. |

### 2.2 Desktop GUI and operations (PRD §11.3, §11.8, §14)

| ID (PRD line) | Pri | Class | Evidence | Deliverables | Notes |
|---|---|---|---|---|---|
| R2 exit (1572) | — | P | Each step is covered somewhere (see A2 path in §5) | DEL-07-11, DEL-07-01/02/05/07 | No native run of a model authored in the GUI. |
| FR-GUI-001 (498) | Must | P | `PipeViewport.tsx:1249-1331` (pipes, nodes, supports, components, labels); load arrows `:1289`; deformed shape and rotation glyphs `:1268-1283`; `e2e/c3-viewport-visibility.spec.ts`, `rotation-glyph.spec.ts` | DEL-07-01, DEL-07-05 | No result colour overlays or reaction arrows (`viewportPalette.ts:125-137`). **Scope unclear**, no ruling. |
| FR-GUI-002 (499) | Must | P | `ModelTree.tsx:40-57` | DEL-07-02 | No lines, states or runs. |
| FR-GUI-003 (500) | Must | P | `PropertyInspector.tsx:1248,1263,1283`; `typedInspector.test.tsx` | DEL-07-02 | No dependencies, linked assumptions or dependent rule-pack inputs. These are not in DEL-07-02 RQ-002. |
| FR-GUI-004 (501) | Must | P | Create, modify and move (`operation_applier/src/lib.rs:385-404,2395-2470`), delete (`:1732,1774`); split, copy, rotate and mirror (`geometry_operations.rs`); `e2e/b4-table-editing.spec.ts` | DEL-07-01, DEL-16-06 | No reconnect: a pipe's `from`/`to` cannot be retargeted. |
| FR-GUI-005 (502) | Should | D | `workspaceSession.ts:1182-1188` (depth 25); `e2e/b3a-session-status.spec.ts` | DEL-00-05, DEL-07-11 | Session-only. |
| FR-GUI-006 (503) | Must | D | `MissingDataBlockingPanel.tsx:5-11,165-187`; `App.test.tsx:2966-3004`; `e2e/gui-workflow-validation.spec.ts:110` | DEL-07-04 | Shown in drift §3 (DEL-07-04). |
| FR-GUI-007 (504) | Should | P | `ComparisonPanel.tsx`; `previewService.ts:654,739` (`single_run_load_basis_review`) | DEL-07-08, DEL-14-05 | Within one run only. |
| FR-AGENT-001 (568) | Should | D | Offline intake `OfflineProposalIntakePanel.tsx` → `atomic_batch.rs:285-345` | DEL-16-01, DEL-16-06 | The live provider is O (D-22/DEC-041; F-PIP-4). |
| FR-AGENT-002 (569) | Should | D | `atomic_batch.rs` preflight and its tests `:717-802` | DEL-16-02, DEL-16-06 | |
| FR-AGENT-003 (570) | Should | D | Queue, review/diff, explicit Apply (`workspaceSession.ts:1180-1191`) | DEL-07-08, DEL-16-06 | No e2e for offline intake. |
| FR-AGENT-004 (571) | Should | P | Rationale and source required (`atomic_batch.rs:322-341`) | DEL-16-03, DEL-16-04 | No assumptions or constraints-considered keys. Receipts are session-only. |
| FR-AGENT-005 (572) | Must | P | Boundary flags fixed and tamper-checked (`atomic_batch.rs:196-202,427-440`) | DEL-16-04 | The prohibited-claim scanner (`agent_rationale/engine.py`) is headless only. **Scope unclear:** DEL-16-04:3 stages it to R7. |

### 2.3 Solver and mechanics (PRD §11.4, §12, §22.2–22.3)

| ID (PRD line) | Pri | Class | Evidence | Deliverables | Notes |
|---|---|---|---|---|---|
| FR-SOL-001 (510) | Must | D | `core/solver/frame_kernel/src/lib.rs:1591`; t3_r1 references | DEL-04-01 | Both routes. |
| FR-SOL-002 (511) | Must | D | `MECH-PORTAL-SWAY`, `-INCLINED-MEMBER-TRANSFORM`, `-FIXED-FIXED-THERMAL-AXIAL` (`validation/benchmarks/mechanics/src/lib.rs`) | DEL-04-01, DEL-09-01 | |
| FR-SOL-003 (512) | Must | D | `core/solver/straight_pipe`; self-weight `product_physics/src/self_weight.rs:507-515`; `STRESS-*` fixtures | DEL-04-02, DEL-05-03 | PF uses Euler–Bernoulli; EX adds Timoshenko. |
| FR-SOL-004 (513) | Must | P | No rigid element in the kernel: `frame_kernel/src/rigid_body.rs` is a rigid-motion screen, used for mechanism detection (`nonlinear_integration/src/structural_adapter.rs:1633,1761`). PF only normalizes and diagnoses (`product_physics/src/validation.rs:1247-1311`); EX refuses (`exact_admission.rs:196-210`) | DEL-03-05 (data) | Offsets exist only on connectors and tie rods (`connector.rs:12`, `tie_rod.rs:17`). DEC-045's stiffness scaling is never assembled. No rigid-link benchmark. **Scope unclear:** §23/R1. |
| FR-SOL-005 (514) | Must | D | `core/solver/linear_supports`; `lib.rs:8263,8315`; EX admits linear restraints and springs (`exact_admission.rs:6`) | DEL-04-03 | Both routes. |
| FR-SOL-006 (515) | Must | P | EX: pressure, loads, load states (`exact_admission.rs:103-108`). PF refuses imposed displacement (`preview_physics.rs:1041-1093`) | DEL-05-01, DEL-05-05, DEL-04-07 | Complete on EX only. |
| FR-SOL-007 (516) | Must | P | Pipe and bend end rows are `element_local` only (`product_physics/src/lib.rs:12363,12384`; test `:16027`). Objective connectors publish global end actions (`lib.rs:4433,4441`) | DEL-04-01/02 | Global-frame end forces are missing for pipes and bends. |
| FR-SOL-008 (517) | Must | D | `core/loads/stress_recovery`; Lamé hoop and longitudinal on EX (`product_physics/tests/t4_u6_section_bases.rs`) | DEL-05-03 | Pressure stresses exist only where pressure exists (EX). |
| FR-SOL-009 (518) | Must | P | `core/loads/load_case_algebra`; tests `preview_physics_runtime.rs:597,682,709` | DEL-05-02 | PF only: EX refuses combinations (`exact_admission.rs:142,240`). |
| FR-SOL-010 (519) | Should | D | `core/solver/curved_bend`; DEC-070; expansion-loop k-sweep | DEL-03-03, DEL-04-07 | |
| FR-SOL-011 (520) | Should | P | SIF → intensified measure (`preview_physics.rs:320-345`) | DEL-03-04 | Branch flexibility is not applied to stiffness. EX refuses tees. |
| FR-SOL-012 (521) | Should | P | As FR-SOL-004 | DEL-03-05 | Component weight is never applied as a load. |
| FR-SOL-013 (522) | Should | D | `frame_kernel/src/connector.rs`, `tie_rod.rs`; t4_i12/u5a/u5b references | DEL-03-06 | EX only. Movement limits are stored as data and not enforced. |
| FR-SOL-014 (523) | Should | P | `core/solver/nonlinear_supports`; `references/kd5_nonlinear`; the native demo converges | DEL-04-04, DEL-04-07, DEL-09-03 | PF only. Rotational gap and friction are refused. |
| FR-SOL-015 (524) | Must | D | `core/solver/diagnostics`; `NUMERICAL_INTEGRITY_*` (`lib.rs:1197-1225`); `ResultsPanel.tsx:110` | DEL-04-06, DEL-05-04 | Health indicator in progress elsewhere (§4). |
| §12.8 near-duplicate nodes (775) | — | P | Only `DUPLICATE_ID` found | DEL-04-06 | No deliverable states this clause. |
| §12.8 verified bounds (783) | — | O | Owner, 2026-10-23: "no verified bounds in v0.1; the retained-precision method stays dormant and owner-held." (commit b71c3a23ad, on the health-indicator branch); held as B8 in DEL-04-08 SOW:336 | DEL-04-08 | Dormant code is in `product_physics/src/retained_*.rs`. |
| §22.2 benchmarks (1403) | — | P | 11 of 14 present (`validation/benchmarks/*`, `validation/references/t3_r1`) | DEL-09-01..03 | **Missing:** L-shaped thermal frame, rigid link. **Partial:** simply supported pipe under weight, imposed anchor displacement. |
| §22.3 stress benchmarks (1422) | — | D | `STRESS-*`, t4_u6/u4b, `STRESS-RANGE-MECHANICS-ORIGINAL` | DEL-09-02 | |

### 2.4 Rule packs, reporting, handoff and export (PRD §11.5, §11.7, §11.9, §16–19)

| ID (PRD line) | Pri | Class | Evidence | Deliverables | Notes |
|---|---|---|---|---|---|
| FR-RULE-001 (530) | Must | P | `schemas/rule_pack.schema.yaml`; `RulePackManagerPanel.tsx:304-519`; `e2e/r2-smoke.spec.ts:1173-1241` | DEL-06-01, DEL-07-03 | No stress categories, load-case mappings or required load cases (§17.2). |
| FR-RULE-002 (531) | Must | D | `core/rules/rule_pack_document/src/lib.rs:993-1002`; units refused `rule_check_runner/src/lib.rs:1286`; `expression_evaluator/tests/conformance_corpus.rs` | DEL-06-02 | Units are checked at run time, not when the pack is validated. |
| FR-RULE-003 (532) | Must | D (qualified) | `rule_check_runner/src/lib.rs:749`; desktop `lib.rs:2906-2925`; tests `lib.rs:7888,7903`; observed natively on both routes (§5) | DEL-06-03 | A check with missing inputs is never evaluated. The aggregate ranks failed above incomplete (`rule_check_runner/src/lib.rs:200,355`), so a pack where one check fails and another is incomplete shows aggregate FAILED. It never shows PASS. Whether that meets "refuses to report pass/fail" is a reading for the owner (Codex). |
| FR-RULE-004 (533) | Must | D | `rulePackService.ts:225-249` (private by default); local SQLite | DEL-06-04 | No pack file export. |
| FR-RULE-005 (534) | Must | D | `LibraryManagerPanel.tsx`; `lib.rs:8544`; `tests/test_library_import_provenance.py` | DEL-03-01, DEL-03-07 | Library records cannot be applied to model materials. |
| FR-RULE-006 (535) | Should | D | Same panel, component kind | DEL-03-02, DEL-03-07 | As above. |
| FR-RULE-007 (536) | Must | D | `tests/test_invented_example_models.py`; `examples/rule_packs/invented_demo.yaml` | DEL-11-04, DEL-06-05 | |
| FR-REP-001 (578) | Must | P | Renderer `core/reporting/report_renderer/src/lib.rs`; desktop `RenderedReportPanel.tsx`; blocked natively for the PF demo (§5); private provenance hard-coded (`renderableReportInput.ts:74-85`); `App.test.tsx:14895-14936` | DEL-08-01, DEL-08-06 | Content also lacks rule-pack refs (`renderableReportInput.ts:433`), software/solver versions, date and settings. EX is refused (10A). The package is refused for every identity. |
| FR-REP-002 (579) | Must | P | §19.3 notice is verbatim (`report_renderer/src/lib.rs:704-737`) | DEL-08-01 | No rule-check section. The notice reaches users only if the render is not blocked. |
| FR-REP-003 (580) | Must | P | `report_renderer/src/lib.rs:514-600` | DEL-08-03 | Materials, sections and libraries are absent. |
| FR-REP-004 (581) | Should | P | Within-run comparison summary only (`ReportPanel.tsx:649-652`) | DEL-08-06 | |
| FR-REP-005 (582) | Must | D | `core/reporting/protected_content_linter`; `tests/test_report_protected_content_linter.py` | DEL-08-05 | |
| FR-HAND-001 (558) | Must | P | `HandoffPanel.tsx:209-232` and `NativePackagePanel.tsx:217-229` produce preview documents; schema producer `core/handoff/exporter/workflow.py:57` has no caller | DEL-15-01, DEL-15-03, DEL-17-03, DEL-07-12 | No model export unless a solved PF result exists; EX refused (10A). |
| FR-HAND-002 (559) | Must | P | `HandoffPanel.tsx:211-283` | DEL-15-01 | `rule_pack_refs: []` hard-coded (`:237`); no model or export hash. |
| FR-HAND-003 (560) | Should | P | `target_field_coverage: "TBD"` (`HandoffPanel.tsx:239-245`) | DEL-15-02 | |
| FR-HAND-004 (561) | Should | P | Loss reports (handoff, PCF) | DEL-15-02, DEL-17-07 | PF only. |
| FR-HAND-005 (562) | Could | D | PCF, MBF, glTF and SDK panels (`App.tsx:979-999`); observed natively (§5) | DEL-17-04/07/08/09 | The MBF target version is `TBD`. |
| Result JSON, stress-neutral | — | D | `App.tsx:977-978`; `exactOutputTwins.test.tsx`; observed natively on both routes | DEL-08-04, DEL-17-06 | |
| Local FEA handoff | — | O | DEC-078 (D-12 O-B): FR-025 post-beta | DEL-10-03 | A gated panel exists. |

### 2.5 States, runs, comparison, V&V (PRD §11.6, §15, §21–22)

| ID (PRD line) | Pri | Class | Evidence | Deliverables | Notes |
|---|---|---|---|---|---|
| FR-CMP-001 (542) | Must | P | `schemas/model_state.schema.json`. Headless persistence keeps caller-supplied state records (`core/project_persistence/service.py:103,494`). The desktop keeps one model/run slot per project (`src-tauri/src/lib.rs:896,1129`) | DEL-14-01, DEL-07-08 | No named-state producer. **Scope unclear:** §23 (MVP) and SOW-071..076 `IN`, versus D-11/DEC-062 "W7 — Forward horizon v0.2" and DEL-14-01:3 "stage-gated: v0.2 R3". |
| FR-CMP-002 (543) | Must | P | `analysisRunCompatibility.ts:160-170` | DEL-14-02 | Synthetic `model_state_ref`; empty rule-pack and library refs; one run per project. |
| FR-CMP-003 (544) | Must | P | `core/comparison/model_state/engine.py:64`; `tests/test_model_state_comparison.py` | DEL-14-03 | Headless only. Scope unclear. |
| FR-CMP-004 (545) | Must | P | `core/comparison/analysis_run/engine.py:165`; `tests/test_analysis_run_comparison.py` | DEL-14-04 | Headless only. Scope unclear. |
| FR-CMP-005, -007 (546, 548) | Should | P | `schemas/comparison_mapping.schema.json`; `model_state/engine.py:232` | DEL-14-05 | No editor. |
| FR-CMP-006 (547) | Must | P | `analysis_run/engine.py:229` | DEL-14-03/04 | Desktop uses it within one run only. |
| FR-CMP-008, -009, -010 (549-551) | Should | P | `schemas/comparison_tolerance.schema.json`; `ComparisonPanel.tsx:57-70` | DEL-14-05, DEL-08-06 | No profiles, no overlays, no CSV. |
| FR-CMP-011 (552) | Could | P | `schemas/caepipe_external_run.schema.json` | DEL-14-05 | §15.6: not MVP. |
| §21.3 human acceptance (1385) | — | D | `docs/claims_registry.md:158,185` | DEL-01-04 | No acceptance workflow, as required. |
| §22.4 regression (1435) | Must | P | `tools/release/run_evidence_sweep.py:163-240`; last sweep `validation/evidence/sweeps/SWEEP_20260924T162044Z_b9dda2addccc.json` | DEL-00-08, DEL-09-05, DEL-10-04 | There are no states to hash. |
| §22.5 prover correlation (1455) | — | O | DEC-080: activation is a future owner-gated row; none exists | DEL-09-04, PKG-15 | The schema lacks the §22.5 fields. |
| R6 reproduction (1636) | — | P | Last bundle `validation/evidence/reproduction/REPRO_DEL0904_20260720T074714Z_a5235340aae3/` | DEL-09-04 | No reproduction since 2026-07-20. Part 1 is a frozen procedure that dated notes now qualify; there is no current runnable procedure (drift D-VM1). |

### 2.6 Data boundary, local operation (PRD §20)

| ID | Class | Evidence | Deliverables |
|---|---|---|---|
| Local-first, private paths (§20.3–20.4) | D | SQLite store in Application Support; no network client in `apps/desktop/src` or `src-tauri/src` | DEL-12-01, DEL-12-04 |
| Telemetry (§20.5) | D | `telemetryPolicyService.ts`; `tests/security/test_telemetry_policy.py` | DEL-12-03 |
| Unit system (MVP) | D | `core/units`; `unitConversion.ts` | DEL-02-02 |

`tauri.conf.json:23` sets `"csp": null`. No outbound path was found in code, but
the webview has no network policy.

### 2.7 MVP (PRD §23) and milestones (§24)

**§23 MVP items:**
- **In the desktop:** 21 of 28 items. Imposed displacement works on EX only; the validation suite is headless.
- **Not in the desktop:**
  - rigid elements (kernel test only);
  - named immutable states (schema only);
  - basic state comparison and basic run comparison (headless only);
  - runs tied to states (partial);
  - basic report generation (blocked, above);
  - canonical model package export (preview only, PF only).

**§24 milestones, factually:**
- **R0:** met.
- **R1:** partly. Whole-suite `run-benchmark` gave 11 matched and 13 blocked
  cases (`docs/validation_manual/headless_runner_reproduction.md`, note of
  2026-07-20). Rigid elements are absent.
- **R2:** ruled met under v0.1 tokens (D-14/DEC-035), but see the R2 row above.
- **R3 (States, Runs, Comparison):** not met, and no owner act has evaluated
  it.
- **R4** and **R5:** met as v0.1 R3/R4 (D-23/DEC-048, D-27/DEC-054). Report
  integration is blocked.
- **R6:** partly.
- **R7:** partly. Offline proposals are delivered; candidate generation is
  absent.

### 2.8 Coverage gaps in the deliverable set

**No user outcome reaches these deliverables directly.** They provide
architecture, governance or contract basis:
- DEL-00-01, DEL-00-02, DEL-00-04, DEL-00-07;
- DEL-01-01, DEL-01-03;
- DEL-02-03, DEL-02-04;
- DEL-10-01, DEL-11-03, DEL-12-05, DEL-17-01;
- DEL-15-04 and DEL-17-05 (external-prover loop, owner-gated).

**No deliverable owns these outcomes:**
- **Formats and stores:** the desktop model document format
  (`openpipestress.product_preview.model` 0.6.0); the desktop project store
  (partly DEL-12-01).
- **Solver:** the rigid element and rigid link; global-frame element-end
  forces; near-duplicate node detection; component-weight load application;
  model-topology connectivity validation.
- **Model and inspector:** the "lines" entity; inspector dependencies and
  assumptions.
- **Records:** the §22.5 correlation-record fields; the R3 (v0.2) exit review.

## 3. Reconciliation drift

Items marked **[fix]** are descriptive and safe. Items marked **[owner]** change
commitment or acceptance text and are listed as exact old → new for the owner.
"Pointers" means `deliverable.yaml` `code_paths`/`checks`.

**Pointers that name unused reference code and omit the desktop code** [fix].
These have the largest effect, because `tools.deliverables touches` and
`impact` cannot see the desktop:
- **PKG-07:**
  - DEL-07-01: add `features/toolkit/**` and `routeDraft.ts`.
  - DEL-07-02, DEL-07-05: their `core/gui/*` paths are unused.
  - DEL-07-03: rule-packs, library and inspector forms.
  - DEL-07-04: `MissingDataBlockingPanel.tsx`.
  - DEL-07-06: no pointers at all.
  - DEL-07-07: `solve_jobs.rs`, `solve_worker.rs`.
  - DEL-07-08: `core/gui/design_workspace`, plus checks.
- **PKG-06, PKG-08:**
  - DEL-06-01: `rule_pack_document` and its tests.
  - DEL-06-03, DEL-06-04, DEL-06-05: examples and tests.
  - DEL-08-01: `report_renderer`, `reportRenderService.ts`.
  - DEL-08-02: `audit_manifest`.
  - DEL-08-05: tests and `ReportLintPanel.tsx`.
- **PKG-15, PKG-17:**
  - DEL-15-01, DEL-15-03: `HandoffPanel.tsx`.
  - DEL-17-03/04/07/08: the export panels.
- **PKG-16:** DEL-16-03, DEL-16-04: `atomic_batch.rs`, `OperationLedgerPanel.tsx`.
- **PKG-02, PKG-03:**
  - DEL-02-01: `core/product_physics/src/model_format.rs`,
    `modelDocumentShape.ts`, `types.ts`.
  - DEL-02-05: the store in `src-tauri/src/lib.rs`, `model_document_file.rs`
    and its tests.
  - DEL-03-01, DEL-03-02: the library panel, service and tests.
  - DEL-03-04, DEL-03-05: `validation.rs`, `exact_admission.rs`.
  - DEL-03-08: `annulus_geometry.rs`.
- **PKG-09, PKG-11, others:**
  - DEL-09-01/02/03: the benchmark crates as checks.
  - DEL-11-02, DEL-11-03, DEL-11-05: the guide and theory documents.
  - DEL-13-01: the fixture and `KnowledgePanel.tsx`.
  - DEL-14-04, DEL-14-05: `previewService.ts`, the tolerance schema.
  - DEL-01-04: `PROFESSIONAL_BOUNDARY.md`.
  - DEL-10-04: the CI workflow.
- **Missing path:** DEL-09-05 lists `tools/coordination`, which does not exist.

**ScopeOfWork statements the code has overtaken** [fix]:
- **DEL-02-05:** cites "PRD section 10 FR-001/FR-002" and §12.4/§15.3/§18 in
  v0.1 numbering. These are now §11.1 FR-MOD-001 via D-21 Annex A, and
  §17.2/§19.2/§20.3. The retired versions now include 0.5.0. The desktop store
  does not use `project_persistence.schema.yaml`.
- **DEL-03-05:44,113:** describes a DEC-045 product route that applies no
  scaling.
- **DEL-05-05:48:** names a crate the product does not link.
- **DEL-07-05:119,158,260:** rotation-glyph evidence now exists.
- **DEL-07-07:126** and **DEL-08-01:79,122,184,205:** "not implemented in this
  setup", although the code ships.
- **DEL-07-03:95,157:** describes a reference editor rather than the manager
  that ships.
- **DEL-14-01:86,168** and **DEL-14-03:83:** module paths still `TBD`.
- **DEL-15-01, DEL-15-03:** do not say that the desktop handoff document is
  outside the schema.
- **DEL-16-04:134:** does not say the desktop route skips the claim scanner.
- **DEL-02-01:** add that the desktop document is a separate surface.

**Product documents** [fix]:
- **Validation manual:**
  - `docs/validation_manual/headless_runner_reproduction.md:144-150` (D-VM1):
    Part 1 is the frozen E1 procedure. Its prose says to add
    `--explicit-local-private-intent` and that the first command is now
    refused, so a current runnable procedure has to be written beside it (W-2).
  - **Fixed in PR #1538:** the `export-results` stub statements, the authority
    basis (`index.md:27-28`) and the DEL-09-05 pointer (`index.md:200`).
  - **Still open:** `index.md:63` (PRD §16.5, which does not exist), `:76`
    (missing path), `:284` (R5 → R6 numbering). The `:168` table is empty.
- **User guide** `docs/user_guide/index.md:70,232,236,394`: describes a
  skeleton and missing install steps, and says the report package works.
- **`apps/desktop/SMOKE.md:22-140`:** historical browser checks against strings
  that no longer exist.
- **`docs/RELEASE_NOTES_v0.1.0_DRAFT.md`:** glTF is offered on EX too. "Rendering
  is refused when…" understates that every session is blocked. This file is in
  the release-rehearsal lane; it is left to that manager.
- **Code comments:**
  - `LibraryManagerPanel.tsx:41-42` says library wiring is absent; it exists.
  - `product_physics/src/validation.rs:1268` implies scaling is applied when a
    mapping is valid; it never is. This one needs a DEC-081 claims check.
- **Corpus-wide:** ScopeOfWork files cite `_STATUS.md`, `_CONTEXT.md`,
  `docs/_Registers/*.csv` and `execution/_Reconciliation/…`, none of which
  exist on this tree. In reference lists these are [fix]; in requirement
  source cells they are [owner].

**Owner-reserved text (exact old → new, not applied).** Line numbers are at the W-1 PR heads (#1540–#1546). "Rec." is the reconciliation manager's recommendation: **Apply** (descriptive consequence of shipped code or an existing ruling), **Leave** (no change needed), or **Owner** (a substantive scope choice).

| # | Location | Old | New | Reason | Rec. |
|---|---|---|---|---|---|
| 1 | DEL-04-07 SOW:29 (CLM-001) | "The PRD's intended pressure capability remains future scope; this does not claim it is implemented." | "Pressure is implemented on the exact route as stated in Design/pressure-contract.md; capabilities that route refuses remain future scope." | EX route ships pressure | Apply |
| 2 | DEL-04-03 SOW:47,125 (R01) | "…imposed displacement boundary data…" | "…imposed displacement boundary data (exact route load/reference states only; the pressure-free route refuses support-targeted loads)…" | PF refuses imposed displacement | Owner (ties to W-16: either build PF or rule EX is the only route) |
| 3 | DEL-06-01 SOW:131-142 (REQ-06-01-001..012) | "The future schema shall" | "The schema shall" | `rule_pack_document` ships | Apply |
| 4 | DEL-08-05 SOW:149-159 (REQ-001..009) | "The future linter shall" | "The linter shall" | linter ships | Apply |
| 5 | DEL-08-05 SOW:160 (REQ-012) | "The future implementation shall preserve" | "The implementation shall preserve" | linter ships | Apply |
| 6 | DEL-08-02 SOW:112 | "The future implementation shall capture reproducibility metadata" | "The implementation shall capture reproducibility metadata" | `audit_manifest` ships | Apply |
| 7 | DEL-08-01 SOW:157 (CLM-013) | "The future implementation should be accepted only after these checks exist and pass:" | "The implementation should be accepted only after these checks exist and pass:" | renderer ships; acceptance stays the owner's | Apply |
| 8 | DEL-08-01 SOW:134 (R-08-01-001 verification) | "Inspect generated report contract/template behavior in future implementation tests." | "Inspect generated report contract/template behavior in implementation tests (`core/reporting/report_renderer/tests/render.rs`)." | tests exist | Apply |
| 9 | DEL-07-04 SOW:123 | "It does not implement GUI components, application state, schemas, tests, package manifests, or product source code." | "The setup pass did not implement product code; the GUI implementation is `apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx`, rendering all six R-DEL-07-04-001 classes." | panel ships (map row; :44 carries different text and needs no change) | Apply |
| 10 | DEL-07-05 SOW:160 (VER-07-05-005) | "…and that focused browser evidence for them (CLM-024) stays recorded as open until it is added." | "…and that the focused browser evidence `apps/desktop/e2e/rotation-glyph.spec.ts` (CLM-024) is bound to the tested candidate." | the spec exists and runs in the PR Chromium lane | Apply |
| 11 | DEL-07-03 SOW:256-260 (CLM-017) | "The DEL-07-03 implementation still accepts only material, component, and rule-pack-reference editor kinds. Load-case and support editor requests remain blocking unsupported kinds in this owner." | "The reference module `core/gui/editors/engine.py` still accepts only material, component, and rule-pack-reference editor kinds and blocks load-case and support requests. The desktop ships load-case editing in `apps/desktop/src/features/load-cases/`; this does not by itself close `DEL-07-03-R-005` or `R-006`." | text describes the reference engine as the implementation | Apply |
| 12 | DEL-07-09 `Vocabulary_Coverage.csv` row 11, ImplementationLandsIn | "DEL-16-01;DEL-16-02;DEL-16-03" | "DEL-16-01;DEL-16-02;DEL-16-03;DEL-16-06" | DEL-16-06 applies the operations | Apply |
| 13 | DEL-17-03 SOW:44 | "It does not implement public API, GUI action, persistence runtime, …" | "The Python foundation does not implement public API, persistence runtime, …; the desktop GUI action is `NativePackagePanel.tsx`, a separate TypeScript preview builder." | a GUI action ships | Apply |
| 14 | DEL-17-04 SOW:44 | "It does not implement a parser, external execution harness, public API, GUI action, …" | "It does not implement a parser, external execution harness, public API, … (the desktop GUI action is `CaepipeMbfExportPanel.tsx`, a separate TypeScript preview builder) …" | a GUI action ships | Apply |
| 15 | DEL-09-05 SOW:360; DEL-10-04 SOW:66,113,215,371,412 | "DEC-025 governs the local evidence sweep;" | "DEC-025's blanket local sweep is no longer a merge requirement; the hosted `harness` result on the exact PR head is the merge check, and `tools/release/run_evidence_sweep.py` is optional macOS release-candidate preparation;" | LOOP_INIT "Build and verification" already rules this | Apply (Agent 0 may, under the 2026-10-19 rule) |
| 16 | DEL-14-05 SOW:180 (R004) | "Evidence to assess: `schemas/comparison_mapping.schema.json` and `tests/test_comparison_contracts.py`" | "Evidence to assess: `schemas/comparison_tolerance.schema.json`, `schemas/comparison_mapping.schema.json` and `tests/test_comparison_contracts.py`" | tolerance profiles live in the tolerance schema | Apply |
| 17 | DEL-14-01 SOW:168 (R1-R3 verification) | "Exact test names TBD." | "Current tests (at `62c1efd75b`): `tests/test_model_state_schema.py`, `tests/test_persistence_schema.py` and `tests/test_project_persistence_service.py` (Python reference store)." | tests exist; desktop has no state store (OD-1) | Apply |
| 18 | DEL-11-02 SOW:137-149 (REQ-11-02-001..013) | "The future developer guide shall" / "The future guide shall" | "The developer guide shall" / "The guide shall" | `docs/developer_guide/index.md` exists | Apply |
| 19 | DEL-09-03 SOW:131 (REQ-09-03-001) | "The future suite shall" | "The suite shall" | nonlinear benchmark crate exists | Apply |
| 20 | DEL-10-04 SOW:158 (REQ-10-04-01) | "Future implementation must provide" | "The implementation must provide" | CI and packaging exist | Apply |
| 21 | DEL-02-05 SOW:34 (CLM-003) | "Every other version (`0.1.0`, `0.2.0`, `0.3.0`, `0.4.0`, or newer)" | "Every other version (`0.1.0`, `0.2.0`, `0.3.0`, `0.4.0`, `0.5.0`, or newer)" | code retires 0.5.0 | Apply |
| 22 | DEL-02-05 requirement source cells (:64-65, :84, :187-211) | v0.1 PRD numbers ("section 10 FR-001/FR-002", "section 12.4", "15.3", "18") | current numbers (§11.1 FR-MOD-001, §17.4, §19.2, §20.3–§20.5) | v0.1 numbering | Leave (the crosswalk note added at :141 resolves them) |
| 23 | DEL-02-05 SOW:201 (REQ-015 source) | "section 22.1" | "§24 R0 (v0.1 §22.1)" or "§22.1 (V&V)" | v0.1 §22.1 was release R0; current §22.1 is Verification Philosophy | Owner (which was meant) |
| 24 | `execution/_Coordination/_COORDINATION.md:28` | "PRD R5 exit criterion" | "historical v0.1 R5 (= v0.3 §24 R6) exit criterion" | numbering | Apply |
| 25 | `apps/desktop/native-smoke/expected.json` | rows 98/99, 625/627; N1 "…198" | 113/113, 745/745; "…199" | stale expected values | Routed to release-rehearsal lane (PR #1532) |
| 26 | Corpus-wide: requirement source cells citing `_STATUS.md`, `_CONTEXT.md`, `docs/_Registers/*.csv`, `execution/_Reconciliation/…` | retired paths | the path kept, annotated with its retiring commit: `_STATUS.md`/`_CONTEXT.md` "(retired in `497a8d81e4`; see Git history)", `docs/_Registers/*` "(retired in `48584391de`; …)", `execution/_Reconciliation/…` "(retired in `11718e61a2`; …)", as #1550 does for reference lists | files removed | Apply as one mechanical pass (#1550 covers the reference lists) |

## 4. Proposed work plan

### 4.1 Ready for development testing now

These are class D outcomes with a desktop path, which development testing can
exercise today:
- **Model and project:** open, save and reopen model documents and projects.
- **Authoring:** nodes, pipes, materials, sections, bends, joints, supports,
  load cases and combinations, with undo and redo.
- **Solving:** PF solve, including nonlinear supports, wind, seismic and
  combinations; EX solve.
- **Results:** results table, standing, diagnostics and deformed shape.
- **Proposals and data:** offline proposals; libraries.
- **Rule packs:** authoring and rule-check runs.
- **Exports:** result JSON and stress-neutral JSON (both routes); PCF and MBF
  package documents (PF). Stress-neutral CSV, glTF, PCF text and MBF text are
  withheld by the desktop (`ControlledExportLink.tsx:36-48`).
- **Boundaries:** local-only storage and telemetry.

These are **not ready**: reports, model states and comparison, canonical model
export, design knowledge, and rigid components.

### 4.2 Owner decisions that shape the plan

These are reserved because they concern v0.1 scope, commitments or a hold.

**OD-1 to OD-5 decided.** Owner, 2026-10-23: "I approve OD-1 through OD-5 as
you recommended." Each outcome is in the row's **Decided** entry. OD-6 remains
open.

| # | Decision | Consequence |
|---|---|---|
| OD-1 | Does D-11's "W7 — Forward horizon v0.2" placement (PKG-13/14/15/16/17) mean outside software 0.1.0, despite PRD §23 listing states, runs tied to states, state and run comparison, and canonical model export in the MVP? | **In:** unit W-12 (L) and W-6 come before validation. **Out:** a ruling, and PRD §23 and the release-notes text follow it. Either answer also settles OD-3 for PKG-13. **Decided:** states, run history, state and run comparison and canonical model export are in v0.1 (W-12, W-6). PKG-15 handoff stays in for model export. PKG-13, further PKG-16 agent features and further PKG-17 export formats are deferred to v0.2; the DEL-16-01/02 offline intake and the DEL-16-06 operation-application seam ship as built. PRD §23 stays as written. |
| OD-2 | Rigid elements (FR-SOL-004, Must, MVP) | **Options:** build DEC-045's stiffness scaling (M); build a dedicated rigid element and offsets, which reopens DEC-045 (L); or defer by ruling. **Decided:** implement DEC-045 stiffness scaling (W-15). |
| OD-3 | Design-knowledge entry and constraint validation (FR-KNOW-001, Must; R6) in v0.1? Settled with OD-1 if the W7 reading applies. | **In:** W-14 (L). **Out:** a ruling. **Decided:** design knowledge is deferred to v0.2. |
| OD-4 | Which model format is canonical: the desktop 0.6.0 document or `schemas/model.schema.yaml`? (FR-MOD-001/004/006) | Fixes the target for W-13, a new document version under the one-version rule. **Decided:** the desktop model document is canonical, and `schemas/model.schema.yaml` is updated to describe it (W-13). |
| OD-5 | Viewport result overlays and reaction arrows (FR-GUI-001, Must); the R7 staging of the FR-AGENT-005 claim scan (DEL-16-04:3) | Each is either an S–M unit or a ruling. **Decided:** viewport overlays and reaction arrows are built now; FR-AGENT-005 stays R7. |
| OD-6 | Owner-reserved text in §3 | Apply as proposed, amend, or leave. |

Opening exact-route reports and handoff needs no new decision to start. Ruling
10A already directs DEL-08-01 to design their limitation sections. Opening the
surfaces afterwards returns to the owner.

### 4.3 Units in recommended order

The lane is heavy where builds or numerical checks are needed and light for
documents. Heavy work that changes numerical meaning or persistence needs
targeted regression checks and independent scrutiny (Root `AGENTS.md`).

| # | Unit | Size | Kind | Lane | Depends on | Owner? |
|---|---|---|---|---|---|---|
| W-1 | Pointer and ScopeOfWork reconciliation sweep (§3 [fix] items) | M | docs | light | — | No |
| W-2 | Validation-manual and user-guide currency (D-VM1 etc.), then a fresh R6 reproduction bundle at HEAD | S–M | docs | light → heavy for the run | — | Acceptance of the reproduction is the owner's |
| W-3 | Rendered report reachable for local private sessions: give the render the same local-private intent the exports already use; add §19.2 content (rule-pack refs, rule-check outcomes, software/solver versions, date, settings, coordinate system) | M | desktop | heavy | DEL-12-02 design check | No, unless DEL-12-02 rules it out |
| W-4 | R2 end to end in the native smoke: a from-blank GUI-authored model → native solve → results, deformed shape → save → reopen; add the §5 probe steps (report, rule checks, EX gates) as permanent checks | S–M | desktop test | heavy | — | No |
| W-5 | Memory-pool headroom: on a loaded Mac, the app refused a 16.56 MiB result-export save with 6.44 MiB "free" (free list only; `src-tauri/src/host_memory.rs:5-13`). Check the interactive-save admission against D-72 criteria and make ordinary saves succeed | S–M | desktop (Rust) | heavy | D-72 | Only if D-72 acceptance text must change |
| W-6 | Canonical model export from the desktop (save model document to file; FR-HAND-001), independent of a solve; schema-conformant handoff manifest with rule refs and hashes (FR-HAND-002) | M | desktop + schema | heavy | OD-4 for "canonical" | No |
| W-7 | Global-frame element-end forces (FR-SOL-007) | S | numerical | heavy, scrutiny | — | No |
| W-8 | Pre-solve connectivity check (disconnected anchored sub-models, invalid branch topology; FR-KNOW-002); near-duplicate node detection (§12.8) | S–M | numerical-adjacent | heavy, scrutiny | — | No |
| W-9 | Missing benchmarks: L-shaped thermal frame, simply supported weight, imposed anchor displacement on a frame; rigid link after OD-2 | S | numerical | heavy | OD-2 (rigid link) | No |
| W-10 | Pipe reconnect operation (FR-GUI-004) | S–M | desktop + Rust | heavy, scrutiny (topology) | — | No |
| W-11 | Exact-route reports: DEL-08-01 limitation sections (ruling 10A), then open | M–L | docs + desktop | heavy | W-3 | Opening returns to owner |
| W-12 | States, run history, state and run comparison in the desktop (FR-CMP-001..010) | L | desktop + persistence | heavy, scrutiny | OD-1 | OD-1 |
| W-13 | Product document ↔ canonical schema; tags, notes, assumptions, rule-pack refs, structured provenance (FR-MOD-001/002/004/005/006) | L | schema + persistence | heavy, scrutiny | OD-4 | OD-4 |
| W-14 | Design-knowledge entry and constraint validation in the desktop (FR-KNOW-001/003/004) | L | desktop + schema | heavy | OD-3 | OD-3 |
| W-15 | Rigid / semi-rigid components (FR-SOL-004/012), including component weight and EX admission | M–L | numerical | heavy, scrutiny | OD-2 | OD-2 |
| W-16 | Combinations on EX (FR-SOL-009); imposed displacement on PF, or a ruling that EX is its only route (FR-SOL-006) | M | numerical | heavy, scrutiny | — | No |
| W-17 | Smaller usability gaps: project unit selection; applying library records to model entities; unit check at rule-pack validation; rule-pack schema categories and load mappings (§17.2) | S–M each | desktop/schema | heavy | §17.2 needs a scope ruling | §17.2 only |

**Sequencing:**
- **Before development testing broadens:** W-1, W-2, W-4 and W-5. These give
  the testers current documents and a native journey that proves itself.
- **Before the owner's hands-on use:** answers to OD-1..OD-4, plus W-3 (a
  report the owner can obtain) and W-6. Otherwise the session meets the MVP
  gaps of §1 in its first minutes.
- **Before validation:** the units the owner's answers place in v0.1 (W-12 to
  W-15), together with W-7 to W-9 and W-16. The reproduction bundle (W-2) is
  then regenerated at the validation basis.

**In progress elsewhere:** the numerical health indicator (FR-SOL-015 display;
branches `claude/piping-nh-*`) is under another manager and is not assessed
here.

## 5. Checks run for this map

**Read-only TASK reviews.** Five reviewed the PRD areas against code and tests
at `7a780551f0`. Codex then spot-checked a sample of D and P placements
independently (§6). `python3 -m tools.deliverables --project
projects/chirality-piping check` reported no structural errors and one cycle
(DEL-09-05 ↔ DEL-10-04).

**Native smoke at `7a780551f0`.** The run was `node
apps/desktop/native-smoke/run.mjs`, a fresh build through `t3_slot.sh` on a
loaded host. 40 of 62 checks passed.
- **Expected-value FAILs:** 10, all row counts and one last digit. These are
  the known stale `expected.json` values (PR #1532).
- **Cascade FAILs:** the rest followed one product refusal. The demo's
  result-export save was refused by the native memory pool: "needs 16.56 MiB,
  6.44 MiB available". This led to W-5.
- **Passed:** refusals by name, solve on both solver modes, model-hash proofs,
  and the save, replace and stale-base prompts.

**Hands-on probe.** The run used the same smoke build, with an untracked probe
driver in place of `driver.js` and a throwaway HOME.

| Step | Observed |
|---|---|
| Demo (PF) solve | completed, 745 rows, `checks_passed` |
| Demo rendered report | `decisions=18031; findings=18031; blocked=true`; no preview, no hash |
| Demo rule checks (bundled pack) | `RULE_INPUTS_INCOMPLETE` with named missing inputs |
| Demo exports | result JSON and stress-neutral JSON (behind local-private intent), stress-neutral CSV, PCF, MBF, CAEPIPE external, SDK, adapter, local FEA, external prover, glTF, headless runner, native package, handoff, export review |
| EX model `fixtures/model_operations/exact_pressure_authoring_model.json` solve | completed, 297 rows |
| EX rendered report | refused, `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` |
| EX rule checks | run; `RULE_INPUTS_INCOMPLETE` |
| EX exports | result JSON and stress-neutral JSON (behind intent), stress-neutral CSV, glTF only; the rest refused with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` |

## 6. Independent spot-check

Codex (GPT-6.1-Sol, read-only, `7a780551f0`) checked 13 claims independently
before this map was returned.

- **Confirmed (7):** FR-SOL-005, FR-GUI-005, FR-RULE-005 and FR-SOL-013 as D;
  FR-HAND-001, FR-REP-001 and FR-MOD-006 as P.
- **Qualified (6):**
  - FR-RULE-003: aggregate ordering.
  - FR-SOL-004: connector offsets exist.
  - FR-SOL-007: connectors publish global end actions.
  - FR-CMP-001: headless state persistence exists.
  - FR-KNOW-002: exact pressure regions check topology.
  - Scope: D-11/DEC-062's W7 placement.
- **Refuted:** none.

Every qualification is folded into the rows above, and none changed a class
other than FR-RULE-003's note. The W7 finding reframed OD-1.
