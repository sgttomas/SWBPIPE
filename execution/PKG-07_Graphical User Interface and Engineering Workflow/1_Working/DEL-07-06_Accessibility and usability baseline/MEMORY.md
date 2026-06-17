# DEL-07-06 Memory

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
`TP-R3UX-PACKAGEKIT-001` / target TP-MAC-188.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.
