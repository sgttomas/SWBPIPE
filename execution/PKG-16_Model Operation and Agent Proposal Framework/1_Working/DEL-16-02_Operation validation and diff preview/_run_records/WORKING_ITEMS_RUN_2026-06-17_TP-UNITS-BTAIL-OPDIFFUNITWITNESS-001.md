# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-OPDIFFUNITWITNESS-001

Date: 2026-06-17
Persona: WORKING_ITEMS
Primary deliverable: DEL-16-02 - Operation validation and diff preview
Supporting deliverables:
- DEL-16-03 - User acceptance and operation audit trail
- DEL-02-02 - Unit system and dimensional-analysis core contract
Tranche: TP-UNITS-BTAIL-OPDIFFUNITWITNESS-001
Smoke target: TP-MAC-202

## Scope

Bounded Phase B-tail app unit-evidence slice while C5.7 remains
human-execution gated. The tranche adds unit-disclosure and unit-preservation
evidence to the local Operation Diff Preview packet for queued operation
changes that already carry unit and dimension metadata.

## Changes

- Added DEC-018 unit-system disclosure to the Operation Diff Preview packet.
- Added `unit_witness_policy` and `unit_preservation_witnesses[]` preserving
  each unit-bearing diff row's before/after value text, unit, and dimension
  with `conversion_performed=false`.
- Added visible desktop panel evidence:
  `data-testid="diff-preview-units"` and
  `data-testid="diff-preview-unit-witnesses"`.
- Added focused App Vitest assertions for disclosure, witness count, value
  preservation, unit/dimension preservation, and conversion posture.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
  passed 2/2 Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `git diff --check` passed.

Playwright smoke was not extended with a witness-specific assertion for this
slice: the broad R2 smoke previews unit-bearing edits but does not retain a
queued diff row without changing later smoke flow. An attempted
queue-and-clear assertion path hung and was removed.

## Evidence Updates

- `apps/desktop/SMOKE.md` TP-MAC-202 entry.
- `plans/PLAN_COMPLETION_LOG.md` entry for
  `TP-UNITS-BTAIL-OPDIFFUNITWITNESS-001`.
- `plans/PLAN_2026-06-17_prd_completion.md` B-tail partial row updated.
- DEL-16-02, DEL-16-03, and DEL-02-02 memories updated.

## Boundary Review

Operation diff unit metadata only. No operation schema change, operation
application, accepted model-state mutation, durable acceptance persistence,
unit conversion API, protected standards content, private payload, lifecycle
state transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim was
introduced.
