---
run-id: WORKING_ITEMS_RUN_2026-06-20_TP-R3UX-CADSHELL-002
timestamp: 2026-06-20T16:21:57-06:00
run-status: SUCCESS
persona: WORKING_ITEMS
primary-deliverable: DEL-07-06
tranche-ids:
  - TP-R3UX-CADSHELL-002
smoke-ids: []
---

# WORKING_ITEMS Run Record - CAD-Shell Collapsible Rails (Phase 2, tranche 2b)

## Objective

Continue the "professional grade" information-architecture restructure the
project authority requested after Phase 1 (the CAD-grade viewport,
TP-R3UX-VIEWPORT-001) and tranche 2a (the menu-bar IA, TP-R3UX-CADSHELL-001).
This run is tranche 2b: make the spatial core's side panes collapsible so the
3D viewport can dominate, and make entity selection reveal the inspector. The
remaining tranche 2c (object-creation toolbar) follows.

Performed at the project authority's direct instruction and committed at the
authority's approval. This run does NOT close F-4, the A3 finding, the packaged
human re-pass, R3 exit review, lifecycle issuance, release readiness,
professional approval, certification, sealing, authentication, or
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

Allowed write scope used: desktop frontend source/styles, the Tauri shell
entrypoint (`src-tauri/src/lib.rs`, menu only), and this DEL-07-06 run record.
Files changed: `src/App.tsx`, `src/styles.css`, `src-tauri/src/lib.rs`. (The
pre-existing flaky-test timing fix in `src/App.test.tsx` was landed as its own
separate commit, `ed2b7cb18`, and is not part of this tranche.)

## Outputs produced

- **Collapsible spatial-core rails.** The model-tree (left) and inspector
  (right) panes each collapse to a thin vertical strip, handing their grid
  column width to the 3D viewport so the spatial core can dominate. Both rails
  default expanded, so existing layout, unit, and e2e behavior are unchanged.
  Column tracks were refactored to CSS custom properties so the collapse rules
  (a two-class selector) outweigh the responsive breakpoint at every width.
- **One persistent toggle per rail.** Each rail carries a single toggle button
  (`toggle-tree` / `toggle-inspector`) that stays mounted across collapse/expand
  — so keyboard focus is never dropped — and carries `aria-expanded` for
  disclosure semantics, matching the convention the shell already uses for its
  drawer toggles. CSS restyles it between a horizontal header bar (expanded) and
  the full-height vertical strip (collapsed). The still-mounted panel is hidden
  via CSS only (never `aria-hidden`), preserving the panels in the DOM and
  accessibility tree for the unit tests.
- **Inspector auto-open on selection.** A single `handleSelectEntity` handler
  sets the selection and re-opens the inspector rail if collapsed, wired into
  all three selection sources — the model tree, the 3D viewport, and the
  load-case manager — so the spatial core behaves like a CAD "click a part to
  see its details" surface.
- **View-menu rail toggles (in-DOM + native).** New `view.tree` / `view.inspector`
  commands toggle the rails from both the tested in-DOM menu bar and the native
  macOS View submenu (`build_app_menu`), dispatched through the same
  `runMenuCommand` sink; the in-DOM items reflect current rail state.
- **Adversarial pre-commit review.** An 18-agent review of the diff was run
  before commit; it confirmed scope/governance compliance and caught three
  in-scope defects, all fixed in this tranche: (1) the load-case manager
  selection was not routed through `handleSelectEntity` (third selection source
  missed); (2) the first-cut two-button collapse swap dropped keyboard focus to
  `<body>` on toggle (resolved by the single persistent toggle); (3) the toggle
  buttons advertised no `aria-expanded` disclosure state (added). Two confirmed
  but pre-existing native-menu ARIA items (native View items lack a checked
  indicator; `aria-pressed` on `role="menuitem"`) were left as out-of-scope.

## Validation

- `npx tsc -b --noEmit` (apps/desktop) — clean.
- `npm test --workspace apps/desktop` — passed: 19 test files / 406 tests,
  including the permanent dead-control audit (the new persistent rail toggles
  produce an observable DOM change on click in the default expanded state).
  Confirmed stable across ten consecutive full-suite runs (406/406 each).
- `cargo check --release` (src-tauri) — clean; the three-line native View menu
  addition compiles.
- `npm run test:e2e --workspace apps/desktop` — passed: 18 Playwright checks
  across chromium desktop and compact projects (real browser), re-confirmed
  after the toggle rework.
- `npm run tauri -- build --bundles app` (apps/desktop) — bundled
  `OpenPipeStress Technical Preview.app`; an earlier build of this tranche was
  6s boot-probed ALIVE/exit-clean/no stdout-stderr (confirming the new native
  menu construction does not panic at startup), and the bundle was rebuilt after
  the review fixes for the authority's review.

## Boundary review

This run changed frontend layout/presentation/styling, added a display-only set
of rail toggles, and added two items to the native View menu. It did not change
solver mechanics, schemas, evaluator grammar, persistence contracts, backend
data APIs (the only Rust change is the menu), unit storage semantics,
private-data policy, protected-content policy, network/telemetry posture,
rule-pack authoring authority, the agent-panel seam (D-21), lifecycle state,
release-readiness status, or professional-boundary semantics. No new operation
kinds or backend/engine data contracts were introduced.

No protected standards content, private project data, repository-default
private-data write, network/telemetry feature, professional approval,
certification, sealing, authentication, code-compliance, release-readiness, or
R3 exit-review claim is created.

## Residuals and next item

- Tranche 2c (object-creation toolbar; making Insert commands arm a creation
  tool rather than only navigate) is the committed next step of the CAD-shell
  restructure.
- Out-of-scope review findings deferred: native View menu items show no
  checked-state indicator that the in-DOM menu has, and the shared menu renderer
  uses `aria-pressed` on `role="menuitem"` (a pre-existing pattern). Both are
  pre-existing and not introduced by this tranche.
- The packaged human re-pass (TP-MAC-189) on a freshly rebuilt bundle remains
  the gate; F-4 and A3 cannot close until it is performed and recorded. The
  bundle reflects 2b; it should be rebuilt after 2c before the re-pass.
- Git commit/push performed in this run at project-authority approval (shared
  monorepo `main`, explicit-path staging, behind-guard) — 2b source + this
  record only.
