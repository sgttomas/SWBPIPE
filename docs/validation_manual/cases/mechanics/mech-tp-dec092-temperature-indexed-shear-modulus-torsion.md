---
doc_id: OPS-VALIDATION-MANUAL-CASE-MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION
doc_kind: governance.validation_manual_case
status: draft_evidence
created: 2026-08-09
generated_by: docs/validation_manual/cases/generate_validation_case_pages.py
refs:
  - rel: implements
    to: DEL-09-04
  - rel: evidences
    to: DEL-09-01
---

# MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION

| Item | Record |
|---|---|
| Suite | Mechanics benchmark suite (`DEL-09-01`), crate `validation/benchmarks/mechanics/` |
| Evidence class | Mechanics verification |
| Evidence state | `DRAFT_EVIDENCE` |
| Independent reference | `validation/hand_calcs/mechanics/tp_dec092_temperature_indexed_shear_modulus_torsion.md` |
| Provenance | Invented public-original fixture data only (recorded in the reference note and fixture metadata) |

## Test Purpose And Problem Statement

Verify D-45 Option B / `DEC-092` temperature-indexed shear modulus with an invented hollow straight-pipe cantilever in pure torsion. Exact-ID and adjacent-interpolation selected `G` values are compared with an independent elementary-mechanics oracle while a distinct base `G` witnesses forbidden fallback.

All inputs are invented or user-entered fixture data; nothing is copied from
protected standards, commercial software examples, or proprietary data.

## Input Model

The machine-readable input model is the fixture constructor `tp_dec092_temperature_indexed_shear_modulus_torsion_fixture()` (fixture id `MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION`) in `validation/benchmarks/mechanics/src/lib.rs`, carrying explicit fixture-local units and public-original provenance metadata. The reference note records the same invented inputs with the longhand derivation.

## Expected Result And Independent Reference

The independent note and mechanics fixture record `J = 1.0540043352793751e-5 m^4`, exact-point `G = 50.0e9 Pa`, and strictly adjacent interpolation between `TP-LOW` (`300 K`, `60.0e9 Pa`) and `TP-HIGH` (`500 K`, `40.0e9 Pa`) at `425 K`, giving `f = 0.625` and `G = 47.5e9 Pa`. The resulting exact-ID and interpolated tip rotations are `9.108121929551094e-2 rad` and `9.587496767948521e-2 rad`. The deliberately distinct base value `G_base = 80.0e9 Pa` gives `5.6925762059694338e-2 rad`; substituting it misses the selected targets by `37.5%` and `40.625%`.

## Software Result And Reproduction

The measured-vs-reference comparison executes inside the named suite
test(s) `temperature_indexed_shear_modulus_torsion_matches_independent_oracle`, which run the current in-repo solver path on the fixture
and assert agreement with the reference expectations.

Product-physics tests separately prove that exact-ID selection consumes the selected point's explicit `G`; temperature selection uses the strictly adjacent bracket and records both source IDs plus `method=linear_temperature_interpolation`; and combination records carry the same basis. They also prove that selection blocks at or outside stored range endpoints without extrapolation, blocks missing, non-positive, non-finite, or dimensionally wrong selected-point `G`, rejects duplicate temperatures and conflicting exact/temperature selectors, and never falls back to base `G`. A load case with no modulus basis continues to consume the explicit base `G`. The mechanics test is an independent numeric oracle; it does not by itself prove product selection behavior.

Reproduction (from `projects/chirality-piping`):

```bash
cargo test --offline --manifest-path validation/benchmarks/mechanics/Cargo.toml temperature_indexed_shear_modulus_torsion_matches_independent_oracle
cargo test --offline --manifest-path core/product_physics/Cargo.toml dec092_fixture_consumes_exact_and_interpolated_g_with_provenance_and_combination_carry_through
cargo test --offline --manifest-path core/product_physics/Cargo.toml selected_point_g_is_base_independent_and_selected_g_sensitive
cargo test --offline --manifest-path core/product_physics/Cargo.toml selected_basis_blocks_missing_invalid_or_dimensionally_wrong_point_g
cargo test --offline --manifest-path core/product_physics/Cargo.toml temperature_g_interpolation_uses_adjacent_points_and_duplicate_temperatures_still_block
cargo test --offline --manifest-path core/product_physics/Cargo.toml interpolation_blocks_at_and_beyond_stored_range_edges
cargo test --offline --manifest-path core/product_physics/Cargo.toml exact_and_interpolated_basis_fields_are_mutually_exclusive
cargo test --offline --manifest-path core/product_physics/Cargo.toml base_material_values_are_used_when_no_modulus_basis_is_named
```

Recorded run: 2026-08-03, toolchain rustc 1.97.1 / cargo 1.97.1; suite
result `PASS; producing mechanics suite 39 passed with zero failures, and the commit-bound sweep records Cargo 36/36 and overall pass` with the named test(s) passing. The full
suite output and the exact commit are recorded in the run record
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-08-02_DEC092_TEMPERATURE_G_IMPLEMENTATION.md`.

Evidence basis: Implementation commit `c394365ca72b8383c7d7203ce5be2cb9ea67d508`; passing commit-bound sweep `validation/evidence/sweeps/SWEEP_20260803T194132Z_c394365ca72b.json`; mechanics fixture/oracle and named test in `validation/benchmarks/mechanics/src/lib.rs`; product behavior and the seven named tests in `core/product_physics/src/lib.rs`. Receipt-87 records closeout and routes this derivative, but is not used as the sole basis for any mechanics or product claim.

Reproduction is through the listed locked/offline crate tests. No GUI or external-runner reproduction was executed for this case.

## Tolerance

The independent mechanics comparison reuses the existing `DEC-024`/`DEC-026` analytic-class relative tier `1.0e-9`. The fixture's `tolerance_policy` remains unresolved (`None`): this case selects no new governed tolerance, release threshold, CI gate, publication scope, external-validation claim, or reliance basis.

## Pass/Fail

`PASS` as development evidence at the producing run and commit-bound sweep: the independent mechanics oracle and the product behavior tests passed. Evidence state remains `DRAFT_EVIDENCE`; public tolerances, maintainer review, GUI validation, release, and professional reliance remain open.

## Solver Version

The in-repo OpenPipeStress solver and benchmark crates at implementation commit `c394365ca72b8383c7d7203ce5be2cb9ea67d508`, bound by the passing sweep above; toolchain rustc 1.97.1 / cargo 1.97.1. No packaged release version exists; release labels remain `TBD`.

## Boundary

This case page documents software verification evidence for an invented
public-original fixture. It is development verification and screening
evidence (claims registry BS-VALID, DEC-081), and it does not settle any
`TBD` threshold or governed tolerance decision.
