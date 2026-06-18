# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-STORAGEUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-02-05 - Project persistence and round-trip serialization

Supporting deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

Additional supporting deliverable: DEL-08-05 - Report protected-content linter

## Scope

Supporting DEL-02-02 evidence for a bounded Phase B-tail Project Storage
Audit unit-policy slice. The tranche exposes the unit-system and
entered-unit-preservation basis for local project unit round-trip metadata.

## Changes

- Project Storage Audit JSON now carries `unit_policy_evidence`.
- The visible `project-storage-unit-round-trip` row now includes sorted model
  units and `conversion=false`.
- Evidence records DEC-018 unit-system reference, DEL-02-02/DEL-02-05 basis
  refs, round-trip status/signature, storage convention, and no-conversion
  policy.
- Report Content Lint inventory now includes the Project Storage Audit
  unit-policy surface.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "computed mechanics diagnostics"`
  - 1/1 selected test passed.
- `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay|R2 from-blank GUI journey authors the A12 rehearsal script" --project=chromium-desktop`
  - 2/2 focused Playwright tests passed.
- `git diff --check`
  - Passed.
- `npm run test --workspace apps/desktop`
  - 18/18 files passed; 399/399 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.
- `npx playwright test e2e/r2-smoke.spec.ts --workers=1`
  - 18/18 tests passed.

## Boundary

This tranche does not change DEC-018 catalog constants, schema dimension
enums, unit-conversion APIs, local persistence semantics, migration policy,
project-envelope schema, model hash canonicalization, report-linter
protected-content semantics, protected standards content, private payloads,
lifecycle state, release readiness, professional approval, certification,
sealing, authentication, or code-compliance posture.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This supporting unit evidence is complete for the project-storage audit slice.
DEC-025 sweep evidence remains to be recorded during git closeout.
