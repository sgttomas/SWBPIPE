# DEL-07-06 Memory

## 2026-07-12 - D-41 R5 T5 PDU-045 hold

- Focused evidence confirms the baseline remains deterministic project-owned
  contract review with desktop runtime evaluation not performed and no
  accessibility conformance claim.
- Contrast/readability findings remain warnings; the measurable target is
  `TBD_by_human_project_authority`. No independent usability basis was added,
  so PDU-045 stays `VERIFIED_NOT_VALIDATED`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T5-PDU045.md`.

Current C5 status: C5 is closed by the 2026-06-20 replacement-criterion human
ruling (`DEC-047`) and the R3 exit-chain packet is assembled as
`TP-R3VERIFY-001` / SMOKE TP-MAC-190. Older entries below that say F-4/A3
remain open are historical pre-closure state.

## 2026-06-20 - TP-R4-D1-BENDVIS-001 R4 bend visibility/provenance slice

WORKING_ITEMS landed the first partial D1 bend-object app-absorption slice
under `DEC-045`. The invented preview model now carries user-entered bend
radius, bend angle, bend-plane orientation, geometry source, user SIF,
user flexibility factor, modifier source, component completeness evidence, and
`mechanics_geometry_only` / `user_rule_pack_inputs_only` interface labels for
`component:C-110`.

The desktop app now renders bend/elbow components as a curved viewport glyph
and surfaces the bend fields in the model view, inspector/provenance rows, grid
mode, editor contract, missing-data blockers, rule-check diagnostics,
validation evidence, and native-package witnesses. Component `kind` is
read-only in the component edit/grid path; bend-specific geometry and
user-modifier edits remain review-only operation intents.

Validation passed: focused App Vitest 57/57; full desktop Vitest 19 files /
406 tests; desktop production build with the existing Vite large-chunk warning;
desktop Playwright 18/18. Evidence is recorded in
`_run_records/WORKING_ITEMS_RUN_2026-06-20_TP-R4-D1-BENDVIS-001.md` and
`apps/desktop/SMOKE.md` TP-MAC-278.

Residual: D1 is not closed. Mechanics/stress-recovery multiplier consumption
and report-provenance closure remain for later D1/D8/D9 work. No lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

## 2026-06-20 - TP-R3UX-CREATIONTOOLS-001 object-creation toolbar

WORKING_ITEMS completed the remaining C5.7R CAD-shell builder increment
(tranche 2c). Insert menu commands now arm the same object-creation tools shown
in the viewport command bar instead of only navigating/collapsing panels. The
toolbar exposes Node, Pipe, Support, Component, and Load tools with visible
active state; arming is separate from queueing. Node/Pipe/Component preview
intents still queue ordinary review-only `EditorOperationIntent`s through the
existing operation seam; explicit node/pipe forms, Inspector support creation,
and Load Cases creation stay on their prior contract surfaces.

Validation passed: focused App Vitest 57/57; full desktop Vitest 19 files /
406 tests; desktop production build with the existing Vite large-chunk warning;
desktop Playwright 18/18; dist Playwright 1/1; `cargo check --release`; Tauri
`.app` build; and an 8-second packaged boot probe (`state=SN`,
`stdout_bytes=0`, `stderr_bytes=0`). Evidence is recorded in
`_run_records/WORKING_ITEMS_RUN_2026-06-20_TP-R3UX-CREATIONTOOLS-001.md` and
`apps/desktop/SMOKE.md` TP-MAC-275.

The freshly rebuilt bundle is
`apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`
(`12M`). F-4 and A3 remain open until the human TP-MAC-189 packaged re-pass is
recorded. No lifecycle transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

## 2026-06-19 - TP-R3UX-PACKAGEKIT-002 C5.7R Inc 7 packaged re-pass kit

WORKING_ITEMS rebuilt the packaged macOS `.app` after C5.7R Inc 0-6 and
prepared the human TP-MAC-189 re-pass handoff. The bundle path is
`apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`,
built from commit `60fb533fe`.

Validation passed: `npm run tauri -- build --bundles app`; 8-second packaged
executable boot probe with process state `SN`, `stdout_bytes=0`,
`stderr_bytes=0`, and clean termination. Inc 6/7 closeout evidence also
includes dist Playwright 1/1 and DEC-025 sweep
`SWEEP_20260619T144814Z_48083bd29407-dirty.json`.

Evidence is recorded in
`_run_records/WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-PACKAGEKIT-002.md` and
`apps/desktop/SMOKE.md` TP-MAC-274.

F-4 and A3 remain open until the human TP-MAC-189 packaged re-pass is recorded.
No lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-19 - TP-R3UX-GRIDMODE-001 C5.7R Inc 6 layout grid

WORKING_ITEMS added the committed Inc 6 bulk-tabular Grid mode to the Model
pane. Tree mode remains available; Grid mode exposes editable tables for nodes,
pipes, supports, materials, sections, components, load cases, and combinations.
Changed cells queue ordinary structured review intents through the existing
operation seam; no new backend contract or direct model mutation path was
introduced.

Evidence is recorded in
`_run_records/WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-GRIDMODE-001.md` and
`apps/desktop/SMOKE.md` TP-MAC-273.

Validation passed: focused App Vitest 57/57; full desktop Vitest 400/400;
desktop production build; dev Playwright 18/18; dist Playwright 1/1. The
existing Vite large-chunk warning remains.

Boundary preserved: frontend authoring ergonomics only. No lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-A11YUNITVISIBILITY-001 unit-visibility evidence

- WORKING_ITEMS added explicit unit-visibility evidence to the desktop
  Accessibility Baseline panel and JSON packet.
- The new review row and packet evidence cite DEC-018/DEL-02-02/DEL-07-05/
  DEL-07-06, keep unit labels visible for unit-bearing review surfaces, and
  record `default_units_inferred=false`, `color_only_unit_signaling=false`,
  and `conversion_performed=false`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-A11YUNITVISIBILITY-001.md`
  and `apps/desktop/SMOKE.md` TP-MAC-221.
