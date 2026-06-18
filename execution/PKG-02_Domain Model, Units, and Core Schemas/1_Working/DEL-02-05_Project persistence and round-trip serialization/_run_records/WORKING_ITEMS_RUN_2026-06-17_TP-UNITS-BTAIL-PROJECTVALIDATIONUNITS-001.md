# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-PROJECTVALIDATIONUNITS-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Primary deliverable: DEL-02-05 - Project persistence and round-trip serialization

Tranche: `TP-UNITS-BTAIL-PROJECTVALIDATIONUNITS-001`

SMOKE row: `TP-MAC-214`

## Scope

Bounded Phase B-tail project-validation/unit-evidence slice while C5.7
remains human-execution gated. This tranche adds explicit DEC-018
unit-policy evidence to the Project Validation Preflight packet for the
existing unit round-trip metadata check.

## Changes

- Added `unit_policy_evidence` to
  `apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx`.
- Recorded `unit-system:dec-018-si-dual-display`, entered-unit preservation,
  sorted model units, model unit-bearing record count, persisted round-trip
  status/signature when available, `conversion_performed=false`, and
  DEC-018/DEL-02-02/DEL-02-05 basis refs.
- Added visible `project-validation-unit-policy` UI evidence showing model
  units, unit-bearing record count, round-trip status, and conversion status.
- Extended App Vitest to cover not-persisted and saved/opened packet/UI unit
  policy evidence.
- Extended R2/R3 Playwright smoke to check the visible saved/opened unit
  policy row in browser execution.
- Updated `apps/desktop/SMOKE.md`, `plans/PLAN_2026-06-17_prd_completion.md`,
  `plans/PLAN_COMPLETION_LOG.md`, and affected deliverable memories.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` - passed 56/56.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` - passed
  14/14 Playwright tests.
- `npm test --workspace apps/desktop` - passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` - passed with the existing Vite
  large-chunk warning.
- `git diff --check` - passed.

## Boundary Review

- No persistence semantics, migration policy, hash canonicalization, schema
  versioning, unit conversion API, release threshold, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim was introduced.
- No protected standards content, proprietary data, private project payload,
  telemetry, network path, daemon, or repository-default private-data write was
  introduced.

## Residuals

- C5.7 human packaged pass remains human-gated; C5.8 depends on that pass.
- Phase B-tail still owns broader app unit entry/pickers beyond the landed
  surfaces and remaining target-format conversion witnesses outside already
  covered boundaries.
