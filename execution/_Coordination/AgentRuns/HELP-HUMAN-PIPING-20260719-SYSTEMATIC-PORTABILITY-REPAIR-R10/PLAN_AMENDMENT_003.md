# PLAN AMENDMENT 003 — Post-v2 Final Sweep

- RunID: `HELP-HUMAN-PIPING-20260719-SYSTEMATIC-PORTABILITY-REPAIR-R10`
- Plan version: `4`
- Disposition: `AMEND`
- Authority: HELP_HUMAN disposition within the owner-approved validation objective and bounded sweeps directory

## Sweep Dispositions

- Preserve `SWEEP_20260719T193438Z_dca98da8527f-dirty.json` byte-exact as `INVALIDATED_BY_POST_SWEEP_CODE_CHANGE`, superseded and non-admitted for final R10 fan-in.
- Authorize exactly one additional post-v2 DEC-025 `SWEEP_*.json` in the already bounded sweeps directory.
- Only the new post-v2 sweep may satisfy final R10 fan-in.

## Execution

Re-trigger the same author identity solely to run the sweep once, validate its
overall result and all five surfaces, bind its SHA-256, and write versioned
`RETURN_V3.md` and `STATUS_V3.json`. No further code or policy edit is
authorized. If the sweep fails, R10 fails closed; no retry occurs without
escalation.

On PASS, re-trigger the same verifier identity for a fresh v2 read-only review
of the exact final tree, including both sweep dispositions and confirmation
that no broad classifier was reintroduced. Preserve all earlier returns,
statuses, and amendments.
