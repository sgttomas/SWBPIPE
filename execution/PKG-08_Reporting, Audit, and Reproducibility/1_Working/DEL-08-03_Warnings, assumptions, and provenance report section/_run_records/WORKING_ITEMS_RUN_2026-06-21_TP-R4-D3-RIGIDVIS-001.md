# WORKING_ITEMS Run Record - TP-R4-D3-RIGIDVIS-001

Date: 2026-06-21
Agent: WORKING_ITEMS
Deliverable: DEL-08-03 - Warnings, assumptions, and provenance report section
Package: PKG-08 - Reporting, Audit, and Reproducibility
Tranche: TP-R4-D3-RIGIDVIS-001
Target stage: R4 / Phase D

## Scope

Landed the report-provenance evidence side of the D3 rigid/semi-rigid component
app-absorption slice under `DEC-045`. The preview report packet and JSON export
now preserve rigid component mapping, geometry/source values, weight/COG
evidence, user-entered stiffness/scaling evidence, and solver-consumption
metadata for the invented rigid component path.

## Implemented Evidence

- Extended report component provenance records with `rigid_pipe_ref`,
  rigid body length, end sizes, weight, center of gravity, connection
  references, stiffness behavior/source references, stiffness scaling, linear
  stiffness, and rotational stiffness.
- Added `DEL-03-05` to report export deliverable references for rigid component
  provenance evidence.
- Kept report boundary flags at no-private-payload, no-protected-content, and
  no release/professional/code-compliance claim.

## Validation

- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed 33/33
  unit tests.
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_results_schema.py tests/test_analysis_run_records.py` -
  passed 20/20 tests.
- `npm test --workspace apps/desktop -- App.test.tsx` - passed 57/57 tests.
- `npm test --workspace apps/desktop` - passed 19/19 files, 406/406 tests.
- `npm run build:desktop` - passed; retained the existing Vite chunk-size
  warning.
- `npm run test:e2e:desktop` - passed 18/18 Playwright tests.

## Boundary

The report remains a technical preview artifact and makes no release,
professional approval, certification, sealing, authentication, or
code-compliance claim. No protected standards content, private project data,
or proprietary catalog data was introduced.

## Residual

Rigid component provenance appears in the preview report packet for the
invented model. Broader D8 report closure and D9 R4 validation evidence remain
open.
