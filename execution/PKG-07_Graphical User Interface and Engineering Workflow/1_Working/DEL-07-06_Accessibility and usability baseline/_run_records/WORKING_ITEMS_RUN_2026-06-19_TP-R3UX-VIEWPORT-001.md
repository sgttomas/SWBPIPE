---
run-id: WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-VIEWPORT-001
timestamp: 2026-06-19T16:30:00-06:00
run-status: SUCCESS
persona: WORKING_ITEMS
primary-deliverable: DEL-07-06
tranche-ids:
  - TP-R3UX-VIEWPORT-001
smoke-ids: []
---

# WORKING_ITEMS Run Record - Professional-Grade Viewport (Phase 1)

## Objective

Raise the 3D viewport to a professional-grade (CAD-style) standard in direct
response to the project authority's review of the rebuilt bundle. This is
Phase 1 of a two-phase "professional grade" push that the authority requested
after the C5.7R corrective tranche (TP-R3UX-CORRECTIVE-001) shipped; Phase 2
(information-architecture restructure: native macOS menu bar, object-creation
toolbar, 3D-dominant collapsible panels) is sequenced after this and not part of
this run.

The authority's verbatim viewport critique drove the scope: tags/arrows overlaid
on the model should be toggleable and should rotate/track with the part they
annotate; clicking a part should bring up its information (not functional
before); the X/Y/Z unit vectors were not showing properly; and the single-plane
lattice from a fixed point was a distraction.

Performed at the project authority's direct instruction ("get this app to
professional grade", "viewport first, then IA"). This run does NOT close F-4,
the A3 authoring-usability finding, the packaged human re-pass, R3 exit review,
lifecycle issuance, release readiness, professional approval, certification,
sealing, authentication, or code-compliance. No new packaged smoke was executed;
the human TP-MAC-189 re-pass remains the gate.

## Authority and scope

- `plans/PLAN_2026-06-18_workspace_redesign_c5_7.md` §8 criterion C (canvas reads
  as 3D) and §10 scope boundaries.
- `DEC-035`: F-4 and A3 remain R3-exit blockers until human packaged evidence
  closes them.
- `DEC-041`/`D-21`: agent panel remains a reserved seam; not built.
- `DEC-018`: dual units are display-only; storage unchanged (not touched here).
- Boundary authorities: `docs/CONTRACT.md`, `docs/IP_AND_DATA_BOUNDARY.md`,
  project `AGENTS.md`.

Allowed write scope used: desktop frontend source/styles and this DEL-07-06 run
record only. Files changed: `src/features/viewport/PipeViewport.tsx`,
`src/styles.css`.

## Outputs produced

All within `PipeViewport.tsx` and its styles; no backend, schema, or contract
change.

- **Click-to-select via raycasting (was not functional).** Scene meshes are
  tagged with `userData.entityRef`; a pointer-down on the canvas raycasts the
  pickable set and, on a hit, selects that entity (pipe / node / support /
  component) — falling back to the existing node-draft capture only when nothing
  is picked. Clicking a part is now the primary way to bring up its information.
- **Orientation gizmo with real X/Y/Z vectors (was not showing).** A dedicated
  corner renderer draws an `AxesHelper` plus canvas-texture X/Y/Z sprite labels,
  with its camera synced to the main view each frame so the triad rotates with
  the model orientation. Replaces the prior static text pills that did not
  reflect orientation.
- **Anchored, toggleable annotations (were free-floating overlays).** Entity
  labels are projected from their real 3D world position to screen space every
  frame, so a tag tracks the part it annotates through orbit/pan/zoom and hides
  when its anchor is behind the camera. A per-kind vertical offset keeps
  co-located support/component labels from overlapping their node. Gated by a
  Labels toggle.
- **3D load arrows (replaced flat overlay glyphs).** Force/moment loads render as
  in-scene `ArrowHelper`s at the node or element midpoint, oriented from the
  load's global axis and sign, colored by kind (force / moment). Gated by a Loads
  toggle.
- **Model-centered reference grid (replaced the distracting fixed-point
  lattice).** A subtle ground grid is centered under the model bounds on the
  global ground plane instead of the prior single-plane lattice anchored at a
  fixed point. Gated by a Grid toggle.
- **Display toggles** for Labels / Loads / Grid added to the viewport toolbar
  (all default-on), with pressed-state styling.
- Removed the now-dead flat overlay HTML/CSS (`viewport-load-glyphs`,
  `viewport-selection-handles`) superseded by the in-scene equivalents.

## Validation

- `npx tsc -b --noEmit` (apps/desktop) — clean.
- `npm test --workspace apps/desktop` — passed: 19 test files / 406 tests (no
  change in count; viewport 3D paths do not execute under jsdom — see note).
- `npm run test:e2e --workspace apps/desktop` — passed: 18 Playwright checks
  across chromium desktop and compact projects. This is the real-browser
  verification of the WebGL paths (raycast pick, gizmo, label anchoring, arrows).
  An initial e2e run surfaced a regression — the support label, now anchored to
  its node's real position, intercepted the node's selection click — which was
  fixed with the per-kind label offset and re-verified green (full suite 18/18).
- `npm run tauri -- build --bundles app` (apps/desktop) — bundled
  `OpenPipeStress Technical Preview.app`; 6s boot probe reported the process
  ALIVE with exit 0 and no stdout/stderr. Built from the working-tree changes for
  the authority's visual review prior to this commit.

Test-coupling note: unit tests run under jsdom, which reports no WebGL, so the
entire 3D effect (raycast/gizmo/anchoring/arrows) early-returns and is exercised
only by the Playwright e2e suite in a real browser. The `viewport-select-*`
selection-layer buttons remain present (Labels default-on) because unit tests and
the pipe-endpoint-pick path select through them.

## Boundary review

This run changed frontend viewport presentation and styling only. It did not
change solver mechanics, schemas, evaluator grammar, persistence contracts,
backend APIs, unit storage semantics, private-data policy, protected-content
policy, network/telemetry posture, rule-pack authoring authority, the agent-panel
seam (D-21), lifecycle state, release-readiness status, or professional-boundary
semantics. No new operation kinds or backend/engine contracts were introduced.

No protected standards content, private project data, repository-default
private-data write, network/telemetry feature, professional approval,
certification, sealing, authentication, code-compliance, release-readiness, or
R3 exit-review claim is created.

## Residuals and next item

- Phase 2 (IA restructure: native macOS menu bar via Tauri menu API,
  object-creation toolbar, 3D-dominant collapsible/hidden-by-default panels,
  Inspector auto-open on selection) is the committed next step.
- The packaged human re-pass (TP-MAC-189) on a freshly rebuilt bundle remains the
  gate; F-4 and A3 cannot close until it is performed and recorded. The bundle
  built in this run reflects Phase 1 only; it should be rebuilt after Phase 2
  before the re-pass.
- Git commit/push performed in this run at project-authority approval (shared
  monorepo `main`, explicit-path staging, behind-guard) — Phase 1 source + this
  record only.
