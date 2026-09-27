# I3 return: slice K-D5, the D-5 formation check

Type 2 TASK I3, 2026-09-27, for the T3 manager.
- **Brief:** `TASK_BRIEFS/I3_KD5_IMPLEMENTATION.md` with addenda 1–4, read with `_COMMON.md`.
- **Worktree:** `<wt>/kd5`, branch `codex/piping-kd5-20260926`.
- **Base:** the K3a head `a2e804a757359d589f4c31ea8e36a923f28ccb8c`, on the S11-K head `4912dc636`.
- **Records:** `T3/IMPLEMENTATION/KD5/`, run records in `_run_records/`, hashes in `SHA256SUMS`.

> **Current state:** the addendum-4 pass on the pre-S11-G tree (`3befacff4` plus the addendum-4 edits) is complete. See **"Addendum-4 pass"** at the end, with records in `_run_records/addendum4/`.
> - The 8 items are done.
> - Suites: 24/24 crates pass.
> - Fixture diff against main `72d5ff864`: 112/112 identical.
> - Mutations: as in phase 1.
> - Both-entry gate against numerics `59fff0d9e`'s lists: **PASS**, with only the 7 FORMATION triples.
> - 122 demotes on both entries in both modes.
>
> Sections 1–11 below are the phase 1 return on base `a2e804a75`, kept as recorded. Where they describe the legacy `solve_with_formation_check`, the `_with_force_terms` variant, the `cfg_attr` on `mod wide;` or the companion pins, the addendum-4 pass supersedes them.

## Status: implemented and verified on the base

**Implemented and verified:**
- Every write-set item is implemented.
- Every required test passes, in both modes and, at product level, on both entries.
- The fixture diff is **112 of 112 outputs byte-identical**, so the stop rule did not trigger.
- Every existing suite in the touched crates and their path dependents passes.

**Mutations:** (23), (26), (27), (28), (31a), (32a) and (32b) are killed. **(31b) survives, and ROOT has accepted it as an equivalent mutant at the 1e-9 criterion** (2026-09-27, `c2042fd9c`; §7).

**Next:** the manager commits this tree and merges origin/main (K3a, PR983). I then resolve per addendum 4 and re-verify.

## 1. Files changed (against `a2e804a75`)

| File | +/− lines | What |
|---|---|---|
| `P/core/solver/frame_kernel/src/structural.rs` | +107 −1 | adds the formation-check wrapper and prepare/solve entries, runs the check and the demotion in `finish_checked_factor`, and adds `formation_check` to `StructuralSolution` (details in §2) |
| `…/frame_kernel/src/structural/formation_check.rs` | new, 873 | the check |
| `…/frame_kernel/src/structural/formation_check_tests.rs` | new, 351 | 5 kernel tests |
| `…/frame_kernel/src/structural/retained/mod.rs` | +12 −6 | inner `#![allow(dead_code)]` removed; see §2 |
| `P/core/solver/nonlinear_integration/src/structural_adapter.rs` | +195 −2 | primitives recorded by `AssemblyEvidence::new`; the two new entries; curved matching; the test module declaration |
| `…/structural_adapter/kd5_tests.rs` | new, 866 | 15 adapter tests, both modes |
| `…/structural_adapter/kd5_models.rs` | new, 116 | generated models and exact references (`_run_records/models/kd5_models.py.txt`) |
| `P/core/product_physics/src/lib.rs` | +17 −1 | only the call in `solve_preview_reduced_system`, at base line 3965 (now 3973) |
| `P/core/product_physics/tests/formation_check_runtime.rs` | new, 182 | 3 product tests through both entries |

The `structural.rs` additions are:
- `mod formation_check` and its re-exports;
- `FormationCheckedSystem`, built by `StructuralSystem::with_formation_source` or `AssembledStructuralSystem::with_formation_source`;
- a private `formation` field on `PreparedSystem`;
- `prepare_formation_checked_structural`, its `_with_force_terms` variant, and `solve_formation_checked_structural_dense`;
- the check and the demotion in `finish_checked_factor`;
- the `formation_check` field on `StructuralSolution`.

Nothing else changed:
- `StructuralReport` is unchanged (D5C-3).
- K3a's `wide.rs` and `wide_tests.rs` are byte-identical to the base.
- No Cargo.toml, lockfile, schema, fixture or committed output changed.
- No struct literal outside the write set changed.

Formatting: the new and changed files were formatted with `rustfmt` in stdin mode, one file at a time.
- The base PP `lib.rs` is not rustfmt-clean (81 regions), so that file was not reformatted. Its one hunk is identical to rustfmt's output for that region.
- `kd5_models.rs` is generated and carries `#![cfg_attr(rustfmt, rustfmt::skip)]`.

## 2. Write-set items (brief and D1 §6 K-D5 row)

