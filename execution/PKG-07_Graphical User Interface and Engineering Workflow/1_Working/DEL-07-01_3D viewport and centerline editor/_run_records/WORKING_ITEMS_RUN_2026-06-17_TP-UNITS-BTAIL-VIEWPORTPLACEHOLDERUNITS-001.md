# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-VIEWPORTPLACEHOLDERUNITS-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Primary deliverable: DEL-07-01 - 3D viewport and centerline editor

Supporting deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Bounded Phase B-tail viewport placeholder/unit-validation slice while C5.7
remains human-execution gated. The tranche covers generic one-click viewport
gesture placeholders for node and pipe-run drafting.

## Changes

- Threaded the existing viewport unit-catalog route and model default length
  unit into generic placeholder intent construction.
- `create_node` and `connect_pipe_run` placeholders now carry `unit=m`,
  `dimension=length`, and `unit_validation=length=<status>` instead of
  `unit_validation=not_run`.
- Browser preview records
  `length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`.
- The `insert_component_symbol` placeholder remains
  `unit_validation=not_required_dimensionless`.
- Preserved review-only `pending_service_validation`, structured-operation
  routing, direct model mutation disallowance, and user-acceptance boundary.
- Updated App and Playwright assertions, `apps/desktop/SMOKE.md` TP-MAC-225,
  the completion log, the active completion-plan B-tail pointer, and
  deliverable memories.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "records viewport editor intents without direct persisted-project mutation"`
  - 1/1 selected test passed.
- `npx playwright test e2e/r2-smoke.spec.ts -g "viewport gesture placeholders record unit validation"`
  - 2/2 focused Playwright tests passed.
- `npm run test --workspace apps/desktop`
  - 18/18 files passed; 399/399 tests passed.
- `npx playwright test e2e/r2-smoke.spec.ts --workers=1`
  - 18/18 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.
- `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
  - 1/1 test passed after replacing the single 700 ms viewport animation
    screenshot sample with a bounded polling assertion.

Command excluded from validation evidence:

- `npx playwright test e2e/r2-smoke.spec.ts` printed all 18 tests as passed
  but did not exit promptly; the run was interrupted and exited nonzero.
- `python3 tools/release/run_evidence_sweep.py --execute` against commit
  `ad9ecf9b032604bd515ff4258b66dddf85488d00` failed in the broad R2
  Playwright smoke before the viewport-animation polling repair. The failed
  sweep artifact was removed and is not closeout evidence.

## Boundary

This tranche changes viewport placeholder metadata only. It does not change
accepted model state, operation-applier validation semantics, explicit
viewport geometry behavior, solver behavior, component schema, unit conversion
APIs, DEC-018 catalog constants, schema dimension enums, protected standards
content, private payloads, lifecycle state, release readiness, professional
approval, certification, sealing, authentication, or code-compliance posture.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This B-tail slice is complete and can be consumed as supporting evidence for
DEL-07-01 and DEL-02-02. DEC-025 sweep evidence remains to be recorded during
git closeout.
