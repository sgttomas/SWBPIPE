---
run-id: WORKING_ITEMS_RUN_2026-06-20_TP-R3UX-CREATIONTOOLS-001
timestamp: 2026-06-20T17:06:53-06:00
run-status: SUCCESS
persona: WORKING_ITEMS
primary-deliverable: DEL-07-06
tranche-ids:
  - TP-R3UX-CREATIONTOOLS-001
smoke-ids:
  - TP-MAC-275
---

# WORKING_ITEMS Run Record - Object-Creation Toolbar (Phase 2, tranche 2c)

## Objective

Complete the remaining C5.7R CAD-shell builder increment: make Insert commands
arm an object-creation tool rather than only navigating or collapsing panels,
and expose the same armed-tool state in the viewport command bar. This is the
2c follow-on to `TP-R3UX-VIEWPORT-001`, `TP-R3UX-CADSHELL-001`, and
`TP-R3UX-CADSHELL-002`.

This run does not close F-4, the A3 authoring-usability finding, the packaged
human re-pass, R3 exit review, lifecycle issuance, release readiness,
professional approval, certification, sealing, authentication, or
code-compliance. The human TP-MAC-189 packaged re-pass on the freshly rebuilt
bundle remains the gate.

## Authority and scope

- `plans/PLAN_2026-06-17_prd_completion.md` C5.7R residual: "tranche 2c
  (object-creation toolbar; make Insert commands arm a creation tool rather
  than only navigate)."
- `plans/PLAN_2026-06-18_workspace_redesign_c5_7.md` §2.5 command/selection
  bar and §10 scope boundaries.
- `DEC-035`: F-4 and A3 remain R3-exit blockers until human packaged evidence
  closes them.
- `DEC-041`/`D-21`: agent panel remains a reserved seam; not built.
- `DEC-037`: rule-pack authoring remains structured-composer-only; no writable
  expression text.
- Boundary authorities: `docs/CONTRACT.md`, `docs/IP_AND_DATA_BOUNDARY.md`,
  project `AGENTS.md`.

Allowed write scope used: desktop frontend source/styles/tests, desktop e2e
tests, app smoke evidence, DEL-07-06 memory/run record, and completion-plan log
surfaces. No backend operation kind, schema, persistence, solver, evaluator, or
engine contract was changed.

## Outputs produced

- **Armed creation-tool state.** `App.tsx` now owns an `armedCreationTool`
  state shared by the in-DOM menu bar, native-menu command sink, and
  `PipeViewport`. Insert commands (`Node`, `Pipe Run`, `Support`, `Component`,
  `Load Case`) arm that tool instead of just navigating.
- **Object-creation toolbar.** The viewport command bar now presents Node, Pipe,
  Support, Component, and Load tools with `aria-pressed` active state and a
  visible active-tool instruction. Arming and queueing are separate actions.
- **Existing operation seam preserved.** Node/Pipe/Component preview intents
  still queue ordinary review-only `EditorOperationIntent`s only after the user
  presses `Queue preview`. Explicit node and pipe forms still queue through the
  existing structured operation seam. Support creation remains in the Inspector;
  load creation remains in the Load Cases panel.
- **Canvas capture gated by intent.** Empty-canvas node coordinate capture now
  occurs only when the Node tool is armed. Pipe arming starts endpoint-pick mode;
  clicking node targets fills from/to endpoints.
- **Menu behavior covered.** The tested in-DOM Insert menu reflects active
  tool state; native menu commands dispatch through the same `runMenuCommand`
  sink and therefore arm the same tools in the packaged shell.
- **Fresh packaged bundle.** Rebuilt
  `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`
  after 2c. Bundle size: `12M`. An 8-second packaged executable boot probe
  reported `alive=true`, process state `SN`, `stdout_bytes=0`, and
  `stderr_bytes=0`.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` — passed: 57/57 tests.
- `npm test --workspace apps/desktop` — passed: 19 test files / 406 tests.
- `npm run build --workspace apps/desktop` — passed; existing Vite large-chunk
  warning remains.
- `npm run test:e2e --workspace apps/desktop` — passed: 18/18 Playwright checks
  across desktop and compact Chromium projects. First run failed 2/18 because
  an old smoke step clicked the canvas without arming Node; the test was updated
  to the new arming contract and the rerun passed.
- `npm run test:e2e:dist --workspace apps/desktop` — passed: 1/1 production
  dist smoke check.
- `cargo check --release` in `apps/desktop/src-tauri` — passed.
- `npm run tauri -- build --bundles app` in `apps/desktop` — passed and
  rebuilt the macOS `.app` bundle.
- Packaged boot probe — passed: 8 seconds alive, state `SN`, no stdout/stderr
  bytes.

## Boundary review

This run changed desktop frontend presentation/interaction state, frontend
tests, and browser smoke tests. It did not introduce a backend operation kind,
solver mechanics change, schema change, evaluator grammar change, persistence
contract change, unit storage change, private-data write path, protected-content
source, network/telemetry feature, or live embedded-agent panel.

No protected standards content, private project data, repository-default
private-data write, professional approval, certification, sealing,
authentication, code-compliance, release-readiness, or R3 exit-review claim is
created.

## Residuals and next item

- The C5.7R builder increments through 2c are now implemented and packaged.
- The next unblocked plan item is the human C5.7 packaged re-pass
  (target SMOKE TP-MAC-189) on the freshly rebuilt `.app`. F-4 and A3 remain
  open until that human record passes or records the next bounded repair.
- `D-16`, `D-18`, and `D-19` remain prepared and awaiting human ruling for the
  later Phase D lead-up; they do not block this R3/C5 packaged re-pass.