| Item | Where / how |
|---|---|
| EF after the residual gate in `finish_checked_factor` | runs only when the `PreparedSystem` carries a formation source and the case would publish Passed (rcond ≥ √eps and no load-fidelity flag). One solve pair with the attempt's factor, in its scaled variables, with an extra power of two so the largest right-hand side lies in [1, 2) |
| Passed → Sensitive demotion | `quality = Sensitive` when the check returns a record. Displacements and every other report field are unchanged (asserted bitwise in the tests) |
| Typed optional formation source on `StructuralSystem` | realized as S11-K realized its typed system: a wrapper, `FormationCheckedSystem`, because a new field would break about 45 struct literals outside the write set (manager note (a), accepted). The typed variant carries the `AssembledForce`, so ρ uses the ledger terms |
| `FormationCheck` on `StructuralSolution`, never on `StructuralReport` | `Option<FormationCheck>` holding the reason (`Estimate` or `FormationCheckUnavailable { detail }`), the row, 2\|w\|, the scale and the trigger value. It is set only on demotion and rendered nowhere (no `{:?}` of `StructuralSolution`, `PreparedSystem` or `AssemblyEvidence` in PP, headless or result_export; checked) |
| `formation_check.rs`: `Wide<2>` re-formation | at p = 128. **Frames:** the chord, the y-reference frame, L, EA/L, GJ/L, 12EI/L³, 6EI/L², 4EI/L, 2EI/L and TᵀKT. **Curved bends (R5-4 §2):** radial vectors, R, cos φ and sin φ from square roots, φ from K3a's `included_angle`, the closed-form flexibility and its Gauss–Jordan inverse (symmetric part), axes, and H from the **actual chord**. **Joints:** relative axial, torsional and angular springs in the actual chord frame; lateral ≠ 0 fails closed. **Springs:** exact binary64 values |
| ρ as one `ExactAccumulator` sum per free row | the load terms (`audit_terms()`: the ledger's terms if typed or supplied, else the folded force), then −K_e·u for every element coefficient through `add_product_to`, and the springs. Prescribed columns are included (K_fc·u_c). A split truncated below 2^-1074 widens ρ_i away from zero by 2^-1074·\|u_j\| (the addendum's allowance). This uses the same exact-sum machinery as `contribution_sums`/`audit_intended_action`; K-D5 supplies the re-formed coefficients in place of binary64 contributions |
| The rule | 2\|w_i\| > 1e-9·max(\|q_i\|, S*_kind). S* follows D1 §4.1.6.1 items 1 and 4–6: bodies from the element graph (springs and restraints do not connect), S over free rows, and L_b and the coupling in the stated fl order. **Zero-scale clause:** scale 0 with w_i ≠ 0 fires |
| Fail closed | any listed unavailable family, any `WideError` (including `AngleDomain` near π, addendum 3), any exact-sum error, a DOF-map mismatch, or a failed correction solve gives `FormationCheckUnavailable`. It is never passed and never an `Err` |
| SA: the source from `AssemblyEvidence::new` | frames, users and springs are recorded as given. Each curved slot records its id, nodes, matrix and whether it is explicit (`symmetry_formation` is `None`, i.e. built by `CurvedBendStiffnessElement::new`) |
| SA: `solve_with_formation_check` beside the unchanged `solve` | takes `curved_sources: &[CurvedBendMacroElement]` (decision 1, option B) and `selected: bool` (decision 2). `selected = false` returns the unchanged `solve`. Matching is by (node_i, node_j) and bitwise `global_stiffness()` equality, in any order. An unmatched or explicit slot is named in `unavailable` |
| SA: typed `solve_assembled_with_formation_check` | added on your S11-F note, mirroring `solve_assembled`. S11-F's typed PP call takes it at the forward merge |
| PP: the one call site | `solve_preview_reduced_system`, base line 3965: `assembly.solve(…)` became `assembly.solve_with_formation_check(…, &curved_sources, built.nonlinear_supports.is_empty())`, where `curved_sources` comes from `built.curved_bend_elements`. No other PP change |
| C2 (numerics worktree) | `withheld_rows.py` gains `support_stiffness_input` (top-level `stiffness`, else `hanger.stiffness`, as PP:10253); `b_proof.py` places a spring DOF through it. Committed by the manager as `d61032f9d`. Result in §4 |
| `retained` dead_code (addenda 1 and 4) | the inner `#![allow(dead_code)]` is removed. On the development branch, `#[cfg_attr(not(test), allow(dead_code))] pub(crate) mod wide;` replaces it: still module-scoped, but non-test builds only. A per-item allowance would have required editing K3a's `wide.rs`. **At the forward merge (addendum 4)**, this becomes per-item `#[allow(dead_code)]` with reasons. The 13 items unused outside tests are listed after this table |

**The 13 items unused outside tests:**
- `ATAN_TOLERANCE_ULPS` and `ATAN_PROVED_BOUND_TENTH_ULPS`: test-only, documentation constants.
- `WideError::NotNormalized`, `Wide::from_parts` and `Wide::parts`: K3 API.
- `Binary64Split::truncated_below_min_subnormal`: test-only; K-D5 uses `add_product_to`'s flag.
- `Wide::{is_sign_negative, exponent, fits_precision}`: K3 API.
- `WorkCounter::rounded_operations` and `WideArith::{precision, work}`: K3/K4 budgets; used by K-D5's tests.
- `WideArith::atan_positive`: K3 API.

The manager has noted K3a's rename of the tolerance constant in `706d027f0`. K-D5 references neither tolerance constant.

## 3a. RF-SKEW-T-CANT-OFF-122-r1e-04: K-D5's required true positive

This is the confirmed M03 skew breach: 2.43e-9 relative on main, published Passed and eligible (P1). ROOT confirms it is K-D5's required true positive, not an exception and not an availability loss.

