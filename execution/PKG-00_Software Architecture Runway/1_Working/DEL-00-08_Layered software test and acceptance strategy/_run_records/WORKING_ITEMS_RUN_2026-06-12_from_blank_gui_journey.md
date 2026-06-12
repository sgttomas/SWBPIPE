# WORKING_ITEMS Run Record - TP-APP-R2-FROMBLANK-E2E-001

- Agent: WORKING_ITEMS (Type 1 persona)
- Date: 2026-06-12
- Tranche: `TP-APP-R2-FROMBLANK-E2E-001`
- Plan item: Phase A8 GUI test harness
- Deliverable context: `DEL-00-08` Layered software test and acceptance strategy
- Authority basis: `execution/_Coordination/_COORDINATION.md`, `plans/PLAN_2026-06-10_prd_completion.md`, PRD §22.3 R2 GUI MVP, `DEC-025` five-surface local sweep posture

## Objective

Automate the A12 from-blank rehearsal script through the browser GUI harness
as the next R2 exit-evidence spine: create a blank local model, author the
fixture-equivalent nodes/material/section/pipe/support/load/combination
through visible app controls, apply each operation through the structured
operation seam, and record the current browser-mode solve/report boundary.

## Scope

- Read and reused `fixtures/product_preview/r2_from_blank_rehearsal.json` as
  the source of GUI-entered values.
- Updated `apps/desktop/e2e/r2-smoke.spec.ts`.
- Updated DEL-00-08 memory, completion-plan/log, and desktop smoke evidence.
- Did not edit lifecycle `_STATUS.md`, release gates, SCA-005, or protected
  data surfaces.

## Changes

- Added a second Playwright test:
  `R2 from-blank GUI journey authors the A12 rehearsal script`.
- The test clicks `New blank`, then fills and queues GUI forms for:
  two nodes, one material, one standalone section, one straight pipe run, one
  anchor support, one load case, one primitive force, and one mechanics
  combination.
- Each queued intent is applied through the Apply Operations panel and asserts
  `route=local_wasm_engine` plus `professional_approval=false`.
- The test then runs browser preview mechanics and asserts the honest browser
  backend boundary: `result_rows=0` plus
  `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`.
- The test clicks the rendered-report control and asserts the existing
  browser report boundary:
  `REPORT-RENDERER-DESKTOP-ONLY`.

## Validation

- `npm run test:e2e --workspace apps/desktop` - passed, 2/2 Playwright tests
  after wasm engine build.
- `npm test --workspace apps/desktop` - passed, 8 files / 213 tests.
- `npm run build --workspace apps/desktop` - passed; Vite emitted the
  pre-existing chunk-size warning.

## Boundary Review

- The scripted data is the invented A12 fixture.
- Browser-mode edited-model solving remains explicitly blocked rather than
  silently substituting bundled solved rows.
- Browser-mode rendered-report output remains explicitly desktop-only rather
  than using a fallback renderer.
- No cloud, daemon, network, telemetry, protected content, repository-default
  private-data write, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim was
  introduced.

## Residuals

- A8 still needs a packaged Tauri/backend saved-project smoke that proves the
  authored model solves and renders through the real desktop command seams.
- A8 still needs broader SMOKE checklist parity.
- Phase B2/B3 riders routed from the 2026-06-12 human findings: preserve
  per-constant derivation/provenance text when schema/app unit bindings land,
  add executable schema-to-crate vocabulary parity for units, and keep future
  process run records in deliverable-local evidence folders rather than
  crate-local `_run_records/`.

## Lifecycle

- `DEL-00-08` remains `CHECKING`.
- This run record is implementation/test evidence only; it is not lifecycle
  issuance, release readiness, professional approval, certification, sealing,
  authentication, or code-compliance acceptance.