- Validation passed: focused App Vitest 56/56; focused Playwright 2/2;
  full desktop Vitest 399/399; desktop production build with the existing
  Vite large-chunk warning.
- Boundary preserved: no final accessibility conformance target, desktop
  accessibility-tree audit claim, lifecycle transition, private/protected
  data, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-05-09 Type 2 Implementation

Implemented deterministic accessibility/usability baseline review records under
`core/gui/accessibility/` with focused coverage in
`tests/test_accessibility_usability_baseline.py`.

The implementation consumes existing invented GUI contract records from the
viewport, model tree, editor, warning, results, and solve-execution slices. It
emits sorted pass/warning/fail/not-applicable findings for keyboard path,
focus order, readable labels, warning visibility, result review, solve-state
feedback, contrast/readability target status, and review workflow continuity.

The baseline does not run a live desktop runtime, select the final
accessibility target, mutate GUI/domain/solver records, fill missing
engineering values, or make software authority claims.

## 2026-05-11 TP-RECON-01 Reconciliation

Reconciled archived Tranche M evidence for `DEL-07-06`. DEV-001 REV05 records
show the 2026-05-09 implementation was promoted from working-tree evidence to
committed evidence in commit `bfb3931` (`core: implement tranche m contracts`)
while the deliverable lifecycle stayed `CHECKING`.

Evidence-bearing artifacts were `core/gui/accessibility/__init__.py`,
`core/gui/accessibility/engine.py`,
`tests/test_accessibility_usability_baseline.py`, this deliverable `MEMORY.md`,
and `_run_records/TASK_RUN_2026-05-09_type2_implementation.md`. The
implemented slice remains deterministic accessibility/usability baseline
records over invented GUI contract inputs, covering keyboard/focus paths,
readability target status, warning visibility, result review, solve-state
feedback, and review workflow continuity.

