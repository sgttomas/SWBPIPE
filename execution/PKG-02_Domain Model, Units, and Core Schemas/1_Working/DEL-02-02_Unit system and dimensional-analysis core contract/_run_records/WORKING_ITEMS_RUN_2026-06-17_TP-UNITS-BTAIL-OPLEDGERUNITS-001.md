# WORKING_ITEMS RUN - TP-UNITS-BTAIL-OPLEDGERUNITS-001

## Scope

- Package: PKG-02 Domain Model, Units, and Core Schemas.
- Supporting deliverable: DEL-02-02 Unit system and dimensional-analysis core
  contract.
- Primary deliverable: DEL-16-03 User acceptance and operation audit trail.
- Tranche: `TP-UNITS-BTAIL-OPLEDGERUNITS-001`.
- Smoke ID: `TP-MAC-230`.

## Supporting Unit Evidence

- The Operation Review Ledger now visibly reports and exports unit-policy
  evidence for queued GUI operation intents and agent proposal review records.
- GUI intent records count explicit unit-bearing and dimensionless changes
  from operation metadata; agent proposal records without unit metadata remain
  dimensionless/no unit-validation evidence.
- The row and packet report unit-validation statuses,
  `receipt_units=not_serialized_in_review_ledger`, and `conversion=false`.

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

- No unit conversion or inference behavior changed.
- No DEC-018 catalog constant, schema dimension enum, operation application,
  acceptance semantics, durable audit persistence, protected standards
  content, private data, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.
