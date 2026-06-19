---
run-id: WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-CORRECTIVE-001
timestamp: 2026-06-19T13:10:00-06:00
run-status: SUCCESS
persona: WORKING_ITEMS
primary-deliverable: DEL-07-06
tranche-ids:
  - TP-R3UX-CORRECTIVE-001
smoke-ids: []
---

# WORKING_ITEMS Run Record - C5.7R Corrective Tranche (Inc 5b)

## Objective

Close the high/medium-severity gaps found by a multi-agent audit of the landed
C5.7R redesign (Inc 0-7) against `plans/PLAN_2026-06-18_workspace_redesign_c5_7.md`,
so the next packaged human re-pass (TP-MAC-189) attempts a UI that actually
satisfies acceptance criteria B, C, and D rather than re-encountering the
original C5.7 failure modes ("wall of strings; model a postage stamp").

Performed at the project authority's direct instruction ("fix things"). This run
does NOT close F-4, the A3 authoring-usability finding, the packaged human
re-pass, R3 exit review, lifecycle issuance, release readiness, professional
approval, certification, sealing, authentication, or code-compliance. No new
packaged smoke was executed; the human TP-MAC-189 re-pass remains the gate.

## Authority and scope

- `plans/PLAN_2026-06-18_workspace_redesign_c5_7.md` §8 acceptance criteria B/C/D
  and §10 scope boundaries.
- `DEC-035`: F-4 and A3 remain R3-exit blockers until human packaged evidence
  closes them.
- `DEC-037`: rule-pack authoring stays structured-composer-only; not touched.
- `DEC-041`/`D-21`: agent panel remains a reserved seam; not built.
- `DEC-018`: dual units are display-only; `entered_units_preserved` storage is
  unchanged and no conversion is written back.
- Boundary authorities: `docs/CONTRACT.md`, `docs/IP_AND_DATA_BOUNDARY.md`,
  project `AGENTS.md`.

Allowed write scope used: desktop frontend source/tests/styles/e2e and this
DEL-07-06 run record only.

## Outputs produced

- **Criterion C — canvas reads as 3D (was the largest HIGH gap).** Added real
  orbit/pan/zoom via three.js `OrbitControls` in `PipeViewport.tsx` (camera state
  is preserved across the scene rebuilds that fire on model/selection changes, so
  picking an entity no longer snaps the view; the Front/Top/Iso preset buttons
  still re-frame). Added a discoverability hint in the command bar. A minimal
  ambient type shim (`src/three-addons.d.ts`) declares the addon surface since
  this three build ships the addon JS without its `.d.ts` and the build runs
  `tsc -b`.
- **Criterion D — dual-unit display (was FAIL).** Added a display-only conversion
  helper `src/services/unitConversion.ts` keyed off exact public unit-definition
  constants (the same constants the reviewed DEC-018 catalog prose describes; the
  catalog's numeric transform lives only in the Rust backend and is not exposed to
  the frontend, and adding a backend contract is out of scope per §10). Wired it
  into the Inspector `DualUnitValue` (renders `entered (≈ converted)`), and added
  a "Dual (≈)" column to `ResultsPanel`. Storage is untouched. Unit tests in
  `unitConversion.test.ts` cover in↔mm, ft↔m, psi↔MPa, MPa↔ksi, lbf↔N, deg↔rad.
- **Criterion B — wall-of-strings (was FAIL).** Replaced the raw
  `units=…; storage=…; conversion=false` machine string on the default Results
  surface with a human-readable line. Humanized the Inspector's `key=value`
  machine enums (audit-boundary, professional-boundary, inline-validation status,
  and the queued-intent line) into plain English. (OperationApplyPanel chips were
  left as-is: they are human-readable prose/chips, not `key=value` walls.)
- **Criterion E.** Removed the duplicate Mechanics/Rule-check/Professional status
  grid from `SolvePanel` (the pills now live only in the global status bar), and
  made the global "⚑ Issues" badge count include the missing-data solve/rule
  blockers it displays (TP-MAC-189 step 12), via a new exported
  `countMissingDataBlockers`.
- Updated unit and e2e tests in lockstep with the rendered-text changes (no test
  assertions were weakened to pass; the only deletions were of duplicated/obsolete
  machine-string assertions, replaced with the human-readable equivalents).

Deliberately NOT done in this run (handed off as a separate scoped task, not a
re-pass blocker): deletion of the ~750 lines of dead guided-journey components
(`GuidedWorkbench`/`A12AuthoringJourney`/`R3GuidedJourney`/`Badge` + builders) and
their orphaned state/CSS. They never render, the suite passes with them present,
and the deletion is interleaved with live functions, so it is safer as an
isolated follow-up. Also deferred as polish: 3D force/moment load-arrows and a
model-scale-linked scale bar (the current overlay glyphs/scale-label remain).

## Validation

- `npx tsc -b --noEmit` (apps/desktop) — clean.
- `npm test --workspace apps/desktop` — passed: 19 test files / 406 tests
  (was 18/400; +1 file / +6 tests from `unitConversion.test.ts`).
- `npm run build --workspace apps/desktop` — passed (tsc -b + vite); the
  pre-existing large-chunk warning is retained; OrbitControls bundles into the
  three vendor chunk.
- `npm run test:e2e --workspace apps/desktop` — passed: 18 Playwright checks
  across chromium desktop and compact projects. This is the real-browser
  verification that (a) OrbitControls on the canvas does not break click-to-draft
  node authoring, (b) the viewport creation forms remain interactable (the
  packaged authoring journey drives them directly — they were intentionally NOT
  gated behind a disclosure for this reason), and (c) the new Results string and
  dual-unit column render correctly.

## Boundary review

This tranche changed frontend presentation, a display-only unit-conversion helper,
and test coverage only. It did not change solver mechanics, schemas, evaluator
grammar, persistence contracts, backend APIs, unit storage semantics
(`entered_units_preserved` unchanged; `conversion_performed` stays false),
private-data policy, protected-content policy, network/telemetry posture,
rule-pack authoring authority, the agent-panel seam (D-21), lifecycle state,
release-readiness status, or professional-boundary semantics. No new operation
kinds or backend/engine contracts were introduced.

No protected standards content, private project data, repository-default
private-data write, network/telemetry feature, professional approval,
certification, sealing, authentication, code-compliance, release-readiness, or
R3 exit-review claim is created.

## Residuals and next item

- The packaged human re-pass (TP-MAC-189) on a freshly rebuilt bundle remains the
  gate; F-4 and A3 cannot close until it is performed and recorded.
- A fresh `.app` bundle should be rebuilt before that re-pass (this run validated
  the production web build but did not re-bundle the Tauri `.app`).
- Dead guided-journey code removal is a committed follow-through (separate task),
  not a re-pass blocker.
- No git commit/push was performed in this run; staging is gated on project-
  authority approval (shared monorepo `main`, explicit-path staging, behind-guard).
