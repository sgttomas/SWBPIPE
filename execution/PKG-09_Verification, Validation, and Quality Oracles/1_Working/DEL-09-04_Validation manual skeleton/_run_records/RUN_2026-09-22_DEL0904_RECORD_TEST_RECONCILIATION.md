---
run_id: RUN-2026-09-22-DEL0904-RECORD-TEST-RECONCILIATION
package_id: PKG-09
deliverable_id: DEL-09-04
status: CANDIDATE_FOR_REVIEW
branch: claude/valmanual-record-reconcile-20260922
base_sha: b3e2ce4ec74e01d6f393fc0bc069699bb079df91
prepared: 2026-09-22
executor: Claude Code (Claude Opus 5.5), untyped session with the owner
---

# DEL-09-04 — Reconcile case records with what their tests assert

## Brief

Owner direction in the session of 2026-09-22 (verbatim scope): reconcile the
validation-manual case records under `docs/validation_manual/cases/**` with what
the benchmark tests in `validation/benchmarks/*/src/lib.rs` actually assert, for
nine discrepancies found while turning the cases into public pages for
swbpipe.org. For each, decide with evidence whether to correct the record or
note wording, or strengthen the test so the record's claim becomes true. Do not
invent thresholds or tolerances. Deliver as a PR against `main`.

## Decisions