| Entry | Mode | Standing on the candidate | Frozen-reference comparison (the product's own values) | K-D5 trigger (adapter test, identical system) |
|---|---|---|---|---|
| captured | sparse | Sensitive, `needs_recompute`; `NUMERICAL_INTEGRITY_SENSITIVE` | breach 1.2139× at th.N1.RX (unchanged values) | 2·EF/criterion = 2.428 (EF 1.214), row 9 (N1 RX) |
| typed | sparse | the same | 1.2139× at th.N1.RX | the same |
| captured | dense | Sensitive, `needs_recompute` | 2.4263× at th.N1.RX | 2·EF/criterion = 4.827 (EF 2.413), row 9 |
| typed | dense | the same | 2.4263× at th.N1.RX | the same |

- **The trigger is the estimate rule** (reason `estimate`): 2|w_i| > 1e-9·max(|q_i|, S*_rot) on N1's RX rotation. EF equals the actual error to better than 1e-3 (2.413 vs 2.413 dense, 1.214 vs 1.214 sparse).
- **Values are unchanged.** The published values still breach, but they are no longer published as trusted. The only standing change against P1's main baseline in the whole gate is this case, checks_passed → sensitive, on both entries and in both modes.
- **The case appears nowhere in the gate as a Passed breach** (§9), and it is not an exception.
- **Tests that pass:**
  - `kd5_required_true_positive_skew_cantilever_122_demotes_in_both_modes` (adapter);
  - `kd5_typed_entry_demotes_122_and_unselected_is_solve_assembled`;
  - `kd5_required_true_positive_122_demotes_in_both_modes_on_both_entries` (product, both entries, both modes).
- **Intended behaviour:** this is the D-5 fail-safe the design specifies (D1 §4.3.1; D5_TRIGGER), not a K-D5 defect.

## 3. Tests (all pass)

Adapter tests (`kd5_tests.rs`) run both modes. Measured values are "actual / trigger" as ratios to the criterion. The trigger value is 2|w|/criterion, and the quoted EF is half of it. All of these values come from the product's own u.

| Test (brief item) | Result |
|---|---|
| `kd5_required_true_positive_skew_cantilever_122_demotes_in_both_modes` (required true positive, adapter) | dense 2.413 / 4.827, sparse 1.214 / 2.428. Plain `solve` publishes Passed (precondition). Demoted; EF/actual − 1 < 1e-3 |
| `kd5_typed_entry_demotes_122_and_unselected_is_solve_assembled` | the same values through the typed entry; `selected = false` equals `solve_assembled` |
| PP `kd5_required_true_positive_122_demotes_in_both_modes_on_both_entries` (**both entries**) | captured and typed, both modes: `NUMERICAL_INTEGRITY_SENSITIVE`, `solve_quality: Sensitive`, report text `quality: Sensitive`, and no `FormationCheck` text in the diagnostics (no in-band marker) |
| `kd5_skew_345_and_invented_m11_do_not_demote`, and PP `kd5_skew_345_and_chain_continuity_controls_do_not_demote` | 345: 0.0059 / 0.0146 actual, not demoted. M11: not demoted (V1's first-order error ≤ 1.5e-4). PP: 345 and the six RF-CHAIN r1e-04 controls give `CHECKS_PASSED` on both entries and both modes |
| `kd5_d5c1_controls_demote_exactly_where_the_actual_error_exceeds_half_the_criterion` | probe D (solve error only): dense 2.650 / 5.301 demoted; sparse 2.0e-7, not demoted. Probe C (absorbed spring): dense 3.541 / 7.082, sparse 1.092 / 2.185, both demoted. Axis-aligned bending-soft: dense 0.134, not demoted; sparse 0.931 / 1.863 demoted. Each class demotes at least once; EF/actual within 1e-3 |
| Zero-scale clause, kernel level (N-2): `kd5_zero_scale_clause_fires_on_ledger_terms_that_differ_from_the_solve_force` | the solve force is 0, so u = 0; the ledger terms (1e80, −1e80, 1) give ρ = 1, S* = 0 and w = 1/2: fires (ratio +∞). Control: (1e80, −1e80) does not fire |
| R5-4: `kd5_realistic_elbows_e1_and_e6_do_not_demote` | E1 9.1e-4 / 1.9e-4, E6 5.7e-4 / 1.9e-3 actual; not demoted; report byte-identical |
| R5-4: `kd5_skew_plane_elbow_cantilever_at_kx_8_5_demotes_in_both_modes` | dense 1.481 / 2.963, sparse 1.096 / 2.191; demoted |
| R5-4 (added): `kd5_curved_intended_element_uses_the_actual_chord` | an admissible radius mismatch (9.2e-10): dense 0.840 / 1.685, sparse 0.869 / 1.742; demoted |
| R5-4: `kd5_expansion_joint_with_zero_lateral_does_not_demote_and_nonzero_lateral_fails_closed` | lateral 0: Passed, unchanged. Lateral 1e5: `user_stiffness_lateral_nonzero` (addendum 2, condition 4) |
| Seeded non-re-formable family, and option B: `kd5_unmatched_explicit_and_one_ulp_curved_slots_fail_closed`, `kd5_curved_matching_is_order_independent` | unmatched slot, explicit slot, and **one-ulp** slot matrix (ROOT condition 1) each give `formation_check_unavailable`; plain is Passed. Reversed macro order gives an identical result |
| Kernel fail-closed: `kd5_unavailable_family_and_wide_error_fail_closed_and_never_err`, `kd5_curved_arctangent_domain_errors_fail_closed` | seeded family; DOF-map mismatch; `AngleDomain` for collinear radii and for an included angle 5e-20 below π (addendum 3). Each is demoted and never an `Err` |
| `kd5_force_terms_entry_uses_the_identified_terms` | the audit-terms binding demotes 122; the load-fidelity report stays `None` |
| Callers and nonlinear pins (mutation 32): `kd5_nonlinear_loop_reaches_no_formation_check` (behavioural) and `kd5_nonlinear_sources_name_no_formation_check_entry_point` (lexed source) | **Behavioural:** first asserts the paths differ (the same system demotes through the check and is Passed through `solve`). Then the active-set loop (122 plus an open gap), both modes: its iteration is Passed and bitwise equal to `solve_binary64`. **Source:** comments, strings and `#[cfg(test)]` items are stripped, with a lexer self-control. It finds no formation entry point in `nonlinear_integration/src/lib.rs` or `product_equilibrium.rs`, and `assembly.solve_binary64(` is still present |
| Nonlinear-support cases never selected: `kd5_not_selected_invocation_runs_the_unchanged_solve`, and PP `kd5_nonlinear_support_invocation_is_never_selected` | `selected = false` equals `solve` bit for bit, while `selected = true` demotes (precondition). **PP:** 122 plus an open gap support: the loop converged, there is no `NUMERICAL_INTEGRITY_SENSITIVE`, and no receipt ordinary attempt is `sensitive`, while plain 122 demotes (precondition). **Byte identity with the base:** see §6 |
| D5C-3 | every non-demoting test asserts a Debug-identical report and bitwise displacements. §6 shows every committed raw byte-identical. `StructuralReport` is untouched |
| `kd5_frame_reformation_agrees_with_the_binary64_formation_to_roundoff`, `kd5_reformed_elements_have_the_rigid_body_null_space` | the frame matches binary64 to 1e-13 and is not equal to it. Frame, joint and curved elements annihilate an exact rigid motion to 1e-25. Curved cost: 2,499 `Wide` operations |

**The references.** `kd5_models.rs` is generated by `_run_records/models/kd5_models.py.txt`, which imports V1's `probe_d5_check.py` (d13cf7c8…) and D1's `curved_ef.py` (1c862cea…) unchanged. Each u_int is exact: Fractions for frames, and Decimal at 60 digits with the objective re-formation for curved elements. The tests use u_int to assert their precondition. The emulation's trigger values (`kd5_models_emulation.json`) equal the Rust values to every printed digit.

## 4. C2 result

Both scripts were run before and after the edit on `fixtures/product_preview` of four trees: numerics head, main `c61a540ea`, T1 `f3270ea79` and K3a `a2e804a75`.
- **All eight outputs are byte-identical before and after** (`_run_records/c2/before_after_sha256.txt`).
- On the numerics head they equal the committed `withheld_rows_merged.json` (f8e22724…) and `b_proof_main.json` (4a4a8603…).
- The committed tree has 2 hanger-only-stiffness supports; neither is in a selected case.
- Committed as `d61032f9d`.

## 5. Callers (`_run_records/callers.txt`, lexer scan `scan_callers.py.txt`)

100 call sites: 65 in tests and 35 outside tests, each classified.
- **The check** is reached only through a `PreparedSystem` built from a `FormationCheckedSystem`. Only `solve_with_formation_check` and `solve_assembled_with_formation_check` build one.
- **The only product call** is PP `solve_preview_reduced_system`. It is reached from `solve_load_case` for every case; `selected` is false with nonlinear supports.
- **The nonlinear loop** is unchanged, with the details below:
  - `solve_active_set_frame_with_mode_and_springs` builds `AssemblyEvidence`, which records the primitives but never uses them.
  - `solve_iteration_with_sliding_friction_evidence` → `solve_linearized_system_evidence`: the four option-(c) `_binary64` targets and `product_equilibrium::evaluate`.
  - `scrutinize_gaps` → `product_equilibrium::evaluate` (binary64).
  - None reaches a formation source.
- **Others:** `source_recovery` builds `AssemblyEvidence` without solving. `sparse_direct::solve_structural_sparse` is reached only through SA `solve`.

## 6. Fixture diff (`_run_records/fixture_diff/`)

- **Method:** S11-K's harness (`fixdiff_main.rs.txt`, ec089c1d…) was built unchanged against the candidate. Every committed JSON request or model under `P/fixtures`, `P/validation` and `P/core` ran through the captured entry in both modes. Each output's sha256 was compared with K3a's recorded base hashes; K3a changed no output, so those are the base's.
- **Result: 112 of 112 outputs identical** (fixtures 72/72, validation 30/30, core 10/10).
- **So:** no committed raw, derived document or hash pin changes, no committed fixture case demotes, and the curved-arc product tests keep their quality. The stop rule did not trigger.

**The nonlinear-support byte identity.** This is ROOT condition 2: the linear attempt and receipt of a case with a nonlinear support stay byte-identical to base.
- The committed request that solves through the nonlinear loop, `core/product_physics/tests/fixtures/preview_physics_invented_model.json`, is among the 112 identical outputs in both modes. It gives `NONLINEAR_SUPPORT_LOOP_CONVERGED` and `NUMERICAL_INTEGRITY_CHECKS_PASSED`, and every byte of its envelope, receipt included, equals the base.
- `fixtures/product_preview/invented_preview_model.json` also carries a nonlinear support, but it is blocked before the solve (`PRESSURE_MODEL_REAUTHOR_REQUIRED`).
- In code, `selected = false` returns the unchanged `solve`, and the adapter test asserts `==` with it.
- The PP test adds the paths-differ precondition on 122 plus a gap.

## 7. Mutations (`_run_records/mutations/`: `mutate.py.txt` patches, `run_mutants.sh.txt`, `MUTANTS.txt`, one log per mutant)

**Method:** each mutant is a fresh copy of `P/core/{solver,loads}` with the patch applied, run with `cargo test --offline --locked kd5` in `nonlinear_integration`, which also builds `frame_kernel`. Killers are the failing tests.

| Mutant (D1 §7.3) | Patch | Killed by |
|---|---|---|
| (23) disable the trigger | `Some(source) if false && …` | 122 true positive, D5C-1 controls, 8.5 elbow, joint, force-terms, not-selected, typed, nonlinear-loop precondition, and more |
| (26) binary64 published residual in place of ρ | ρ_i = fl(f_i − Σ K_ij u_j) | D5C-1 controls (probe D), 122, 8.5 elbow |
| (27) element-level ΔK plus binary64 residual (springs, assembly and average left out) | ρ = r64 + Σ_e (K_e,b64 − K_e,int)·u | D5C-1 controls (probe C), 122, E1/E6, 8.5 elbow, matching order, joint |
| (28) binary64 local coefficients | coefficients lifted from `local_stiffness` | D5C-1 controls (bending-soft), 122 |
| (31a) curved K_int = product's binary64 matrix | `shared` matrix lifted | 8.5 elbow, actual-chord test |
| **(31b) curved H from the product's chord** | the product's binary64 R(cos φ − 1), R sin φ | **M31b: equivalent at the criterion (ROOT, 2026-09-27, `c2042fd9c`)** |
| (31b0) the same chord formula evaluated at p | R, cos φ, sin φ at p | not killed (same reason) |
| (32a) the loop's `solve_binary64` routed through the check | body → `solve_with_formation_check(…, &[], true)` | `kd5_nonlinear_loop_reaches_no_formation_check` |
| (32b) the loop calls the new entry | `assembly.solve_binary64(` → `solve_with_formation_check(` | the behavioural pin and the lexed source pin |

**M31b: equivalent at the criterion (ROOT, 2026-09-27, `c2042fd9c`).** ROOT's conditions:
- The test `kd5_curved_intended_element_uses_the_actual_chord`, M31a's kill and the actual-chord implementation (R5-4 §2 step 6) stay required. The equivalence concerns the tests' power, not the code.
- RV-K-D5 will try to construct an admissible model on which M31b is observable. If it finds one, that model becomes a required test and M31b must be killed before merge.

The evidence:
- **CSKEW_8_5:** the mutant's trigger equals the correct check's to 4+ digits.
- **The added admissible-mismatch model:** the k_X = 30 skew elbow with its binary64 centre moved −6.5e-10 R along the chord. Then |ri| − |rj| = 9.2e-10 relative, inside the product's 1e-9 tolerance. The formula chord differs from the actual chord by 1.4e-10 m per component, confirmed in an instrumented mutant build. The triggers are 1.681 (mutant) against 1.685 (correct), dense: an EF shift of about 0.002 of the criterion.
- **Why:** a chord error δc gives the rigid-rotation force pair a net moment of second order in δc, so the soft mode barely sees it. Its first-order stiff-mode effect is far below 1e-9 relative.
- **M31b0**, the formula evaluated at p, behaves the same.
- **Consistent with V1:** VERIFY_R5 item 16 reproduced mutation 31 only through the shared matrix (0.254).
- **No test weakened.** `kd5_curved_intended_element_uses_the_actual_chord` was added and kept; it kills M31a.

The M31b investigation records are kept in `_run_records/mutations/` and `_run_records/models/probe_chord.py.txt`.

## 8. Suites (`_run_records/suites/`, one cargo job at a time, `--offline --locked`)

| Crate | Passed | Failed | Base (K3a record) |
|---|---|---|---|
| core/solver/frame_kernel (final re-run) | 140 | 0 | 135 (+5 K-D5) |
| core/solver/straight_pipe | 39 | 0 | 39 |
| core/solver/curved_bend | 25 | 0 | 25 |
| core/loads/load_case_algebra | 21 | 0 | 21 |
| core/solver/sparse_direct | 25 | 0 | 25 |
| core/solver/nonlinear_integration (final re-run) | 83 | 0 | 69 (+14 K-D5) |
| core/loads/primitive_loads | 49 | 0 | 49 |
| core/solver/linear_supports | 15 | 0 | 15 |
| core/solver/nonlinear_supports | 22 | 0 | 22 |
| core/solver/diagnostics | 24 | 0 | 24 |
| core/solver/performance_harness | 25 | 0 | 25 |
| core/loads/stress_recovery | 48 | 0 | 48 |
| core/loads/user_loads | 28 | 0 | 28 |
| core/loads/self_weight_wasm | 14 | 0 | 14 |
| core/product_physics | 451 (1 ignored, as base) | 0 | 448 (+3 K-D5) |
| core/model_operations/operation_applier | 194 | 0 | 194 |
| core/runner/headless (`--no-fail-fast`) | 83 | 0 | 83 |
| core/reporting/result_export | 91 | 0 | 91 |
| validation/benchmarks/mechanics | 41 | 0 | 41 |
| validation/benchmarks/nonlinear (DEC-046 limits untouched) | 19 | 0 | 19 |
| validation/benchmarks/stress | 23 | 0 | 23 |
| validation/benchmarks/physics_audit_regression | 15 | 0 | 15 |
| validation/benchmarks/numerical_integrity | 0 (observer; builds) | 0 | 0 |
| apps/desktop/src-tauri | 114 | 0 | 114 |

- **Later re-runs.** The full sweep ran on the candidate before three late changes: the K3a `wide.rs` restore, the near-π test, and the actual-chord test with its model. Frame_kernel and nonlinear_integration were re-run afterwards (counts above). None of the late changes touches another crate's inputs.
- **No fixture reader touched.** The Python and desktop TS suites were therefore not run, per the brief.
- **Builds are warning-free** in the changed crates.

## 9. The no-Passed-breach gate through both entries (`_run_records/gate/`)

**Method.** P1's own tools, unchanged:
- the probe `DETECTION/probe/main.rs.txt`, built `--release --offline` against the candidate;
- `run.py`'s `run_one`, with RLIMIT_AS 6 GiB, 600 s and 1800 s for ≥ 1000 members, and one fresh process per run;
- `compare.py`'s exact-rational predicate and trusted rule (checks_passed or numerically_eligible).

`gate_run.py.txt` drives the runs: every authorable case of P1's generated R1 requests (the generator re-run unchanged on `references.py` `c0f14201c`), both modes, both entries. It waits for idle cargo and sweeps before each run. `gate_check.py.txt` evaluates the result.

**Result: PASS.**
- 888 runs: 222 cases × 2 modes × 2 entries. 768 of them are on frozen-reference cases.
- 362 runs are published as trusted.
- There are **228 trusted breach triples** (captured 88, typed 140). Every one lies in the pinned exceptions:
  - `GATE/S11_EXCEPTIONS.json`: 221 triples, sha256 1d8979f6…;
  - `GATE/FORMATION_EXCEPTIONS.json`: 7 triples, sha256 454bbc24…, from ROOT's F12 ruling `db665f2cb`, which re-pinned those 7 formation-class triples out of the S11 file.
- There are **no violations**, and every pinned triple is still breached.
- The skew and curved cases are not exceptions. RF-SKEW-T-CANT-OFF-122-r1e-04 is Sensitive on both entries in both modes, so it is not a Passed breach (§3a).

**Standing against P1's main baseline.** Compared per entry (`standing_vs_p1.txt`) over the 540 runs that have a same-entry P1 record, **the only outcome or quality change is 122: checks_passed → sensitive**, on both entries and in both modes. Every refusal, timeout and memory refusal matches P1.

**Timeouts.** Four 1800 s timeouts, all pre-existing and identical in outcome to P1 on main:
- RF-LARGE-CHAIN-n01000-ROT dense, both entries;
- RF-LARGE-TREE-n01000-AX dense, both entries.

No timeout was raised. The 24 n10000 runs are memory-refused under RLIMIT_AS, as in P1.

**Cost at 1000 members.** Wall time, candidate against P1 on main, on a busier host:

| Case | Mode | Candidate | P1 on main |
|---|---|---|---|
| RF-LARGE-CONT-n01000-AX (Passed, so the check runs) | dense | 193 s | 204 s |
| RF-LARGE-CONT-n01000-AX (Passed, so the check runs) | sparse | 15 s | 17 s |
| RF-LARGE-CONT-n01000-ROT | dense | 194 s | 215 s |
| RF-LARGE-CHAIN-n01000-AX | dense | 430 s | 487 s |
| RF-LARGE-TREE-n01000-ROT | dense | 427 / 449 s | 451 s |

So the check's cost is within run-to-run noise even at 1000 members. P1's baseline of about 5,200 s per entry is matched: the large-case total per entry is essentially unchanged.

**Provenance and the reported container restart** (`provenance.txt`):
- ROOT reported a container restart during the run. The gate process survived it: uptime continued, and the runner started at 04:31:33 UTC stayed alive, with its probe child in RF-LARGE-CHAIN-n01000-ROT dense typed. Only my foreground wait shell was killed (exit 137).
- A second runner I started at 05:30:13 was waiting on cargo, wrote nothing, and was stopped at once.
- **So the gate is one continuous run of one binary.** Nothing is pre- or post-restart stitched, and nothing needed re-running.
- The probe binary's sha256 is 219d7128…. HEAD plus the sha256 and mtime of every uncommitted product path are recorded; all precede the build, and no product source changed after it.
- The final runs file has 888 lines and sha256 b7e883bf…. It is not committed; its hash is.

**Authority targets.** They were absent in kd5, and were rebuilt with `tools/serialization/build_checked_json.py` (both profiles) and `tools/units/build_units_authority.py`, one job at a time, after the gate. They are gitignored and were not cleaned.

## 10. Toolchain

`_run_records/toolchain.txt`:
- rustc and cargo 1.97.1, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, own target `<wt>/kd5-target`, `--offline --locked`;
- rustfmt 1.8.0 (stable), formatting only;
- Python 3.11.15 (the DEC-025 venv), standard library only.

## 11. What I did not do, and deviations

- **No Git write, except one disclosed working-tree command.** I ran `git checkout -- …/retained/wide.rs` to discard my own uncommitted edit to that file. It touched no index, commit or branch; ROOT accepted it as disclosed (addendum 4). No other Git write. The manager commits.
- **Untouched files:**
  - `CurvedBendStiffnessElement` and `nonlinear_integration/src/lib.rs`;
  - I1's `s11k_tests.rs`: my pins are a companion module, to be folded into I1's at the forward merge;
  - K3a's `wide.rs` and `wide_tests.rs` (byte-identical to the base);
  - any fixture, reference, schema or protected criterion.
- **No test was skipped, no timeout raised, and nothing weakened.**
- **Not run:** the Python and desktop TS suites (no fixture reader touched), hosted CI, and the DEC-025 sweep (ROOT's).
- **Base only.** The work is not yet merged forward onto main (K3a, PR983) or S11-F. At those merges:
  - per-item `#[allow(dead_code)]` (addendum 4);
  - PP switches to `solve_assembled_with_formation_check`;
  - the pins fold into I1's module;
  - the checks are re-run.
- **Stated limits (D1 §4.3.1):**
  - EF is first order with factor 2, not a bound. With a stiffness defect near 1e-9 relative, as in the admissible-mismatch elbow, it agrees with the actual error to a few per cent rather than 1e-5.
  - Load formation before S11-F, member-action and reaction recovery, and input representation are outside EF.
- **Wait loops.** An earlier wait loop of mine (`pgrep -f gate_run.py`) matched its own shell and only ended at its timeout. It is replaced by a pattern matching only the python process.
- **Scratch use.** Scratch output went under `<scratch>/kd5-i3` and was pruned. My own target was pruned twice on the disk floor. The authority targets were not touched.

## Addendum-4 pass: forward merge onto S11-F and PR1002 (2026-09-27)

- **Brief:** the manager's addendum-4 instructions, which follow RV3's `T3/REVIEW/S11F_REVIEW.md` §11 in numerics at `bf82f6cfd`.
- **Candidate:** `3befacff4` plus this pass's working tree.
  - `3befacff4` is the manager's merge of origin/main `72d5ff864` (PR1002) into the branch.
  - It sits on `8fd409e78`, the merge of `43b8f83aa` (S11-F, PR1000) onto K-D5 `17f3d6e05`/`b6156d49d`.
  - At `8fd409e78` the PP conflict was resolved to main's `solve_assembled`.
- **Diff against main `72d5ff864`:** 12 files, +2569 −15 in `P/core`, and no other product path.
- **Diff against `3befacff4`:** 8 files, +267 −405 (`git diff --shortstat`).

### Status

**All 8 items are done.** The verification pass found nothing to fix:
- **Suites:** every suite passes, 24 of 24 crates.
- **Fixture diff:** base main `72d5ff864` against the candidate, **112 of 112 byte-identical**. The stop rule did not trigger.
- **Mutations:** all as in phase 1.
- **Gate:** the both-entry gate against the exception lists as they stand on main now (numerics `59fff0d9e`: S11 empty, FORMATION 7 triples) **passes**. Its 7 trusted breach triples are exactly the 7 FORMATION triples, and none of the former 221 re-breaches. 122 is Sensitive on both entries in both modes.
- **Tree:** this is the gate on the **pre-S11-G tree** (`3befacff4` plus the addendum-4 edits), per ROOT's plan.

### A4-1. The 8 items

| # | Item | Where / how |
|---|---|---|
| 1 | Typed entry at the PP call | `solve_preview_reduced_system`: `assembly.solve_assembled(original_stiffness, global_force, …)` becomes `assembly.solve_assembled_with_formation_check(original_stiffness, global_force, &free, prescribed, mode, &curved_sources, built.nonlinear_supports.is_empty())`, where `global_force: &AssembledForce` and `curved_sources` comes from `built.curved_bend_elements`. ρ now starts from the ledger terms |
| 2 | `load_fidelity` | `PreviewLinearSolve.load_fidelity: checked.load_fidelity`, unchanged. SA's `solve_assembled_with_formation_check` returns the `StructuralSolution` of `solve_prepared(prepare_formation_checked_structural(&system)?, mode)`, whose `PreparedSystem` carries the typed binding, so the S11-F load audit and N5 run as in `solve_assembled` |
| 3 | Around S11-F's N5 block | `finish_checked_factor` keeps N5 byte-for-byte: an audit error gives `Some(LoadFidelityReport { rows: vec![], audit_error })`, and then `ordinary_sensitive`. The formation check runs after it, only when `prepared.formation` is `Some` and the case is still not `ordinary_sensitive`, i.e. it would publish Passed. A flagged or unauditable row therefore publishes Sensitive through N5, exactly as on main, and never reaches the check |
| 4 | I1's SA defining list | `option_c_structural_adapter_legacy_variants_reach_only_binary64_entry_points` gains `"fn solve_assembled_with_formation_check("`, the one new SA function that prepares an exact typed system (through `self.solve_assembled(` when not selected, and `prepare_formation_checked_structural(` when selected). Its doc names it. The legacy `&[f64]` `solve_with_formation_check` and FK's `prepare_formation_checked_structural_with_force_terms` are **removed**: with PP on the typed entry, neither has a caller. So no other SA function calls `prepare_assembled_structural(`/`prepare_structural(` |
| 5 | Product site test `rule_1` | `solve_preview_reduced_system` must contain `assembly.solve_assembled_with_formation_check(` and `built.nonlinear_supports.is_empty()`, and must not contain `assembly.solve_assembled(`. The comment names the behavioural backing, which carries the paths-differ preconditions: PP `formation_check_runtime.rs` (122 demotes on both entries, the controls do not, the nonlinear-support case is never selected while plain 122 demotes) and SA `kd5_not_selected_invocation_runs_the_unchanged_solve_assembled` (not selected equals `solve_assembled` bit for bit, while selected demotes) |
| 6 | Rule 6 `FORCE_FUNCTIONS` | `FK/structural/formation_check.rs` is appended to `KERNEL`, so indices 4 and 6 are unchanged. `check` and `evaluate` are listed. Rule 6 is exact (`actual == expected` over `FORCE_TOKENS`) and passes, so these two are every new FK/SA function that touches the force. The SA entry passes the `&AssembledForce` without reading it, and `formation_source`, `with_formation_source` and `binding()` read no force token |
| 7 | Nonlinear pins folded into `s11k_tests.rs` | I1's `EXACT_ENTRY_POINTS` and its pins are kept unchanged. Appended: `FORMATION_ENTRY_POINTS`; `kd5_nonlinear_sources_name_no_formation_check_entry_point`, which uses I1's `lex` and `strip_cfg_test` and has a lexer self-control; and `kd5_nonlinear_loop_reaches_no_formation_check`, whose precondition is that the same system is Sensitive through the check and Passed through `solve_assembled` (paths differ), after which the active-set loop, both modes, is Passed and bit-equal to `solve_binary64`. The companion lexer module and pins in `kd5_tests.rs` are removed |
| 8 | Per-item `#[allow(dead_code)]` on K3a's `wide.rs` | `retained/mod.rs` now carries no attribute on `mod wide;`, only a comment. The 13 `wide.rs` items with no non-test caller each carry `#[allow(dead_code)] // <reason>`: `ATAN_REGRESSION_TOLERANCE_TENTH_ULPS`, `ATAN_PROVED_BOUND_TENTH_ULPS`, `truncated_below_min_subnormal`, `rounded_operations` and `work` (test-only, or K4 budgets); `NotNormalized`, `from_parts`, `parts`, `is_sign_negative`, `exponent`, `fits_precision`, `precision` and `atan_positive` (K3 API for slice K3). `wide.rs` gains exactly those 13 lines, and the FK build has no warnings |

**Adapter tests after the conversion.** `kd5_tests.rs` now drives the typed entry only. `Built::ledger()` builds an `AssembledForce` from the model force and asserts bit equality. `plain()` is `solve_assembled`.
- **Adapter tests (10):** the 122 true positive, 345/M11, the D5C-1 controls, E1/E6, the 8.5 elbow, the actual chord, matching order, unmatched/explicit/one-ulp slots, the joint, and not-selected = `solve_assembled`.
- **In I1's module (2):** the two nonlinear pins.
- **Removed with their entries:** the force-terms test, the old not-selected test, and the separate typed-entry test, whose content is now the ordinary path.
- **Test counts:**
  - NI has +12 against main;
  - FK +5 (the kernel tests, unchanged);
  - PP +3 (`formation_check_runtime.rs`, unchanged).
- **All trigger values are identical to phase 1.** For example, 122 is 4.827 dense and 2.428 sparse, and CSKEW_8_5 is 2.963 and 2.191.

**One doc-comment edit after the verification builds.** In `structural_adapter.rs`, the `FormationPrimitives` doc still named the removed `solve_with_formation_check`. It now names `solve_assembled_with_formation_check`. The edit is comment-only (1 line becomes 2). It was made after the gate probe, fixture harness and mutation builds, and it changes no code. So `candidate_sources.txt` records the file as built (`87c7dc96…`); the committed file differs only in this comment (`c6379d80…`). The suites on the S11-G combined tree (ROOT's plan, step 3) re-run `nonlinear_integration` on it.

### A4-2. Suites (`_run_records/addendum4/suites/`)

The tree is `3befacff4` plus this pass. Cargo ran one job at a time with `--offline --locked`.
- The sweep yielded to RV3's PR1002 delta build: NI waited 120 s.
- PP and every crate after it ran after the manager's PR1002 merge. The PP log includes PR1002's two `s1_` tests.

| Crate | Passed | Failed |
|---|---|---|
| core/solver/frame_kernel | 144 | 0 |
| core/solver/straight_pipe | 39 | 0 |
| core/solver/curved_bend | 25 | 0 |
| core/loads/load_case_algebra | 21 | 0 |
| core/solver/sparse_direct | 25 | 0 |
| core/solver/nonlinear_integration | 86 | 0 |
| core/loads/primitive_loads | 49 | 0 |
| core/solver/linear_supports | 15 | 0 |
| core/solver/nonlinear_supports | 22 | 0 |
| core/solver/diagnostics | 24 | 0 |
| core/solver/performance_harness | 25 | 0 |
| core/loads/stress_recovery | 48 | 0 |
| core/loads/user_loads | 28 | 0 |
| core/loads/self_weight_wasm | 14 | 0 |
| core/product_physics | 479 (1 ignored: the existing `source_receipt` resource measurement) | 0 |
| core/model_operations/operation_applier | 194 | 0 |
| core/runner/headless (`--no-fail-fast`) | 83 | 0 |
| core/reporting/result_export | 91 | 0 |
| validation/benchmarks/mechanics | 41 | 0 |
| validation/benchmarks/nonlinear | 19 | 0 |
| validation/benchmarks/stress | 23 | 0 |
| validation/benchmarks/physics_audit_regression | 15 | 0 |
| validation/benchmarks/numerical_integrity | 0 (observer; builds) | 0 |
| apps/desktop/src-tauri | 114 | 0 |

The targeted runs before the sweep (`fk`, `NI`, PP `formation_check_runtime` 3/3 and `s11f_site_test` 9/9) are in the same folder.

### A4-3. Fixture diff (`_run_records/addendum4/fixture_diff/`)

- **Method:** S11-K's harness (`ec089c1d…`, unchanged) was built twice: against main `72d5ff864` (a `git archive` of core, fixtures, validation and schemas) and against the candidate. The two harness binaries differ.
- **Inputs:** every committed JSON request or model under `P/fixtures`, `P/validation` and `P/core`, in both modes. The committed inputs are identical in the two trees.
- **Result: 112 of 112 outputs byte-identical** (fixtures 72/72, validation 30/30, core 10/10). This includes the committed nonlinear-support request `preview_physics_invented_model.json` (ROOT condition 2) and the 6 outputs that are `ERR` on both trees.
- **So:** no committed raw, derived document or hash pin changes, and the stop rule did not trigger.
- **Four outputs differ from phase 1's recorded base hashes:** `model_operations/physics_thermal_ui_model` and `product_preview/load_reference/connected.request`, both modes. These changed on main with S11-F; main and the candidate agree on them.

### A4-4. Mutations (`_run_records/addendum4/mutations/`)

**Method:** the phase 1 runner and patches are unchanged in method. Each mutant is a fresh copy of `P/core/{solver,loads}` from the candidate tree, run with `cargo test --offline --locked kd5` in `nonlinear_integration`. The patches were re-anchored to the typed entry:
- M32a routes `solve_binary64` through `self.formation_source(&[])` and `prepare_formation_checked_structural`;
- M32b makes the loop build a ledger and call `solve_assembled_with_formation_check`.

| Mutant | Result | Killed by |
|---|---|---|
| (23) trigger disabled | killed | 122 true positive, D5C-1 controls, 8.5 elbow, actual chord, joint, one-ulp slots, not-selected precondition, and the nonlinear-loop precondition |
| (26) binary64 residual | killed | 122, D5C-1 controls, 8.5 elbow, actual chord |
| (27) element ΔK plus binary64 residual | killed | 122, D5C-1, E1/E6, 8.5 elbow, actual chord, matching order, joint, one-ulp slots |
| (28) binary64 local coefficients | killed | 122, D5C-1 controls |
| **(31a) curved K_int = product's matrix** | **killed** | **`kd5_curved_intended_element_uses_the_actual_chord`** and the 8.5 elbow |
| (31b) H from the product's chord | survives | **M31b: equivalent at the criterion (ROOT, `c2042fd9c`)**, recorded as in phase 1 §7 |
| (31b0) the same chord at p | survives | the same reason |
| (32a) `solve_binary64` routed through the check | killed | `s11k_tests::kd5_nonlinear_loop_reaches_no_formation_check` |
| (32b) the loop calls the typed formation entry | killed | `kd5_nonlinear_loop_reaches_no_formation_check` and `kd5_nonlinear_sources_name_no_formation_check_entry_point` |

### A4-5. The no-Passed-breach gate through both entries (`_run_records/addendum4/gate/`)

**Exception lists, as on main now.** Both come from numerics commit **`59fff0d9e`** ("T3 empty the S11 gate exception list at the S11-F merge"), each verified against `git show 59fff0d9e:…`:
- `GATE/S11_EXCEPTIONS.json`: **0 triples**, sha256 `138515b3e1f95f96…`;
- `GATE/FORMATION_EXCEPTIONS.json`: **7 triples**, sha256 `454bbc24bcbe5e1b…`;
- `REFERENCES/references.json`: `7b176dbbf2296be0…`.

Any trusted breach outside the 7 formation triples, including a re-breach of any of the former 221, is a FAIL.

**Method.** This is phase 1's method, with P1's probe, `run.py` and `compare.py` unchanged:
- the probe (`8dc727f4…`) was rebuilt `--release --offline` against the candidate, giving binary sha256 `88bcc841…`, with candidate sources recorded in `candidate_sources.txt`;
- all 222 authorable cases × 2 modes × 2 entries, each in a fresh process with P1's limits;
- written to a fresh runs file.

**Result: PASS.**
- **Runs:** 888, which is 222 cases × 2 modes × 2 entries. 768 are on frozen-reference cases, and 362 of those are published as trusted.
- **Trusted breach triples: 7** (captured 1, typed 6). They are **exactly the 7 triples of `FORMATION_EXCEPTIONS.json`** (its 14 rows over both modes). There are no violations, and no pinned triple is left unbreached.
- **Nothing outside FORMATION.** No trusted breach lies outside the FORMATION list, so none of the former 221 S11 triples re-breaches on either entry or in either mode.
- **RF-SKEW-T-CANT-OFF-122-r1e-04** (K-D5's required true positive) is `sensitive`, standing `needs_recompute`, with `NUMERICAL_INTEGRITY_SENSITIVE`, on **both entries in both modes** (captured sparse, typed sparse, captured dense, typed dense). It is not a Passed breach and not an exception.

**Standing against P1's main baseline** (`standing_vs_p1.txt`, the 540 runs with a same-entry P1 record):
- **The only outcome or quality change is 122: checks_passed → sensitive**, on both entries in both modes, the same as in phase 1.
- S11-F and PR1002 changed no standing on these runs; their fixes changed values, which is why the former 221 no longer breach.
- So on the pre-S11-G tree the only standing change K-D5 makes is the required true positive.
- Every refusal, timeout and memory refusal matches P1.

**Timeouts and limits:**
- Four 1800 s timeouts, all existing and matching P1 on main: RF-LARGE-CHAIN-n01000-ROT dense and RF-LARGE-TREE-n01000-AX dense, both entries.
- The 24 n10000 runs are memory-refused under RLIMIT_AS, as in P1.
- No timeout was raised.

**Cost at 1000 members** (wall time):

| Case | Mode | Candidate (this pass) | Phase 1 | P1 on main |
|---|---|---|---|---|
| RF-LARGE-CONT-n01000-AX (Passed, so the check runs) | dense | 190 / 191 s | 193 s | 204 s |
| RF-LARGE-CONT-n01000-AX | sparse | 16 / 16 s | 15 s | 17 s |
| RF-LARGE-CONT-n01000-ROT | dense | 196 / 191 s | 194 s | 215 s |
| RF-LARGE-CHAIN-n01000-AX | dense | 413 / 407 s | 430 s | 487 s |
| RF-LARGE-TREE-n01000-ROT | dense | 423 / 425 s | 427 / 449 s | 451 s |

**Provenance and duration:**
- **Run:** one continuous run of one binary, started 10:13 UTC and finished 14:01 UTC.
- **Pauses:** the runner held while ROOT's DEC-025 evidence sweep and other agents' cargo jobs ran, about 40 minutes in all.
- **Run time:** the runs total 10,094 s:
  - 836 small runs, 43 s;
  - 28 runs at 1000 members, 9,885 s, of which the 4 timeouts are 7,200 s;
  - 24 memory refusals, 166 s.
- **Runs file:** `runs.jsonl` has 888 lines, sha256 `9b374786…`. It is not committed; its hash is.
- **Records:** `gate_result.json` and `standing_vs_p1.txt` are committed.
