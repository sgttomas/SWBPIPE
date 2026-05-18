---
doc_id: TP-MAC-07
doc_kind: implementation.plan
status: implemented
created: 2026-05-10
closed: 2026-05-10
---

# TP-MAC-07 Midspan Station Preview

## Purpose

Add the first governed intermediate station recovery slice to the
product-preview workflow: one deterministic `midspan` station for each solved
straight-pipe preview element.

TP-MAC-06 is the closed uniform axial thermal baseline, TP-MAC-05 is the
closed endpoint stress-component baseline, TP-RUN-01 is the closed runtime
baseline, and TP-PER-01 is the closed persistence baseline.

## Scope

- Emit one midspan station per solved straight-pipe preview element.
- Add midspan force and moment rows:
  - `result:force:{pipe}:midspan:axial`
  - `result:moment:{pipe}:midspan:torsion`
  - `result:moment:{pipe}:midspan:bending-y`
  - `result:moment:{pipe}:midspan:bending-z`
- Add midspan stress rows:
  - `result:stress:{pipe}:midspan:axial-normal`
  - `result:stress:{pipe}:midspan:bending-normal-y`
  - `result:stress:{pipe}:midspan:bending-normal-z`
  - `result:stress:{pipe}:midspan:torsional-shear`
  - pressure membrane midspan rows only when explicit pressure basis exists.
- Compute midspan force and moment resultants by linear interpolation from
  TP-MAC-06 corrected endpoint local resultants.
- Label midspan force and stress metadata basis as
  `interpolated_from_endpoint_resultants`.
- Feed midspan resultants through the existing open-mechanics stress recovery
  path.
- Preserve endpoint result IDs, TP-MAC-06 thermal behavior, runtime interfaces,
  and persistence service boundaries.
- Update invented fixtures, generated mechanics results, analysis-run/report
  selected refs, persistence run-history refs, and hashes only as needed for
  explicit midspan output.
- Update the desktop mechanics gap ledger so midspan station preview recovery
  is implemented while broader station sweeps remain deferred.

## Boundaries

- No arbitrary station arrays or user-defined station grids.
- No exact internal force diagrams for distributed loads.
- No shear force recovery.
- No pressure-to-frame load conversion.
- No equivalent or principal stress.
- No load-case algebra, rule/code checks, allowables, owner criteria, private
  rule packs, release claims, or professional acceptance workflows.
- Use invented or explicitly cleared data only.

## Public Interfaces

- Result metadata basis enum includes
  `interpolated_from_endpoint_resultants`.
- Existing result metadata location `midspan` is used for the station rows.
- Existing endpoint IDs remain unchanged.
- Existing runtime request and persistence service APIs remain unchanged.

## Acceptance Criteria

- A valid invented preview solve emits midspan force, moment, and stress rows
  for each solved straight pipe.
- Midspan pressure membrane rows are emitted only when explicit pressure basis
  exists.
- Midspan rows use `location: "midspan"` and
  `basis: "interpolated_from_endpoint_resultants"`.
- Desktop result rows render and select midspan results normally.
- Endpoint-pair display remains endpoint-only and does not pair midspan rows.
- The mechanics gap ledger marks station recovery implemented for midspan
  preview only and keeps arbitrary station sweeps deferred.
- Existing TP-MAC-06 thermal behavior and endpoint result IDs continue to pass
  focused checks.

## Verification

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `npm run generate:product-preview-mechanics`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py tests/test_project_persistence_service.py`
- `python3 tests/test_results_schema.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `git diff --check`

## Closeout

TP-MAC-07 is implemented for the current product-preview workflow.

Delivered:

- deterministic midspan force and moment rows for solved straight pipes;
- deterministic midspan stress rows through existing open-mechanics stress
  recovery;
- pressure membrane midspan rows only when explicit pressure basis exists;
- result metadata basis `interpolated_from_endpoint_resultants`;
- generated mechanics fixture updates and run-history/report selected-ref
  coverage for a midspan result;
- desktop result selection for midspan rows without endpoint-pair display;
- mechanics gap ledger entry marking midspan station preview recovery
  implemented while arbitrary station sweeps and exact internal diagrams remain
  deferred.

The desktop build retained the existing Vite chunk-size warning. Arbitrary
station grids, exact distributed-load diagrams, shear force recovery,
pressure-to-frame conversion, equivalent/principal stress, protected rule/code
checks, release/professional claims, and professional acceptance remain
deferred.
