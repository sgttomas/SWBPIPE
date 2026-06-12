---
run-id: WORKING_ITEMS_RUN_2026-06-12_solver_boundary_unit_normalization
timestamp: 2026-06-12T03:28:06-06:00
persona: WORKING_ITEMS
tranche-id: TP-UNITS-B2-SOLVERNORM-001
package-id: PKG-02
deliverable-id: DEL-02-02
run-status: SUCCESS
authority:
  - docs/PRD.md §10 FR-002
  - docs/CONTRACT.md OPS-K-UNIT-1
  - docs/SPEC.md §4
  - execution/_Decomposition/SOFTWARE_DECOMP.md DEC-018
  - plans/PLAN_2026-06-10_prd_completion.md Phase B B2/B3
write-scope:
  - core/product_physics/Cargo.toml
  - core/product_physics/Cargo.lock
  - core/product_physics/src/lib.rs
  - core/product_physics/src/validation.rs
  - apps/desktop/src-tauri/Cargo.lock
  - apps/desktop/SMOKE.md
  - execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/MEMORY.md
  - execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/MEMORY.md
  - execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/**
  - execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/**
  - plans/PLAN_2026-06-10_prd_completion.md
  - plans/PLAN_COMPLETION_LOG.md
---

# WORKING_ITEMS Run - TP-UNITS-B2-SOLVERNORM-001

## Scope

Implemented the B2 solver-boundary normalization slice for the preview
mechanics adapter. `core/product_physics` now accepts DEC-018-compatible input
units at its public model boundary, normalizes unit-bearing values to
SI-canonical quantities before solver assembly, and still blocks incompatible
unit dimensions.

## Changes

- Added `open_pipe_stress_units` as an explicit `core/product_physics`
  dependency.
- Replaced exact string unit validation with dimension-aware catalog checks for
  material moduli, thermal expansion coefficient, pipe dimensions, support
  stiffness, and primitive-load magnitudes.
- Added a normalization pass before model assembly so compatible inputs such
  as `MPa`, `mm`, and `kPa` are converted to canonical `Pa`, `m`, and `Pa`
  solver-boundary quantities.
- Preserved incompatible-unit blocking through `UNIT_INPUT_INVALID` and added
  `UNIT_CONVERSION_UNAVAILABLE` for any normalization failure after validation.
- Added product-physics regression coverage proving a mixed-unit model solves
  to the same rounded result surface as the SI fixture, while an incompatible
  material unit and incompatible load unit remain blocking.

## Validation

- `cargo test --manifest-path core/product_physics/Cargo.toml`
  - PASS, 25 unit tests and 0 doctests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - PASS, 32 unit tests and 0 doctests.
- `cargo test --manifest-path core/runner/headless/Cargo.toml`
  - PASS, 11 unit tests and 0 doctests.
- `npm test --workspace apps/desktop -- --run src/App.test.tsx src/features/report/renderedReport.test.tsx src/services/unitCatalogService.test.ts`
  - PASS, 56 tests across 3 files.
- `npm run test:e2e --workspace apps/desktop -- --project=chromium-desktop apps/desktop/e2e/r2-smoke.spec.ts`
  - PASS, 2 Playwright tests after wasm engine build.
- `npm test --workspace apps/desktop -- --run`
  - PASS, 216 tests across 9 files.
- `npm run build --workspace apps/desktop`
  - PASS with the pre-existing Vite chunk-size warning.

## Boundary Review

- This tranche does not add visible unit picker controls, report-time
  conversion, import/export unit conversion, rule-pack unit I/O, browser
  fallback unit catalogs, protected standards content, proprietary vendor
  data, private project data, release-readiness claims, professional approval,
  certification, sealing, authentication, or code-compliance claims.
- The preview mechanics solver boundary now consumes SI-canonical normalized
  values, but entered model units remain preserved in the app model document
  and report packet disclosure.

## Handoffs

- B2 still owns broader visible app unit entry/pickers, report renderer body
  expansion beyond packet disclosure, imports/exports, and rule-pack unit I/O.
- B3 still owns the broader conversion-witness/tolerance corpus under
  D-04/DEC-026, including more exhaustive mixed-unit round trips beyond the
  product-physics boundary witness landed here.