| # | Case | Finding (evidence) | Decision |
|---|---|---|---|
| 1 | `MECH-FIXED-FIXED-THERMAL-AXIAL` | `thermal_growth_fixture_records_open_axial_restraint_formula` compared the fixture's own `expected_values` with literals `0.0009` and `5.4`; no product code ran. | **Strengthen.** New `solve_fixed_fixed_thermal_restraint_force()` computes the restrained force through `prepare_straight_pipe_axial_effects` (the TP-PHYS-008 path) from the fixture inputs; the test asserts it equals the reference. The record now states that path. The strain stays reference arithmetic. |
| 2 | `MECH-PORTAL-SWAY-ORIGINAL` | `portal_frame_sway_fixture()` sets its expected value by calling `solve_portal_frame_sway()`, so the test compares the solve with itself. The note gives no hand value. | **Correct wording.** The record's reference row, expected-result and pass/fail text now say repeatability only; the note says so explicitly. An independent portal-frame derivation is out of scope for a reconciliation. |
| 3 | `MECH-CANTILEVER-TIP-FORCE` | The fixture records `fixed_end_moment_z = 60.0`; the test compared only the tip displacement. | **Strengthen.** New `solve_cantilever_tip_force_fixed_end_moment()` recovers the node 0 `RZ` reaction `K u - F` (the constant-effort fixture's recovery); the test asserts it equals `-60.0` (magnitude 60.0). The note records the sign from equilibrium. |
| 4 | `MECH-TP-PHYS-002-LINEAR-STATIC-INTEGRATION` | The note called `u_y` and the local shear "deterministic regression values"; the test asserted only `shear != 0`. | **Strengthen.** The note now derives `u_y,j = -4.0 / (3 E I_z / L^3 + k_s) = -0.020545746388443017` and `V_y,j = -3.178170144462279`; the test asserts both with the existing `INTERNAL_ASSERTION_EPSILON`. |
| 5 | `STRESS-TP-PHYS-015-CANONICAL-RESULTANT-STRESS-RECOVERY` | `recover_tp_phys_015_canonical_resultant_stress_fixture()` hard-codes `V_y = 4.0`, `M_z = 4.0`. `validate_tp_phys_014_canonical_analytical_payload()` asserts those midspan values from the solved payload. | **Correct wording.** Record and note state that this case checks stress recovery from fixed inputs and that TP-PHYS-014 checks the resultants. |
| 6 | `STRESS-TP-PMM-P3-MODULUSBASIS-RANGE-STRESS` | `F_hot` and the 400 K values are benchmark arithmetic; only `recover_stresses` and `recover_stress_range_with_modulus_basis` run. Product-side hot-point selection, interpolation provenance and edge blocking are asserted in `core/product_physics` tests with the same invented points. | **Correct wording.** Record and note say what the benchmark asserts and name the four product tests. The ambiguous "blocks requests at or beyond 300 K and 500 K" now reads: only strictly between the stored points; at or below 300 K or at or above 500 K blocks with `MODULUS_BASIS_UNRESOLVED` (as `interpolation_blocks_at_and_beyond_stored_range_edges` asserts for 250, 300, 500, 550 K). |
| 7 | `STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS` | `milltol_section_from_wall()` computes the section in the benchmark; product effective-wall derivation is tested in `core/product_physics`. | **Correct wording.** Record and note state it and name the four product tests. |
| 8 | Nonlinear convergence observations | `force_displacement_residual_observation_with_policy` multiplies `max_abs_translation_delta_from_previous` by 1000 and labels it `mm`, but the fixtures are already in fixture-local mm (`EA/L = 100` N/mm, gap `0.05` mm), so a `0.05` mm delta is reported as `50.0 mm`. The governed `DEC-046` delta records (seed and multi-support) carry the ×1000 figures as `mm` limits. The ledger row for friction slide (`100.0 mm`, `10.0 N`) is also stale against the measured `40.0` / `4.0 N`. | **Owner-held; no change.** Changing the scaling, the unit label or the limits alters protected `DEC-046` criteria. Raised in `execution/_Coordination/NOTICE_2026-09-22_DEC046_TRANSLATION_DELTA_UNIT.md`. No case record states these figures. |
| 9 | `STRESS-TP-PHYS-004-LOAD-TO-RESULTANT` | The stress benchmark enters the mechanics note's solved displacements and loads; the stress note only pointed to it. | **Correct wording.** The stress note lists the inputs it takes from the mechanics note; the record names both notes as the reference and states that the frame is solved in the mechanics case. |

No threshold, tolerance, governed record or protected criterion was changed.
New assertions reuse the existing `INTERNAL_ASSERTION_EPSILON = 1.0e-9`.

## Changed files

- `validation/benchmarks/mechanics/src/lib.rs` — two helpers, three strengthened tests.
- `validation/hand_calcs/mechanics/{cantilever_tip_force,fixed_fixed_thermal_axial,portal_frame_sway,tp_phys_002_linear_static_integration}.md`
- `validation/hand_calcs/stress/{tp_phys_004_load_to_resultant_stress,tp_phys_015_canonical_resultant_stress,tp_pmm_p3_milltol_effective_wall_stress,tp_pmm_p3_modulusbasis_range_stress}.md`
- `docs/validation_manual/cases/generate_validation_case_pages.py` — `reference_detail` and `software_path_detail` overrides; defaults render byte-identically (checked with `--check` before any case override).
- Eight regenerated case records (items 1–7, 9).

## Commands and results

Environment: macOS (Darwin 25.6.0), rustc 1.97.1 (8bab26f4f 2026-07-14),
cargo 1.97.1, Python 3; base `b3e2ce4ec74e01d6f393fc0bc069699bb079df91`.
From `projects/chirality-piping`:

| Command | Result |
|---|---|
| `cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml` | `ok. 41 passed; 0 failed; 0 ignored` |
| `cargo test --manifest-path validation/benchmarks/stress/Cargo.toml` | `ok. 23 passed; 0 failed; 0 ignored` |
| `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` | `ok. 19 passed; 0 failed; 0 ignored` |
| `cargo test --manifest-path core/product_physics/Cargo.toml -- <the eight cited tests>` | `ok. 8 passed; 0 failed` |
| Mutation check: hand `u_y` denominator 194.0, reaction sign flipped, `A = 3.1` in the thermal helper | the three strengthened tests failed (`38 passed; 3 failed`); restored source passes 41 |
| `python3 docs/validation_manual/cases/generate_validation_case_pages.py --check` | `checked 64 case page(s)`; no mismatch |

Repository checks for the candidate (piping-pytest, evidence sweep,
harness checks) and independent review are recorded in the PR.

## Boundary

Development verification evidence only (BS-VALID, DEC-081). No lifecycle
transition, release threshold, tolerance adoption, governed-record change,
professional, certification or code-compliance claim.
