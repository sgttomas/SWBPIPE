# WORKING_ITEMS Run Record - TP-PMM-P3-MODULUSBASIS-001

Date: 2026-07-10
Agent: WORKING_ITEMS (bounded Type-2 implementation worker; owner-directed session)
Deliverable: DEL-05-02 - Load-case algebra engine
Package: PKG-05 - Loads, Load Cases, and Stress Recovery
Tranche: TP-PMM-P3-MODULUSBASIS-001
Target stage: R5 / physical-model mechanics program P3 tranche (a)
(`plans/PLAN_2026-07-09_physical_model_mechanics.md`; ruling `DEC-068`,
D-36 item 1)

## Scope

Third bounded slice of the D-36 P3 workflow-physics set: a load case may
name which user-entered temperature-dependent material property values
(E, α) it solves with, and expansion-range combinations record their
modulus basis explicitly — giving the `schemas/material.schema.yaml`
temperature-dependent slots solve semantics.

## Files Touched

- `core/product_physics/src/lib.rs` (`MaterialInput.temperature_points`
  user-entered temperature-indexed point inputs;
  `PreviewLoadCase.modulus_basis_ref`; DEC-018 unit normalization for
  point quantities; `materials_for_modulus_basis` exact resolution with
  blocking `MODULUS_BASIS_UNRESOLVED` / `MODULUS_BASIS_INPUT_MISSING` /
  `MODULUS_BASIS_INPUT_INVALID`; per-distinct-basis model build and
  stiffness assembly cached across load cases; `solve_load_case` consumes
  basis-resolved materials for stiffness, thermal loads, and stress
  recovery E-selection; `modulus_basis_record` evidence row per solving
  load case; `combination_modulus_basis_record` rows for every operand of
  result-state-subtraction and range-envelope combinations; both new
  record kinds excluded from combination scalar algebra)
- `core/loads/stress_recovery/src/lib.rs`
  (`StressRangeModulusBasisRecord`,
  `recover_stress_range_with_modulus_basis`, and the
  `StressRangeResult.modulus_basis` verbatim record slot; empty-label
  rejection via `StressRecoveryError::MissingModulusBasisRef`)
- `schemas/model.schema.yaml` (`LoadCase.modulus_basis_ref` with
  exact-selection description; `Combination.modulus_basis_records` +
  `ModulusBasisRecord` def with the reserved `material_base_values`
  label)
- `schemas/material.schema.yaml`
  (`MaterialPropertyValue.temperature_ref` description now states the
  solve semantics: exact match, blocking on unmatched basis, no
  interpolation while D-38 is unruled)
- `tests/test_material_schema.py`, `tests/test_model_schema.py`,
  `tests/test_physical_to_analytical_transform.py`
- `validation/hand_calcs/stress/tp_pmm_p3_modulusbasis_range_stress.md`
  (witness, hot-solve/cold-eval with basis recorded) and
  `validation/hand_calcs/stress/README.md`
- `validation/benchmarks/stress/{src/lib.rs, README.md}` (fixture
  `STRESS-TP-PMM-P3-MODULUSBASIS-RANGE-STRESS`, family registration,
  inventory count, tests)
- `execution/_Coordination/_DECISIONS/D-38_temperature_interpolation_policy.md`
  (new PROPOSAL packet) and `_REGISTER.md` (AWAITING_RULING row)

## Recorded Design Choices

- **Exact basis selection only.** A load case's `modulus_basis_ref` must
  match a stored user-entered temperature-point id on every material used
  by a pipe segment; an unmatched reference or a point without an elastic
  modulus is blocking. No interpolation between stored points is
  performed. Interpolation was judged a genuine method fork, so
  **D-38 was drafted as a PROPOSAL packet** (options O-A exact-only /
  O-B declared linear interpolation, recommended non-bindingly /
  O-C defer) with an AWAITING_RULING register row — not ruled.
- The point's α, when supplied, replaces the base value for that load
  case; when the point supplies no α, no base-value substitute is mixed
  in (thermal consumption then behaves as α-absent).
- Shear modulus stays the user-entered base value; a temperature-indexed
  shear-modulus slot is a recorded residual outside the (E, α) grant.
- Each distinct named basis gets its own built model and assembled global
  stiffness (per-load-case E-correct solve); the base (`None`) basis
  keeps the existing single-assembly path byte-for-byte (all 66
  pre-existing product tests unchanged).
- Range records are declarations of the solved basis, not selectors; the
  reserved label `material_base_values` marks operands that solved on
  base values.

## Implemented Evidence

- Product tests: hot-point selection changes the fixed-fixed thermal
  axial resultant to `E_hot * A * alpha_hot * dT` exactly; base-values
  path unchanged when no basis is named; unresolved basis and
  E-less point are blocking with no-interpolation wording; range
  combinations emit one basis record per operand with correct labels
  (5 new tests).
- Stress-recovery tests: the with-basis range equals the plain range and
  carries the record verbatim; empty labels are rejected (2 new tests).
- Benchmark `STRESS-TP-PMM-P3-MODULUSBASIS-RANGE-STRESS` reproduces the
  witness hot stress and range at the 1.0e-9 internal epsilon with both
  state bases recorded. Tolerance posture per `DEC-024`/`DEC-026`
  (governed values `TBD`; tighten-only).

## Checks

- `cargo test` `core/product_physics`: 71 passed (66 + 5 new), including
  the dense/sparse parity rows (`dense_scrutiny_mode_keeps_sparse_parity_row`
  green — solve path touched, oracle green).
- `cargo test` `core/loads/stress_recovery`: 26 passed (24 + 2 new).
- `cargo test` `validation/benchmarks/stress`: 22 passed (21 + 1 new).
- `python3 -m pytest -q tests`: 377 passed (376 + 1 new).
- `cargo fmt --check` clean on touched crates.
- Repo-wide harness `self-check` exit 0.
- `DEC-025` five-surface sweep recorded on the clean head with this
  tranche set (commit-bound summary under `validation/evidence/sweeps/`).

## Boundaries And Residuals

- Interpolation between temperature points: **not implemented**; D-38
  PROPOSAL drafted, AWAITING_RULING. Exact selection is the shipped
  conservative floor.
- Temperature-indexed shear modulus: recorded residual (base value used
  under any basis).
- `core/loads/load_case_algebra` needed no change: combination scalar
  algebra is basis-agnostic; the explicit records ride as excluded
  evidence rows in the result envelope.
- The canonical-schema `modulus_basis_records` slot on `Combination` is a
  recording surface; GUI/editors do not yet emit it.
- Schema files carry no literal version constant to bump (instance
  `schema_version` remains `0.1.0`); recorded as an
  instructions-vs-live-tree delta.
- All temperature-dependent values are user-entered; no catalog, curve,
  code table, or default ships. No lifecycle transition, no
  release-readiness, professional, certification, or code-compliance
  claim.
