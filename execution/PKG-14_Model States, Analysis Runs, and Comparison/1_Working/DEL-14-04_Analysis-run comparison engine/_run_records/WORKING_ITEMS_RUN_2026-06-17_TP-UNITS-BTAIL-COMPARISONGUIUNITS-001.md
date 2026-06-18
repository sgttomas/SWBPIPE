# WORKING_ITEMS RUN - TP-UNITS-BTAIL-COMPARISONGUIUNITS-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Primary deliverable: DEL-14-04 - Analysis-run comparison engine

Supporting deliverables:

- DEL-14-05 - Comparison mapping, tolerance, and export contracts
- DEL-02-02 - Unit system and dimensional-analysis core contract

## Objective

Advance the Phase B-tail unit-aware I/O remainder while C5.7 remains
human-execution gated by adding explicit unit-policy evidence to the desktop
comparison workspace packet and visible comparison panel.

## Scope

- App slices:
  - `apps/desktop/src/services/previewService.ts`
  - `apps/desktop/src/types.ts`
  - `apps/desktop/src/features/comparison/ComparisonPanel.tsx`
- Test slices:
  - `apps/desktop/src/App.test.tsx`
  - `apps/desktop/e2e/r2-smoke.spec.ts`
- Evidence surfaces: `apps/desktop/SMOKE.md`,
  `plans/PLAN_COMPLETION_LOG.md`,
  `plans/PLAN_2026-06-17_prd_completion.md`, this run record, and supporting
  deliverable memory/run records.

## Changes

- Added `unit_policy_evidence` to the `PreviewComparison` packet.
- Recorded equal-explicit-unit matching policy, matched result units, unmatched
  row counts, DEC-018/DEC-026/DEL-14-05 basis refs,
  `conversion_performed=false`, and `tolerance_profile_ref=TBD`.
- Added visible `comparison-unit-policy` UI evidence showing units,
  equal-unit matching, conversion status, and tolerance status.
- Extended App Vitest to cover packet-level evidence and the rendered row.
- Extended R2/R3 Playwright smoke to check the visible comparison unit-policy
  row in browser execution.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` - passed 56/56.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` - passed
  14/14 Playwright tests.
- `npm test --workspace apps/desktop` - passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` - passed with the existing Vite
  large-chunk warning.
- `git diff --check` - passed.

## Boundary Review

- No comparison delta math, tolerance profile, default tolerance, solver
  convergence policy, external validation decision, conversion API, release
  threshold, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim was introduced.
- No protected standards content, proprietary data, private project payload,
  telemetry, network path, daemon, or repository-default private-data write was
  introduced.

## Residuals

- C5.7 human packaged pass remains human-gated; C5.8 depends on that pass.
- Phase B-tail still owns broader app unit entry/pickers beyond the landed
  surfaces and remaining target-format conversion witnesses outside already
  covered boundaries.
