---
agent: WORKING_ITEMS
tranche_id: TP-APP-R2-FROMBLANK-REHEARSAL-001
date: 2026-06-12
status: completed
plan_item: A12
---

# WORKING_ITEMS Run Record - A12 From-Blank R2 Exit Rehearsal

## Scope

Completion-plan A12: rehearse the PRD R2 exit chain by starting from a blank
local model document, applying structured operation steps for nodes, material,
section, pipe run, support, load case, primitive load, and load combination,
then solving through the backend preview mechanics command and rendering the
A7 deterministic HTML report.

## Changes

- Added invented fixture
  `fixtures/product_preview/r2_from_blank_rehearsal.json` as the reusable A12
  script input for A8 automation.
- Added Tauri regression
  `r2_from_blank_rehearsal_authors_solves_and_renders_report`, which consumes
  the fixture, converts each fixture step into the same structured operation
  envelope used by app authoring surfaces, applies every step through
  `apply_model_operation`, solves the resulting model through
  `run_preview_mechanics`, and renders a hash-bound report through
  `render_calculation_report`.
- Bridged the A4 primitive-load creation vocabulary into
  `core/product_physics`: `concentrated_force` and `concentrated_moment` map to
  the existing equivalent-static `Occasional` mechanics category, and
  `distributed_force` maps to `Weight`. This fixes the A4->A5 integration gap
  exposed by the rehearsal.
- Added product-physics regression
  `operation_authored_primitive_categories_map_to_preview_mechanics`.

## Validation

- `python3 -m json.tool fixtures/product_preview/r2_from_blank_rehearsal.json`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
  - 24/24 passed.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 30/30 passed.
- `npm test --workspace apps/desktop`
  - 8 files, 213 tests passed.
- `npm run build --workspace apps/desktop`
  - passed; Vite emitted the pre-existing chunk-size warning.
- `npm run test:e2e --workspace apps/desktop`
  - wasm engine build passed; Playwright R2 smoke 1/1 passed.

## Evidence And Boundaries

The A12 fixture and report inputs are invented public examples only. The
regression asserts no private payload, protected content, release readiness,
professional approval, certification, sealing, authentication, or code
compliance claim.

Browser fixture mode still intentionally refuses to publish solved rows for
edited models without the Tauri backend. A12 therefore lands as a
backend-backed rehearsal script and regression. The next unblocked item is A8:
automate this script as the GUI/e2e journey evidence backbone.
