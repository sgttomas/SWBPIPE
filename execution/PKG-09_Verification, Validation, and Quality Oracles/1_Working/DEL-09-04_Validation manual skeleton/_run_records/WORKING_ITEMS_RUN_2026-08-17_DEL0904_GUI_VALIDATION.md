# WORKING_ITEMS Run — DEL-09-04 GUI Workflow Validation

Date: 2026-08-17

RunID: `HELP-HUMAN-PIPING-20260817-DEL0904-GUI-VALIDATION`

Branch base: `1a995d571a6509d82321e4c982c7b788f16aff36`

## Result

The tranche added a representative Playwright candidate for the invented
saved-project/load-edit workflow and executed the same visible workflow in
Safari using Computer Use. The Safari smoke completed through edit, local WASM
apply, save/list/reopen, persisted `-225 N/m`, and the honest edited-model
`MODEL_INCOMPLETE` browser-solve boundary.

The initial local Playwright attempts did not execute the case: Chromium failed
during launch in both registered viewports with macOS Mach rendezvous permission
denial, and a fresh read-only retry reproduced the failure. The earlier full
DEC-025 sweep likewise failed at the host-only Playwright surface after cargo,
project pytest, and desktop Vitest passed. A later human host run did execute
the case body and supplied the repair input below. Until the repaired case is
rerun on that host, this run does not close or narrow the GUI-workflow portion
of `_STATUS.md ## Remaining`, and no canonical validation evidence is written.

## Host attempt and in-place case repair

A subsequent human host run launched both Chromium viewport projects and
passed every assertion through the initial solved-state checks. Both then
timed out after 120 seconds at `gui-workflow-validation.spec.ts:49`: the
`queue-editor-intent` control resolved and was enabled, but pointer events were
intercepted by `modeling-workspace` / `viewport-scale-bar` while the Solve dock
remained open.

This is classified as a case-sequencing defect. The test opened Solve for the
initial run but did not restore the authoring layout. The amended case now uses
the visible `workspace-dock-close` control and asserts the dock is collapsed
and the Solve section hidden before selecting and editing `load:L-100`. No
product behavior, timeout, CSS, assertion, or click semantics changed. A host
rerun of both projects remains required; no Playwright pass is claimed.

Post-repair local checks passed: the focused spec passed strict TypeScript
checking, Playwright discovery listed both viewport projects, desktop
TypeScript/production build passed, desktop Vitest passed 523 tests across 29
files, practitioner-harness pytest passed 349 tests, and self-check exited zero
at the unchanged baseline finding counts.

## Escalated host run and corrected classification

At instruction commit `32360f2eb53936a526a98a41b0a571c7af287483`, Agent 0
made pinned `wasm-bindgen-cli 0.2.123` available and ran from
`apps/desktop`:

`npm run build:wasm && PLAYWRIGHT_WORKERS=1 ../../node_modules/.bin/playwright test e2e/gui-workflow-validation.spec.ts --project=chromium-desktop --project=chromium-compact`

Both projects launched and reached `queue-editor-intent` with the Solve dock
closed. Each timed out after 120 seconds. Playwright reported the control as
visible, enabled, and stable while `modeling-workspace` and viewport
command/scale descendants intercepted pointer events. Host trace inspection
showed `workspace-pane-inspector` rendered at width zero despite
`toggle-inspector` reporting expanded.

The corrected classification is a product layout defect. In the both-expanded
state, `.workspace-pane-agent` was `display:none`, but the grid retained its
zero-width third track. CSS grid auto-placement put the inspector, the fourth
DOM child, in that zero-width third track and left the intended fourth track
empty.

The bounded repair changes only the exact both-expanded selector to a
three-column tree/viewport/inspector template. Layouts with either collapsed
rail continue using the existing four-track template. The e2e case preserves
the visible Solve-dock close sequence and all prior assertions, and adds a
direct regression that the inspector width exceeds 300 px before authoring.
Focused TypeScript, Playwright discovery, production build, and 523-test desktop
Vitest checks pass. Fresh read-only `TASK + software-code-review` and another
host Playwright run remain required; no Playwright pass is claimed.

