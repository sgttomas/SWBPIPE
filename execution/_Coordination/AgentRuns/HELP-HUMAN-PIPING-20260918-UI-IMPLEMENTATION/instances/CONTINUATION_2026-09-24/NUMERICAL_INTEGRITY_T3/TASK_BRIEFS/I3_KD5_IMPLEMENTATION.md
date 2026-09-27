# I3: implement slice K-D5 (the D-5 formation check)

This is an implementation TASK. Read `_COMMON.md` first. This brief overrides it where they differ (writes and builds).

## Purpose

Implement slice K-D5 of the selected design (`ROOT_SELECTION_DESIGNS.md`). K-D5 is the formation-error check on the ordinary linear route:
- EF = K̃⁻¹ρ, where ρ = f − K_int·u is the exact residual of the intended system, formed in `Wide<2>` from binary64 primitives, with a factor of 2 on the coupled S\*.
- A Passed invocation whose check exceeds the criterion is demoted to Sensitive.

K-D5 repairs main's Passed-band wrong values in the skew and absorbed-spring class and in the curved class (R5-4). It must never publish a silently wrong value, strip a load or feature, skip a test or raise a timeout.

## Governing basis (read in this order)

1. `T3/ROOT_SELECTION_DESIGNS.md`, including **C2** (the scripts read `hanger.stiffness`) and **C5**.
2. `T3/DESIGN_NUMERICS/DESIGN.md` **revision 5a.2** (`932698d7a`, `fb62ef4a…`):
   - **§4.3 and §4.3.1**: the check, the trigger (2|w|/criterion > 1; a quoted EF is |w|/criterion), the factor-2 margin, the zero-scale clause, and nonlinear supports never selected;
   - **the K-D5 row of the §6 slice table** (write set and tests, quoted below);
   - the §9 mutation table: (23), (26)–(28), (31), (32).
3. `T3/DESIGN_NUMERICS/R5_4_CURVED.md` (`2c9fae78`) and `_run_records/curved_ef.py`: the objective curved and joint re-formation from the actual chord, with sine and cosine from square roots, and the atan from K3a.
4. `T3/DESIGN_NUMERICS/D5_TRIGGER.md` revision 2a (`f6e24a69`), for background only: §4.3.1 of DESIGN replaces its (a2) rule, §5 and §9.
5. `T3/ROOT_RULINGS_V1.md`: D-5 (O1 final), D5C-1 to D5C-5, R5-4, S11-K's option (c), and the nonlinear-support ruling.
6. `T3/REVIEW/D5_CHECK.md` and `VERIFY_R5.md`: V1's probes C and D, and the R5-4 reproductions.
7. K3a's `retained/wide.rs` and S11-K's `exact_sum.rs` and `ExactAccumulator` on your base. Use them; do not modify them.

## Base, worktree and branch

- ROOT creates `<kd5-worktree>`, on a new branch from **the K3a head** (which already contains S11-K).
- **K-D5 lands after K3a merges**, with its own full-gate PR.
- Make no Git writes. The manager commits.

## Write set (from the K-D5 row)

- **`P/core/solver/frame_kernel/src/structural.rs`:**
  - EF after the residual gate in `finish_checked_factor`;
  - the Passed→Sensitive demotion;
  - a typed optional formation source on `StructuralSystem`;
  - a `FormationCheck` record on `StructuralSolution`, **never on `StructuralReport`**.
- **New `P/core/solver/frame_kernel/src/structural/formation_check.rs`:**
  - the `Wide<2>` re-formation of straight frames, realized curved bends (objective, from the actual chord) and user-stiffness elements from their primitives;
  - ρ as one `ExactAccumulator` sum per free row, extending `contribution_sums`/`audit_intended_action`.
- **`P/core/solver/nonlinear_integration/src/structural_adapter.rs` (SA):**
  - the formation source from `AssemblyEvidence::new`: frame, curved and user primitives, plus a flag for any family the check cannot re-form;
  - a **new `solve_with_formation_check`** beside the unchanged `solve`.
- **`P/core/product_physics/src/lib.rs`:** only the one call site at the merged `PP:3965` (`solve_preview_reduced_system`), switched to `solve_with_formation_check`. Report the exact line on your base. No other `PP` change.
- **C2:** `T3/DESIGN_NUMERICS/_run_records/withheld_rows.py` and `b_proof.py` in the **numerics** worktree read `hanger.stiffness` as `support_stiffness_input` does. Re-run both and **report any change to their outputs to the manager before continuing.** This is a records write outside your product worktree; it is the only one allowed.
- Tests in the touched crates, and records in `T3/IMPLEMENTATION/KD5/**` in `<kd5-worktree>`.

## Callers (ROOT's recorded lesson)

**Enumerate every caller of every function you change.** At least:
- `solve_structural_dense` and `_sparse`;
- `StructuralAssembly::solve` / `AssemblyEvidence::solve` and `solve_binary64`;
- `finish_checked_factor`;
- `evaluate_original_residual`.

