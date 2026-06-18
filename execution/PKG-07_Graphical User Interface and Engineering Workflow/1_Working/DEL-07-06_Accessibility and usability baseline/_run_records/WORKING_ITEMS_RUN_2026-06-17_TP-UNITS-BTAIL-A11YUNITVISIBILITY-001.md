# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-A11YUNITVISIBILITY-001

## Scope

- Persona: WORKING_ITEMS.
- Tranche: Phase B-tail / C5 usability-support accessibility unit-visibility
  evidence.
- Primary deliverable: DEL-07-06 accessibility and usability baseline.
- Supporting deliverables: DEL-02-02 unit system and dimensional-analysis core
  contract; DEL-07-05 results viewer.

## Changes

- `AccessibilityBaselinePanel` now exports `unit_visibility_evidence`.
- The evidence records DEC-018/DEL-02-02/DEL-07-05/DEL-07-06 basis refs,
  sorted project units, review surfaces that keep unit labels visible, no
  color-only unit signaling, no inferred/default units, and
  `conversion_performed=false`.
- The panel now renders `accessibility-baseline-unit-visibility` so browser
  review shows the unit-visibility policy, model unit signature, result-row
  unit visibility basis, and no-conversion posture.

## Validation

- `npm test --workspace apps/desktop -- --run src/App.test.tsx` passed 1/1
  file and 56/56 tests.
- `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop -- e2e/r2-smoke.spec.ts -g "R2 from-blank GUI journey authors the A12 rehearsal script"`
  passed 2/2 focused Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- DEC-025 sweep evidence is recorded separately at closeout.

## Boundary

- Accessibility-baseline review metadata only.
- No final accessibility conformance target, desktop accessibility-tree audit
  claim, solver behavior, rule-check behavior, unit conversion API, DEC-018
  catalog constant, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## Residual

- C5.7 remains gated on a human packaged A12/R3 pass record; C5.8 remains
  dependent on C5.7.
- Broader B-tail target-format conversion witnesses and remaining app
  unit-policy surfaces stay tracked in the active completion plan.
