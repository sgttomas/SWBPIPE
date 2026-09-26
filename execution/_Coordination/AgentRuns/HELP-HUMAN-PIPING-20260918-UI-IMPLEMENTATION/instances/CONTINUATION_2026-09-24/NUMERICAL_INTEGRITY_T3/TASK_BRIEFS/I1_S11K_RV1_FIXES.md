# I1: S11-K fixes for RV1's SHOULD-FIX findings (S1, S2) and N4

A focused follow-up for I1 (resumed). `_COMMON.md` and I1's original brief apply, except where this brief differs. ROOT ruled (2026-09-26) that these are fixed before merge. RV1 then backchecks only the delta.

## Basis

- RV1's review: `T3/REVIEW/S11K_REVIEW.md` (`65e98c259` on the T3 branch, sha256 `eddd454b…`), findings **RV1-S1**, **RV1-S2** and **RV1-N4**, and its mutants RV-OPT1, RV-OPT3, RV-OPT4 and RV-PUB (patches under `REVIEW/_run_records/s11k_review/`).
- ROOT's regeneration approvals, now recorded in `T3/ROOT_RULINGS_V1.md` (section "S11-K regeneration and hash-pin approvals", `cef281b21`). Read them from the numerics worktree.

## Worktree

`/home/user/wt/s11k-pr`, branch `codex/piping-s11k-pr-20260926`. **First** run `git merge --ff-only origin/codex/piping-s11k-pr-20260926`, which brings it to `f76643235` (ROOT's main merge; no piping change). That is a fast-forward only; make no other Git write. The manager commits.

## Fixes

1. **S1: the option (c) loop pin must actually pin.**
   - The source pin in `nonlinear_integration/src/s11k_tests.rs` strips comments before matching. Reuse the site table's `lex()` from `frame_kernel/tests/s11_site_table.rs`, or an equivalent comment and string-aware stripper, and cite it.
   - **No exact kernel entry point appears anywhere in `nonlinear_integration/src/lib.rs` outside `cfg(test)` code.** That covers `reduce_system_with_prescribed_displacements(`, `assembly.solve(`, `solve_assembled(`, `solve_structural_dense(`, `solve_structural_sparse(`, `prepare_structural(` and any other exact entry. The pin is scoped to the whole file, not only to `solve_linearized_system_evidence`, so a helper cannot hide one.
   - **A new behavioural test.** Run a closed-gap active-set case, dense and sparse, with and without `AssemblyEvidence`.
     - First assert, inside the test, that the binary64 fold of f − ΣK·g differs from the exact value. This is the precondition rule.
     - Then assert that the reduced force, the displacements and the residual rows are **bit-equal to the binary64 legacy values**.
     - Derive the expected binary64 values in the test from the legacy fold order, not from the code under test.
   - **Re-run RV-OPT1, RV-OPT3 and RV-OPT4** in a scratch copy, and show each is now killed **by the behavioural test** (not only by the source pin). Record the patch, the command and the killing test for each.
2. **S2: the public-residual pin must not be vacuous.**
   - In `frame_kernel/src/structural/s11k_tests.rs` `option_c_public_original_residual_stays_binary64_on_coupled_rows`, choose `u` (for example perturb `u[11]` or `u[7]` by a few ulp, or use a different g) so that row 11's binary64 expression differs from the exact numerator.
   - `assert_ne!` that difference first, then keep the equality check.
   - **Re-run RV-PUB** in scratch and show that this test now fails.
3. **N4: the CHANGE_RECORD.md header.**
   - The header names the PR branch `codex/piping-s11k-pr-20260926`, base `6bb3ee490` (now merged forward to `f76643235`), and the basis: S11_CONTAINMENT **revision 5a.2** (`e6507587`), selected by `ROOT_SELECTION_DESIGNS.md`.
   - "Checks run" records the command actually used. If the recorded `run_suites.sh.txt` ran without `--no-fail-fast`, say so. Also record your later `--no-fail-fast` re-run for headless, as it was actually run.
   - Add a line citing ROOT's approvals entry in `ROOT_RULINGS_V1.md` (RV1-S3).
   - Add the new tests and mutation re-runs to the test and mutation sections.

## Rules

- **No production-code change.** Only tests, the pin and the records change. If a fix seems to need production code, stop and tell the manager.
- **No fixture or committed-output change.** If one appears, stop and report.
- **Suites:** `frame_kernel`, `nonlinear_integration` and `validation/benchmarks/nonlinear` (DEC-046 unchanged) must be green. Run `straight_pipe` too if you touch shared helpers.
- **One cargo job at a time.** I2 (K3a) yields cargo to you for this; still check `pgrep -x cargo` before each run. Use `CARGO_TARGET_DIR=<t3-target>` (freshly pruned). Keep free disk above about 8 GB.
- No Git writes apart from the initial `--ff-only`. No index operations.

## Return

Append a short "RV1 fixes" section to `IMPLEMENTATION/S11K/RETURN.md` covering:
- the files changed, with line counts;
- each fix and how it now pins;
- the mutation re-runs (RV-OPT1, RV-OPT3, RV-OPT4, RV-PUB): the killing test for each;
- the suite counts.

Put logs under `_run_records/rv1_fixes/`, and refresh `SHA256SUMS`. Send the manager a SendMessage summary.
