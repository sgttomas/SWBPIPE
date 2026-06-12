---
run-id: WORKING_ITEMS_RUN_2026-06-12_solver_boundary_unit_normalization
timestamp: 2026-06-12T03:28:06-06:00
persona: WORKING_ITEMS
tranche-id: TP-UNITS-B2-SOLVERNORM-001
package-id: PKG-07
deliverable-id: DEL-07-07
run-status: SUCCESS
authority:
  - docs/PRD.md §10 FR-002
  - docs/PRD.md §22.3
  - docs/CONTRACT.md OPS-K-UNIT-1
  - execution/_Decomposition/SOFTWARE_DECOMP.md DEC-018
  - plans/PLAN_2026-06-10_prd_completion.md Phase B B2
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

Recorded the solve-execution side of the B2 solver-boundary normalization
tranche. The desktop/backend solve path continues to run through
`core/product_physics`, but that adapter now normalizes compatible mixed-unit
inputs before solver assembly instead of requiring every incoming quantity to
already use the canonical string.

## Changes

- `core/product_physics` now depends on the accepted `core/units` crate.
- The adapter validates quantity unit symbols by DEC-018 dimension
  compatibility, then converts compatible values to canonical solver units.
- Product-physics tests prove the invented preview model still solves
  deterministically when material moduli are entered in `MPa`, pipe dimensions
  in `mm`, and pressure loads in `kPa`.
- Incompatible unit dimensions continue to block solve execution with
  `UNIT_INPUT_INVALID`.

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

This is backend/app integration evidence for the solve boundary only. It does
not add GUI unit pickers, browser-mode edited-model solving, report-time
conversion, import/export conversion, rule-pack unit I/O, protected standards
content, private project data, release-readiness claims, professional
approval, certification, sealing, authentication, or code-compliance claims.

## Handoffs

R2 solve/report smoke remains green. Remaining B2 unit work sits outside this
solve-execution slice: broader visible app unit entry/pickers, report renderer
body expansion beyond packet disclosure, imports/exports, and rule-pack unit
I/O.
