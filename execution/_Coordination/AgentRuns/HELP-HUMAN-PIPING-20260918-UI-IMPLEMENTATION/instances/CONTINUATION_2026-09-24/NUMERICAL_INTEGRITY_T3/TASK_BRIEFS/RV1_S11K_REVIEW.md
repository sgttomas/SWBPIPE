# RV1: independent full-diff review of slice S11-K

This is a review TASK. Read `_COMMON.md` first; this brief overrides it where they differ.

You must be independent: you did not design S11, review its design, or implement S11-K. Your job is to find defects in the candidate, not to confirm it. Report what you find. You fix nothing.

## Candidate

- **Branch:** `codex/piping-s11k-20260926`, in `<s11k-worktree>`. ROOT gives the path at spawn.
- **Candidate revision:** the head the manager names at spawn, after I1's regeneration commit. Review the full diff from the merge base with `origin/main` to that head (`git diff $(git merge-base origin/main <head>)..<head>`). Record both revisions in your return.
- **Every line of the diff is in scope:**
  - product source;
  - tests and mutations;
  - lockfiles;
  - regenerated fixtures and derived documents;
  - the two hash-pin test constants;
  - the records under `T3/IMPLEMENTATION/S11K/**`, including `CHANGE_RECORD.md`, `RETURN.md`, `PRE_REGENERATION_REPORT.md` and `_run_records/`.

## Basis (what the candidate must satisfy)

1. `T3/ROOT_SELECTION_S11.md`: the selection and conditions R3-1 to R3-4.
2. `T3/DESIGN_NUMERICS/S11_CONTAINMENT.md`, the revision named at spawn (revision 5 or later):
   - §2.4 and §4.6 (KS1–KS3);
   - §4 (the rule), §5.3 (the invariant);
   - **§8.1 (the S11-K write set and live effect)**, §8.3 (disclosure and the fixture stop rule);
   - §9 (K1–K12 and the mutation table).
3. `T3/ROOT_RULINGS_V1.md`:
   - D-S11-1 to D-S11-4;
   - S11B-1;
   - the no-interim ruling;
   - **ROOT's rulings on I1's stop report**: option (c), with A and B2 pre-registered and B1 accepted;
   - the S11 caller-list lesson;
   - BACKCHECK_R5's R5-3;
   - the regeneration approval and hash-pin approval of 2026-09-26.
4. `T3/TASK_BRIEFS/I1_S11K_IMPLEMENTATION.md`: I1's brief, including its write set, the out-of-scope list and the build rules.

## What to check (at least)

1. **§8.1 items.**
   - Each one is implemented as designed:
     - the correctly rounded accumulator (68 limbs, quantum 2^-2148, +0.0 for an exact zero or an underflowed nonzero net, no copy-bits shortcut);
     - the load ledger and typed seams beside today's entry points;
     - E1–E4, E6 and E13;
     - `pressure_sum` as a wrapper only;
     - the zero witness only at `FK/structural.rs:554`.
   - `PP` compiles unchanged, with no `PP` producer edited.
   - Nothing outside the write set changes, apart from the approved regenerations and hash pins.
2. **The R3 fixes.**
   - **R3-1:** bit-identity holds only with all-zero prescribed values, and the nonzero cases are exactly the pre-registered diffs.
   - **R3-2:** K4's axial case kills M1b and the axial half of E3.
   - **R3-3:** the site-test constant table is complete for S11-K's crates. Every floating-point fold is either an E-site or an explicit exemption, and the unit-force allow-list row is justified.
   - **R3-4:** KS1 and KS3 scale before rounding, or a recorded proof shows there is no double rounding.
3. **R5-3, the caller classification.** Independently re-derive the complete caller list of every KS-affected function:
   - `reduce_system_with_prescribed_displacements` and `reduce_system`;
   - `prepare_structural` and everything composed on it;
   - `evaluate_original_residual`;
   - `AssemblyEvidence::solve`.
   Use your own search, then compare with I1's `_run_records/fixture_diff/ks_callers.txt`. Any caller missing or misclassified (linear, exact / zero-prescribed, bit-identical / nonlinear, legacy) is a finding.
4. **The option (c) pins.**
   - The nonlinear active-set loop, including `product_equilibrium::evaluate` and SA `scrutinize_gaps`, reaches only the named binary64 variants.
   - Those variants are byte-identical in behaviour to the merge-base code. Compare them against the base source, not only against the tests.
   - The source pin cannot be satisfied by a trivial textual workaround. Say whether a behavioural pin should back it.
   - `validation/benchmarks/nonlinear` and the DEC-046 limits are unchanged.
5. **Exactness claims.** Spot-check the accumulator and each E-site against your own exact rational computation, with the standard library `fractions` module:
   - cancellation at G = 1e7, 1e8 and 1e80;
   - subnormal and underflow nets;
   - signed zero;
   - order independence.
   Check that the precondition rule holds in every fold-killing test: the test asserts that the binary64 fold differs from the correctly rounded net.
6. **Mutations.**
   - Read the mutation table and the patches.
   - Re-run a sample: at least M1 on two E-sites, M12 or M13, and one KS mutant. Confirm each is killed by the named test.
   - A mutant killed only by a source-text pin, where a behavioural test was expected, is a finding.
7. **Fixtures and regeneration.**
   - The committed diff equals I1's measured pre-regeneration diff: the same files and the same leaf counts per file (`PRE_REGENERATION_REPORT.md` §5).
   - Every changed file was produced by its producer: re-run at least one producer per kind and compare the bytes.
   - No frozen reference or historical raw changed.
   - The hash-pin constants equal the new raws' sha256, and the change record states old → new.
8. **Disclosure.** `CHANGE_RECORD.md` states:
   - which quantities change bits and why;
   - the non-cancelling bound (at most one rounding of the gross);
   - that no case changes status unless it was absorbing a load;
   - every size;
   - the explanation of the eigen_motion sparse `invocation_work` / publication_charged −12;
   - that there is no in-band marker.
   It follows `.agents/skills/chirality-change/SKILL.md`.
9. **Hygiene.**
   - No machine-specific absolute paths in committed files.
   - No `node_modules` links or scratch output committed.
   - `cargo fmt` is clean on changed files.
   - `git diff --check` is clean, or the exceptions are recorded.
   - Lockfile changes are path-dependency lines only, with no registry change.

## Running things

- You may build and run tests, with `RUSTUP_TOOLCHAIN=1.97.1`, `CARGO_INCREMENTAL=0`, `CARGO_TARGET_DIR=<t3-target>` and `--offline --locked`.
- Run **one heavy cargo job at a time**: check `pgrep -x cargo` first, and wait if another is running.
- Keep free disk above about 8 GB. Put scratch output under `<scratch>`, and delete your build output when done.
- Skip no tests and raise no timeouts. Make no Git writes, and edit nothing in the worktree. Mutations run in a scratch copy only.

## Verdict and return

Write `T3/REVIEW/S11K_REVIEW.md` in `<s11k-worktree>`. It contains:
- the revisions reviewed;
- a findings table (ID, severity BLOCKING / SHOULD-FIX / NOTE, site, evidence, what would resolve it);
- a section per check item above;
- your independent caller list;
- what you ran, with commands and results;
- what you did not check.

The verdict is **PASS** (no unresolved BLOCKING findings) or **FAIL**. Then send the manager a SendMessage summary with the verdict, the findings count by severity, and the file's sha256.