Classify each caller. Requirements:
- **Linear:** the new linear entry is reached only from `PP:3965`.
- **Nonlinear:** the loop's calls (the `_binary64` targets from `solve_linearized_system_evidence`, `StructuralAssembly::solve` at the loop, `product_equilibrium`, `scrutinize_gaps`) reach **no** formation check. Extend I1's pin test `option_c_nonlinear_loop_is_pinned_to_the_binary64_kernel_path` to assert this (mutation 32).
- **Nonlinear-support cases** are never selected and keep their ordinary result (ROOT ruling).

Put the list in `_run_records/callers.txt`, by lexer scan, as I1 did.

## Tests (from the K-D5 row; all required)

- **The required true positive:** RF-SKEW-T-CANT-OFF-122-r1e-04 demotes in both modes **and on both entries** (captured and typed).
- **Must not demote:** RF-SKEW 345, the RF-CHAIN r1e-04 continuity controls, the invented M11.
- **D5C-1 controls,** each demoting where its actual error exceeds half the criterion:
  - a solve-error-only case (V1's probe D class);
  - an absorbed-spring case (probe C);
  - an axis-aligned bending-soft case near the criterion.
- **The zero-scale clause,** as a kernel-level test supplying ledger terms that differ from the solve's force (N-2).
- **R5-4:**
  - E1 and E6 must not demote;
  - the skew-plane elbow cantilever at k_X = 8.5 demotes in both modes;
  - an expansion-joint model (lateral zero) must not demote;
  - a seeded non-re-formable family demotes with `formation_check_unavailable`.
- **Callers:** a nonlinear-loop model reaches no formation check (the pin).
- **D5C-3:** every committed raw is byte-identical when nothing demotes, and `StructuralReport` is unchanged.
- **The committed-fixture diff** (run every committed request through base and candidate, in both modes): expected unchanged. Any committed-byte change, including any demotion of a committed fixture case, **stops the work** and is reported to the manager with its site and reason before anything is regenerated.
- **The no-Passed-breach gate** through both entries, over the frozen references, with exactly `GATE/S11_EXCEPTIONS.json`'s triples as exceptions. The skew and curved cases are not exceptions.
- **Mutations** (23), (26)–(28), (31) and (32), in a scratch copy: record each patch, the command and the killing test. A survivor is a defect; never weaken a test.
- **Every existing suite** in the touched crates and their path dependents, including `benchmarks/nonlinear` (unchanged, with DEC-046's limits untouched), `benchmarks/mechanics`, `runner/headless` (with `--no-fail-fast`), `result_export`, `apps/desktop/src-tauri`, and the Python and desktop TS suites if any fixture reader is touched.

## Build rules

- `RUSTUP_TOOLCHAIN=1.97.1`, `CARGO_INCREMENTAL=0`, `CARGO_TARGET_DIR=<t3-target>`, `--offline --locked`.
- **One heavy cargo job at a time across T3.** Check `pgrep -x cargo` first.
- Keep free disk above about 8 GB.
- Run `cargo fmt` on changed files. **Make no Git index operations.**

## Disclosure and return

- **`T3/IMPLEMENTATION/KD5/CHANGE_RECORD.md`**, following `.agents/skills/chirality-change/SKILL.md`. It states:
  - which cases can change standing (Passed → Sensitive), and why;
  - that no value changes;
  - the measured fixture result;
  - the cost per curved element;
  - that there is no in-band marker.
- **`T3/IMPLEMENTATION/KD5/RETURN.md`**, with logs under `_run_records/`, `SHA256SUMS` and no machine paths. It covers:
  - the files changed and their line counts;
  - each write-set item and test;
  - the caller list;
  - the C2 script result;
  - the per-crate counts;
  - the mutation table;
  - the fixture diff;
  - the toolchain;
  - what was not done.

Send the manager a SendMessage summary. Message the manager at once if the stop rule triggers or a design item cannot be implemented as specified. Don't improvise a different design.

## Addendum: K3a's actual API (manager, 2026-09-26, on K3a head `a2e804a75`)

Base: branch `codex/piping-k3a-20260926` at `a2e804a757359d589f4c31ea8e36a923f28ccb8c`. K3a's surface, all `pub(crate)` in `frame_kernel`, is in `FK/src/structural/retained/wide.rs`:

- **Types.** `Wide2` (= `Wide<2>`), `Wide2::from_f64` (the exact lift), `WideArith::new(p)` with `add`, `sub`, `mul`, `div` and `sqrt` (correctly rounded at p ≤ 128, and each counted), and `work() -> WorkCounter`.
- **Angle.** `WideArith::included_angle(s, c)` = 2·atan(s/(1 + c)) ∈ (0, π), for s > 0 and 1 + c > 0. `atan_positive(t)` is also available. This is the arctangent R5-4 needs: sine and cosine come from the radial vectors with square roots, and φ from `included_angle`. Its proved bound is 23.6 ulp at p ≤ 128, and the test tolerance is 6 ulp.
- **Into the ledger.** `Wide2::add_product_to(&mut ExactAccumulator, factor: f64) -> Result<bool, WideError>` adds `self · factor` through the exact split into at most three binary64 terms. It returns `true` when the split was truncated below 2^-1074. **Use it for ρ's K_int·u terms,** and carry the truncation flag into the per-row allowance the design specifies (2^-1074·|factor|).
- **Errors.** `WideError` (InvalidPrecision, NonFinite, ExponentRange, DivisionByZero, NegativeSqrt, AngleDomain, ArctangentLimit, SplitOverflow, Accumulator). **Any `WideError` inside the formation check fails closed:** the case is demoted with `formation_check_unavailable`. It is never passed, and it never becomes an `Err` from the solve.
- **Module placement.** `retained` is declared privately in `structural.rs` (`mod retained;`), so `formation_check.rs` must sit under `structural/`, as the design places it, to reach `super::retained::wide`. Do not widen `retained`'s visibility.
- **dead_code.** K-D5 is K3a's first caller. Remove the `#![allow(dead_code)]` in `retained/mod.rs`, and fix any items that remain unused. Report which items those are.
- **Environment.** Use `RUSTUP_AUTO_INSTALL=0` and `RUSTUP_TOOLCHAIN=1.97.1`. Cargo priority: I1's S11-K fixes, then ROOT's DEC-025 sweep, then RV2, then you. Hold your cargo while the sweep runs.

## Addendum 2: realization decisions (manager, 2026-09-26, on I3's report; ROOT informed)

1. **Curved primitives (option B).**
   - `solve_with_formation_check` takes the curved macro elements as an extra argument, passed at the single PP:3965 call from `built.curved_bend_elements`.
   - SA matches each curved slot by (node_i, node_j) and bitwise equality of `macro.global_stiffness()` with the slot's matrix.
   - Any unmatched slot, and any explicit `CurvedBendStiffnessElement::new` slot, sets the cannot-re-form flag, so the case is demoted with `formation_check_unavailable`.
   - `CurvedBendStiffnessElement` and nonlinear `lib.rs` are not touched.
   - Tests: matched macro elements re-form (E1 and E6 not demoted, the k_X = 8.5 elbow demoted); a mismatched or explicit slot demotes; matching is order-independent.
2. **Nonlinear-support cases.**
   - The PP:3965 call passes `selected = built.nonlinear_supports.is_empty()`. When it is false, SA runs the unchanged `solve`. That is ROOT's "never selected, never refused".
   - Test: a nonlinear-support model with a skew soft member that would demote keeps its linear attempt, ordinary_attempt and receipt byte-identical to base.
3. **Accepted realizations:**
   - the `FormationCheckedSystem` wrapper, following S11-K's `AssembledStructuralSystem` pattern (StructuralReport unchanged; no struct literal outside the write set changes);
   - the K-D5 nonlinear pins in a new `structural_adapter/kd5_tests.rs`, not edits to I1's pin function, integrated when K-D5 is merged forward after S11-K and K3a merge;
   - joints with nonzero lateral stiffness are non-re-formable, and the lateral-zero test is at SA level.

**ROOT (2026-09-26): addendum 2 accepted, with conditions.**
- **Option B:** add a test in which a curved slot's matrix differs by **one ulp** from its macro element's re-formed matrix. The slot must demote with `formation_check_unavailable` (fail closed) and never pass. Option B must add **no new call path from nonlinear `lib.rs`**.
- **selected = built.nonlinear_supports.is_empty():** accepted. Keep the byte-identity test on such a case's linear attempt and receipt.
- **Nonlinear pins (RV1's S1 lesson, applied from the start):** every pin gets a behavioural test that first asserts the two paths differ. The pins are integrated into I1's module at the forward merge.
- **Joints with nonzero lateral stiffness:** they demote (the product refuses them anyway; M07, T4).

## Addendum 3: angles near π (ROOT, 2026-09-27, from RV2's K3a review, note N5)

`WideArith::included_angle(s, c)` refuses with `AngleDomain` when 1 + c ≤ 0, which happens for angles within about 1e-19 of π, where c rounds to exactly −1. **That refusal routes to K-D5's "cannot re-form" path:** the case is demoted with `formation_check_unavailable`, and it never passes. Add a test with a curved element at an included angle within 1e-19 of π, asserting the demotion. The accuracy contract you may cite for the arctangent is the **proved 23.6 ulp** (the 6-ulp figure is K3a's regression tolerance only; RV2 S1).

## Addendum 4: dead_code and the Git slip (ROOT, 2026-09-27)

- **The slip** (`git checkout -- …/retained/wide.rs`, discarding I3's own uncommitted edit): accepted as disclosed, and recorded in RETURN. The rule stands: no checkout, restore, reset or stash. To undo an edit, re-apply bytes from `git show HEAD:<path>`.
- **dead_code.** The module-level `#[cfg_attr(not(test), allow(dead_code))]` on `mod wide;` is acceptable **on the development branch only**. At the forward merge, after K3a is on main, replace it with **per-item `#[allow(dead_code)]` on exactly the items still unused outside tests**, each with a one-line reason ("test-only", or "K3 API, slice K3"). Nothing unused may stay hidden behind a module-wide attribute; that is what fulfils "K-D5 removes the allowance". The K-D5 reviewer checks the list.
