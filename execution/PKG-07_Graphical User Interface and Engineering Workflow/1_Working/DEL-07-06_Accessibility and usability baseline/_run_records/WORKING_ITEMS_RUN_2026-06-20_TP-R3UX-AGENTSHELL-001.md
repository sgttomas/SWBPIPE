---
run-id: WORKING_ITEMS_RUN_2026-06-20_TP-R3UX-AGENTSHELL-001
timestamp: 2026-06-20T19:15:00-06:00
run-status: SUCCESS
persona: WORKING_ITEMS
primary-deliverable: DEL-07-06
tranche-ids:
  - TP-R3UX-AGENTSHELL-001
smoke-ids:
  - TP-MAC-276
---

# WORKING_ITEMS Run Record - Agent-First Viewport Shell

## Objective

Record and implement the 2026-06-20 human bypass/redirect of the pending C5.7
packaged re-pass. The human explicitly declined the old pass route: the app
must not become a heavily user-laden workflow screen, and the primary screen
must become a 3D model interaction surface with visual aids. Workflow/data-entry
surfaces, menus, and status detail should not own the primary screen. The
missing primary affordance is an agent panel because the agent is expected to
enter data, manage workflows, and redesign the model from user input, while the
viewport/tools let the user focus, investigate, and make targeted changes.

This run does not mark TP-MAC-189 passed, does not close F-4/A3, and does not
start R3 exit review. It records a new bounded repair direction and rebuilds
the app with a local, review-only agent workbench panel.

## Authority and scope

- Human directive on 2026-06-20: bypass the old C5.7 test path and proceed
  toward a 3D-model-first, agent-mediated primary screen.
- `DEC-035`: F-4 and A3 remain R3-exit blockers until human evidence closes
  them or records a replacement closure criterion.
- `DEC-042`: harness-independent embedded-agent preparation is allowed; live
  external app-dev harness consumption remains gated.
- `D-21`/`D-22`: no v0.2/R7 scope promotion and no live app-dev harness
  integration in this tranche.
- Boundary authorities: `docs/CONTRACT.md`, `docs/IP_AND_DATA_BOUNDARY.md`,
  project `AGENTS.md`.

Allowed write scope used: desktop frontend source/styles/tests, desktop e2e
tests, app smoke evidence, DEL-07-06 memory/run records/assets, and completion
plan/log surfaces. No backend operation kind, schema, persistence, solver,
evaluator, engine contract, private-data path, protected-content source,
network feature, telemetry feature, or live agent runtime was changed.

## Outputs produced

- **Viewport-first default shell.** The model tree and inspector now default to
  collapsed rails. Selecting a model entity no longer forces the inspector open,
  keeping the primary surface focused on the 3D model and visual investigation.
- **Local agent workbench panel.** Added a persistent right-side
  `Design Agent` panel to the primary workspace. It exposes selection, target,
  queued/applied operation counts, professional boundary, and review-only
  proposal status. It can run mechanics, generate the existing deterministic
  review proposal after results exist, and open Operations/Results surfaces.
- **No live agent runtime.** The panel is a local deterministic workbench over
  existing app state. It does not call an external SDK or app-dev harness, does
  not autonomously mutate accepted model state, and does not promote `D-21`.
- **Responsive spatial layout.** With the workflow dock collapsed, the primary
  model/agent area keeps the available vertical space even on short screens.
  When both secondary rails are expanded, the agent column tucks away to avoid
  horizontal overflow in dense manual-edit mode.
- **Tests updated to the new product shape.** Vitest and Playwright now assert
  collapsed rails by default, agent workbench visibility, local review-only
  proposal state, and explicit rail expansion before legacy workflow/data-entry
  checks.
- **Fresh packaged bundle.** Rebuilt
  `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`.
  An 8-second packaged executable boot probe reported the process still running
  and `stdout_bytes=0`, `stderr_bytes=0`.

## Visual evidence

Saved visual captures under `_run_records/assets/`:

- `TP-R3UX-AGENTSHELL-001_1440x920.png`
- `TP-R3UX-AGENTSHELL-001_1280x800.png`
- `TP-R3UX-AGENTSHELL-001_1440x920_canvas.png`
- `TP-R3UX-AGENTSHELL-001_1280x800_canvas.png`
- `TP-R3UX-AGENTSHELL-001_probe.json`

The probe recorded no horizontal overflow at 1440x920 or 1280x800, collapsed
tree/inspector rails by default, visible agent panel, canvas containment inside
the workspace, and nonblank canvas pixel samples:

- 1440x920: canvas 984x520, 150 sampled unique colors, 238/5330 sampled
  non-pale pixels.
- 1280x800: canvas 844x520, 160 sampled unique colors, 269/5525 sampled
  non-pale pixels.

## Validation

- `npm test --workspace apps/desktop` — passed: 19 test files / 406 tests.
- `npm run build --workspace apps/desktop` — passed; existing Vite large-chunk
  warning remains.
- `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop` — passed:
  18/18 Playwright checks across desktop and compact Chromium projects.
- `npm run test:e2e:dist --workspace apps/desktop` — passed: 1/1 production
  dist smoke check.
- `cargo check --release` in `apps/desktop/src-tauri` — passed.
- `npm run tauri -- build --bundles app` in `apps/desktop` — passed and
  rebuilt the macOS `.app` bundle.
- Packaged boot probe — passed: launched bundled executable for 8 seconds,
  observed it still running, `stdout_bytes=0`, `stderr_bytes=0`, then
  terminated it.

## Boundary review

This run changed desktop frontend layout, local deterministic proposal access,
frontend tests, browser smoke tests, and evidence records. It did not introduce
a live embedded-agent runtime, autonomous model mutation, backend operation
kind, solver mechanics change, schema change, evaluator grammar change,
persistence contract change, unit storage change, protected-content source,
private-data write path, network/telemetry feature, lifecycle transition, R3
exit review, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim.

## Residuals and next item

- The old C5.7 packaged re-pass criterion is bypassed by human direction, not
  passed.
- F-4 and A3 remain open against the prior `DEC-035` blocker vocabulary until
  the human accepts a replacement C5.7 closure criterion or records a new gate.
- The next useful judgment surface is a human review of the rebuilt
  3D-model-first, agent-mediated primary screen against the redirected criteria:
  model/visual interaction dominance; secondary workflow/data-entry surfaces
  kept off the primary screen; agent panel present and review-only; no live
  agent-runtime or professional/release claim.
