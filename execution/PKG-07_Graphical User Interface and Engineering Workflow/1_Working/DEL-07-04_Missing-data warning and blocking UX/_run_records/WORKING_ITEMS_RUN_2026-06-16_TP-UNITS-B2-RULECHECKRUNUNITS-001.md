# WORKING_ITEMS Run Record - TP-UNITS-B2-RULECHECKRUNUNITS-001

## Scope

- Persona: WORKING_ITEMS.
- Tranche: B2/B3 rule-check run-time unit binding controls.
- Primary deliverable: DEL-07-04 missing-data warning and blocking UX.
- Supporting deliverables: DEL-06-02 unit-aware expression evaluator and
  DEL-02-02 unit system contract.

## Changes

- `RuleCheckRunPanel` now loads the DEC-018 unit catalog when a loaded rule
  pack has runtime user-supplied value or value-slot bindings.
- Desktop/Tauri paths render dimension-filtered unit selectors for those
  binding units and preserve out-of-catalog stored units as current values.
- Browser preview keeps the prior manual text unit fields and reports that no
  fallback catalog was synthesized.
- The same explicit run-check binding payload shape is preserved:
  `{value, unit, dimension}`. No schema or evaluator behavior changed.

## Validation

- `npm test --workspace apps/desktop -- RuleCheckRunPanel.test.tsx` passed
  18/18.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  chunk-size warning.
- `npm run test:e2e --workspace apps/desktop -- --grep "run-rule-checks panel"`
  passed 2/2.
- `npm test --workspace apps/desktop` passed 18 files / 389 tests.
- `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 10/10.
- In-app Browser at `http://127.0.0.1:5173/` verified browser fallback
  status, `INPUT` unit controls, `demo_unit` / `ratio` values, and
  model-metadata basis text.
- `python3 tools/release/run_evidence_sweep.py --execute` passed on the dirty
  tree and wrote
  `validation/evidence/sweeps/SWEEP_20260616T034816Z_9be2d805ab17-dirty.json`.

## Boundary

- No schema, evaluator, rule-pack parser, writable expression text syntax,
  protected content, private project data, network/telemetry path,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## Residual

- Broader B2/B3 app unit entry/pickers and conversion/tolerance witnesses
  remain tracked in the active completion plan.
