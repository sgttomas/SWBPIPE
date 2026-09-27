# I4 return: S11-F (exact load sums, facade side)

**Author.** Type 2 TASK I4 (Claude). This is not owner review, and it has not had independent review; RV3 is next.

**Worktree.** `<wt>` on `codex/piping-s11f-20260927`, base `origin/main` `3488a236a`.
- The records commit is `c99e322d6` (the manager's).
- The implementation and the regeneration are uncommitted; the manager commits them.
- A merge-forward to current main follows, and gets its own addendum below.

**Brief.** `T3/TASK_BRIEFS/I4_S11F_IMPLEMENTATION.md`, with addendum 1 (boundary decisions) and addendum 2 (F12/formation, `db665f2cb`).

**Rulings used.** From `T3/ROOT_RULINGS_V1.md`:
- the F12 option (c) ruling;
- the S11-F regeneration pre-approval (`1397e8c43`);
- the formation-row amendment (`42b300344`).

**Records.**
- `CHANGE_RECORD.md` (the PR body draft);
- `PRE_REGENERATION_REPORT.md`;
- `_run_records/` (with `SHA256SUMS`).

Machine paths are written as `<wt>`, `<scratch>`, `<s11f-target>`, `<dec025-venv>`, `<node24>` and `<engine-worktree>`.

## 1. Files changed

| File | Lines |
|---|---|
| `core/product_physics/src/lib.rs` | +549 −214 |
| `core/product_physics/src/pressure_runtime.rs` | +10 −0 |
| `core/product_physics/src/source_receipt.rs` | +9 −3 |
| `core/product_physics/src/source_receipt/{tests,load_state_tests,load_state_join_tests}.rs` | +2 −2, +4 −5, +2 −3 |
| `core/product_physics/src/source_recovery.rs` | +27 −15 (including the T1 test expectation change, CHANGE_RECORD) |
| `core/product_physics/src/s11f_tests.rs` | new, 1969 |
| `core/product_physics/tests/s11f_site_test.rs` | new, 1216 |
| `core/product_physics/tests/fixtures/s11f/rf_cancel_cases.json` | new, 24130 (generated) |
| `core/solver/frame_kernel/src/structural.rs` | +150 −81 (N5) |
| `core/solver/frame_kernel/src/structural/s11f_tests.rs` | new, 195 |
| `core/solver/frame_kernel/src/structural/s11f_n5_base_signature.txt` | new, 12 |
| `core/solver/frame_kernel/tests/s11_site_table.rs` | +110 −20 (N1) |
| `core/solver/nonlinear_integration/src/structural_adapter.rs` | +17 −1 (`solve` pub(crate); N8 twin) |
| `core/solver/nonlinear_integration/src/lib.rs` | +9 −0 (N8 twin) |
| `core/solver/nonlinear_integration/src/s11k_tests.rs` | +54 −0 (B-N1) |
| `core/loads/primitive_loads/src/lib.rs` | +8 −0 (doc note) |
| `core/reporting/result_export/tests/load_reference_contract.rs`, `tests/test_load_reference_readers.py` | +2 −2 each (hash pins) |
| 10 regenerated fixtures (CHANGE_RECORD table) | 1–3 lines each |

**Generator:** `IMPLEMENTATION/S11F/generators/gen_rf_cancel_cases.py` (standard library only). Its inputs are `REFERENCES/references.json` `7b176dbb…`, `GATE/S11_EXCEPTIONS.json` `1d8979f6…` and `GATE/FORMATION_EXCEPTIONS.json` `454bbc24…`. All three are recorded in the output, and F1 asserts them.

## 2. Write set (S11 §8.2) and carried items

| Item | State |
|---|---|
| Ledger at every §4.2 producer | done; site rule 5 lists the 10 producers |
| `solve_load_case`: force only from the ledger | done; `global_load_vector` has no product caller (rule 1) |
| `solve_preview_reduced_system` typed | `solve_assembled`, shape unchanged (K-D5 boundary kept) |
| E5, E7–E12, E15, E16 | done (CHANGE_RECORD) |
| Sensitive mapping | `LOAD_CONTRIBUTION_ABSORBED` warning; no envelope field (D-S11-4) |
| Product site test §4.3 | `tests/s11f_site_test.rs`: rules 1–8, plus sparse_direct and the scanner self-checks |
| pressure_runtime operands | `assembled_operands` |
| T1's three sites, plus the 0.4.0 prescribed-motion wiring | done (F5, F9) |
| Removal of `&[f64]` product entry points | SA `solve` is `pub(crate)`. Option (c) `_binary64` variants kept; I1's pins unedited and green |
| sparse_direct sibling | none needed; recorded; site test `sparse_direct_factor_inherits_the_prepared_ledger_binding` |
| `global_load_vector` doc note | done: "no product caller remains" |
| RV1-N1 | both scanners detect self-assignment folds; headers reworded; counts re-baselined; no new unlisted hit |
| RV1-N5 | unaudited row, or an audit error, becomes Sensitive and never `Err`; 3 FK tests |
| RV1-N6 | no rename; site rule plus the behavioural test `n6_product_residual_rows_use_the_exact_numerator` |
| RV1-N8 | positive doctest twins (nonlinear doctests 4 passed) |
| RV1-B-N1 | entry points added; new scan test; doc note that the behavioural pins are authoritative |

## 3. Callers (lexer scan)

`_run_records/callers/callers.json`, from `callers.py.txt` with `scan.py.txt`, re-run on the final tree. It covers every changed, added or removed function, split into non-test and test callers. The key results:
- **`AssemblyEvidence::solve` / SA `solve`:** no non-test caller in any crate. Its only callers are the option (c) tests in `s11k_tests.rs`.
- **`solve_assembled`:** one non-test caller, `product_physics/src/lib.rs:4177`.
- **`global_load_vector`:** no product caller. The only non-test callers are in `validation/benchmarks/mechanics`.
- **`evaluate_original_residual`:** one non-test caller, `nonlinear_integration/src/product_equilibrium.rs:56` (option (c)). The product never calls it (rule 1).
- **The removed helpers** (`corrected_local_forces_for_axial_effects`, `curved_bend_uniform_intensity_by_pipe`, `pressure_thrust_for_pipe`) have no callers.

## 4. F1–F14

All tests are in `product_physics::s11f_tests` unless noted, and all pass (`_run_records/s11f_tests/`). Each has the paths-differ precondition inside the test, where §9 or the brief's "every pin" rule requires one.

| § | Test | Result |
|---|---|---|
| F1, F11, F12 | `f1_f11_f12_rf_cancel_cases_meet_the_binding_predicate_on_both_entries` | 41 RF-CANCEL cases, both entries, both modes (G = 1e80 typed; captured asserted to refuse). No breach outside the 14 formation rows. The 221 S11 triples (87 captured, 134 typed) are published, checked and inside their intervals, so **the S11 list is empty**. The 14 formation rows are exactly bit-pinned (`FORMATION_PINS`). At least 20 discriminating-order preconditions hold. |
| F2 | `f2_probe_a_through_the_product_g1e8`, `_g1e80` | Cantilever: stations within 1e-9 of the exact net, rows bit-equal to the net-load model, C3-detect flags, CHECKS_PASSED. **Plus a simply supported variant**, whose interior elastic maximum depends on E7's intensity; it was added after M1g first survived. |
| F3 | `f3_realistic_hot_run_publishes_the_exact_net_displacement`, `f3_realistic_hot_run_on_the_0_4_0_eigen_route` | pass |
| F4 | `f4_retained_source_selects_the_cancelling_case_and_matches_the_exact_net` | pass (torque DOF, because of the N05 budget, see CHANGE_RECORD) |
| F5 | `f5_eigen_join_is_selected_and_publishes_the_exact_net` | pass |
| F6 | `f6_multi_case_pre_0_4_invocation_never_errs_on_cancelling_loads` | pass |
| F7 | `tests/s11f_site_test.rs` (9 tests) | pass |
| F8 | `f8_curved_bend_uniform_g1e8`, `_g1e80`, `f8_curved_bend_thermal_cancelling_strains`, `f8_curved_bend_thermal_terms_are_exact_products` | pass. The last test was added so that M5 has a behavioural killer: the exact product against the rounded product, with the precondition. |
| F9 | `f9_support_motion_on_the_0_4_0_route_is_exact` | pass (limit in CHANGE_RECORD) |
| F10 | `f10_pressure_and_joint_thrust_g1e8`, `_g1e80` | pass (under the historical pressure scope) |
| F13 | `nonlinear_integration::s11k_tests::option_c_*` and DEC-046's `multisupport_acceptance_inventory_uses_narrow_dec_046_policy` | pass, unchanged |
| F14 | `f14_p1_probe_a_cases_publish_the_exact_net_answer_on_both_entries` | G 1e7 and 1e8, both entries and modes: pass |
| N5 | `frame_kernel::structural::s11f_tests` (3) | pass |
| N6 | `n6_product_residual_rows_use_the_exact_numerator` | pass |

## 5. Mutations

**Setup.**
- Driver: `_run_records/mutations/run_mutants.py.txt`. Patches: `mutants.py.txt`. Results: `mutants_result.json` and `mutants_run.log`.
- Each mutant is applied to a scratch copy, one at a time, and restored afterwards.
- Command: `cargo test --offline --locked --no-fail-fast --lib --test s11f_site_test -- s11f_tests rule_ sparse_direct scanner` in product_physics. M12 also runs frame_kernel `--lib`.
- G = 1e8 and G = 1e80 are separate tests, so the table shows each kill set.

**All 18 are killed.**

| Mutant | Site | Behavioural killers | Site-test killers |
|---|---|---|---|
| M1f | E5 end forces | f2 g1e8, f2 g1e80, f14, f10 g1e8, f10 g1e80 | rule 3, rule 8 |
| M1g | E7 span intensity | f2 g1e8, f2 g1e80 (simply supported variant) | rule 8 |
| M1h | E8 curved thermal strain | f8 thermal cancelling strains | rule 8 |
| M1i | E9 curved recovery | f8 uniform g1e8, g1e80; f8 thermal | rule 8 |
| M1j | E10 intensity per load | f8 uniform g1e8, g1e80 | rule 8 |
| M1k | E11 section resultant | f8 uniform g1e8, g1e80 | rule 8 |
| M1l | E12 reactions | f2 g1e8, g1e80; f14; f8 uniform g1e8, g1e80; f10 g1e8, g1e80 | rule 8 |
| M1n | E15 pressure | f10 g1e8, g1e80 | rule 8 |
| M1o | E16 joint aggregate | f10 g1e8, g1e80 | rule 8 |
| M2 | E5 as two roundings | f10 g1e8, g1e80 | rule 3, rule 8 |
| M3 | ledger fold | F1, f2 ×2, f3 ×2, f4, f5, f6, f8 ×3, f10 ×2, f14 | rule 5, rule 8 |
| M4a | `global_load_vector` call in PP | (site only) | rule 1 |
| M4b | `values().to_vec()` handed to `solve` | compile error E0624 (`solve` is private): the typed seam | n/a |
| M4c | `force[i] +=` on a copy | (site only) | rules 2, 3, 8 |
| M5 | curved thermal pre-summed | f8 thermal terms are exact products | rule 5, rule 8 |
| M8 | pre-0.4 replay on `global_load_vector` | f4, f6 | rule 1, rule 5 |
| M9 | 0.4.0 replay on the fold | f5 | rules 1, 5, 8 |
| M12 | KS1 binary64 fold | f9; FK s11k k11 and option_c probe P | n/a |

**Survivors during the campaign, and how they were repaired.** Both were repaired by adding tests. No test was weakened.
- **M1g** first survived the behavioural tests; only rule 8 killed it. On probe A's cantilever the elastic maximum sits at the root, so E7's intensity never reaches a published value. I added the simply supported variant to F2: an interior maximum at midspan, whose row identity is asserted to be published.
- **M5** was killed only by the site rules. I added `f8_curved_bend_thermal_terms_are_exact_products`.
- M4a and M4c are site-only by design (§9: "fails to compile at the typed seams; F7"). M4b fails to compile.

## 6. Fixture diff and regeneration

- **The stop and the pre-regeneration report** are in `PRE_REGENERATION_REPORT.md`.
- **Scope of the diff.** 112 runs: 108 identical, 4 differ. That is 10 committed files plus 2 pin files.
- **Approval.** ROOT pre-approved regeneration on conditions (`1397e8c43`). The manager confirmed that the conditions were met.
- **Regeneration.** Done by the actual producers only (`_run_records/regeneration/regeneration_commands.txt`). All 12 files are byte-identical to `fixture_diff/measurement_sha256.txt`.
- **Line 2709.** The pre-existing drift is disclosed separately in CHANGE_RECORD.

## 7. Suites

### Full Rust suites

These ran with `--no-fail-fast` on a fresh `<s11f-target>` (`_run_records/suites/SUMMARY.txt`, with per-crate summaries):

| Crate | Passed | Failed |
|---|---|---|
| frame_kernel | 120 | 0 |
| straight_pipe | 39 | 0 |
| curved_bend | 25 | 0 |
| load_case_algebra | 21 | 0 |
| primitive_loads | 49 | 0 |
| nonlinear_integration | 74 | 0 |
| sparse_direct | 25 | 0 |
| linear_supports | 15 | 0 |
| nonlinear_supports | 22 | 0 |
| diagnostics | 24 | 0 |
| performance_harness | 25 | 0 |
| stress_recovery | 48 | 0 |
| user_loads | 28 | 0 |
| self_weight_wasm | 14 | 0 |
| **product_physics** | **474** | **0** |
| operation_applier | 194 | 0 |
| runner/headless | 83 | 0 |
| result_export | 91 | 0 |
| benchmarks/mechanics | 41 | 0 |
| benchmarks/nonlinear (DEC-046 test ok) | 19 | 0 |
| benchmarks/stress | 23 | 0 |
| benchmarks/physics_audit_regression | 15 | 0 |
| benchmarks/numerical_integrity (observer executable, no tests) | 0 | 0 |
| apps/desktop/src-tauri | 114 | 0 |

**Notes on the product_physics row.**
- It includes 1 pre-existing `#[ignore]` (`composite_fields_work_measurement`, a private resource measurement).
- The first full run failed 1 test, the T1 test that asserted the absorbed 0.0. After the disclosed change, the re-run passed 474 with 0 failed (`SUMMARY_product_physics_rerun.txt`).

**A shared-target artefact, disclosed.**
- An earlier, aborted suite run shared `<s11f-target>` with the mutation copies.
- In that run frame_kernel showed 2 failures. They were exactly M12's killers (k11, option_c probe P), from a stale M12-mutated artefact: cargo's hashes for a crate are relative to its workspace, so a scratch copy and the worktree collide.
- I wiped the target and moved the mutation runs to their own target (`<s11f-target>/mut`). The fresh full run above is clean.

### Post-regeneration pinning and consumer tests

| Check | Result |
|---|---|
| result_export `load_reference_contract` | 5 |
| result_export `load_reference_source_contract` | 5 |
| result_export `physics_contract` | 10 |
| result_export `physics_source_contract` | 7 |
| headless lane with the artifact directories set | 83; 28 + 60 artifacts |
| Python readers and consumers | 1309 passed, 5 skipped (gated parity and producer-control lanes) |
| T1 cp3 writer | exit 0 |
| T1 joined `--check` | exit 0 |

### Desktop

- **Scoped runs:**
  - the 6 consumers of the changed fixtures, including `physicsResultEvidence.parity.test.ts`: 89/89;
  - `npm run build`: OK, after `build:wasm`;
  - `node_modules` was linked from `<engine-worktree>`, whose package-lock sha256 is identical (`0dd1616e…`).
- **Full vitest before the restart:**
  - 133 of 134 files and 2821 of 2822 tests passed.
  - The failure: `App.test.tsx > SWBPIPE desktop preview > qualifies only a matching captured native unit replay and retires it on reference inspection`. A `findByTestId("historical-run-context")` wait expired while the host was loaded (load average 4.60 over 1 minute on this host).
  - That test does not read the regenerated fixtures: it replays `physics_connected_ui_*`, `precision_connected_ui_*` and `preview_physics_connected_*`, none of which changed.
  - The same test alone passed. The whole file (225/225) then passed with load average about 2.
  - No timeout was raised and nothing was skipped.
- **Post-restart full vitest:** 134/134 files, 2822/2822 tests passed (§9).

### Python suites (brief list)

Post-restart, in full: 1705 passed, 5 skipped (environment-gated), 83 subtests. `numerical_integrity/test_reference.py`: OK (§9).

## 8. Toolchain and what was not done

**Toolchain.**
- Rust 1.97.1 (`RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`).
- rustfmt 1.8.0-stable through stdin, because the 1.97.1 toolchain here has no rustfmt. New files are rustfmt-clean. In `product_physics/src/lib.rs`, `pressure_runtime.rs` and `nonlinear_integration/src/lib.rs`, whose base is not rustfmt-clean (110, 5 and 24 hunks), only my hunks are formatted; the module-order hunks are left as base had them.
- `git diff --check` is clean.
- Python 3.11 with the DEC-025 venv. Node 24.

**Not done:**
- S-H, and the captured G = 1e80 route (by ruling).
- The 14 formation rows (S11-G, then F2/F3).
- K-D5's formation check (I3's).
- The merge-forward (the manager's).
- A DEC-025 sweep (ROOT's).
- Whether the App.test failure reproduces on origin/main under similar load was not checked: that needs a second desktop build of main.

## 9. Post-restart (2026-09-27)

The container restarted during the Python suite run, and my in-flight processes were killed.

**State re-established from disk:**
- HEAD `c99e322d6`, 37 uncommitted paths, as expected.
- All 12 regenerated files still match `measurement_sha256.txt`, so no producer was re-run.
- Both authority targets are present and working (checked through the adapters), so they were not rebuilt.

**Accepted as complete from before the restart,** with logs kept:
- the regeneration, the writers, the full Rust suites, the mutations, the desktop scoped runs and the build;
- the first full vitest.

**Re-run in full after the restart:**
1. result_export pinning and consumer tests: 5, 5, 10, 7, all passed (`_run_records/restart/rust_pinning_consumers.txt`).
2. The Python suites of the brief, interrupted by the restart (`_run_records/restart/python_suites.log`):
   - load_reference readers, schema and headless artifacts;
   - preview_physics consumer;
   - physics consumer and source contracts;
   - qualification (gate, load_reference, physics, integration, structure);
   - stress_neutral (package, physics source, precision);
   - plus `validation/benchmarks/numerical_integrity/test_reference.py`.
3. The headless artifact lane: re-generated on the clean target, then its pytest.
4. The full desktop vitest and build.

**Results of the post-restart re-runs** (logs in `_run_records/restart/`):

| Re-run | Result |
|---|---|
| 1. result_export pinning and consumer tests | 5, 5, 10, 7 passed |
| 2. Python suites of the brief | 1705 passed, 5 skipped, 83 subtests passed. The 5 skips are the two parity-log lanes and the three gated producer-control lanes, all environment-gated. |
| 2. `numerical_integrity/test_reference.py` | OK |
| 3. Headless lane with the artifact directories set | 83 passed; 28 + 60 artifacts. `test_load_reference_headless_artifacts.py`: 15 passed. |
| 4. Full desktop vitest | **134/134 files, 2822/2822 tests passed.** Load average was 2.13 at start and 5.50 at end. No timeout raised; nothing skipped or isolated. |
| 4. `npm run build` | OK |

**Notes on these runs.**
- The wasm engine artifacts under `apps/desktop/public/` (git-ignored, built from the unchanged `operation_applier`) survived the restart and were reused.
- The `node_modules` links were removed afterwards.
- The pre-restart App.test failure did not recur. It reads none of the regenerated fixtures. I did not check whether it also fails on origin/main under load: that needs a second desktop build of main, which I did not run.

**Tree after the re-runs.** 38 status paths:
- the 37 before the restart;
- plus `_run_records/restart/` and the new record directories, now listed individually;
- the `node_modules` links are gone.

No committed file changed apart from the records named in this return.
