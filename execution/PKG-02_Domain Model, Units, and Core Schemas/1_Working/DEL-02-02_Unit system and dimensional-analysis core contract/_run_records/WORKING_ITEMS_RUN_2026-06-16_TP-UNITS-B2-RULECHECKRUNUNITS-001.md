# WORKING_ITEMS Run Record - TP-UNITS-B2-RULECHECKRUNUNITS-001

## Scope

- Persona: WORKING_ITEMS.
- Tranche: B2/B3 rule-check run-time unit binding controls.
- Supporting deliverable: DEL-02-02 unit system and dimensional-analysis core
  contract.

## Changes

- The run-check panel now uses the DEC-018 unit catalog as GUI metadata for
  runtime user-supplied value and value-slot binding units in desktop mode.
- Browser preview keeps manual stored-unit text entry and explicitly reports
  that it did not synthesize a fallback catalog.
- The selector path is dimension-filtered and preserves any current stored
  out-of-catalog unit instead of silently replacing it.

## Validation

- Focused `RuleCheckRunPanel` Vitest passed 18/18.
- Full desktop Vitest passed 18 files / 389 tests.
- Desktop build passed with the existing Vite chunk-size warning.
- Focused run-check Playwright passed 2/2; full desktop Playwright passed
  10/10 with `--workers=1`.
- In-app Browser verified the browser fallback status, unit values, and basis
  text.
- `python3 tools/release/run_evidence_sweep.py --execute` passed on the dirty
  tree and wrote
  `validation/evidence/sweeps/SWEEP_20260616T034816Z_9be2d805ab17-dirty.json`.

## Boundary

- No DEC-018 catalog constant, schema dimension enum, tolerance policy,
  evaluator behavior, hidden unit fallback, protected content, private data,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## Residual

- Remaining B2/B3 unit I/O and witness/tolerance corpus work stays tracked in
  the active completion plan.
