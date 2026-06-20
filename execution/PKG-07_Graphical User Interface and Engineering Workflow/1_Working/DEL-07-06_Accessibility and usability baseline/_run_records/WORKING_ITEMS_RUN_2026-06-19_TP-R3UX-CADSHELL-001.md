---
run-id: WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-CADSHELL-001
timestamp: 2026-06-19T18:45:00-06:00
run-status: SUCCESS
persona: WORKING_ITEMS
primary-deliverable: DEL-07-06
tranche-ids:
  - TP-R3UX-CADSHELL-001
smoke-ids: []
---

# WORKING_ITEMS Run Record - CAD-Shell Menu IA (Phase 2, tranche 2a)

## Objective

Begin the "professional grade" information-architecture restructure the project
authority requested after Phase 1 (the CAD-grade viewport, TP-R3UX-VIEWPORT-001):
move workspace navigation off the always-visible workflow ribbon and onto a
menu bar, with panels hidden by default and the 3D spatial core dominant. The
authority chose a native macOS menu bar (Tauri) and an "aggressive CAD shell"
target, to be landed in green tranches. This run is tranche 2a: the menu bar
(native + in-DOM) and the collapsible viewport-dominant dock. Tranches 2b
(collapsible tree/inspector rails + Inspector auto-open) and 2c (object-creation
toolbar) follow.

Performed at the project authority's direct instruction and reviewed by the
authority on the rebuilt bundle. This run does NOT close F-4, the A3 finding,
the packaged human re-pass, R3 exit review, lifecycle issuance, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance. No new packaged smoke was executed; the human TP-MAC-189
re-pass on a freshly rebuilt bundle remains the gate.

## Authority and scope

- `plans/PLAN_2026-06-18_workspace_redesign_c5_7.md` §8 criterion C (canvas reads
  as 3D / spatial core dominant) and §10 scope boundaries.
- `DEC-035`: F-4 and A3 remain R3-exit blockers until human packaged evidence
  closes them.
- `DEC-041`/`D-21`: agent panel remains a reserved seam; not built.
- Boundary authorities: `docs/CONTRACT.md`, `docs/IP_AND_DATA_BOUNDARY.md`,
  project `AGENTS.md`.

Allowed write scope used: desktop frontend source/styles/tests/e2e, the Tauri
shell entrypoint (`src-tauri/src/lib.rs`, menu only), and this DEL-07-06 run
record. Files changed: `src/App.tsx`, `src/styles.css`, `src/services/nativeMenu.ts`
(new), `src-tauri/src/lib.rs`, `src/App.test.tsx`, `e2e/r2-smoke.spec.ts`.

## Outputs produced

- **Menu-bar information architecture (replaces the workflow ribbon).** Added a
  single command sink (`runMenuCommand`) driving File / Edit / View / Insert /
  Analyze. View summons/dismisses the ten workspace sections and the Issues /
  Audit drawers; File covers project new/open/list/save; Edit covers model
  undo/redo; Analyze runs/cancels the mechanics preview; Insert surfaces the
  authoring panes. The always-visible ribbon (`workflow-ribbon`, ribbon stops,
  current-step summary, section switcher) is retired.
- **Native macOS menu bar (Tauri).** `build_app_menu` in `lib.rs` constructs the
  OS-level menu with the same command ids; `on_menu_event` forwards each click to
  the webview via a `native-menu-command` event. A guarded bridge
  (`src/services/nativeMenu.ts`, dynamic-imported and inert outside Tauri) listens
  and dispatches through the same `runMenuCommand` sink.
- **In-DOM menu bar is the tested source of truth, suppressed in the shell.** The
  in-DOM menu bar renders in the browser/Playwright preview (where there is no OS
  menu and tests must navigate); it is hidden in the packaged Tauri shell so the
  authority sees a single native menu, not a redundant second row. Gated on
  `isTauriRuntime()`, with a one-fewer-row grid variant (`.app-shell.native-menu`).
