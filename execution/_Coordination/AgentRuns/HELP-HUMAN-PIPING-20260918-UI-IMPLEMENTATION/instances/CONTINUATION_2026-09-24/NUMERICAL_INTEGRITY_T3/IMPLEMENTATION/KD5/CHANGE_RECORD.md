# K-D5 change record: the D-5 formation check

This is the draft PR record for slice K-D5 of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. It was implemented by I3 (TASK).

- **Branch:** `codex/piping-kd5-20260926`, from the K3a head `a2e804a75`, which sits on the S11-K head `4912dc636`.
- **Landing:**
  - S11-K has merged (PR973, `3488a236a`). K3a is under review.
  - S11-F lands first. K-D5 then merges forward onto main, after S11-F and K3a, with its own full-gate PR.
  - At that merge, the PP call switches to the typed entry (see "Forward merge").
- **Basis:**
  - `ROOT_SELECTION_DESIGNS.md`, conditions C2 and C5;
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.3, §4.3.1, the §6 K-D5 row and §7.3 mutations (23), (26)–(28), (31) and (32);
  - `R5_4_CURVED.md` (`2c9fae78…`);
  - `ROOT_RULINGS_V1.md`: D-5, D5C-1 to D5C-5, R5-4, option (c), and the nonlinear-support ruling;
  - the I3 brief with addenda 1–3. Addendum 2 records ROOT's acceptance of option B for curved sources, the `selected` flag, pins backed by behavioural tests, and the rule that joints with nonzero lateral stiffness demote.

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
- **Fail-closed cases.** These cannot arise from any committed model today.
- **Admissible radius mismatch.** A realized bend whose binary64 centre is admissible but not exactly equidistant (up to the product's 1e-9 tolerance) demotes where the error exceeds half the criterion. The check measures it from the actual chord, as the added test shows.
- **Not affected:**
  - invocations with any nonlinear support: never selected, per ROOT, so the unchanged `solve` runs;
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
| `P/core/solver/frame_kernel/src/structural.rs` | `mod formation_check` and re-exports; the `FormationCheckedSystem` wrapper, built by `StructuralSystem::with_formation_source` or typed by `AssembledStructuralSystem::with_formation_source`; a private `formation` field on `PreparedSystem`; `prepare_formation_checked_structural` and its `_with_force_terms` variant; `solve_formation_checked_structural_dense`; the check and the demotion in `finish_checked_factor`; `formation_check` on `StructuralSolution` |
| `…/frame_kernel/src/structural/formation_check.rs` (new) | the check: `Wide<2>` re-formation of frames, curved bends and joints, ρ, w, S* and the rule |
| `…/frame_kernel/src/structural/formation_check_tests.rs` (new) | kernel-level tests, including the zero-scale clause (N-2) and both AngleDomain cases |
| `…/frame_kernel/src/structural/retained/mod.rs` | inner `#![allow(dead_code)]` removed (K-D5 is K3a's first caller). On the development branch, `#[cfg_attr(not(test), allow(dead_code))]` on `mod wide;` covers the 13 K3a items unused outside tests (listed in RETURN §2), so that `wide.rs` stays byte-identical to K3a. **At the forward merge (addendum 4)**, this becomes per-item `#[allow(dead_code)]` with a one-line reason each |
| `P/core/solver/nonlinear_integration/src/structural_adapter.rs` | `AssemblyEvidence::new` records the primitives; `solve_with_formation_check` (legacy force) and `solve_assembled_with_formation_check` (typed) beside the unchanged `solve`, `solve_assembled` and `solve_binary64`; curved matching (option B) |
| `…/structural_adapter/kd5_tests.rs`, `kd5_models.rs` (new) | adapter tests in both modes, and generated models with exact references |
| `P/core/product_physics/src/lib.rs` | only the call in `solve_preview_reduced_system` at base line 3965, switched to `solve_with_formation_check(…, &curved_sources, built.nonlinear_supports.is_empty())` |
| `P/core/product_physics/tests/formation_check_runtime.rs` (new) | product tests through both entries and both modes |

Line counts are in RETURN §1. There is no Cargo.toml, lockfile, schema, fixture or committed-output change.

## Measured fixture result

- Every committed JSON request or model under `P/fixtures`, `P/validation` and `P/core` was run through the captured entry in both modes, using S11-K's harness unchanged.
- **All 112 of 112 outputs are byte-identical to the base.**
- So no committed raw, derived document or hash pin changes, and the stop rule did not trigger (RETURN §6).

## The no-Passed-breach gate (both entries)

- **Result: PASS.** 888 runs: 222 cases × both modes × both entries.
- The 228 trusted breach triples all lie in the pinned `S11_EXCEPTIONS.json` (221) plus `FORMATION_EXCEPTIONS.json` (7, ROOT's F12 ruling `db665f2cb`); there are no violations.
- The only standing change against P1's main baseline is RF-SKEW 122 → Sensitive, on both entries.
- A container restart reported during the run did not interrupt the gate process: it is one continuous run of one binary (RETURN §9).

## Cost

- **Per curved element:** 2,499 correctly rounded `Wide<2>` operations and one arctangent. This was measured with K3a's work counter on a 139° bend, and covers flexibility, inverse, H and transform. It is O(6³ + 12³), with no extra solve.
- **Per frame:** a few hundred operations.
- **Per free row:** one exact sum over the element contributions.
- **Per case:** one solve pair with the existing factor, run only when the case would publish Passed.

## Forward merge (with S11-F)

- S11-F switches PP:3965 to the typed `solve_assembled`, and narrows the legacy `solve` to `pub(crate)`.
- At the merge, PP calls `solve_assembled_with_formation_check(…, &AssembledForce, …, &curved_sources, built.nonlinear_supports.is_empty())`. ρ then uses the ledger terms.
- The legacy `solve_with_formation_check` can then be narrowed or removed.
- The nonlinear pins in `kd5_tests.rs` are to be folded into I1's module (`nonlinear_integration/src/s11k_tests.rs`).

## Checks run

See RETURN: every suite in the touched crates and their path dependents, the fixture diff, the no-Passed-breach gate through both entries, and mutations (23), (26)–(28), (31a/b) and (32a/b).

**Mutation (31b) is equivalent at the criterion** (ROOT, 2026-09-27, `c2042fd9c`). It keeps K_t re-formed but builds H from the product's chord R(cos φ − 1), R sin φ. It survives every test:
- On CSKEW_8_5 the mutant's trigger agrees with the correct check's to 4+ digits.
- On an admissible mismatch the triggers are 1.681 against 1.685 dense, an EF shift of about 0.002 of the criterion. The mismatch: the centre moved −6.5e-10 R along the chord, |ri| − |rj| = 9.2e-10 relative, and the chord differs from the actual one by 1.4e-10 m per component.
- The reason: a chord error gives the rigid-rotation force pair a net moment of second order, and its first-order stiff-mode effect is far below the criterion.
- The at-p variant (31b0) behaves the same.

**Note on the design.** D1 §7.3 (31) and R5_4_CURVED §4 say that building H from the product's chord misses the k_X = 8.5 case. That holds only for the whole-matrix form (31a): the product's binary64 curved matrix is taken as K_int. This agrees with V1's VERIFY_R5 item 16, which reproduced the miss through the shared matrix (0.254).

**Still required:** the actual-chord implementation (R5-4 §2 step 6), `kd5_curved_intended_element_uses_the_actual_chord`, and M31a's kill. RV-K-D5 will try to construct an admissible model on which 31b is observable. If it finds one, that model becomes a required test and 31b must be killed before merge.

## Not claimed

- Load formation before S11-F, member-action and reaction recovery, and input representation remain outside EF (D1 §4.3.1).
- EF is a first-order estimate with factor 2, not a bound.
- This PR accepts no governed truth and does not release anything.
