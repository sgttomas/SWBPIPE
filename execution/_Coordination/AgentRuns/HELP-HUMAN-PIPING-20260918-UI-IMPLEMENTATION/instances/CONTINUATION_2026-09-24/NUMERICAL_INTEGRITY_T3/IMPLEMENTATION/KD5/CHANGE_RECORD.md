# K-D5 change record: the D-5 formation check

This is the draft PR record for slice K-D5 of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. It was implemented by I3 (TASK).

- **Branch:** `codex/piping-kd5-20260926`. It started from the K3a head `a2e804a75` (on the S11-K head `4912dc636`) and was merged forward onto main:
  - K3a, `b6156d49d`;
  - S11-F, PR1000, `8fd409e78`;
  - PR1002, `3befacff4`.
- **Landing:**
  - The addendum-4 pass (below) is verified on `3befacff4` plus its edits.
  - S11-G (PR1003, `b24b3d536`) is merged into the branch (`fbc9661a4`), followed by the T20 doc-comment correction (`2409de83e`). The combined-tree checks passed: suites 24/24, T9 112/112 byte-identical against main, and the gate union PASS with 0 trusted breaches against main's empty lists (RETURN, "Combined-tree pass and RV5 repair").
  - The RV5 repair (tests and records only) adds the required M31b tests and kills; see below.
- **Basis:**
  - `ROOT_SELECTION_DESIGNS.md`, conditions C2 and C5;
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.3, §4.3.1, the §6 K-D5 row and §7.3 mutations (23), (26)–(28), (31) and (32);
  - `R5_4_CURVED.md` (`2c9fae78…`);
  - `ROOT_RULINGS_V1.md`: D-5, D5C-1 to D5C-5, R5-4, option (c), and the nonlinear-support ruling;
  - the I3 brief with addenda 1–4, and the manager's addendum-4 instructions (RV3 `S11F_REVIEW.md` §11). Addendum 2 records ROOT's acceptance of option B for curved sources, the `selected` flag, pins backed by behavioural tests, and the rule that joints with nonzero lateral stiffness demote.

## What changes

After an ordinary linear solve passes its residual gate and would publish **Passed**, the kernel estimates the forward error of the published displacements:

- EF = K̃⁻¹ρ, using the attempt's own factor, in its scaled variables. This is one extra solve pair.
- ρ = f − K_int·u is the residual of the *intended* system. It is formed as one `ExactAccumulator` sum per free row.
- K_int is re-formed in `Wide<2>` at p = 128 from the binary64 primitives:
  - straight frames: the frame, L, and the coefficients EA/L, GJ/L, 12EI/L³, 6EI/L², 4EI/L and 2EI/L, then TᵀKT, with nothing shared with binary64 `local_stiffness`;
  - realized curved bends, as an objective element: square roots, the K3a `included_angle`, the closed-form flexibility and its inverse at p, and H from the actual chord;
  - expansion joints with zero lateral stiffness;
  - ground springs, as exact binary64 values.
- The prescribed coupling K_fc·u_c is part of ρ.
- **The case is demoted from Passed to Sensitive** if 2|w_i| > 1e-9·max(|q_i|, S*_kind) on any free nodal row, or if the scale is zero while w_i ≠ 0.
- It is also demoted, with reason `formation_check_unavailable`, if the check cannot re-form or evaluate a contribution. This covers:
  - an explicit or unmatched curved slot;
  - a joint with lateral stiffness;
  - any `WideError`, such as `AngleDomain` for a bend within about 1e-19 of π;
  - an exact-sum range error;
  - a failed correction solve.

The check fails closed. It never passes a case silently and never turns a solve into an `Err`.

**Which cases can change standing (Passed → Sensitive), and why.** Only Passed cases on the ordinary linear route, and only these:
- **The skew and absorbed-spring class and pure solve error** at cond 1e6–6.7e7. Here main publishes values up to about 4e-9 relative, which is up to about 4× the criterion.
  - The required true positive, P1's RF-SKEW-T-CANT-OFF-122-r1e-04, demotes in both modes on both entries.
  - V1's probe C (absorbed spring) and probe D (solve error only), and an axis-aligned bending-soft member, each demote where their actual error exceeds half the criterion.
