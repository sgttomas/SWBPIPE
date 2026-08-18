# WORKING_ITEMS Return — PKG-09 / DEL-09-04

## Outcome

`N1` is **accepted for CHANGE fan-in**. A durable representative Playwright case was
added and the same visible workflow passed as a Safari manual/computer-use
smoke. Local Chromium failed before assertions. Human host runs reached the
case body in both projects; the second, escalated run proved the remaining
failure is a product grid-layout defect rather than sequencing alone. The
smallest CSS repair is implemented, passed fresh full-diff code review, and
passed the focused host run in both registered viewport projects.
The first DEL-09-04 `## Remaining` bullet is unchanged.

## Product candidate

- `apps/desktop/e2e/gui-workflow-validation.spec.ts`
- SHA-256:
  `2ae72d668342674cebd156b9ba41810bbe783f0ce2e234464be13a5e51c87bb2`
- Coverage: initial invented fixture solve; unit-bearing L-100 edit; local WASM
  apply receipt; save/list/open-by-ID; persistence after reopen; honest edited
  browser solve blocker; viewport blocked state; local network/telemetry fence;
  no external requests.
- `apps/desktop/src/styles.css`
- SHA-256:
  `476afc6a496955ad6719c6024e06c97353f8d2f5f006510afad70adaf572e1dc`
- Repair: the exact both-expanded selector uses a three-track
  tree/viewport/inspector grid while the hidden agent pane has no retained
  zero-width track. Collapsed-rail layouts retain the existing four-track
  template. The e2e case asserts inspector width exceeds 300 px.

## Checks

- Playwright discovery: PASS, 2 tests listed across `chromium-desktop` and
  `chromium-compact`.
- Focused Playwright manager run: FAIL before assertions, 0/2, Chromium
  MachPortRendezvous permission denial.
- Fresh read-only execution retry: same FAIL before assertions, 0/2.
- Human host focused run before sequencing repair: both projects launched and
  passed through the initial solve assertions, then each timed out after 120 s
  at the `queue-editor-intent` click because `modeling-workspace` /
  `viewport-scale-bar` intercepted pointer events while Solve remained open.
- Classification: case sequencing defect. Repair closes `workspace-dock-close`
  and asserts `workspace-dock` is collapsed and `workspace-section-solve` is
  hidden before authoring. No force, coordinates, event dispatch, CSS, timeout,
  product-source, click bypass, or assertion weakening was used.
- Post-repair focused Playwright under capability `host`: PASS, exit 0, 2 passed
  in 22.9 seconds (`chromium-desktop` 11.3 seconds; `chromium-compact` 10.6
  seconds), bound to HEAD `32360f2eb53936a526a98a41b0a571c7af287483`
  and the product/test hashes above.
- Agent-0 escalated amended host command, run from `apps/desktop` after making
  pinned `wasm-bindgen-cli 0.2.123` available:
  `npm run build:wasm && PLAYWRIGHT_WORKERS=1 ../../node_modules/.bin/playwright test e2e/gui-workflow-validation.spec.ts --project=chromium-desktop --project=chromium-compact`
- Escalated amended host result: FAIL in both `chromium-desktop` and
  `chromium-compact` after 120 seconds at `queue-editor-intent`; the button was
  visible, enabled, and stable, but `modeling-workspace` and viewport
  command/scale descendants intercepted pointer events with Solve closed.
- Corrected classification: product layout defect. With both rails expanded,
  `.workspace-pane-agent` was `display:none` while a zero-width third grid track
  remained, so auto-placement put the inspector into that track and left the
  intended inspector track empty.
- Product repair: exact both-expanded CSS selector now declares three tracks;
  the e2e case retains panel close and asserts inspector width before editing.
- `CODE-REVIEW-01`: FAIL on one blocking test sequence. Issues remained open
  before Audit; co-located fixed drawers could intercept or occlude the final
  boundary evidence.
- Review remediation: close Issues through its visible Close button, assert it
  is removed, then open and assert Audit visible before local-status checks.
- `CODE-REVIEW-02`: PASS; frozen hashes and scope matched, 100% of both path
  deltas reviewed, zero actionable findings, prior drawer finding closed, and
  fan-in valid for the code-review gate.
- Focused runtime residual: closed by the bound host PASS above.
- Post-repair Playwright syntax/discovery: PASS, both viewport projects listed.
- Post-repair focused TypeScript check: PASS under strict ES2022/Bundler
  settings.
- Full desktop Vitest: PASS, 29 files / 523 tests.
- Desktop production build: PASS, `tsc -b && vite build`.
- Practitioner harness self-check: exit 0; current baseline findings
  INFO=14, NOT_APPLICABLE=1, REVIEW=4, WARN=31.
- Full practitioner-harness pytest: PASS, 349 tests.
- DEC-025 dirty-tree sweep:
  - cargo crate sweep: PASS (36 manifests)
  - project Python pytest: PASS, 576 tests
  - desktop Vitest plus WASM build: PASS, 523 tests
  - desktop Playwright: FAIL, 22/22 launch failures at 0 ms with the same
    Mach rendezvous denial
  - desktop production build: `not_run` by fail-fast ordering; passed separately
  - overall: FAIL
  - temporary summary:
    `/var/folders/96/1_3ww0012pv5ptwwsd1990yw0000gn/T/openpipestress-dirty-sweeps/SWEEP_20260817T144418Z_1a995d571a65-dirty.json`
  - summary SHA-256:
    `a4701c93664033afd3673975b472ce6b420cfba02b9354c11e95ac08861c1c1d`
- `git diff --check`: PASS.
- Exact write-path containment: PASS.
- Staged paths: none.
- Product-source review trigger: active; `apps/desktop/src/styles.css` changed,
  so `CODE-REVIEW-01` must PASS before fan-in.

Host rerun from `projects/chirality-piping/apps/desktop`:

`npm run build:wasm && PLAYWRIGHT_WORKERS=1 ../../node_modules/.bin/playwright test e2e/gui-workflow-validation.spec.ts --project=chromium-desktop --project=chromium-compact`

## Closure

Changed paths are contained to the CSS repair, new e2e spec, this run root, and
the one DEL-09-04 run record. There is no canonical evidence, status, memory, public
comparison, maintainer-review promotion, foreign-domain, or shared fan-in
write. The DEL residual is unchanged. Exact paths and the clean-head DEC-025
requirement for CHANGE are in `HANDOFF_STATE.md`.

Standard claim fence applies (F-PIP-2; DEC-081 claims taxonomy).