- **Collapsible, viewport-dominant dock.** `activeSection` is now nullable and
  starts null, so the dock is collapsed by default and the spatial core owns the
  surface. A View command opens a section as a bottom panel with a "Close panel"
  button; re-selecting the active section, or "Close Panel (show viewport)",
  collapses it. All sections stay mounted (form drafts/state survive) and hidden
  via CSS, preserving the dead-control audit and panel-discovery tests.
- **Dead-code removal.** With the ribbon retired, its now-unused implementation
  (the `Ribbon` component, `RIBBON_STOPS`/`JOURNEY_STEPS`, the `RibbonStop`/
  `JourneyStep` types, and the `ribbonStopForSection`/`ribbonStopBadge`/
  `journeyStepIcon`/`journeyStepStatus`/`currentJourneyStep` helpers, plus an
  orphaned icon import) was deleted — ~357 lines. The journey-event machinery
  wired to the library/rule-pack panels (`recordR3JourneyEvent` et al.) was kept.
- Updated unit and e2e tests in lockstep: the central e2e `openWorkspaceSection`
  helper and the unit shell tests now drive the View menu instead of the ribbon
  (made idempotent so re-opening an active section does not toggle it shut); the
  compact-layout geometry probe was retargeted from `.workflow-ribbon`/`.ribbon-stop`
  to `.app-menu-bar`/`.app-menu-trigger` and hardened so a missing element fails
  loudly rather than passing on zeroes. No assertions were weakened to pass.

A browser-only stacking bug was caught by reasoning before the first e2e run
(jsdom does no layout, so unit tests could not surface it): the menu click-catcher
backdrop would have painted above the dropdown and swallowed item clicks. Fixed by
giving the menu bar its own stacking context above the backdrop; confirmed by the
real-browser e2e pass.

## Validation

- `npx tsc -b --noEmit` (apps/desktop) — clean.
- `npm test --workspace apps/desktop` — passed: 19 test files / 406 tests
  (unchanged count; the viewport/menu WebGL-and-native paths do not execute under
  jsdom — the in-DOM menu does, and is the unit-tested navigation).
- `cargo check --release` (src-tauri) — clean; the Tauri v2 menu API compiles.
- `npm run test:e2e --workspace apps/desktop` — passed: 18 Playwright checks
  across chromium desktop and compact projects (real browser; re-confirmed after
  the dead-code removal). This verifies the in-DOM menu navigation, the
  collapsible dock, and that the backdrop stacking fix works.
- `npm run tauri -- build --bundles app` (apps/desktop) — bundled
  `OpenPipeStress Technical Preview.app`; 6s boot probe ALIVE, exit-clean, no
  stdout/stderr (also confirms the native menu construction does not panic at
  startup). Authority-reviewed.

## Boundary review

This run changed frontend navigation/presentation/styling, added a display-only
native-menu bridge, and added an OS menu to the Tauri shell. It did not change
solver mechanics, schemas, evaluator grammar, persistence contracts, backend
data APIs (the only Rust change is the menu and its event emission), unit storage
semantics, private-data policy, protected-content policy, network/telemetry
posture, rule-pack authoring authority, the agent-panel seam (D-21), lifecycle
state, release-readiness status, or professional-boundary semantics. No new
operation kinds or backend/engine data contracts were introduced.

No protected standards content, private project data, repository-default
private-data write, network/telemetry feature, professional approval,
certification, sealing, authentication, code-compliance, release-readiness, or
R3 exit-review claim is created.

## Residuals and next item

- Tranche 2b (collapsible tree/inspector rails + Inspector auto-open) and 2c
  (object-creation toolbar; making Insert commands arm a creation tool rather than
  only navigate) are the committed next steps of the CAD-shell restructure.
- The packaged human re-pass (TP-MAC-189) on a freshly rebuilt bundle remains the
  gate; F-4 and A3 cannot close until it is performed and recorded. The bundle
  reflects 2a; it should be rebuilt after 2b/2c before the re-pass.
- Git commit/push performed in this run at project-authority approval (shared
  monorepo `main`, explicit-path staging, behind-guard) — 2a source + tests +
  this record only.
