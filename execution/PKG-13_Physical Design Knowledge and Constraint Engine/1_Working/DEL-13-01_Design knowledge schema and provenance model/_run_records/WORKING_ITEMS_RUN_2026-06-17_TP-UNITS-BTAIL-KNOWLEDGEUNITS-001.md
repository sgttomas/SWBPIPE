# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-KNOWLEDGEUNITS-001

- Agent: WORKING_ITEMS
- Date: 2026-06-17
- Package: PKG-13
- Deliverable: DEL-13-01 Design knowledge schema and provenance model
- Tranche: Phase B-tail design-knowledge computed-unit context evidence.
- Status: implemented and validated.

## Scope

Bounded Phase B-tail review-surface evidence slice while C5.7 remains
human-execution gated. The desktop Design Knowledge panel now surfaces explicit
unit context for computed result-derived knowledge records.

## Changes

- Added `data-testid="knowledge-unit-context"` to the Design Knowledge panel
  when computed mechanics results are present.
- The row reports the count of computed unit-bearing knowledge result refs,
  their result units in the established result-display order,
  `source=computed_preview_result`, and `conversion=false`.
- Added focused App and R2 Playwright checks that verify the unit context after
  the mechanics preview solve.
- Updated `apps/desktop/SMOKE.md` with TP-MAC-223.

## Validation

- `npm test --workspace apps/desktop -- --run src/App.test.tsx` passed 1/1
  file and 56/56 tests.
- `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop --
  e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results,
  report, and viewport overlay"` passed 2/2 focused Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.

## Boundary

This tranche changed design-knowledge review UI evidence only. It did not
change the design-knowledge schema, result values, solver behavior, unit
conversion API, DEC-018 catalog constant, schema dimension enum, protected
standards content, private payload, lifecycle state, release-readiness
posture, professional approval, certification, sealing, authentication, or
code-compliance posture.

## Handoff

- Accepted upstream basis: DAG-006 current authority, DEL-13-01 design
  knowledge schema/provenance model, DEL-02-02 unit-system contract, and the
  app's existing invented mechanics result envelope.
- Derivative-package status: App UI evidence, SMOKE entry, and this run record
  are derivative evidence; they are not decomposition truth.
- Closure verdict: implementation and validation evidence recorded for this
  bounded tranche.
- Rerun requirements: rerun the focused App test, focused R2 smoke, full
  desktop Vitest, and desktop build after any change to computed knowledge
  records, result-unit ordering, or result envelope binding.
- Remaining blockers: C5.7 remains a human packaged-pass action; C5.8 remains
  dependent on C5.7.
