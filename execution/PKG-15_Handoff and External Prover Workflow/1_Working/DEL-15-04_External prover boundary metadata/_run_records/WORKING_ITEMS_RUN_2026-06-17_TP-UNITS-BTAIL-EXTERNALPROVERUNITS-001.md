# WORKING_ITEMS RUN - TP-UNITS-BTAIL-EXTERNALPROVERUNITS-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Primary deliverable: DEL-15-04 - External prover boundary metadata

Supporting deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

## Objective

Advance the Phase B-tail unit-aware I/O remainder while C5.7 remains
human-execution gated by adding explicit DEC-018 unit-policy evidence to the
metadata-only external-prover boundary preview.

## Scope

- App slice: `apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx`.
- Test slice: `apps/desktop/src/App.test.tsx`.
- Evidence surfaces: `apps/desktop/SMOKE.md`,
  `plans/PLAN_COMPLETION_LOG.md`,
  `plans/PLAN_2026-06-17_prd_completion.md`, this run record, and supporting
  deliverable memory/run record.

## Changes

- Added `unit_policy_evidence` to the External Prover Boundary exported JSON
  packet.
- Added visible `external-prover-unit-policy` UI evidence showing source/result
  unit disclosure, conversion status, external-prover unit policy, and witness
  count.
- Reused the shared `buildExportUnitSystemDisclosure` helper so the packet
  records `unit-system:dec-018-si-dual-display`, `entered_units_preserved`,
  source/result unit disclosure, and `conversion_performed=false`.
- Extended App Vitest assertions for both run-context-pending and
  mechanics-context-bound external-prover packets.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` - passed 55/55.
- `npm test --workspace apps/desktop` - passed 18/18 files and 398/398 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` - passed
  14/14 Playwright tests.
- `npm run build --workspace apps/desktop` - passed with the existing Vite
  large-chunk warning.
- `git diff --check` - passed.

## Boundary Review

- No external solver/prover was invoked.
- No target parser, commercial result ingestion, target writer, conversion
  API, target compatibility claim, target support claim, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim was introduced.
- No protected standards content, proprietary data, private project payload,
  telemetry, network path, daemon, or repository-default private-data write was
  introduced.

## Residuals

- C5.7 human packaged pass remains human-gated; C5.8 depends on that pass.
- Phase B-tail still owns broader app unit entry/pickers beyond the landed
  surfaces and remaining target-format conversion witnesses outside already
  covered boundaries.
