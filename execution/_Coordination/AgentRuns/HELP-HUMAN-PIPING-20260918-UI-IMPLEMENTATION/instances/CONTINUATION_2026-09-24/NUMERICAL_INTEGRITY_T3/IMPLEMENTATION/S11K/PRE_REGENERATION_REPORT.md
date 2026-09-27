# S11-K: stop record and pre-regeneration report

I1 (TASK), 2026-09-26. Worktree `<s11k-worktree>` on branch `codex/piping-s11k-20260926`, base `163cd44ab`. No Git write and no regeneration were made. Every regeneration measurement below was run in a scratch copy of the tree.

This report answers:
- ROOT's rulings on the stop report (option (c); A and B2 pre-registered; B1 accepted as the intended repair);
- the manager's follow-ups: the full caller list, the unit-force allow-list, the pin list and the derived diffs;
- V1's R5-3 and ROOT's ruling on it;
- ROOT's ruling on P1's S11 probes.

## 1. Option (c) as implemented

The separation is clean. I implemented option (c), not the fallback (a).

**New named, unchanged binary64 entry points.** They are byte-identical to the behaviour at `163cd44ab`:

| Function | Where | What it keeps |
|---|---|---|
| `reduce_system_with_prescribed_displacements_binary64` | `FK/lib.rs` | fold `f − ΣK·g` in prescribed-list order, every row |
| `prepare_structural_binary64`, `solve_structural_dense_binary64` | `FK/structural.rs` | KS1 fold; the refinement residual (KS3) folds |
| `AssemblyEvidence::solve_binary64` | `SA` | the same, with the assembly evidence |
| `structural_adapter::solve_structural_sparse_binary64` | `SA` | the prepare, skyline LDL (`sparse_direct::structural::factor_structural_ldlt`), witness and finish sequence of `solve_structural_sparse`, on the binary64 kernel path. It lives in SA because sparse_direct is outside the write set |

**Public `structural::evaluate_original_residual` stays today's binary64 evaluation, byte for byte.** The exact KS3 numerator is applied only in:
- the solve's own residual and refinement (`finish_checked_factor` with the exact binding);
- `evaluate_assembled_original_residual`.

**Nonlinear loop: call targets only.** `nonlinear_integration/src/lib.rs`, `solve_linearized_system_evidence`:
- `reduce_system_with_prescribed_displacements` → `…_binary64` (and its `use` line);
- `assembly.solve` → `assembly.solve_binary64`;
- `structural::solve_structural_dense` → `…_binary64`;
- `sparse_direct::structural::solve_structural_sparse` → `structural_adapter::solve_structural_sparse_binary64`.

There are no signature changes and no loop logic changes. Its `product_equilibrium::evaluate` call is unchanged and uses the binary64 public residual.

**Pins:**
- `nonlinear_integration::s11k_tests::option_c_nonlinear_loop_is_pinned_to_the_binary64_kernel_path`: a source pin on `solve_linearized_system_evidence`. It requires the four `_binary64` targets and `product_equilibrium::evaluate(`, and forbids every exact entry point (`reduce_system_with_prescribed_displacements(`, `assembly.solve(`, `assembly.solve_assembled(`, `solve_structural_dense(`, `solve_structural_sparse(`, `prepare_structural(`).
- `nonlinear_integration::s11k_tests::option_c_product_equilibrium_uses_the_binary64_residual`: `product_equilibrium::evaluate` calls `structural::evaluate_original_residual`, and SA's gap scrutiny calls `product_equilibrium::evaluate`.
- `frame_kernel::structural::s11k_tests::option_c_binary64_variants_keep_todays_fold_on_probe_p`: on V1's probe P, the binary64 reduction and prepare equal today's in-test fold bit for bit, and the exact variants differ.
- `frame_kernel::structural::s11k_tests::option_c_public_original_residual_stays_binary64_on_coupled_rows`: on a coupled row, the public evaluation equals today's in-test expression bit for bit, and the typed evaluation equals the exact sum.

**Result.** `validation/benchmarks/nonlinear` passes unchanged (19/19). The DEC-046 zero limits are untouched. The committed-request fixture diff is byte-identical to the pre-(c) candidate: A, B1 and B2 as before, nothing else.

## 2. Complete caller list of the KS-affected functions (merged tree, non-test code)

The affected functions are:
- KS2: `reduce_system_with_prescribed_displacements`, plus `reduce_system`, which reaches the same code with all-zero prescribed values;
- KS1 and KS3 in the solve: `prepare_structural` and everything composed on it (`solve_structural_dense`, `sparse_direct::structural::solve_structural_sparse`, `AssemblyEvidence::solve`, and `finish_structural`, which runs KS3 in the refinement);
- `evaluate_original_residual`.

