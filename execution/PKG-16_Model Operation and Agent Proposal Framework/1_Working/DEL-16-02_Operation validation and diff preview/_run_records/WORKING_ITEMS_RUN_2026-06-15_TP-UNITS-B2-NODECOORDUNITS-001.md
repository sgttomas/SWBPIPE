---
run-id: WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-NODECOORDUNITS-001
timestamp: 2026-06-15T17:59:00-06:00
agent: WORKING_ITEMS
plan-item: B2/B3 unit-aware I/O
deliverable: DEL-16-02
status: SUCCESS
---

# WORKING_ITEMS_RUN TP-UNITS-B2-NODECOORDUNITS-001

## Scope

Added DEC-018-compatible unit entry for node coordinate edit intents in the
Property Inspector and normalized accepted entered length units back to the
model document's stored `project.units.length` basis in the operation applier.

This is a Phase B2/B3 units slice. It does not change lifecycle state, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance status.

## Changes

- `PropertyInspector` now marks node `position.x`, `position.y`, and
  `position.z` as unit-editable length fields.
- Node coordinate edit intents now emit an explicit `{ value, unit }` payload
  when authored through the unit-aware editor controls.
- `operation_applier` accepts project-unit quantity edits in either legacy
  scalar form or explicit `{ value, unit }` form.
- For bare project-unit fields, accepted DEC-018 units such as `ft` and `mm`
  are converted to the model's stored project length unit before writing the
  scalar coordinate value.
- Incompatible units, such as a length field carrying a stress unit, remain
  blocked with `OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE`.
- The shared operation contract corpus unit-mismatch case now uses an actually
  incompatible stress/length mismatch instead of `ksi`, which is accepted by
  the DEC-018 stress catalog.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check`
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  - 58 unit tests, canonical hash parity, and 66-case contract corpus passed.
- `npm run build:wasm --workspace apps/desktop`
- `npm test --workspace apps/desktop`
  - 18 files, 385 tests passed.
- `npm run build --workspace apps/desktop`
  - passed with the existing Vite chunk-size warning.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke covers solve, results, report, and viewport overlay"`
  - 2 Playwright projects passed (`chromium-desktop`, `chromium-compact`).

## Boundary Review

- Local-only application and test behavior; no cloud, daemon, telemetry, or
  network behavior added.
- No protected standards content, private project data, private rule-pack
  values, code-specific defaults, or bundled engineering allowables added.
- Node coordinates remain user-authored model data. Browser preview still does
  not synthesize a DEC-018 fallback catalog; it uses model metadata only.
- No release-readiness, professional approval, certification, sealing,
  authentication, or code-compliance claim created.

## Residuals

- B2 still has broader unit entry/picker work outside the already covered
  material/section/node-coordinate/primitive-load and rule-pack
  declaration/expression forms.
- B2/B3 still has import round-trip unit I/O and target-format conversion
  witnesses beyond disclosure.
- B3 still owns the larger mixed-unit round-trip, incompatible-unit rejection,
  and D-04/DEC-026 tolerance corpus outside the named operation-seam witnesses.
