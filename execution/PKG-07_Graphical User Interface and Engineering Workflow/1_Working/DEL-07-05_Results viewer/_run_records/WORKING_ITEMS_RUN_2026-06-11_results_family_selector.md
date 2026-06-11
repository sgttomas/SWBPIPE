# WORKING_ITEMS Run Record — Results Family Selector

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), app-integration tranche under
  `execution/_Coordination/_COORDINATION.md` Application Integration And
  Issuance Loop.
- Tranche: `TP-APP-R2-RESULTFAMILY-001`, completion plan Phase A6 second
  sub-slice.
- Deliverable context: DEL-07-05 (results viewer).

## What Changed

- The desktop results panel now exposes result-family selectors with counts
  for the current solved result envelope.
- Current invented fixture families are `Displacement=15`, `Reaction=9`,
  `Force=180`, `Moment=180`, and `Stress=263` over 647 total result rows.
- Selecting a family constrains the existing grouped result tables and resets
  pagination. The free-text result filter still composes with the selected
  family.
- Support-reaction rows and stress rows can now be inspected as dedicated
  table views without relying on text-search terms.

## Validation Evidence

- `npm test --workspace apps/desktop`: 58 passed, 0 failed.
- `npm run build --workspace apps/desktop`: TypeScript check and Vite
  production build passed with the existing chunk-size warning.
- `npm run test:e2e:desktop`: 1 passed, 0 failed.
- In-app browser smoke at `http://127.0.0.1:5175/`:
  - solved preview mechanics to `state=completed; result_rows=647`;
  - confirmed `Reaction` showed `9 of 647` and
    `Showing 1 to 9 of 9 matching results; page 1 of 1`;
  - confirmed `Stress` showed `263 of 647` and
    `Showing 1 to 50 of 263 matching results; page 1 of 6`;
  - confirmed selector buttons were visible and not overflowing;
  - confirmed zero browser console errors.
- `git diff --check -- . ':!init/init-prompt.md'`: passed for touched scope.

## Boundary Review

- This does not add governing-ratio result generation, code/rule checks,
  protected standards data, private data, release readiness, professional
  approval, certification, sealing, authentication, or code-compliance claims.
- Governing-ratio views remain deferred until result envelopes contain ratio
  rows.
- No lifecycle state change was made.