- **The curved class (R5-4).** An example is the skew-plane elbow cantilever at k_X = 8.5, with an actual error of 1.48 dense and 1.10 sparse.
- **Realized curved bends at very large coordinates** (about 2e6 m and more, UTM northing scale). PP computes the arc centre in binary64, so there the centre is admissible but not equidistant, and the product's element (built on the formula chord R(cos φ − 1), R sin φ) carries formation error above the criterion. K-D5 re-forms from the actual chord and demotes these correctly. Example: X = 5e6 m, Y = 3.5e6 m, R = 0.3 m, φ = 5°, actual error 1.126, which demotes on both entries in both modes. At X ≈ 5e5 m the error is 0.044 and nothing demotes. No committed fixture or gate case realizes a curved bend, so none changes standing through this effect. The product effect is routed to T4/W1c.
- **Fail-closed cases.** These cannot arise from any committed model today.
- **Admissible radius mismatch.** A realized bend whose binary64 centre is admissible but not exactly equidistant (up to the product's 1e-9 tolerance) demotes where the error exceeds half the criterion. The check measures it from the actual chord, as the added test shows.
- **Not affected:**
  - invocations with any nonlinear support: never selected, per ROOT, so the unchanged `solve_assembled` runs;
  - the nonlinear active-set loop: unchanged `_binary64` path, pinned;
  - Sensitive, unresolved and refused cases.

**No value changes.** A demoted case publishes exactly the same displacements and every other report field. Only `quality: Sensitive` differs. As for any Sensitive case, it then enters today's Sensitive path, including exact-block recovery where that selects. The tests assert bitwise-equal displacements and a report equal in everything except quality.

**No in-band marker.** `StructuralReport` is unchanged (D5C-3). The record `FormationCheck` holds the row, 2|w|, the scale, the trigger value and the reason. It is an `Option` on `StructuralSolution`, set only on demotion, and nothing renders it. F1 will render it as an evidence line.

## The required true positive

RF-SKEW-T-CANT-OFF-122-r1e-04, the confirmed M03 skew breach (2.43e-9 relative), moves from checks_passed to **sensitive on both entries (captured and typed) and in both modes**.
- Trigger values: 4.827 dense and 2.428 sparse, so EF is 2.413 and 1.214, equal to the measured frozen-reference breach.
- Values are unchanged. The case is not an exception, and it appears nowhere as a Passed breach in the gate.
- Tests: `kd5_required_true_positive_122_demotes_in_both_modes_on_both_entries` (product) and `kd5_required_true_positive_skew_cantilever_122_demotes_in_both_modes` (adapter).
- It is the only standing change against P1's main baseline over the frozen references.

## Files

| File | Change |
|---|---|
| `P/core/solver/frame_kernel/src/structural.rs` | `mod formation_check` and re-exports; the `FormationCheckedSystem` wrapper, built by `StructuralSystem::with_formation_source` or typed by `AssembledStructuralSystem::with_formation_source`; a private `formation` field on `PreparedSystem`; `prepare_formation_checked_structural`; `solve_formation_checked_structural_dense`; the check and the demotion in `finish_checked_factor`, after S11-F's N5 block and only for a case that is not already ordinary-Sensitive; `formation_check` on `StructuralSolution` |
| `…/frame_kernel/src/structural/formation_check.rs` (new) | the check: `Wide<2>` re-formation of frames, curved bends and joints, ρ, w, S* and the rule |
| `…/frame_kernel/src/structural/formation_check_tests.rs` (new) | kernel-level tests, including the zero-scale clause (N-2) and both AngleDomain cases |
| `…/frame_kernel/src/structural/retained/mod.rs` | the inner `#![allow(dead_code)]` is removed, because K-D5 is K3a's first caller. No attribute remains on `mod wide;` |
| `…/frame_kernel/src/structural/retained/wide.rs` | +13 lines: per-item `#[allow(dead_code)] // <reason>` on the 13 items with no non-test caller (test-only, K3 API or K4 budgets; RETURN A4-1 item 8) |
| `P/core/solver/nonlinear_integration/src/structural_adapter.rs` | `AssemblyEvidence::new` records the primitives. Adds the typed `solve_assembled_with_formation_check(k, &AssembledForce, free, prescribed, mode, curved_sources, selected)` beside the unchanged `solve`, `solve_assembled` and `solve_binary64`; `selected = false` returns `solve_assembled`. Adds curved matching (option B). There is no legacy `&[f64]` formation entry |
| `…/structural_adapter/kd5_tests.rs`, `kd5_models.rs` (new) | 12 adapter tests through the typed entry, both modes, and generated models with exact references. The RV5 repair adds the M31b kill on admissible centre mismatches (CPLANAR_60, CSKEW_30_N122) and the PP_UTM_2 large-coordinate control |
| `P/core/solver/nonlinear_integration/src/s11k_tests.rs` (I1's) | the SA defining list gains `fn solve_assembled_with_formation_check(`. Adds `FORMATION_ENTRY_POINTS` and the nonlinear pins. The RV5 repair (E4) strengthens the lexed source pin to every non-test module of the crate and of product_physics (module walk from `lib.rs`; test modules excluded by their `#[cfg(test)]` declarations), and adds a behavioural pin on the derived-friction unit-force solves over several iterations, with a paths-differ precondition. The first-iteration behavioural pin is kept. `EXACT_ENTRY_POINTS` is unchanged |
| `P/core/product_physics/src/lib.rs` | only the call in `solve_preview_reduced_system`: `assembly.solve_assembled(…)` becomes `assembly.solve_assembled_with_formation_check(original_stiffness, global_force, &free, prescribed, mode, &curved_sources, built.nonlinear_supports.is_empty())`. `load_fidelity` is still filled from its result |
| `P/core/product_physics/tests/s11f_site_test.rs` | the `rule_1` pin names the formation-checked typed call, backed behaviourally. `FK/structural/formation_check.rs` is appended to `KERNEL`, and its `check` and `evaluate` are added to rule 6's `FORCE_FUNCTIONS` |
| `P/core/product_physics/tests/formation_check_runtime.rs` (new) | product tests through both entries and both modes. The RV5 repair adds the PP-route elbow pair: 5e6 m demotes (the product-level M31b kill), 5e5 m does not. It also replaces the vacuous receipt loop with an assertion that `source_block_recovery` is null |

Line counts are in RETURN §1. There is no Cargo.toml, lockfile, schema, fixture or committed-output change.

## Measured fixture result

- Every committed JSON request or model under `P/fixtures`, `P/validation` and `P/core` was run through the captured entry in both modes, using S11-K's harness unchanged.
- **All 112 of 112 outputs are byte-identical to the base.** This held in phase 1 against K3a, and again in the addendum-4 pass against main `72d5ff864`, built as a separate harness.
- So no committed raw, derived document or hash pin changes, and the stop rule did not trigger (RETURN §6).

## The no-Passed-breach gate (both entries)

- **Addendum-4 pass** (pre-S11-G tree `3befacff4` plus edits; lists as on main, numerics `59fff0d9e`, with S11 empty and FORMATION holding 7 triples): **PASS**.
  - 888 runs: 222 cases × both modes × both entries.
  - The trusted breach triples are **exactly the 7 FORMATION triples**. None of the former 221 re-breaches.
  - 122 is Sensitive on both entries in both modes.
  - The only standing change against P1's main baseline is 122 → Sensitive (RETURN A4-5).
- **Combined tree** (`2409de83e`, K-D5 with S11-G; lists as on main `b24b3d536`, both empty): **PASS**, as the union of part 1 (884 runs) and part 2 (the 4 known dense timeouts, which time out on a quiet host as on main).
  - 888 runs, with 0 trusted breach triples.
  - Against main only 122 differs, on both entries in both modes; main alone fails the empty-list gate on 122's 8 triples.
  - The 34 moves against the pre-S11-G gate are S11-G's own demotions, which main makes too (ROOT: S11-G's forecast demotions).
- **Phase 1** (base `a2e804a75`, pre-S11-F lists): PASS. Its 228 trusted triples all lay in S11 (221) plus FORMATION (7) (RETURN §9).

## Cost

- **Per curved element:** 2,499 correctly rounded `Wide<2>` operations and one arctangent. This was measured with K3a's work counter on a 139° bend, and covers flexibility, inverse, H and transform. It is O(6³ + 12³), with no extra solve.
- **Per frame:** a few hundred operations.
- **Per free row:** one exact sum over the element contributions.
- **Per case:** one solve pair with the existing factor, run only when the case would publish Passed.
- **Measured on the combined tree:**
  - Over the 382 gate runs under 1000 members where the check ran, the wall-time difference from main is median −0.0004 s and at most +0.081 s per run.
  - In an interleaved timed comparison at 1000 members (dense), the candidate was 3–5% faster than main.
  - Both S11-G-bearing probes are about 15–20% slower than the pre-S11-G tree. That is recorded as an S11-G performance finding, not a K-D5 cost.

## Forward merge (with S11-F): done in the addendum-4 pass

- PP calls `solve_assembled_with_formation_check(…, &AssembledForce, …, &curved_sources, built.nonlinear_supports.is_empty())`, so ρ starts from the ledger terms.
- S11-F's N5 audit block runs first, unchanged. The check runs only for a case that would still publish Passed.
- The legacy `solve_with_formation_check` and FK's `prepare_formation_checked_structural_with_force_terms` are removed; neither had a caller left.
- The nonlinear pins are folded into I1's `s11k_tests.rs`.
- Per-item `#[allow(dead_code)]` is on K3a's `wide.rs`.

## Checks run

See RETURN: every suite in the touched crates and their path dependents, the fixture diff, the no-Passed-breach gate through both entries, and mutations (23), (26)–(28), (31a/b) and (32a/b).

**Mutations (31b) and (31b0) are killed** (RV5 repair). ROOT withdrew the equivalence below on RV5's counterexample, confirmed in Rust (`ROOT_RULINGS_V1.md`, "K-D5 mutation M31b: equivalence withdrawn", numerics `3547029576`). The required tests are:
- `kd5_admissible_centre_mismatch_demotes_where_the_product_chord_hides_the_error` (adapter, both modes; planar 60° and skew 30° admissible centre mismatches);
- `kd5_very_large_coordinate_pp_route_elbow_demotes_on_both_entries` (product, both entries and modes, X = 5e6 m). Under M31b this case publishes CHECKS_PASSED.

The earlier claim follows, **superseded** and kept as recorded. The clean mutation re-run (RETURN R-3) also supersedes the earlier M32a and M32b rows, which could run against a stale `frame_kernel` build.

**Superseded: mutation (31b) is equivalent at the criterion** (ROOT, 2026-09-27, `c2042fd9c`). It keeps K_t re-formed but builds H from the product's chord R(cos φ − 1), R sin φ. It survives every test:
- On CSKEW_8_5 the mutant's trigger agrees with the correct check's to 4+ digits.
- On an admissible mismatch the triggers are 1.681 against 1.685 dense, an EF shift of about 0.002 of the criterion. The mismatch: the centre moved −6.5e-10 R along the chord, |ri| − |rj| = 9.2e-10 relative, and the chord differs from the actual one by 1.4e-10 m per component.
- The reason: a chord error gives the rigid-rotation force pair a net moment of second order, and its first-order stiff-mode effect is far below the criterion.
- The at-p variant (31b0) behaves the same.

**Note on the design (superseded in part).** The equivalence argument in this note is withdrawn. The superseding record is ROOT's rulings section (numerics `3547029576`), which also supersedes DESIGN.md 5a.2 §9 item 31's claim; the hash-pinned design is not edited. The observation about CSKEW_8_5 still holds for that model. As first written: D1 §7.3 (31) and R5_4_CURVED §4 say that building H from the product's chord misses the k_X = 8.5 case. That holds only for the whole-matrix form (31a): the product's binary64 curved matrix is taken as K_int. This agrees with V1's VERIFY_R5 item 16, which reproduced the miss through the shared matrix (0.254).

**Still required:** the actual-chord implementation (R5-4 §2 step 6), `kd5_curved_intended_element_uses_the_actual_chord`, and M31a's kill. RV-K-D5 (RV5) found an admissible model on which 31b is observable. As this condition required, those models are now required tests, and 31b and 31b0 are killed (above).

## Not claimed

- Load formation before S11-F, member-action and reaction recovery, and input representation remain outside EF (D1 §4.3.1).
- EF is a first-order estimate with factor 2, not a bound.
- This PR accepts no governed truth and does not release anything.
