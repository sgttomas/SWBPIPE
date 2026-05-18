---
doc_id: TP-MAC-09
doc_kind: implementation.plan
status: implemented
created: 2026-05-10
closed: 2026-05-10
---

# TP-MAC-09 Pressure Thrust Frame Preview

## Purpose

Add the first governed pressure-to-frame mechanics slice to the
product-preview workflow: explicit internal pressure on straight preview pipes
is converted into closed-end axial equivalent nodal loads.

TP-MAC-09 extends the closed TP-MAC-08 explicit mechanics-basis load
combination baseline while preserving the closed TP-MAC-07 midspan, TP-MAC-06
thermal, TP-MAC-05 endpoint stress-component, TP-RUN-01 runtime, and TP-PER-01
persistence baselines.

## Scope

- Support element-targeted straight-pipe pressure primitive loads with
  `dimension: "pressure"` and unit `Pa`.
- Compute internal area from explicit section geometry:
  `ID = OD - 2t` and `A_i = pi * ID^2 / 4`.
- Compute closed-end axial thrust as `N_p = pressure * A_i`.
- Apply equivalent local axial nodal loads to the frame vector:
  `-N_p` at end-i local UX and `+N_p` at end-j local UX, transformed through
  the existing straight-pipe orientation.
- Treat pressure load `direction` as legacy syntactic input for scalar
  pressure. It must not affect pressure thrust direction or sign.
- Remove the prior `PRESSURE_LOAD_NOT_APPLIED_TO_FRAME_VECTOR` warning for
  supported straight-pipe pressure loads.
- Preserve explicit diagnostics for unsupported pressure targets, invalid
  pressure inputs, or unsupported pressure-frame paths.
- Recover endpoint and midspan axial resultants from the pressure-thrust-active
  solve.
- Suppress `pressure_longitudinal_stress` rows when pressure thrust is active
  for that pipe/load case, because closed-end longitudinal pressure behavior is
  represented through axial-normal stress from frame resultants.
- Keep pressure hoop stress rows from the explicit pressure basis.
- Preserve open-formula stress summary rows for pressure-thrust cases after the
  longitudinal pressure membrane component is suppressed.
- Update invented fixtures, report/run-history refs, hashes, desktop gap
  ledger text, and tests.

## Boundaries

- No external pressure behavior.
- No open-ended pipe semantics or pressure-area overrides.
- No bends, components, expansion joints, Bourdon flexibility, or pressure
  load behavior outside straight preview pipes.
- No hydrotest special procedure beyond existing primitive-load representation.
- No pressure-to-code stress categories, allowables, SIF/flexibility tables,
  protected checks, private rule criteria, release claims, or professional
  acceptance.
- No desktop save/open UX, physical project containers, migrations, external
  storage, final CLI syntax, external execution, or broader report generation.
- Use invented or explicitly cleared data only.

## Diagnostics

Structured diagnostics are emitted for:

- invalid pressure units or non-finite pressure magnitudes;
- unsupported pressure target shapes;
- unsupported pressure-frame paths outside element-targeted straight preview
  pipes;
- stress recovery limitations after pressure thrust resultants are recovered.

Supported straight-pipe pressure thrust must not emit the prior
pressure-not-applied warning.

## Acceptance Criteria

- A valid invented preview solve with pressure primitive loads applies
  pressure thrust to the frame load vector.
- Displacements, support reactions, endpoint force rows, midspan force rows,
  endpoint axial-normal stress rows, midspan axial-normal stress rows, and
  explicit mechanics-basis combination rows update deterministically from
  pressure thrust.
- Pressure load `direction` does not affect pressure thrust magnitude or sign.
- Pressure hoop stress rows remain when explicit pressure basis exists.
- Pressure longitudinal stress rows are not emitted for pressure-thrust-active
  straight pipes.
- Open-formula stress summaries remain available for pressure-thrust cases
  without longitudinal pressure membrane double counting.
- The mechanics gap ledger marks only bounded closed-end pressure thrust frame
  preview implemented while broader pressure behavior remains deferred.
- Existing TP-MAC-08, TP-MAC-07, TP-MAC-06, TP-MAC-05, TP-RUN-01, and
  TP-PER-01 baselines continue to pass focused checks.

## Verification

- `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `npm run generate:product-preview-mechanics`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py tests/test_project_persistence_service.py`
- `python3 tests/test_results_schema.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- Browser smoke of the desktop preview pressure gap ledger, pressure-thrust
  results, pressure hoop rows, suppressed pressure-longitudinal rows, and
  report refs at `http://127.0.0.1:5173/`
- `git diff --check`

## Assumptions

- Pressure thrust is limited to straight preview pipes with explicit internal
  pressure and explicit section geometry.
- Existing pressure hoop stress remains an inspectable open-mechanics
  component.
- Closed-end longitudinal pressure behavior is represented by axial-normal
  stress from pressure thrust for thrust-active cases.
- Public examples remain invented or cleared.

## Closeout

TP-MAC-09 is implemented for the current product-preview workflow.

Delivered:

- closed-end pressure thrust equivalent nodal loads for element-targeted
  straight preview pipes with explicit pressure basis inputs;
- internal pressure area from explicit section geometry;
- pressure load `direction` retained as legacy syntax that does not affect
  pressure thrust magnitude or sign;
- endpoint and midspan axial resultants recovered from the
  pressure-thrust-active solve;
- pressure hoop stress rows retained from the explicit pressure basis;
- pressure-longitudinal membrane rows suppressed when closed-end pressure
  thrust is represented through axial-normal stress from frame resultants;
- open-formula stress summaries retained without pressure-longitudinal membrane
  double counting;
- generated mechanics and persistence fixture updates;
- desktop gap ledger and smoke coverage for the bounded pressure thrust slice.

Final verification was run on 2026-05-10:

- `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `npm run generate:product-preview-mechanics`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py tests/test_project_persistence_service.py`
- `python3 tests/test_results_schema.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- browser smoke at `http://127.0.0.1:5173/`
- `git diff --check`

External pressure, open-ended pipe semantics, pressure-area overrides,
bends/components, expansion joints, Bourdon flexibility, hydrotest special
procedures, pressure-to-code stress categories, protected rule/code checks,
release/professional claims, and professional acceptance remain deferred.
