# WORKING_ITEMS Run Record - TP-PMM-P3-TEMPINTERP-001

Date: 2026-07-15  
Agent: single bounded implementation agent  
Deliverable: DEL-05-02 - Load-case algebra engine  
Target stage: PRD R5  
Authority: D-38 Option O-B, codified as `DEC-077`

## Decomposition, Requirements, And Objective

The tranche decomposed into schema/model input, product-physics selection,
structured-operation authoring, desktop authoring, independent numerical
evidence, and deliverable-governance tasks. The accepted requirement is exactly
the `DEC-077` grant: interpolate user-entered E and alpha linearly between
adjacent temperature points; name both points and the method in provenance;
block rather than extrapolate at and beyond stored range edges; preserve exact
ID selection. No source authorizes temperature-indexed shear modulus.

The objective was to make that ruled path usable end to end without adding a
catalog, curve, code table, default, or authority claim.

## Method And Validation Criteria

- Normalize the explicit solve temperature as an absolute-temperature
  quantity and require exactly one of exact basis ID or solve temperature.
- Sort and validate user-entered points, require a unique strictly bracketing
  pair, and derive E/alpha with the declared linear formula only.
- Block duplicate temperatures, missing required values, exact-edge requests,
  and out-of-range requests; never extrapolate.
- Preserve exact-ID behavior and carry full basis provenance into combination
  records.
- Expose the quantity through the governed schema/operation/desktop seams with
  explicit temperature units and a selection-conflict diagnostic.
- Verify a midpoint independently in hand calculation and benchmark evidence,
  including its resulting fixed-fixed thermal force; verify exact, edge,
  outside, provenance, authoring, and mutual-selection cases.

## Implementation And Evaluation

- `schemas/model.schema.yaml` adds the optional unit-bearing solve-temperature
  slot and forbids simultaneous exact-ID and temperature selections;
  `schemas/material.schema.yaml` records the ruled point semantics and retires
  the resolved open-decision enum topic.
- `core/product_physics` implements the bracketing interpolation, blocking
  diagnostics, two-source provenance, and unchanged exact-ID path. Shear
  modulus remains the explicit base value.
- `core/model_operations/operation_applier` validates and authors the new
  quantity while rejecting mutual selection; the desktop tree, grid,
  inspector, and selected-property view expose the slot.
- The hand calculation and stress benchmark independently reproduce midpoint
  E, alpha, and thermal force. Focused product-physics, schema, benchmark,
  operation-applier, desktop component, desktop build, and real-browser
  authoring checks passed.

The evaluation satisfied the predeclared criteria for the authorized E/alpha
continuation. Temperature-indexed G remains owner-shaped and is routed to D-45
with `AWAITING_RULING` status.

## Verification Results

- Product physics: 74 tests passed.
- Operation applier: 74 unit tests plus 3 canonical/corpus tests passed.
- Stress benchmark: 23 tests passed.
- Project Python suite: 498 tests passed.
- Desktop Vitest: 476 tests passed across 23 files.
- Desktop Playwright: 20 tests passed across desktop and compact projects,
  including the explicit unit-bearing DEC-077 authoring slice.
- Desktop production build passed with the pre-existing Vite large-chunk
  warning.
- Practitioner-harness pytest: 266 tests passed; repo-wide harness self-check
  exited 0. Its reported cross-project review/warning inventory predates and is
  outside this bounded tranche.
- Rust formatting and `git diff --check` passed on the touched surfaces.
- The DEC-025 five-surface sweep passed on clean implementation commit
  `e2ed4d3471df38bca2371ed621ac53db19cb3fe6`; commit-bound summary:
  `validation/evidence/sweeps/SWEEP_20260716T035057Z_e2ed4d3471df.json`.

## Governance And Boundaries

This record is deliverable-local implementation evidence, not authoritative
decomposition truth. Its authority is the accepted `DEC-077` row and D-38
ruling packet. Generated validation-manual output is derivative of the
hand-calculation source and must be regenerated if that source or its renderer
changes. Solver evidence must be rerun after changes to the interpolation,
unit-normalization, material-point, operation, or authoring paths.

No lifecycle transition, issuance, release readiness, material-data authority,
professional approval, certification, sealing, authentication, or
code-compliance claim is made.
