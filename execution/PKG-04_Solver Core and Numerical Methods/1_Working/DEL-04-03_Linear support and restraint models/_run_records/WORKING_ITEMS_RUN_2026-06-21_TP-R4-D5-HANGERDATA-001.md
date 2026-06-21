---
doc_id: WORKING-ITEMS-RUN-2026-06-21-TP-R4-D5-HANGERDATA-001-DEL-04-03
doc_kind: working_items.run_record
status: draft
created: 2026-06-21
agent: WORKING_ITEMS
package_id: PKG-04
deliverable_id: DEL-04-03
tranche_id: TP-R4-D5-HANGERDATA-001
decision_refs: [DEC-049]
---

# WORKING_ITEMS Run - D5 Spring-Hanger User Data

## Scope

Bounded R4 Phase-D D5 implementation under `DEC-049`: add a minimal
dedicated user-entered spring-hanger model for variable-rate and
constant-effort support records. Generic `spring` support behavior alone is
not treated as satisfying D5.

## Implementation Evidence

- Added a named `hanger` payload to support records with explicit
  `hanger_type`, user-entered stiffness/load/travel fields, source reference,
  manufacturer/reference notes, load-side review notes, and mechanics
  consumption posture.
- Added invented fixture records:
  `support:SH-140` (`variable_spring_hanger`) and `support:CE-120`
  (`constant_effort_support`). Both are invented/user-entered examples and
  carry no catalog, protected, default, code-compliance, or professional
  reliance claim.
- `core/product_physics` validates hanger units and required fields before
  solve. Missing variable-hanger stiffness blocks with
  `SPRING_HANGER_STIFFNESS_MISSING` instead of applying a hidden default.
- The variable-rate hanger reuses the existing linear spring primitive only
  for the explicit user-entered stiffness. Constant-effort support values are
  emitted as review/load-side evidence only; no global constant-effort load or
  nonlinear behavior is claimed in this tranche.
- Mechanics results now emit `spring_hanger_user_input_review` and
  `constant_effort_user_input_review` rows plus
  `spring_hanger_user_input_count`.
- Desktop model tree, selected-property details, report packet, rendered
  report input, project validation, native-package unit witnesses, and
  regression fixtures expose the hanger fields and provenance.

## Validation

- `python3 -m json.tool fixtures/product_preview/invented_preview_model.json`
  passed.
- `npm run generate:product-preview-mechanics` regenerated
  `fixtures/product_preview/invented_mechanics_result.json`.
- `python3 -m json.tool fixtures/product_preview/invented_mechanics_result.json`
  passed.
- `cargo fmt --manifest-path core/product_physics/Cargo.toml` passed.
- `cargo test --manifest-path core/product_physics/Cargo.toml` passed:
  43/43 Rust tests.
- `npm test --workspace apps/desktop` passed: 19/19 files, 407/407 tests.
- From `apps/desktop`,
  `npm exec -- playwright test e2e/r2-smoke.spec.ts --workers=1` passed:
  18/18 Playwright checks after refreshing fixture-count expectations for the
  800-row D5 mechanics result surface.
- `npm run build:desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning.
- Full DEC-025 evidence sweep passed:
  `validation/evidence/sweeps/SWEEP_20260621T202442Z_4829dea6c2e0-dirty.json`
  (`overall_status=pass`, 5/5 surfaces).

## Boundaries

This tranche does not introduce catalog sizing, protected standards or vendor
data, hidden spring/support defaults, constant-effort global load application,
new nonlinear spring-hanger solve behavior, sparse live-path adoption,
threshold policy, release readiness, professional approval, certification,
sealing, authentication, or code-compliance claims.

## Residual

D5's minimal user-entered data model is landed. Deeper constant-effort solve
behavior, catalog integrations, governed validation thresholds, and sparse
live-path evidence remain outside this tranche. The next unblocked R4
dependency-spine item is D7 sparse live-path evidence-lane adoption under
`DEC-050`.