Recorded verification evidence includes the focused accessibility/usability
test, adjacent PKG-07 GUI contract tests, `py_compile` for the accessibility
module and test files, `git diff --check`, and scoped protected/private/
credential/authority-claim scans. Deferred scope remains live desktop runtime
evaluation, final accessibility target selection, GUI/domain/solver mutation,
missing engineering data defaults, solver execution, protected/private data,
and software authority claims.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-07-06`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-06 - PKG-07 Test Discovery Evidence

Recorded parent `TASK-PKG07-TESTDISC-001` evidence for the PKG-07 pytest wrapper
discovery change. DEL-07-06 is represented by
`tests/test_accessibility_usability_baseline.py::test_accessibility_usability_baseline_main`.
The parent transcript reports 11 collected and 11 passing Python tests across
the eight-file PKG-07 suite, passing direct wrapper invocations, passing
`npm test --workspace apps/desktop` results, and passing viewport-editor Cargo
tests.

The DEL-07-06 evidence remains technically supported by that test-discovery
change. This addendum records evidence support only; it does not alter local
lifecycle state, acceptance posture, release posture, professional boundary,
certification, sealing, or code-compliance status. No local
`Review_Findings.csv` was found in the DEL-07-06 deliverable folder.

## 2026-06-06 - CHECKING Readiness Review

Performed a deliverable-local TASK review for `DEL-07-06` CHECKING readiness.
Required reads included the local context, status, references, dependency
surfaces, memory, four-document artifacts, latest local test-discovery evidence,
the PKG-07 test-discovery fan-in record, and the PKG-07 human disposition
record.

Created `_REVIEW.md`, header-only `Review_Findings.csv`, and
`_run_records/TASK_RUN_2026-06-06_DEL-07-06_CHECKING_READINESS_REVIEW.md`.
Recommendation is `MOVE_TO_CHECKING`: current lifecycle state is `IN_PROGRESS`,
active architecture-basis dependencies are satisfied or not applicable, retired
PKG-07 sibling GUI rows are not local blockers, latest test-discovery evidence
supports the DEL-07-06 pytest wrapper and package checks, and no DEL-07-06
local review finding requires disposition.

No lifecycle state was changed. This addendum records readiness-review evidence
only and does not assert product release, professional approval, accessibility
certification, sealing, automatic engineering acceptance, or `ISSUED` status.

## 2026-06-12 - TP-APP-R2-UXSHELL-001 Desktop Shell Usability Repair

Executed the completion-plan A3 usability tranche after the human TP-MAC-141
attempt-2 abandonment ("no logical layout of the panels at all... things are
cut off or don't scroll... many things are unresponsive"). Three repairs in
`apps/desktop`:

- **Workspace IA.** Persistent spatial core (model tree | 3D centerline
  viewport | property inspector) plus an always-visible section navigation in
  A12 journey order: Operation Apply (with queued-count badge), Load Cases,
  Solve, Results, Report, Project, Exports, Audit & Boundaries. Inactive
  sections stay mounted and are CSS-hidden, so form drafts and queue state
  survive navigation. Design-kit basis: PRD section 14.1 surface list and
  14.3 inspector content; DEL-07-02 Specification RQ-001/002 (tree as
  navigation, inspector presents selected-entity fields) and Guidance
  boundary split; DEL-07-06 Guidance "Considerations" major-panel list
  (model tree, property inspector, editor surfaces, solver/diagnostic
  surfaces, results browser, report preview) — those names became the nav
  vocabulary; DEL-07-05 Guidance (result categories navigable, professional
  boundary notices near result status — boundary strip stays permanently
  visible). PRD 14.1 surfaces with no implementation (material/component
  library editors, rule-pack manager) stay absent; no placeholder buttons.
  Implementation decision (PROPOSAL): kits are silent on the concrete
  navigation pattern, so the simplest conventional pattern was chosen —
  horizontal section tabs above a docked panel area, core kept persistent;
  default section is Operation Apply because every authoring step ends
  there.
- **Scroll/overflow.** Structural stylesheet repair (structure map at the
  top of `styles.css`): independently scrolling core panes, scrolling dock
  body, bounded scrolling apply queue, wrapping toolbar/form rows, two-row
  model-tree filter, tooltip-backed boundary-strip ellipsis. Verified in a
  real browser at 1440x920 and 1280x800 with an automated
  horizontal-overflow probe (only tooltip-backed boundary values remain
  ellipsized) and zero console errors.
- **Dead-control audit.** Permanent Vitest regression test
  (`src/App.deadControls.test.tsx`): three shell states, every button must
  click-to-observable-DOM-change or carry an accessible disabled reason;
  aria-pressed="true" toggles exempt. Caught 4: tree-row-project (now
  aria-pressed selection state), cancel-mechanics-preview,
  generate-review-proposal, rendered-report-render (now carry explicit
  disabled-reason titles naming the enabling step). All testids preserved.

Validation (verbatim): Vitest `10 passed (10)` files, `242 passed (242)`
tests; `npm run build` green (`✓ built in 1.30s`); dev e2e `4 passed`
(2 specs x 1440x920 + 1280x800 viewport matrix, navigation driven through
visible controls); dist e2e `1 passed`. Python tests not run: nothing
python-consumed changed.

SMOKE.md: appended TP-MAC-146; TP-MAC-141 received navigation wording
inserts only (steps 2, 9-12, 13, 14, 17 plus a navigation note) — payloads,
expectations, and human execution records untouched. F-4 remains open; the
human TP-MAC-141 re-run decides usability, not this entry. Residuals: dock
height at 1280x800 is usable but tight (~200px, scrolls); canvas gesture
work is a separate A3 scope; WCAG conformance target remains TBD per
DEL-07-06-CF-001. Evidence: `_run_records/TASK_RUN_2026-06-12_1756.md`.

No lifecycle state change; no release-readiness, professional approval,
accessibility certification, sealing, authentication, or code-compliance
claim.

## 2026-06-17 - TP-R3UX-PACKAGEKIT-001 Packaged Successor Kit

Prepared C5.6 for the R3 exit-readiness path. A fresh macOS `.app` was built
from commit `c013b49b8` with
`cd apps/desktop && npm run tauri -- build --bundles app` after installing
workspace dependencies with `npm ci` in the fresh worktree. The bundle path is
`apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`;
boot probe launched the bundled executable for 8 seconds, observed WebKit child
processes, recorded zero stdout/stderr bytes, and terminated cleanly.

Appended SMOKE TP-MAC-188 as the prepared C5.7 human successor checklist
covering the A12 guided authoring journey and the R3 private-library/rule-pack
guided journey. This records package readiness only. F-4 and the A3
authoring-usability finding remain open until the human packaged pass/fail is
recorded. Post-evidence desktop Vitest passed 18 files / 390 tests. No
lifecycle, release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim was made.

## 2026-06-16 - TP-APP-R3-A3-JOURNEYPATH-001 Authoring Journey Status Rail

Executed a bounded A3 usability follow-up for the R3 blocking residual carried
from TP-MAC-141. The desktop shell now exposes a compact authoring-journey
status rail below the existing workspace section navigation. The rail is
derived from existing model/session state and reports model, apply, loads,
solve, rules, report, and project status. Each item is a real navigation
control into the corresponding workspace section.

Layout changes are limited to the desktop shell. The persistent model tree,
3D viewport, and property inspector remain the core workspace. The compact
1280x800 layout now allocates more height to the lower dock while keeping the
journey rail bounded and non-overflowing.

Validation: focused App + dead-control Vitest passed (55/55); desktop
production build passed with the existing Vite chunk-size warning; focused
compact Playwright smoke passed (1/1); in-app Browser at 1280x800 verified
zero body/viewport/item overflow, active Loads navigation, dock body height
168px, and rail height 66px; full desktop Vitest passed (386/386); full
desktop Playwright passed (10/10 across 1440x920 and 1280x800).

Evidence: `apps/desktop/SMOKE.md` TP-MAC-177 and
`_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-APP-R3-A3-JOURNEYPATH-001.md`.
F-4 remains open: the human packaged GUI journey still must be completed to
close the recorded finding. This tranche does not change lifecycle state,
release-readiness, professional approval, accessibility certification, sealing,
authentication, or code-compliance status.

## 2026-06-16 - TP-R3EXIT-PLAN-001 / TP-R3UX-DISCOVERY-001 C5 routing and UX baseline

The human approved the R3 Exit Readiness + Guided Workbench Redesign Program
as a revision to the active completion plan. WORKING_ITEMS implemented the
first two bounded tranches:

- `TP-R3EXIT-PLAN-001` / TP-MAC-183: inserted Phase C5 after landed C4 and
  before Phase D in `plans/PLAN_2026-06-10_prd_completion.md`; updated
  `docs/PLAN.md`, `_COORDINATION.md`, and `NEXT_INSTANCE_PROMPT.md` so
  ordinary R3 work selects C5 before Phase D unless the human explicitly
  overrides.
- `TP-R3UX-DISCOVERY-001` / TP-MAC-184: recorded the C5 UX baseline from
  TP-MAC-141 attempts, the human-provided 2026-06-16 screenshot/user
  evaluation, current 1440x920 and 1280x800 screenshots, viewport probe JSON,
  and the dead-control audit rerun.

Evidence assets live under `_run_records/assets/`:
`TP-R3UX-DISCOVERY-001_1440x920.png`,
`TP-R3UX-DISCOVERY-001_1280x800.png`, and
`TP-R3UX-DISCOVERY-001_viewport_probe.json`. The probe recorded zero console
errors, no horizontal page overflow, and no horizontally clipped visible
primary controls at both viewport sizes. The current blocker is work-surface
clarity, not structural clipping: the default shell still exposes too much
internal/evidence state before the user knows the next action.

Validation: `npm test --workspace apps/desktop -- App.deadControls` passed
1/1. Product behavior was not changed. F-4 and the A3 authoring-journey
usability finding remain open; next unblocked tranche is
`TP-R3UX-SHELL-001` / target TP-MAC-185, the guided workbench shell.

## 2026-06-16 - TP-R3UX-SHELL-001 guided workbench shell

Implemented the first C5 guided-workbench shell for TP-MAC-185. The desktop
app keeps the spatial core visible (model tree, 3D centerline viewport,
property inspector) and adds a task-guided lower workspace with journey rail,
current-step panel, compact queue facts, and compact R3 exit journey status.

The prior default evidence-heavy surfaces remain mounted and reachable behind
the Review evidence detail drawer in Operation Apply: editor contract,
operation diff preview, operation ledger, and agent proposal context. Existing
panel test IDs remain available after details open; new C5 shell test IDs cover
the guided workbench, journey steps, review/apply drawer, and R3 exit journey
status.

Evidence assets live under `_run_records/assets/`:
`TP-R3UX-SHELL-001_1440x920_iab.png`,
`TP-R3UX-SHELL-001_1280x800_iab.png`, and
`TP-R3UX-SHELL-001_iab_probe.json`. The probe recorded no horizontal page
overflow and no clipped guided-workbench primary controls at both viewports.

Validation: focused guided-workbench Vitest, dead-control audit, focused
Playwright guided-workbench e2e at 1440x920 and 1280x800, full desktop Vitest,
and desktop production build. No lifecycle state was changed. No schema,
evaluator grammar, solver, persistence, backend API, private-data, protected
content, release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim was introduced.

F-4 and the authoring-journey usability finding remain open. Next unblocked C5
tranche is `TP-R3UX-A12FLOW-001` / target TP-MAC-186.

## 2026-06-16 - TP-R3UX-A12FLOW-001 A12 guided authoring journey

Implemented the C5.4 A12 authoring journey redesign for TP-MAC-186. The
guided workbench now includes an A12-specific checklist for blank-model
authoring: blank document, nodes, material, section, pipe, support, load case,
primitive load, combination, solve, report, and save/reopen.

The checklist is derived from current model/session state instead of hard-coded
A12 values. It reports a clear next action, per-step completion counts, and the
current operation queue. When an operation is queued, the A12 panel names the
created object when available and exposes `Apply queued`; that affordance uses
the existing structured-operation apply handler and receipt path, preserving
undo/result-reset behavior and keeping Operation Apply available as the audit
surface.

The A12 step buttons also maintain selected-step state so controls that route
to the same underlying workspace section still produce visible feedback and do
not regress the permanent dead-control audit.

Evidence assets live under `_run_records/assets/`:
`TP-R3UX-A12FLOW-001_1440x920_iab_blank.png`,
`TP-R3UX-A12FLOW-001_1280x800_iab_queued.png`,
`TP-R3UX-A12FLOW-001_1280x800_iab_applied.png`, and
`TP-R3UX-A12FLOW-001_iab_probe.json`. The probe recorded no horizontal page
overflow and no clipped A12 primary controls at 1440x920 or 1280x800; the
compact queued state showed the inline apply button visible inside the
viewport, and the applied state recorded `route=local_wasm_engine`.

Validation: focused guided-workbench Vitest, dead-control audit, focused A12
Playwright e2e at 1440x920 and 1280x800, full desktop Vitest 18 files / 386
tests, desktop production build, full Playwright e2e 12/12, and in-app browser
visual verification.

No lifecycle state was changed. No rule-pack schema, evaluator grammar,
solver, persistence, backend API, project-store semantics, rule-pack checksum,
private-data boundary, protected-content boundary, network/telemetry posture,
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim was introduced.

F-4 and the authoring-usability finding remain open pending packaged human
successor journey evidence. Next unblocked C5 tranche is
`TP-R3UX-R3FLOW-001` / target TP-MAC-187.

## 2026-06-16 - TP-R3UX-R3FLOW-001 R3 guided rule-pack/private-library flow

Implemented the C5.5 R3 guided flow for TP-MAC-187. The guided workbench now
has an R3 mode next to the A12 mode. The R3 path routes private local library
template load/validate/save request, private non-code rule-pack draft/validate,
checksum/save request, mechanics solve, rule-check binding review, and
rule-check run request.

The guide records visible actions from the existing library manager, rule-pack
manager, and rule-check run panel. The implementation did not change rule-pack
schema, evaluator grammar, solver behavior, persistence, backend API, checksum
semantics, or local store commands.

Evidence assets live under `_run_records/assets/`:
`TP-R3UX-R3FLOW-001_1440x920_iab_r3_start.png`,
`TP-R3UX-R3FLOW-001_1280x800_iab_r3_run.png`, and
`TP-R3UX-R3FLOW-001_iab_probe.json`. The probe recorded no horizontal page
overflow, no clipped R3 primary controls, `r3Progress=6/6`, and an explicit
blocker that browser preview keeps pass/fail blocked until the desktop checker
returns complete inputs.

Validation: full desktop Vitest 18 files / 386 tests, permanent dead-control
audit, desktop production build, full Playwright e2e 14/14 across 1440x920 and
1280x800, and in-app browser visual verification.

No lifecycle state was changed. No private data was committed. No protected
content, network/telemetry posture change, release-readiness, professional
approval, certification, sealing, authentication, or code-compliance claim was
introduced.

F-4 and the authoring-usability finding remain open pending packaged human
successor journey evidence. Next unblocked C5 tranche is
`TP-R3UX-HUMANPASS-001` / target TP-MAC-189.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-18 - TP-UNITS-BTAIL-EXPORTREVA11YUNITS-001 supporting export-review evidence

- Supporting role for DEL-07-06: Export Safety Review now inventories
  `accessibility_usability_baseline_review` as unit-evidence-required because
  the Accessibility Baseline panel already exposes unit-visibility evidence.
- The export row cites
  `unit-visibility-evidence:accessibility-baseline-preview`, records no
  inferred default units, and records `conversion_performed=false`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVA11YUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-267; primary DEL-12-02 run record and
  supporting DEL-02-02 run record.
- Validation passed: focused App workspace-render test. Full App, full
  desktop Vitest, build, Playwright, and DEC-025 sweep evidence are recorded
  in closeout artifacts for this tranche.
- Boundary preserved: no accessibility finding count, target conformance
  selection, runtime accessibility evaluation, visible warning semantics,
  color-only signaling policy, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-06-20 - TP-R3UX-AGENTSHELL-001 C5.7 human bypass and agent-first viewport shell

- Human bypassed the stale C5.7 packaged re-pass route instead of passing it.
  The app should not continue toward a heavily user-laden workflow/data-entry
  primary screen; the primary screen must be a 3D model and visual interaction
  surface with an agent panel.
- Implemented the replacement local shell direction: model tree and inspector
  default collapsed; selection no longer forces inspector open; a persistent
  local `Design Agent` workbench sits beside the viewport with selection,
  target, queue counts, boundary, review-only proposal status, mechanics run,
  proposal generation, and Operations/Results openers.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-20_TP-R3UX-AGENTSHELL-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-276; visual captures and probe under
  `_run_records/assets/TP-R3UX-AGENTSHELL-001_*`.
- Validation passed: full desktop Vitest 19 files / 406 tests; desktop build;
  desktop Playwright 18/18; dist Playwright 1/1; `cargo check --release`;
  Tauri `.app` build; 8-second packaged boot probe clean.
- Boundary preserved: local deterministic review-only workbench only. No live
  external agent SDK/harness, autonomous accepted-model mutation, schema,
  solver, evaluator, persistence, protected content, private data,
  network/telemetry, lifecycle, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.
- Residual: F-4 and A3 remain open under `DEC-035` vocabulary until the human
  accepts a replacement C5.7 closure criterion or records a new gate. TP-MAC-189
  is bypassed, not passed.

## 2026-06-20 - TP-R3UX-PRIMARYCANVAS-001 contextual authoring drawer

- Followed the human screenshot feedback that the agent-first shell is a step
  in the right direction but still carries old manual data-entry clutter.
- The viewport editor-intents drawer is now visually collapsed by default; idle
  primary screen shows the 3D canvas plus agent panel without Node/Pipe forms.
  Arming Node reveals only the node geometry form. Arming Pipe reveals only the
  pipe connectivity form. Existing review/apply operation contracts are
  unchanged.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-20_TP-R3UX-PRIMARYCANVAS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-277; visual captures and probe under
  `_run_records/assets/TP-R3UX-PRIMARYCANVAS-001_*`.
- Validation passed: focused App Vitest 57/57; focused Playwright 8/8; full
  desktop Vitest 19 files / 406 tests; desktop build; desktop Playwright 18/18;
  dist Playwright 1/1; Tauri `.app` build; 8-second packaged boot probe clean.
- Boundary preserved: viewport presentation/contextual drawer/tests/evidence
  only. No live agent runtime, external SDK/harness, autonomous mutation,
  schema, solver, evaluator, persistence, protected content, private data,
  network/telemetry, lifecycle, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-20 - TP-R3VERIFY-001 C5 closure and R3 exit-chain packet

- Human accepted that the replacement C5 conditions have been fulfilled and
  authorized C5 closure. F-4/A3 are closed for the replacement C5 criterion;
  TP-MAC-189 remains bypassed, not passed.
- Assembled the derivative R3 exit-chain verification packet for human review:
  `plans/VERIFICATION_2026-06-20_r3_exit_chain.md`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-20_TP-R3VERIFY-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-190; `DEC-047` in
  `execution/_Decomposition/SOFTWARE_DECOMP.md` §12.
- Evidence basis: C1-C4 SMOKE TP-MAC-147..167 and TP-MAC-180; C5 SMOKE
  TP-MAC-183..188 and TP-MAC-272..277; latest app-code validation/package
  evidence at commit `3abf5d9bb`.
- Boundary preserved: evidence/coordination only. No R3 exit review passed, no
  R3-to-R4 stage advancement, and no app code, schema, solver, evaluator,
  persistence, package artifact, live embedded-agent runtime, external
  SDK/harness consumption, autonomous mutation, private-data write path,
  protected-content source, network/telemetry feature, lifecycle,
  release-readiness, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.
