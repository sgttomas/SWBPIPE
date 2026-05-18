---
doc_id: TP-MAC-10
doc_kind: implementation.plan
status: implemented
created: 2026-05-10
closed: 2026-05-10
---

# TP-MAC-10 Station Grid Result Recovery Preview

## Purpose

Expand product-preview station recovery from the current midspan-only slice
into a deterministic fixed station grid for solved straight preview pipes.

TP-MAC-10 extends the closed TP-MAC-09 pressure thrust frame preview baseline
while preserving the closed TP-MAC-08 load-combination, TP-MAC-07 midspan,
TP-MAC-06 thermal, TP-MAC-05 endpoint stress-component, TP-RUN-01 runtime, and
TP-PER-01 persistence baselines.

## Scope

- Recover a fixed straight-pipe station grid at `end_i`, `quarter_1`,
  `midspan`, `quarter_3`, and `end_j`.
- Preserve existing endpoint and midspan result IDs.
- Do not emit duplicate endpoint station-prefixed rows.
- Add quarter-station local force rows for axial, shear-y, and shear-z
  resultants.
- Add quarter-station local moment rows for torsion, bending-y, and bending-z.
- Add endpoint and midspan local shear force resultant rows.
- Compute station-grid resultants by linear interpolation from TP-MAC-09
  corrected endpoint local force vectors, including thermal fixed-end and
  pressure thrust corrections.
- Add metadata components `shear_force_y` and `shear_force_z`.
- Add metadata locations `quarter_1` and `quarter_3`.
- Recover quarter-station open-mechanics stress rows from interpolated axial,
  bending, torsion, and pressure basis inputs.
- Keep shear stress recovery deferred.
- Keep TP-MAC-09 pressure behavior: pressure hoop rows remain, pressure
  longitudinal rows remain suppressed for pressure-thrust-active cases, and
  open-formula summaries avoid longitudinal pressure double counting.
- Include quarter-station stress values in the open-formula summary maximum.
- Preserve TP-MAC-08 load-case and explicit-combination row behavior for all
  matching new scalar rows.
- Update invented fixtures, persistence refs, report selected refs, hashes,
  desktop gap ledger text, and tests.

## Boundaries

- No request schema change and no arbitrary station input.
- No exact internal force diagrams.
- No station envelopes.
- No shear stress recovery.
- No equivalent or principal stress.
- No code/rule combinations, allowables, protected rule checks, SIF/flexibility
  behavior, release claims, or professional acceptance.
- No desktop save/open UX, physical project containers, migrations, external
  storage, final CLI syntax, external execution, or broader report generation.
- Use invented or explicitly cleared data only.

## Diagnostics

TP-MAC-10 should preserve existing diagnostics for unsupported or incomplete
mechanics paths. It does not add a new user-facing diagnostic category unless a
station-grid row cannot be recovered after endpoint force recovery succeeds.

Station-grid rows must be labeled as interpolated preview outputs, not exact
internal diagrams.

## Acceptance Criteria

- Valid invented preview solves emit fixed station-grid rows for solved
  straight-pipe elements.
- Existing endpoint and midspan result IDs are preserved.
- Quarter-station force and moment rows use the planned IDs and carry
  `location: "quarter_1"` or `location: "quarter_3"`.
- Endpoint, midspan, and quarter-station shear force resultant rows are emitted
  as force rows with local shear metadata.
- No shear stress rows are emitted.
- Quarter-station stress rows are emitted from interpolated axial, bending,
  torsion, and pressure basis inputs.
- Pressure hoop rows remain present and pressure-longitudinal rows remain
  absent for pressure-thrust-active cases.
- Open-formula stress summaries include quarter-station stress values without
  pressure-longitudinal double counting.
- Explicit load-case and combination rows include matching station-grid scalar
  rows with correct `basis_ref` and `source_result_refs`.
- Desktop gap ledger describes fixed station-grid recovery while keeping exact
  diagrams and arbitrary station sweeps deferred.
- Existing TP-MAC-09, TP-MAC-08, TP-MAC-07, TP-MAC-06, TP-MAC-05, TP-RUN-01,
  and TP-PER-01 baselines continue to pass focused checks.

## Verification

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/reporting/result_export/Cargo.toml`
- `npm run generate:product-preview-mechanics`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py tests/test_project_persistence_service.py`
- `python3 tests/test_results_schema.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- Browser smoke of the desktop preview fixed station-grid gap ledger, quarter
  rows, shear force rows, endpoint-pair display, pressure hoop rows, suppressed
  pressure-longitudinal rows, report refs, and hashes at
  `http://127.0.0.1:5173/`
- `git diff --check`

## Assumptions

- The station grid is fixed at fractions `0.0`, `0.25`, `0.5`, `0.75`, and
  `1.0`.
- Existing endpoint rows represent `0.0` and `1.0`; existing midspan rows
  represent `0.5`.
- Interpolated station rows are open mechanics preview rows.
- Shear force resultants are exposed only as local force components.
- Public examples remain invented or cleared.

## Closeout

TP-MAC-10 is implemented for the current product-preview workflow.

Delivered:

- fixed station-grid force, moment, and stress rows for solved straight
  preview pipes;
- local shear force resultant rows at endpoints, midspan, and quarter stations;
- metadata components `shear_force_y` and `shear_force_z`;
- metadata locations `quarter_1` and `quarter_3`;
- quarter-station stress recovery from interpolated axial, bending, torsion,
  and pressure basis inputs;
- open-formula stress summaries that include quarter-station stress values;
- TP-MAC-08 load-case and explicit-combination propagation for matching
  station-grid scalar rows;
- generated mechanics and persistence fixture updates;
- desktop gap ledger, report refs, and smoke coverage for fixed station-grid
  recovery.

Final verification was run on 2026-05-10:

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/reporting/result_export/Cargo.toml`
- `npm run generate:product-preview-mechanics`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py tests/test_project_persistence_service.py`
- `python3 tests/test_results_schema.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- browser smoke at `http://127.0.0.1:5173/`
- `git diff --check`

Arbitrary station input, exact internal force diagrams, station envelopes,
shear stress recovery, equivalent/principal stress, code/rule checks,
allowables, SIF/flexibility behavior, release claims, and professional
acceptance remain deferred.
