# I3 return: slice K-D5, the D-5 formation check

Type 2 TASK I3, 2026-09-27, for the T3 manager.
- **Brief:** `TASK_BRIEFS/I3_KD5_IMPLEMENTATION.md` with addenda 1–4, read with `_COMMON.md`.
- **Worktree:** `<wt>/kd5`, branch `codex/piping-kd5-20260926`.
- **Base:** the K3a head `a2e804a757359d589f4c31ea8e36a923f28ccb8c`, on the S11-K head `4912dc636`.
- **Records:** `T3/IMPLEMENTATION/KD5/`, run records in `_run_records/`, hashes in `SHA256SUMS`.

> **Current state (2026-09-27, latest):** the combined-tree pass (K-D5 with S11-G) and the RV5 repair are recorded at the end, in **"Combined-tree pass and RV5 repair"**, with records in `_run_records/combined/` and `_run_records/repair/`. The earlier state notes below are kept as recorded.
>
> **Earlier state:** the addendum-4 pass on the pre-S11-G tree (`3befacff4` plus the addendum-4 edits) is complete. See **"Addendum-4 pass"** at the end, with records in `_run_records/addendum4/`.
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

**Superseded (2026-09-27):** ROOT withdrew the M31b equivalence on RV5's counterexample, confirmed in Rust (`ROOT_RULINGS_V1.md`, "K-D5 mutation M31b: equivalence withdrawn", numerics `3547029576`). M31b and M31b0 are now killed by required tests; see "Combined-tree pass and RV5 repair", R-1 and R-3.

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

## 5. Callers (`_run_records/combined/callers_combined.txt`, lexer scan `repair/callers/scan_callers.py.txt`)

> **Corrected in the RV5 repair (R-4).** This section first described phase 1: 100 call sites, 65 in tests and 35 outside, with the legacy `solve_with_formation_check` called at PP:3973. That entry and FK's `_with_force_terms` variant were removed in the addendum-4 pass. The list below was regenerated on the combined candidate and added as the new file `_run_records/combined/callers_combined.txt`, which supersedes phase 1's list. The phase 1 files `_run_records/callers.txt` (sha256 `cd8e2ae1…`) and `scan_callers.py.txt` describe phase 1; they are hash-bound and kept unchanged as recorded.

