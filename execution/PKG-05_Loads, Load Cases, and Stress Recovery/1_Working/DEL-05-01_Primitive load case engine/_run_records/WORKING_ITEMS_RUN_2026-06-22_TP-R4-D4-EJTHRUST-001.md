# WORKING_ITEMS Run Record - TP-R4-D4-EJTHRUST-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Deliverable: DEL-05-01 - Primitive load case engine
Package: PKG-05 - Loads, Load Cases, and Stress Recovery
Tranche: TP-R4-D4-EJTHRUST-001
Target stage: R4 / Phase D

## Scope

Supporting load-side record for the DEL-03-06 D4 expansion-joint
pressure-thrust tranche. The preview mechanics engine now treats an eligible
expansion-joint effective area as the pressure-thrust area for pressure loads
on the mapped pipe. Ordinary pressure loads without that component mapping
continue to use the existing pipe-internal-area fallback.

## Implemented Evidence

- Added explicit EJ-mapped pressure primitives for `L-100` and `L-200` in the
  invented preview model.
- Added component pressure-thrust result rows for the primary load cases and
  the `C-OPER-ALT` combination, with source load ids and load-case refs.
- Added tests that verify the values are generated from `pressure *
  effective_area` and that the combination row uses the existing load-case
  algebra.

## Validation

- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed
  44/44 tests.
- `python3 -m pytest tests/product_preview/test_product_preview_service.py` -
  passed 9/9 tests.
- `npm test --workspace apps/desktop` - passed 19/19 files and 407/407 tests.
- Full DEC-025 evidence sweep - passed 5/5 surfaces:
  `validation/evidence/sweeps/SWEEP_20260622T063558Z_d3f658288543-dirty.json`.

## Boundary

No protected standards content, hidden pressure-thrust default, private project
data, lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

## Residual

This supporting load record covers the invented preview pressure-thrust path
only. Broader R4 validation and final exit-chain evidence remain open.
