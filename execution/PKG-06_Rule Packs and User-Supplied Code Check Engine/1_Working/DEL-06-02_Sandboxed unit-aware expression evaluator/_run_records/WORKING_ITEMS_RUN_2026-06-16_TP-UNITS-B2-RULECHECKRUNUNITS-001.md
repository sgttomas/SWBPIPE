# WORKING_ITEMS Run Record - TP-UNITS-B2-RULECHECKRUNUNITS-001

## Scope

- Persona: WORKING_ITEMS.
- Tranche: B2/B3 rule-check run-time unit binding controls.
- Supporting deliverable: DEL-06-02 sandboxed unit-aware expression evaluator.

## Changes

- The C4 run-check GUI now supplies runtime value-binding units through
  DEC-018-aware desktop controls while preserving browser manual text entry.
- The evaluator-facing payload remains explicit `{value, unit, dimension}`;
  this tranche does not change evaluator grammar, normalization rules, schema,
  parser, or writable text syntax.
- The desktop selector filters accepted catalog entries by the declared
  binding dimension and keeps out-of-catalog stored units visible.

## Validation

- `npm test --workspace apps/desktop -- RuleCheckRunPanel.test.tsx` passed
  18/18.
- `npm test --workspace apps/desktop` passed 18 files / 389 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  chunk-size warning.
- Focused run-check Playwright passed 2/2; full desktop Playwright passed
  10/10 with `--workers=1`.
- In-app Browser verified the browser route for runtime binding units.
- `python3 tools/release/run_evidence_sweep.py --execute` passed on the dirty
  tree and wrote
  `validation/evidence/sweeps/SWEEP_20260616T034816Z_9be2d805ab17-dirty.json`.

## Boundary

- No schema, evaluator, grammar, parser, writable expression text syntax,
  protected content, private project data, network/telemetry path,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## Residual

- Mixed-unit normalization behavior remains governed by the existing
  `TP-UNITS-B2B3-RULECHECKNORM-001` evidence; this tranche covers GUI binding
  unit controls only.