116 call sites: 77 in tests and 39 outside tests, each classified.
- **The check** (`formation_check::check`) is called only from `finish_checked_factor`. It runs only when the `PreparedSystem` carries a formation source and the case is not already ordinary-Sensitive. Only `prepare_formation_checked_structural` attaches a formation source, and only from a `FormationCheckedSystem`.
- **The builder.** SA's typed `solve_assembled_with_formation_check` is the only non-test builder of a `FormationCheckedSystem`; `selected = false` returns `solve_assembled`. FK's `solve_formation_checked_structural_dense` has no non-test caller.
- **The only product call** is PP:4399 in `solve_preview_reduced_system`, with `selected = built.nonlinear_supports.is_empty()`. It is reached from `solve_load_case` (PP:2708) for every case.
- **The nonlinear loop** is unchanged:
  - `solve_active_set_frame_with_mode_and_springs` builds `AssemblyEvidence`, which records the primitives but never uses them, and runs `scrutinize_gaps`.
  - `solve_iteration_with_sliding_friction_evidence` → `solve_linearized_system_evidence`: the four option-(c) `_binary64` targets and `product_equilibrium::evaluate`.
  - `scrutinize_gaps` → `product_equilibrium::evaluate` (binary64).
  - None of them reaches a formation source. The nonlinear pins in `s11k_tests` enforce this: the whole-crate source pin and two behavioural pins, one on the first iteration and one on the derived-friction unit-force solves over several iterations (mutations 32a and 32b, and RV5's E4).
- **Others:** `source_recovery::prepare_sources` builds `AssemblyEvidence` without solving. `sparse_direct::solve_structural_sparse` and `finish_structural` are the unchanged completion paths.

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
| **(31b) curved H from the product's chord** | the product's binary64 R(cos φ − 1), R sin φ | **M31b: equivalent at the criterion (ROOT, 2026-09-27, `c2042fd9c`)**. *Superseded: now killed (R-3)* |
| (31b0) the same chord formula evaluated at p | R, cos φ, sin φ at p | not killed (same reason). *Superseded: now killed (R-3)* |
| (32a) the loop's `solve_binary64` routed through the check | body → `solve_with_formation_check(…, &[], true)` | `kd5_nonlinear_loop_reaches_no_formation_check` |
| (32b) the loop calls the new entry | `assembly.solve_binary64(` → `solve_with_formation_check(` | the behavioural pin and the lexed source pin |

**Superseded (2026-09-27):** ROOT withdrew the M31b equivalence on RV5's counterexample, confirmed in Rust (`ROOT_RULINGS_V1.md`, "K-D5 mutation M31b: equivalence withdrawn", numerics `3547029576`). M31b and M31b0 are now killed by required tests; see "Combined-tree pass and RV5 repair", R-1 and R-3. RV-K-D5 (RV5) found the admissible model the condition below anticipated. The M32a and M32b rows above also ran on a runner that could reuse a stale `frame_kernel` build (R-3); the clean re-run supersedes them.

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
| (31b) H from the product's chord | survives | **M31b: equivalent at the criterion (ROOT, `c2042fd9c`)**, recorded as in phase 1 §7. *Superseded: now killed (R-3)* |
| (31b0) the same chord at p | survives | the same reason. *Superseded: now killed (R-3)* |
| (32a) `solve_binary64` routed through the check | killed | `s11k_tests::kd5_nonlinear_loop_reaches_no_formation_check` |
| (32b) the loop calls the typed formation entry | killed | `kd5_nonlinear_loop_reaches_no_formation_check` and `kd5_nonlinear_sources_name_no_formation_check_entry_point` |

**Superseded (2026-09-27):** ROOT withdrew the M31b equivalence on RV5's counterexample, confirmed in Rust (`ROOT_RULINGS_V1.md`, "K-D5 mutation M31b: equivalence withdrawn", numerics `3547029576`). M31b and M31b0 are now killed by required tests; see "Combined-tree pass and RV5 repair", R-1 and R-3. The M32a and M32b rows above ran on the runner that could reuse a stale `frame_kernel` build, i.e. against a `frame_kernel` carrying M31b0; the clean re-run (R-3) supersedes this table.

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


## Combined-tree pass and RV5 repair (2026-09-27)

This addendum was written by I3R, the replacement TASK for I3 after a container restart, from I3's on-disk state (brief `TASK_BRIEFS/I3R_KD5_REPAIR_RESUME.md`, numerics `9c266ec77`). I3's uncommitted drafts were read in full, checked, and completed; nothing was taken on trust.

- **Tree:**
  - `a89fde17b`: the addendum-4 work, committed by the manager;
  - `fbc9661a4`: the manager's merge of origin/main `b24b3d536` (S11-G, PR1003). It was textually clean, so there was nothing to resolve by hand;
  - `2409de83e`: the T20 doc-comment correction (C-2), committed on its own;
  - the repair (R-1 to R-7): tests, generated test models and records only, on `2409de83e`. No product source changes.
- **Records:** the combined-tree evidence is in `_run_records/combined/`, and the repair's in `_run_records/repair/`. Machine paths are replaced by `<wt>`, `<scratch>` and similar placeholders. No committed, hash-bound record was rewritten: new evidence is in new files, including the regenerated caller list (R-4). Only `SHA256SUMS` is refreshed.

### Status

**Combined tree:** complete and green.
- The suites pass, 24 of 24, on `fbc9661a4`.
- T9 against main `b24b3d536` is 112 of 112 byte-identical.
- The gate union **PASSES**: 888 runs, with 0 trusted breaches against main's empty lists.
- Against main only 122 differs, and the 34 moves against the pre-S11-G gate are S11-G's own.

**Repair:** complete and green, with tests and records only.
- The required M31b and M31b0 kills are in, at behavioural assertions, on the adapter and at product level on both entries.
- The large-coordinate product control does not demote.
- RV5's E4 is closed by strengthened nonlinear pins.
- The clean mutation re-run kills all 10 mutants (M23, M26, M27, M28, M31a, M31b, M31b0, M32a, M32b and E4), and the no-patch control passes.
- The NI suite passes 89 of 89 and the PP suite 516 of 516, with 1 ignored as before.
- GEN-8 passes on the repair tree with the records in place.

### C-1. Composition check (before any run)

The manager relayed the check; it was accepted with no design decision needed.
- **Untouched by S11-G.** S11-G left FK `structural.rs`, `structural/*` (formation_check, retained), all of `nonlinear_integration` (SA included) and `sparse_direct` alone: `git diff 72d5ff864 b24b3d536` over them is empty. PP's `solve_preview_reduced_system`, with the K-D5 call, is byte-identical to `a89fde17b`'s.
- **The integrity diagnostic.**
  - `append_integrity_report` takes its code from `report.quality`, which carries K-D5's kernel Sensitive.
  - S11-G's `formation_guard::demote` returns at once unless the code is `NUMERICAL_INTEGRITY_CHECKS_PASSED`. R-b′'s `amend_integrity_report` uses the same `demote`.
  - So a K-D5-Sensitive case gets no guard sentence and no byte change, and the two demotions cannot stack. K-D5 adds no text of its own (D5C-3).
- **Routing.**
  - `report_sensitive` reads `attempted_linear`'s `structural_report.quality`, which is `checked.report`. So a K-D5-Sensitive case routes as Sensitive.
  - D22-1's `formation_decline_without_attempt` fires only when `needs_source_recovery(report_sensitive, attempt_err, None)` is false (report Passed, no Err). A K-D5-Sensitive case therefore always gets main's real attempt and its receipt entry.
  - A case on which S11-G's load-row guard also fired is declined by G-3's `decline_for_formation` after the attempt, exactly as main handles any ordinary-Sensitive case with a finding.
- **Site test.** `git diff b24b3d536 HEAD` of `tests/s11f_site_test.rs` is exactly K-D5's three hunks: the KERNEL append, two `FORCE_FUNCTIONS` rows, and rule 1. S11-G's PRODUCT entry, `push_formed` counting, rule 5 edit, T8 and T10b are intact, and KERNEL indices 4 and 6 are unchanged. Rule 6's exact union passes in the PP suite.
- **Ledger API.** S11-G's `load_ledger.rs` changes are additive. `formation_check.rs` uses only `ForceTerm`, which is unchanged.

### C-2. The T20 doc-comment correction (`2409de83e`)

This was approved by ROOT and relayed by the manager.
- **The change.** In `product_physics/src/s11g_tests.rs`, T20's doc comment (`t20_characterization_rb_prime_residual_c1`) said the residual "needs per-case modulus bases and a pre-0.4 captured invocation". It now says:
  - C1 uses per-case modulus bases;
  - reach with a single modulus basis (through FK's load audit, or through K-D5's formation check) is not refuted;
  - the residual needs a pre-0.4 captured invocation.
- **Scope.** +6 −4, comment-only: no code or assertion change. The manager verified that every changed line is a comment.
- **Timing.** It was written after the suite run on `fbc9661a4`. T9 and the gate ran on `2409de83e`.

### C-3. Suites on `fbc9661a4` (`_run_records/combined/suites/`)

Every crate passes, 24 of 24, with 0 failures. This run also covers the addendum-4 doc-comment edit to `structural_adapter.rs` (A4-1).

| Crate | Passed | Crate | Passed |
|---|---|---|---|
| frame_kernel | 149 | self_weight_wasm | 14 |
| straight_pipe | 42 | product_physics | 514 (1 ignored: the existing `source_receipt` measurement) |
| curved_bend | 25 | operation_applier | 194 |
| load_case_algebra | 21 | headless (`--no-fail-fast`) | 84 |
| sparse_direct | 25 | result_export | 91 |
| nonlinear_integration | 86 | mechanics | 41 |
| primitive_loads | 49 | nonlinear (DEC-046 limits untouched) | 19 |
| linear_supports | 15 | stress | 23 |
| nonlinear_supports | 22 | physics_audit_regression | 15 |
| diagnostics | 24 | numerical_integrity | 0 (observer; builds) |
| performance_harness | 25 | src-tauri | 114 |
| stress_recovery | 48 | | |
| user_loads | 28 | | |

### C-4. T9: the fixture diff against main `b24b3d536` (`_run_records/combined/fixture_diff/`)

- **Method:** S11-K's harness (`ec089c1d…`, unchanged) was built against main `b24b3d536` (a `git archive`) and against `2409de83e`. The two binaries differ.
- **Inputs:** every committed JSON request or model under `P/fixtures`, `P/validation` and `P/core`, in both modes. The committed inputs are identical in the two trees.
- **Result: 112 of 112 outputs byte-identical** (fixtures 72, validation 30, core 10), including the 6 outputs that are `ERR` on both trees.
- **Against the pre-S11-G pass,** none of the 112 outputs changed.
- **So:** no committed byte changes, and the stop rule did not trigger.

### C-5. The gate on the combined tree: union verdict PASS (`_run_records/combined/gate/`)

**Lists (main's, now):** both from `b24b3d536`:
- `GATE/S11_EXCEPTIONS.json`: empty, `138515b3…`;
- `GATE/FORMATION_EXCEPTIONS.json`: empty, `0e110b4b…`;
- `REFERENCES/references.json`: `7b176dbb…`.

**Zero trusted breaches are allowed.**

**Method.**
- P1's probe (`8dc727f4…`), `run.py` and `compare.py` are used unchanged.
- The probe was rebuilt on `2409de83e` with a clean tree: binary sha256 `39791d93…`.
- `gate_run_parts.py.txt` is `gate_run.py` plus a part filter (ROOT's ruling, numerics `8fcf14d7a`):
  - Part 1 is every run except the 4 known dense timeouts: RF-LARGE-CHAIN-n01000-ROT and RF-LARGE-TREE-n01000-AX, dense, both entries.
  - Part 2 is those 4, run on a quiet host with no other cargo, with the load average recorded at the start and end of each run.
- Both parts use one binary and one runs file. The verdict is the union.

**Part 1: PASS.**
- 884 runs, from 14:59 to 15:54 UTC. 764 are on frozen-reference cases, and 328 of those are trusted.
- Trusted breach triples: 0 (captured 0, typed 0).

**Part 2: the 4 dense runs time out at 1800 s, as on main and in every earlier pass.** The host was quiet: the load average stayed between 0.46 and 1.27 at every start and end (`gate_part2.log`). No timeout was raised.

**The union: PASS.**
- 888 runs, which is 222 cases × 2 modes × 2 entries. 768 are on frozen-reference cases, and 328 of those are trusted.
- **0 trusted breach triples** against main's empty lists. There are no violations.
- The runs file has 888 lines, sha256 `e365541b…`. It is not committed; its hash is (`provenance.txt`).
- **RF-SKEW-T-CANT-OFF-122-r1e-04** is `sensitive` / `needs_recompute` with `NUMERICAL_INTEGRITY_SENSITIVE` on **both entries in both modes**.
- **RV5's recount** agrees: 888 runs, none missing, the same 4 timeouts, and 836 runs compared with main (numerics `T3/REVIEW/_run_records/kd5_review/gate/rv5_gate_recount.txt`).

### C-6. Attribution against main `b24b3d536`: only 122 differs

- **Method:** the P1 probe was built against main itself (sha256 `12811c32…`), and all 836 runs under 1000 members were run on it, on both entries and in both modes (`gate_run_attrib.py.txt`, `main_small.log`, `main_small_result.json`).
- **Result:** the candidate differs from main in exactly one case, **122**: `checks_passed` / `numerically_eligible` on main and `sensitive` / `needs_recompute` on the candidate, in all 4 runs. I re-derived this independently (`combined/gate/main_vs_candidate.stdout.txt`). No other outcome, quality or standing differs, and **no published displacement differs in any of the 836 runs**.
- **So:** on main alone, the empty-list gate **FAILS** on 122's 8 trusted triples (4 per entry: th.N0.RX, th.N1.RX, u.N1.UY, u.N1.UZ; `main_small_result.json`). K-D5 removes exactly those, and changes nothing else on these runs.

### C-7. The 34 S11-G-attributed moves

Against the pre-S11-G gate (A4-5), **34 runs moved from `checks_passed` / `numerically_eligible` to `sensitive` / `needs_recompute`**, with their published displacements identical. I re-derived the list from the two runs files (`combined/gate/moves_vs_pre_s11g.stdout.txt`). They cover 10 cases:
- **The cases of the 7 former FORMATION triples, 10 runs:** RF-CANCEL-F-G1e80-GnG-INPLANE and RF-CANCEL-M-G1e80-GnG-INPLANE (typed, both modes), RF-CANCEL-UDL-W1e8 (both entries, both modes), and RF-CANCEL-UDL-W1e80 (typed, both modes).
- **RF-INVARIANCE-LFRAME-{BASE, OFF-1e3, OFF-1e6, RELABEL},** on both entries in both modes: 16 runs.
- **RF-WEAK-W-{3D, AX}-rho1e-08,** on both entries in both modes: 8 runs.

**ROOT has accepted these as S11-G's forecast demotions.** The FORMATION cases are the ones S11-G was built to demote. LFRAME×4 and WEAK-W-3D/AX-rho1e-08 are R-b′'s disclosed false demotions. Main makes the same 34 moves (C-6). So K-D5 causes none of them.

Against P1's baseline (`final_standing_vs_p1.txt`, 540 runs with a same-entry P1 record), the changes are the subset of these moves that P1 recorded, plus 122.

### C-8. Timing (`_run_records/combined/timing/`)

**Combined tree against the pre-S11-G tree.** At 1000 members the dense runs of the combined gate took about 15% longer than those of the pre-S11-G gate (A4-5), on a quiet host:

| Case | Mode | Combined | Pre-S11-G |
|---|---|---|---|
| RF-LARGE-CONT-n01000-AX (Passed, so the check runs) | dense | 212 / 210 s | 190 / 191 s |
| RF-LARGE-CONT-n01000-ROT | dense | 217 / 208 s | 196 / 191 s |
| RF-LARGE-CHAIN-n01000-AX | dense | 490 / 481 s | 413 / 407 s |
| RF-LARGE-TREE-n01000-ROT | dense | 479 / 475 s | 423 / 425 s |

Each cell gives captured / typed. The sparse runs are 17–21 s, against 16–23 s before.

**Main against the candidate, timed and interleaved** (ROOT's request; `timing_compare.py.txt`, `timing.jsonl`). The main probe (`12811c32…`, main `b24b3d536`, S11-G without K-D5) and the candidate probe (`39791d93…`) ran in the order main, candidate, main, candidate on each case. Each run was dense, on the captured entry, in a fresh process with P1's limits, with the load average recorded:

| Case | Rep | Main (S11-G, no K-D5) | Candidate (S11-G + K-D5) | Load average, start → end of the pair |
|---|---|---|---|---|
| RF-LARGE-CHAIN-n01000-AX | 1 | 493.9 s | 474.2 s | 1.18 → 1.01 |
| RF-LARGE-CHAIN-n01000-AX | 2 | 495.4 s | 485.2 s | 1.01 → 1.33 |
| RF-LARGE-TREE-n01000-ROT | 1 | 498.1 s | 470.8 s | 1.33 → 1.08 |
| RF-LARGE-TREE-n01000-ROT | 2 | 488.9 s | 469.1 s | 1.08 → 1.00 |

All 8 runs completed with no timeout. Peak RSS was 3.69–3.71 GB in every run, and every run published `sensitive` / `needs_recompute` on both binaries. The load averages are the 1-minute values.

**Reading.**
- **K-D5 adds no measurable cost on these cases.** On the means, the candidate was 3.0% faster than main on CHAIN-AX and 4.8% faster on TREE-ROT, and it was faster in every pair. These two cases publish Sensitive, so the check does not run on them. The comparison shows that K-D5 leaves the non-Passed path unchanged.
- **The increase belongs to S11-G.** Both S11-G-bearing probes take about 470–495 s on these cases: main (S11-G without K-D5) takes 489–498 s, and the candidate 469–485 s. The pre-S11-G K-D5 tree took 407–425 s in A4-5. So the increase over the pre-S11-G tree is not K-D5's.
- **This is recorded as an S11-G performance finding** for the manager to route: S11-G costs about 15–20% on dense 1000-member solves.
- **Caveat.** The pre-S11-G numbers were not interleaved with these runs; they come from the A4-5 gate on a different occasion. Part of the difference may therefore be host variation.

**K-D5's own cost is well under 1 s per case.** Over the 836 runs under 1000 members (`combined/gate/wall_main_vs_candidate.stdout.txt`):
- On the 382 runs where the candidate published Passed, so the check ran, the median per-run difference from main is −0.0004 s and the largest is +0.081 s. The totals are 15.04 s for the candidate and 15.54 s for main.
- On the other 454 runs, the median difference is −0.0004 s and the largest is +0.043 s.
- These wall times include process start and JSON I/O.

### R-1. M31b: the equivalence is withdrawn, and M31b and M31b0 are now killed

**Superseded.** Phase 1 §7, A4-4 and CHANGE_RECORD claimed that "(31b) is equivalent at the criterion" (ROOT, `c2042fd9c`). **That claim is superseded.** ROOT withdrew the equivalence after RV5's Rust confirmation on `2409de83e`. The record is `ROOT_RULINGS_V1.md`, section "K-D5 mutation M31b: equivalence withdrawn" (numerics `3547029576`), and the `c2042fd9c` section is marked SUPERSEDED there. That section also supersedes DESIGN.md 5a.2 §9 item 31's claim that building H from the product's chord misses only the whole-matrix case. The hash-pinned design itself is not edited. **The emulation in my generator gives the same values as RV5's Rust run:** CANT60 actual 1.1019, trigger 2.2037; CANT30 1.9005, trigger 3.8010; PP_UTM at 5e6 m, φ = 5°, 1.126. RV5's release probes also show the product-level flip: under M31b the 5e6 m case publishes CHECKS_PASSED in all 4 runs, with byte-identical results. The earlier text is kept where it was recorded and is marked superseded there.

**Why the claim failed.** RV5-B1 found that the M31b chord-only mutant survives and that an admissible counterexample exists. The phase 1 argument measured only the max-row trigger shift, about 0.002, on the k_X = 30 mismatch model. It missed the **first-order** translation error that a chord error gives at node j on the stiff rows. ROOT: "No blame attaches to I3's analysis. It answered the question I asked, which was the wrong question."

**The exact references.** They come from the K-D5 generator (`repair/models/kd5_models.py.txt`), not from RV5's files.
- It uses D1's objective curved re-formation (`curved_ef.py`, `1c862cea…`, imported unchanged) at 60 digits, on **exact** binary64 inputs: `dec` converts each input through `Fraction`, not through its decimal repr.
- I re-ran it on the candidate: the generated `kd5_models.rs`, the JSON and the stdout are byte-identical to the drafts.
- The generated u_int of CPLANAR_60 and CSKEW_30_N122, and the product-section u_int of the 5e6 m elbow, equal RV5's independently generated exact-input references (`kd5_review/m31b/exact_inputs/rv5_models.rs.txt`) to every printed digit.
- The emulated product centre (`pp_centre`) and section (π(od² − id²)/4, π(od⁴ − id⁴)/64 with id = od − 2t) follow PP's binary64 operations in the same order (PP `lib.rs` 6102–6110 and 8472–8476).

**The required tests** (all test-only):

| Test | Level, entries, modes | What it asserts |
|---|---|---|
| `kd5_admissible_centre_mismatch_demotes_where_the_product_chord_hides_the_error` | adapter, one entry, both modes | on RV5's admissible given centres, CPLANAR_60 (planar 60°) and CSKEW_30_N122 (skew 30°), each at the edge of the product's 1e-9 radius-match tolerance. **Preconditions:** plain `solve_assembled` publishes Passed, and the actual error is above the criterion. **Behavioural:** demoted only in quality (values and the rest of the report bitwise equal), and EF within 5% of the actual error |
| `kd5_large_coordinate_pp_route_elbow_does_not_demote` | adapter, both modes | PP_UTM_2 (X = 5e5 m, φ = 2°, PP's own centre): Passed, actual error below half the criterion, and the checked solution byte-identical to the plain one |
| `kd5_very_large_coordinate_pp_route_elbow_demotes_on_both_entries` | product, both entries, both modes | ROOT's required demotion test: X = 5e6 m, Y = 3.5e6 m, R = 0.3 m, φ = 5°, PP's own centre and derived section. **Precondition:** M1 is realized as an arc, and the published error is above the criterion. **Behavioural:** `NUMERICAL_INTEGRITY_SENSITIVE` and `Sensitive` quality |
| `kd5_large_coordinate_pp_route_elbow_is_published_accurately_and_not_demoted` | product, both entries, both modes | ROOT's control: X ≈ 5e5 m, φ = 2°. The published error is below half the criterion, and the case publishes `NUMERICAL_INTEGRITY_CHECKS_PASSED` with `ChecksPassed` quality |

The adapter cannot express the designed centres as product requests, because PP computes a bend's centre itself. The product-level, both-entry evidence is therefore the PP-route pair.

**Measured values** (actual error, and the trigger value 2|w|/criterion, as ratios to the criterion):

| Model (test) | Entry | Dense: actual / trigger | Sparse: actual / trigger | Result |
|---|---|---|---|---|
| CPLANAR_60 (adapter) | typed adapter | 1.1019 / 2.2037 | 1.1019 / 2.2037 | demoted at row 8 (N1 UZ) in both modes; EF/actual − 1 below 1e-4 |
| CSKEW_30_N122 (adapter) | typed adapter | 1.9005 / 3.8010 | 1.9005 / 3.8010 | demoted at row 6 (N1 UX) in both modes; EF/actual − 1 below 1e-4 |
| PP_UTM_2, X = 5e5 m (adapter) | typed adapter | 0.0436 / not demoted | 0.0427 / not demoted | unchanged, byte-identical to the plain solve |
| PP-UTM-5E6-PHI5 (product) | captured and typed | 1.1257 | 1.1258 | `NUMERICAL_INTEGRITY_SENSITIVE` / `Sensitive` on both entries |
| PP-UTM-5E5-PHI2 (product) | captured and typed | 0.0441 | 0.0431 | `NUMERICAL_INTEGRITY_CHECKS_PASSED` / `ChecksPassed` on both entries |

These equal the generator's emulation (`kd5_models_emulation.json`) and RV5's Rust values to every printed digit. The adapter log is `repair/tests/ni_kd5.log` and the product log `repair/tests/pp_formation_check_runtime.log`, both run with `--nocapture`.

**The kills** (R-3 has the full table):

- **M31b.**
  - Adapter: `kd5_admissible_centre_mismatch_demotes_where_the_product_chord_hides_the_error` fails in `assert_demoted_only_in_quality` (`kd5_tests.rs:248`, `assert_eq!(checked.report.quality, SolveQuality::Sensitive)`). The quality is `Passed`.
  - Product: `kd5_very_large_coordinate_pp_route_elbow_demotes_on_both_entries` fails at `formation_check_runtime.rs:386`, where the integrity code is `["NUMERICAL_INTEGRITY_CHECKS_PASSED"]` instead of `["NUMERICAL_INTEGRITY_SENSITIVE"]`. **With the M31b patch applied, the 5e6 m case publishes CHECKS_PASSED.**
  - FK's `kd5_reformed_elements_have_the_rigid_body_null_space` also fails (`formation_check_tests.rs:141`).
- **M31b0:** the same adapter assertion (`kd5_tests.rs:248`) and the same product assertion (`formation_check_runtime.rs:386`) fail.
- **Only behavioural assertions fail.** In both kill tests the preconditions read only the unmutated plain solve (Passed, and actual above the criterion), and they held under both mutants.
- **The logs:** `repair/mutations/M31b.log` and `M31b0.log`. Every earlier I3 test passes under both mutants, as RV5 found.

**Near π: a NOTE, with no test.** RV5 measured ΔEF ≤ 2e-4 of the criterion there, which is harmless.

### R-1a. RV5's E4: the nonlinear pins strengthened (RV5 SHOULD-FIX, tests only)

**The evasion.** RV5's E4 adds a sibling module to `nonlinear_integration`. The module routes only the derived-friction unit-force solves through `solve_assembled_with_formation_check`, behind a neutral-named SA helper (`solve_binary64_audited`). E4 survived every NI test and the nonlinear benchmark, for three reasons:
- the source pin scanned only `lib.rs` and `product_equilibrium.rs`;
- the SA scan did not flag `.solve_assembled_with_formation_check(`;
- the behavioural pin exercised only the first gap iteration.

**The strengthened pins,** all in `s11k_tests.rs`:
- **`kd5_nonlinear_sources_name_no_formation_check_entry_point` now scans every non-test module.** A new helper, `non_test_modules`, walks the module tree from `lib.rs` through every `mod name;` declaration, honouring `#[path]` and `mod.rs`.
  - A module is a test module only when its own declaration is under `#[cfg(test)]`, or it is declared inside a test module. So `s11k_tests`, `kd5_tests` and `kd5_models` are excluded by their declarations, never by file path, and inline `#[cfg(test)]` items are blanked by `strip_cfg_test`.
  - Every `.rs` file under `src` must be reached, so a new module cannot be skipped.
  - The same walk is applied to `product_physics/src`.
  - **In `nonlinear_integration`:** `solve_assembled_with_formation_check` may appear only once, as its definition in `structural_adapter.rs`; any call to it, under any name or in qualified form, fails. `with_formation_source`, `prepare_formation_checked_structural`, `.formation_source(`, `FormationCheckedSystem`, `solve_formation_checked_structural_dense` and `formation_check::` may appear only inside that definition's body. `FormationSource` may appear only in `structural_adapter.rs`, and no `solve_with_formation…` name may appear at all.
  - **In `product_physics`:** the entry is called exactly once, inside `solve_preview_reduced_system`, and the rest of the plumbing does not appear.
- **New: `kd5_nonlinear_loop_unit_force_solves_reach_no_formation_check`.** The model is probe P's beam and gaps with a sliding friction support at the middle node's UX, whose normal is derived from the left gap's reaction, and a 50 N UX load. Seeded sliding makes the loop run several iterations; from the second iteration on it performs its derived-friction base and unit-force solves.
  - For every such iteration, in both modes and at g = 0.03 and 0.09 m, the test recomputes the derived friction force from the loop's own previous iterate and boundary, with base and unit solves on the binary64 path. It then asserts that the loop's applied force is bit-identical to it.
  - **Precondition (the paths differ):** the same unit solve through `solve_assembled_with_formation_check` gives different displacement bits and a different derived force. The g values were chosen by this precondition: at g = 0.05 and 0.20 m the two paths coincide bitwise after the right gap opens.
  - It also asserts that at least one iteration after the first carries the derived force, so the test is not vacuous.
- The first-iteration pin `kd5_nonlinear_loop_reaches_no_formation_check` is kept unchanged.

**The E4 kill:** in the clean run (`repair/mutations/E4.log`), two tests fail.
- **`kd5_nonlinear_loop_unit_force_solves_reach_no_formation_check`** fails at its behavioural assertion (`s11k_tests.rs:1411`). At g = 0.03, dense, iteration 2, the loop's derived friction force is −29.999999999919282: exactly the formation-checked path's value, where the binary64 path gives −30.00000000000472. Its preconditions held.
- **`kd5_nonlinear_sources_name_no_formation_check_entry_point`** fails at `s11k_tests.rs:1071`: `structural_adapter.rs` names the formation-checked entry 2 times, where only its definition may name it.

`mutate.py` gains `E4`, which reproduces RV5's patch from `kd5_review/rust/rv5_mutate.py.txt`; it is part of the clean run (R-3).

### R-2. The repr-input artefact; the earlier ~5e5 m figure is superseded

- **The artefact.** RV5's first emulation converted every binary64 input with `Decimal(repr(v))`, the shortest decimal string, not the exact value.
  - At X ≈ 5e5 m that perturbs a coordinate by up to half an ulp (about 2.9e-11 m). On a 0.0105 m chord that is about 2.5 of the criterion.
  - Running the generator with `Decimal(repr(x))` inputs (`repair/models/check_repr.py.txt`) reproduces RV5's first PP_UTM u_int.
- **With exact inputs, and in the product itself,** PP_UTM at 5e5 m is accurate: 0.044 of the criterion. RV5 confirmed this with exact inputs (0.0432).
- **So the earlier ~5e5 m onset is superseded as a repr-input artefact.** RV5's exact-input coordinate scan puts the onset near 2e6 m: at most 0.36 of the criterion for X ≤ 1e6 m, up to 1.00 at 2e6 m, and 1.13–3.12 at 5e6 m.

### R-3. The clean mutation re-run (`_run_records/repair/mutations/`)

**Stale-build disclosure.** RV5 found that the earlier runner, `run_mutants.sh`, could reuse a stale `frame_kernel` artefact.
- **The mechanism.** `tar` preserved the worktree's mtimes, and the target dir was shared across mutants. So a mutant that did not touch `frame_kernel` (M32a, M32b) was built against the previous mutant's `frame_kernel`, which was M31b0's.
- **Affected rows:** M32a and M32b in phase 1 (§7) and in the addendum-4 pass (A4-4). The other rows each patched `frame_kernel`, or ran first.
- **Those rows are superseded by the table below.** Their logs stay committed as recorded.

**The fix, in `run_mutants_clean.sh`:**
- every mutant gets a fresh copy of `P/core/{solver,loads,serialization,units,product_physics}`, extracted with `tar -m`;
- the shared target dir is removed before every mutant and at the end, so nothing is reused;
- a no-patch control run (NONE) must pass every test.
- I3R added two things to I3's runner. It also waits for a timed probe comparison (`timing_compare`), anchoring the pattern on the interpreter. And it lists each killer's panic site.

**Tests per mutant:** FK `kd5`, NI `kd5` (which includes the two nonlinear pins in `s11k_tests`), and PP `--test formation_check_runtime`, each with `--test-threads=1`. The patches are those of `mutate.py`, unchanged. Each applies exactly once to the candidate tree; I checked this before the run.

| Mutant | Patch | FK / NI / PP exit | Killed by (failing tests) | Panic sites |
|---|---|---|---|---|
| NONE | control, no patch | 0 / 0 / 0 | **none (control passes)** | NONE |
| M23 | (23) trigger disabled | 101 / 101 / 101 | `kd5_curved_arctangent_domain_errors_fail_closed`, `kd5_unavailable_family_and_wide_error_fail_closed_and_never_err`, `kd5_nonlinear_loop_reaches_no_formation_check`, `kd5_admissible_centre_mismatch_demotes_where_the_product_chord_hides_the_error`, `kd5_curved_intended_element_uses_the_actual_chord`, `kd5_d5c1_controls_demote_exactly_where_the_actual_error_exceeds_half_the_criterion`, `kd5_expansion_joint_with_zero_lateral_does_not_demote_and_nonzero_lateral_fails_closed`, `kd5_not_selected_invocation_runs_the_unchanged_solve_assembled`, `kd5_required_true_positive_skew_cantilever_122_demotes_in_both_modes`, `kd5_skew_plane_elbow_cantilever_at_kx_8_5_demotes_in_both_modes`, `kd5_unmatched_explicit_and_one_ulp_curved_slots_fail_closed`, `kd5_nonlinear_support_invocation_is_never_selected`, `kd5_required_true_positive_122_demotes_in_both_modes_on_both_entries`, `kd5_very_large_coordinate_pp_route_elbow_demotes_on_both_entries` | src/s11k_tests.rs:1181:9x1; src/structural/formation_check_tests.rs:254:5x1; src/structural/formation_check_tests.rs:339:9x1; src/structural_adapter/kd5_tests.rs:248:5x7; src/structural_adapter/kd5_tests.rs:656:9x1; tests/formation_check_runtime.rs:154:13x1; tests/formation_check_runtime.rs:386:9x1; tests/formation_check_runtime.rs:70:13x1 |
| M26 | (26) binary64 published residual in place of ρ | 101 / 101 / 101 | `kd5_zero_scale_clause_fires_on_ledger_terms_that_differ_from_the_solve_force`, `kd5_admissible_centre_mismatch_demotes_where_the_product_chord_hides_the_error`, `kd5_curved_intended_element_uses_the_actual_chord`, `kd5_d5c1_controls_demote_exactly_where_the_actual_error_exceeds_half_the_criterion`, `kd5_required_true_positive_skew_cantilever_122_demotes_in_both_modes`, `kd5_skew_plane_elbow_cantilever_at_kx_8_5_demotes_in_both_modes`, `kd5_very_large_coordinate_pp_route_elbow_demotes_on_both_entries` | src/structural/formation_check_tests.rs:204:6x1; src/structural_adapter/kd5_tests.rs:248:5x4; src/structural_adapter/kd5_tests.rs:305:9x1; tests/formation_check_runtime.rs:386:9x1 |
| M27 | (27) element ΔK plus binary64 residual | 101 / 101 / 101 | `kd5_zero_scale_clause_fires_on_ledger_terms_that_differ_from_the_solve_force`, `kd5_admissible_centre_mismatch_demotes_where_the_product_chord_hides_the_error`, `kd5_curved_intended_element_uses_the_actual_chord`, `kd5_curved_matching_is_order_independent`, `kd5_d5c1_controls_demote_exactly_where_the_actual_error_exceeds_half_the_criterion`, `kd5_expansion_joint_with_zero_lateral_does_not_demote_and_nonzero_lateral_fails_closed`, `kd5_large_coordinate_pp_route_elbow_does_not_demote`, `kd5_realistic_elbows_e1_and_e6_do_not_demote`, `kd5_required_true_positive_skew_cantilever_122_demotes_in_both_modes`, `kd5_skew_plane_elbow_cantilever_at_kx_8_5_demotes_in_both_modes`, `kd5_unmatched_explicit_and_one_ulp_curved_slots_fail_closed`, `kd5_large_coordinate_pp_route_elbow_is_published_accurately_and_not_demoted` | src/structural/formation_check_tests.rs:204:6x1; src/structural_adapter/kd5_tests.rs:248:5x1; src/structural_adapter/kd5_tests.rs:258:5x4; src/structural_adapter/kd5_tests.rs:305:9x1; src/structural_adapter/kd5_tests.rs:411:9x1; src/structural_adapter/kd5_tests.rs:439:9x1; src/structural_adapter/kd5_tests.rs:474:13x1; src/structural_adapter/kd5_tests.rs:512:9x1; tests/formation_check_runtime.rs:362:9x1 |
| M28 | (28) binary64 local coefficients | 101 / 101 / 0 | `kd5_reformed_elements_have_the_rigid_body_null_space`, `kd5_d5c1_controls_demote_exactly_where_the_actual_error_exceeds_half_the_criterion`, `kd5_required_true_positive_skew_cantilever_122_demotes_in_both_modes` | src/structural/formation_check_tests.rs:141:13x1; src/structural_adapter/kd5_tests.rs:258:5x1; src/structural_adapter/kd5_tests.rs:305:9x1 |
| M31b | (31b) curved H from the product's binary64 chord R(cos φ − 1), R sin φ | 101 / 101 / 101 | `kd5_reformed_elements_have_the_rigid_body_null_space`, `kd5_admissible_centre_mismatch_demotes_where_the_product_chord_hides_the_error`, `kd5_very_large_coordinate_pp_route_elbow_demotes_on_both_entries` | src/structural/formation_check_tests.rs:141:13x1; src/structural_adapter/kd5_tests.rs:248:5x1; tests/formation_check_runtime.rs:386:9x1 |
| M31b0 | (31b0) the same chord formula at p | 0 / 101 / 101 | `kd5_admissible_centre_mismatch_demotes_where_the_product_chord_hides_the_error`, `kd5_very_large_coordinate_pp_route_elbow_demotes_on_both_entries` | src/structural_adapter/kd5_tests.rs:248:5x1; tests/formation_check_runtime.rs:386:9x1 |
| M32a | (32a) the loop's `solve_binary64` routed through the check | 0 / 101 / 0 | `kd5_nonlinear_loop_reaches_no_formation_check`, `kd5_nonlinear_loop_unit_force_solves_reach_no_formation_check`, `kd5_nonlinear_sources_name_no_formation_check_entry_point` | src/s11k_tests.rs:1089:13x1; src/s11k_tests.rs:1196:9x1; src/s11k_tests.rs:1391:17x1 |
| M32b | (32b) the loop calls the typed formation entry | 0 / 101 / 0 | `kd5_nonlinear_loop_reaches_no_formation_check`, `kd5_nonlinear_loop_unit_force_solves_reach_no_formation_check`, `kd5_nonlinear_sources_name_no_formation_check_entry_point` | src/s11k_tests.rs:1071:9x1; src/s11k_tests.rs:1196:9x1; src/s11k_tests.rs:1391:17x1 |
| E4 | RV5 E4: sibling module routes the derived-friction unit-force solves through the entry via a neutral-named SA helper | 0 / 101 / 0 | `kd5_nonlinear_loop_unit_force_solves_reach_no_formation_check`, `kd5_nonlinear_sources_name_no_formation_check_entry_point` | src/s11k_tests.rs:1071:9x1; src/s11k_tests.rs:1411:17x1 |
| M31a | (31a) curved K_int = the product's binary64 matrix | 101 / 101 / 101 | `kd5_curved_arctangent_domain_errors_fail_closed`, `kd5_admissible_centre_mismatch_demotes_where_the_product_chord_hides_the_error`, `kd5_curved_intended_element_uses_the_actual_chord`, `kd5_skew_plane_elbow_cantilever_at_kx_8_5_demotes_in_both_modes`, `kd5_very_large_coordinate_pp_route_elbow_demotes_on_both_entries` | src/structural/formation_check_tests.rs:307:9x1; src/structural_adapter/kd5_tests.rs:248:5x3; tests/formation_check_runtime.rs:386:9x1 |

**Reading the table.** Every mutant is killed, and the no-patch control passes all three crates. Panic sites are given as file:line×count.
- **M31b and M31b0:** see R-1.
- **E4:** see R-1a.
- **M32a:** all three nonlinear pins fail. The unit-force pin fails at its precondition (`s11k_tests.rs:1391`), because M32a mutates the oracle's own `solve_binary64`. The first-iteration pin (`:1196`) and the source pin (`:1089`) kill it behaviourally and by scan.

**Two disclosures about this run.**
- **The first attempt failed the control.** It copied only `P/core/{solver,loads,serialization,units,product_physics}`, and product_physics did not compile, because it `include_str!`s non-test files from `P/fixtures/results/` (for example `semantic_contract_v0_3_physics_1.json`). That attempt was stopped after M23 and is superseded. Its control log and summary are committed as `repair/mutations/attempt1_NONE.log` and `attempt1_MUTANTS.txt`. The runner now also copies `P/fixtures`, `P/schemas` and `validation/benchmarks/numerical_integrity/fixtures.json`, and the restarted control passed.
- **M31a's patch missed a literal.** In the restarted run, M31a failed to compile frame_kernel's tests: `formation_check_tests.rs` holds a second `CurvedFormation` literal (line 104) that the patch did not extend. NI and PP still killed it behaviourally. The patch now covers both literals. M31a was re-run clean, and that result is in the table. The first log is kept as `M31a_first_patch_incomplete.log`.

### R-4. The callers, regenerated (`_run_records/combined/callers_combined.txt`; scripts in `_run_records/repair/callers/`)

**This corrects §5.** §5 described phase 1: 100 sites, the legacy `solve_with_formation_check`, and PP:3973. Both that entry and the `_with_force_terms` variant were removed in the addendum-4 pass.
- `scan_callers.py` was re-run on the candidate, with its patterns extended by `solve_assembled_with_formation_check(`, `.solve_assembled(` and `FormationCheckedSystem`.
- `classify_callers.py` classifies every non-test site by file and enclosing function, and fails if any is unclassified.
- The regenerated list is the new file `combined/callers_combined.txt`, which supersedes phase 1's. The phase 1 `_run_records/callers.txt` (sha256 `cd8e2ae1…`) describes phase 1. It is hash-bound and kept unchanged, because committed evidence is never rewritten (manager and ROOT ruling, which overrides the brief's wording "replace").

**Result: 116 sites, 77 in tests and 39 outside tests, all classified.**
- The check is reached only from `finish_checked_factor`, through a `PreparedSystem` built by `prepare_formation_checked_structural` from a `FormationCheckedSystem`.
- SA's `solve_assembled_with_formation_check` is the only non-test builder.
- FK's `solve_formation_checked_structural_dense` has no non-test caller.
- The only product call is PP:4399 in `solve_preview_reduced_system`.
- The option-(c) loop reaches no formation entry point.

This agrees with RV5's scan. The list was generated on `2409de83e` plus the finished repair tree, which is test-only. Its 39 non-test sites are identical to I3's list of 16:42 UTC; the 3 additional test sites are the E4 pin (R-1a).

### R-5. Findings (the manager records them in the work graph)

**1. The gate corpus lacks realized curved bends** (routed to the gate-corpus owner). Curved-bend formation integrity is therefore evidenced by unit and product tests only. The M31b miss is what this gap looks like. My evidence:
- **None of the 222 authorable gate requests has a component at all** (P1's generator output, re-scanned), so none realizes a curved bend.
- **No committed JSON request or model** under `P/fixtures`, `P/validation` or `P/core` realizes one.
  - The committed bend components with a solver consumption are all `mechanics_geometry_only`: `preview_physics_invented_model.json`, `preview_physics_unicode_ids_model.json`, `invented_preview_model.json` and `tp_runner_015_final_cli_solve_input.json`. PP does not build a curved element for that consumption.
  - `curved_bend_macro_element` appears in a committed JSON document only in `fixtures/results/invented/result_export_v0_2.json`, which is a result-export semantic fixture, not a solve request.
- **Routing:** a candidate addition of curved-bend references, including large-coordinate ones. K-D5 added no corpus cases.

**2. The product's curved element carries formation error above the criterion at coordinates of about 2e6 m and more** (UTM northing scale; routed to T4/W1c).
- **The mechanism.** PP computes the arc centre in binary64, so its components are rounded at ulp(X). At large X the centre is still admissible (radius-match tolerance 1e-9) but no longer equidistant. The product's element then uses the formula chord R(cos φ − 1), R sin φ, which differs from the actual chord x_j − x_i.
- **At 5e5 m** the effect is negligible: 0.044 of the criterion.
- **At 5e6 m, φ = 5°,** the actual error is 1.126. RV5 also reports 3.12 at 5e6 m with φ = 2°, and 2.28 at 7.3e6 m with φ = 10°.
- **K-D5 demotes these correctly.** The cost is availability for GIS-scale models with small-angle realized elbows; it is not a K-D5 defect.
- The earlier ~5e5 m figure is superseded as a repr-input artefact (R-2).

**3. ROOT's fact: no committed fixture or gate case changes standing through this effect.** My evidence:
- No gate request realizes a curved bend (finding 1).
- T9 is 112 of 112 byte-identical against main (C-4), and against main only 122 differs in the gate (C-6).
- **The bend-realizing solve models are all Rust tests, and none is at large coordinates.** ROOT's wording names only the `arc_model` tests at 1.2 m or less. That list is incomplete, but its conclusion holds. The full set is:

| Where | What | Largest coordinate |
|---|---|---|
| PP `tests/preview_physics_runtime.rs` | `arc_model` and REF-B2 | 1.2 m |
| PP `src/s11f_tests.rs` | `curved_bend` (F8) | 2 m chord at the origin |
| PP `src/lib.rs` tests | `curved_bend_span_request` | 2 m chord at the origin |
| PP `src/s11g_tests.rs` | `curved_body` | origin 0 or 100 m, so 102 m at most |
| NI `src/lib.rs` tests | the invented macro element | small |
| `validation/benchmarks/mechanics` | direct `CurvedBendMacroElement` elements, not through PP | small |
| K-D5's own tests | E1, E6, the CSKEW models and CPLANAR_60 | 22.2 m or less |
| K-D5's own tests | PP_UTM_2 and PP-UTM-5E5 | 5e5 m (not demoted) |
| K-D5's own tests | PP-UTM-5E6 | 5e6 m (demoted, by design) |

### R-6. NOTEs

- **The vacuous loop is fixed.** In PP's `kd5_nonlinear_support_invocation_is_never_selected`, the loop over `source_block_recovery.body.cases` iterated nothing. A nonlinear-support invocation is not source-eligible (`source_eligible(captured, nonlinear, combinations)` is false), so it carries no receipt. The test now asserts that `source_block_recovery` is null on both entries in both modes, which is the state the loop relied on.
- **The empty nohup logs.** Five committed K-D5 records are empty files (sha256 `e3b0c442…`). They are the runner's empty stdout; every result went to the per-mutant logs, `MUTANTS.txt` and the suite logs:
  - the four `_run_records/mutations/nohup{,2,3,4}.log` of phase 1;
  - `_run_records/addendum4/targeted/suites_nohup.log`.

  They are hash-bound, so they are kept as recorded. They are not evidence of anything. No empty file was added in this pass.
- **Conservative S\*.** The check's body scale S* is formed over free rows only (D1 §4.1.6.1 item 4). Restrained rows, whose displacement is zero or prescribed, never enlarge it, so the rule is at least as strict as a scale taken over all rows.

### R-7. The repair's runs and records

All runs used `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0` and `--offline --locked`, one cargo job at a time. The targeted runs and suites used `<wt>/kd5-target`; the mutation runs used a scratch target that was removed per mutant. Each run started only after checks that no other cargo and no sweep or timing process was running. The logs are in `repair/tests/`, with the scripts `run_repair_tests.sh.txt` and `run_ni_rerun.sh.txt`.

| Run | Result |
|---|---|
| NI `cargo test kd5 -- --nocapture` (after the E4 edits) | 15 passed, 0 failed |
| NI full suite (after the E4 edits) | 89 passed, 0 failed (86 before K-D5's repair, +3: two adapter tests and the E4 unit-force pin) |
| FK `cargo test kd5` | 5 passed, 0 failed |
| PP `--test formation_check_runtime -- --nocapture` | 5 passed, 0 failed |
| PP full suite | 516 passed, 0 failed, 1 ignored (the existing `source_receipt` measurement; 514 before, +2 PP-route tests) |
| PP `--test s11f_site_test` (after the E4 edits) | 11 passed, 0 failed |
| Clean mutation run | NONE + 10 mutants; all killed, control passes (R-3) |

The NI kd5 run and NI suite were first run before the E4 edits (14/14 and 88/88); those logs were superseded by the re-runs and are not committed. The PP suite ran before the E4 edits. Those edits touch only `s11k_tests.rs`, a `#[cfg(test)]` module of `nonlinear_integration` that product_physics does not compile. The PP site test was re-run afterwards.

**The files changed by the repair** (all tests or generated test models):
- `P/core/solver/nonlinear_integration/src/structural_adapter/kd5_tests.rs`: two tests (R-1);
- `…/structural_adapter/kd5_models.rs`: three generated exact-reference models appended; the existing constants are byte-identical;
- `P/core/solver/nonlinear_integration/src/s11k_tests.rs`: the strengthened source pin, with `non_test_modules` and `occurrences_outside`, and the E4 unit-force pin (R-1a);
- `P/core/product_physics/tests/formation_check_runtime.rs`: the PP-route elbow pair, and the vacuous-loop fix (R-6).

**Formatting.** `kd5_tests.rs`, `formation_check_runtime.rs` and `s11k_tests.rs` were formatted with rustfmt 1.8.0-stable in stdin mode, one file at a time. The 1.97.1 toolchain carries no rustfmt component, so this is the stable toolchain's rustfmt, the same version as `toolchain.txt`. `kd5_models.rs` is generated and carries `rustfmt::skip`. No other file was formatted.

**Records:**
- `_run_records/combined/`: suites, fixture_diff, gate (with the attribution and move analyses), timing, and `callers_combined.txt`;
- `_run_records/repair/`: tests, mutations, callers scripts, models (the generator re-run) and product_probe (the pre-repair probe runs of the two PP-route requests);
- `SHA256SUMS` is refreshed. No committed record was rewritten, and no empty file was added.
- Machine paths are replaced by placeholders.

**GEN-8** (`pytest tools/practitioner_harness/test_live_baseline.py -k gen8`, from the repository root of `<wt>/kd5`): **1 passed** (`test_live_gen8_semantic_portability_invariants`, not skipped), run with the DEC-025 venv's Python 3.11.15 and pytest 9.1.1 on the repair tree with these records in place.

**What was not done.**
- No product source was changed. No Git write or index operation was made; the manager commits.
- No test was skipped, no timeout was raised, and nothing was weakened.
- Hosted CI, the DEC-025 sweep and the Python and desktop TS suites were not run. No fixture reader was touched.
- No gate re-run was needed: the repair changes tests only, and the gate probe links no test code.