The first mandatory read-only code review accepted the CSS geometry repair but
returned FAIL on one blocking case sequence: Issues remained open before Audit,
and the two co-located fixed drawers could intercept or occlude the final GUI
boundary evidence. The case now closes Issues through its visible Close button,
asserts it is removed, then opens and asserts Audit visible before checking
network/telemetry text. Focused strict TypeScript and two-project discovery pass
after this repair. The second fresh full-diff review returned PASS: frozen
hashes and scope matched, the complete CSS/e2e delta was reviewed, the first
review finding is closed, and no actionable finding remains. The focused host
result is recorded below; the later integrated sweep remains required.

## Focused host acceptance

Agent 0 ran the exact command from `apps/desktop` under execution capability
`host`, at frozen HEAD `32360f2eb53936a526a98a41b0a571c7af287483`:

`npm run build:wasm && PLAYWRIGHT_WORKERS=1 ../../node_modules/.bin/playwright test e2e/gui-workflow-validation.spec.ts --project=chromium-desktop --project=chromium-compact`

The command exited 0 with 2 passed in 22.9 seconds: `chromium-desktop` passed in
11.3 seconds and `chromium-compact` passed in 10.6 seconds. The result binds to
`styles.css` SHA-256
`476afc6a496955ad6719c6024e06c97353f8d2f5f006510afad70adaf572e1dc`
and e2e SHA-256
`2ae72d668342674cebd156b9ba41810bbe783f0ce2e234464be13a5e51c87bb2`.

N1 is accepted for CHANGE fan-in. The DEL-09-04 residual remains unchanged and
no canonical evidence is written by this node. CHANGE must commit the exact N1
path set and then run the full DEC-025 sweep at clean committed HEAD before the
commit-bound merge gate can be claimed.

## After-the-fact host DEC-025 closeout

Agent 0 ran the complete ordered sweep under execution capability `host` at
the committed N1 HEAD. The three escalated attempts were:

1. `python3 tools/release/run_evidence_sweep.py --execute` — exit 1 before
   prerequisite probing or surface 1 because Python 3.9.6 is below the required
   Python 3.11 floor.
2. `/Users/ryan/.local/share/mise/installs/python/3.13/bin/python3.13 tools/release/run_evidence_sweep.py --execute`
   — exit 1 after the cargo surface passed because missing `jsonschema` caused
   Python test-collection errors. The failed canonical summary from this
   attempt was discarded before retry.
3. `/private/tmp/chirality-piping-dec025-venv/bin/python tools/release/run_evidence_sweep.py --execute`
   — exit 0; all five ordered surfaces passed: cargo crate sweep, Python 576,
   desktop Vitest 523, development Playwright 22/22, distribution Playwright
   2/2, and desktop production build.

The passing clean-head summary is
`validation/evidence/sweeps/SWEEP_20260818T011106Z_47eb94e9d6b8.json`, SHA-256
`4506dfb573100c53619dd577bdf4ecbba7e6cd78de92e5da6fc7bc9c0cefb837`,
bound to Git commit `47eb94e9d6b8d8ab68e31f49cd0528ffeaf1e4b3` with
`working_tree_dirty=false`. This closes the N1 commit-bound DEC-025 rerun. The
DEL-09-04 residual and lifecycle state remain unchanged.

## Evidence pointers

- Candidate:
  `projects/chirality-piping/apps/desktop/e2e/gui-workflow-validation.spec.ts`
- Manager return and exact checks:
  `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260817-DEL0904-GUI-VALIDATION/RETURN.md`
- Manual smoke:
  `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260817-DEL0904-GUI-VALIDATION/MANUAL_SAFARI_SMOKE.md`
- Retry return:
  `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260817-DEL0904-GUI-VALIDATION/instances/TEST-EXECUTION-01/RETURN.md`
- Handoff:
  `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260817-DEL0904-GUI-VALIDATION/HANDOFF_STATE.md`

## Rerun

No N1 verification rerun is outstanding for commit
`47eb94e9d6b8d8ab68e31f49cd0528ffeaf1e4b3`. If that commit binding changes,
rerun the focused host case and complete DEC-025 sweep. This verification does
not narrow the unchanged DEL-09-04 residual.

Standard claim fence applies (F-PIP-2; DEC-081 claims taxonomy).
