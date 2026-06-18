# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-STORAGEUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-02-05 - Project persistence and round-trip serialization

Supporting deliverables:

- DEL-02-02 - Unit system and dimensional-analysis core contract
- DEL-08-05 - Report protected-content linter

## Scope

Bounded Phase B-tail Project Storage Audit unit-policy evidence slice while
C5.7 remains human-execution gated. The tranche exposes structured unit-policy
evidence for the existing local project unit round-trip metadata in the
Project Storage Audit panel and downloaded local storage JSON.

## Changes

- Added `unit_policy_evidence` to the Project Storage Audit packet.
- Extended `data-testid="project-storage-unit-round-trip"` to report sorted
  model units and `conversion=false` alongside the existing round-trip
  status, checked-ref count, and signature.
- Added DEC-018, DEL-02-02, and DEL-02-05 basis refs, entered-unit
  preservation, round-trip status/signature, and no-conversion policy to the
  storage audit packet.
- Added the Project Storage Audit surface to the Report Content Lint
  public unit-policy inventory, increasing unit-policy targets from 17 to 18
  while leaving target-format conversion-witness count at two.

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

This tranche changes Project Storage Audit UI/export evidence and the Report
Content Lint unit-policy inventory only. It does not change local persistence
semantics, migration policy, project-envelope schema, model hash
canonicalization, report-linter protected-content semantics, unit-conversion
APIs, DEC-018 catalog constants, schema dimension enums, protected standards
content, private payloads, lifecycle state, release readiness, professional
approval, certification, sealing, authentication, or code-compliance posture.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This B-tail slice is complete and can be consumed as supporting evidence for
DEL-02-05, DEL-02-02, and DEL-08-05. DEC-025 sweep evidence remains to be
recorded during git closeout.
