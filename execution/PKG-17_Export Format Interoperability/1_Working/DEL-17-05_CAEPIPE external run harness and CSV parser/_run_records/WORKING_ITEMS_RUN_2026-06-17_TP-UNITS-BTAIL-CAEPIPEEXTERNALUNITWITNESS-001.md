# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-CAEPIPEEXTERNALUNITWITNESS-001

Date: 2026-06-17
Persona: WORKING_ITEMS
Primary deliverable: DEL-17-05 - CAEPIPE external run harness and CSV parser
Supporting deliverables:
- DEL-02-02 - Unit system and dimensional-analysis core contract
- DEL-08-04 - Result export format
Tranche: TP-UNITS-BTAIL-CAEPIPEEXTERNALUNITWITNESS-001
Smoke target: TP-MAC-201

## Scope

Bounded Phase B-tail target-format conversion witness slice while C5.7
remains human-execution gated. The tranche adds unit-disclosure and
unit-preservation evidence to the parser-only CAEPIPE external harness package
for invented public CSV rows.

## Changes

- Added DEC-018 unit-system disclosure to the CAEPIPE external harness packet.
- Added `unit_witness_policy` and `unit_preservation_witnesses[]` preserving
  each parsed invented CSV row's source value, unit, and inferred dimension
  with `conversion_performed=false`.
- Added visible desktop panel evidence:
  `data-testid="caepipe-external-units"` and
  `data-testid="caepipe-external-unit-witnesses"`.
- Added App Vitest assertions for disclosure, target units, witness count,
  row-level force witness preservation, and validation-report checks.
- Added R2 Playwright smoke assertions for the visible unit disclosure and
  witness count.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
  passed 2/2 Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `git diff --check` passed.

## Evidence Updates

- `apps/desktop/SMOKE.md` TP-MAC-201 entry.
- `plans/PLAN_COMPLETION_LOG.md` entry for
  `TP-UNITS-BTAIL-CAEPIPEEXTERNALUNITWITNESS-001`.
- `plans/PLAN_2026-06-17_prd_completion.md` B-tail partial row updated.
- DEL-17-05, DEL-02-02, and DEL-08-04 memories updated.

## Boundary Review

CAEPIPE external parser unit metadata only. No external CAEPIPE execution,
executable/license/path requirement, target compatibility claim, solver
validation claim, schema contract change, unit conversion API, protected
standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim was introduced.
