---
run-id: WORKING_ITEMS_RUN_2026-06-20_TP-R3UX-PRIMARYCANVAS-001
timestamp: 2026-06-20T19:40:00-06:00
run-status: SUCCESS
persona: WORKING_ITEMS
primary-deliverable: DEL-07-06
tranche-ids:
  - TP-R3UX-PRIMARYCANVAS-001
smoke-ids:
  - TP-MAC-277
---

# WORKING_ITEMS Run Record - Primary Canvas Declutter

## Objective

Apply the next human-feedback pass after the agent-first viewport shell. The
attached 2026-06-20 screenshot was judged "a step in the right direction" but
still showed old manual-authoring forms occupying the primary screen while no
creation tool was active. This tranche removes that idle data-entry surface
from the primary viewport and makes Node/Pipe forms contextual to the active
creation tool.

This run does not mark TP-MAC-189 passed, close F-4/A3, start C5.8, or create
any lifecycle, release, professional, certification, sealing, authentication,
or code-compliance claim.

## Authority and scope

- Human feedback on 2026-06-20: the UI is moving in the right direction, but
  the app should continue toward a 3D-model/user-interaction primary surface
  mediated by the agent, not a cluttered workflow/data-entry surface.
- Prior run `TP-R3UX-AGENTSHELL-001`: local review-only agent panel and
  viewport-first shell.
- Boundary authorities: `docs/CONTRACT.md`, `docs/IP_AND_DATA_BOUNDARY.md`,
  project `AGENTS.md`.

Allowed write scope used: viewport frontend source/styles/tests, desktop e2e
tests, app smoke evidence, DEL-07-06 memory/run records/assets, and completion
plan/log surfaces. No backend operation kind, schema, persistence, solver,
evaluator, engine contract, private-data path, protected-content source,
network feature, telemetry feature, or live agent runtime was changed.

## Outputs produced

- **Idle viewport form removal.** `Viewport editor intents` is visually
  collapsed when no Node/Pipe tool is active and no viewport intents are queued.
  The primary screen now starts with the 3D canvas and agent panel, not the
  Node/Pipe input forms.
- **Contextual creation drawer.** Arming Node reveals only the explicit node
  geometry form. Arming Pipe reveals only the straight-pipe connectivity form.
  Component preview intents still queue through the command bar; Support and
  Load remain routed to their existing structured surfaces.
- **Test contract updated.** Browser smoke tests now assert the viewport intent
  drawer is collapsed by default, then explicitly arm Node/Pipe before using
  the corresponding controls. This preserves the existing operation seam and
  review/apply behavior while matching the new product posture.
- **Fresh packaged bundle.** Rebuilt
  `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`.
  An 8-second packaged executable boot probe reported the process still running
  and `stdout_bytes=0`, `stderr_bytes=0`.

## Visual evidence

Saved visual captures under `_run_records/assets/`:

- `TP-R3UX-PRIMARYCANVAS-001_2048x1200_idle.png`
- `TP-R3UX-PRIMARYCANVAS-001_2048x1200_node_armed.png`
- `TP-R3UX-PRIMARYCANVAS-001_probe.json`

The probe recorded idle state as `viewport-intents collapsed`, node form hidden,
pipe form hidden, no horizontal overflow, and a 1592x854 canvas at 2048x1200.
After arming Node, the drawer became active, the node form was visible, and the
pipe form stayed hidden.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` — passed: 57/57 tests.
- `cd apps/desktop && npx playwright test e2e/r2-smoke.spec.ts -g "guided workbench shell|R2 desktop preview smoke|viewport gesture|R2 from-blank" --workers=1`
  — first run exposed one stale smoke assumption, then passed 8/8 after the
  endpoint-pick helper was made idempotent.
- `npm test --workspace apps/desktop` — passed: 19 test files / 406 tests.
- `npm run build --workspace apps/desktop` — passed; existing Vite large-chunk
  warning remains.
- `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop` — passed:
  18/18 Playwright checks across desktop and compact Chromium projects.
- `npm run test:e2e:dist --workspace apps/desktop` — passed: 1/1 production
  dist smoke check.
- `npm run tauri -- build --bundles app` in `apps/desktop` — passed and rebuilt
  the macOS `.app` bundle.
- Packaged boot probe — passed: launched bundled executable for 8 seconds,
  observed it still running, `stdout_bytes=0`, `stderr_bytes=0`, then
  terminated it.

## Boundary review

This run changed viewport presentation, CSS visibility, browser smoke steps,
and evidence records. It did not introduce a live embedded-agent runtime,
autonomous model mutation, backend operation kind, solver mechanics change,
schema change, evaluator grammar change, persistence contract change, unit
storage change, protected-content source, private-data write path,
network/telemetry feature, lifecycle transition, R3 exit review,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Residuals and next item

- The primary screen is closer to the human-stated direction: 3D canvas and
  agent remain primary; manual Node/Pipe authoring is contextual rather than
  idle clutter.
- Remaining product-direction work, if selected, should keep reducing primary
  chrome and moving workflow/status/data-entry detail behind the agent,
  contextual tools, or explicit secondary surfaces.
- F-4 and A3 remain open under `DEC-035` vocabulary until the human accepts a
  replacement C5.7 closure criterion or records a new gate.
