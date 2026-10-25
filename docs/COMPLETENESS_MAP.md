# SWBPIPE completeness and reconciliation map

**Basis:** `origin/main` at `8a8a8b2584` (2026-10-23). Folded in since: W-4
(#1582) and the owner's rulings and readings of 2026-10-23 on §3 rows 6–15,
PRD §14.4 topology, FR-MOD-007 and FR-RULE-003. This is a replaceable
current-state record for the owner and the work plan. Replace it when the
basis moves materially; do not append to it. It replaces the map at
`7a780551f0`; Git holds that version.

It answers three questions:
- Which intended outcomes does the product deliver today?
- Which deliverables' documents have drifted from the code?
- What must come before development testing, the owner's hands-on use and
  validation?

It is not a release assessment. Classifications are facts read from code, tests,
one native smoke run and one hands-on probe (§5). They are not acceptance:
acceptance, issuance and release remain the owner's (F-PIP-3).

Standard claim fence applies (claims taxonomy per DEC-081).

**Classes**
- **D:** delivered, with evidence (code a user reaches, and tests).
- **R:** delivered, but the deliverable's ScopeOfWork, Design or
  `deliverable.yaml` does not match the code.
- **P:** partial or missing.
- **O:** out of scope for v0.1 by a cited owner ruling.

A Must requirement met only on one solve route, only headless, or only as a
schema is P. The owner's rulings OD-1 to OD-6 (§4.2) settle the scope questions
the previous map left open; rows cite them where they apply.

**Two solve routes** recur throughout:
- **PF:** the pressure-free route, a model with no `pressure_contract`.
- **EX:** the exact pressure route, `3.0.0/exact_pressure_v3`.

**Not on main, so not counted here (in flight):**
- branch `claude/piping-w7-global-end-forces-20261023` (W-7, no PR yet);
- #1567 (DBM concordance review).

## 1. What a user can do today

Working in the macOS desktop app on invented data, a user can do the following.
The native smoke and the hands-on probe (§5) exercised this path in the real
app at the basis.

- **Projects:** open a model document, open the bundled demo, or start a blank
  project. A blank project now opens a units dialog first (SI, US customary or
  a custom unit per key).
- **Authoring:** place nodes and pipes in the 3D viewport, assign materials and
  sections, and add bends, expansion joints, supports, load cases and
  combinations. Split, copy, rotate, mirror and reconnect pipes. Every edit is
  a validated, diffed and undoable operation.
  - The probe authored a cantilever from blank in the native app: node,
    material, pipe, fixed support, load case and a 350 N tip force. It solved
    natively, saved, reopened in a fresh process and solved again identically.
    The tip displacement (1.3703 mm) agrees with PL³/3EI.
- **Solving:** solve on either route.
  - PF covers linear and nonlinear supports, wind, seismic and combinations.
  - EX covers pressure, load/reference states, imposed support motion and
    objective-connector joints.
  - Before solving, both routes warn about disconnected parts, coincident or
    near-duplicate nodes and invalid tees (W-8). The near-duplicate tolerance
    is 5 mm unless the user sets another in the Project page's Model settings.
  - A load case whose authored loads are not applied is refused by name
    instead of solving to zeros (#1568).
- **Results:** read displacements, element-local forces, reactions and
  stresses, each with a numerical-integrity standing and a per-case numerical
  health indicator. View the deformed shape, colour members by a result
  quantity, and show support reaction arrows.
- **Saving:** save the project to the local store and reopen it.
- **Rule packs:** author a private rule pack. Validation reports static unit
  findings per check, and running it against results gives
  `RULE_INPUTS_INCOMPLETE` with the missing inputs named.
- **Reports:** render a calculation report for a PF result as a local private
  document. Tick "Include known private values in this local rendered report"
  and the render shows a preview and its canonical SHA-256, with Save canonical
  HTML and Print / PDF. The document carries rule-pack references, user-rule
  check outcomes, software and solver versions, the generation time, run
  settings and coordinate systems.
- **Libraries:** import private material, section and component libraries.
- **Exports:** export result JSON and stress-neutral JSON on both routes, each
  behind local-private intent. PF results also export the PCF and MBF package
  documents (redacted), the adapter SDK and framework envelopes, a local FEA
  packet, a native package review manifest and a preview handoff JSON. The
  byte-exact downstream outputs (stress-neutral CSV, glTF review geometry, PCF
  text and MBF text) are withheld by the desktop, because session values are
  private (`ControlledExportLink.tsx`).

The biggest gaps:

1. **No shareable report, no EX report, no report package.** A public render of
   any session stays `blocked=true`, because session provenance is hard-coded
   private (`renderableReportInput.ts:119-131`). EX results are refused for
   every report (ruling 10A). Save Report Package refuses every current result
   (`REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`). The report lacks materials,
   sections and library records. Affected: FR-REP-001..004.
2. **No model states, run history or comparison between states or runs.** These
   are in v0.1 by OD-1. Named immutable states are in the desktop (W-12 S1,
   #1589): save, open, label and delete, kept only in the SQLite store. Runs are
   recorded against states (W-12 S2, #1596), with the model's rule packs
   resolved at solve (#1602). Both comparisons exist
   as schemas and headless Python engines only (W-12 S3). Affected:
   FR-CMP-001..004, -006.
3. **No rigid elements.** OD-2 chose DEC-045 stiffness scaling (W-15). It is
   validated but never applied, and EX refuses these components. Affected:
   FR-SOL-004/012.
4. **No canonical model export.** No save-model-to-file command. The handoff
   download is a preview document outside `schemas/handoff_package.schema.json`,
   with `rule_pack_refs: []` hard-coded. W-6 has a design only. Affected:
   FR-HAND-001/002.
5. **Model document slots.** The model document is 0.7.0 (#1564), and the
   schema describes it. Tags, notes, unresolved assumptions and open issues,
   rule-pack references, lines, constraints, equipment interfaces and structured
   entity provenance with `library_ref` have slots, checked by one document
   validator, and the desktop authors them (#1565). Library records apply to
   model entities with their provenance (#1566). Affected:
   FR-MOD-001/002/004/005/006.
6. **Mechanics gaps on one route.**
   - Combinations are refused on EX (FR-SOL-009).
   - Imposed displacement is refused on PF (FR-SOL-006); the owner ruled to
     build it there in two stages.
   - Pipe and bend end forces are element-local only (FR-SOL-007; W-7 is in
     flight).
7. **Rule-pack scope (§17.2).** No stress categories, load-case mappings or
   required load cases (FR-RULE-001). In v0.1 by ruling (owner, 2026-10-24,
   option A: all three; W-17 rest).

## 2. Outcome table

Each row gives the PRD requirement (`docs/PRD.md` line), its priority and
class, the evidence, and the deliverables. Paths are under
`projects/chirality-piping/`. "Desktop `lib.rs`" is `apps/desktop/src-tauri/src/lib.rs`;
"PP `lib.rs`" is `core/product_physics/src/lib.rs`.

### 2.1 Model, schema, design knowledge (PRD §10.1, §11.1–11.2, §13)

| ID (PRD line) | Pri | Class | Evidence | Deliverables | Notes |
|---|---|---|---|---|---|
| FR-MOD-001 (476) | Must | P | **Store:** desktop `lib.rs:2432` save, `:1189` load. **Tests:** desktop `lib.rs:6283,6338,6510,7565`; `e2e/b3b-project-persistence.spec.ts`. **Schema:** `schemas/model.schema.yaml` describes the 0.7.0 document; `tests/test_model_document_schema.py:247,254` validates the bundled demo and e2e documents against it | DEL-02-05, DEL-02-01, DEL-00-04 | Tags, notes, assumptions and rule-pack references have 0.7.0 slots (#1564) and are authored in the desktop (#1565). No save-to-file command. Nothing validates against the schema at run time. |
| FR-MOD-002 (477) | Must | P | Typed entities in PP `lib.rs:367,382,449,578,657` | DEL-02-01 | Lines, constraints and equipment interfaces have 0.7.0 slots as typed records without evaluation (#1564), authored in the Model tree and the Model records tab (#1565). |
| FR-MOD-003 (478) | Must | D | Identity changes refused: `operation_applier/src/rich_authoring.rs:1092`, test `:864`; `atomic_batch.rs:690`; reconnect keeps ids (`pipe_reconnect.rs`) | DEL-02-01, DEL-16-06 | No all-operations id-survival test. |
| FR-MOD-004 (479) | Must | P | Only `project.name` and entity `label` | DEL-02-01 | Tags and notes have 0.7.0 slots (#1564), authored in the inspector for every entity and the project (#1565). |
| FR-MOD-005 (480) | Must | P | Library records carry structured provenance: `core/library_import/library_import_document/src/lib.rs:339-402`. Library insert and apply set the entity's `provenance_record` with `library_ref` (record digest `library_version`): `apps/desktop/src/features/library/libraryApply.ts`; tests `libraryApply.test.ts`, `operation_applier/tests/library_apply.rs` | DEL-03-01, DEL-03-07 | 0.7.0 adds a structured `provenance_record` with `library_ref` (#1564). Applying library records (#1566): entities written from a library carry a combined `provenance_record`, and the store check warns on unresolved or changed records (`check_model_library_refs`). |
| FR-MOD-006 (481) | Must | P | `unresolved_assumptions` exists in `schemas/physical_transform_model.schema.yaml`, `model_state.schema.json:364` and `handoff_package.schema.json:477`, not in the desktop model document | DEL-02-01, DEL-13-01 | 0.7.0 adds the slot (#1564); the Model records tab enters, resolves and rejects them (#1565). State-level assumptions are labels on a saved state, carried into its `ModelStateRecord` (W-12 S1, #1589). Run-level assumptions are labels on a recorded run, shown with it; analysis_run 0.3 has no assumption slot (W-12 S2, #1596). |
| FR-MOD-007 (482) | Must | D (bounded) | On the live solve path the product derives its analytical model (`BuiltModel`, PP `lib.rs:2261`; `build_model` `:7930`) from the supported canonical-document fields and records diagnostics (PP `lib.rs:2635-2739`) | DEL-13-04, DEL-04-07 | Owner reading (2026-10-23, accepted): D bounded to this derivation. The richer PKG-13 transform contract (`core/model_transform/physical_to_analytical/contract.py`, test-only) stays deferred under OD-1. |
| FR-KNOW-001 (488) | Must | O | OD-3 (DEL-13-01 SOW:15). Desktop `lib.rs:1773` returns a bundled fixture; `KnowledgePanel.tsx` has no inputs | DEL-13-01 | |
| FR-KNOW-002 (489) | Must | D | W-8: `core/product_physics/src/topology.rs:46`, called once at PP `lib.rs:2739` on the shared path of both routes; `MODEL_TOPOLOGY_DISCONNECTED` (`topology.rs:136`), `BRANCH_TOPOLOGY_INVALID` (`validation.rs:1310-1320`); unsupported parts still refused (PP `lib.rs:15412`); EX pressure-region topology (`pressure_runtime.rs:1538,1548,1595`). **Tests:** `tests/topology_checks_runtime.rs` (9). **Desktop:** Diagnostics panel (`App.test.tsx:10877`) | DEL-04-06 | Findings are warnings; PRD §14.4 lists topology under "blocking for model validity" (§6). Tests use PF requests only. The bundled demo now warns `BRANCH_TOPOLOGY_INVALID` (tee C-120). |
| FR-KNOW-003 (490) | Should | O | OD-1/OD-3 (DEL-13-03 SOW:15). `core/constraints/validation/engine.py`, Python only | DEL-13-03, DEL-13-02 | |
| FR-KNOW-004 (491) | Should | O | OD-1/OD-3 (DEL-13-02 SOW:15). `schemas/constraint.schema.json` | DEL-13-02 | |
| FR-KNOW-005 (492) | Could | O | PRD §13.3 "In future releases"; OD-1 defers further PKG-16 features; DEL-16-04:3 "v0.2 R7" | (R7) | |
| §10.1 step 1, select units (415) | — | D | W-17 (a): `features/project-units/ProjectUnitsDialog.tsx`, mounted at `App.tsx:1087`; `projectService.ts:380-414` (seven keys, SI/US/custom, `PROJECT-UNITS-INCOMPLETE`). **Tests:** `ProjectUnitsDialog.test.tsx`, `projectUnitSelection.test.ts`; e2e `confirmProjectUnits`. Observed natively (§5) | DEL-02-02 | Changing units after creation (slice b) is not built. The e2e confirms SI only. |

### 2.2 Desktop GUI and operations (PRD §11.3, §11.8, §14)

| ID (PRD line) | Pri | Class | Evidence | Deliverables | Notes |
|---|---|---|---|---|---|
| R2 exit (1572) | — | D | W-4 (#1582): the native smoke authors a cantilever from blank through the GUI (`native-smoke/driver.js`, `authorFromBlank`), solves natively, checks the tip deflection against PL³/3EI within 1e-9, saves in place, reopens in a fresh process and re-solves to the same rows, value and model hash (`run.mjs`, "from blank" checks) | DEL-07-11, DEL-07-01/02/05/07 | The browser journey still ends at `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`, by design. One model shape and the SI preset only. |
| FR-GUI-001 (498) | Must | D | `PipeViewport.tsx:1289-1291` (nodes, supports, components); deformed shape `:1306-1310`; rotation glyphs `:1313`; reaction arrows `:1315-1317`; result colour `:1323-1327`; load arrows `:1332`; labels `:1374-1429`; legend `:2456`. **Tests:** `e2e/result-overlays.spec.ts:158` (PR core lane), `PipeViewport.overlays.test.tsx`, `c3-viewport-visibility.spec.ts`, `rotation-glyph.spec.ts` | DEL-07-01, DEL-07-05 | Overlays are off by default. Colour is one value per member. Reaction arrows come from global-frame rows only. The D-72 orbit qualification with result colour has not run. §14.2 bend arcs, corridors and no-go volumes are not drawn. |
| FR-GUI-002 (499) | Must | P | `ModelTree.tsx:40-57` | DEL-07-02 | No lines, states or runs. |
| FR-GUI-003 (500) | Must | P | `PropertyInspector.tsx:1276,1291,1311`; `typedInspector.test.tsx` | DEL-07-02 | No dependencies, linked assumptions or dependent rule-pack inputs. |
| FR-GUI-004 (501) | Must | D | Create, modify, move, delete (refuses while any record names the pipe; `tests/pipe_delete_dependents.rs`); split, copy, rotate, mirror (`geometry_operations.rs`); reconnect (`operation_applier/src/pipe_reconnect.rs`, `tests/pipe_reconnect.rs`, `e2e/pipe-reconnect.spec.ts:52`; UI `GeometryToolsPanel.tsx:65-88`) | DEL-07-01, DEL-16-06 | Declared v0.1 limitation (DEL-16-06 SOW:79): reconnect refuses component-bearing and pressure-region pipes. All four demo pipes are refused. |
| FR-GUI-005 (502) | Should | D | `workspaceSession.ts` (depth 25); `e2e/b3a-session-status.spec.ts` | DEL-00-05, DEL-07-11 | Session-only. |
| FR-GUI-006 (503) | Must | D | `MissingDataBlockingPanel.tsx:24,148-181`; `App.test.tsx`; `e2e/gui-workflow-validation.spec.ts:110` | DEL-07-04 | Topology findings are warnings, not blocking. |
| FR-GUI-007 (504) | Should | P | `ComparisonPanel.tsx`; `previewService.ts` (`single_run_load_basis_review`) | DEL-07-08, DEL-14-05 | Within one run only (W-12). |
| FR-AGENT-001 (568) | Should | D | `OfflineProposalIntakePanel.tsx` → `atomic_batch.rs:285-345` | DEL-16-01, DEL-16-06 | The live provider is O (D-22/DEC-041). |
| FR-AGENT-002 (569) | Should | D | `atomic_batch.rs:8,112`; tests `:529,593,690,746,806` | DEL-16-02, DEL-16-06 | |
| FR-AGENT-003 (570) | Should | D | Diff review and explicit Apply (`workspaceSession.ts:1345,1496`) | DEL-07-08, DEL-16-06 | No e2e for offline intake. |
| FR-AGENT-004 (571) | Should | P | Rationale and source required (`atomic_batch.rs:326-341`) | DEL-16-03, DEL-16-04 | No assumptions or constraints-considered keys; receipts are session-only. |
| FR-AGENT-005 (572) | Must | O | OD-5: the claim scan stays R7 (DEL-16-04:3, SOW:383). Boundary flags fixed and tamper-checked (`atomic_batch.rs:196-204,424-440`) | DEL-16-04 | The scanner is headless only (SOW:136). |

### 2.3 Solver and mechanics (PRD §11.4, §12, §22.2–22.3)

| ID (PRD line) | Pri | Class | Evidence | Deliverables | Notes |
|---|---|---|---|---|---|
| FR-SOL-001 (510) | Must | D | `core/solver/frame_kernel/src/lib.rs:17,1591`; t3_r1 references | DEL-04-01 | Both routes. |
| FR-SOL-002 (511) | Must | P | `validation/benchmarks/mechanics/src/lib.rs:934,963,1693`; W9-A (both routes), W9-C | DEL-04-01, DEL-09-01 | Beams, frames, transforms and thermal pass on both routes. The imposed-displacement benchmarks (W9-C, `mechanics/src/lib.rs:948`) run on EX only, because PF refuses support motion (FR-SOL-006). |
| FR-SOL-003 (512) | Must | D | `core/solver/straight_pipe`; `self_weight.rs:505-512`; `STRESS-*` | DEL-04-02, DEL-05-03 | PF Euler–Bernoulli; EX adds Timoshenko. |
| FR-SOL-004 (513) | Must | P | No rigid element and no DEC-045 scaling applied: `stiffness_scaling_user_value` is unit-normalized (PP `lib.rs:9483`) and validated (`validation.rs:777,1346-1411`) only. EX refuses rigid and specialty components (`exact_admission.rs:208-210,285`) | DEL-03-05 | OD-2: build the scaling (W-15). Offsets exist only on connectors and tie rods. No rigid-link benchmark. |
| FR-SOL-005 (514) | Must | D | `core/solver/linear_supports`; PP `lib.rs:2741`; EX admits linear restraints and springs (`exact_admission.rs:295-305`) | DEL-04-03 | Both routes. |
| FR-SOL-006 (515) | Must | P | EX applies `boundary_motion` (`exact_admission.rs:106`; W9-C `tests/w9_solver_benchmarks.rs:413`). PF refuses support-targeted loads (`preview_physics.rs:1041-1093`, `IMPOSED_DISPLACEMENT_REQUIRES_LOAD_STATE_MODEL`) | DEL-04-03, DEL-04-07, DEL-05-01, DEL-05-05 | Owner (OD-6 row 2): build on PF in two stages (`DEL-04-07 Design/pressure-contract.md`, "Imposed displacement on the pressure-free route"). |
| FR-SOL-007 (516) | Must | P | Pipe and bend end rows are `element_local` only (PP `lib.rs:12366-12392`; test `:16030`). Connectors publish global end actions (`:4436`) | DEL-04-01/02 | W-7 in flight. No deliverable commits to global end forces (DEL-04-01 SOW:140 covers transforms). |
| FR-SOL-008 (517) | Must | D | `core/loads/stress_recovery`; `tests/t4_u6_section_bases.rs:270` | DEL-05-03 | Pressure stresses exist where pressure exists (EX). |
| FR-SOL-009 (518) | Must | P | `core/loads/load_case_algebra`; `preview_physics_runtime.rs:597,682,709` | DEL-05-02 | EX refuses every combination (`pressure_runtime.rs:337-343`; `exact_admission.rs:229`). |
| FR-SOL-010 (519) | Should | D | `core/solver/curved_bend`; DEC-070 | DEL-03-03, DEL-04-07 | |
| FR-SOL-011 (520) | Should | P | SIF → intensified measure (`preview_physics.rs:321-347`); `BRANCH_TOPOLOGY_INVALID` (W-8) | DEL-03-04 | Branch flexibility is not assembled (PP `lib.rs:12861`). EX refuses tees (`exact_admission.rs:205`). |
| FR-SOL-012 (521) | Should | P | As FR-SOL-004; the self-weight request rejects `component_weights` (`self_weight.rs:1453-1457`) | DEL-03-05 | Component weight is never applied as a load. |
| FR-SOL-013 (522) | Should | D | `frame_kernel/src/connector.rs`, `tie_rod.rs`, `joint.rs:727`; `tests/connector_pp_paths.rs` | DEL-03-06 | EX only. `movement_limit` is normalized, not enforced (PP `lib.rs:9378`). |
| FR-SOL-014 (523) | Should | P | `core/solver/nonlinear_supports`; `references/kd5_nonlinear`; the native demo converges | DEL-04-04, DEL-04-07, DEL-09-03 | PF only. Rotational gap and friction refused (`validation.rs:88,100`). |
| FR-SOL-015 (524) | Must | D | `NUMERICAL_INTEGRITY_*` (PP `lib.rs:1198-1230`); health indicator `operation_applier/src/numerical_health.rs`, `ResultsPanel.tsx:97-130`, status bar and Solve panel, with tests (#1544). Observed natively on both routes (§5) | DEL-04-06, DEL-05-04, DEL-07-05 | |
| §12.3 reducers (no row before) | — | P | No eccentric offset vector in `core/product_physics/src` | DEL-03-05 | Part of W-15. |
| §12.5 load types (no row before) | — | P | Wind, seismic and hydrotest categories (PP `lib.rs:14333-14335`) | DEL-05-01 | EX refuses `equivalent_static`; imposed rotations and nozzle movement follow FR-SOL-006. |
| §12.8 near-duplicate nodes (775) | — | D | W-8 with #1574: `NODE_COINCIDENT`, `NODE_NEAR_DUPLICATE` within an absolute tolerance, 5 mm unless `model_settings.node_proximity_tolerance` sets another (`topology.rs:148,281`; read through `PreviewModel::node_proximity_tolerance_m`); a malformed value is refused by name (`validation.rs` `validate_model_settings`); tests `topology_checks_runtime.rs:143,182,193`. The desktop's Project page sets and resets it (`ModelSettingsSection.tsx`; applier `set_model_setting`/`clear_model_setting`, `model_settings_authoring.rs`); through-solve tests `operation_applier/tests/model_settings_authoring.rs`, src-tauri `a_tolerance_set_through_the_apply_command_changes_which_nodes_warn` | DEL-04-06, DEL-02-01 | REQ-04-06-012 (owner) is met on both routes, and a desktop user can see the effective value ("5 mm (default)") and change it. |
| §12.8 verified bounds (783) | — | O | Owner, 2026-10-23: "no verified bounds in v0.1; the retained-precision method stays dormant and owner-held." (`DEL-04-08 Design/b7-plan.md:34`; SOW:336, AX-004) | DEL-04-08 | Dormant code is in `product_physics/src/retained_*.rs`. |
| §22.2 benchmarks (1403) | — | P | 13 of 14: tip force, tip moment, fixed-fixed thermal, simply supported under weight (W9-B, both routes), inclined member, L-frame (W9-A, both routes), U-loop, torsion, anchor displacement (W9-C, EX only), spring, gap, friction, branch | DEL-09-01..03 | **Missing:** rigid link (after W-15). Anchor displacement on PF waits for FR-SOL-006 stage 1. |
| §22.3 stress benchmarks (1422) | — | D | `validation/benchmarks/stress/src/lib.rs:489,513,546,570`; pressure hoop and longitudinal stress in the frozen references `validation/references/t4_u6` (EX); t4_u4b; the suite's pressure family `STRESS-PRESSURE-HOOP-LONGITUDINAL-ORIGINAL` (W-20, #1599: Lamé hoop and radial, closed-end, open-end and restrained membrane, on both section bases, through the exact-pressure preview entry) | DEL-09-02 | The headless runner has no binding for the pressure family (its comparison is the suite's relative check), so it fails closed there. |

### 2.4 Rule packs, reporting, handoff and export (PRD §11.5, §11.7, §11.9, §16–19)

| ID (PRD line) | Pri | Class | Evidence | Deliverables | Notes |
|---|---|---|---|---|---|
| FR-RULE-001 (530) | Must | P | `schemas/rule_pack.schema.yaml`; `RulePackManagerPanel.tsx:302-602`; `e2e/r2-smoke.spec.ts:1175` | DEL-06-01, DEL-07-03 | No stress categories, load-case mappings or required load cases (§17.2). |
| FR-RULE-002 (531) | Must | D | `rule_pack_document/src/lib.rs:1184` calls `check_rule_pack_units` (`:1482`); UI `RulePackManagerPanel.tsx:503-560`; run time `rule_check_runner/src/lib.rs:1286`. **Tests:** `tests/unit_check.rs`, `RulePackManagerPanel.unitCheck.test.tsx`, desktop `lib.rs:8452,8499` | DEL-06-02 | Static findings do not block save or run; an inconsistent check still blocks when it runs. A dimensionless limit declared `1` does not match a ratio formula's `ratio`. |
| FR-RULE-003 (532) | Must | D | `rule_check_runner/src/lib.rs` (`aggregate_rank`, `status_counts`); desktop `RuleCheckRunPanel.tsx` headline; renderer `user_rule_check_aggregate_headline`. Tests: runner unit tests and `tests/point_path_non_finite_run.rs`, `report_renderer/tests/render.rs`, `RuleCheckRunPanel.test.tsx` ("FR-RULE-003 pack headline"); observed natively on both routes (§5) | DEL-06-03 | W-19 (#1585): while any check is incomplete the pack headlines "INCOMPLETE — n check(s) failed; m check(s) blocked", with each check listed (owner reading, 2026-10-23; DEL-06-03 Design `pack-aggregate-incomplete.md`). A check with missing inputs is never evaluated. |
| FR-RULE-004 (533) | Must | D | `rulePackService.ts:241-284` (private by default); local SQLite | DEL-06-04 | No pack file export. |
| FR-RULE-005 (534) | Must | D | `LibraryManagerPanel.tsx`; desktop `lib.rs:8593`; `tests/test_library_import_provenance.py`; `LibraryApplySection.tsx`, `libraryApply.ts`, applier `library_apply.rs`; tests `libraryApply.test.ts`, `operation_applier/tests/library_apply.rs` | DEL-03-01, DEL-03-07 | Records feed rule checks (desktop `lib.rs:3029`). Applying them to model entities (#1566): stored material and section records insert as new entities or apply to the selected one through one reviewed batch with one undo (material E/ν or E/G mode; a section apply updates its bound pipes). |
| FR-RULE-006 (535) | Should | D | Same panel, component kind; `libraryApply.ts` `buildComponentApply` | DEL-03-02, DEL-03-07 | Applying records (#1566): stored component records apply to an existing component of the matching kind by a fixed source-kind table (apply only; no insert). |
| FR-RULE-007 (536) | Must | D | `tests/test_invented_example_models.py`; `examples/rule_packs/invented_demo.yaml` | DEL-11-04, DEL-06-05 | |
| FR-REP-001 (578) | Must | P | W-3: local-private render (`RenderedReportPanel.tsx`, `reportRenderService.ts`); rule-pack refs, user-rule outcomes, versions, date, settings and coordinate systems (`renderableReportInput.ts:362-470,637,685`; renderer test `render.rs:321`). Observed natively (§5) | DEL-08-01, DEL-08-06 | A public render is blocked for every session (provenance hard-coded private, `renderableReportInput.ts:119-131`). EX refused (10A). Package refused (`reportPackageRequest.ts:30`). `assumptions: []`; build id and model coordinates "not recorded"; no library refs. |
| FR-REP-002 (579) | Must | P | Rule-check section `report_renderer/src/lib.rs:845-904`; §19.3 notice and human-review record `:954-990`; test `render.rs:390` | DEL-08-01 | Reached only in the local-private PF render. |
| FR-REP-003 (580) | Must | P | Provenance notes for session, components and spring hangers (`renderableReportInput.ts:537-545`) | DEL-08-03 | Materials, sections and library records absent. |
| FR-REP-004 (581) | Should | P | Within-run comparison summary (`ReportPanel.tsx:210-214,663-665`) | DEL-08-06 | No comparison section in the renderer; the package that carries one is refused. |
| FR-REP-005 (582) | Must | D | `core/reporting/protected_content_linter`; `tests/test_report_protected_content_linter.py` | DEL-08-05 | |
| §19.1 report types (no row before) | — | P | — | DEL-08-06 | No state-comparison, run-comparison or handoff-manifest report a user can reach. |
| FR-HAND-001 (558) | Must | P | W-6a: File › Save Model Document… saves the canonical model document on either route with no solve (`modelDocumentSave.ts`, `model_document_save.rs`); `HandoffPanel.tsx:211-283`, `NativePackagePanel.tsx:217-229` produce preview documents; `core/handoff/exporter/workflow.py:57` is called only by tests | DEL-15-01, DEL-15-03, DEL-17-03, DEL-07-12 | W-6b: File › Save Handoff… writes a verified directory package (model member plus schema manifest) on a saved model state (`handoff_package.rs`, `handoffPackageSave.ts`). Runs are not referenced in v0.1 (`AnalysisRunRef` const). The preview panel needs a solved PF result; EX refused (10A). |
| FR-HAND-002 (559) | Must | P | W-6b manifest: model hash, units manifest, entity ids, library and rule-pack references, warnings, diagnostics and unresolved assumptions from the document (`handoff_package.rs`); witness `tests/test_handoff_package_witness.py`; verifier and tamper checks | DEL-15-01 | Hanger library kind has no schema value (`TBD`); run references are empty. |
| §16.1–16.3 handoff contents and workflow (no row before) | — | P | — | DEL-15-01 | Export hash, model hash, rule-pack manifest and entity manifests in schema form (W-6b); the basis model state (W-12 S1); state names, tags and notes (W-12). Run references wait for an `AnalysisRunRef` contract edit. |
| FR-HAND-003 (560) | Should | P | `HandoffPanel.tsx:243` `target_field_coverage: "TBD"` | DEL-15-02 | |
| FR-HAND-004 (561) | Should | P | Loss reports (`HandoffPanel.tsx:187`, PCF) | DEL-15-02, DEL-17-07 | PF only. |
| FR-HAND-005 (562) | Could | D | PCF, MBF, adapter and SDK panels (`App.tsx:1009-1017`); observed natively (§5) | DEL-17-04/07/08/09 | The MBF target version is `TBD` (`CaepipeMbfExportPanel.tsx:38`). |
| Result JSON, stress-neutral | — | D | `App.tsx:1007-1008`; `exactOutputTwins.test.tsx`; observed natively (§5) | DEL-08-04, DEL-17-06 | |
| Local FEA handoff | — | O | DEC-078 (D-12 O-B): FR-025 post-beta (DEL-10-03 SOW:486) | DEL-10-03 | A gated panel exists. |

### 2.5 States, runs, comparison, V&V (PRD §11.6, §15, §21–22)

FR-CMP-001..004 and -006 are in v0.1 by OD-1 (unit W-12, design
`DEL-14-01/Design/desktop-states-runs-comparison.md`). FR-CMP-005, -007, -008, -009
and -010 remain in v0.2 R3 under that design and PRD §24. FR-CMP-011 is not an
MVP requirement (PRD §15.6).

| ID (PRD line) | Pri | Class | Evidence | Deliverables | Notes |
|---|---|---|---|---|---|
| FR-CMP-001 (542) | Must | P | **Store:** desktop `src-tauri/src/model_states.rs` (store v12: `model_states`, `model_state_labels`, immutability triggers; save, list, read, label edit, delete), with its tests there. **UI:** `features/history/HistoryPanel.tsx`, `useModelStateHistory.ts`; `modelStateHistory.session.test.tsx`. **Record:** `model_state.schema.json` record with Python-pinned record hash; `tests/test_model_state_record_parity.py` | DEL-14-01, DEL-07-08 | Save, reopen, tag and delete named immutable states in the desktop (W-12 S1, #1589); SQLite-only, with history export open under OI-015. Compare is W-12 S3. The desktop crate's Rust store tests do not run in hosted CI. |
| FR-CMP-002 (543) | Must | P | **Store:** desktop `src-tauri/src/analysis_runs.rs` (store v13, Record Run, list, read, label edit, delete) with `analysis_runs_tests.rs`. **UI:** `features/history/HistoryPanel.tsx`; `recordRun.session.test.tsx`. **Record:** `analysisRunCompatibility.ts` recorded form, `core/analysis_runs/compatibility.py`, `tests/test_recorded_run_record.py`. **Rule packs:** `services/modelRulePackResolution.ts`; `features/workspace/solveRulePacks.session.test.tsx`; fixtures `persistence/transient_run_input_rule_packs.json`, `recorded_run_witness_rule_packs.json` | DEL-14-02, DEL-07-08 | Record Run (W-12 S2, #1596) stores an immutable v0.3 revision tied to a saved state (`{ModelState, id}`), checked against the stored input manifest's model payload. The manifest binds the solver, settings, units and load cases, and many runs can be recorded per state. Library refs come from the shared `library_ref` walk. Rule-pack refs come from the manifest's `active_rule_packs`: at solve the session resolves the model's `rule_pack_refs` against the local rule-pack store (`modelRulePackResolution.ts`), in the model's order; an unresolved or changed reference is excluded and recorded as a run-record diagnostic (`MODEL-RULE-PACK-REF-UNRESOLVED`, `MODEL-RULE-PACK-REF-CHECKSUM-MISMATCH`), kept by the recorded revision under its hash (#1602). Exports and the handoff do not yet carry those diagnostics (reports lane). |
| FR-CMP-003 (544) | Must | P | `core/comparison/model_state/engine.py:64`; `tests/test_model_state_comparison.py` | DEL-14-03 | Headless only. |
| FR-CMP-004 (545) | Must | P | `core/comparison/analysis_run/engine.py:165`; `tests/test_analysis_run_comparison.py` | DEL-14-04 | Headless only. |
| FR-CMP-005, -007 (546, 548) | Should | P | `schemas/comparison_mapping.schema.json`; `model_state/engine.py:232` | DEL-14-05 | No editor. |
| FR-CMP-006 (547) | Must | P | `analysis_run/engine.py:229` | DEL-14-03/04 | Desktop uses it within one run only. |
| FR-CMP-008, -009, -010 (549-551) | Should | P | `schemas/comparison_tolerance.schema.json`; `ComparisonPanel.tsx:55-70` | DEL-14-05, DEL-08-06 | No profiles, overlays or CSV. |
| FR-CMP-011 (552) | Could | P | `schemas/caepipe_external_run.schema.json` | DEL-14-05 | §15.6: not MVP. |
| §21.3 human acceptance (1385) | — | D | `docs/claims_registry.md:6,184-186` | DEL-01-04 | No acceptance workflow, as required. |
| §22.4 regression (1435) | Must | P | `tools/release/run_evidence_sweep.py:163`; last sweep `validation/evidence/sweeps/SWEEP_20260924T162044Z_b9dda2addccc.json` | DEL-00-08, DEL-09-05, DEL-10-04 | No states to hash. The sweep is optional release preparation; the hosted `harness` is the merge check. |
| §22.5 prover correlation (1455) | — | O | DEC-080: activation is a future owner-gated row | DEL-09-04, PKG-15 | The schema lacks the §22.5 fields. |
| R6 reproduction (1636) | — | P | Current runnable procedure `docs/validation_manual/headless_runner_reproduction.md:47-193` (W-2); last bundle still `validation/evidence/reproduction/REPRO_DEL0904_20260720T074714Z_a5235340aae3/` | DEL-09-04 | No fresh bundle (the heavy half of W-2). |

### 2.6 Data boundary, local operation (PRD §20)

| ID | Class | Evidence | Deliverables |
|---|---|---|---|
| Public boundary and provenance (§20.1–20.2, no row before) | D (repository level) | `tools/release/run_release_candidate_scan.py`, `export_public_openpipestress.py`; `tests/test_release_candidate_scan.py`, `tests/test_export_public_openpipestress.py` | DEL-12-05, DEL-10-04 |
| Local-first, private paths (§20.3–20.4) | D | SQLite store in Application Support; no `fetch`, `XMLHttpRequest`, `WebSocket` or `sendBeacon` in non-test `apps/desktop/src`, and no HTTP client crate in `src-tauri/Cargo.toml` | DEL-12-01, DEL-12-04 |
| Telemetry (§20.5) | D | `telemetryPolicyService.ts`; `tests/security/test_telemetry_policy.py` | DEL-12-03 |
| Unit system (MVP) | D | `core/units`; `unitConversion.ts`; `ProjectUnitsDialog.tsx` | DEL-02-02 |

`tauri.conf.json:23` sets `"csp": null`. No outbound path was found in code, but
the webview has no network policy.

### 2.7 Class counts, MVP (PRD §23) and milestones (§24)

**Class counts.** The #1539 map at `7a780551f0` had 48 P, 28 D, 3 O and no R.
This map has 41 P, 35 D, 8 O and no R:
- **P → D (6):** FR-GUI-001 (#1556), FR-GUI-004 (#1563), FR-KNOW-002 (#1559),
  §10.1 step 1 (#1569), the R2 exit (W-4, #1582, after the basis), and §12.8
  near-duplicate nodes (the desktop tolerance control, after the basis). The
  first two and the last were updated in the map by their own changes.
- **D → P (2):** FR-SOL-002. Its acceptance criterion includes imposed
  displacements, which pass on EX only; the route rule now applies (Codex, §6).
  (FR-RULE-003 moved D → P by the owner's reading of 2026-10-23 and back to D
  with W-19, #1585.)
- **P → O (5):** FR-KNOW-001, -003, -004 and -005 (OD-1/OD-3); FR-AGENT-005
  (OD-5).
- **P → D (bounded) (1):** FR-MOD-007, by the owner's reading of 2026-10-23.
- **New rows (5):** §12.3 reducers, §12.5 load types, §16.1–16.3 and §19.1 (P);
  §20.1–20.2 (D).
- **Reason changed, class kept:** FR-CMP-001..006 lose "scope unclear" (OD-1);
  FR-SOL-015 now includes the health indicator.

**§23 MVP items:**
- **In the desktop:** 22 of 28 items. Imposed displacement works on EX only;
  combinations on PF only; the validation suite is headless.
- **Not in the desktop:**
  - rigid elements (W-15);
  - basic state comparison and basic run comparison (headless only; W-12);
  - runs tied to states (partial; W-12);
  - basic report generation (local-private PF render only; W-11 and the
    package);
  - canonical model package export (the model document, W-6a; the handoff
    directory package with its manifest, W-6b).

**§24 milestones, factually:**
- **R0:** met.
- **R1:** partly. Rigid elements are absent; §22.2 lacks the rigid link.
- **R2:** ruled met under v0.1 tokens (D-14/DEC-035). The native smoke now runs
  a GUI-authored model through solve, review, save and reopen (W-4).
- **R3 (States, Runs, Comparison):** not met; states, run history and basic state/run comparison are in v0.1 by OD-1. Mapping editor/manual mappings, tolerance profiles, graphical diffs and comparison export remain in v0.2 R3.
- **R4** and **R5:** met as v0.1 R3/R4 (D-23/DEC-048, D-27/DEC-054). A report
  is obtainable only as a local private PF document.
- **R6:** partly. Design knowledge is deferred (OD-3); no fresh reproduction
  bundle.
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
- **Formats and stores:** the desktop project store (partly DEL-12-01).
- **Solver:** the rigid link; global-frame element-end forces (no deliverable
  commitment; W-7 is building them under DEL-04-01); component-weight load
  application; the eccentric reducer offset.
- **Model and inspector:** the "lines" entity; inspector dependencies and
  assumptions.
- **Records:** the §22.5 correlation-record fields; the R3 exit review.

Since the previous map, DEL-02-01 owns the desktop model document format
(#1554), and DEL-04-06 owns topology and near-duplicate node checks (W-8,
REQ-04-06-012).

## 3. Reconciliation drift

The W-1 sweep (#1540–#1546, #1550, #1557, #1560, #1575) fixed the previous
map's pointer and ScopeOfWork items, including those for DEL-06-01, DEL-07-01..08,
DEL-08-01/02/05, DEL-15-01/03, DEL-17-03/04/07/08, DEL-02-01, DEL-02-05,
DEL-03-04/05/08, DEL-05-05, DEL-09-01..03, DEL-14-01/03..05, DEL-16-04, DEL-01-04
and DEL-11-02/03/05. W-2 (#1543) fixed the validation manual items. The rows
below are what remains or is new at this basis.

**Fixed in this change** [fix]:
- `docs/user_guide/index.md`: said the rendered report is always blocked and
  that a blank project starts in SI. It now describes the local-private render,
  its content, and the units dialog.
- DEL-03-05 SOW Attributes: "Realization also includes the DEC-045 product
  route … with candidate-bound evidence" contradicted the current-implementation
  paragraph; it now says the route validates but does not apply the scaling.
  The paragraph's basis and `validation.rs` line range are current.
- DEL-02-02 `deliverable.yaml`: `code_paths` add `features/project-units/**`
  and `projectService.ts` (its checks already listed the tests).
- DEL-15-02 `deliverable.yaml`: `code_paths` add `HandoffPanel.tsx` and
  `PcfExportPanel.tsx` (the loss reports).
- DEL-12-01 `deliverable.yaml`: `code_paths` add `tauri.conf.json` (the CSP
  setting).

- `DEL-02-01/Design/model-document-0.7.0.md:51-52` said `schemas/model.schema.yaml`
  "describes an older shape … The bundled demo fails it with 11 errors", stale
  since #1554. It now records that state as before S1a.
- `DEL-02-02/Design/project-unit-selection.md` §1 "Where things stand" ("Blank
  projects fix SI", "No `angle` key is written") described the state before
  slice (a). It is now headed as the starting point, with one sentence on what
  slice (a) changed. Without this, the §10.1 row would be R (Codex, §6).
  #1565 and #1566 edit other hunks of these two files.

**Done in #1574** [fix]: `DEL-04-06/Design/model-topology-checks.md` and
REQ-04-06-012's verification row (SOW:189) carry the implemented wording.

**Still open** [fix]:
- **Release-notes draft** (`docs/RELEASE_NOTES_v0.1.0_DRAFT.md`, release lane;
  release is parked):
  - `:102` "Rendering is refused when the public-report redaction control would
    have to withhold values." The public render is blocked for every session,
    and the local-private render (with rule-check and run context) is not
    mentioned.
  - `:106` lists glTF review geometry under "For pressure-free results also".
    `exactCurrentPanels.test.tsx:7-8,153-157` offers it for exact Current
    results too. The §5 probe saw no glTF action on either route, because the
    byte-exact downloads are withheld.
- **Code comment:** `product_physics/src/validation.rs:1367` "no stiffness
  scaling is silently applied without a valid mapping" (and the info text at
  `:1407`) implies scaling is applied when a mapping is valid; it never is.
  This is product code and needs a DEC-081 claims check. It goes with W-15.
- **Pointers:** DEL-10-04 lists `software-workflow.json` and `tools/ci/**` but
  not the root `.github/workflows/piping-*.yml`. DEL-07-02/05/08 still list the
  unused `core/gui/*` reference modules beside the desktop paths. DEL-13-03
  names only Python for connectivity; the product check is DEL-04-06's.
- **Corpus-wide retired paths:** "Sources:" prose and the line-12
  "Reconciliation evidence" prose of 81 SOWs cite `execution/_Reconciliation/…`
  without annotation. These are outside #1550 and row 26 of the previous table.
  Identification and Attributes boilerplate stays as written (Agent 0,
  2026-10-23).

**Owner-reserved text (exact old → new).** These change commitment,
scope or acceptance wording. "Rec." is the recommendation.

| # | Location | Old | New | Reason | Rec. |
|---|---|---|---|---|---|
| 1 | `apps/desktop/native-smoke/expected.json`, milestone `sparse_interactive.rows` | `98` | `113` | #1461 (T4-U4b/U5b) adds 15 σ±/shear rows per member per case (`DEL-04-07 Design/u4b-u5b-implementation.md:296`); milestone: 1 member × 1 case | Applied (owner, 2026-10-24: "I accept your recommendations for each of these three: G10, W-12, W-18.") |
| 2 | same, milestone `dense_scrutiny.rows` | `99` | `113` | #1461 +15; #1387 (B7 E14) deleted the dense parity row, −1 (`DEL-04-08 Design/b7-plan.md:292`) | Applied (owner, 2026-10-24: "I accept your recommendations for each of these three: G10, W-12, W-18.") |
| 3 | same, milestone `sparse_interactive.probe_entered` | `"Entered: 0.09426732782247198 mm"` | `"Entered: 0.09426732782247199 mm"` | #1387 (B7 E15): one correctly rounded `norm3` (`b7-plan.md:293`); one unit in the last printed digit. The dense value is unchanged | Applied (owner, 2026-10-24: "I accept your recommendations for each of these three: G10, W-12, W-18.") |
| 4 | same, demo `sparse_interactive.rows` | `625` | `745` | #1461 +120 (4 members × 2 cases × 15) | Applied (owner, 2026-10-24: "I accept your recommendations for each of these three: G10, W-12, W-18.") |
| 5 | same, demo `dense_scrutiny.rows` | `627` | `745` | #1461 +120; #1387 −2 parity rows (one per case) | Applied (owner, 2026-10-24: "I accept your recommendations for each of these three: G10, W-12, W-18.") |
| 6 | DEL-09-01 SOW:69 | "The future mechanics benchmark suite must stay aligned…" | "The mechanics benchmark suite must stay aligned…" | The suite exists (`validation/benchmarks/mechanics`, W-9) | Applied (owner, 2026-10-23: "I approve items 6-15 in the form above.") |
| 7 | DEL-09-01 SOW:135 | "This deliverable specifies setup evidence for a future mechanics benchmark suite covering…" | "This deliverable specifies the contract and evidence basis for the implemented mechanics benchmark suite covering…" | as #6 | Applied (owner, 2026-10-23: "I approve items 6-15 in the form above.") |
| 8 | DEL-09-02 SOW:81 | "This setup kit defines the future benchmark-suite boundary only. It does not create benchmark source files, implement tests, …" | "The original setup kit defined the benchmark-suite boundary only. It did not create benchmark source files, implement tests, …; the suite now exists in `validation/benchmarks/stress`." | the suite exists | Applied (owner, 2026-10-23: "I approve items 6-15 in the form above.") |
| 9 | DEL-09-02 SOW:126 | "This deliverable specifies setup evidence for a future stress recovery benchmark suite. It covers benchmark planning for…" | "This deliverable specifies the contract and evidence basis for the stress recovery benchmark suite in `validation/benchmarks/stress`. Its required coverage remains …; implemented fixture coverage is recorded in the suite's fixture inventory." | as #8 | Applied (owner, 2026-10-23: "I approve items 6-15 in the form above.") |
| 10 | DEL-09-03 SOW:108 | "…the setup surface for a future nonlinear support regression suite…" | "…the contract and evidence basis for the implemented nonlinear support regression suite…" | conflicts with its own current-implementation paragraph | Applied (owner, 2026-10-23: "I approve items 6-15 in the form above.") |
| 11 | DEL-08-03 SOW:81 | "The future report section is expected to be a report-renderer subsection…" | "The report section is a report-renderer subsection…" | ships, as its `:99` says | Applied (owner, 2026-10-23: "I approve items 6-15 in the form above.") |
| 12 | DEL-08-06 SOW:343 | "ASSUMPTION: the future implementation should assemble sections…" | "ASSUMPTION: the implementation should assemble sections…" | `report_package` exists | Applied (owner, 2026-10-23: "I approve items 6-15 in the form above.") |
| 13 | DEL-06-02 SOW:77 (Determinism) | "Future implementation must produce repeatable results…" | "The implementation must produce repeatable results…" | evaluator ships | Applied (owner, 2026-10-23: "I approve items 6-15 in the form above.") |
| 14 | DEL-14-02 SOW:170, :320 | the "PRD v0.2 … not locally accessible" source row / "the PRD source text was not locally read in this task" bullet | Source "PRD analysis-run requirements", status "Current primary source: docs/PRD.md §§8.7, 11.6 (FR-CMP-002), and 15.2", with resolution guidance / "Use `docs/PRD.md` §§8.7, 11.6 (FR-CMP-002), and 15.2 for analysis-run requirements; retain explicit TBDs…" | `docs/PRD.md` is present | Applied (owner, 2026-10-23: "I approve items 6-15 in the form above.") |
| 15 | DEL-14-01 SOW:342 (DEL-14-01:3); DEL-15-01 SOW:387 (DEL-15-01:2) | "(stage-gated: v0.2 R3) " / "(stage-gated: v0.2 R6; Owner ruling," | deleted / "(Owner ruling," | the tag and the ruling read as mixed signals | Applied (owner, 2026-10-23: "I approve items 6-15 in the form above.") |
| 16 | DEL-15-01 SOW:183 (V-02) | "Confirm all non-trivial schema requirements trace to `_CONTEXT.md`, decomposition, …" | "… trace to `_CONTEXT.md` (retired in `497a8d81e4`; see Git history), decomposition, …" | the row-26 annotation, in a verification check cell | Apply — **Applied** (Agent 0, 2026-10-24, under the row-26 rule; #1582) |

**Previous table, now closed.** Rows 1, 3–21, 23, 24 and 26 are applied. Row 2
stays not applied by the owner's ruling (FR-SOL-006 is built on PF instead), and
row 22 stays Leave. Row 25 (`expected.json`) is rows 1–5 above, now with each
cause named.

**Owner readings recorded** (owner, 2026-10-23: "I accept your other
recommendations for the matters you needed from me"):
- **PRD §14.4 topology:** "topology invalid" is topology that prevents the
  mechanics being formed (unknown pipe endpoints, unsupported parts, invalid EX
  pressure-region chains), which already blocks. W-8's other findings are
  warnings and REQ-04-06-012 stands (DEL-04-06 Design `model-topology-checks.md`).
  Two follow-ons go to the numerical lane: the malformed-branch warning will say
  the component's branch stress factors are still applied, and the bundled
  demo's malformed tee C-120 is fixed.
- **FR-MOD-007:** D, bounded to the live derivation of `BuiltModel`; the PKG-13
  transform contract stays deferred (§2.1).
- **FR-RULE-003:** the aggregate headlines INCOMPLETE while any check is
  incomplete (DEL-06-03 Design `pack-aggregate-incomplete.md`; built in W-19).

**Further SOW wording, applied** (owner, 2026-10-23: "I approve the 24 items as
above (14 as drafted, 9 amended, and the :110/:117 alternative in place of
:114)"): stale "future" and setup-only wording in DEL-09-01, DEL-09-02,
DEL-09-03, DEL-08-03, DEL-08-06 and DEL-06-02, with still-open `TBD`s, held
choices and obligations for new cases kept.

**Owner notes (not text changes):**
- REQ-04-06-012 (5 mm the user can change: met in the document, not yet in the
  desktop) and DEL-04-03 R01 (imposed displacement) state ruled commitments
  that the code does not yet fully meet. Both are disclosed, and the work
  is in flight or planned (FR-SOL-006 stages); REQ-04-06-012's desktop control is
  not yet in any unit.
- DEL-13-03-R1 (deferred PKG-13) still commits connectivity, which DEL-04-06
  now delivers.

## 4. Proposed work plan

### 4.1 Ready for development testing now

These are class D outcomes with a desktop path, which development testing can
exercise today:
- **Model and project:** open, save and reopen model documents and projects;
  choose project units at creation.
- **Authoring:** nodes, pipes, materials, sections, bends, joints, supports,
  load cases and combinations; split, copy, rotate, mirror and reconnect; undo
  and redo. A from-blank model solves natively (§5).
- **Solving:** PF solve, including nonlinear supports, wind, seismic and
  combinations; EX solve; pre-solve topology warnings; refusal of load cases
  that apply nothing.
- **Results:** results table, standing, numerical health indicator,
  diagnostics, deformed shape, result colour and reaction arrows.
- **Reports:** the local-private rendered report for PF results.
- **Proposals and data:** offline proposals; libraries.
- **Rule packs:** authoring, static unit check and rule-check runs.
- **Exports:** result JSON and stress-neutral JSON (both routes); PCF and MBF
  package documents and the other PF export panels. Stress-neutral CSV, glTF,
  PCF text and MBF text are withheld by the desktop.
- **Boundaries:** local-only storage and telemetry.

The native smoke's five stale `expected.json` values (§3 rows 1–5) are applied,
so a run on main shows no known failure.

These are **not ready**: shareable or EX reports, the report package, model
states and comparison, canonical model export, rigid components, EX
combinations, PF imposed displacement, global-frame end forces and the 0.7.0
document slots.

### 4.2 Owner decisions that shape the plan

These are reserved because they concern v0.1 scope, commitments or a hold.

**OD-1 to OD-5 decided.** Owner, 2026-10-23: "I approve OD-1 through OD-5 as
you recommended." Each outcome is in the row's **Decided** entry. OD-6 was
decided the same day (option B), and rows 2 and 23 of the §3 table were then
ruled separately (see the OD-6 row).

| # | Decision | Consequence |
|---|---|---|
| OD-1 | Does D-11's "W7 — Forward horizon v0.2" placement (PKG-13/14/15/16/17) mean outside software 0.1.0, despite PRD §23 listing states, runs tied to states, state and run comparison, and canonical model export in the MVP? | **In:** unit W-12 (L) and W-6 come before validation. **Out:** a ruling, and PRD §23 and the release-notes text follow it. Either answer also settles OD-3 for PKG-13. **Decided:** states, run history, state and run comparison and canonical model export are in v0.1 (W-12, W-6). PKG-15 handoff stays in for model export. PKG-13, further PKG-16 agent features and further PKG-17 export formats are deferred to v0.2; the DEL-16-01/02 offline intake and the DEL-16-06 operation-application seam ship as built. PRD §23 stays as written. |
| OD-2 | Rigid elements (FR-SOL-004, Must, MVP) | **Options:** build DEC-045's stiffness scaling (M); build a dedicated rigid element and offsets, which reopens DEC-045 (L); or defer by ruling. **Decided:** implement DEC-045 stiffness scaling (W-15). |
| OD-3 | Design-knowledge entry and constraint validation (FR-KNOW-001, Must; R6) in v0.1? Settled with OD-1 if the W7 reading applies. | **In:** W-14 (L). **Out:** a ruling. **Decided:** design knowledge is deferred to v0.2. |
| OD-4 | Which model format is canonical: the desktop 0.6.0 document or `schemas/model.schema.yaml`? (FR-MOD-001/004/006) | Fixes the target for W-13, a new document version under the one-version rule. **Decided:** the desktop model document is canonical, and `schemas/model.schema.yaml` is updated to describe it (W-13). |
| OD-5 | Viewport result overlays and reaction arrows (FR-GUI-001, Must); the R7 staging of the FR-AGENT-005 claim scan (DEL-16-04:3) | Each is either an S–M unit or a ruling. **Decided:** viewport overlays and reaction arrows are built now; FR-AGENT-005 stays R7. |
| OD-6 | Owner-reserved text in §3 | Apply as proposed, amend, or leave. **Decided (owner, 2026-10-23, "OD‑6, option B (I want to discuss 2 and 23 more)."):** the 22 Apply rows are applied. Owner, 2026-10-23: "Row 2 I approve option A and Row 23 I approve your recommendation." Row 2 is not applied (the requirement stays; imposed displacement is to be built on the pressure-free route, linear first, then with nonlinear supports); row 23 is applied. |

Also decided on 2026-10-23 and recorded in the governing text: the 5 mm
node-proximity tolerance, user-settable (REQ-04-06-012); no verified bounds in
v0.1 (`DEL-04-08 Design/b7-plan.md`); the W-10 reconnect limitation
(DEL-16-06 SOW:79).

Opening exact-route reports and handoff needs no new decision to start. Ruling
10A already directs DEL-08-01 to design their limitation sections. Opening the
surfaces afterwards returns to the owner.

### 4.3 Units

The lane is heavy where builds or numerical checks are needed and light for
documents. Heavy work that changes numerical meaning or persistence needs
targeted regression checks and independent scrutiny (Root `AGENTS.md`).

**Done on main** (evidence; the owner's acceptance is separate):

| # | Unit | Evidence |
|---|---|---|
| W-1 | Pointer and ScopeOfWork reconciliation sweep | #1540–#1546, #1550, #1557, #1560, #1575; remaining drift in §3 |
| W-3 | Rendered report for local private sessions, with §19.2 content | #1553 (+#1555); observed natively (§5) |
| W-5 | Memory-pool headroom (transient free-queue dip) | #1547; the demo result-JSON save succeeded in the smoke and the probe (§5) |
| W-8 | Pre-solve topology and near-duplicate warnings | #1559; the absolute 5 mm, model-settable tolerance and the branch warning's applied-factor statement are #1574; the desktop control that sets and resets the tolerance is branch `claude/piping-tolerance-control-20261024` |
| W-9 | Missing benchmarks: L-frame, simply supported weight, anchor settlement | #1548; the rigid link waits for W-15 |
| W-10 | Pipe reconnect | #1563, with its declared limitation |
| — | Viewport result colour and reaction arrows (OD-5) | #1556 |
| — | Numerical health indicator (FR-SOL-015 display) | #1544 |
| — | Refusal of load cases that apply nothing (silent zero) | #1568 |
| — | Strict inputs: no silent drops of authored input or combination rows (PF closed records, `UNKNOWN_MODEL_FIELD`; duplicate terms and withheld combination rows named) | #1573 |
| W-17 (part) | Project units at creation; static rule-pack unit check | #1569, #1570 |
| W-2 (part) | Validation manual current procedure, user guide, SMOKE note | #1543; the user guide updated again in #1580 after W-3 and W-17 |
| W-19 | Rule-check aggregate headlines INCOMPLETE while any check is incomplete (FR-RULE-003) | #1585 |
| W-13 S1b | Model document 0.7.0: the new member slots, one document validator, `model_settings.node_proximity_tolerance` (default 5 mm) | #1564; native smoke with no change beyond the five stale `expected.json` values (§3 rows 1–5) |
| W-13 S2a | Authoring for the 0.7.0 slots: lines, assumptions and open issues, constraints, equipment interfaces, rule-pack references, tags, notes and provenance records; validator findings in the desktop | #1565 |
| W-13 S2b, W-17 (part) | Applying library records to model entities: material (E/ν or E/G), section (propagated to bound pipes) and component records, with `provenance_record`/`library_ref` and the store's unresolved/changed-record warnings | #1566 |
| W-12 S2 | Analysis runs recorded against saved model states (store v13, Record Run with a checked manifest and model binding, run labels, recorded-form validators in TS and Python), with stored-length read admission for states and runs | #1596 |
| W-12 S1 | Named immutable model states in the desktop (store v12, Save/Open State, History panel with labels); one unsaved-changes guard for Open Project, New, Open Model Document and Open State | #1589 |
| W-4 | From-blank native journey and the output gates as permanent native-smoke checks | #1582: 25 checks, all passing on its head; no `expected.json` change |

**In flight** (not on main):

| # | Unit | PR or branch |
|---|---|---|
| W-7 | Global-frame element-end forces (FR-SOL-007) | branch `claude/piping-w7-global-end-forces-20261023` (no PR) |
| — | DBM concordance review | #1567 |
| W-20 | Pressure stress benchmark family in the DEL-09-02 suite (hoop and longitudinal stress against closed forms) | #1599 |

**Remaining:**

| # | Unit | Size | Kind | Lane | Depends on | Owner? |
|---|---|---|---|---|---|---|
| W-2 (rest) | Fresh R6 reproduction bundle at the validation basis, with the current procedure | S | run | heavy | the validation basis | Acceptance of the reproduction is the owner's |
| W-6 | Canonical model export (save model document to file, independent of a solve; FR-HAND-001); schema-conformant handoff manifest with rule refs and hashes (FR-HAND-002) | M | desktop + schema | heavy | W-13 (0.7.0) | No |
| W-11 | Exact-route reports: DEL-08-01 limitation sections (ruling 10A), then open; with it, the report's materials, sections and library records (FR-REP-003) | M–L | docs + desktop | heavy | W-3 (done) | Opening returns to the owner |
| W-12 | States, run history, state and run comparison in the desktop (FR-CMP-001..004, -006) | L | desktop + persistence | heavy, scrutiny | W-13 (design agreement) | No (OD-1 decided) |
| W-15 | DEC-045 stiffness scaling for rigid and semi-rigid components (FR-SOL-004/012), component weight, EX admission, eccentric reducer offset (§12.3), rigid-link benchmark; fix the `validation.rs:1367` comment | M–L | numerical | heavy, scrutiny | — | No (OD-2 decided) |
| W-16 | Combinations on EX (FR-SOL-009); imposed displacement on PF, stage 1 linear, then stage 2 with nonlinear supports (FR-SOL-006, owner's row-2 ruling; its PF benchmarks also close FR-SOL-002); `equivalent_static` on EX (§12.5) | M | numerical | heavy, scrutiny | — | No |
| W-17 (rest) | Units after creation (slice b, `set_project_unit`); rule-pack §17.2 stress categories, load-case mappings and required load cases | S–M each | desktop / schema | heavy | — | No (§17.2 decided, owner 2026-10-24: "I approve your recommended option A."; stress categories, load-case mappings and required load cases in v0.1, under a new rule-pack format version) |
| W-18 (new) | Report package for current results (FR-REP-001; `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`), and a shareable report path if any (provenance is hard-coded private). Slice (a), the Rust 2.0.0 producer, reader and runner witness, is on main (#1597); slice (b1), native admission and disposition screening of the save (`report_package_save.rs`, shared `export_disposition_screen.rs`), replaces its native hold; (b2) desktop opening remains (DEL-08-01 Design `report-package-current-results.md` §6) | M | desktop + Rust | heavy | W-3 | Shareable path decided (owner, 2026-10-24, option B: a per-report user release, W-18b, after (a)–(b2)) |
| W-14 | Design-knowledge entry and constraint validation | — | — | — | — | Deferred to v0.2 (OD-3) |

**Sequencing:**
- **Before development testing broadens:** the G10 values (§3 rows 1–5) are
  applied by the owner's ruling, so the native smoke proves itself without known
  failures (W-4 is done). The §3 [fix] remainder can go with it.
- **Before the owner's hands-on use:** W-6 and W-18 (a model the owner can
  export and a report package). W-3, W-5 and the W-13 slices (#1564–#1566) are
  done.
- **Before validation:** W-12, W-15, W-16, W-20 (#1599), W-11 and the in-flight W-7.
  The R6 reproduction bundle (W-2) is then made at the validation
  basis.

## 5. Checks run for this map

**Read-only TASK reviews.** Three reviewed the PRD areas against code and tests
at `8a8a8b2584`. Codex then spot-checked a sample of placements independently
(§6). `python3 -m tools.deliverables --project projects/chirality-piping check`
reported no structural errors and one cycle, a strongly connected group of nine
deliverables (DEL-04-07, DEL-04-08, DEL-05-02, DEL-08-01, DEL-08-04, DEL-08-05,
DEL-09-05, DEL-10-04, DEL-10-05). The same cycle exists at `7a780551f0`.

**Native smoke at `8a8a8b2584`.** A fresh build in a heavy-work tree through
`t3_slot.sh`, then `node apps/desktop/native-smoke/run.mjs --skip-build`. 73 of
83 checks passed.
- **All 10 FAILs are stale `expected.json` values,** each failing once on open
  and once after reopen: milestone rows 98→113 (sparse) and 99→113 (dense),
  milestone N1 `…198`→`…199` (sparse), demo rows 625→745 and 627→745. Causes
  are in §3 rows 1–5.
- **Passed:** refusals by name; both routes' fixtures on both solver modes;
  model-hash proofs; the save, replace and stale-base prompts; the demo result
  export (W-5); reopen in a fresh process.

**Hands-on probe.** It used the same build, with an untracked probe driver in
place of `driver.js` and a throwaway HOME.

| Step | Observed |
|---|---|
| Demo (PF) solve | completed, 745 rows, `checks_passed`; "Numerical health: checks passed" |
| Demo render, no local-private selection | `context=public_report; decisions=18082; blocked=true`; no preview, no hash |
| Demo render, local-private selected | canonical HTML SHA-256 shown; "export gates passed"; preview with the LOCAL PRIVATE notice, the software/solver/run section, the numerical-health line and three "not recorded" values; Save canonical HTML and Print offered |
| Report package | Save disabled; `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE` |
| Demo rule checks (bundled pack) | `RULE_INPUTS_INCOMPLETE`, missing inputs named |
| Demo result JSON save (W-5) | saved, 3,636,709 bytes |
| Demo exports | result export, PCF, MBF, adapter SDK, adapter framework, local FEA, native package and handoff offered; stress-neutral CSV and glTF not offered |
| EX model `fixtures/model_operations/exact_pressure_authoring_model.json` solve | completed, 297 rows, `checks_passed` |
| EX render | refused, `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` |
| EX rule checks | run; `RULE_INPUTS_INCOMPLETE` |
| EX exports | result and stress-neutral JSON links only (behind local-private intent); MBF and the rest refused or not offered |
| From blank (W-17, R2) | units dialog (SI, US customary, custom) → node, material, pipe, fixed support, load case and 350 N tip force through the GUI → native solve, 89 rows, `checks_passed`, tip displacement 1.3703288675916625 mm (PL³/3EI ≈ 1.370 mm) → Save Local Project in place → fresh process: reopened, solved again, the same rows and value |

The probe's first run used New Local Project for the save; the store already
held the blank project, so the app asked before replacing it, as designed. The
second run saved in place.

## 6. Independent spot-check

Codex (GPT-6.1-Sol, xhigh, read-only, branch head `289f9039ba` over basis
`8a8a8b2584`) checked 19 placements independently, 7 of them changed class.

- **Confirmed (18)** (FR-MOD-007 and FR-RULE-003 were later re-classed by the
  owner's readings, §3): FR-KNOW-002 (D), FR-MOD-007, FR-KNOW-001 and
  FR-AGENT-005 (O), FR-GUI-001 and FR-GUI-004 (D), FR-MOD-001 (P), FR-SOL-015
  (D), FR-REP-001 (P), FR-RULE-002 (D), FR-SOL-006, -007 and -009 (P),
  FR-HAND-002 and FR-CMP-001 (P), §12.8 near-duplicate nodes (P), §22.2 13 of
  14 (P), FR-RULE-005 (D).
- **Refuted (1):** §10.1 step 1 as D. The DEL-02-02 Design still described
  blank projects as offering no choice, so under the class rules it was R. This
  change corrects that Design text, and the row is D on this branch.
- **Further corrections, folded in:**
  - FR-SOL-002 moves D → P. Its acceptance includes imposed displacements,
    which pass on EX only.
  - The FR-MOD-006 evidence named one schema as the only home of
    `unresolved_assumptions`; it is also in the model-state and handoff
    schemas.
- **`expected.json`:** all five causes are plausible from the records and
  fixtures (15 rows per member per case; parity row removal; `norm3`
  rounding). Codex could not run the app; the observed values come from the
  native smoke in §5.
