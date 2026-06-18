# WORKING_ITEMS RUN - TP-UNITS-BTAIL-OPLEDGERUNITS-001

## Scope

- Package: PKG-16 Model Operation and Agent Proposal Framework.
- Primary deliverable: DEL-16-03 User acceptance and operation audit trail.
- Supporting deliverable: DEL-02-02 Unit system and dimensional-analysis core
  contract.
- Tranche: `TP-UNITS-BTAIL-OPLEDGERUNITS-001`.
- Smoke ID: `TP-MAC-230`.

## Work Performed

- Added visible `operation-ledger-unit-policy` evidence to the Operation
  Review Ledger panel.
- Added exported `unit_policy_evidence` to the local ledger JSON packet.
- The summary records record count, unit-bearing change count,
  dimensionless change count, unit-validation statuses,
  `receipt_units=not_serialized_in_review_ledger`, and `conversion=false`.
- Extended App and Playwright coverage for GUI intent and agent proposal
  ledger paths.

## Validation

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "records viewport editor intents without direct persisted-project mutation|carries queued editor intents into the report packet as review-only operation context|shows computed mechanics diagnostics in results, knowledge, and review-only proposal context"`
  passed 3/3 selected tests.
- `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
  passed 1/1 focused Chromium desktop test.
- `git diff --check` passed.
- `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
  tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18 tests.

## Boundary

- UI/export review evidence only.
- No operation application, acceptance semantics, durable audit persistence,
  receipt schema, solver behavior, unit-conversion API, DEC-018 catalog
  constant, schema dimension enum, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.
