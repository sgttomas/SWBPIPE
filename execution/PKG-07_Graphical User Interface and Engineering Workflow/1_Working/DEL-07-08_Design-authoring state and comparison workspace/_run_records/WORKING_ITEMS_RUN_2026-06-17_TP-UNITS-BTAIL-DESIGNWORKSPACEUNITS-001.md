# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-DESIGNWORKSPACEUNITS-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Primary deliverable: DEL-07-08 - Design-authoring state and comparison workspace

Tranche: `TP-UNITS-BTAIL-DESIGNWORKSPACEUNITS-001`

SMOKE row: `TP-MAC-213`

## Scope

Bounded Phase B-tail design-workspace/unit-evidence slice while C5.7 remains
human-execution gated. This tranche adds explicit unit-policy evidence to the
desktop design-authoring workspace packet and makes that evidence visible in
the Results section.

## Changes

- Added `unit_policy_evidence` to the design workspace export packet in
  `apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx`.
- Recorded `unit-system:dec-018-si-dual-display`, entered-unit preservation,
  sorted model units, result units, comparison units, the embedded comparison
  unit-policy evidence ref, `conversion_performed=false`,
  `tolerance_profile_ref=TBD`, and DEC-018/DEC-026/DEL-02-02/DEL-14-04 basis
  refs.
- Added visible `design-workspace-units` UI evidence showing model units,
  result units, comparison units, and conversion status.
- Extended App Vitest to cover unsolved and solved workspace packet/UI unit
  evidence.
- Extended R2/R3 Playwright smoke to check the visible solved workspace unit
  row in browser execution.
- Updated `apps/desktop/SMOKE.md`, `plans/PLAN_2026-06-17_prd_completion.md`,
  `plans/PLAN_COMPLETION_LOG.md`, and affected deliverable memories.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` - passed 56/56.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` - passed
  14/14 Playwright tests.
- `npm test --workspace apps/desktop` - passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` - passed with the existing Vite
  large-chunk warning.
- `git diff --check` - passed.

## Boundary Review

- No model mutation, comparison delta math, tolerance profile, default
  tolerance, solver behavior, external validation decision, conversion API,
  release threshold, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim was
  introduced.
- No protected standards content, proprietary data, private project payload,
  telemetry, network path, daemon, or repository-default private-data write was
  introduced.

## Residuals

- C5.7 human packaged pass remains human-gated; C5.8 depends on that pass.
- Phase B-tail still owns broader app unit entry/pickers beyond the landed
  surfaces and remaining target-format conversion witnesses outside already
  covered boundaries.