Found by a lexer scan of every `.rs` file outside `execution/`, with test code excluded. List: `_run_records/fixture_diff/ks_callers.txt`.

| Caller (merged tree) | Function called | Class | Path after S11-K |
|---|---|---|---|
| `core/product_physics/src/lib.rs:2333` `solve_load_case` | `reduce_system_with_prescribed_displacements` | linear | exact KS2 (live only where a prescribed value is nonzero: T1's 0.4.0 support motion, the A and B2 fixtures) |
| `core/product_physics/src/lib.rs:2340` `solve_load_case` | `reduce_system` | linear | the exact path gated off (all prescribed values zero), bit-identical |
| `core/product_physics/src/lib.rs:3965` `solve_preview_reduced_system` | `AssemblyEvidence::solve` | linear | exact KS1/KS3 (live only with nonzero prescribed values) |
| `core/solver/linear_supports/src/lib.rs:467` `apply_linear_supports` | `reduce_system_with_prescribed_displacements` | linear | exact KS2 |
| `core/solver/linear_supports/src/lib.rs:487` `validate_global_system` | `reduce_system_with_prescribed_displacements` with no prescribed DOFs | linear | no coupling, bit-identical |
| `core/solver/sparse_direct/src/structural.rs:21` `solve_structural_sparse` | `prepare_structural` | linear (called by SA::solve and PP) | exact |
| `core/solver/nonlinear_integration/src/structural_adapter.rs:301/303` `AssemblyEvidence::solve` | `solve_structural_dense` / `solve_structural_sparse` | linear (PP:3965) | exact |
| `…/structural_adapter.rs:297` `solve` with `with_force_terms` | `prepare_structural_with_force_terms` | dormant (C3-detect seam) | exact binding, legacy vector |
| `…/structural_adapter.rs:334`, `:382` (`solve_binary64`, `solve_structural_sparse_binary64`) | `prepare_structural_binary64` | nonlinear | legacy |
| `…/structural_adapter.rs:1146` `scrutinize_gaps` (V1's "SA:1030") | `product_equilibrium::evaluate` → `evaluate_original_residual` | nonlinear: `scrutinize_gaps` is `pub(crate)`, called only from the active-set loop (`nonlinear_integration/src/lib.rs:690`) | legacy binary64 |
| `core/solver/nonlinear_integration/src/product_equilibrium.rs:56` `evaluate` | `evaluate_original_residual` | nonlinear (callers: lib.rs:2025 and SA:1146) | legacy binary64 |
| `core/solver/nonlinear_integration/src/lib.rs:1968/1981/1999/2004/2025` `solve_linearized_system_evidence` (loop, influence solves, unit-force solves; V1's ":1957" is `:2025`) | the four `_binary64` targets and `product_equilibrium::evaluate` | nonlinear | legacy, pinned |
| `validation/benchmarks/mechanics/src/lib.rs` (10 × `reduce_system`, 8 × `apply_linear_supports`, 6 of those with imposed displacements) | as named | linear | exact |
| `core/solver/performance_harness/src/lib.rs:773/959/966` | `reduce_system` | linear | the exact path gated off (zero prescribed), bit-identical |
| `core/solver/frame_kernel/src/structural.rs:1510/1517/1531/1540` | internal composition | — | — |

`source_recovery.rs` uses `AssemblyEvidence` only for its contributions and the exact context: it never calls a solve or reduction. `runner/headless`, `result_export` and `src-tauri` call none of these functions directly.

**Published residual rows (manager item 1).**
- The public `evaluate_original_residual` has no linear product caller. Its only non-test callers are nonlinear, through `product_equilibrium::evaluate`, and it stays binary64.
- Linear published residual rows (`StructuralReport.residual_rows`, the Debug text in PP diagnostics) come from the solve's own residual. That residual uses the exact KS3 numerator on prescribed-coupled rows, the same arithmetic as the solve's refinement, so no row diverges from the refinement residual.
- On the nonlinear path the loop's check `product_equilibrium.rows == structural_report.residual_rows` still compares two binary64 evaluations, now both legacy. Nothing is relabelled.

**linear_supports / mechanics benchmark (R5-3 item 2).**
- `apply_linear_supports` is a linear caller and takes the exact KS2.
- I ran every mechanics-benchmark fixture observation (`fixture_inventory` → `fixture_observations`) and every `solve_tp_phys_002/004/005/006/007/008/009/014` / `tp_phys_015a` result through a scratch harness on the base and on the candidate.
- The Debug outputs (36 lines, shortest round-trip floats) are byte-identical. `validate_tp_phys_002_linear_static_integration` and `validate_imposed_displacement_fixture` are true on both.
- The benchmark suite passes (41/41).
- No benchmark result, fixture or protected check moves.

**Unit-force influence solves (manager item 2).**
- At `c61a540ea`: `nonlinear_integration/src/lib.rs:1333` (`unit_force[load_candidate.global_dof] += 1.0`) and `:1334-1340` (`solve_linearized_system_evidence(Some(assembly), …)`).
- At the merged tree the same lines are `:1399` and `:1400`, inside `solve_iteration_with_sliding_friction_evidence`.
- They go through `solve_linearized_system_evidence`, so they are on the legacy path.
- The site-test row `("nonlinear_integration/lib.rs", "solve_iteration_with_sliding_friction_evidence", 2, "allow-listed as T5's: unit-force influence solves (c61a540ea :1333) and their integer coupling pattern")` stays accurate: the test passes with count 2.

## 3. Full suites after option (c)

Summary: `_run_records/suites/SUMMARY_final.txt`. All suites ran with `--offline --locked`, with nothing skipped or filtered. Every crate passes except `runner/headless`: 49 passed, 2 failed, both pre-registered A.

| Crate | Result |
|---|---|
| frame_kernel | 115 passed incl. site table and doctests (116 after the R5-3 pin) |
| straight_pipe | 38 passed (39 after the P1-probe test, §7) |
| curved_bend | 25 |
| load_case_algebra | 21 |
| primitive_loads | 49 |
| nonlinear_integration | 68 (69 after the R5-3 pins) |
| sparse_direct | 25 |
| linear_supports | 15 |
| nonlinear_supports | 22 |
| diagnostics | 24 |
| performance_harness | 25 |
| stress_recovery | 48 |
| user_loads | 28 |
| self_weight_wasm | 14 |
| product_physics | 448 (1 ignored in source) |
| operation_applier | 194 |
| **runner/headless** | **49 passed, 2 failed**, both pre-registered A: `load_reference_one_actual_solve_mints_bound_evidence_and_canonical_document_both_modes` ("connected/sparse_interactive: actual producer changed") and `joined_actual_solve_retains_invocation_bound_receipt_without_canonical_export_both_modes` ("eigen_motion/sparse_interactive: actual producer changed") |
| result_export | 91 |
| benchmarks/mechanics | 41 |
| **benchmarks/nonlinear** | **19 (passes unchanged)** |
| benchmarks/stress | 23 |
| benchmarks/physics_audit_regression | 15 |
| benchmarks/numerical_integrity | 0 tests |
| apps/desktop/src-tauri | 114 |

The frame_kernel, straight_pipe and nonlinear_integration suites were re-run after the R5-3 pins and the P1-probe test were added, and they pass.

## 4. Tests and fixtures that pin the bytes of B1, A and B2

The scan covers every source, test, fixture and manifest outside `execution/`, by file name and by sha256 (`_run_records/fixture_diff/pin_scan.json`), plus pattern-built paths found by grep. Historical records under `execution/` are not listed and never change.

**Hash pins, which are test-source constants and need an edit when the raws are regenerated:**
- `core/reporting/result_export/tests/load_reference_contract.rs`, `frozen_inputs_table_and_schema_are_pinned`: sha256 of `connected-sparse_interactive.raw.json` (`915965a4…`) and `connected-dense_scrutiny.raw.json` (`3824035c…`).
- `tests/test_load_reference_readers.py`, `FROZEN` in `test_frozen_inputs_and_table_identity`: the same two sha256s.
- Neither is a fixture. No producer rewrites them; the edit is to a test constant outside my write set. ROOT or the manager should decide who edits them, and when, alongside the regeneration.
- No pin was found for the eigen_motion raws, the B2 raws or the B1 files by hash.

**By name, reading the committed bytes:**

| Fixture | Tests / fixtures |
|---|---|
| `fixtures/results/preview_physics_invented_dense.json` | `core/reporting/result_export/tests/preview_physics_contract.rs`; `apps/desktop/src/features/results/previewPhysicsEvidence.test.ts`; `tests/test_preview_physics_consumer_contract.py` (`preview_physics_invented_{mode}.json`) |
| `fixtures/results/preview_physics_invented_sparse.json` | `preview_physics_contract.rs`; TS: `services/ruleCheckService.test.ts`, `features/workspace/currentResultUnitPolicy.test.tsx`, `features/results/previewPhysicsEvidence.test.ts`, `features/results/knownSemanticLimitations.test.ts`, `features/results/KnownSemanticNotices.test.tsx`, `features/rule-check/RuleCheckRunPanel.test.tsx`; `tests/test_preview_physics_consumer_contract.py` |
| `load_reference/connected-dense_scrutiny.raw.json` | `load_reference_contract.rs` (name and hash), `tests/test_load_reference_readers.py` (name and hash), `result_export/tests/fixtures/load_reference_mutations.json`; headless `load_reference_route_tests` (live producer compared with the committed raw) |
| `load_reference/connected-sparse_interactive.raw.json` | the above, plus `load_reference_source_contract.rs`, `tests/test_load_reference_source_schema.py`, `result_export/tests/fixtures/load_reference_source_mutations.json`; TS: `services/loadReferenceRoundTrip.test.ts`, `features/workspace/loadReference.resultsSessionState.test.ts`, `features/results/loadReferenceReaderCases.{cases.json,test.ts}`, `features/load-cases/loadStateFixtures.test.tsx`; `apps/desktop/src-tauri/src/lib.rs` |
| `load_reference_source/eigen_motion-dense_scrutiny.raw.json` | `load_reference_source_contract.rs`, `load_reference_source_mutations.json`, `services/loadReferenceRoundTrip.test.ts`; headless `joined_actual_solve…` |
| `load_reference_source/eigen_motion-sparse_interactive.raw.json` | `load_reference_source_contract.rs`, `load_reference_source_mutations.json`; headless `joined_actual_solve…` |
| `result_export/tests/fixtures/load_reference_fallback_uz-dense_scrutiny.raw.json` (B2) | `load_reference_source_contract.rs`, `load_reference_source_mutations.json` |
| `…/load_reference_fallback_uz-sparse_interactive.raw.json` (B2) | the above, plus TS `features/workspace/loadReference.resultsSessionState.test.ts`, `features/results/loadReferenceReaderCases.cases.json` |
| derived `fixtures/results/load_reference_connected_*.{document,analysis_run,stress_neutral}.json`, `load_reference_source_eigen_motion_*.*` | pattern-built: `load_reference_contract.rs:479` and `load_reference_source_contract.rs:578` (the documents, byte compare); `tests/test_load_reference_readers.py:250` and `tests/test_load_reference_source_readers.py` (analysis_run, byte compare); `tests/test_load_reference_headless_artifacts.py:158/187` (analysis_run); `tests/test_load_reference_source_schema.py` (the `load_reference_[cp]*_*.document.json` glob, and the connected stress_neutral and analysis_run carriers); `apps/desktop/src/services/loadReferenceAnalysisRun.test.ts` (`${carrierName}.analysis_run.json`) |

**Check in scratch.** With the candidate raws, the B1 files and the regenerated derived documents in place, the Python suites for these files all pass except the hash pin:
- `tests/test_preview_physics_consumer_contract.py`, `test_load_reference_{readers,source_readers,schema,source_schema,headless_artifacts}.py` and `test_stress_neutral_physics_source.py`: 1280 passed, 17 skipped, 1 failed. The one failure is `test_frozen_inputs_and_table_identity`, the hash pin.
- The result_export writer tests pass except the Rust hash pin.
- TS tests were not run: the worktree has no `node_modules`, and installing them was not attempted.

## 5. Measured diff of every derived document of A and B1

Measured in a scratch copy, using the actual producers:
- raws: the candidate harness output. It reproduces the committed raws byte for byte on the base; the raw producer path is the same `run_linear_static_preview_value_with_mode` output;
- documents: `result_export` tests with `LOAD_REFERENCE_WRITE_FIXTURES=1` / `LOAD_REFERENCE_SOURCE_WRITE_FIXTURES=1`;
- analysis runs: `tests/test_load_reference_readers.py` / `test_load_reference_source_readers.py` with the same flags;
- stress-neutral packages: T1's reproducers `cp3_stress_neutral_outputs.py` and `t1_joined_stress_neutral_outputs.py` (session 2 and session 4 run records).

Unaffected carriers (pressure, fields, mixed, n05, n06) came out byte-identical, which confirms that the pipeline reproduces committed bytes. Summary by kind: `_run_records/fixture_diff/diff_derived_documents.json`.

| Derived file | Bytes before → after | Changed leaves | What moves |
|---|---|---|---|
| `load_reference_connected_dense.document.json` | 1452801 → 1452871 | 240 | 28 result values and their unit witnesses (≤3.9e-16 relative), 6 exact-case extrema values (≤2.2e-16), and the row checksums and source hashes that bind them |
| `load_reference_connected_sparse.document.json` | 1445239 → 1445192 | 234 | as dense, ≤4.3e-16 |
| `load_reference_connected_dense.analysis_run.json` | 419656 → 419728 | 34 | diagnostics text (15 tokens), result hash refs (29), run hashes (2), parity observation (1) |
| `load_reference_connected_sparse.analysis_run.json` | 415718 → 415863 | 32 | diagnostics text, hash refs (28), hashes (2) |
| `load_reference_connected_dense.stress_neutral.json` | 883762 → 883920 | 194 | the same values in rows, CSV text and witnesses, `source_value_bits`, member and package checksums |
| `load_reference_connected_sparse.stress_neutral.json` | 876321 → 876244 | 187 | as dense |
| `load_reference_source_eigen_motion_dense.document.json` | 531385 → 531385 | 6 | `receipt_sha256`, `source_block_recovery.body` digest, reproducibility hashes (no result value moves) |
| `load_reference_source_eigen_motion_sparse.document.json` | 527466 → 527466 | 8 | as dense, plus `invocation_work` charged / publication_charged (−12) |
| `load_reference_source_eigen_motion_dense.analysis_run.json` | 210125 → 210125 | 5 | diagnostics text (6 tokens), receipt and body digests, hashes |
| `load_reference_source_eigen_motion_sparse.analysis_run.json` | 208027 → 208026 | 7 | as dense, plus invocation_work |
| `load_reference_source_eigen_motion_dense.stress_neutral.json` | 347791 → 347791 | 8 | receipt / publication digests, member, package and source checksums |
| `load_reference_source_eigen_motion_sparse.stress_neutral.json` | 343999 → 343999 | 10 | as dense, plus invocation_work |
| **B1** `preview_physics_invented_dense.json` (producer: `core/product_physics/examples/preview_physics_capture.rs`, `dense --model core/product_physics/tests/fixtures/preview_physics_invented_model.json`; my harness is the same call, and its base output equals the committed file byte for byte) | 568494 → 568492 | 6 | pipe:P-120 quarter-3 bending moment z (L-100, L-200; 5.9e-16), the two matching stresses, and end-j bending stress z (roundoff-level near-zero value, 4.985e-15 → 5.816e-15 MPa) |
| **B1** `preview_physics_invented_sparse.json` | 565864 → 565865 | 6 | as dense |

B1 has no committed derived documents. B2 has no committed derived documents: its raws are read only by tests.

## 6. Mutation re-run on the final candidate

`_run_records/mutations/mutation_results_final.json`, run after option (c). Every mutant is killed:
- M1a, M1b, M1c, M1d, M1f and M1m, at G=1e8 and G=1e80 separately;
- M6, M7a, M7b, M10, M11, M12, M13, M14, M15 and M15b.

M12 and M13 are now killed by two tests each, K11 plus the option (c) pin. The anchors were updated for the option (c) text.

## 7. P1's S11 probes (ROOT ruling)

- **S11-PROBE-A-G1e7 / -G1e8**, from T3 DETECTION at `516c34c1a`: `scripts/gen.py.txt` `supplementary_cases`, and `results.json` `exceptions.captured.s11_class`.
- **S11-K part (added).** `straight_pipe::s11k_tests::p1_probe_a_recovery_is_exact_at_g1e7_and_g1e8`: the same 2 m cantilever, (G, 0.3, −G) N/m, OD 0.2 m, wall 0.01 m, E 200 GPa, with the product's binary64 section values. The tip load is the exact net, as the ledger gives it. The checks:
  - the tip deflection is w·L^4/(8EI);
  - E3 root shear and moment equal the exact dyadic w·L = w·L²/2 = 0.6 within 1e-9, at G = 1e7 and 1e8;
  - precondition: the producer-order fold differs.
  The expected values are derived in the test, not copied from P1.
- **S11-F part (listed).** The probes as product models through the captured entry. They cover all six P1 rows (u.tip.UY, th.tip.RZ, R.root.UY/RZ, Mb/Vb.pipe.i) in both modes. P1's observed error comes from PP's force-side fold of the three element equivalents (PP:7797 class) and from E5 in PP, which is S11-F's ledger and recovery. Expected values are exact rationals: w = 3/10, L = 2, with EI from the product's section.

## 8. What happens next

Nothing is regenerated. On ROOT's go-ahead, the regeneration is:
- the A raws, B2 raws and B1 files by their actual producers;
- then the documents, analysis runs and stress-neutral packages by the producers listed in §5;
- then the two hash-pin test constants in §4, by whoever ROOT assigns.

After that, the change record will disclose the sizes above.
