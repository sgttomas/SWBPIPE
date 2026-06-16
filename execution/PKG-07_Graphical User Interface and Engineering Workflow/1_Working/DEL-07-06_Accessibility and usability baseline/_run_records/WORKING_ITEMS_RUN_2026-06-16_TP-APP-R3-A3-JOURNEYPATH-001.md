# WORKING_ITEMS Run Record — TP-APP-R3-A3-JOURNEYPATH-001

**Date:** 2026-06-16
**Persona:** WORKING_ITEMS
**Primary deliverable:** DEL-07-06 — Accessibility and usability baseline
**Package:** PKG-07 — Graphical User Interface and Engineering Workflow
**Tranche:** TP-APP-R3-A3-JOURNEYPATH-001
**Plan lane:** A3 authoring-journey usability / R3 blocking residual support

## Scope

Added a compact authoring-journey status rail to the desktop shell, derived
from existing session/model state:

- Model counts: nodes, pipes, supports.
- Apply status: queued and applied operation counts.
- Loads status: load cases, primitive loads, combinations.
- Solve status: solve state and result-row count.
- Rule status: current rule-check aggregate or model rule-check status.
- Report and Project status: solved-run availability and local project
  operation state.

Each rail item is an actual navigation control into the existing workspace
section. The change preserves the persistent tree / viewport / inspector core
and does not add schema, solver, persistence, rule-runner, grammar, or
protected/private-data behavior.

The compact desktop layout was adjusted to give the lower dock more height at
the 1280x800 lane while keeping the journey rail bounded.

## Files Changed

- `apps/desktop/src/App.tsx`
- `apps/desktop/src/styles.css`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `apps/desktop/SMOKE.md`
- `plans/PLAN_COMPLETION_LOG.md`
- `plans/PLAN_2026-06-10_prd_completion.md`
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/MEMORY.md`

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx App.deadControls.test.tsx`
  - 2 files, 55 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite chunk-size warning.
- `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop -- --project=chromium-compact --grep "R2 desktop preview smoke covers"`
  - 1 compact-viewport test passed.
- In-app Browser at 1280x800:
  - `workspace-journey-loads` present with `2 cases; 7 loads; 1 combos`.
  - Coordinate click activated the Load Cases section.
  - Measured geometry: `bodyOverflow=0`, `viewportOverflow=0`,
    `itemOverflow=0`, `dockBodyHeight=168`, `journeyHeight=66`.
- `npm test --workspace apps/desktop`
  - 18 files, 386 tests passed.
- `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop`
  - 10 tests passed across 1440x920 and 1280x800.

## Boundary Review

This is a desktop-shell usability and evidence tranche only. It uses invented
preview state and existing local-session model state. It does not load or
commit private project data, add network or telemetry behavior, alter solver or
rule-check math, introduce protected standards content, or make release,
professional approval, certification, sealing, authentication, or
code-compliance claims.

## Residuals

F-4 remains open: only a human-completed packaged GUI journey can close the
recorded packaged-runtime finding. This tranche reduces the authoring-journey
usability residual and proves compact browser geometry; it does not by itself
claim the human journey is complete.

Remaining A3 residuals include broader canvas/component/rigid authoring
coverage as new surfaces land and the DEL-07-06 WCAG target decision
(`DEL-07-06-CF-001`).
